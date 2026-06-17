## 2026-06-17 — Review of: Add Phase 1 Rust coverage CI job (`cargo llvm-cov` LCOV artifact) + `mise run coverage` task

**Verdict:** PASS

**Summary:** The advance agent added a standalone `coverage` CI job that builds an LCOV report for
`iscc-lib` via `cargo llvm-cov`, mirrored the exact command as `mise run coverage`, gitignored
`lcov.info`, and flipped the Phase 1 checkbox in `ci-cd.md`. The diff is CI-config-only — no source,
API, conformance, or perf surface touched — matches next.md exactly, and all local verification
passes. Clean, low-risk Phase 1 of the coverage/CRAP gate issue.

**Verification:**

- [x] `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` exits 0, non-empty report —
    cargo-llvm-cov 0.8.7 installed; report produced (5156 lines / 137KB)
- [x] `mise run coverage` exits 0 and produces `lcov.info` — re-ran; 22 lib tests pass under
    instrumentation, 5156-line report
- [x] `mise tasks | grep '^coverage'` shows the task — present with description
- [x] `git status --porcelain` does NOT list `lcov.info` — gitignored (`lcov.info` added under "Unit
    test / coverage reports")
- [x] `mise run check` passes on edited files — all 15 hooks Passed; no out-of-scope context-file
    reflow this run
- [x] `grep cargo-llvm-cov/llvm-cov ci.yml` shows the `coverage` job wiring — job + toolchain +
    install-action + run + upload all present
- [x] CI job count — 17 YAML job entries (was 16); python-test matrix `['3.10','3.14']` → 18 actual
    GitHub Actions jobs. (Advance handoff says "17 jobs"; that counts YAML entries, not matrix
    expansion. Not a defect — noted for accuracy.)
- [~] Deferred: new `Coverage` job appears green and the other jobs stay green — confirmed on the
    next CI run after push (cannot verify pre-push)

**Issues found:**

- (none) — scope-disciplined, no gate weakening (this *adds* a gate), no API-break, no hot-path
    change. The "Add Rust coverage + CRAP-metric quality gate" issue stays open: only Phase 1 (LCOV
    artifact) is delivered; Phase 2 (`cargo crap` report-only + SARIF) and Phase 3
    (`--fail-regression` baseline) remain.

**Codex review:** Confirmed the coverage CI job, mise task, and gitignore update are consistent with
Phase 1 scope and that the coverage command succeeds locally. No discrete regressions identified.

**Next:** Two unstarted `normal` backlog gates remain and pair naturally as self-contained CI steps:

1. **Phase 2 report-only `cargo crap`** — install pinned `cargo-crap` via `cargo binstall`, run vs
    `lcov.info` with `--format github` annotations + SARIF upload to Code Scanning, add
    `.cargo-crap.toml` and the `mise run crap` task (non-failing). Builds directly on the Phase 1
    `lcov.info` just landed.
2. **`iai-callgrind` perf gate** — instruction-count benches for the hot `gen_*_v0`/hashing/CDC/
    MinHash paths + committed baseline, fail on >10% regression. The PyO3 0.23→0.29 bump (`normal`)
    is the remaining non-CI option but is a multi-step migration. Phase 2 CRAP is the most
    incremental follow-on.

**Notes:**

- CI does NOT use `mise` — it calls `cargo` directly. The `mise run coverage` task is local
    reproducibility only; the command string is identical to the CI step, so they cannot drift
    silently. If `cargo-llvm-cov` ever proves flaky on the shared runner, the documented fallback is
    `continue-on-error: true` — but Phase 1 intentionally omits it so test failures under
    instrumentation stay visible.
- `lcov.info` is 137KB — well under the `check-added-large-files` 256KB threshold even if a future
    step needs to commit a sample, but it remains gitignored for now.
- No version-sync impact (no manifest versions changed); no new release inputs.
