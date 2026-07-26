# Handoff

## 2026-07-26 — Fix the `docs/llms.txt` drift and gate the three hand-wired docs page lists

**Done:** Added the six missing page links to `docs/llms.txt` (Ruby/C#/.NET/C-C++/Swift/Kotlin
how-tos + Ruby API — the five previously invisible languages are now linked, 23 links total) and
landed `scripts/check_docs_nav.py`, a stdlib-only parity gate asserting that the pages on disk
(`docs/**/*.md` minus `includes/`), the `zensical.toml` nav, `ORDERED_PAGES` in
`scripts/gen_llms_full.py`, and the `docs/llms.txt` links are the same set. Wired as a scoped prek
pre-commit hook (`check-docs-nav`) and into CI via 8 pytest tests.

**Files changed:**

- `docs/llms.txt`: six new `- [Title](URL): description` lines in nav order (howto/ruby after
    Python; howto/dotnet, c-cpp, swift, kotlin after Java; ruby-api after Java API)
- `scripts/check_docs_nav.py`: new checker — `disk_pages`/`nav_pages`/`ordered_pages`/`llms_pages`
    build the four sets, `run_checks(docs_dir, toml_path, script_path, llms_path)` returns error
    strings (path-injectable for fixtures), `main()` prints all mismatches then exits 1, or prints
    the single OK line. Nav parsed by regex (no `tomllib` — CI matrix includes Python 3.10);
    `ORDERED_PAGES` loaded via `importlib.util.spec_from_file_location` (single source of truth)
- `.pre-commit-config.yaml`: `check-docs-nav` hook after `check-release-workflow`, `files:` scoped
    to `docs/*.md`, `docs/llms.txt`, `zensical.toml`, `scripts/gen_llms_full.py`,
    `pass_filenames: false`, with a comment explaining the hand-wired lists
- `tests/test_check_docs_nav.py`: 8 tests — real-repo anchor (carries the gate into CI), consistent
    fixture tree, each of the three lists individually missing a page, `main()` returning 1 on a
    missing page (monkeypatched constants + captured output), a ghost page absent from disk, and
    `includes/abbreviations.md` rejected as a page
- `docs/development.md`: one bullet in the pre-commit hook list describing what the checker asserts,
    that it is network-free, and that adding a docs page means updating all three lists together

**Verification:** All next.md criteria pass with evidence from this session:

- `uv run scripts/check_docs_nav.py` → exit 0, prints
    `OK: 23 documentation pages consistent across nav, ORDERED_PAGES and llms.txt.`
- `grep -c 'https://lib\.iscc\.codes/[^ ]*\.md' docs/llms.txt` → **23**; the six-page grep loop
    exits 0 ("all six present")
- `uv run pytest tests/test_check_docs_nav.py` → 8 passed (≥5 required, incl. the non-zero-exit
    test); full `uv run pytest` → **341 passed** (was 333, +8)
- `uv run prek run check-docs-nav --all-files` → Passed
- `uv run python scripts/gen_llms_full.py` → exit 0, no `Auto-discovered` line (23 pages)
- `uv run zensical build` → exit 0, "No issues found"
- `mise run check` → all **17** hooks Passed; `uv run ruff check` → "All checks passed!";
    `uv run ruff format --check` → exit 0 (163 files already formatted)
- `git status --porcelain .claude/context/specs/ docs/unicode.md` → empty (parked items untouched)

**Next:** The other unblocked `normal` issue: harden `--check-action-inputs` against its four filed
blind spots (`IncompleteRead`/`YAMLError` escaping the fails-open contract, one-directional check,
Docker-action `args`/`entrypoint` false positive, job-level `uses:` unscanned). Cadence note: this
would make the window 142/143/144/145 = 3 of 4 tooling — define-next may prefer something
user-facing first.

**Notes:**

- Non-test/non-doc budget respected: 2 files (`scripts/check_docs_nav.py`,
    `.pre-commit-config.yaml`). `docs/llms.txt` and `docs/development.md` are the in-scope doc
    edits.
- Design choice: `run_checks` uses the **disk set as the reference** — a page missing from a list
    reports as `missing`, a listed page absent from disk as `unexpected`. Symmetric difference is
    fully covered; each list is diffed and reported independently before the non-zero exit.
- The llms.txt regex `https://lib\.iscc\.codes/(\S+?\.md)` naturally excludes the `llms-full.txt`
    link (no `.md` suffix), as next.md anticipated.
- First `mise run format` attempt hit my 2-minute Bash timeout (the full pre-commit sweep takes
    longer than that here); rerun with a larger timeout exited 0 with no file modifications. Ruff
    then wrapped one long assert line in the new test file — included in the staged version.
- `mise run check` was run **after** staging the new files (prek `--all-files` only sees
    git-tracked/indexed files), so the new hook and both new files were inside the sweep.
- `.claude/context/iterations.jsonl` is modified in the working tree; left unstaged (runner-owned).
- The intro paragraph of `docs/llms.txt` still names only six binding languages — explicitly left
    as-is per next.md ("not in scope").
