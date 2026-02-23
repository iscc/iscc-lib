## 2026-02-23 — Review of: Scaffold WASM bindings crate with all 9 gen functions

**Verdict:** PASS

**Summary:** The WASM binding crate was scaffolded correctly with all 9 `gen_*_v0` wrappers
following the identical thin-wrapper pattern as PyO3 and napi-rs crates. Type mappings are correct —
simple types use direct wasm-bindgen mapping, complex array types (`gen_video_code_v0`,
`gen_mixed_code_v0`, `gen_iscc_code_v0`) use `JsValue` + `serde_wasm_bindgen`. All verification
passes: 143 Rust tests, clippy clean, fmt clean, wasm32-unknown-unknown compilation succeeds.

**Issues found:**

- `js-sys` is declared as a dependency in `crates/iscc-wasm/Cargo.toml` but never imported or used
    in `src/lib.rs`. It was prescribed by next.md and may be useful for future WASM features
    (returning typed arrays, Date objects), so leaving it as-is is acceptable — but it should be
    removed if still unused after WASM conformance tests are added.

**Next:** Add WASM conformance tests and a CI job. This requires `wasm-pack` tooling and a
`wasm-pack test --node` harness. Also add `package.json` for the `@iscc/wasm` npm package. The
conformance test data is already vendored at `crates/iscc-lib/tests/data.json` and shared by all
binding test suites. Consider also updating `state.md` to reflect the WASM scaffold milestone.

**Notes:** The three binding crates (PyO3, napi-rs, wasm-bindgen) now share an identical
thin-wrapper pattern with no logic in the binding layer. `js-sys` may be needed when returning
structured JS objects from WASM functions (instead of JSON strings). The `wasm-opt` profile metadata
mentioned in `notes/02-language-bindings.md`
(`[package.metadata.wasm-pack.profile.release] wasm-opt = ["-Os"]`) was not added — this is a build
optimization for release, appropriate to add when the CI/build pipeline for WASM is set up.
