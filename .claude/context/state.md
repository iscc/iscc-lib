# Project State

## Status: IN_PROGRESS

## Phase: Codec module implemented — gen\_\*\_v0 stubs remain unimplemented

The Rust workspace has one crate (`iscc-lib`) with all 9 `gen_*_v0` function stubs and a fully
implemented `codec` module (type enums, header encoding/decoding, base32, varnibble). All 42 tests
pass. No conformance vectors vendored yet. No binding crates exist.

## What Exists

- **22 git commits** — tooling bootstrap + workspace skeleton + codec module
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate with `thiserror` dependency
- **`src/codec.rs`**: complete codec module — `MainType`, `SubType`, `Version` enums, header
    encode/decode, `encode_component`, base32 encode/decode, varnibble encode/decode, length
    encoding (33 codec tests)
- **All 9 `gen_*_v0` function stubs** in `src/lib.rs` with correct signatures (returning
    `Err(NotImplemented)`)
- **42 tests total** — 33 codec tests + 9 stub tests — all pass
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **Conformance test vectors** — `iscc-core/data.json` not yet vendored
- **All 9 function implementations** — every `gen_*_v0` is a stub returning `NotImplemented`
- **Internal modules** — no `cdc`, `minhash`, `simhash`, `blake3`, `utils` modules yet
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **42 tests pass** (33 codec + 9 stubs, no conformance tests)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- No conformance vector testing yet — zero real correctness verification

## Next Milestone

Vendor conformance test vectors (`iscc-core/data.json`) and implement the first `gen_*_v0` function
end-to-end (likely `gen_instance_code_v0` or `gen_data_code_v0`, leveraging existing Rust from
`bio-codes/iscc-sum`), with conformance tests verifying output matches `iscc-core`.
