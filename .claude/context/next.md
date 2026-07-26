# Next Work Package

## Step: Fix the `docs/llms.txt` drift and gate the three hand-wired docs page lists

## Goal

Close the tracked `normal` issue "Gate parity of the three hand-wired docs page lists" (issues.md,
`[review]`): add the six missing page links to `docs/llms.txt` — today five of eleven supported
languages (Ruby, C#/.NET, C/C++, Swift, Kotlin) are invisible to `llms.txt` consumers, so
`specs/documentation.md` line 32 ("links to **all** documentation pages") is literally false — and
land a pure-local checker that keeps the four lists in sync from now on.

**Cadence note:** the tooling window is 141 tests / 142 tooling / 143 docs / 144 tooling = 2 of 4,
below the threshold, and this step is a user-facing data fix anyway.

## Scope

- **Create**: `scripts/check_docs_nav.py`, `tests/test_check_docs_nav.py`
- **Modify**: `.pre-commit-config.yaml`, `docs/llms.txt` (doc), `docs/development.md` (doc — add the
    new checker next to the existing `check_release_workflow.py` description)
- **Reference**: `scripts/check_release_workflow.py` and `tests/test_check_release_workflow.py` (the
    established gate-script + `importlib.util.spec_from_file_location` test pattern),
    `scripts/gen_llms_full.py` (`ORDERED_PAGES`, `EXCLUDE_DIRS`, `discover_pages`), `zensical.toml`
    (`nav`), `scripts/version_sync.py` (stdlib-only regex parsing style),
    `.claude/context/issues.md` lines 182–204 (the issue text with the measured drift)

Non-test/non-doc file budget: **2** (`scripts/check_docs_nav.py`, `.pre-commit-config.yaml`).

## Not In Scope

- Hardening `--check-action-inputs` against its four blind spots — separate tracked issue, next up.
- Anything Unicode: the boundary-vector propagation into the 12 bindings and the full-code-space
    sweep are both parked on human rulings. Do not touch `docs/unicode.md`.
- Reconciling `specs/ci-cd.md`'s 14-row CI job table, or any other edit under
    `.claude/context/specs/` or `target.md`.
- Making the checker **auto-fix** `docs/llms.txt`. It reports and exits non-zero; humans/agents edit
    the list. Auto-generating `llms.txt` is a different (larger) design decision.
- Adding, splitting or renaming any documentation page, or writing new prose beyond the six link
    lines and the `development.md` sentence.
- Wiring the checker into `.github/workflows/docs.yml` or a new CI job — the pytest test carries it
    into CI (see notes).

## Implementation Notes

**1. Data fix — `docs/llms.txt`.** Add the six missing pages under `## Reference`, keeping the
existing nav-derived order and the existing one-line `- [Title](URL): description` style with
absolute `https://lib.iscc.codes/<path>.md` URLs:

- `howto/ruby.md` after the Python how-to
- `howto/dotnet.md`, `howto/c-cpp.md`, `howto/swift.md`, `howto/kotlin.md` after the Java how-to
- `ruby-api.md` after the Java API entry

Descriptions should mirror the phrasing already used ("Guide to using iscc-lib from X", "X API
reference"). After the fix the `## Reference` block has **23** page links. The intro paragraph
mentions only some bindings — leaving it as is, is fine (not in scope).

**2. Checker — `scripts/check_docs_nav.py`.** A stdlib-only script (module docstring, short pure
functions, `main()` returning an exit code, `if __name__ == "__main__": sys.exit(main())`) that
builds four sets of page paths relative to `docs/` and asserts they are equal:

- **disk**: `docs/**/*.md` via `rglob`, skipping the `EXCLUDE_DIRS` allowlist — keep a commented
    `EXCLUDE_DIRS = {"includes"}  # snippet partials, not pages` that mirrors `gen_llms_full.py`
- **nav**: from `zensical.toml` — slice the `nav = [ ... ]` block and `re.findall` quoted values
    ending in `.md`. **Do not use `tomllib`**: CI's `python-test` matrix includes Python 3.10 where
    `tomllib` does not exist, and the pytest test below imports this module. Regex parsing matches
    `scripts/version_sync.py`'s existing style.
- **ordered**: `ORDERED_PAGES` — load `scripts/gen_llms_full.py` by path with
    `importlib.util.spec_from_file_location` (it is side-effect free on import; `main()` is guarded)
    or regex the list block. Either is acceptable; prefer the import (single source of truth).
- **llms**: from `docs/llms.txt` — `re.findall(r"https://lib\.iscc\.codes/(\S+?\.md)")`; the
    `llms-full.txt` link is naturally excluded because it does not end in `.md`.

Report every mismatch with the list name and the sorted symmetric difference (e.g.
`llms.txt: missing 6 page(s): [...]` / `unexpected 1 page(s): [...]`), print all failures before
exiting `1`, and print a single
`OK: <n> documentation pages consistent across nav, ORDERED_PAGES and llms.txt.` line on success.
Make the paths injectable (module-level `ROOT`/path constants plus functions taking a `Path`) so the
tests can point at fixtures instead of the real repo. No network, no writes.

**3. prek hook.** Add a local `pre-commit`-stage hook after `check-release-workflow` in
`.pre-commit-config.yaml`, with a short comment saying the three lists are hand-wired and nothing
else checks they agree:

```text
- id: check-docs-nav
  name: Docs page list parity
  entry: uv run scripts/check_docs_nav.py
  language: system
  files: <regex matching docs/*.md, docs/llms.txt, zensical.toml, scripts/gen_llms_full.py>
  stages: [pre-commit]
  pass_filenames: false
```

**4. Test — `tests/test_check_docs_nav.py`.** Follow `tests/test_check_release_workflow.py`: load
the script by path, then cover (a) the real repo tree passes — this is what carries the gate into
CI, since CI runs pytest but never prek; (b) each of the three lists individually failing when a
page is dropped from it, using `tmp_path` fixtures; (c) a page present in a list but absent from
disk; (d) `includes/abbreviations.md` on disk does not count as a page. Simple `def test_*`
functions, no classes, no mocks beyond fixture files.

**5. `docs/development.md`.** One short entry alongside the existing release-workflow checker
description: what `scripts/check_docs_nav.py` asserts, that it is network-free, and that adding a
docs page means updating `zensical.toml` nav, `ORDERED_PAGES` and `docs/llms.txt` together.

Run `mise run format` before staging.

## Verification

- `uv run scripts/check_docs_nav.py` exits **0** and prints no missing/unexpected page lines
- `grep -c 'https://lib\.iscc\.codes/[^ ]*\.md' docs/llms.txt` reports **23**
- Each of the six previously-missing pages is present:
    `for p in howto/ruby.md howto/dotnet.md howto/c-cpp.md howto/swift.md howto/kotlin.md ruby-api.md; do grep -q "lib.iscc.codes/$p" docs/llms.txt || exit 1; done`
    exits 0
- `uv run pytest tests/test_check_docs_nav.py` passes with at least 5 tests, including at least one
    that asserts a **non-zero** exit for a list that is missing a page
- `uv run pytest` (full suite) passes — currently 333 tests, so ≥338 after this step
- `uv run prek run check-docs-nav --all-files` reports **Passed**
- `uv run python scripts/gen_llms_full.py` exits 0 and its output contains **no** `Auto-discovered`
    line
- `uv run zensical build` exits 0 and reports "No issues found"
- `mise run check` — all hooks Passed (17 with the new one); `uv run ruff check` and
    `uv run ruff format --check` clean
- `git status --porcelain .claude/context/specs/ docs/unicode.md` is empty (parked items untouched)

## Done When

`docs/llms.txt` links all 23 real documentation pages, `scripts/check_docs_nav.py` proves the four
lists agree, that check runs both as a scoped prek hook and inside the CI-executed pytest suite, and
every verification command above passes.
