# Project State

## Status: IN_PROGRESS

## Phase: API expansion — extended Rust core + Python drop-in compatibility + documentation

All 9 `gen_*_v0` functions implemented in Rust core with 143 conformance tests passing. All 4
binding crates (Python, Node.js, WASM, C FFI) complete with conformance tests. CI (5 jobs) and Docs
workflows green. Release workflow covers crates.io, PyPI, and npm.

Three areas need work before first release:

1. **Rust core API expansion** — the Tier 1 API needs to grow from 9 gen functions to 22 public
    symbols (text utils, algorithm primitives, soft hashes, streaming hashers, codec ops, encoding,
    diagnostics). See `specs/rust-core.md` for the full spec. The gen functions also need
    structured return types instead of plain strings.
2. **Python bindings are not drop-in compatible with iscc-core** — functions return plain strings
    instead of dicts with structured fields (`metahash`, `name`, `characters`, `datahash`,
    `filesize`, `parts`). Streaming functions accept only `bytes` instead of file-like objects. The
    new Tier 1 functions must also be bound. See `specs/python-bindings.md`.
3. **Documentation lacks ISCC branding and features** — missing custom CSS, logo/favicon, copy-page
    feature, llms.txt generation, abbreviations, Diátaxis navigation structure, per-language tabbed
    code examples, and Open Graph meta tags. See `specs/documentation.md`.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy
    clean, fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`
- **Python tests (`tests/`)**: 49 tests passing (46 conformance + 3 smoke)
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests in `__tests__/conformance.test.mjs`
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
- **Criterion benchmarks**: all 9 gen functions benchmarked
- **Comparative pytest-benchmark**: iscc-core vs iscc_lib (1.3x-158x speedups measured)
- **Documentation**: 5 pages (landing, Python API, Rust API, architecture, benchmarks) — stock
    theme, no ISCC branding
- **CI workflow**: `.github/workflows/ci.yml` — 5 jobs, green
- **Release workflow**: `.github/workflows/release.yml` — OIDC for crates.io + PyPI, NPM_TOKEN for
    npm (first release), Node 22
- **Docs workflow**: `.github/workflows/docs.yml` — build + deploy, green
- **Infrastructure**: GitHub Pages enabled (lib.iscc.codes), `@iscc` npm org created, `NPM_TOKEN`
    repo secret configured

## What's Missing

- **Rust core API expansion**: 13 new Tier 1 symbols (text utils, algorithm primitives, soft hash,
    encoding, codec, streaming hashers, diagnostics) plus structured return types for existing gen
    functions. See `specs/rust-core.md`
- **Python API compatibility**: see `specs/python-bindings.md`
- **Binding updates**: all 4 binding crates need to expose the new Tier 1 functions
- **Documentation refinement**: see `specs/documentation.md`
- **Untracked napi build artifacts**: `crates/iscc-napi/` has local build artifacts that should be
    gitignored

## CI

- **CI workflow (ci.yml): PASSING** — all 5 jobs green
- **Docs workflow (docs.yml): PASSING** — build + deploy both green

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean
- No `unsafe` in core crate
- Python tests: **49 passed** (46 conformance + 3 smoke)
- Node.js: 46 conformance tests in CI
- WASM: conformance tests in CI
- C FFI: 20 unit tests + C test program in CI

## Next Milestone

Rust core API expansion (structured return types + 13 new Tier 1 symbols), then Python API drop-in
compatibility (dict returns, stream inputs, new function bindings), then documentation refinement.
