# Handoff

## 2026-07-26 — Review of: Fix the `docs/llms.txt` drift and gate the three hand-wired docs page lists

**Verdict:** PASS_WITH_NOTES

**Summary:** The six missing pages are in `docs/llms.txt` (23 links, matching the 23 real pages),
and `scripts/check_docs_nav.py` genuinely gates the four lists — every mutation I wrote fires,
including the *real* pre-fix regression, and all three malformed-input paths fail closed. Scope was
tight (2 non-test/non-doc files, budget 2) and no quality gate was weakened. One latent blind spot
found independently by Codex and confirmed here (a commented-out `zensical.toml` nav entry still
counts as present) — filed as a follow-up issue, not a blocker: `zensical.toml` has zero comment
lines at HEAD.

**Verification:**

- [x] `uv run scripts/check_docs_nav.py` exits 0 — prints
    `OK: 23 documentation pages consistent across nav, ORDERED_PAGES and llms.txt.`, no
    missing/unexpected lines
- [x] `grep -c 'https://lib\.iscc\.codes/[^ ]*\.md' docs/llms.txt` → **23**
- [x] All six previously-missing pages present — the loop over `howto/ruby.md`, `howto/dotnet.md`,
    `howto/c-cpp.md`, `howto/swift.md`, `howto/kotlin.md`, `ruby-api.md` exits 0
- [x] `uv run pytest tests/test_check_docs_nav.py` → **8 passed** (≥5 required), incl.
    `test_main_exits_nonzero_on_missing_page` asserting `main() == 1`
- [x] `uv run pytest` (full) → **341 passed** (was 333, ≥338 required)
- [x] `uv run prek run check-docs-nav --all-files` → Passed
- [x] `uv run python scripts/gen_llms_full.py` → exit 0, `23 pages`, **no** `Auto-discovered` line
- [x] `uv run zensical build` → exit 0, "No issues found"
- [x] `mise run check` → 17/17 hooks Passed; `uv run ruff check` "All checks passed!";
    `uv run ruff format --check` exit 0
- [x] `git status --porcelain .claude/context/specs/ docs/unicode.md` empty — parked items untouched

Independent probes beyond next.md (a green gate is not a working gate):

- [x] **Real regression fires**: restoring `HEAD~1`'s `docs/llms.txt` into a temp checkout →
    `llms.txt: missing 6 page(s): [...]` — exactly the drift the issue measured
- [x] **Each list mutated separately** on a copy of the real tree: nav minus `unicode.md`,
    `ORDERED_PAGES` minus `howto/kotlin.md`, a brand-new page on disk → each reported against the
    right list only, and the new page reported against all three
- [x] **Bidirectional**: a list entry with no file on disk reports `unexpected` (fixture + ghost
    tests)
- [x] **Fail-closed, not fail-open**: missing `nav = [ ... ]` block → exit 1 with a message;
    unimportable `gen_llms_full.py` → traceback, non-zero; missing `llms.txt` → `FileNotFoundError`,
    non-zero
- [x] **Hook really fires and is really scoped**: staging a broken `docs/llms.txt` → `Failed` with
    the mismatch; staging only `README.md` → `(no files to check)Skipped`
- [x] **Site output resolves**: after the correct `zensical build` → `gen_llms_full.py` order, all
    six new URLs have non-empty `site/**/*.md` targets (23 files)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean; `mise run version:check` 21
    `OK:` lines
- [x] Gate integrity across all unpushed commits (`@{upstream}..HEAD`): the only gate-config change
    is the **addition** of `check-docs-nav`; zero suppressions, skips or threshold changes added

**Issues found:**

- **Commented-out nav entries defeat the gate** (confirmed, latent). `NAV_MD_RE` matches inside TOML
    comments, so commenting out `{ "Kotlin" = "howto/kotlin.md" },` leaves `run_checks(...) == []`
    while Zensical genuinely drops the page. No comment lines exist in `zensical.toml` today. Filed
    as a `normal` `[review]` issue with the fix options and the reason `tomllib` is unavailable (CI
    matrix pins Python 3.10).
- **The prek hook does not fire on a page *deletion*** (confirmed). `files:` matching only sees
    added/copied/modified paths, so `git rm docs/foo.md` alone → `(no files to check)Skipped`. Not a
    real hole — the pytest anchor test catches it at pre-push and in CI — but worth a comment in
    `.pre-commit-config.yaml`. Filed with the item above.
- **Minor (fixed in this review):** `test_real_repo_passes` asserted only that the four sets agree,
    which equal *empty* sets also satisfy. Added `assert len(cdn.disk_pages(cdn.DOCS_DIR)) >= 20` so
    a future `DOCS_DIR` refactor cannot make the gate vacuous. Still 8 tests, still green.
- **Nit, not filed:** `EXCLUDE_DIRS = {"includes"}` is duplicated in `check_docs_nav.py` even though
    the module already `exec_module`s `gen_llms_full.py` and could read its constant. A divergence
    would fail closed (noisy, visible), so it is DRY-only.

**Codex review:** One P2 finding — "Exclude TOML comments from nav pages"
(`scripts/check_docs_nav.py:63`). Independently reproduced and accepted; see the first issue above.
Correct call: it is the one input of the four whose parsing is not comment-aware, and it goes to the
gate's stated purpose. Codex raised nothing else and flagged no false positives.

**Next:** Two candidates, both `normal`:

1. **Harden the two gate scripts' blind spots in one step** —
    `scripts/check_release_workflow.py --check-action-inputs` (four filed gaps: `IncompleteRead`/
    `YAMLError` escaping the fails-open contract, the one-directional required-input check, the
    Docker-action `args`/`entrypoint` false positive, job-level `uses:` unscanned) plus the two new
    `check_docs_nav.py` items above. Both issues live in `scripts/`, share a test pattern, and the
    docs-nav half is a handful of lines.
2. **Something user-facing instead.** Cadence check: 141 tests / 142 tooling / 143 docs / 144
    tooling / 145 docs+tooling. Option 1 would make it 4 tooling-ish steps in a 5-window, which is
    heavy for a library whose remaining unblocked user-facing work exists. define-next should weigh
    this deliberately rather than defaulting to the cheapest issue.

Still parked on Titusz and NOT available to CID: the Unicode sequence-adjacency ruling (blocks
boundary-vector propagation into the 11 bindings, the full sweep, and the `docs/unicode.md`
placeholder sentence), the `rubygems/configure-rubygems-credentials@main` pin (tag vs SHA
convention), and npm OIDC (needs npmjs.com-side trusted publishers).

**Notes:**

- The advance handoff's claims all checked out on re-measurement — 23 links, 23 pages, 341 tests, 17
    hooks, 2 non-test/non-doc files. No inflated or unverifiable claims this iteration.
- `run_checks` taking four `Path` arguments is what made independent verification cheap: I could
    point it at `git archive HEAD | tar -x` copies and mutate them freely. Keep that shape for
    future gate scripts.
- **Ordering gotcha for anyone verifying docs output:** `zensical build` wipes `site/`. Running
    `gen_llms_full.py` *before* it makes every per-page `site/**/*.md` look missing. `docs.yml` has
    the right order (build, then generate); my first check did not.
- `.claude/context/iterations.jsonl` is modified in the working tree and deliberately left unstaged
    (runner-owned).
- `zensical.toml` currently contains zero comment lines, which is why the Codex finding is latent
    rather than active — but it also means nothing would warn a future editor who reaches for `#`.
