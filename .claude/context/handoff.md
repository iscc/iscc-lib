# Handoff

## 2026-07-25 — Land the `release.yml` static checks 1–2 as an executable gate

**Done:** Created `scripts/check_release_workflow.py` — a PyYAML-based static gate for
`.github/workflows/release.yml` covering the registry-guard shape (check 1, incl. input
declaration/reference cross-check), artifact upload/download wiring with matrix-include expansion
(check 2), and the cheap `needs:` graph check (check 3 bonus from next.md). Wired it into prek as a
pre-commit hook scoped to `^\.github/workflows/release\.yml$` and into CI via
`tests/test_check_release_workflow.py` (anchor test on the real workflow + 7 mutation tests on
`tmp_path` copies + 3 unit tests). Declared `pyyaml` explicitly in the dev group.

**Files changed:**

- `scripts/check_release_workflow.py`: new gate script — short pure functions returning `list[str]`,
    optional positional path arg defaulting to the tracked workflow, exits 1 with one error per
    line; handles the YAML-1.1 `on:`→`True` key gotcha; nothing hardcoded beyond the
    `prepare-release` constant (with the why-unguarded comment)
- `tests/test_check_release_workflow.py`: new — loads the script by path (`importlib`, same pattern
    as `test_iai_regression.py`); anchor test asserts zero errors on the tracked workflow; mutation
    tests cover guard stripped, missing `if`, wrapped `prepare-release`, dropped
    `inputs.version != ''` alternative, typo'd registry flag (both directions), renamed upload
    breaking download resolution, broken `needs:` entry; unit tests for matrix expansion, wildcard
    matching, and the `on:` bool-key handling
- `.pre-commit-config.yaml`: added `check-release-workflow` local hook (pre-commit stage,
    `pass_filenames: false`) with a comment on why the file needs a static gate
- `pyproject.toml`: `pyyaml` added to `[dependency-groups] dev` with inline comment (taplo aligned
    the comment column)
- `uv.lock`: regenerated (`uv lock`; pyyaml 6.0.3 was already resolved transitively — 2-line diff)
- `CLAUDE.md`: pre-commit stage list mentions the new hook
- `docs/development.md`: pre-commit bullet list gains the new hook entry

**Verification:** All next.md criteria pass, each backed by a command run this session:
`uv run scripts/check_release_workflow.py` → exit 0, OK line; sed mutation probe under
`/tmp/relprobe` → exit 1 with 28 `guard:` errors naming the stripped wrapper;
`git status --porcelain .github/workflows/release.yml` → empty (workflow untouched);
`uv run pytest tests/test_check_release_workflow.py` → 11 passed;
`prek run check-release-workflow --files .github/workflows/release.yml` → Passed,
`--files pyproject.toml` → Skipped; `uv run ruff check` and `uv run ty check` → "All checks
passed!"; `uv lock --check` → resolves clean; `uv run python -c "import yaml"` → ok (6.0.3);
`mise run check` → every hook Passed including the new one; full `uv run pytest --timeout=120` → 325
passed.

**Next:** The remaining halves of the ledger item: check 3 (action-input compatibility against each
ref's `action.yml`) as a CI-only network step, and the parked Unicode boundary-fixture propagation
(still blocked on Titusz's ordering ruling + Go 15.0-tables decision). The
`rubygems/configure-rubygems-credentials@main` pin also stays human-gated.

**Notes:**

- **Deliberate deviation from next.md's check-2 matching rule.** As literally written ("regex from
    upload name, download with `*` removed must fullmatch"), the rule cannot resolve the `wheels-*`
    download against the matrix-expanded literal `wheels-ubuntu-latest-x86_64` uploads; the reversed
    direction alone cannot resolve the literal `gem-x86_64-linux` download against the `gem-*`
    upload (build-gem uses a plain `matrix.platform` list — no `include` — so its upload name keeps
    a wildcard after expansion). Neither single direction satisfies next.md's own "resolves all 20
    downloads at HEAD" claim. Implemented the symmetric closure — a pairing matches if *either*
    glob, with the other side's `*` collapsed, fully matches the other — which resolves all 20
    downloads at HEAD and still fires on the renamed-upload mutation
    (`test_renamed_upload_breaks_download_resolution`). Documented in `references_match`'s
    docstring.
- `uv sync` after the lock regeneration uninstalled the editable `iscc_lib` extension as usual;
    re-ran `uv run maturin develop -m crates/iscc-py/Cargo.toml` before the full pytest run.
- `.claude/context/iterations.jsonl` shows as modified in the working tree (runner-owned); left
    unstaged per protocol.
- No Rust changes; `.crap-baseline.json` / `.iai-baseline.json` untouched. No hot paths involved.
