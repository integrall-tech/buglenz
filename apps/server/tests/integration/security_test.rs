//! Package 023 (ADR-0018): fixes from the upstream security review that the base lacked.
//!
//! H-1 login timing, H-2/M-3 password length, M-2 state kept across login, H-4 webhook
//! destinations (the notifier unit tests cover the URL rules), M-1 oversized ingest body.

use crate::common::TestDb;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::Key, test, web, App};
use rustrak::models::{CreateInvitation, CreateUserRequest, UserRole};
use rustrak::routes;
use rustrak::services::{InvitationService, UsersService};
use serde_json::json;
use std::time::{Duration, Instant};

async fn user(pool: &rustrak::db::DbPool, email: &str, password: &str) -> rustrak::models::User {
    UsersService::create_user(
        pool,
        &CreateUserRequest {
            email: email.to_string(),
            password: password.to_string(),
        },
        UserRole::Member,
    )
    .await
    .expect("create user")
}

fn login(email: &str, password: &str) -> test::TestRequest {
    test::TestRequest::post()
        .uri("/auth/login")
        .insert_header(("Content-Type", "application/json"))
        .set_json(json!({ "email": email, "password": password }))
}

macro_rules! auth_app {
    ($db:expr) => {
        test::init_service(
            App::new()
                .app_data(web::Data::new($db.pool.clone()))
                .wrap(
                    SessionMiddleware::builder(
                        CookieSessionStore::default(),
                        Key::from(&[0u8; 64]),
                    )
                    .cookie_secure(false)
                    .build(),
                )
                .route(
                    "/plant",
                    web::get().to(|s: Session| async move {
                        s.insert("sso_state", "planted-by-attacker").unwrap();
                        actix_web::HttpResponse::Ok().finish()
                    }),
                )
                .route(
                    "/peek",
                    web::get().to(|s: Session| async move {
                        let v: Option<String> = s.get("sso_state").unwrap();
                        actix_web::HttpResponse::Ok().body(v.unwrap_or_default())
                    }),
                )
                .configure(routes::auth::configure),
        )
        .await
    };
}

fn median(mut v: Vec<Duration>) -> Duration {
    v.sort();
    v[v.len() / 2]
}

// H-1 ---------------------------------------------------------------------------

#[actix_web::test]
async fn login_for_an_unknown_email_costs_about_as_much_as_a_wrong_password() {
    let db = TestDb::new().await;
    user(&db.pool, "known@example.com", "password123").await;
    let app = auth_app!(db);

    let mut unknown = Vec::new();
    let mut wrong = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        let r = test::call_service(
            &app,
            login("nobody@example.com", "password123").to_request(),
        )
        .await;
        assert_eq!(r.status(), 401);
        unknown.push(t.elapsed());

        let t = Instant::now();
        let r = test::call_service(
            &app,
            login("known@example.com", "wrong-password").to_request(),
        )
        .await;
        assert_eq!(r.status(), 401);
        wrong.push(t.elapsed());
    }
    let (unknown, wrong) = (median(unknown), median(wrong));
    assert!(
        unknown * 3 >= wrong,
        "unknown email answered in {unknown:?}, wrong password in {wrong:?}: the difference tells accounts apart"
    );
}

// H-2 / M-3 ---------------------------------------------------------------------

#[actix_web::test]
async fn login_rejects_an_oversized_password_before_any_work() {
    let db = TestDb::new().await;
    user(&db.pool, "known@example.com", "password123").await;
    let app = auth_app!(db);
    let long = "a".repeat(1025);

    for email in ["known@example.com", "nobody@example.com"] {
        let t = Instant::now();
        let r = test::call_service(&app, login(email, &long).to_request()).await;
        assert_eq!(r.status(), 400, "{email}: oversized password");
        assert!(
            t.elapsed() < Duration::from_millis(50),
            "{email}: no Argon2 for an oversized password"
        );
    }

    // The limit itself is still a normal (failed) login.
    let r = test::call_service(
        &app,
        login("known@example.com", &"a".repeat(1024)).to_request(),
    )
    .await;
    assert_eq!(r.status(), 401);
}

#[actix_web::test]
async fn accepting_an_invitation_rejects_an_oversized_password() {
    let db = TestDb::new().await;
    let admin = user(&db.pool, "admin@example.com", "password123").await;
    let invitation = InvitationService::create(
        &db.pool,
        CreateInvitation {
            email: "new@example.com".into(),
            role: "member".into(),
        },
        admin.id,
    )
    .await
    .expect("invitation");

    let err = InvitationService::accept(&db.pool, &invitation.token, &"a".repeat(1025))
        .await
        .expect_err("oversized password");
    assert!(err.to_string().to_lowercase().contains("1024"), "{err}");
    // The invitation is still usable afterwards.
    InvitationService::accept(&db.pool, &invitation.token, "short")
        .await
        .expect("short password still allowed");
}

#[actix_web::test]
async fn changing_a_password_rejects_oversized_values() {
    let db = TestDb::new().await;
    let u = user(&db.pool, "known@example.com", "password123").await;

    assert!(
        UsersService::change_password(&db.pool, u.clone(), "password123", &"a".repeat(1025))
            .await
            .is_err()
    );
    assert!(
        UsersService::change_password(&db.pool, u.clone(), &"a".repeat(1025), "newpassword1")
            .await
            .is_err()
    );
    UsersService::change_password(&db.pool, u, "password123", "newpassword1")
        .await
        .expect("normal change");
}

// M-2 ---------------------------------------------------------------------------

#[actix_web::test]
async fn login_drops_session_data_set_before_it() {
    let db = TestDb::new().await;
    user(&db.pool, "known@example.com", "password123").await;
    let app = auth_app!(db);

    let planted =
        test::call_service(&app, test::TestRequest::get().uri("/plant").to_request()).await;
    let cookie = planted
        .response()
        .cookies()
        .next()
        .expect("session cookie")
        .into_owned();

    let r = test::call_service(
        &app,
        login("known@example.com", "password123")
            .cookie(cookie)
            .to_request(),
    )
    .await;
    assert_eq!(r.status(), 200);
    let after = r
        .response()
        .cookies()
        .next()
        .expect("new session cookie")
        .into_owned();

    let peek = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/peek")
            .cookie(after)
            .to_request(),
    )
    .await;
    let body = test::read_body(peek).await;
    assert!(
        body.is_empty(),
        "data planted before login survived it: {:?}",
        body
    );
}

// M-1 ---------------------------------------------------------------------------

#[actix_web::test]
async fn an_oversized_ingest_body_is_refused_with_a_json_413() {
    use rustrak::config::{Config, DashboardConfig, DatabaseConfig, RateLimitConfig};
    use rustrak::services::{
        DbSourceMapProvider, LocalSourceMapStore, ProjectService, SourceMapProvider, SourceMapStore,
    };
    use std::sync::Arc;

    let db = TestDb::new().await;
    let project = ProjectService::create(
        &db.pool,
        rustrak::models::CreateProject {
            name: "Oversize".into(),
            slug: None,
            platform: None,
        },
    )
    .await
    .unwrap();
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 0,
        database: DatabaseConfig {
            url: "postgres://test:test@localhost/test".to_string(),
            max_connections: 5,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(5),
            idle_timeout: Duration::from_secs(60),
            max_lifetime: Duration::from_secs(300),
        },
        rate_limit: RateLimitConfig {
            max_events_per_minute: 1000,
            max_events_per_hour: 10000,
            max_events_per_project_per_minute: 500,
            max_events_per_project_per_hour: 5000,
        },
        security: rustrak::config::SecurityConfig {
            ssl_proxy: false,
            session_secret_key: None,
        },
        ingest_dir: None,
        public_url: None,
        sourcemap_storage_path: "/tmp/test_sourcemaps".to_string(),
        sourcemap_cache_bytes: 64 * 1024 * 1024,
        max_chunk_size_bytes: 10 * 1024 * 1024,
        session_flush_interval_secs: 30,
        session_cardinality_cap: 10_000,
        dashboard: DashboardConfig {
            dir: "./static".to_string(),
            enabled: true,
            url: None,
        },
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data({
                let store: Arc<dyn SourceMapStore> =
                    Arc::new(LocalSourceMapStore::new("/tmp/test_sourcemaps"));
                let provider: Arc<dyn SourceMapProvider> =
                    Arc::new(DbSourceMapProvider::new(db.pool.clone(), store));
                web::Data::new(provider)
            })
            .app_data(web::Data::new(
                rustrak::digest::processors::Processors::new(
                    rustrak::ingest::get_ingest_dir(config.ingest_dir.as_deref()),
                    config.rate_limit.clone(),
                    crate::common::null_sourcemap_provider(),
                    None,
                ),
            ))
            .configure(routes::ingest::configure),
    )
    .await;

    let body = vec![0u8; 100 * 1024 * 1024 + 1];
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!("/api/{}/envelope/", project.id))
            .insert_header((
                "X-Sentry-Auth",
                format!("Sentry sentry_key={}, sentry_version=7", project.sentry_key),
            ))
            .insert_header(("Content-Type", "application/x-sentry-envelope"))
            .set_payload(body)
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), 413);
    let content_type = resp
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap().to_string())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("application/json"),
        "413 must be JSON, got {content_type:?}"
    );
}
