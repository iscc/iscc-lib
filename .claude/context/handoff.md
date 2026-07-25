# Handoff

## 2026-07-25 — Close two silent no-op gates — release.yml re-trigger guards and `.pyi` hook coverage

**Done:** Wrapped the 19 unguarded `test-*`/`assemble-*`/`pack-*`/`publish-*` job `if:` conditions
in `release.yml` with `${{ !cancelled() && !failure() && (…) }}`, matching the shape already used at
`publish-crates-io`, so `-f <registry>=true` re-triggers no longer inherit `prepare-release`'s skip.
Added the `pyi` type tag to both local ruff hooks (`ruff-check` → `types_or: [python, pyi]`,
`ruff-format` → `types_or: [python, pyi, markdown]`) so the published `_lowlevel.pyi` stub is gated
locally, not just in CI. Updated the release skill and the two hook-list docs to match.

**Files changed:**

- `.github/workflows/release.yml`: exactly 19 `if:` lines wrapped (4 npm, 3 nuget, 3 maven, 3
    maven-kotlin, 2 pypi, 2 ffi, 2 rubygems); `prepare-release` and all `needs:` lists untouched; no
    `uses:` ref bumped
- `.pre-commit-config.yaml`: the two `types`/`types_or` tokens only; evergreen comment above
    `ruff-format` kept unchanged (still accurate — with `pyi` the hook is now a strict superset of
    CI's surface)
- `.claude/skills/release/SKILL.md` (docs): "Known bug" subsection replaced with "Re-triggering a
    registry with `-f <registry>=true`" describing the guarded behaviour (honest that it's
    statically verified, first real confirmation on the next release); the Phase-1 "Do NOT use … -f
    npm=true" blockquote rewritten to match (it repeated the retired bug and would have contradicted
    the new section); `gh run rerun --failed` retained as preferred path and the "Never pass
    `-f version=`" warning kept verbatim
- `docs/development.md` (docs): ruff bullet now enumerates `.pyi` type stubs (one clause)
- `CLAUDE.md` (docs): pre-commit-stage sentence now enumerates `.pyi` type stubs (one clause)

**Verification:** All criteria reproduced in this session:

- next.md's guard-shape script → `release.yml guards OK` (29 jobs, `prepare-release` untouched,
    token counts exactly
    `{version: 29, crates-io: 1, pypi: 4, npm: 6, maven: 4, ffi: 3, nuget: 4,   rubygems: 3, maven-kotlin: 4}`)
- `actionlint@v1.7.7` on release.yml → exit 0, no output
- `prek run check-yaml` / `yamlfix --files release.yml` → both Passed, no modifications
- Both `types_or` greps → exit 0; `grep -c 'id:'` → 22 (no hook added/removed)
- `prek run ruff-check` / `ruff-format --files crates/iscc-py/python/iscc_lib/_lowlevel.pyi` → both
    `Passed`, neither printed `no files to check`
- `uv run ruff check` → `All checks passed!`, exit 0; `uv run ruff format --check` → exit 0 but
    reports **155** files, not next.md's 153 — the delta is the two tracked `.md` agent-memory files
    added by this iteration's own update-state (`lint-tooling.md`) and define-next
    (`release-yml-static-gates.md`) commits, confirmed via `git diff HEAD~3..HEAD --diff-filter=A`.
    Exit 0 is the gate; the count was a stale prediction.
- `grep -c 'exclude' .pre-commit-config.yaml` → **2, not next.md's 0** — mis-specified criterion:
    both matches are the pre-existing `--force-exclude` CLI flags in the pre-push S/C901 hooks
    (present at HEAD too, verified via `git show HEAD:…`). The intent holds:
    `grep -cE '^\s*exclude:'` → 0 (no prek scope key added), diff of the file is exactly the two
    type-tag lines.
- `grep -q 'Known bug' SKILL.md` → exit 1; `grep -q 'gh run rerun' SKILL.md` → exit 0
- `grep -q 'pyi' docs/development.md && grep -q 'pyi' CLAUDE.md` → exit 0
- `uv run zensical build` → exit 0, `No issues found`
- `mise run format` then `mise run check` → 15/15 hooks Passed, exit 0; porcelain afterwards shows
    only the 5 scoped files + runner-owned `iterations.jsonl`

**Next:** The 97-ref GHA `uses:` refresh in `release.yml` as its own bisectable step (checkout
v4→v7, `upload-artifact`/`download-artifact` moved as a pair, setup-java/node/dotnet/python;
`setup-uv` must be the exact `@v9.0.0`), verified statically the same way (yaml parse + actionlint +
artifact-name consistency). Still parked: Unicode boundary vectors (open HUMAN REVIEW
freeze-rule-ordering ruling) and the held majors (jni 0.22, magnus 0.8, xunit 3.x, Gradle wrapper,
JUnit 6.x).

**Notes:**

- Guard semantics double-checked: `!failure()`/`!cancelled()` evaluate the job's own `needs` chain,
    so a publish job still won't run after a genuinely failed build/test — the guard only
    neutralises the *skipped* status propagated from `prepare-release` on registry-only dispatches.
    `needs:` lists untouched.
- Two doc edits go one sentence beyond the letter of next.md's scope, both inside scoped files: the
    SKILL.md Phase-1 blockquote (would have contradicted the rewritten re-trigger section) and a
    lead-in clause in the new subsection noting `gh run rerun --failed` cannot re-run `skipped` jobs
    (restates a fact already in the section above it).
- For the issue ledger (review agent): both cited issues are fully resolved by this step — "Fix
    broken single-registry re-trigger in release.yml" (`[human]`) and "Local ruff hooks skip `.pyi`
    files" (`[review]`). The re-trigger issue's closing note asks to also update the
    `release-workflow.md` user-level memory (outside the repo, explicitly out of scope per next.md)
    — that pointer should survive somewhere until a human/interactive session updates it, since that
    memory still calls the `-f <registry>=true` path BROKEN.
- The guard fix is verified statically only (yaml shape + actionlint); no workflow run exercises
    release.yml on develop pushes. First real-world confirmation comes with the next release or
    registry re-trigger.
- No Rust/Python/Go/JVM/.NET/Ruby/WASM source touched — no CRAP baseline, iai perf, cargo-deny or
    semver refresh owed; no benchmarked hot path; no API surface change.
