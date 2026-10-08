//! Package 023 / H-4: webhook destinations may not be internal or private addresses.

use rustrak::models::ChannelType;
use rustrak::services::create_dispatcher;
use rustrak::services::notification::destination;
use serde_json::json;

const TEMPLATE: &str = r#"{"text":"{{ issue.title }}"}"#;

const BLOCKED: &[&str] = &[
    "http://127.0.0.1/hook",
    "http://127.1.2.3/hook",
    "http://0.0.0.0/hook",
    "http://10.0.0.1/hook",
    "http://10.255.255.255/hook",
    "http://172.16.0.1/hook",
    "http://172.31.255.255/hook",
    "http://192.168.0.1/hook",
    "http://169.254.169.254/latest/meta-data/",
    "http://100.64.0.1/hook",
    "http://[::1]/hook",
    "http://[fc00::1]/hook",
    "http://[fd12:3456::1]/hook",
    "http://[fe80::1]/hook",
    "http://[::ffff:127.0.0.1]/hook",
    "http://[::ffff:10.0.0.1]/hook",
    "http://localhost/hook",
    "https://LOCALHOST/hook",
    "http://localhost./hook",
    "http://db.internal/hook",
    "http://printer.local/hook",
    "http://x.localhost/hook",
    "http://2130706433/hook",
    "http://0x7f.0.0.1/hook",
];

const ALLOWED: &[&str] = &[
    "https://hooks.example.com/webhook",
    "https://discord.com/api/webhooks/123/abc",
    "http://203.0.113.10/hook",
    "https://172.32.0.1/hook",
    "https://[2001:db8::1]/hook",
];

#[test]
fn the_destination_check_blocks_internal_and_private_hosts() {
    for url in BLOCKED {
        assert!(
            destination::check_url(url).is_err(),
            "{url} must be blocked"
        );
    }
}

#[test]
fn the_destination_check_allows_public_hosts() {
    for url in ALLOWED {
        assert!(destination::check_url(url).is_ok(), "{url} must be allowed");
    }
}

#[test]
fn an_exempt_host_passes_and_only_that_host() {
    let allowed = destination::parse_allowed(Some(" Archflow.internal , 10.1.2.3 ,, [::1] "));
    assert!(destination::check_url_with("http://archflow.internal/hook", &allowed).is_ok());
    assert!(destination::check_url_with("http://ARCHFLOW.INTERNAL./hook", &allowed).is_ok());
    assert!(destination::check_url_with("http://10.1.2.3:8080/hook", &allowed).is_ok());
    assert!(destination::check_url_with("http://10.1.2.4/hook", &allowed).is_err());
    assert!(destination::check_url_with("http://other.internal/hook", &allowed).is_err());
    assert!(destination::check_url_with("http://169.254.169.254/", &allowed).is_err());
    assert!(destination::check_url_with("http://127.0.0.1/", &allowed).is_err());
}

#[test]
fn the_exemption_list_is_empty_by_default() {
    assert!(destination::parse_allowed(None).is_empty());
    assert!(destination::parse_allowed(Some(" , ")).is_empty());
}

#[test]
fn webhook_validation_blocks_internal_hosts() {
    let webhook = create_dispatcher(ChannelType::Webhook);
    for url in BLOCKED {
        assert!(
            webhook.validate_config(&json!({ "url": url })).is_err(),
            "webhook {url}"
        );
    }
    for url in ALLOWED {
        assert!(
            webhook.validate_config(&json!({ "url": url })).is_ok(),
            "webhook {url}"
        );
    }
}

#[test]
fn custom_webhook_validation_blocks_internal_hosts() {
    let custom = create_dispatcher(ChannelType::CustomWebhook);
    for url in BLOCKED {
        assert!(
            custom
                .validate_config(&json!({ "url": url, "template": TEMPLATE }))
                .is_err(),
            "custom webhook {url}"
        );
    }
    for url in ALLOWED {
        assert!(
            custom
                .validate_config(&json!({ "url": url, "template": TEMPLATE }))
                .is_ok(),
            "custom webhook {url}"
        );
    }
}

// A routing override is saved without validation, so the check also runs when the alert is sent.

fn integration(
    kind: rustrak::models::ProviderType,
    credentials: serde_json::Value,
) -> rustrak::models::AlertIntegration {
    let now = chrono::Utc::now();
    rustrak::models::AlertIntegration {
        id: 1,
        name: "t".into(),
        provider_type: kind,
        credentials,
        is_enabled: true,
        failure_count: 0,
        last_failure_at: None,
        last_failure_message: None,
        last_success_at: None,
        created_at: now,
        updated_at: now,
    }
}

fn payload() -> rustrak::models::AlertPayload {
    let now = chrono::Utc::now();
    rustrak::models::AlertPayload {
        alert_id: "a".into(),
        alert_type: "new_issue".into(),
        triggered_at: now,
        project: rustrak::models::ProjectInfo {
            id: 1,
            name: "p".into(),
            slug: "p".into(),
        },
        issue: rustrak::models::IssueInfo {
            id: "i".into(),
            short_id: "P-1".into(),
            title: "t".into(),
            level: None,
            first_seen: now,
            last_seen: now,
            event_count: 1,
        },
        issue_url: "https://x.test/i".into(),
        actor: "BugLenz".into(),
    }
}

#[actix_web::test]
async fn sending_to_an_internal_routing_override_fails_without_connecting() {
    for (kind, creds) in [
        (
            ChannelType::Webhook,
            json!({ "url": "https://hooks.example.com/ok" }),
        ),
        (
            ChannelType::CustomWebhook,
            json!({ "url": "https://hooks.example.com/ok", "template": TEMPLATE }),
        ),
    ] {
        let result = create_dispatcher(kind)
            .send(
                &integration(kind, creds),
                &json!({ "url": "http://169.254.169.254/latest/meta-data/" }),
                &payload(),
            )
            .await;
        assert!(!result.success, "{kind:?}");
        assert!(
            result
                .error_message
                .unwrap_or_default()
                .contains("internal or private"),
            "{kind:?}: the failure must name the reason, not a connection error"
        );
    }
}
