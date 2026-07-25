# Next Work Package

## Step: Close the ruff/Markdown gate-parity gap in the prek hooks

## Goal

Make the local `ruff format` surface cover Python code blocks inside Markdown, so the formatter gate
that CI runs (`uv run ruff format --check`, 153 files since ruff 0.16) can no longer reject a change
that `mise run format`, `mise run check` and `git push` all call clean. Picks up the `normal`
`[review]` issue "`ruff format` covers Markdown in CI but no local hook does".

## Scope

- **Modify**: `.pre-commit-config.yaml` (1 non-doc file — the `ruff-format` local hook)
- **Modify (docs)**: `docs/development.md` (hook list, line ~137), `CLAUDE.md` (Pre-commit Hooks
    section, line ~179)
- **Reference**: `.claude/context/issues.md` (the `[review]` issue, "Fix options"),
    `.claude/context/decisions.md` (2026-07-25 entry — why no `[tool.ruff.format]` exclude),
    `.github/workflows/ci.yml` lines 68–71 (the two CI ruff steps), `mise.toml` `[tasks.lint]` /
    `[tasks.check]` / `[tasks.format]`

## Not In Scope

- **Do not widen the `ruff-check` hook to Markdown.** Probed at HEAD: `uv run ruff check <file>.md`
    prints `warning: No Python files found under the given path(s)` and exits 0 — ruff 0.16 formats
    Markdown but does not *lint* it, so widening `ruff-check` adds per-commit warning noise and zero
    gate. CI's `uv run ruff check` is equally a no-op on `.md`, so there is no asymmetry to close
    there. Leave `types: [python]`.
- **Do not also add fix option (b)** (a bare `ruff format --check` pre-push hook with
    `pass_filenames: false`). Once the pre-commit surface matches CI, a second copy of the same gate
    only duplicates work and slows every push. Pick option (a) only.
- Do not add a `[tool.ruff.format]` / `[tool.ruff]` `exclude` for Markdown — `decisions.md`
    (2026-07-25) already rejected that as scope exclusion.
- Do not add `--force-exclude` to the `ruff-format` entry (see Implementation Notes — the resulting
    superset is deliberate).
- Do not pin `ruff` or `mdformat-ruff` inside the mdformat hook's `additional_dependencies`, and do
    not bump the mdformat `rev`.
- Do not reformat any Markdown content, retag any code fence, or "fix" prose while in the files.
- Do not touch the Unicode boundary-vector work (still parked on the HUMAN REVIEW ordering ruling)
    or `.github/workflows/release.yml`.

## Implementation Notes

**The change is one line plus a comment.** In `.pre-commit-config.yaml`, the local `ruff-format`
hook (lines 44–49) currently reads `types: [python]`. Change it to `types_or: [python, markdown]`
and add a short evergreen comment above it stating that ruff formats Python code blocks inside
Markdown, so the hook covers the same surface as the CI `ruff format --check` step. Leave `entry`,
`language` and `stages` untouched.

**Evidence gathered while scoping (do not re-derive, but do re-verify):**

- `prek 0.4.11` supports `types_or` and does pass `.md` files to the hook. Probed with an alternate
    config: a staged `probe.md` containing an unformatted `py` fence made the hook report
    `Failed — files were modified by this hook / 1 file reformatted` and left the fence formatted.
- **The gap is narrower than the issue text implies, and still real.**
    `mdformat-mkdocs[recommended]` pulls in `mdformat-ruff`, which registers an
    `mdformat.codeformatter` entry point for the fence tag `python` **only**. Ruff 0.16 additionally
    formats fences tagged `py`, `python3` and `pycon`. Probed: a file with `py`/`python3`/`pycon`
    fences containing `x=1` survives `uv run prek run mdformat` untouched, then
    `uv run ruff format --check` flags all three. The tracked corpus today has 46 `python` fences
    and zero `py`/`python3`/`pycon` fences, so nothing is red — this closes a trap, it does not fix
    a break.
- **The two formatters converge on this tree.** Probed `python` fences nested in an ordered-list
    item and inside a `!!! note` admonition: mdformat and `ruff format` agree, and mdformat → ruff →
    mdformat is a fixed point. Hook order matters and is already correct — the mdformat repo hook is
    declared *before* the local `ruff-format` hook, so the project's pinned ruff (0.16.0, the same
    binary CI runs) has the last word over the unpinned ruff that prek resolves inside mdformat's
    isolated env. Verification below re-runs `mise run check` twice to prove there is no ping-pong.

**Surface arithmetic (state this in the docs edit):**

- CI's bare `uv run ruff format --check` discovers **153** files: 129 tracked `.md` + 24 tracked
    `.py` + 1 tracked `.pyi` = 154, minus one tracked-but-gitignored file under `.claude/plans/`
    that recursive discovery skips.
- The widened prek hook at `--all-files` passes all **154** tracked python+markdown files, because
    explicitly-named paths bypass ruff's exclusions without `--force-exclude`. Local is therefore a
    strict *superset* of CI — the safe direction, and the reason not to add `--force-exclude`. Both
    counts are green at HEAD.

**Docs edits** — keep them to one sentence each, evergreen wording (no "new"/"now"):

- `docs/development.md` line ~137: the `ruff check --fix` + `ruff format` bullet should say that
    `ruff format` also covers Python code blocks inside Markdown, matching the `ruff format --check`
    step in CI.
- `CLAUDE.md` line ~179: same qualification in the pre-commit-stage hook list.

## Verification

- `grep -A7 'id: ruff-format' .pre-commit-config.yaml | grep -q 'types_or: \[python, markdown\]'`
    exits 0
- `grep -A6 'id: ruff-check' .pre-commit-config.yaml | grep -q 'types: \[python\]'` exits 0 (the
    lint hook is deliberately unchanged)
- `uv run prek run check-yaml --files .pre-commit-config.yaml` passes, and
    `uv run prek run yamlfix --files .pre-commit-config.yaml` leaves the file unchanged
- Probe that the hook catches a Markdown fence (create, assert, delete — the tree must be clean
    afterwards):

````text
printf '# probe\n\n```py\nx=1\n```\n' > probe_gate.md
uv run prek run ruff-format --files probe_gate.md   # expect: Failed / files were modified
grep -q 'x = 1' probe_gate.md && echo FIXED         # expect: FIXED
rm probe_gate.md
````

- `uv run ruff format --check` exits 0 and reports `153 files already formatted` (the exact CI
    command and surface)
- `uv run ruff format --check $(git ls-files '*.md' '*.py' '*.pyi')` exits 0 and reports
    `154 files already formatted` (the widened hook's surface — a superset of CI's 153)
- `uv run ruff check` → `All checks passed!` (exit 0)
- `mise run check` runs all 15 pre-commit-stage hooks Passed, **twice in a row**, and afterwards
    `git status --porcelain` shows nothing beyond the runner-owned
    `.claude/context/iterations.jsonl` (proves mdformat and `ruff format` do not fight over the 129
    tracked `.md` files)
- `uv run zensical build` exits 0 with "No issues found" (docs page changed)
- `grep -n 'ruff format' docs/development.md CLAUDE.md` shows both files mentioning Markdown code
    blocks in the ruff bullet
- `git status --porcelain -- crates/ packages/ .github/ mise.toml pyproject.toml uv.lock` is empty
    (no source, CI or dependency file touched)

## Done When

`.pre-commit-config.yaml`'s `ruff-format` hook covers Markdown, a `py`-tagged unformatted fence is
caught and fixed locally, `mise run check` is green twice over with a clean tree, and
`docs/development.md` + `CLAUDE.md` state that the local formatter surface matches CI's.
