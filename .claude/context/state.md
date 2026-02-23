# Project State

## Status: IN_PROGRESS

## Phase: Polish — Python API compatibility and documentation refinement

All 9 `gen_*_v0` functions implemented in Rust core with 143 conformance tests passing. All 4
binding crates (Python, Node.js, WASM, C FFI) complete with conformance tests. CI (5 jobs) and Docs
workflows green. Release workflow covers crates.io, PyPI, and npm.

Two areas need work before first release:

1. **Python bindings are not drop-in compatible with iscc-core** — functions return plain strings
    instead of dicts with structured fields (`metahash`, `name`, `characters`, `datahash`,
    `filesize`, `parts`). Streaming functions accept only `bytes` instead of file-like objects. The
    Rust core API also needs structured return types to support this.
2. **Documentation lacks ISCC branding and features** — missing custom CSS, logo/favicon, copy-page
    feature, llms.txt generation, abbreviations, Diátaxis navigation structure, per-language tabbed
    code examples, and Open Graph meta tags. Reference: iscc-usearch at usearch.iscc.codes.

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

- **Python API compatibility**: see `.claude/context/specs/python-bindings.md`
- **Rust structured return types**: core API returns plain strings, needs structured result types to
    carry additional fields (metahash, characters, datahash, filesize, etc.)
- **Documentation refinement**: see `.claude/context/specs/documentation.md`
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

Python API drop-in compatibility with iscc-core (structured return types in Rust core, dict returns
in Python bindings, stream input support). Then documentation refinement.
