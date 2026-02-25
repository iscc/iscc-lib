# Next Work Package

## Step: Gate iscc-wasm `conformance_selftest` behind Cargo feature

## Goal

Gate the `conformance_selftest` WASM export behind an opt-in Cargo feature (`conformance`) so that
production WASM binaries don't include the embedded JSON test vectors and parsing logic (~543 lines
of conformance module + vendored `data.json`). This reduces bundle size for browser consumers who
don't need diagnostics.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-wasm/Cargo.toml` — add `[features]` section with `conformance = []`
    - `crates/iscc-wasm/src/lib.rs` — gate `conformance_selftest` export with
        `#[cfg(feature = "conformance")]`
    - `crates/iscc-wasm/tests/unit.rs` — gate `test_conformance_selftest_returns_true` with
        `#[cfg(feature = "conformance")]`
    - `.github/workflows/ci.yml` — update wasm-pack test command to pass `--features conformance`
- **Reference**:
    - `crates/iscc-wasm/src/lib.rs` (lines 192–200 — the `conformance_selftest` export)
    - `crates/iscc-wasm/tests/unit.rs` (lines 121–129 — the unit test calling it)
    - `.github/workflows/ci.yml` (line 77 — current wasm-pack test invocation)

## Not In Scope

- Feature-gating the conformance module in `iscc-lib` itself (the Rust core should always include
    it; the WASM linker performs dead code elimination when nothing references it)
- Updating `crates/iscc-wasm/CLAUDE.md` stale content about DataHasher/InstanceHasher (separate
    [low] issue — next step)
- Measuring or documenting the actual binary size reduction
- Adding `wasm-opt` size optimization settings to the release profile
- Changing the conformance test file (`tests/conformance.rs`) — those tests exercise individual gen
    functions independently and should always run

## Implementation Notes

1. **Cargo.toml** — add below `[dev-dependencies]`:

    ```toml
    [features]
    conformance = []
    ```

2. **lib.rs** — wrap the existing `conformance_selftest` export:

    ```rust
    #[cfg(feature = "conformance")]
    #[wasm_bindgen]
    pub fn conformance_selftest() -> bool {
        iscc_lib::conformance_selftest()
    }
    ```

    Keep the section comment `// ── Conformance ───` intact.

3. **unit.rs** — gate the single test:

    ```rust
    #[cfg(feature = "conformance")]
    #[wasm_bindgen_test]
    fn test_conformance_selftest_returns_true() {
        assert!(
            iscc_wasm::conformance_selftest(),
            "conformance selftest should pass"
        );
    }
    ```

4. **ci.yml** — change the WASM test run command from:

    ```yaml
    run: wasm-pack test --node crates/iscc-wasm
    ```

    to:

    ```yaml
    run: wasm-pack test --node crates/iscc-wasm -- --features conformance
    ```

    This ensures CI still tests the conformance selftest via the WASM export.

## Verification

- `wasm-pack test --node crates/iscc-wasm -- --features conformance` passes all 54 tests
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `grep -q '^\[features\]' crates/iscc-wasm/Cargo.toml` exits 0 (features section exists)
- `grep -q 'cfg.*feature.*conformance' crates/iscc-wasm/src/lib.rs` exits 0 (feature gate present)
- `grep -q '\-\-features conformance' .github/workflows/ci.yml` exits 0 (CI updated)

## Done When

All 5 verification criteria pass — the `conformance_selftest` WASM export is gated behind an opt-in
feature, CI enables the feature for testing, and all existing tests still pass.
