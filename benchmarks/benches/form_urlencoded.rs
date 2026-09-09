//! Benchmarks for the `form_urlencoded` crate.

use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

const SMALL: &str = "one=1&two=2&three=3";
const ESCAPED: &str = "name=John+Doe&city=S%C3%A3o+Paulo&emoji=%F0%9F%A6%80&note=a%20b%20c";
const EMPTY_VALUES: &str = "a=&b=&c=&d&e=&f";

fn large() -> String {
    let mut input = String::new();
    for i in 0..256 {
        if i > 0 {
            input.push('&');
        }
        input.push_str(&format!("key{i}=value+number+{i}"));
    }
    input
}

/// Iterating over the decoded pairs of a query string.
#[divan::bench(args = ["small", "escaped", "empty-values", "large"])]
fn parse(bencher: Bencher, kind: &str) {
    let large = large();
    let input: &str = match kind {
        "small" => SMALL,
        "escaped" => ESCAPED,
        "empty-values" => EMPTY_VALUES,
        "large" => &large,
        _ => unreachable!(),
    };
    let bytes = input.as_bytes();
    bencher.bench(|| {
        form_urlencoded::parse(black_box(bytes))
            .map(|(key, value)| key.len() + value.len())
            .sum::<usize>()
    });
}

/// Collecting the decoded pairs into owned `String`s.
#[divan::bench(args = ["escaped", "large"])]
fn parse_into_owned(bencher: Bencher, kind: &str) {
    let large = large();
    let input: &str = match kind {
        "escaped" => ESCAPED,
        "large" => &large,
        _ => unreachable!(),
    };
    let bytes = input.as_bytes();
    bencher.bench(|| {
        form_urlencoded::parse(black_box(bytes))
            .into_owned()
            .collect::<Vec<(String, String)>>()
    });
}

/// Serializing pairs into an `application/x-www-form-urlencoded` string.
#[divan::bench(args = ["plain", "escaped"])]
fn serialize(bencher: Bencher, kind: &str) {
    let pairs: &[(&str, &str)] = match kind {
        "plain" => &[("one", "1"), ("two", "2"), ("three", "3")],
        "escaped" => &[
            ("name", "John Doe"),
            ("city", "São Paulo"),
            ("emoji", "🦀"),
            ("note", "a b c"),
        ],
        _ => unreachable!(),
    };
    bencher.bench(|| {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(black_box(pairs).iter())
            .finish()
    });
}

/// Serializing many pairs, which stresses the encoder and the output buffer.
#[divan::bench]
fn serialize_large(bencher: Bencher) {
    let pairs: Vec<(String, String)> = (0..256)
        .map(|i| (format!("key{i}"), format!("value number {i}")))
        .collect();
    bencher.bench(|| {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(black_box(&pairs).iter())
            .finish()
    });
}

/// The raw byte serializer.
#[divan::bench]
fn byte_serialize(bencher: Bencher) {
    let input = "a value with spaces, punctuation & unicode: São Paulo 🦀".as_bytes();
    bencher.bench(|| form_urlencoded::byte_serialize(black_box(input)).collect::<String>());
}
