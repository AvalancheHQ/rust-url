rust-url benchmarks
===================

CodSpeed benchmarks for the crates of this repository: `url`, `idna`,
`percent-encoding`, `form_urlencoded` and `data-url`.

They are written with [divan](https://github.com/nvzqz/divan) through the
[CodSpeed compatibility layer](https://codspeed.io/docs/benchmarks/rust/divan),
and run on every pull request by the `CodSpeed` workflow.

This crate is deliberately kept out of the root workspace (like `url/fuzz`):
the CodSpeed compatibility layers require a recent Rust version, while the
published crates keep a much lower MSRV that the CI still builds against.

Running the benchmarks
----------------------

Locally, with plain divan (wall time, no CodSpeed instrumentation):

```sh
cd benchmarks
cargo bench
# a single suite, or a single benchmark
cargo bench --bench url_parse
cargo bench --bench url_parse -- parse[plain]
```

With CodSpeed's CPU simulation instrument, which is what CI measures:

```sh
cd benchmarks
cargo codspeed build -m simulation
codspeed run --mode simulation -- cargo codspeed run
```

`cargo codspeed` comes from the
[`cargo-codspeed`](https://codspeed.io/docs/reference/codspeed-rust/cargo-codspeed)
crate, and `codspeed` from the [CodSpeed CLI](https://codspeed.io/docs/cli).

Suites
------

| Suite               | What it covers                                                              |
| ------------------- | --------------------------------------------------------------------------- |
| `url_parse`         | Parsing absolute URLs, relative reference resolution, parse errors          |
| `url_api`           | Accessors, setters, path/query mutation, serialization, `file:` path round-trip |
| `idna`              | Domain to ASCII (Punycode) and to Unicode conversions, error paths          |
| `percent_encoding`  | Percent-encoding and decoding for several ASCII sets and inputs             |
| `form_urlencoded`   | Parsing and serializing `application/x-www-form-urlencoded` data            |
| `data_url`          | `data:` URL processing and base64/percent-encoded body decoding             |
