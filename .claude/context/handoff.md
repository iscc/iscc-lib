## 2026-06-17 — Review of: CRAP gate Phase 2 — report-only `cargo crap` (GitHub annotations + SARIF) + `.cargo-crap.toml` + `mise run crap`

**Verdict:** PASS

**Summary:** The advance agent extended the existing `coverage` CI job to run report-only
`cargo crap` (pinned `0.2.2`, installed via `cargo binstall`) against the Phase-1 `lcov.info` —
emitting `--format github` annotations and uploading a `--format sarif` report to Code Scanning —
plus a repo-root `.cargo-crap.toml`, a `mise run crap` task, and `.gitignore`/`ci-cd.md` updates.
Diff is tightly scoped (config + CI + docs only, no source/API surface), all verification criteria
pass, and the one deviation from next.md (the explicit `crates/iscc-lib/benches/**` exclude) is
justified and confirmed necessary.

**Verification:**

- [x] `cargo crap --lcov lcov.info` exits 0 (report-only) — 97 functions analyzed, none exceed
    threshold 30; highest is `gen_meta_code_v0` at CRAP 22.3
- [x] `mise run crap` exits 0, regenerates `lcov.info` via `depends=["coverage"]`, prints the table;
    `mise tasks | grep '^crap'` shows the task
- [x] SARIF valid — `cargo crap ... --format sarif --output /tmp/crap.sarif` then
    `jq -e '.runs[0].tool.driver.name'` → `"cargo-crap"`; written to /tmp, not the repo
- [x] Report contains only `iscc-lib` functions — no binding crates, `packages/`, or benches (grep
    for those paths returns nothing)
- [x] `grep -E 'cargo-crap|cargo crap|upload-sarif|security-events' ci.yml` shows install + both
    runs + SARIF upload + the `security-events: write` permission
- [x] `grep -E 'fail-above|fail-regression|baseline' ci.yml` returns nothing (strictly report-only)
- [x] `git status --porcelain` lists neither `lcov.info` nor `crap.sarif` (both gitignored)
- [x] `mise run check` passes — all 15 pre-commit hooks green, no out-of-scope context reflow
- [x] `ci-cd.md` Phase 2 checkboxes (411, 415, 416, 417) flipped to `[x]`; Phase 3 (413, 414) stay
    `[ ]`

**Issues found:**

- (none) — clean. Verified the next.md deviation: with `crates/iscc-lib/benches/**` removed from the
    exclude list, `bench_cdc_chunks` leaks in at CRAP 42.0 (no coverage, pure harness noise), so the
    explicit exclude is necessary, not dead config. The handoff documented this accurately.

**Codex review:** No regressions identified. "The CI, cargo-crap configuration, gitignore, and mise
task changes are consistent with the intended report-only CRAP workflow."

**Next:** Two unstarted `normal` v1.0.0 backlog gates remain — pick the most incremental:

1. **CRAP Phase 3 (regression gate)** — builds directly on this Phase 2 work: capture a baseline
    JSON (`cargo crap --format json --output`), run `--fail-regression --baseline`, and refresh the
    baseline on merges to `develop`. This is the enforcing step Phase 2 deliberately deferred.
2. **`iai-callgrind` perf-regression CI gate** — instruction-count benches for hot
    `gen_*_v0`/hashing/CDC/MinHash paths with a committed baseline failing on >10% regression.
3. **PyO3 0.23 → 0.29 bump** — clears two RustSec advisories shipped in the wheel; six-minor-version
    jump, migrate incrementally (start 0.23 → 0.24), scope to `crates/iscc-py/` only.

**Notes:**

- Phase 2 is intentionally non-failing — the report-only steps exit 0 regardless of scores, and with
    the benches exclude the highest CRAP (22.3) is below threshold 30, so CI emits **zero** GitHub
    annotations in practice. The annotation path will only activate if a function crosses 30.
- SARIF upload + GitHub annotations cannot be verified pre-push (require the live runner); the
    `upload-sarif@v3` step needs `security-events: write`, added at the `coverage` job level (does
    not affect other jobs). iscc-lib is public so Code Scanning is available.
- CI does NOT use `mise`; `mise run crap`/`coverage` mirror the CI commands for local repro only
    (identical command strings — cannot drift silently).
- `cargo-crap` 0.2.2 reads `.cargo-crap.toml` only when run from the repo root (config path is
    `.`-relative) — relevant if a future job changes CWD.
- Archived all per-binding propagation learnings to `learnings-archive.md` (all bindings met) to
    keep learnings.md under the 200-line budget; review MEMORY.md condensed to stay under 200.
