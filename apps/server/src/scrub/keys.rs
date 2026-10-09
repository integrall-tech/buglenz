//! Keys whose values never reach the database.
//!
//! A key is compared in a normalised form: lowercase, with `_`, `-`, spaces
//! and dots removed, so `Set-Cookie`, `set_cookie` and `setcookie` are one
//! key. Two lists apply: exact matches, and substrings that mark a key as
//! sensitive wherever they appear (`accessToken`, `x-api-key`,
//! `db_password`). Short words such as `auth` or `session` are exact only,
//! or `author` and `session_replay_id` would be lost.
//!
//! `RUSTRAK_SCRUB_EXTRA_KEYS` adds exact keys per instance, comma-separated,
//! normalised the same way. There is no switch that turns the list off.

use std::collections::HashSet;
use std::sync::OnceLock;

/// Normalised keys filtered wherever they appear.
const EXACT: &[&str] = &[
    "password",
    "passwd",
    "pwd",
    "senha",
    "secret",
    "token",
    "accesstoken",
    "refreshtoken",
    "idtoken",
    "authorization",
    "auth",
    "cookie",
    "setcookie",
    "session",
    "sessionid",
    "csrf",
    "csrftoken",
    "xsrftoken",
    "apikey",
    "privatekey",
    "credentials",
    "creditcard",
    "cardnumber",
    "cartao",
    "cpf",
    "cnpj",
    "rg",
    "ipaddress",
    "remoteaddr",
    "xforwardedfor",
    "xrealip",
];

/// Substrings that make any key sensitive: long enough not to collide with
/// ordinary words.
const CONTAINS: &[&str] = &[
    "password",
    "passwd",
    "senha",
    "secret",
    "token",
    "authorization",
    "cookie",
    "apikey",
    "privatekey",
    "creditcard",
    "cardnumber",
];

/// Keys whose string values are identifiers or timestamps, never free text:
/// the masks skip them, or a span id made of digits that happens to pass
/// Luhn would turn into `[cartao]` and collide with its siblings. Compared on
/// the normalised key: anything ending in `id`, plus the time and version
/// fields of the protocol.
pub fn is_identifier(key: &str) -> bool {
    let key = normalise(key);
    key.ends_with("id")
        || matches!(
            key.as_str(),
            "timestamp" | "starttimestamp" | "endtimestamp" | "sentat" | "release" | "dist"
        )
}

/// Whether `key` names an id (`id`, `user_id`, `spanId`...), as opposed to the other identifier
/// fields (timestamps, release, dist).
pub fn is_id_key(key: &str) -> bool {
    normalise(key).ends_with("id")
}

/// The environment variable that extends the exact list per instance.
pub const EXTRA_KEYS_VAR: &str = "RUSTRAK_SCRUB_EXTRA_KEYS";

/// Lowercase, without the separators people disagree about.
pub fn normalise(key: &str) -> String {
    key.chars()
        .filter(|c| !matches!(c, '_' | '-' | ' ' | '.'))
        .flat_map(char::to_lowercase)
        .collect()
}

fn extra_keys() -> &'static HashSet<String> {
    static EXTRA: OnceLock<HashSet<String>> = OnceLock::new();
    EXTRA.get_or_init(|| parse_extra(std::env::var(EXTRA_KEYS_VAR).ok().as_deref()))
}

/// The extra list as it is parsed from the variable: comma-separated,
/// blanks ignored.
pub fn parse_extra(raw: Option<&str>) -> HashSet<String> {
    raw.unwrap_or("")
        .split(',')
        .map(normalise)
        .filter(|k| !k.is_empty())
        .collect()
}

/// Whether a JSON key's value must be replaced.
pub fn is_denied(key: &str) -> bool {
    is_denied_with(key, extra_keys())
}

/// Same as [`is_denied`], against an explicit extra list; what the tests use
/// so they do not depend on the process environment.
pub fn is_denied_with(key: &str, extra: &HashSet<String>) -> bool {
    let key = normalise(key);
    if key.is_empty() {
        return false;
    }
    EXACT.contains(&key.as_str()) || matches_by_content(&key) || extra.contains(&key)
}

/// The substring rules, minus the one collision that matters: `tokens` is a
/// count (`gen_ai.usage.input_tokens`, `max_tokens`), never a credential.
fn matches_by_content(key: &str) -> bool {
    CONTAINS
        .iter()
        .any(|needle| key.contains(needle) && !(*needle == "token" && key.contains("tokens")))
}
