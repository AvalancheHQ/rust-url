//! Benchmarks for the `idna` crate: domain name conversion to ASCII (Punycode)
//! and back to Unicode.

use divan::{black_box, Bencher};
use idna::AsciiDenyList;

fn main() {
    divan::main();
}

/// Domains covering the interesting shapes: pure ASCII (fast path), mixed
/// scripts, already encoded Punycode, left-to-right and right-to-left scripts.
const DOMAINS: &[(&str, &str)] = &[
    ("ascii", "example.com"),
    ("ascii-hyphen", "hyphenated-example.com"),
    ("ascii-leading-digit", "1test.example"),
    ("ascii-uppercase", "EXAMPLE.COM"),
    ("ascii-long", "a.very.deeply.nested.sub.domain.example.com"),
    ("unicode-mixed", "مثال.example"),
    ("punycode-mixed", "xn--mgbh0fb.example"),
    ("unicode-ltr", "නම.උදාහරණ"),
    ("punycode-ltr", "xn--r0co.xn--ozc8dl2c3bxd"),
    ("unicode-rtl", "الاسم.مثال"),
    ("punycode-rtl", "xn--mgba0b1dh.xn--mgbh0fb"),
    ("unicode-latin", "beispiel.vermögensberater"),
    ("punycode-latin", "beispiel.xn--vermgensberater-ctb"),
    ("unicode-cjk", "例え.テスト"),
    ("unicode-emoji-adjacent", "☃.example"),
];

fn domain(name: &str) -> &'static str {
    DOMAINS
        .iter()
        .find(|(case, _)| *case == name)
        .expect("unknown case")
        .1
}

/// The allocation-free entry point used by the `url` crate.
#[divan::bench(args = DOMAINS.iter().map(|(name, _)| *name))]
fn domain_to_ascii_cow(bencher: Bencher, name: &str) {
    let bytes = domain(name).as_bytes();
    bencher.bench(|| idna::domain_to_ascii_cow(black_box(bytes), AsciiDenyList::URL));
}

/// The `String`-returning, WHATWG-compliant conversion.
#[divan::bench(args = ["ascii", "unicode-mixed", "punycode-mixed", "unicode-rtl", "unicode-cjk"])]
fn domain_to_ascii(bencher: Bencher, name: &str) {
    let domain = domain(name);
    bencher.bench(|| idna::domain_to_ascii(black_box(domain)).unwrap());
}

/// The stricter (STD3 rules) variant.
#[divan::bench(args = ["ascii", "unicode-mixed", "punycode-mixed", "unicode-latin"])]
fn domain_to_ascii_strict(bencher: Bencher, name: &str) {
    let domain = domain(name);
    bencher.bench(|| idna::domain_to_ascii_strict(black_box(domain)).unwrap());
}

/// Decoding back to Unicode.
#[divan::bench(args = ["ascii", "punycode-mixed", "punycode-ltr", "punycode-rtl", "punycode-latin"])]
fn domain_to_unicode(bencher: Bencher, name: &str) {
    let domain = domain(name);
    bencher.bench(|| idna::domain_to_unicode(black_box(domain)));
}

/// Domains that must be rejected, exercising the error paths.
#[divan::bench(args = ["invalid-punycode", "invalid-punycode-label"])]
fn domain_to_ascii_invalid(bencher: Bencher, kind: &str) {
    let domain = match kind {
        "invalid-punycode" => "xn--a-ecp.example",
        "invalid-punycode-label" => "xn--u-ccb.com",
        _ => unreachable!(),
    };
    bencher.bench(|| idna::domain_to_ascii(black_box(domain)).unwrap_err());
}

/// Domains rejected only by the STD3 rules of the strict conversion.
#[divan::bench(args = ["empty-label", "disallowed-character", "leading-hyphen"])]
fn domain_to_ascii_strict_invalid(bencher: Bencher, kind: &str) {
    let domain = match kind {
        "empty-label" => "example..com",
        "disallowed-character" => "exa mple.com",
        "leading-hyphen" => "-example.com",
        _ => unreachable!(),
    };
    bencher.bench(|| idna::domain_to_ascii_strict(black_box(domain)).unwrap_err());
}
