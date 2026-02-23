## 2026-02-23 — Review of: Add DataHasher/InstanceHasher opaque pointer FFI to C bindings

**Verdict:** PASS

**Summary:** DataHasher and InstanceHasher streaming types added to C FFI bindings via the opaque
pointer lifecycle pattern (new/update/finalize/free), bringing C FFI from 21/23 to 23/23 Tier 1
parity. All four binding targets (Python, Node.js, WASM, C FFI) now have complete Tier 1 coverage.
Implementation is clean, follows the exact spec from next.md, and all quality gates pass.

**Verification:**

- [x] `cargo test -p iscc-ffi` passes — 62 tests (50 existing + 12 new), all OK
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean — no warnings
- [x] `cargo test --workspace` passes — 292 tests across all crates, no regressions
- [x] C test program compiles and passes — 30/30 assertions (20 existing + 10 new), exit 0
- [x] `mise run check` passes — all 14 pre-commit hooks green

**Issues found:**

- (none)

**Next:** All four binding targets now have 23/23 Tier 1 parity. Consider these next priorities in
order: (1) updating state.md to reflect C FFI at 23/23, (2) structured return types for gen
functions across non-Python bindings (Node.js, WASM, C FFI currently return only `.iscc` strings
while Python returns full result dicts), (3) splitting `iscc-ffi/src/lib.rs` into submodules (now
~1,880 lines, exceeding the ~1,500 line guidance in the crate's CLAUDE.md), (4) documentation
branding (ISCC colors, logo, favicon), or (5) OIDC publishing configuration.

**Notes:** The implementation uses the `Option<Inner>` wrapper pattern consistently across all
binding targets (Python `#[pyclass]`, napi-rs `#[napi]`, wasm-bindgen `#[wasm_bindgen]`, and now C
FFI opaque pointers). The C FFI variant is the most involved because it requires manual
`Box::into_raw`/`Box::from_raw` lifecycle management, but the advance agent handled this correctly
with comprehensive NULL checks, error reporting via thread-local last-error, and proper memory
management. The `iscc-ffi/src/lib.rs` file is now ~1,880 lines, which exceeds the crate's CLAUDE.md
guidance of ~1,500 lines — the file should be split into submodules in a future iteration.
