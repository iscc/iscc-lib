## 2026-03-22 — Add root Package.swift manifest smoke test to CI

**Done:** Added a `swift package dump-package` step to the `swift` CI job that validates the
consumer-facing root `Package.swift` manifest parses correctly. The step runs early (after
rust-cache, before the expensive UniFFI build) and requires no `working-directory` since it targets
the repo root.

**Files changed:**

- `.github/workflows/ci.yml`: Added "Validate root Package.swift manifest" step at line 239-240 in
    the `swift` job, between `Swatinem/rust-cache@v2` and "Build UniFFI native library"

**Verification:**

- YAML valid: `yaml.safe_load()` exits 0
- `grep -c 'dump-package'` returns 1 (step exists exactly once)
- `grep -B2 'dump-package' | grep -c 'working-directory'` returns 0 (runs at repo root)
- `grep -B5 'dump-package' | grep 'Validate root'` confirms step name
- `dump-package` step appears BEFORE "Build UniFFI native library" (visual inspection confirmed)
- `mise run format` produces no changes
- `mise run check` — all 15 pre-commit hooks pass

**Next:** The "CI does not exercise root Package.swift" issue in issues.md should be marked resolved
by the review agent. The only remaining issues are "Add programming language logos to docs site"
(low priority, CID-skipped) and the benchmarks documentation gap.

**Notes:** No tests to write — this is a CI-only YAML change. The `dump-package` command parses the
manifest and outputs JSON but does NOT resolve/download binary targets, so the PLACEHOLDER checksum
on develop is fine. This is exactly the right level of validation for the develop branch.
