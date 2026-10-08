//! Where a webhook may point (ADR-0018, H-4).
//!
//! A webhook URL is typed by an administrator, but it is the server that connects to it, from
//! inside the network. Without a check, a channel aimed at the cloud metadata address, a
//! database or an admin console makes the server fetch it and show the answer in the delivery
//! log. The check is on the host as written: it does not resolve names, so a public name that
//! resolves to a private address still passes. The network policy of the host (egress allow
//! list, invariant I3) is what closes that.
//!
//! Some destinations are internal on purpose (a chat server or an automation agent on the same
//! network). `RUSTRAK_WEBHOOK_ALLOWED_HOSTS` lists them, comma-separated, as the host is written in
//! the URL (a name or an address). It is empty by default and there is no switch that turns the
//! check off.

use std::collections::HashSet;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::OnceLock;

use crate::error::{AppError, AppResult};

const MESSAGE: &str = "Webhook URL must not target internal or private addresses";

/// The environment variable that exempts specific internal hosts.
pub const ALLOWED_HOSTS_VAR: &str = "RUSTRAK_WEBHOOK_ALLOWED_HOSTS";

fn allowed_hosts() -> &'static HashSet<String> {
    static ALLOWED: OnceLock<HashSet<String>> = OnceLock::new();
    ALLOWED.get_or_init(|| parse_allowed(std::env::var(ALLOWED_HOSTS_VAR).ok().as_deref()))
}

/// The exemption list as it is parsed from the variable: comma-separated, lowercase, blanks and
/// trailing dots ignored.
pub fn parse_allowed(raw: Option<&str>) -> HashSet<String> {
    raw.unwrap_or("")
        .split(',')
        .map(|h| h.trim().trim_end_matches('.').to_ascii_lowercase())
        .filter(|h| !h.is_empty())
        .collect()
}

/// Rejects a URL whose host is a loopback, private, link-local or otherwise internal address, or
/// a name that is internal by convention. A URL that does not parse is left to the caller's own
/// format check.
pub fn check_url(raw: &str) -> AppResult<()> {
    check_url_with(raw, allowed_hosts())
}

/// Same as [`check_url`], against an explicit exemption list; what the tests use so they do not
/// depend on the process environment.
pub fn check_url_with(raw: &str, allowed: &HashSet<String>) -> AppResult<()> {
    let Ok(url) = url::Url::parse(raw) else {
        return Ok(());
    };
    let (blocked, written) = match url.host() {
        Some(url::Host::Ipv4(ip)) => (is_internal_v4(ip), ip.to_string()),
        Some(url::Host::Ipv6(ip)) => (is_internal_v6(ip), ip.to_string()),
        Some(url::Host::Domain(name)) => (
            is_internal_name(name),
            name.trim_end_matches('.').to_ascii_lowercase(),
        ),
        None => (false, String::new()),
    };
    if blocked && !allowed.contains(&written) {
        return Err(AppError::Validation(MESSAGE.to_string()));
    }
    Ok(())
}

fn is_internal_name(name: &str) -> bool {
    let lower = name.trim_end_matches('.').to_ascii_lowercase();
    lower == "localhost"
        || lower == "ip6-localhost"
        || lower == "ip6-loopback"
        || lower.ends_with(".localhost")
        || lower.ends_with(".local")
        || lower.ends_with(".internal")
}

fn is_internal_v4(ip: Ipv4Addr) -> bool {
    let [a, b, ..] = ip.octets();
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_unspecified()
        || a == 0
        // 100.64.0.0/10, carrier-grade NAT: reachable from inside many networks.
        || (a == 100 && (64..=127).contains(&b))
}

fn is_internal_v6(ip: Ipv6Addr) -> bool {
    if let Some(v4) = ip.to_ipv4_mapped() {
        return is_internal_v4(v4);
    }
    let first = ip.segments()[0];
    ip.is_loopback()
        || ip.is_unspecified()
        // fc00::/7, unique local.
        || first & 0xfe00 == 0xfc00
        // fe80::/10, link-local.
        || first & 0xffc0 == 0xfe80
}
