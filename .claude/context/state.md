# Project State

## Status: IN_PROGRESS

## Phase: 3 of 9 gen\_\*\_v0 implemented — MinHash module added

The Rust workspace has `iscc-lib` with codec, simhash, minhash, and utils internal modules. Three
gen functions (`gen_instance_code_v0`, `gen_meta_code_v0`, `gen_text_code_v0`) pass all conformance
vectors. 6 gen functions remain stubs. No binding crates, CI, benchmarks, or docs exist.

## What Exists

- **30 git commits** — bootstrap + codec + gen_instance + gen_meta + gen_text
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate with `blake3`, `data-encoding`, `thiserror`, `hex`,
    `serde_json`, `unicode-normalization`, `xxhash-rust` dependencies
- **`src/codec.rs`**: complete codec module — MainType, SubType, Version enums, header
    encode/decode, encode_component, base32, varnibble (33 tests)
- **`src/simhash.rs`**: SimHash algorithm + sliding_window helper (9 tests)
- **`src/minhash.rs`**: MinHash with MPA/MPB constants, compress, alg_minhash_256 (7 tests)
- **`src/utils.rs`**: text_clean, text_remove_newlines, text_trim, text_collapse, multi_hash_blake3
    (15 tests)
- **`gen_instance_code_v0`**: BLAKE3 hash → encode_component, all conformance vectors pass
- **`gen_meta_code_v0`**: SimHash-based metadata code, 13/16 conformance vectors pass (3 skipped —
    meta object/Data-URL inputs not yet supported)
- **`gen_text_code_v0`**: MinHash-based text code using xxh32 + sliding_window(13), all 5
    conformance vectors pass
- **6 remaining `gen_*_v0` stubs** returning `Err(NotImplemented)`
- **Conformance vectors**: `tests/data.json` vendored from iscc-core
- **85 tests total** — all pass (33 codec + 9 simhash + 7 minhash + 15 utils + 21 gen function
    tests)
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **6 function implementations** — gen_image/audio/video/mixed/data/iscc_code_v0
- **`gen_meta_code_v0` meta object support** — 3 conformance vectors skipped (Data-URL/JSON meta)
- **Internal modules** — `cdc` (needed for gen_data_code_v0)
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **85 tests pass** (33 codec + 9 simhash + 7 minhash + 15 utils + 21 gen)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- `gen_instance_code_v0` conformance: **PASS** (all vectors)
- `gen_meta_code_v0` conformance: **PASS** (13/16 vectors; 3 skipped for meta object inputs)
- `gen_text_code_v0` conformance: **PASS** (all 5 vectors)
- 6 remaining gen functions: **stubs only**

## Next Milestone

Implement `gen_data_code_v0` — requires a `cdc` (content-defined chunking) module plus the existing
`minhash` module. The CDC algorithm splits byte streams into variable-size chunks, feeds chunk
hashes to MinHash. Prior Rust implementation exists in `bio-codes/iscc-sum` to adapt from.
