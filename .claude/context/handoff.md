# Handoff

## 2026-07-25 — Review of: Land the `release.yml` static checks 1–2 as an executable gate

**Verdict:** PASS

**Summary:** `scripts/check_release_workflow.py` turns three hand-retyped `release.yml` invariants
(registry-guard shape, artifact wiring, `needs:` graph) into a real gate, wired into prek (scoped to
that one file) and into CI via an 11-test pytest suite. The script is short, pure and structural —
nothing hardcoded but the `prepare-release` job id, with the "why" comment next to it — and I
verified it actually bites by writing my own mutations beyond the committed tests. Scope discipline
is exact: 3 non-test, non-doc files, `release.yml` itself untouched, check 3 correctly left out.

**Verification:**

- [x] `uv run scripts/check_release_workflow.py` exits 0 — prints the `OK:` line on the tracked
    workflow
- [x] `git status --porcelain .github/workflows/release.yml` prints nothing — workflow unmodified
- [x] Mutation probe (guard wrapper stripped, `/tmp/relprobe`) exits **1** with 28 `guard:` errors
    naming each affected job
- [x] `uv run pytest tests/test_check_release_workflow.py` → **11 passed**, incl.
    `test_real_workflow_passes` running every check against the tracked `release.yml`
- [x] `uv run prek run check-release-workflow --files .github/workflows/release.yml` → `Passed`
- [x] `uv run prek run check-release-workflow --files pyproject.toml` → `Skipped` (scoping holds)
- [x] `uv run ruff check` → "All checks passed!"; `uv run ty check` → "All checks passed!" (script
    not excluded from `ty`)
- [x] `uv lock --check` → resolves clean; `import yaml` works (pyyaml 6.0.3, dev group)
- [x] `mise run check` → exit 0, every hook `Passed` including the new one; only `iterations.jsonl`
    dirty afterwards (runner-owned, left unstaged)
- [x] `uv run pytest --timeout=120` → **325 passed** (314 + 11 new), no regressions
- [x] Extra (review-added) gates: pre-push `ruff --select S` and `--select C901 --force-exclude`
    both clean on the new files
- [x] Gate-integrity scan of `@{upstream}..HEAD`: no suppressions, no skips, no threshold or hook
    weakening — the diff *adds* a hook

**Independent mutation probes (beyond the committed test suite):** the tests rename an *upload*
`name:`; I also renamed a *download* (`kotlin-jar` → `kotlin-jarr` → fired) and appended a job with
no `if` and a bogus `needs:` (fired on both check 1a and check 3). The gate is not vacuous. I also
dumped the checker's internals via `importlib`: 34 expanded upload names, 21 expanded download
references, and **no** upload collapsing to a bare `*` (which would have matched everything).

**Issues found:**

- (none blocking) The `references_match` docstring overclaimed — it said the matcher "rejects a
    pairing only when no overlap is possible", which is false: it is a strict *approximation* of
    glob intersection. **Fixed in this review** (docstring only, no behavior change) and recorded in
    `decisions.md`.

**Deviation from next.md, accepted:** advance replaced next.md's one-directional artifact-matching
rule with a symmetric one. I re-proved the prescribed rule genuinely fails at HEAD — neither
direction alone resolves all 21 expanded download references (`wheels-*` needs download→upload;
`gem-x86_64-linux` vs the wildcard `gem-*` upload needs upload→download), so next.md's own "resolves
all 20 downloads at HEAD" claim was untested. Documented in `decisions.md` 2026-07-25.

**Codex review:** one P2 finding, and it is correct: `references_match` is not a complete glob
intersection — `gem-*` (upload) and a hypothetical `*-linux` (download) overlap at
`gem-x86_64-linux` but both directions return false. Not a blocker, because the matcher errs
**strict**: it can raise a false alarm on a valid future edit but can never let broken wiring
through, and no such pairing exists at HEAD (verified from the dumped upload/download sets). Codex
recommends real intersection logic; I judged that over-engineering for a 34-upload file whose only
wildcards come from matrix spans, and instead had the limitation stated in the docstring plus
`decisions.md`, with the explicit instruction that the fix is a real intersection, never a looser
match.

**Next:** The `[review]` issue was rewritten, not deleted — checks 1–2 are done, **check 3
(action-input compatibility) remains** and is now the natural follow-up: extend the script with an
opt-in `--check-action-inputs` mode that fetches each `uses:` ref's published `action.yml` and
verifies every `with:` key and every `steps.<id>.outputs.<x>` the workflow reads, plus a `ci.yml`
step to run it. Keep it out of the pytest suite (that must stay network-free) and make it skip
cleanly when offline/unauthenticated. This is the one check review still performs by hand on every
action bump (iters 127, 140). If define-next prefers a non-workflow item, the other unblocked
options are unchanged — everything Unicode-related is still parked on Titusz.

**Notes:**

- **Still parked on Titusz (no CID progress possible):** the freeze-rule sequence-ordering ruling
    and the Go 15.0-tables decision, which jointly block propagating
    `crates/iscc-lib/tests/unicode_boundary.json` into the 11 bindings; the
    `rubygems/configure-rubygems-credentials@main` tag-vs-SHA pin; npm OIDC trusted publishing. Not
    IDLE — `normal` work (check 3) is fully unblocked.
- `pyyaml` is now an explicit dev dep. This is the right call and not a PEP 723 candidate: the
    pytest suite imports the script in-process, so an inline-script env would not be on the path.
- The gate's blind spots, for whoever touches `release.yml` next: it does not look at step-level
    `if:` conditions, does not flag an upload nobody downloads (deliberate), and splits guard
    alternatives on a naive top-level `||` (fine for the current flat conditions).
- `.claude/context/learnings.md` was over its 200-line budget on arrival; the closed v0.6.0
    dependency-refresh and ruff-0.16 threads were archived to `learnings-archive.md` to bring it
    back under.
