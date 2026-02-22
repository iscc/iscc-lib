# Project State

## Status: IN_PROGRESS

## Phase: First gen function implemented — 8 of 9 gen\_\*\_v0 remain stubs

The Rust workspace has the `iscc-lib` core crate with a complete codec module and a fully working
`gen_instance_code_v0` passing all conformance vectors. The remaining 8 `gen_*_v0` functions are
stubs. No binding crates, CI, benchmarks, or docs exist yet.

## What Exists

- **24 git commits** — tooling bootstrap + workspace skeleton + codec module + gen_instance_code_v0
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate with `blake3`, `data-encoding`, `thiserror` dependencies
- **`src/codec.rs`**: complete codec module — `MainType`, `SubType`, `Version` enums, header
    encode/decode, `encode_component`, base32 encode/decode, varnibble encode/decode (33 tests)
- **`gen_instance_code_v0`**: fully implemented (BLAKE3 hash → encode_component → ISCC prefix),
    passing all conformance vectors from vendored `data.json`
- **8 remaining `gen_*_v0` stubs** returning `Err(NotImplemented)`
- **Conformance vectors**: `tests/data.json` vendored from iscc-core
- **43 tests total** — 33 codec + 1 instance empty + 1 instance conformance + 8 stubs — all pass
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **8 function implementations** — gen_meta/text/image/audio/video/mixed/data/iscc_code_v0
- **Internal modules** — `cdc`, `minhash`, `simhash`, `utils` (needed for remaining functions)
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **43 tests pass** (33 codec + 10 gen function tests including
    conformance)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- `gen_instance_code_v0` conformance: **PASS** (all vectors from data.json match iscc-core output)
- 8 remaining gen functions: **not tested** (stubs only)

## Next Milestone

Implement `gen_data_code_v0` — requires CDC (content-defined chunking) and MinHash modules. Leverage
existing production Rust from `bio-codes/iscc-sum` which already has optimized CDC and MinHash. This
is the most complex remaining function and establishes patterns for the others.
