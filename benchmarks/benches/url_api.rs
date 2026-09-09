//! Benchmarks for the `url` crate API beyond parsing: accessors, mutation,
//! serialization and file path conversion.

use divan::{black_box, Bencher};
use url::Url;

fn main() {
    divan::main();
}

const URL: &str =
    "https://user:password@example.com:8080/a/b/c/index.html?one=1&two=2&three=3#frag";

fn sample() -> Url {
    Url::parse(URL).unwrap()
}

/// Serialization back to a string.
#[divan::bench]
fn serialize(bencher: Bencher) {
    let url = sample();
    bencher.bench(|| black_box(&url).to_string());
}

/// Cloning a parsed URL, a common operation in consumers of the crate.
#[divan::bench]
fn clone(bencher: Bencher) {
    let url = sample();
    bencher.bench(|| black_box(&url).clone());
}

/// Reading the components of a parsed URL.
#[divan::bench]
fn accessors(bencher: Bencher) {
    let url = sample();
    bencher.bench(|| {
        let url = black_box(&url);
        (
            url.scheme(),
            url.username(),
            url.password(),
            url.host_str(),
            url.port(),
            url.path(),
            url.query(),
            url.fragment(),
        )
    });
}

/// Iterating over the path segments.
#[divan::bench]
fn path_segments(bencher: Bencher) {
    let url = sample();
    bencher.bench(|| black_box(&url).path_segments().unwrap().count());
}

/// Decoding the query string into key/value pairs.
#[divan::bench]
fn query_pairs(bencher: Bencher) {
    let url = sample();
    bencher.bench(|| {
        black_box(&url)
            .query_pairs()
            .map(|(key, value)| key.len() + value.len())
            .sum::<usize>()
    });
}

/// Computing the origin (tuple or opaque).
#[divan::bench(args = ["special", "non-special"])]
fn origin(bencher: Bencher, kind: &str) {
    let url = match kind {
        "special" => sample(),
        "non-special" => Url::parse("git+ssh://git@example.com/servo/rust-url.git").unwrap(),
        _ => unreachable!(),
    };
    bencher.bench(|| black_box(&url).origin());
}

/// Mutating a parsed URL through the setters.
#[divan::bench(args = ["path", "query", "fragment", "host", "port"])]
fn setters(bencher: Bencher, kind: &str) {
    let url = sample();
    bencher
        .with_inputs(|| url.clone())
        .bench_refs(|url| match kind {
            "path" => url.set_path("/other/path/resource.json"),
            "query" => url.set_query(Some("four=4&five=5")),
            "fragment" => url.set_fragment(Some("other")),
            "host" => url.set_host(Some("other.example")).unwrap(),
            "port" => url.set_port(Some(4443)).unwrap(),
            _ => unreachable!(),
        });
}

/// Appending path segments with the mutable segments API.
#[divan::bench]
fn push_path_segments(bencher: Bencher) {
    let url = Url::parse("https://example.com/base").unwrap();
    bencher.with_inputs(|| url.clone()).bench_refs(|url| {
        url.path_segments_mut()
            .unwrap()
            .push("with space")
            .push("ünicode")
            .push("last");
    });
}

/// Building a query string with the mutable query pairs API.
#[divan::bench]
fn append_query_pairs(bencher: Bencher) {
    let url = Url::parse("https://example.com/search").unwrap();
    bencher.with_inputs(|| url.clone()).bench_refs(|url| {
        url.query_pairs_mut()
            .append_pair("q", "rust url crate")
            .append_pair("lang", "rust")
            .append_pair("unicode", "vermögensberater");
    });
}

// Same fixture as the `url_to_file_path` benchmark of `url/benches/parse_url.rs`,
// so that the measurements stay comparable with the upstream benchmark.
#[cfg(unix)]
const FILE_PATH: &str = "/data/dir/next_dir/sub_sub_dir/testing/testing.json";
#[cfg(windows)]
const FILE_PATH: &str = r"C:\dir\next_dir\sub_sub_dir\testing\testing.json";

// A path with spaces, which exercises the percent-encoding paths of the
// conversion on a longer input.
#[cfg(unix)]
const FILE_PATH_WITH_SPACES: &str = "/usr/share/doc/rust-url/some file with spaces.html";
#[cfg(windows)]
const FILE_PATH_WITH_SPACES: &str = r"C:\Program Files\rust-url\some file with spaces.html";

/// `file:` URL to path conversion, and back.
#[cfg(any(unix, windows))]
#[divan::bench]
fn to_file_path(bencher: Bencher) {
    let url = Url::from_file_path(FILE_PATH).unwrap();
    bencher.bench(|| black_box(&url).to_file_path().unwrap());
}

#[cfg(any(unix, windows))]
#[divan::bench]
fn from_file_path(bencher: Bencher) {
    bencher.bench(|| Url::from_file_path(black_box(FILE_PATH)).unwrap());
}

#[cfg(any(unix, windows))]
#[divan::bench]
fn to_file_path_with_spaces(bencher: Bencher) {
    let url = Url::from_file_path(FILE_PATH_WITH_SPACES).unwrap();
    bencher.bench(|| black_box(&url).to_file_path().unwrap());
}

#[cfg(any(unix, windows))]
#[divan::bench]
fn from_file_path_with_spaces(bencher: Bencher) {
    bencher.bench(|| Url::from_file_path(black_box(FILE_PATH_WITH_SPACES)).unwrap());
}
