# Target State — iscc-lib

High-performance polyglot implementation of ISO 24138:2024 (ISCC). Every component below is verified
by concrete criteria that agents can check.

## Rust Core Crate — `iscc-lib` on crates.io

A pure Rust library (no binding dependencies) publishable to crates.io as
[`iscc-lib`](https://crates.io/crates/iscc-lib). Name is available.

Detailed spec: `.claude/context/specs/rust-core.md`

**Tier 1 API** — 22 public symbols bound in all languages:

- 9 `gen_*_v0` functions with structured return types (matching iscc-core dict fields)
- 4 text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- 4 algorithm primitives: `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`
- 1 soft hash: `soft_hash_video_v0`
- 1 encoding utility: `encode_base64`
- 1 codec operation: `iscc_decompose`
- 2 streaming types: `DataHasher`, `InstanceHasher`
- 1 diagnostic: `conformance_selftest`

**Tier 2 API** — `codec` module (Rust-only, not bound): `MainType`, `SubType`, `Version` enums,
`encode_header`, `decode_header`, `encode_base32`, `decode_base32`, `encode_component`,
`encode_length`, `decode_length`, `encode_units`.

**Verified when:**

- `cargo test -p iscc-lib` passes with all conformance vectors from `iscc-core/data.json`
- Output of every `gen_*_v0` function matches `iscc-core` reference for every test vector
- All Tier 1 functions produce output matching their iscc-core counterparts
- `DataHasher` / `InstanceHasher` produce identical results to `gen_data_code_v0` /
    `gen_instance_code_v0` for the same input
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` without documented justification
- Crate has zero binding dependencies (no PyO3, napi, wasm-bindgen)

## Python Bindings — `iscc_lib` on PyPI

A Python package installable from PyPI as [`iscc_lib`](https://pypi.org/project/iscc-lib/), a
drop-in replacement for `iscc-core`. Name is available. (You already own `iscc` and `iscc-core` on
PyPI.)

Detailed spec: `.claude/context/specs/python-bindings.md`

**Verified when:**

- `pip install .` succeeds and exposes all 9 `gen_*_v0` functions
- All functions return `dict` with the same keys and values as iscc-core
- Streaming functions accept both `bytes` and file-like objects
- `pytest` passes the same conformance vectors from Python
- `ruff check` and `ruff format --check` clean
- Single wheel per platform (abi3-py310)

## Node.js Bindings — `@iscc/lib` on npm

An npm package [`@iscc/lib`](https://www.npmjs.com/package/@iscc/lib) exposing all 9 entrypoints as
native addon via napi-rs. Published under the `@iscc` npm org.

**Verified when:**

- `npm test` passes conformance vectors from JavaScript
- Package installs cleanly via `npm install`

## WASM Bindings — `@iscc/wasm` on npm

A browser-compatible WASM package [`@iscc/wasm`](https://www.npmjs.com/package/@iscc/wasm) exposing
all 9 entrypoints via wasm-bindgen. Published under the same `@iscc` npm scope.

**Verified when:**

- Conformance tests pass in a WASM runtime
- Package builds with `wasm-pack`

## C FFI

A C-compatible shared library with generated headers, enabling Go/Java/C# bindings.

**Verified when:**

- cbindgen generates valid C headers
- A C test program can call the entrypoints and get correct results

## Documentation

Project documentation hosted at lib.iscc.codes with ISCC branding and agent-friendly features
matching `iscc/iscc-usearch` at `usearch.iscc.codes`.

Detailed spec: `.claude/context/specs/documentation.md`

**Verified when:**

- Site builds and deploys via GitHub Pages
- ISCC branding (colors, logo, favicon, dark mode) matches iscc-usearch
- Covers Rust API, Python API, architecture, and per-language how-to guides
- All code examples use tabbed multi-language format (Python, Rust, Node.js, WASM)
- Copy-page feature and `llms-full.txt` generation for agent consumption
- Navigation follows Diátaxis framework (tutorials, howto, explanation, reference)

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
- **Conformance vectors**:
    `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- **Architecture notes**: @notes/00-overview.md — design decisions, tooling stack, workspace layout.
