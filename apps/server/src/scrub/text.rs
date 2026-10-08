//! Masks for personal data inside free text: CPF, CNPJ, card numbers and
//! e-mail addresses become `[cpf]`, `[cnpj]`, `[cartao]`, `[email]`.
//!
//! Hand-written scanners rather than regular expressions: `regex` is not a
//! direct dependency of this crate, and the patterns are small. Numbers are
//! only masked when their check digits (CPF, CNPJ) or Luhn (cards) hold, so
//! a timestamp, a bundle hash or a phone number stays as it is.

use std::borrow::Cow;

pub const CPF: &str = "[cpf]";
pub const CNPJ: &str = "[cnpj]";
pub const CARD: &str = "[cartao]";
pub const EMAIL: &str = "[email]";

/// Masks every recognised value in `text`. Returns the input untouched (no
/// allocation) when there is nothing to mask.
pub fn scrub_text(text: &str) -> Cow<'_, str> {
    let numbers = mask_numbers(text);
    let both = mask_emails(&numbers);
    if numbers.as_ref() == text && both.as_ref() == numbers.as_ref() {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(both.into_owned())
    }
}

// ---------------------------------------------------------------------------
// numbers
// ---------------------------------------------------------------------------

fn is_sep(c: char) -> bool {
    matches!(c, '.' | '-' | '/' | ' ')
}

/// A run of digits with at most one separator between digits, delimited by
/// characters that are neither digits nor letters.
struct Run {
    start: usize,
    end: usize,
    digits: Vec<u8>,
    /// Digit groups as written, for the card shape check.
    groups: Vec<usize>,
    separators: Vec<char>,
}

fn find_runs(text: &str) -> Vec<Run> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut runs = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let (start, c) = chars[i];
        if !c.is_ascii_digit() {
            i += 1;
            continue;
        }
        // Left boundary: the previous char must not be a digit or a letter.
        if i > 0 && chars[i - 1].1.is_alphanumeric() {
            i += 1;
            while i < chars.len() && chars[i].1.is_ascii_digit() {
                i += 1;
            }
            continue;
        }
        let mut digits = Vec::new();
        let mut groups = Vec::new();
        let mut separators = Vec::new();
        let mut group = 0usize;
        let mut j = i;
        let mut end = start;
        while j < chars.len() {
            let (pos, ch) = chars[j];
            if ch.is_ascii_digit() {
                digits.push(ch as u8 - b'0');
                group += 1;
                end = pos + ch.len_utf8();
                j += 1;
            } else if is_sep(ch)
                && j + 1 < chars.len()
                && chars[j + 1].1.is_ascii_digit()
                && group > 0
            {
                groups.push(group);
                group = 0;
                separators.push(ch);
                j += 1;
            } else {
                break;
            }
        }
        groups.push(group);
        // Right boundary: the next char must not be a letter.
        let bounded = j >= chars.len() || !chars[j].1.is_alphanumeric();
        if bounded && !digits.is_empty() {
            runs.push(Run {
                start,
                end,
                digits,
                groups,
                separators,
            });
        }
        i = j.max(i + 1);
    }
    runs
}

fn mask_numbers(text: &str) -> Cow<'_, str> {
    let runs = find_runs(text);
    let mut out: Option<String> = None;
    let mut last = 0;
    for run in &runs {
        let mask = classify(run);
        let Some(mask) = mask else { continue };
        let buf = out.get_or_insert_with(|| String::with_capacity(text.len()));
        buf.push_str(&text[last..run.start]);
        buf.push_str(mask);
        last = run.end;
    }
    match out {
        None => Cow::Borrowed(text),
        Some(mut buf) => {
            buf.push_str(&text[last..]);
            Cow::Owned(buf)
        }
    }
}

fn classify(run: &Run) -> Option<&'static str> {
    let d = &run.digits;
    match d.len() {
        11 if looks_like_cpf(run) && valid_cpf(d) => Some(CPF),
        14 if looks_like_cnpj(run) && valid_cnpj(d) => Some(CNPJ),
        13..=19 if looks_like_card(run) && luhn(d) => Some(CARD),
        _ => None,
    }
}

/// `12345678909`, `123.456.789-09`, `123456789-09`: no spaces, no slash.
fn looks_like_cpf(run: &Run) -> bool {
    run.separators.iter().all(|c| matches!(c, '.' | '-'))
        && (run.groups.len() == 1 || run.groups == [3, 3, 3, 2] || run.groups == [9, 2])
}

/// `12345678000195`, `12.345.678/0001-95`.
fn looks_like_cnpj(run: &Run) -> bool {
    run.separators.iter().all(|c| matches!(c, '.' | '-' | '/'))
        && (run.groups.len() == 1 || run.groups == [2, 3, 3, 4, 2] || run.groups == [8, 4, 2])
}

/// All digits, or groups of at least four digits separated by single spaces
/// or hyphens (`4111 1111 1111 1111`, `3782-822463-10005`). Two-digit groups
/// are what phone numbers look like, so they are left alone.
fn looks_like_card(run: &Run) -> bool {
    run.separators.iter().all(|c| matches!(c, ' ' | '-'))
        && (run.groups.len() == 1 || run.groups.iter().all(|g| *g >= 4))
}

fn valid_cpf(d: &[u8]) -> bool {
    if d.len() != 11 || d.iter().all(|x| *x == d[0]) {
        return false;
    }
    let dv = |n: usize| -> u8 {
        let sum: usize = d[..n]
            .iter()
            .enumerate()
            .map(|(i, x)| (*x as usize) * (n + 1 - i))
            .sum();
        let r = (sum * 10) % 11;
        if r == 10 {
            0
        } else {
            r as u8
        }
    };
    dv(9) == d[9] && dv(10) == d[10]
}

fn valid_cnpj(d: &[u8]) -> bool {
    if d.len() != 14 || d.iter().all(|x| *x == d[0]) {
        return false;
    }
    const W1: [usize; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    const W2: [usize; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let dv = |w: &[usize]| -> u8 {
        let sum: usize = d.iter().zip(w).map(|(x, w)| (*x as usize) * w).sum();
        let r = sum % 11;
        if r < 2 {
            0
        } else {
            (11 - r) as u8
        }
    };
    dv(&W1) == d[12] && dv(&W2) == d[13]
}

fn luhn(d: &[u8]) -> bool {
    let mut sum = 0u32;
    for (i, x) in d.iter().rev().enumerate() {
        let mut v = *x as u32;
        if i % 2 == 1 {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
    }
    sum.is_multiple_of(10)
}

// ---------------------------------------------------------------------------
// e-mail
// ---------------------------------------------------------------------------

fn is_local(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '%' | '+' | '-')
}

fn is_domain(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '-')
}

fn mask_emails(text: &str) -> Cow<'_, str> {
    let bytes = text.as_bytes();
    let mut out: Option<String> = None;
    let mut last = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'@' {
            i += 1;
            continue;
        }
        // Local part: extend left over local characters.
        let mut s = i;
        while s > 0 && is_local(bytes[s - 1] as char) {
            s -= 1;
        }
        // Domain: extend right over domain characters, then trim trailing dots.
        let mut e = i + 1;
        while e < bytes.len() && is_domain(bytes[e] as char) {
            e += 1;
        }
        while e > i + 1 && bytes[e - 1] == b'.' {
            e -= 1;
        }
        let local = &text[s..i];
        let domain = &text[i + 1..e];
        let tld_ok = domain
            .rsplit('.')
            .next()
            .is_some_and(|t| t.len() >= 2 && t.bytes().all(|b| b.is_ascii_alphabetic()));
        if !local.is_empty() && domain.contains('.') && tld_ok && s >= last {
            let buf = out.get_or_insert_with(|| String::with_capacity(text.len()));
            buf.push_str(&text[last..s]);
            buf.push_str(EMAIL);
            last = e;
            i = e;
        } else {
            i += 1;
        }
    }
    match out {
        None => Cow::Borrowed(text),
        Some(mut buf) => {
            buf.push_str(&text[last..]);
            Cow::Owned(buf)
        }
    }
}
