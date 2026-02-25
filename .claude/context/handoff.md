## 2026-02-25 — Gate iscc-wasm `conformance_selftest` behind Cargo feature

**Done:** Added `conformance` Cargo feature to iscc-wasm that gates the `conformance_selftest` WASM
export. Production WASM binaries no longer include the selftest function (and its embedded JSON
vectors/parsing logic) unless explicitly opted in. CI workflow updated to enable the feature during
testing.

**Files changed:**

- `crates/iscc-wasm/Cargo.toml`: added `[features]` section with `conformance = []`
- `crates/iscc-wasm/src/lib.rs`: added `#[cfg(feature = "conformance")]` gate on
    `conformance_selftest` export
- `crates/iscc-wasm/tests/unit.rs`: added `#[cfg(feature = "conformance")]` gate on
    `test_conformance_selftest_returns_true` test
- `.github/workflows/ci.yml`: changed wasm-pack test command to
    `wasm-pack test --node crates/iscc-wasm --features conformance`

**Verification:** All 54 wasm-pack tests pass (9 conformance + 45 unit). Clippy clean. All 14
pre-commit hooks pass. All 5 verification criteria from next.md satisfied:

- `[x]` `wasm-pack test --node crates/iscc-wasm --features conformance` passes all 54 tests
- `[x]` `cargo clippy -p iscc-wasm -- -D warnings` clean
- `[x]` `grep -q '^\[features\]' crates/iscc-wasm/Cargo.toml` exits 0
- `[x]` `grep -q 'cfg.*feature.*conformance' crates/iscc-wasm/src/lib.rs` exits 0
- `[x]` `grep -q '\-\-features conformance' .github/workflows/ci.yml` exits 0

**Next:** Remaining `[low]` issues: iscc-wasm stale CLAUDE.md (DataHasher/InstanceHasher documented
as unbound), iscc-jni exception mapping, and TypeScript port evaluation.

**Notes:** The next.md template suggested
`wasm-pack test --node crates/iscc-wasm -- --features conformance` (with `--` separator), but this
is incorrect — `--` in wasm-pack passes args to the test runner binary (wasm-bindgen-test-runner),
not to cargo. The correct syntax is `wasm-pack test --node crates/iscc-wasm --features conformance`
(features as extra options after the path). This was discovered and fixed during implementation. The
issue #3 in issues.md (`conformance_selftest unconditionally exported`) is now resolved by this
change.
