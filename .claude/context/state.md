# Project State

## Status: IN_PROGRESS

## Phase: Rust core complete with 100% conformance — pre-binding

All nine `gen_*_v0` functions are implemented in pure Rust with full conformance (all 16 meta
vectors now pass, including meta object and Data-URL support added in the last iteration). The core
crate is clean and ready to serve as the foundation for language bindings.

## What Exists

- **47 git commits** — bootstrap through meta object support (100% conformance)
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate — zero binding dependencies (blake3, data-encoding, hex,
    serde_json, thiserror, unicode-normalization, unicode-general-category, xxhash-rust)
- **8 source modules**: lib.rs, codec.rs, simhash.rs, minhash.rs, cdc.rs, dct.rs, wtahash.rs,
    utils.rs
- **All 9 gen functions** with 100% conformance vector coverage:
    - `gen_meta_code_v0`: 16/16 vectors (including meta object + Data-URL)
    - `gen_text_code_v0`: 5/5 vectors
    - `gen_image_code_v0`: 3/3 vectors
    - `gen_audio_code_v0`: 5/5 vectors
    - `gen_video_code_v0`: 3/3 vectors
    - `gen_mixed_code_v0`: 2/2 vectors
    - `gen_data_code_v0`: 4/4 vectors
    - `gen_instance_code_v0`: all vectors pass
    - `gen_iscc_code_v0`: 5/5 vectors
- **Conformance vectors**: `tests/data.json` vendored from iscc-core
- **143 tests total** — all pass, 0 ignored
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **`crates/iscc-py/`** — PyO3/maturin Python bindings (highest priority)
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **143 tests pass** (0 failures, 0 ignored)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- Conformance: all 9 gen functions pass all vectors (meta 16/16, text 5/5, image 3/3, audio 5/5,
    video 3/3, mixed 2/2, data 4/4, instance ✅, iscc_code 5/5)

## Next Milestone

Begin Python bindings (`crates/iscc-py/`) with PyO3/maturin exposing all 9 `gen_*_v0` functions,
using abi3-py310 for single wheel per platform. This is the highest-value binding target.
