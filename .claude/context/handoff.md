## 2026-03-22 — Review of: Guard Swift release build against main/tag SHA mismatch

**Verdict:** PASS

**Summary:** The advance agent correctly added a provenance guard step to the `build-xcframework`
job in `release.yml`. The guard is minimal (11 lines of YAML), correctly placed after checkout and
before expensive build steps, and properly conditioned on tag-triggered runs only. All verification
criteria pass. The resolved "Swift release job checks out `ref: main`" issue has been deleted from
issues.md.

**Verification:**

- [x] `yaml.safe_load` exits 0 — valid YAML confirmed
- [x] `grep -c 'Verify main matches tag'` returns 1 — guard step exists exactly once
- [x] `startsWith(github.ref, 'refs/tags/v')` confirmed — step conditional on tag triggers only
- [x] `github.sha` present in step — tag SHA comparison confirmed
- [x] `git rev-parse HEAD` present in step — main HEAD comparison confirmed
- [x] `mise run format` — no changes (clean)
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** One advisory finding: if a tag-triggered build fails for transient reasons and
main advances before "Re-run failed jobs" is clicked, the guard blocks the retry. This is by design
— the `workflow_dispatch` re-trigger path (`--ref main -f swift=true`) bypasses the guard because
`github.ref` is `refs/heads/main`, not a tag. The error message explicitly guides operators to this
recovery path. No action needed.

**Next:** One normal-priority issue remains: "CI does not exercise root Package.swift." This would
add a manifest-resolution smoke check on the macOS CI runner to validate the consumer-facing
`Package.swift` at the repo root. There's also the benchmarks documentation gap (speedup factors not
published). The root `Package.swift` CI smoke test is the last normal issue.

**Notes:** With the `ref: main` race condition resolved, the Swift release workflow is hardened
against provenance mismatch. The remaining normal issue (root Package.swift CI) is defense-in-depth
— the release workflow already patches the checksum at publish time, so regressions are caught at
release even without CI. The low-priority docs logo issue remains, CID-skipped.
