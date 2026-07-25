---
name: lint-tooling
description: Ruff/prek/mdformat configuration, the local-vs-CI gate-parity trap, and the exact commands that measure each lint surface
metadata:
  type: project
---

# Lint & Formatting Tooling (ruff, prek, mdformat)

Detail split out of `MEMORY.md`. Everything here is verifiable at HEAD — re-check before acting on
it.

## Ruff configuration (`pyproject.toml`)

**Verify via `tomllib`, not `grep`** — long inline code spans get rewrapped by mdformat in prose
copies of the config, so grep patterns rot.

- `[tool.ruff]` sets `src = [".", "crates/iscc-py/python"]`. Load-bearing: without it `iscc_lib`
    sorts as third-party and `I001` fires across the test suite.
- `[tool.ruff.lint.isort]` sets `combine-as-imports = true`. Load-bearing: without it the
    `_lowlevel` re-export block in `__init__.py` shatters into ~60 statements instead of one.
- `extend-select = ["S", "C901", "I", "RUF022", "RUF100"]` and there is still **no `select` key**.
    That absence is load-bearing — a bare `select` would drop ruff's `E4`/`E7`/`E9`/`F` defaults.
- Project ruff is **0.16.0 since iter 137** (the `ruff<0.16` pin and its `# held:` comment are
    gone). Ground truth: `uv run ruff --version`. `uv run ruff check` exits 0 at HEAD.
- **Never run `ruff@0.16 check --fix .`** — it deletes the 14 load-bearing `# noqa: S603/S607`
    directives in `tools/` and `scripts/`. Every landed slice used an explicit `--select`.

## The prek type-tag / gate-parity trap

prek's file-type tags do **not** match ruff's own recursive discovery. This has now bitten twice.

| Surface              | Command                              | Files seen              |
| -------------------- | ------------------------------------ | ----------------------- |
| CI (`ci.yml` L71)    | `uv run ruff format --check`         | 153                     |
| prek hook, all files | `prek run ruff-format --all-files`   | 153 (a *different* 153) |
| Tracked candidates   | `git ls-files '*.md' '*.py' '*.pyi'` | 154                     |

154 = 129 `.md` + 24 `.py` + 1 `.pyi`. CI skips the tracked-but-gitignored `.claude/plans/*.md`; the
hook includes it but excludes the `.pyi`. So local is **neither a subset nor a superset** of CI —
the symmetric difference is exactly those two files.

- **Markdown half CLOSED (iter 138):** `.pre-commit-config.yaml` L50 — `ruff-format` is
    `types_or: [python, markdown]`. `ruff-check` L42 deliberately stays `types: [python]`: ruff 0.16
    formats Python fences inside `.md` but never lints them.
- **`.pyi` half STILL OPEN:** prek types `.pyi` as `pyi`, not `python`, so both hooks skip
    `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (the workspace's only tracked `.pyi`, and
    consumer-facing — the wheel ships `py.typed`). CI catches it; local hooks do not. Fix is two
    tokens: `types_or: [python, pyi]` and `types_or: [python, pyi, markdown]`.
- **Measure a hook's real surface by probing it**, never by `git ls-files` arithmetic:
    `git add probe.ext && uv run prek run <hook> --files probe.ext`. The iter-138 handoff's "strict
    superset (154 vs 153)" claim came from the arithmetic route and was wrong.
- prek reports `files were modified by this hook` only for **tracked** files. An untracked probe is
    silently fixed and reported `Passed`. Unstage with `git rm --cached -f` (plain `--cached`
    refuses once the hook has rewritten the worktree copy).
- `ruff format` does not error on a syntactically-invalid Python fence in Markdown — it leaves it
    alone and exits 0. Widening the hook cannot break commits over illustrative pseudo-code.
- `decisions.md` (2026-07-25) rejected both escape hatches: no `[tool.ruff.format]` exclude (scope
    exclusion) and no duplicate pre-push `ruff format --check` (widen the existing hook instead).

## mdformat

Hook args are `--wrap 100 --number`. Always:

```
cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff /tmp/c.md f
```

then `cp /tmp/c.md f` to adopt, and re-run once to confirm idempotence. A bare `mdformat` (no
`--number`) renumbers ordered lists to all-`1.`.

It **aborts the commit** on:

- nested or escaped backticks inside a code span,
- a re-wrapped line that starts with `+`, `-` or `>`,
- a bare `<number>.` landing at the start of a re-wrapped line (escaped to `26\.`) — reword, e.g.
    "26 findings" instead of a leading "26.".
