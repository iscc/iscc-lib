# Project State

## Status: IN_PROGRESS

## Phase: All 9 gen\_\*\_v0 implemented — pre-binding

All nine `gen_*_v0` functions are implemented in pure Rust and pass their conformance vectors. The
core crate is clean (clippy, fmt, no unsafe). No binding crates, CI, benchmarks, or docs exist yet.

## What Exists

- **45 git commits** — bootstrap through gen_video_code_v0
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate — zero binding dependencies (blake3, data-encoding, hex,
    thiserror, unicode-normalization, unicode-general-category, xxhash-rust)
- **8 source modules**: lib.rs, codec.rs, simhash.rs, minhash.rs, cdc.rs, dct.rs, wtahash.rs,
    utils.rs
- **`gen_meta_code_v0`**: 13/16 vectors pass (3 skipped — meta object/Data-URL inputs deferred)
- **`gen_text_code_v0`**: all 5 vectors pass
- **`gen_image_code_v0`**: all 3 vectors pass
- **`gen_audio_code_v0`**: all 5 vectors pass
- **`gen_video_code_v0`**: all 3 vectors pass
- **`gen_mixed_code_v0`**: all 2 vectors pass
- **`gen_data_code_v0`**: all 4 vectors pass
- **`gen_instance_code_v0`**: all vectors pass
- **`gen_iscc_code_v0`**: all 5 vectors pass
- **Conformance vectors**: `tests/data.json` vendored from iscc-core
- **134 tests total** — all pass
- **`cargo clippy`** clean, **`cargo fmt`** clean, **no `unsafe`**
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents
- **Python smoke test**: `tests/test_smoke.py` (infrastructure only, no binding tests)

## What's Missing

- **`gen_meta_code_v0` meta object support** — 3 conformance vectors skipped (Data-URL/JSON meta)
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **134 tests pass** (0 failures)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- Conformance: meta ✅ (13/16, 3 skipped) | text ✅ | image ✅ | audio ✅ | video ✅ | mixed ✅ | data ✅ |
    instance ✅ | iscc_code ✅

## Next Milestone

Begin Python bindings (`crates/iscc-py/`) with PyO3/maturin exposing all 9 `gen_*_v0` functions,
using abi3-py310 for single wheel per platform. This is the highest-value binding target and the
most mature toolchain.
