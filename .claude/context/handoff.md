## 2026-02-23 — Add DataHasher/InstanceHasher streaming classes to WASM bindings

**Done:** Added `DataHasher` and `InstanceHasher` streaming classes to the WASM bindings crate,
bringing WASM from 21/23 to 23/23 Tier 1 parity. Both classes follow the `Option<Inner>` pattern
with `new() → update() → finalize()` semantics, matching the napi-rs binding pattern adapted for
wasm-bindgen.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added `DataHasher` and `InstanceHasher` structs with
    `#[wasm_bindgen]` constructor, `update`, and `finalize` methods. Includes `Default` impls to
    satisfy clippy `new_without_default` lint.
- `crates/iscc-wasm/tests/unit.rs`: Added 14 new wasm_bindgen_test cases — 7 for DataHasher (basic
    usage, matches gen function, multi-update, empty data, finalize-once, update-after-finalize
    error, default bits) and 7 for InstanceHasher (same test matrix).

**Verification:**

- [x] `cargo build -p iscc-wasm --target wasm32-unknown-unknown` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `cargo test --workspace` passes with 280 tests (no regressions)
- [x] `wasm-pack test --node crates/iscc-wasm` passes all 54 tests (9 conformance + 45 unit — 31
    existing + 14 new)
- [x] `mise run check` passes all 14 pre-commit hooks

**Next:** The only remaining binding gap is C FFI, which needs `DataHasher` and `InstanceHasher`
with the opaque pointer lifecycle pattern (`*mut T` + create/update/finalize/free). This is more
involved than the class-based bindings since it requires explicit memory management functions.

**Notes:** wasm-bindgen allows using `finalize` directly as a method name (no lifecycle conflict
like napi-rs's `ObjectFinalize`). The WASM structs use the same names as core types (`DataHasher`,
`InstanceHasher`) since they're in a separate crate — no prefix needed. `Default` impls are the
proper fix for clippy's `new_without_default` lint (same pattern used in napi-rs bindings).
