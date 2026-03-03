## 2026-03-03 — Adapt conformance_selftest to skip disabled features

**Done:** Made `conformance_selftest()` always available regardless of feature flags. Removed
`#[cfg(feature = "meta-code")]` gates from the `pub mod conformance` declaration and
`pub use conformance::conformance_selftest` re-export in `lib.rs`. In `conformance.rs`, gated
`run_meta_tests` behind `meta-code`, `run_text_tests` behind `text-processing`, and wrapped their
calls in the `conformance_selftest()` body with corresponding `#[cfg]` blocks. Updated module and
function docstrings to reflect skip-not-fail behavior.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: removed `#[cfg(feature = "meta-code")]` from conformance module
    declaration (line 9) and `conformance_selftest` re-export (line 22)
- `crates/iscc-lib/src/conformance.rs`: gated `gen_meta_code_v0` import behind `meta-code`,
    `gen_text_code_v0` import behind `text-processing`; gated `run_meta_tests` and `run_text_tests`
    functions behind their respective features; wrapped calls in `conformance_selftest()` body with
    `#[cfg]` blocks; updated module docstring and function docstring

**Verification:**

- `cargo test -p iscc-lib` — 314 tests pass (default features, conformance test present)
- `cargo test -p iscc-lib --no-default-features` — 250 tests pass (conformance_selftest callable,
    runs 7 of 9 test sections, skips meta and text)
- `cargo test -p iscc-lib --no-default-features --features text-processing` — 284 tests pass (runs 8
    of 9 test sections, skips meta only)
- `cargo clippy -p iscc-lib -- -D warnings` — clean (default features)
- `cargo clippy -p iscc-lib --no-default-features -- -D warnings` — clean
- `cargo clippy -p iscc-lib --all-features -- -D warnings` — clean
- `mise run check` — all pre-commit hooks pass
- `uv run prek run --stage pre-push --all-files` — all pre-push hooks pass

**Next:** Issue #16 has one remaining sub-task: add CI workflow jobs for feature matrix testing
(`--all-features`, `--no-default-features`, and per-feature testing). This is a `.github/workflows/`
change, not a Rust code change.

**Notes:** No surprises. The `cargo fmt` reordered the `#[cfg]`-gated imports above the main `use`
block — this is standard rustfmt behavior (cfg-gated items sort separately). The existing
`test_conformance_selftest_passes` test works unchanged under all feature combinations as expected.
