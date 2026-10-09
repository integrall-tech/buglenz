//! The distinct-user id of a session (`did`) is stored as a keyed pseudonym (ADR-0009, invariant I4).

use rustrak::scrub::pseudonym::{pseudonym_with, PREFIX};

#[test]
fn the_same_input_gives_the_same_pseudonym_and_another_input_another() {
    let key = b"k";
    assert_eq!(
        pseudonym_with(key, "ana@example.com"),
        pseudonym_with(key, "ana@example.com")
    );
    assert_ne!(
        pseudonym_with(key, "ana@example.com"),
        pseudonym_with(key, "bia@example.com")
    );
}

#[test]
fn the_pseudonym_does_not_carry_the_input_and_has_a_fixed_shape() {
    for input in ["ana@example.com", "529.982.247-25", "u-42", "10.1.2.3", ""] {
        let p = pseudonym_with(b"k", input);
        assert!(p.starts_with(PREFIX), "{p}");
        assert_eq!(p.len(), PREFIX.len() + 32, "{p}");
        assert!(
            p[PREFIX.len()..].chars().all(|c| c.is_ascii_hexdigit()),
            "{p}"
        );
        if !input.is_empty() {
            assert!(!p.contains(input), "{p} contains {input}");
        }
    }
}

#[test]
fn another_key_gives_another_pseudonym() {
    assert_ne!(
        pseudonym_with(b"one", "u-42"),
        pseudonym_with(b"two", "u-42")
    );
}
