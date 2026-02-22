# Project State

## Status: IN_PROGRESS

## Phase: 2 of 9 gen\_\*\_v0 implemented — core internal modules established

The Rust workspace has `iscc-lib` with a complete codec module, `simhash` and `utils` internal
modules, and two fully working gen functions (`gen_instance_code_v0`, `gen_meta_code_v0`) passing
all conformance vectors. 7 gen functions remain stubs. No binding crates, CI, benchmarks, or docs
exist.

## What Exists

- **27 git commits** — bootstrap + codec + gen_instance_code_v0 + gen_meta_code_v0
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate with `blake3`, `data-encoding`, `thiserror`, `hex`,
    `serde_json`, `unicode-normalization` dependencies
- **`src/codec.rs`**: complete codec module — MainType, SubType, Version enums, header
    encode/decode, encode_component, base32 encode/decode, varnibble encode/decode (33 tests)
- **`src/simhash.rs`**: SimHash algorithm + sliding_window helper (8 tests)
- **`src/utils.rs`**: text_clean, text_remove_newlines, text_trim, text_collapse, multi_hash_blake3
    (15 tests)
- **`gen_instance_code_v0`**: BLAKE3 hash → encode_component, all conformance vectors pass
- **`gen_meta_code_v0`**: SimHash-based metadata code with text normalization, 13/16 conformance
    vectors pass (3 skipped — meta object/Data-URL inputs not yet supported)
- **7 remaining `gen_*_v0` stubs** returning `Err(NotImplemented)`
- **Conformance vectors**: `tests/data.json` vendored from iscc-core
- **76 tests total** — all pass (33 codec + 8 simhash + 15 utils + 20 gen function tests)
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **7 function implementations** — gen_text/image/audio/video/mixed/data/iscc_code_v0
- **`gen_meta_code_v0` meta object support** — 3 conformance vectors skipped (Data-URL/JSON meta)
- **Internal modules** — `cdc`, `minhash` (needed for gen_data_code_v0)
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **76 tests pass** (33 codec + 8 simhash + 15 utils + 20 gen functions)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- `gen_instance_code_v0` conformance: **PASS** (all vectors)
- `gen_meta_code_v0` conformance: **PASS** (13/16 vectors; 3 skipped for meta object inputs)
- 7 remaining gen functions: **stubs only**

## Next Milestone

Implement `gen_text_code_v0` — uses the same SimHash pipeline already built for gen_meta (
`text_collapse` → `sliding_window(3)` → BLAKE3 per ngram → `alg_simhash`) but without the name/extra
interleaving. This is the natural next step since the simhash and utils modules are already in
place.
