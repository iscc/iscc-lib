## 2026-02-23 — Add WASM conformance tests runnable with wasm-pack test --node

**Done:** Added conformance tests for the `iscc-wasm` crate that validate all 9 `gen_*_v0`
WASM-exported functions against the vendored `data.json` test vectors (46 cases total). Tests run in
a Node.js WASM runtime via `wasm-pack test --node`. Also removed the unused `js-sys` dependency from
both `crates/iscc-wasm/Cargo.toml` and root `Cargo.toml` workspace dependencies.

**Files changed:**

- `Cargo.toml`: Added `wasm-bindgen-test = "0.3"` to workspace dependencies, removed `js-sys`
- `crates/iscc-wasm/Cargo.toml`: Removed `js-sys` from dependencies, added `rlib` to crate-type
    (needed for integration test linking), added dev-dependencies (`wasm-bindgen-test`,
    `serde_json`, `hex`)
- `crates/iscc-wasm/tests/conformance.rs`: New file — 9 `#[wasm_bindgen_test]` functions covering
    all gen functions with the same test vector parsing patterns as the Rust core tests

**Verification:** `wasm-pack test --node crates/iscc-wasm` passes (9 tests, all 46 vectors
validated). `cargo test -p iscc-lib` passes (143 tests).
`cargo clippy --workspace --all-targets -- -D warnings` clean. `cargo fmt --all --check` clean. No
regressions.

**Next:** Add a WASM CI job to `.github/workflows/ci.yml` (install wasm-pack, run
`wasm-pack test --node crates/iscc-wasm`). This completes the WASM bindings target criterion. After
that, consider C FFI bindings, documentation site, or CI/CD publishing pipelines.

**Notes:** The `cdylib`-only crate type doesn't support integration tests (Rust can't link cdylib
for test harness), so `rlib` was added alongside `cdylib` in `crate-type`. This is a standard
pattern for WASM crates that need integration tests — wasm-pack builds the cdylib for packaging and
uses the rlib for test compilation. The `js-sys` crate was confirmed unused in source code (grep
verified no imports) and is now fully removed.
