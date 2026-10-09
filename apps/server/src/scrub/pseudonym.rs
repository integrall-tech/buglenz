//! The distinct-user id of a session (`did`), stored as a keyed pseudonym (BugLenz, ADR-0009,
//! invariant I4).
//!
//! An SDK builds `did` from `user.id`, or, when that is missing, from the user's e-mail, username
//! or IP address. The session tables only need to tell users apart, never to say who they are, so
//! the value is replaced before it is counted by an HMAC-SHA256 of it, keyed with the instance's
//! `SESSION_SECRET_KEY`. Masking it like text (`[email]`) would collapse every e-mail into one user
//! and break the distinct-user counts; a plain hash of an e-mail can be reversed by trying
//! addresses, the key is what prevents that.
//!
//! Rotating `SESSION_SECRET_KEY` changes every pseudonym: users already counted that day count
//! again. The same function turns an erasure request's `user_id` into what the table holds.

use std::sync::OnceLock;

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// What every pseudonym starts with, so a stored value can be told from a raw one.
pub const PREFIX: &str = "p1:";

/// Domain separation: the key is shared with the cookie sessions, this tag is not.
const DOMAIN: &[u8] = b"buglenz-session-did/v1\0";

type HmacSha256 = Hmac<Sha256>;

/// The pseudonym of `did` under `key`: [`PREFIX`] and 32 hex digits (128 bits).
pub fn pseudonym_with(key: &[u8], did: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take a key of any size");
    mac.update(DOMAIN);
    mac.update(did.as_bytes());
    let digest = mac.finalize().into_bytes();
    format!("{PREFIX}{}", hex::encode(&digest[..16]))
}

fn instance_key() -> &'static [u8] {
    static KEY: OnceLock<Vec<u8>> = OnceLock::new();
    KEY.get_or_init(|| {
        std::env::var("SESSION_SECRET_KEY")
            .unwrap_or_default()
            .into_bytes()
    })
}

/// The pseudonym of `did` under the instance's key.
pub fn pseudonym(did: &str) -> String {
    pseudonym_with(instance_key(), did)
}
