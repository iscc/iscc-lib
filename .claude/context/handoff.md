## 2026-02-23 — Add DataHasher/InstanceHasher streaming classes to Node.js bindings

**Done:** Added `DataHasher` and `InstanceHasher` streaming classes to the napi-rs Node.js bindings,
bringing Tier 1 parity from 21/23 to 23/23 (full parity with Python). Both classes follow the
`new() → update() → finalize()` pattern with finalize-once semantics using `Option<Inner>`.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added `NapiDataHasher` and `NapiInstanceHasher` structs with
    `#[napi]` constructor, `update`, and `finalize` methods. Both wrap the core Rust types in
    `Option<T>` for finalize-once enforcement. Added `Default` impls to satisfy clippy, and made
    structs `pub` to avoid dead_code warnings in test targets (napi macro glue only references
    methods in cdylib builds).
- `crates/iscc-napi/__tests__/functions.test.mjs`: Added 14 new tests across
    `describe('DataHasher')` (7 tests) and `describe('InstanceHasher')` (7 tests) — basic usage, gen
    function matching, multi-update, empty data, finalize-once errors, and default bits.

**Verification:**

- `cargo build -p iscc-napi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test --workspace` passes all 280 tests (no regressions)
- `node --test crates/iscc-napi/__tests__/functions.test.mjs` passes all 57 tests (43 existing + 14
    new)
- Smoke test: `node -e "..."` with both `DataHasher` and `InstanceHasher` prints valid ISCC strings
- `mise run check` passes all 14 pre-commit hooks

**Next:** Node.js bindings are now at full 23/23 Tier 1 parity. Next candidates:

1. **Add streaming hashers to WASM bindings** — wasm-bindgen has `#[wasm_bindgen]` class support;
    similar pattern to napi-rs
2. **Add streaming hashers to C FFI bindings** — requires opaque pointer pattern (`*mut DataHasher`
    \+ create/update/finalize/free lifecycle)
3. **Structured return types for gen functions** — returning full result objects instead of just
    `.iscc` strings across all bindings

**Notes:** napi-rs classes with `#[napi]` attribute generate N-API glue code only for cdylib
targets, not test targets. This causes dead_code warnings when building with `--all-targets`
(clippy). Making the structs `pub` resolves this by marking them as externally reachable. The
`Default` impl requirement is a standard clippy lint for any `pub` type with a `new() -> Self`
method.
