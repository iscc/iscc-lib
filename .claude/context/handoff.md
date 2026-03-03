## 2026-03-03 — Review of: Adapt conformance_selftest to skip disabled features

**Verdict:** PASS

**Summary:** The advance agent correctly made `conformance_selftest()` always available regardless
of feature flags by removing the module-level `#[cfg(feature = "meta-code")]` gates from `lib.rs`
and adding granular `#[cfg]` gates on the individual `run_meta_tests`/`run_text_tests` functions and
their imports/calls in `conformance.rs`. The implementation is minimal, correct, and cleanly scoped.

**Verification:**

- [x] `cargo test -p iscc-lib` — 314 tests pass (258 unit + 31 streaming + 24 utils + 1 doctest)
- [x] `cargo test -p iscc-lib --no-default-features` — 250 tests pass; conformance_selftest runs 7
    of 9 sections
- [x] `cargo test -p iscc-lib --no-default-features --features text-processing` — 284 tests pass;
    runs 8 of 9 sections
- [x] `cargo clippy -p iscc-lib -- -D warnings` — clean (default features)
- [x] `cargo clippy -p iscc-lib --no-default-features -- -D warnings` — clean
- [x] `cargo clippy -p iscc-lib --all-features -- -D warnings` — clean
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention (no `#[allow]`, `#[ignore]`, threshold changes)

**Issues found:**

- (none)

**Next:** Issue #16 has one remaining sub-task: add CI workflow jobs for feature matrix testing. The
`.github/workflows/ci.yml` needs new steps (or a matrix expansion) to run
`cargo test -p iscc-lib --no-default-features`, `cargo test -p iscc-lib --all-features`, and
`cargo test -p iscc-lib --no-default-features --features text-processing`. This is a YAML-only
change with no Rust code modifications. Completing this closes issue #16.

**Notes:** The Codex review was run against the define-next commit (HEAD~1) and found no code issues
(expected — that commit only changed context files). The advance commit at HEAD was independently
verified through the 6 criteria above. The existing `test_conformance_selftest_passes` test works
correctly across all feature combinations without modification.
