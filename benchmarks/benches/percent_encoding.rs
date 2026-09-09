//! Benchmarks for the `percent-encoding` crate.

use divan::{black_box, Bencher};
use percent_encoding::{
    percent_decode_str, percent_encode, utf8_percent_encode, AsciiSet, CONTROLS, NON_ALPHANUMERIC,
};

fn main() {
    divan::main();
}

/// The set used for URL paths, as defined by the URL Standard.
const PATH: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'#')
    .add(b'?')
    .add(b'{')
    .add(b'}');

const INPUTS: &[(&str, &str)] = &[
    ("nothing-to-encode", "/some/plain/path/index.html"),
    ("few-bytes-to-encode", "/some/path with spaces/index.html"),
    (
        "mostly-to-encode",
        "{\"key\": \"value\", \"other\": [1, 2]}",
    ),
    ("unicode", "/döner/kebab/日本語/🦀"),
    (
        "long-nothing-to-encode",
        "/a/long/path/without/anything/that/needs/to/be/percent/encoded/at/all/index.html",
    ),
    (
        "long-mixed",
        "/a/long path/with/some things/that need/encoding/日本語/index.html?q=a b c",
    ),
];

fn input(name: &str) -> &'static str {
    INPUTS
        .iter()
        .find(|(case, _)| *case == name)
        .expect("unknown case")
        .1
}

/// Encoding with the path set, the hot path when serializing URLs.
#[divan::bench(args = INPUTS.iter().map(|(name, _)| *name))]
fn encode_path_set(bencher: Bencher, name: &str) {
    let input = input(name);
    bencher.bench(|| utf8_percent_encode(black_box(input), PATH).to_string());
}

/// Encoding everything that is not alphanumeric, the worst case.
#[divan::bench(args = ["nothing-to-encode", "unicode", "long-mixed"])]
fn encode_non_alphanumeric(bencher: Bencher, name: &str) {
    let input = input(name);
    bencher.bench(|| utf8_percent_encode(black_box(input), NON_ALPHANUMERIC).to_string());
}

/// Encoding raw bytes instead of a `str`.
#[divan::bench]
fn encode_bytes(bencher: Bencher) {
    let input: Vec<u8> = (0u8..=255).collect();
    bencher.bench(|| percent_encode(black_box(&input), NON_ALPHANUMERIC).to_string());
}

const ENCODED: &[(&str, &str)] = &[
    ("nothing-to-decode", "/some/plain/path/index.html"),
    ("few-bytes-to-decode", "/some/path%20with%20spaces/index.html"),
    (
        "unicode",
        "/d%C3%B6ner/kebab/%E6%97%A5%E6%9C%AC%E8%AA%9E/%F0%9F%A6%80",
    ),
    ("invalid-utf8", "/%80%81invalid%FF/path"),
    (
        "long-mixed",
        "/a/long%20path/with/some%20things/that%20need/decoding/%E6%97%A5%E6%9C%AC%E8%AA%9E/index.html",
    ),
];

fn encoded(name: &str) -> &'static str {
    ENCODED
        .iter()
        .find(|(case, _)| *case == name)
        .expect("unknown case")
        .1
}

/// Decoding to bytes.
#[divan::bench(args = ENCODED.iter().map(|(name, _)| *name))]
fn decode(bencher: Bencher, name: &str) {
    let input = encoded(name);
    bencher.bench(|| percent_decode_str(black_box(input)).collect::<Vec<u8>>());
}

/// Decoding to UTF-8, replacing invalid sequences.
#[divan::bench(args = ENCODED.iter().map(|(name, _)| *name))]
fn decode_utf8_lossy(bencher: Bencher, name: &str) {
    let input = encoded(name);
    bencher.bench(|| percent_decode_str(black_box(input)).decode_utf8_lossy());
}
