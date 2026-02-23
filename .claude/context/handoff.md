## 2026-02-23 — Scaffold WASM bindings crate with all 9 gen functions

**Done:** Created the `crates/iscc-wasm/` binding crate with wasm-bindgen wrappers for all 9
`gen_*_v0` functions, following the identical thin-wrapper pattern as the napi-rs and PyO3 crates.
Added `wasm-bindgen`, `serde-wasm-bindgen`, and `js-sys` to workspace dependencies.

**Files changed:**

- `Cargo.toml` (root): Added `crates/iscc-wasm` to workspace members; added `wasm-bindgen`,
    `serde-wasm-bindgen`, and `js-sys` to `[workspace.dependencies]`
- `crates/iscc-wasm/Cargo.toml`: New crate config with `cdylib` type, `publish = false`
- `crates/iscc-wasm/src/lib.rs`: All 9 `gen_*_v0` wasm-bindgen wrappers — simple types use direct
    wasm-bindgen mapping (`&str`, `&[u8]`, `Vec<i32>`, `Option<T>`); complex array types
    (`gen_video_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`) use `JsValue` +
    `serde_wasm_bindgen::from_value`; all errors map through `JsError`

**Verification:**

- `cargo check -p iscc-wasm --target wasm32-unknown-unknown`: ✅ compiles
- `cargo test -p iscc-lib`: ✅ 143 passed, 0 failed (no regression)
- `cargo clippy -p iscc-wasm -- -D warnings`: ✅ clean
- `cargo clippy -p iscc-lib -- -D warnings`: ✅ clean
- `cargo fmt --all --check`: ✅ clean

**Next:** Add WASM conformance tests (runnable via `wasm-pack test --node`), a `package.json` for
`@iscc/wasm`, and a CI job for the WASM target. The crate compiles and mirrors the napi/PyO3 pattern
exactly — testing infrastructure is the next milestone.

**Notes:** No unit tests in the WASM crate itself — wasm-bindgen functions can only be tested in a
WASM runtime (e.g., `wasm-pack test --node`), which requires `wasm-pack` tooling and a different
test harness. This is a scaffold step; conformance tests will follow in a dedicated work package.
The `wasm32-unknown-unknown` target was installed via `rustup target add` during this step.
