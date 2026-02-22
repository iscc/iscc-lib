# Target State — iscc-lib

High-performance polyglot implementation of ISO 24138:2024 (ISCC). Every component below is
verified by concrete criteria that agents can check.

## Rust Core Crate — `iscc-lib` on crates.io

A pure Rust library (no binding dependencies) publishable to crates.io as
[`iscc-lib`](https://crates.io/crates/iscc-lib). Name is available.

**Entrypoints** — nine `gen_*_v0` functions matching `iscc/iscc-core` Python reference:
`gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
`gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
`gen_iscc_code_v0`

**Verified when:**
- `cargo test -p iscc-lib` passes with all conformance vectors from `iscc-core/data.json`
- Output of every function matches `iscc-core` reference for every test vector
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` without documented justification
- Crate has zero binding dependencies (no PyO3, napi, wasm-bindgen)

## Python Bindings — `iscc-lib` on PyPI

A Python package installable from PyPI as
[`iscc-lib`](https://pypi.org/project/iscc-lib/), a drop-in replacement for `iscc-core`.
Name is available. (You already own `iscc` and `iscc-core` on PyPI.)

**Verified when:**
- `pip install .` succeeds and exposes all 9 `gen_*_v0` functions
- `pytest` passes the same conformance vectors from Python
- `ruff check` and `ruff format --check` clean
- Single wheel per platform (abi3-py310)

## Node.js Bindings — `@iscc/lib` on npm

An npm package [`@iscc/lib`](https://www.npmjs.com/package/@iscc/lib) exposing all 9 entrypoints
as native addon via napi-rs. Requires creating the `@iscc` npm org (scope does not exist yet).
Name is available.

**Verified when:**
- `npm test` passes conformance vectors from JavaScript
- Package installs cleanly via `npm install`

## WASM Bindings — `@iscc/wasm` on npm

A browser-compatible WASM package [`@iscc/wasm`](https://www.npmjs.com/package/@iscc/wasm)
exposing all 9 entrypoints via wasm-bindgen. Published under the same `@iscc` npm scope.
Name is available.

**Verified when:**
- Conformance tests pass in a WASM runtime
- Package builds with `wasm-pack`

## C FFI

A C-compatible shared library with generated headers, enabling Go/Java/C# bindings.

**Verified when:**
- cbindgen generates valid C headers
- A C test program can call the entrypoints and get correct results

## Documentation

Project documentation hosted at lib.iscc.codes.

**Verified when:**
- Site builds and deploys via GitHub Pages
- Covers Rust API, Python API, and architecture
- Same style and features as `iscc/iscc-usearch` documentation (zensical)

## Benchmarks

Performance comparisons against the Python reference implementation.

**Verified when:**
- criterion benchmarks exist for all Rust `gen_*_v0` functions
- pytest-benchmark compares Python bindings vs `iscc-core`
- Speedup factors published in documentation

## CI/CD and Publishing

Automated quality gates and publishing pipelines.

**Verified when:**
- All quality gates run automatically on push/PR
- crates.io and PyPI publishing via OIDC trusted publishing (no API keys)
- All CI workflows green

## Key Resources

- **Existing Rust code**: `bio-codes/iscc-sum` — production-quality, optimized Rust for Data-Code
  and Instance-Code (CDC, MinHash, BLAKE3, streaming). Use as foundation, adapt rather than
  rewrite from Python. Available via deepwiki MCP.
- **Python reference**: `iscc/iscc-core` — the conformance baseline. Available via deepwiki MCP.
- **Conformance vectors**: `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- **Architecture notes**: @notes/00-overview.md — design decisions, tooling stack, workspace layout.
