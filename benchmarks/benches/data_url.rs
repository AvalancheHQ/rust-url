//! Benchmarks for the `data-url` crate: `data:` URL processing and body
//! decoding.

use data_url::DataUrl;
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

/// A padding-free base64 chunk, so that it can be repeated to build larger
/// valid payloads.
const BASE64_CHUNK: &str = "SGVsbG8sIFdvcmxk";

fn base64_url(repetitions: usize) -> String {
    let mut url = String::from("data:application/octet-stream;base64,");
    for _ in 0..repetitions {
        url.push_str(BASE64_CHUNK);
    }
    url
}

fn percent_encoded_url(repetitions: usize) -> String {
    let mut url = String::from("data:text/plain;charset=utf-8,");
    for _ in 0..repetitions {
        url.push_str("Hello%2C%20World%E2%82%AC%0A");
    }
    url
}

/// Parsing the header of a `data:` URL (MIME type and base64 flag).
#[divan::bench(args = ["plain", "with-charset", "base64", "with-parameters"])]
fn process(bencher: Bencher, kind: &str) {
    let url = match kind {
        "plain" => "data:,Hello%2C%20World!",
        "with-charset" => "data:text/plain;charset=utf-8,Hello%2C%20World!",
        "base64" => "data:image/png;base64,SGVsbG8sIFdvcmxk",
        "with-parameters" => {
            "data:text/html;charset=utf-8;boundary=\"something\";other=value,<p>Hello</p>"
        }
        _ => unreachable!(),
    };
    bencher.bench(|| DataUrl::process(black_box(url)).unwrap());
}

/// Decoding a base64 body of increasing size.
#[divan::bench(args = [1, 64, 1024])]
fn decode_base64(bencher: Bencher, repetitions: usize) {
    let url = base64_url(repetitions);
    let data_url = DataUrl::process(&url).unwrap();
    bencher.bench(|| black_box(&data_url).decode_to_vec().unwrap());
}

/// Decoding a percent-encoded body of increasing size.
#[divan::bench(args = [1, 64, 1024])]
fn decode_percent_encoded(bencher: Bencher, repetitions: usize) {
    let url = percent_encoded_url(repetitions);
    let data_url = DataUrl::process(&url).unwrap();
    bencher.bench(|| black_box(&data_url).decode_to_vec().unwrap());
}

/// Processing and decoding in one go, as a consumer of the crate would.
#[divan::bench]
fn process_and_decode(bencher: Bencher) {
    let url = base64_url(64);
    bencher.bench(|| {
        DataUrl::process(black_box(&url))
            .unwrap()
            .decode_to_vec()
            .unwrap()
    });
}

/// Reading the parsed MIME type.
#[divan::bench]
fn mime_type_to_string(bencher: Bencher) {
    let data_url =
        DataUrl::process("data:text/html;charset=utf-8;boundary=\"something\",<p>Hi</p>").unwrap();
    bencher.bench(|| black_box(&data_url).mime_type().to_string());
}
