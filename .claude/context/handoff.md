## 2026-02-25 — Review of: Gate iscc-wasm `conformance_selftest` behind Cargo feature

**Verdict:** PASS

**Summary:** Clean, minimal implementation adding a `conformance` Cargo feature to iscc-wasm that
gates the `conformance_selftest` WASM export. All 54 wasm-pack tests pass with the feature enabled,
CI workflow correctly updated, and no quality gate circumvention.

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` passes all 54 tests (9
    conformance + 45 unit)
- [x] `cargo clippy -p iscc-wasm -- -D warnings` clean
- [x] `grep -q '^\[features\]' crates/iscc-wasm/Cargo.toml` exits 0 — features section exists
- [x] `grep -q 'cfg.*feature.*conformance' crates/iscc-wasm/src/lib.rs` exits 0 — feature gate
    present
- [x] `grep -q '\-\-features conformance' .github/workflows/ci.yml` exits 0 — CI updated

**Issues found:**

- (none)

**Next:** Remaining `[low]` issues: iscc-wasm stale CLAUDE.md (DataHasher/InstanceHasher documented
as unbound), iscc-jni exception mapping, and TypeScript port evaluation. Define-next should pick the
next highest-value item from the remaining low-priority issues.

**Notes:** The advance agent correctly identified that next.md's suggested CI syntax
`wasm-pack test --node crates/iscc-wasm -- --features conformance` (with `--` separator) is wrong —
`--` in wasm-pack passes args to wasm-bindgen-test-runner, not to cargo. The correct form is
`wasm-pack test --node crates/iscc-wasm --features conformance`. This is now documented in
learnings.md. Issue #3 (`conformance_selftest unconditionally exported`) resolved and deleted.
