# Next Work Package

## Step: Adapt conformance_selftest to skip disabled features

## Goal

Make `conformance_selftest()` always available regardless of feature flags, skipping test sections
for disabled code types instead of being entirely absent. This resolves the first remaining sub-task
of issue #16.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/lib.rs` — remove `#[cfg(feature = "meta-code")]` from the
        `pub mod   conformance` declaration (line 9) and the
        `pub use conformance::conformance_selftest` re-export (line 22)
    - `crates/iscc-lib/src/conformance.rs` — gate `run_meta_tests` and `run_text_tests` behind their
        respective features; update imports and the `conformance_selftest()` body to conditionally
        call/skip these functions
- **Reference**:
    - `crates/iscc-lib/src/conformance.rs` — current implementation (all 9 run\_\*\_tests functions)
    - `crates/iscc-lib/src/lib.rs` — current feature gates
    - `.claude/context/issues.md` — issue #16 requirements

## Not In Scope

- CI workflow changes for feature matrix testing — that's the next step after this
- Changing the conformance test vectors or data.json
- Modifying the existing `#[cfg(test)] mod tests` block in conformance.rs — the test there
    (`test_conformance_selftest_passes`) should continue to work under all feature combinations
- Propagating feature flag changes to binding crates (not needed — they always use default features)
- Refactoring the repetitive `run_*_tests` functions into a generic helper

## Implementation Notes

**Changes to `lib.rs`:**

- Remove `#[cfg(feature = "meta-code")]` from line 9 (`pub mod conformance`) and line 22
    (`pub use   conformance::conformance_selftest`)
- The module and its public symbol should always be available

**Changes to `conformance.rs`:**

1. **Imports**: Gate `gen_meta_code_v0` behind `#[cfg(feature = "meta-code")]` and
    `gen_text_code_v0` behind `#[cfg(feature = "text-processing")]`. All other 7 imports are always
    available.

2. **`run_meta_tests`**: Gate the entire function behind `#[cfg(feature = "meta-code")]`.

3. **`run_text_tests`**: Gate the entire function behind `#[cfg(feature = "text-processing")]`.

4. **`conformance_selftest()` body**: Use conditional compilation for the meta and text calls:

    ```rust
    #[cfg(feature = "meta-code")]
    { passed &= run_meta_tests(&data); }
    #[cfg(feature = "text-processing")]
    { passed &= run_text_tests(&data); }
    // remaining 7 always run unconditionally
    ```

5. **Module docstring**: Update to mention that disabled features are skipped, not failed.

6. **Test in `mod tests`**: The existing `test_conformance_selftest_passes` test should work
    unchanged — it calls `conformance_selftest()` and asserts true, which is correct regardless of
    which features are enabled (skipped sections don't fail).

## Verification

- `cargo test -p iscc-lib` passes (314 tests, default features — conformance test still present)
- `cargo test -p iscc-lib --no-default-features` passes — `conformance_selftest` is callable and
    returns `true` (runs 7 of 9 test sections, skips meta and text)
- `cargo test -p iscc-lib --no-default-features --features text-processing` passes —
    `conformance_selftest` runs 8 of 9 test sections (skips meta only)
- `cargo clippy -p iscc-lib -- -D warnings` clean (default features)
- `cargo clippy -p iscc-lib --no-default-features -- -D warnings` clean
- `cargo clippy -p iscc-lib --all-features -- -D warnings` clean

## Done When

All six verification commands pass — `conformance_selftest()` is always available and skips disabled
code types gracefully under every feature combination.
