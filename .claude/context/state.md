# Project State

## Status: IN_PROGRESS

## Phase: 7 of 9 gen\_\*\_v0 implemented — gen_data_code_v0 added

The Rust workspace has `iscc-lib` with codec, simhash, minhash, cdc, and utils internal modules.
Seven gen functions pass all conformance vectors. Two stubs remain (image, video). No binding
crates, CI, benchmarks, or docs exist yet.

## What Exists

- **41 git commits** — bootstrap through gen_data_code_v0
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate with `blake3`, `data-encoding`, `thiserror`, `hex`,
    `serde_json`, `unicode-normalization`, `xxhash-rust` dependencies
- **`src/codec.rs`**: complete codec module — MainType, SubType, Version enums, header
    encode/decode, encode_component, encode_units, base32, varnibble
- **`src/simhash.rs`**: SimHash algorithm + sliding_window helper
- **`src/minhash.rs`**: MinHash with MPA/MPB constants, compress, alg_minhash_256
- **`src/cdc.rs`**: content-defined chunking for gen_data_code_v0
- **`src/utils.rs`**: text_clean, text_remove_newlines, text_trim, text_collapse, multi_hash_blake3
- **`gen_instance_code_v0`**: BLAKE3 hash → encode_component, all conformance vectors pass
- **`gen_meta_code_v0`**: SimHash-based metadata code, 13/16 vectors pass (3 skipped — meta
    object/Data-URL inputs not yet supported)
- **`gen_text_code_v0`**: MinHash-based text code, all 5 vectors pass
- **`gen_audio_code_v0`**: multi-stage SimHash, all 5 vectors pass
- **`gen_mixed_code_v0`**: SimHash-based code mixing, all 2 vectors pass
- **`gen_data_code_v0`**: CDC + MinHash, all 4 conformance vectors pass
- **`gen_iscc_code_v0`**: composite ISCC-CODE assembly, all 5 vectors pass
- **2 remaining `gen_*_v0` stubs** returning `Err(NotImplemented)`: image, video
- **Conformance vectors**: `tests/data.json` vendored from iscc-core
- **117 tests total** — all pass
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **2 function implementations** — gen_image_code_v0, gen_video_code_v0
- **`gen_meta_code_v0` meta object support** — 3 conformance vectors skipped (Data-URL/JSON meta)
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **117 tests pass** (all green, 0 failures)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- Conformance: instance ✅ | meta ✅ (13/16, 3 skipped) | text ✅ | audio ✅ | mixed ✅ | data ✅ |
    iscc_code ✅
- Stubs only: image, video

## Next Milestone

Implement `gen_image_code_v0` — takes grayscale pixel values and produces a Content-Code for images.
Need to study iscc-core's Python reference to understand the image hashing algorithm (likely a
perceptual hash). This is one of two remaining stub functions.
