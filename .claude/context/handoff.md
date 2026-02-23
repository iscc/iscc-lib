## 2026-02-23 — Scaffold napi-rs crate with all 9 gen functions

**Done:** Created the `crates/iscc-napi/` crate exposing all 9 `gen_*_v0` functions as a native
Node.js addon via napi-rs. Follows the same thin-wrapper pattern as the PyO3 bindings — no logic in
the binding layer, just type conversion and error mapping.

**Files changed:**

- `Cargo.toml`: Added `crates/iscc-napi` to workspace members; added `napi`, `napi-derive`, and
    `napi-build` workspace dependencies
- `crates/iscc-napi/Cargo.toml`: New crate config with cdylib output, iscc-lib + napi dependencies
- `crates/iscc-napi/build.rs`: Minimal napi build script (`napi_build::setup()`)
- `crates/iscc-napi/src/lib.rs`: All 9 gen functions with `#[napi(js_name = "...")]` attributes,
    `Buffer` for byte data, `Option<u32>`/`Option<bool>` for default parameters, `napi::Error` for
    error propagation
- `crates/iscc-napi/package.json`: Minimal `@iscc/lib` npm package config with napi-rs build scripts

**Verification:**

- `cargo build -p iscc-napi` — compiles successfully
- `cargo clippy -p iscc-napi -- -D warnings` — clean, no warnings
- `cargo fmt --all --check` — passes
- `cargo test -p iscc-lib` — 143 tests pass (core unaffected)
- `uv run pytest tests/` — 49 tests pass (Python bindings unaffected)

**Next:** Consider adding Node.js integration tests (requires `npm install` + `@napi-rs/cli` to
build the `.node` binary and a JS test runner). Alternatively, proceed with CI workflow updates to
include the napi crate in the build matrix, or move on to the WASM binding crate scaffold.

**Notes:** napi-rs type mappings differ slightly from PyO3 — napi uses owned `String` (not `&str`)
and `Buffer` (not `&[u8]`) for function parameters. Default parameter values are handled via
`Option<T>` with `.unwrap_or()` since napi-rs doesn't support Python-style default args. The
`#[napi(js_name = "gen_meta_code_v0")]` attribute preserves snake_case naming for JS consumers
(napi-rs default would auto-convert to camelCase). The `gen_video_code_v0` function takes
`Vec<Vec<i32>>` which maps directly to the Rust core's `&[Vec<i32>]` signature.
