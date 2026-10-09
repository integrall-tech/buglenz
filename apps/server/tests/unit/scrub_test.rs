//! The scrubbing layer: denied keys, text masks and the recursive walk.

use std::collections::HashSet;

use rustrak::scrub::keys::{is_denied_with, normalise, parse_extra};
use rustrak::scrub::{scrub_text, scrub_with, FILTERED};
use serde_json::{json, Value};

fn denied(key: &str) -> bool {
    is_denied_with(key, &HashSet::new())
}

// =============================================================================
// Keys
// =============================================================================

#[test]
fn keys_are_compared_without_case_or_separators() {
    assert_eq!(normalise("Set-Cookie"), "setcookie");
    assert_eq!(normalise("x_api.key "), "xapikey");
    for key in [
        "password",
        "Password",
        "PASSWORD",
        "senha",
        "Authorization",
        "Set-Cookie",
        "cookie",
        "x-api-key",
        "X-API-Key",
        "cpf",
        "CNPJ",
        "ip_address",
        "X-Forwarded-For",
        "remote_addr",
        "credit_card",
    ] {
        assert!(denied(key), "{key} must be denied");
    }
}

#[test]
fn long_markers_deny_by_substring_short_ones_only_exactly() {
    assert!(denied("accessToken"));
    assert!(denied("db_password"));
    assert!(denied("stripeSecretKey"));
    assert!(denied("auth"));
    assert!(denied("session"));
    // Ordinary words that merely contain a short marker stay.
    assert!(!denied("author"));
    // Token counts are numbers about usage, not credentials.
    assert!(!denied("gen_ai.usage.input_tokens"));
    assert!(!denied("max_tokens"));
    assert!(!denied("gen_ai.usage.total_tokens"));
    assert!(!denied("session_replay_id"));
    assert!(!denied("authors"));
    assert!(!denied("message"));
    assert!(!denied("transaction"));
    assert!(!denied(""));
}

#[test]
fn extra_keys_come_from_the_variable_normalised() {
    let extra = parse_extra(Some("documento, Telefone ,,"));
    assert!(is_denied_with("documento", &extra));
    assert!(is_denied_with("TELEFONE", &extra));
    assert!(is_denied_with("tele_fone", &extra));
    assert!(!is_denied_with("endereco", &extra));
    assert!(parse_extra(None).is_empty());
}

// =============================================================================
// Text masks
// =============================================================================

#[test]
fn valid_cpfs_are_masked_in_any_usual_shape() {
    assert_eq!(scrub_text("cpf 529.982.247-25 ok"), "cpf [cpf] ok");
    assert_eq!(scrub_text("52998224725"), "[cpf]");
    assert_eq!(scrub_text("529982247-25"), "[cpf]");
}

#[test]
fn numbers_that_fail_the_check_digits_stay() {
    assert_eq!(scrub_text("529.982.247-26"), "529.982.247-26");
    assert_eq!(scrub_text("11111111111"), "11111111111");
    assert_eq!(scrub_text("pedido 20261008123"), "pedido 20261008123");
    // A timestamp or an id glued to letters is never a document.
    assert_eq!(scrub_text("2026-10-08T12:34:56Z"), "2026-10-08T12:34:56Z");
    assert_eq!(scrub_text("id-52998224725x"), "id-52998224725x");
}

#[test]
fn valid_cnpjs_are_masked() {
    assert_eq!(scrub_text("11.222.333/0001-81"), "[cnpj]");
    assert_eq!(scrub_text("11222333000181"), "[cnpj]");
    assert_eq!(scrub_text("11.222.333/0001-82"), "11.222.333/0001-82");
}

#[test]
fn card_numbers_need_luhn_and_a_card_shape() {
    assert_eq!(scrub_text("cartao 4111 1111 1111 1111"), "cartao [cartao]");
    assert_eq!(scrub_text("4111111111111111"), "[cartao]");
    assert_eq!(scrub_text("3782-822463-10005"), "[cartao]");
    assert_eq!(scrub_text("4111 1111 1111 1112"), "4111 1111 1111 1112");
    // Phone numbers: short groups, never a card even when Luhn happens to hold.
    assert_eq!(scrub_text("+55 11 91234 5678"), "+55 11 91234 5678");
}

#[test]
fn emails_are_masked_and_the_rest_of_the_text_kept() {
    assert_eq!(
        scrub_text("usuario ana.silva+x@example.com.br falhou"),
        "usuario [email] falhou"
    );
    assert_eq!(scrub_text("a@b.co, c@d.org."), "[email], [email].");
    assert_eq!(scrub_text("sem arroba"), "sem arroba");
    assert_eq!(scrub_text("@handle"), "@handle");
    assert_eq!(scrub_text("x@localhost"), "x@localhost");
}

#[test]
fn masks_compose_and_are_idempotent() {
    let once = scrub_text("ana@example.com cpf 529.982.247-25 cartao 4111111111111111");
    assert_eq!(once, "[email] cpf [cpf] cartao [cartao]");
    let twice = scrub_text(&once);
    assert_eq!(twice, once);
}

#[test]
fn clean_text_is_returned_without_copying() {
    let text = "nothing to see here 123";
    assert!(matches!(scrub_text(text), std::borrow::Cow::Borrowed(_)));
}

// =============================================================================
// The walk
// =============================================================================

fn event() -> Value {
    json!({
        "event_id": "abc",
        "message": "login de ana@example.com com cpf 529.982.247-25 falhou",
        "user": { "id": "u-1", "email": "ana@example.com", "ip_address": "10.0.0.1" },
        "request": {
            "url": "https://app.example.com/pedidos?token=abc",
            "headers": { "Cookie": "sid=1", "Authorization": "Bearer x", "Accept": "*/*" },
            "data": { "password": "hunter2", "nome": "Ana" }
        },
        "extra": { "tentativas": 3, "cartoes": ["4111 1111 1111 1111", "nope"] },
        "exception": { "values": [{ "type": "TypeError", "value": "cpf 52998224725 invalido",
            "stacktrace": { "frames": [{ "filename": "src/a.ts", "context_line": "const senha = 'x'" }] } }] }
    })
}

#[test]
fn denied_keys_are_filtered_at_every_depth_and_text_is_masked() {
    let mut v = event();
    scrub_with(&mut v, &denied);
    assert_eq!(v["user"]["id"], "u-1");
    assert_eq!(v["user"]["email"], "[email]");
    assert_eq!(v["user"]["ip_address"], FILTERED);
    assert_eq!(v["request"]["headers"]["Cookie"], FILTERED);
    assert_eq!(v["request"]["headers"]["Authorization"], FILTERED);
    assert_eq!(v["request"]["headers"]["Accept"], "*/*");
    assert_eq!(v["request"]["data"]["password"], FILTERED);
    assert_eq!(v["request"]["data"]["nome"], "Ana");
    assert_eq!(v["extra"]["tentativas"], 3);
    assert_eq!(v["extra"]["cartoes"][0], "[cartao]");
    assert_eq!(v["extra"]["cartoes"][1], "nope");
    assert_eq!(v["message"], "login de [email] com cpf [cpf] falhou");
    assert_eq!(v["exception"]["values"][0]["value"], "cpf [cpf] invalido");
    assert_eq!(
        v["exception"]["values"][0]["stacktrace"]["frames"][0]["filename"],
        "src/a.ts"
    );
    // The URL's query carries a token as text, not as a key: kept as is.
    assert_eq!(
        v["request"]["url"],
        "https://app.example.com/pedidos?token=abc"
    );
}

#[test]
fn identifiers_and_timestamps_are_never_masked() {
    // 4111111111111111 passes Luhn; as a span id it must stay a span id.
    let mut v = json!({
        "span_id": "4111111111111111", "trace_id": "52998224725", "event_id": "ab",
        "user": { "id": "52998224725" }, "sid": "4111111111111111",
        "timestamp": "4111111111111111", "release": "4111111111111111",
        "message": "cartao 4111111111111111"
    });
    scrub_with(&mut v, &denied);
    assert_eq!(v["span_id"], "4111111111111111");
    assert_eq!(v["trace_id"], "52998224725");
    assert_eq!(v["user"]["id"], "52998224725");
    assert_eq!(v["sid"], "4111111111111111");
    assert_eq!(v["timestamp"], "4111111111111111");
    assert_eq!(v["release"], "4111111111111111");
    assert_eq!(v["message"], "cartao [cartao]");
}

#[test]
fn a_denied_key_loses_its_whole_subtree_whatever_the_type() {
    let mut v =
        json!({ "credentials": { "user": "a", "pass": "b" }, "token": 12345, "secret": [1, 2] });
    scrub_with(&mut v, &denied);
    assert_eq!(v["credentials"], FILTERED);
    assert_eq!(v["token"], FILTERED);
    assert_eq!(v["secret"], FILTERED);
}

#[test]
fn scrubbing_is_idempotent_and_leaves_clean_payloads_alone() {
    let mut once = event();
    scrub_with(&mut once, &denied);
    let mut twice = once.clone();
    scrub_with(&mut twice, &denied);
    assert_eq!(once, twice);

    let clean = json!({ "level": "error", "platform": "javascript", "tags": { "env": "prod" } });
    let mut copy = clean.clone();
    scrub_with(&mut copy, &denied);
    assert_eq!(copy, clean);
}

// An SDK may build an id from the user's e-mail (the JavaScript one does when there is no id), so the
// identifier exemption must not let an e-mail through. Only e-mail is masked in `*id` keys: a number
// that looks like a CPF or a card is far more likely a real id there (audit of 2026-10-09, I4).
#[test]
fn an_email_in_an_id_field_is_masked() {
    let mut v = json!({
        "user": { "id": "ana@example.com" },
        "customer_id": "bia@example.com.br",
        "contexts": { "trace": { "span_id": "a1b2c3d4e5f60718" } }
    });
    scrub_with(&mut v, &denied);
    assert_eq!(v["user"]["id"], "[email]");
    assert_eq!(v["customer_id"], "[email]");
    assert_eq!(v["contexts"]["trace"]["span_id"], "a1b2c3d4e5f60718");
}

#[test]
fn an_id_that_is_not_an_email_is_left_alone() {
    let mut v = json!({
        "user": { "id": "u-42" }, "trace_id": "52998224725", "span_id": "4111111111111111",
        "order_id": "no-reply-12", "valid": "ok", "paid": "4111111111111111"
    });
    let before = v.clone();
    scrub_with(&mut v, &denied);
    assert_eq!(v, before);
}

#[test]
fn a_release_is_not_an_id_and_keeps_its_at_sign() {
    // `release` is an identifier for the number masks, but it is not an `*id` key, and its
    // name@version shape must survive either way.
    let mut v = json!({ "release": "vendax-web@1.4.2", "dist": "7" });
    let before = v.clone();
    scrub_with(&mut v, &denied);
    assert_eq!(v, before);
}
