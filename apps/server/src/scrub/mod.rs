//! Personal data never reaches the database (BugLenz, ADR-0009, invariant I4).
//!
//! The digest calls [`scrub_value`] on every payload before grouping and
//! before any insert: values under denied keys become `"[Filtered]"`, and
//! CPF, CNPJ, card numbers and e-mail addresses inside free text become
//! `[cpf]`, `[cnpj]`, `[cartao]`, `[email]`. The source address is not read
//! at ingestion at all (`routes/ingest.rs`). There is no switch: the SDK side
//! (`sendDefaultPii`, `beforeSend`) is the first layer, this is the second.

pub mod keys;
pub mod text;

pub use keys::{is_denied, EXTRA_KEYS_VAR};
pub use text::scrub_text;

use serde_json::Value;

/// What replaces the value of a denied key.
pub const FILTERED: &str = "[Filtered]";

/// Scrubs `value` in place, at every depth. Idempotent.
pub fn scrub_value(value: &mut Value) {
    scrub_with(value, &|key| is_denied(key));
}

/// [`scrub_value`] with an explicit key predicate, for tests and callers that
/// hold their own list.
pub fn scrub_with(value: &mut Value, denied: &dyn Fn(&str) -> bool) {
    match value {
        Value::Object(map) => {
            for (key, child) in map.iter_mut() {
                if denied(key) {
                    *child = Value::String(FILTERED.to_string());
                } else {
                    scrub_with(child, denied);
                }
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                scrub_with(item, denied);
            }
        }
        Value::String(s) => {
            if let std::borrow::Cow::Owned(masked) = scrub_text(s) {
                *s = masked;
            }
        }
        _ => {}
    }
}
