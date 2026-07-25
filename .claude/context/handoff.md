# Handoff

## 2026-07-25 — Refresh the 97 GitHub Actions `uses:` refs in release.yml

**Done:** Bumped the nine stale action refs in `.github/workflows/release.yml` to their current
floating majors as a pure find/replace of full `uses:` values: `checkout` v4→v7, `download-artifact`
v4→v8 and `upload-artifact` v4→v7 (moved as a pair in this one commit), `setup-java` v4→v5,
`setup-node` v4→v7, `setup-dotnet` v4→v6, `setup-python` v5→v7, `softprops/action-gh-release` v2→v3,
`cache` v4→v6. 73 of the 97 `uses:` lines changed; nothing else in the file was touched.

**Files changed:**

- `.github/workflows/release.yml`: 9 distinct `uses:` refs bumped across 73 lines (73 insertions /
    73 deletions; verified via `git diff` that every changed line is a `uses:` line — no `if:`,
    `needs:`, `with:`, `env:` or `run:` block moved)

**Verification:** All next.md criteria pass, each reproduced in this session:

- `uses:` histogram diff against the expected 18-ref multiset → empty diff, exit 0; total `uses:`
    count still exactly 97 (no step added, removed or duplicated)
- `grep -c 'setup-uv'` → 0 (none added, per Not-In-Scope)
- Guard invariant script → `release.yml guards OK` both **before and after** the edit (29 jobs,
    `prepare-release` bare `inputs.version != ''`, all 28 others match the exact
    `!cancelled() && !failure()` shape, token counts identical)
- Artifact-wiring script → `unmatched: []` both before and after (11 upload names, 20 downloads,
    every `name:`/`pattern:` resolves)
- `actionlint@v1.7.7 .github/workflows/release.yml` → exit 0, no output
- `prek run check-yaml` and `prek run yamlfix` on the file → both `Passed`, no `files were modified`
- `mise run check` → 15/15 hooks Passed, exit 0
- `git status --porcelain` → only `.github/workflows/release.yml` plus runner-owned
    `iterations.jsonl`. No Rust source touched — no CRAP/iai baseline, `cargo deny` or semver
    refresh owed; no API surface change.
- Pre-edit sweep confirmed none of the nine old ref strings appeared anywhere except `uses:` lines
    (no comments to update)

**Next:** The tooling/CI thread is now genuinely exhausted (per the previous review's assessment).
The defensible next move is the Rust-core-only Unicode boundary fixture (format + loader, three
single-code-point cases, no binding wiring), which the parked HUMAN REVIEW sequence-ordering
question does not affect. Also still open as a review-filed follow-up candidate: landing the guard-
shape check as a `scripts/` file wired into prek/CI (explicitly out of scope here).

**Notes:**

- **Statically verified only.** `release.yml` is `workflow_dispatch`-only — no CI or CID push
    exercises it. First real-world confirmation of these majors in this file is the next release
    run. Six of the nine targets already run green in `ci.yml` with the same inputs; the other three
    (`download-artifact@v8`, `action-gh-release@v3`, `cache@v6`) rest on the scoping analysis in
    next.md (no `artifact-ids:` uses; `with:` keys unchanged in the new majors).
- `download-artifact@v8` defaults `digest-mismatch: error` (was warn) — intentionally left strict,
    per next.md.
- `rubygems/configure-rubygems-credentials@main` remains on a floating branch — left untouched per
    Not-In-Scope; review may file the supply-chain pin question as a follow-up.
- Trimmed advance MEMORY.md from 142 to under its 140-line budget (the Codex P3 from last iteration)
    by compressing entries whose detail already lives in topic files; also folded the iter-140
    refresh into the release-workflow entry.
