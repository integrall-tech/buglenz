//! Personal data never reaches the database (BugLenz, ADR-0009, invariant I4).
//!
//! The digest calls [`scrub_value`] on every payload before grouping and
//! before any insert: values under denied keys become `"[Filtered]"`, and
//! CPF, CNPJ, card numbers and e-mail addresses inside free text become
//! `[cpf]`, `[cnpj]`, `[cartao]`, `[email]`. The source address is not read
//! at ingestion at all (`routes/ingest.rs`). There is no switch: the SDK side
//! (`sendDefaultPii`, `beforeSend`) is the first layer, this is the second.

pub mod keys;
pub mod pseudonym;
pub mod text;

pub use keys::{is_denied, EXTRA_KEYS_VAR};
pub use text::{mask_document_number, scrub_emails, scrub_text};

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
            mask_email_keys(map);
            for (key, child) in map.iter_mut() {
                if denied(key) {
                    *child = Value::String(FILTERED.to_string());
                } else if child.is_number() && keys::is_identifier(key) {
                    // A numeric id that happens to pass a check digit is still an id.
                } else if child.is_string() && keys::is_identifier(key) {
                    // Identifiers and timestamps are not free text: no number mask (a Luhn-valid
                    // span id would turn into `[cartao]`). An `*id` value is still checked for an
                    // e-mail, because an SDK builds `user.id` from the e-mail when it has no id.
                    if keys::is_id_key(key) {
                        if let Value::String(s) = child {
                            if let std::borrow::Cow::Owned(masked) = scrub_emails(s) {
                                *s = masked;
                            }
                        }
                    }
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
        Value::Number(n) => {
            // A CPF or CNPJ sent as a JSON number (a key with no name that says "document").
            if let Some(mask) = n.as_u64().and_then(mask_document_number) {
                *value = Value::String(mask.to_string());
            }
        }
        _ => {}
    }
}

/// An e-mail used as an object key (tags are a map, and some apps key by user) is personal data
/// too. The key is masked; two keys that mask to the same text keep both, the later ones numbered.
fn mask_email_keys(map: &mut serde_json::Map<String, Value>) {
    let renames: Vec<(String, String)> = map
        .keys()
        .filter_map(|k| match scrub_emails(k) {
            std::borrow::Cow::Owned(masked) => Some((k.clone(), masked)),
            std::borrow::Cow::Borrowed(_) => None,
        })
        .collect();
    for (old, masked) in renames {
        let Some(value) = map.remove(&old) else {
            continue;
        };
        let mut new = masked.clone();
        let mut n = 2;
        while map.contains_key(&new) {
            new = format!("{masked} ({n})");
            n += 1;
        }
        map.insert(new, value);
    }
}
