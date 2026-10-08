//! The process counters in the request path: the HTTP middleware, the digest
//! outcomes and failed alert deliveries. The upstream's anonymous telemetry
//! (PostHog adapter, instance id, volume queries, reporter loop, preview
//! route) was removed in the BugLenz fork (ADR-0004); its tests went with it.

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::Value;

/// A local collector: records every JSON body it receives and answers with
/// one fixed status.
struct Collector {
    url: String,
    received: Arc<Mutex<Vec<Value>>>,
}

impl Collector {
    async fn start(status: u16) -> Self {
        let received: Arc<Mutex<Vec<Value>>> = Arc::default();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind collector");
        let addr = listener.local_addr().expect("collector addr");
        let sink = received.clone();
        let server = HttpServer::new(move || {
            let sink = sink.clone();
            App::new().default_service(web::to(move |body: web::Json<Value>| {
                let sink = sink.clone();
                async move {
                    sink.lock().unwrap().push(body.into_inner());
                    HttpResponse::build(
                        actix_web::http::StatusCode::from_u16(status).expect("valid status"),
                    )
                    .finish()
                }
            }))
        })
        .workers(1)
        .listen(listener)
        .expect("listen")
        .run();
        tokio::spawn(server);
        Self {
            url: format!("http://{addr}/hook"),
            received,
        }
    }

    fn received(&self) -> Vec<Value> {
        self.received.lock().unwrap().clone()
    }
}

// =============================================================================
// The HTTP middleware: outcomes by status, never by path
// =============================================================================

mod middleware {
    use actix_web::{http::StatusCode, test, web, App, HttpResponse};
    use rustrak::middleware::telemetry::TelemetryMiddleware;
    use rustrak::telemetry::Counters;

    fn leaked_counters() -> &'static Counters {
        Box::leak(Box::new(Counters::new()))
    }

    async fn answer(req: actix_web::HttpRequest) -> HttpResponse {
        let status: u16 = req.match_info().query("status").parse().unwrap();
        HttpResponse::build(StatusCode::from_u16(status).unwrap()).finish()
    }

    async fn drive(counters: &'static Counters, requests: &[&str]) {
        let app = test::init_service(
            App::new()
                .wrap(TelemetryMiddleware::new(counters))
                .route(
                    "/api/{project_id}/envelope/{status}",
                    web::post().to(answer),
                )
                .route("/api/{project_id}/store/{status}", web::post().to(answer))
                .route("/api/issues/{id}/{status}", web::get().to(answer))
                .route("/api/projects/{status}", web::get().to(answer)),
        )
        .await;
        for uri in requests {
            let req = if uri.contains("/envelope/") || uri.contains("/store/") {
                test::TestRequest::post().uri(uri).to_request()
            } else {
                test::TestRequest::get().uri(uri).to_request()
            };
            let _ = test::call_service(&app, req).await;
        }
    }

    #[actix_web::test]
    async fn ingest_outcomes_are_bucketed_by_status() {
        let counters = leaked_counters();
        drive(
            counters,
            &[
                "/api/1/envelope/200",
                "/api/1/store/200",
                "/api/1/envelope/429",
                "/api/1/envelope/401",
                "/api/1/envelope/403",
                "/api/1/envelope/413",
                "/api/1/envelope/400",
                "/api/1/envelope/418",
            ],
        )
        .await;
        let ingest = counters.snapshot_and_reset().ingest;
        assert_eq!(ingest.accepted, 2);
        assert!(ingest.latency_ms.p50.is_some());
        let r = ingest.rejected;
        assert_eq!(
            (r.rate_limit, r.auth, r.too_large, r.malformed, r.other),
            (1, 2, 1, 1, 1)
        );
    }

    #[actix_web::test]
    async fn server_errors_are_counted_by_route_pattern_not_path() {
        let counters = leaked_counters();
        drive(
            counters,
            &[
                "/api/issues/17/500",
                "/api/issues/99/503",
                "/api/projects/500",
                "/api/1/envelope/500",
            ],
        )
        .await;
        let health = counters.snapshot_and_reset();
        assert_eq!(health.http_5xx_by_route["/api/issues/{id}/{status}"], 2);
        assert_eq!(health.http_5xx_by_route["/api/projects/{status}"], 1);
        assert_eq!(
            health.http_5xx_by_route["/api/{project_id}/envelope/{status}"],
            1
        );
        assert!(health.http_5xx_by_route.keys().all(|k| !k.contains("17")));
        assert_eq!(
            health.ingest.accepted, 0,
            "a 500 on ingest is not an accept"
        );
        assert_eq!(health.ingest.rejected.other, 0, "nor a rejection");
    }

    #[actix_web::test]
    async fn ordinary_api_traffic_is_not_counted_at_all() {
        let counters = leaked_counters();
        drive(counters, &["/api/projects/200", "/api/issues/1/404"]).await;
        let health = counters.snapshot_and_reset();
        assert_eq!(health.ingest.accepted, 0);
        assert_eq!(health.ingest.rejected.other, 0);
        assert!(health.http_5xx_by_route.is_empty());
    }
}

// =============================================================================
// Digest outcomes
// =============================================================================

mod digest {
    use chrono::Utc;
    use serde_json::json;
    use tempfile::TempDir;
    use uuid::Uuid;

    use crate::common::TestDb;
    use rustrak::config::RateLimitConfig;
    use rustrak::digest::processors::Processors;
    use rustrak::ingest::{store_event, EventMetadata};
    use rustrak::models::CreateProject;
    use rustrak::routes::ingest::digest_stored_event;
    use rustrak::services::ProjectService;
    use rustrak::telemetry::Counters;

    fn leaked_counters() -> &'static Counters {
        Box::leak(Box::new(Counters::new()))
    }

    async fn setup(
        pool: &rustrak::db::DbPool,
        ingest_dir: &std::path::Path,
    ) -> (Processors, EventMetadata) {
        let project = ProjectService::create(
            pool,
            CreateProject {
                name: "Telemetry".to_string(),
                slug: None,
                platform: None,
            },
        )
        .await
        .expect("project");
        let processors = Processors::new(
            ingest_dir.to_path_buf(),
            RateLimitConfig {
                max_events_per_minute: 1000,
                max_events_per_hour: 10000,
                max_events_per_project_per_minute: 500,
                max_events_per_project_per_hour: 5000,
            },
            crate::common::null_sourcemap_provider(),
            None,
        )
        .with_counters(leaked_counters());
        let event_id = Uuid::new_v4().to_string().replace('-', "");
        let metadata = EventMetadata {
            event_id,
            project_id: project.id,
            ingested_at: Utc::now(),
            remote_addr: None,
        };
        (processors, metadata)
    }

    #[actix_web::test]
    async fn a_digested_event_counts_as_ok() {
        let db = TestDb::new().await;
        let dir = TempDir::new().expect("temp dir");
        let (processors, metadata) = setup(&db.pool, dir.path()).await;
        let event = json!({
            "event_id": metadata.event_id,
            "timestamp": Utc::now().timestamp() as f64,
            "platform": "rust",
            "exception": { "values": [{ "type": "TypeError", "value": "boom" }] }
        });
        store_event(
            dir.path(),
            &metadata.event_id,
            &serde_json::to_vec(&event).unwrap(),
        )
        .await
        .expect("store");

        digest_stored_event(&processors, &db.pool, &metadata).await;

        let digest = processors.counters().snapshot_and_reset().digest;
        assert_eq!((digest.ok, digest.failed), (1, 0));
    }

    #[actix_web::test]
    async fn an_event_that_cannot_be_digested_counts_as_failed() {
        let db = TestDb::new().await;
        let dir = TempDir::new().expect("temp dir");
        let (processors, metadata) = setup(&db.pool, dir.path()).await;
        // Nothing was stored under this id: the digest has nothing to read.

        digest_stored_event(&processors, &db.pool, &metadata).await;

        let digest = processors.counters().snapshot_and_reset().digest;
        assert_eq!((digest.ok, digest.failed), (0, 1));
    }
}

// =============================================================================
// Alert deliveries that fail
// =============================================================================

mod alerts {
    use std::time::Duration;

    use chrono::Utc;
    use serde_json::json;

    use super::Collector;
    use crate::common::TestDb;
    use rustrak::models::{
        AlertRuleChannelInput, AlertType, ChannelType, CreateAlertRule, CreateNotificationChannel,
        CreateProject,
    };
    use rustrak::services::grouping::DenormalizedFields;
    use rustrak::services::{AlertService, IssueService, ProjectService};
    use rustrak::telemetry::Counters;

    /// Every dispatcher is a static method with no instance to hand a
    /// counter set to, so this is the one path that counts on the global set.
    #[actix_web::test]
    async fn a_failed_delivery_is_counted_under_its_provider_kind() {
        let db = TestDb::new().await;
        let collector = Collector::start(500).await;
        let project = ProjectService::create(
            &db.pool,
            CreateProject {
                name: "Alerts".to_string(),
                slug: None,
                platform: None,
            },
        )
        .await
        .expect("project");
        let channel = AlertService::create_channel(
            &db.pool,
            CreateNotificationChannel {
                name: "Broken webhook".to_string(),
                provider_type: ChannelType::Webhook,
                credentials: json!({ "url": collector.url }),
                is_enabled: true,
            },
        )
        .await
        .expect("channel");
        AlertService::create_rule(
            &db.pool,
            project.id,
            CreateAlertRule {
                name: "Any new issue".to_string(),
                alert_type: AlertType::NewIssue,
                channels: vec![AlertRuleChannelInput {
                    integration_id: channel.id,
                    routing_override: json!({}),
                }],
                conditions: json!({}),
                cooldown_minutes: 0,
            },
        )
        .await
        .expect("rule");
        let issue = IssueService::create(
            &db.pool,
            project.id,
            Utc::now(),
            &DenormalizedFields {
                calculated_type: "Error".to_string(),
                calculated_value: "broken webhook".to_string(),
                transaction: "/test".to_string(),
                last_frame_filename: "test.rs".to_string(),
                last_frame_module: "test".to_string(),
                last_frame_function: "test".to_string(),
                culprit: "test".to_string(),
                logger: String::new(),
                release: String::new(),
            },
            Some("error"),
            Some("rust"),
        )
        .await
        .expect("issue");

        AlertService::trigger_new_issue_alert(&db.pool, &project, &issue, "http://localhost:3000")
            .await
            .expect("trigger");

        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            let failed = Counters::global()
                .snapshot_and_reset()
                .alerts_failed_by_provider;
            if failed.get("webhook").is_some_and(|n| *n >= 1) {
                break;
            }
            assert!(
                tokio::time::Instant::now() < deadline,
                "no failure counted: {failed:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        assert_eq!(
            collector.received().len(),
            1,
            "the delivery was attempted once"
        );
    }
}
