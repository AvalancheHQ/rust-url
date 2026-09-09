//! Parsing benchmarks for the `url` crate.

use divan::{black_box, Bencher};
use url::Url;

fn main() {
    divan::main();
}

/// Representative URL shapes: the cases from `url/benches/parse_url.rs` plus
/// inputs exercising hosts, paths and queries that are common in the wild.
const CASES: &[(&str, &str)] = &[
    ("plain", "https://example.com/"),
    ("short", "https://example.com/bench"),
    ("query", "https://example.com/parkbench?tre=es&st=uff"),
    (
        "fragment",
        "https://example.com/parkbench?tre=es&st=uff#fragment",
    ),
    ("port", "https://example.com:8080"),
    ("hyphen", "https://hyphenated-example.com/"),
    ("leading-digit", "https://1test.example/"),
    ("unicode-mixed", "https://مثال.example/"),
    ("punycode-mixed", "https://xn--mgbh0fb.example/"),
    ("unicode-ltr", "https://නම.උදාහරණ/"),
    ("punycode-ltr", "https://xn--r0co.xn--ozc8dl2c3bxd/"),
    ("unicode-rtl", "https://الاسم.مثال/"),
    ("punycode-rtl", "https://xn--mgba0b1dh.xn--mgbh0fb/"),
    ("ipv4", "https://192.168.0.1:8080/status"),
    ("ipv6", "https://[2001:db8::ff00:42:8329]:8080/status"),
    ("userinfo", "https://user:p%40ssword@example.com/secret"),
    (
        "deep-path",
        "https://example.com/a/b/c/d/e/f/g/h/i/j/index.html",
    ),
    (
        "dot-segments",
        "https://example.com/a/./b/../c/./d/../../e/f",
    ),
    (
        "percent-encoded",
        "https://example.com/%E2%82%AC%20/%F0%9F%A6%80?q=%E2%82%AC",
    ),
    (
        "many-query-pairs",
        "https://example.com/search?a=1&b=2&c=3&d=4&e=5&f=6&g=7&h=8&i=9&j=10",
    ),
    (
        "non-special",
        "git+ssh://git@example.com/servo/rust-url.git",
    ),
    ("file", "file:///usr/share/doc/rust-url/index.html"),
    ("data", "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ=="),
];

fn input(name: &str) -> &'static str {
    CASES
        .iter()
        .find(|(case, _)| *case == name)
        .expect("unknown case")
        .1
}

/// Full parse of an absolute URL.
#[divan::bench(args = CASES.iter().map(|(name, _)| *name))]
fn parse(bencher: Bencher, name: &str) {
    let url = input(name);
    bencher.bench(|| black_box(url).parse::<Url>().unwrap());
}

/// Relative reference resolution against an already parsed base URL.
#[divan::bench(args = ["path", "query", "fragment", "absolute-path", "authority", "absolute"])]
fn join(bencher: Bencher, kind: &str) {
    let base = Url::parse("https://example.com/a/b/c?query#fragment").unwrap();
    let relative = match kind {
        "path" => "d/e",
        "query" => "?other=1",
        "fragment" => "#other",
        "absolute-path" => "/x/y/z",
        "authority" => "//other.example/x",
        "absolute" => "https://other.example/x",
        _ => unreachable!(),
    };
    bencher.bench(|| base.join(black_box(relative)).unwrap());
}

/// Parsing with a base URL provided through `Url::options`.
#[divan::bench]
fn parse_with_base(bencher: Bencher) {
    let base = Url::parse("https://example.com/a/b/c").unwrap();
    bencher.bench(|| {
        Url::options()
            .base_url(Some(&base))
            .parse(black_box("../d/e?f=g#h"))
            .unwrap()
    });
}

/// Parsing of inputs that must be rejected, which exercises the error paths.
#[divan::bench(args = ["no-scheme", "empty-host", "bad-port", "bad-ipv6", "bad-idna"])]
fn parse_invalid(bencher: Bencher, kind: &str) {
    let url = match kind {
        "no-scheme" => "example.com/foo",
        "empty-host" => "https://",
        "bad-port" => "https://example.com:99999/",
        "bad-ipv6" => "https://[2001:db8::/",
        "bad-idna" => "https://xn--a-ecp.example/",
        _ => unreachable!(),
    };
    bencher.bench(|| black_box(url).parse::<Url>().unwrap_err());
}
