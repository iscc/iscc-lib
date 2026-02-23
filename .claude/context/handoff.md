## 2026-02-23 — Review of: Add DataHasher/InstanceHasher streaming classes to Node.js bindings

**Verdict:** PASS_WITH_NOTES

**Summary:** DataHasher and InstanceHasher streaming classes added to napi-rs Node.js bindings,
bringing Node.js from 21/23 to 23/23 Tier 1 parity with Python. Implementation follows the
`Option<Inner>` finalize-once pattern exactly as specified. Code quality is high — mirrors the PyO3
binding pattern, comprehensive tests (14 new), all quality gates pass. One process note: the advance
agent added 5 per-crate CLAUDE.md files in a separate commit that was not in scope for this work
package.

**Verification:**

- [x] `cargo build -p iscc-napi` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `cargo test --workspace` passes with 280 tests (no regressions)
- [x] `node --test crates/iscc-napi/__tests__/functions.test.mjs` passes all 57 tests (43 existing +
    14 new)
- [x] Smoke test: `node -e "..."` with both DataHasher and InstanceHasher prints valid ISCC strings
    (`ISCC:GAAWSPUDF7GJWQVD`, `ISCC:IAA6VDYWHWZYNAUS`)
- [x] `mise run check` passes all 14 pre-commit hooks
- [x] Node.js conformance tests still pass (all 57 tests across 14 suites)

**Issues found:**

- Out-of-scope commit `c297adf` added 5 per-crate CLAUDE.md files (734 lines) that were not
    requested in next.md. These are benign documentation/context files that don't affect behavior,
    but the advance agent should stay within the defined scope. Not blocking since the feature work
    itself is clean.

**Next:** Two non-Python bindings remain at 21/23 — WASM and C FFI both need DataHasher and
InstanceHasher streaming classes. WASM is the natural next target since `#[wasm_bindgen]` supports
class methods similarly to napi-rs. C FFI requires the opaque pointer pattern (`*mut DataHasher` +
create/update/finalize/free lifecycle) which is more involved.

**Notes:** The `Default` impl and `pub` visibility on `NapiDataHasher`/`NapiInstanceHasher` are
legitimate resolutions for clippy lints on napi-rs classes (not suppression). The napi macro
generates glue code only for cdylib targets, so the structs appear "dead" in `--all-targets` builds.
Making them `pub` is the standard workaround.
