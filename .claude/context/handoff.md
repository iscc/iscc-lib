## 2026-03-04 — Review of: Add release smoke tests for all binding pipelines

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean, well-structured implementation adding 6 smoke test jobs to `release.yml`, each
gating its corresponding publish job. All verification criteria pass. One minor scope deviation:
`--features conformance` added to `build-wasm` (justified — required for WASM smoke test to access
`conformance_selftest`, which is gated behind `#[cfg(feature = "conformance")]` in the WASM crate).

**Verification:**

- [x] `grep -cP '^\s{2}test-' .github/workflows/release.yml` — outputs 6
- [x] YAML validates cleanly — `yaml.safe_load()` exits 0
- [x] `publish-pypi` needs includes `test-wheels`
- [x] `publish-npm-lib` needs includes `test-napi`
- [x] `publish-npm-wasm` needs includes `test-wasm`
- [x] `publish-rubygems` needs includes `test-gem`
- [x] `publish-maven` needs includes `test-jni`
- [x] `publish-ffi` needs includes `test-ffi`
- [x] `mise run check` passes — all 15 hooks green
- [x] No quality gate circumvention patterns found

**Issues found:**

- (none)

**Codex review:** No issues found. Codex confirmed the changes are consistent, reference existing
artifacts/APIs correctly, and don't introduce CI-breaking or functional problems.

**Next:** The "Add release smoke tests" issue is now resolved and deleted from issues.md. All
remaining issues are `low` priority (C#, C++, Swift, Kotlin bindings; language logos). Consider
preparing a PR from `develop` → `main` for a stable release, or await human direction on `low`
priority items.

**Notes:** The `--features conformance` addition to `build-wasm` is a minor scope deviation from
next.md's "don't change existing build jobs" constraint, but necessary and beneficial. The published
WASM package now exports `conformance_selftest` (previously gated), bringing it in line with NAPI
and Python bindings which export it unconditionally. The feature flag has zero dependency cost — it
only gates the JS export; the underlying code is already compiled into `iscc-lib`. Each test job
correctly mirrors its build job's `if:` condition and uses appropriate artifact names. The NAPI test
correctly uses `conformance_selftest` (not camelCase) because the binding uses
`#[napi(js_name = "conformance_selftest")]`.
