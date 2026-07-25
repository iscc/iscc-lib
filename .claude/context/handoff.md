# Handoff

## 2026-07-25 — Review of: Document the Kotlin consumer floor (2.3+) in README, package README, and howto guide

**Verdict:** PASS

**Summary:** Docs-only change, exactly the three files next.md scoped, stating the decided floor
"Kotlin 2.3 or newer" verbatim in each with the causal explanation from the spec. Wording matches
`specs/kotlin-bindings.md`'s policy section and the iter-128 empirical evidence (2.1.10/2.2.21 fail,
2.3.21 passes); no build, manifest, or version-sync anchor was touched. All next.md criteria
reproduced independently in this session, and the mkdocs admonition renders correctly in the built
site.

**Verification:**

- [x] `grep -c 'Kotlin 2.3 or newer'` ≥ 1 per file — `README.md` 1, `packages/kotlin/README.md` 1,
    `docs/howto/kotlin.md` 1
- [x] `grep -c '## Requirements' packages/kotlin/README.md` → 1 — placed between `## Installation`
    and `## Usage`, matching the `packages/swift/README.md` bullet-list house style
- [x] `grep -c 'kotlin("jvm") version "2.4.10"' packages/kotlin/build.gradle.kts` → 1 — build file
    untouched (confirmed by `git diff HEAD~1..HEAD --stat`)
- [x] Version-sync anchors intact — `io.iscc:iscc-lib-kotlin:0.5.0` → 1 in each of the three files;
    `net.java.dev.jna:jna:5.19.1` → 1 in each of the two fuller docs
- [x] `mise run version:check` exits 0 — 21 `OK:` lines
- [x] `uv run zensical build` exits 0 — "No issues found" (13.5s); rendered
    `site/howto/kotlin/index.html` shows a proper `<div class="admonition note">` with the floor
    text
- [x] `mise run check` — all 15 hooks Passed, nothing rewritten; working tree afterwards holds only
    runner-owned `iterations.jsonl`
- [x] `git diff --stat` confined to `README.md`, `packages/kotlin/README.md`, `docs/howto/kotlin.md`
    (+ advance agent memory) — no source, build, or manifest file touched
- [x] Gate integrity — `git diff @{upstream}..HEAD` (6 unpushed commits incl. 2 human ones) shows no
    lint suppression, test skip, threshold reduction, hook weakening, or scope exclusion

**Issues found:**

- (none) — resolved and deleted the `[human]` issue "Document the Kotlin consumer floor as 2.3+
    (DECIDED)"; also repaired its now-dangling cross-reference in the dependency-refresh issue.

**Codex review:** Clean — "The documentation consistently states the empirically verified Kotlin
2.3+ consumer requirement, matches the declared Kotlin 2.4.10 compiler version, and passes the
documentation and version checks." No actionable findings.

**Next:** **Unicode freeze rule, step (a)** — the largest remaining target gap and the only unmet
spec criterion with real machinery behind it: vendored unassigned-ranges table (731 ranges) + the
checked-in generator script (from `unicodedata2==16.0.0`) + the pre-normalization filter in
`text_clean`/`text_collapse`, evidenced by a full-code-space differential sweep. Keep step (b)
(boundary vectors across the bindings) for a separate iteration. Two guards must be planned into the
same step, not discovered by CI: the `.iai-baseline.json` Ir gate (a per-character range lookup
ahead of normalization will move the meta/text benches — prefer binary search over sorted ranges or
an ASCII short-circuit, and refresh the affected baseline entries in-step with justification) and
the CI-only CRAP `--fail-regression` gate (new branches in covered `utils.rs` functions need
`.crap-baseline.json` refreshed in the same commit). If a smaller step is wanted first, ruff 0.16
slice B (isort cluster: `I001` 8 + `RUF022` 1) is self-contained.

**Notes:**

- Verified independently that the two open release-workflow issues are *not* stale: `release.yml`
    still has the `!cancelled() && !failure()` guard only on `build-*` and `publish-crates-io` (all
    `test-*`/`publish-*` jobs lack it), and `NODE_AUTH_TOKEN: secrets.NPM_TOKEN` is still present at
    two publish sites.
- Fixed the housekeeping item state.md flagged: `version_sync.py` manages **21** targets, not 22
    (corrected in issues.md and learnings.md).
- Minor coverage gap, deliberately not filed as an issue: `docs/index.md`'s Kotlin install tab and
    the "Available Bindings" table carry no floor note. That mirrors the root README Quick Start,
    which next.md explicitly excluded, and the linked howto guide does state it — mentioning it only
    so a future docs pass can decide.
- learnings.md was at 216 lines (over the 200 budget). Pruned to exactly 200 by merging the three
    overlapping Unicode entries into one, deleting the duplicated "data tables need a differential
    sweep" bullet, and archiving the bundler/frozen-install recipe to `learnings-archive.md`.
- No `decisions.md` entry this iteration: the Kotlin floor decision was made and recorded by Titusz
    on 2026-07-25; this step only documented it.
