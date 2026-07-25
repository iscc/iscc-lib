# Handoff

## 2026-07-25 — Review of: ruff 0.16 adoption — slice A: clear the config-free findings (stub `...` + unused unpacked vars)

**Verdict:** PASS

**Summary:** The advance agent deleted exactly the 36 lone `...` placeholder lines from
`crates/iscc-py/python/iscc_lib/_lowlevel.pyi` and `_`-prefixed 6 unused unpacked bindings in
`tests/test_new_symbols.py`, taking the whole-tree ruff-0.16 finding count from 104 to 26 without
touching a single suppression, config, lock or gate. The diff is precisely the scoped set
(`git diff` on the stub contains **nothing but** `-        ...` / `-    ...` lines, 9 + 27), every
Not-In-Scope path (`tools/`, `scripts/`, `pyproject.toml`, `uv.lock`, `__init__.py`,
`.pre-commit-config.yaml`, `.github/`) is byte-untouched, and all 14 next.md criteria reproduced
exactly as claimed.

**Verification:**

- [x] `uvx ruff@0.16.0 check crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — "All checks passed!",
    exit 0
- [x] `uvx ruff@0.16.0 check --select PIE790,PYI048,RUF059 .` — "All checks passed!", exit 0
- [x] `uvx ruff@0.16.0 check . --statistics` — exactly **26** errors: `RUF100` 15, `I001` 8,
    `EXE001` 1, `PLW1510` 1, `RUF022` 1. No `PIE790`/`PYI048`/`RUF059` left (exit 1 is expected and
    deliberate)
- [x] `grep -c '^[[:space:]]*\.\.\.[[:space:]]*$' …/_lowlevel.pyi` → `0`
- [x] Stub integrity — `^def ` = 27, `^class ` = 3, `^    def ` = 9, file 440 lines. Went further:
    an AST pass confirms **every** one of the 27 functions and 9 methods now has a docstring-only
    body (no accidental empty body, no lost signature)
- [x] `uv run ruff check .` exit 0; `uv run ruff format --check .` exit 0 ("24 files already
    formatted" — docstring-only bodies produce no blank-line churn)
- [x] `uv run ruff check --select S --force-exclude` exit 0 (pre-push security gate intact)
- [x] `uv run ruff check --select C901 --force-exclude` exit 0 (pre-push complexity gate intact)
- [x] noqa counts unchanged — `tools/cid.py` 11, `tools/metrics.py` 4, `scripts/test_install.py` 1;
    `git diff --stat -- tools/ scripts/` is empty
- [x] `uv run ty check` — "All checks passed!"
- [x] `uv run pytest -q` — **314 passed** in 29.8s
- [x] `grep -c 'ruff<0.16' pyproject.toml` → `1`; `grep -c 'held: 0.16 expands'` → `1` (hold-back
    and its inline reason still in place); `git diff --stat -- pyproject.toml uv.lock` empty
- [x] `mise run check` — all 15 hooks **Passed**, nothing rewritten (working tree afterwards holds
    only the runner-owned `iterations.jsonl`)
- [x] `cargo test -p iscc-lib` — 270 + 28 + 22 unit/integration + 1 doctest, 0 failed

Additional checks beyond next.md:

- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean (only the known dev-only
    `proc-macro-error2` future-incompat note from `iai-callgrind-macros`)
- [x] **Downstream stub safety** — the wheel ships `py.typed` next to `_lowlevel.pyi`, so the stub
    is consumer-facing and the repo only gates it with `ty`. Verified the docstring-only form
    independently against `mypy 1.18.2 --strict` ("Success: no issues found") and `pyright 1.1.407`
    ("0 errors, 0 warnings")
- [x] Gate-circumvention sweep over all 6 unpushed commits (`git diff @{upstream}..HEAD`) — no
    suppression, skip, threshold change, hook edit or scope exclusion; every `noqa`/`allow` hit is
    prose inside context files

**Issues found:** (none)

**Codex review:** Clean — no findings. Verbatim: "The stub bodies remain valid for Python type
checkers, and prefixing unused test variables does not alter behavior. Ruff, ty, mypy, pyright, and
the affected tests all pass." (Independently corroborated here by running mypy/pyright directly.)

**Next:** Continue the ruff 0.16 adoption. Two remaining decision clusters — recommend taking them
in this order:

1. **Slice B — the isort cluster** (`I001` 8 + `RUF022` 1, in the Python package `__init__.py`,
    `tests/`, and `benchmarks/python/bench_iscc_lib.py`). Smallest and fully independent of gate
    semantics: decide the `[tool.ruff.lint.isort]` src-root / `known-first-party` setting first,
    then let `--fix` sort. Verify that `ruff format` and the pinned 0.15 lint stay green under the
    new isort config.
2. **Slice C — the `# noqa` cluster, and a real gate strengthening.** `[tool.ruff.lint]` in
    `pyproject.toml` currently declares **no `select` key at all** — `S` and `C901` are applied
    only by the two pre-push hooks' explicit `--select` args, so they never run in the pre-commit
    Ruff-linting hook or in `mise run check`. Adding `S` and `C901` to `[tool.ruff.lint] select`
    both (a) promotes the security/complexity scans into the fast local loop, and (b) clears all 15
    `RUF100` findings at once, because the `# noqa: S603/S607` directives become "used" again. That
    is the fix — never delete the directives. Sweep for newly-surfaced `S`/`C901` findings across
    `tools/`, `scripts/`, `benchmarks/` before committing, and keep the existing `per-file-ignores`
    for `tests/**`. `EXE001` (shebang on a non-executable file) and `PLW1510` (`subprocess.run`
    without `check=`) are two one-line fixes that can ride along.
3. **Slice D — retire the hold-back**: only once `uvx ruff@0.16.0 check .` exits 0, drop `<0.16`
    from `pyproject.toml:27` (and its `# held:` comment), `uv lock --upgrade-package ruff`, and
    re-run the full gate set. That closes the last autonomously verifiable dependency-refresh item.

**Notes:**

- The two `normal` `[review]` issues (Kotlin consumer floor, Unicode 16/17 divergence) remain open
    and are **HUMAN REVIEW REQUESTED** policy calls — the loop must not decide either. They are not
    blockers for the ruff slices.
- After slice D there is no autonomously verifiable dependency work left: what remains is
    `release.yml` GHA refs (only exercised by a real release run; best bundled with the existing
    `release.yml` `if:`-guard issue) and the deferred majors (xunit 3.x, `Microsoft.NET.Test.Sdk`
    18.x, Gradle wrapper, JUnit 6.x, `jni` 0.22, `magnus` 0.8) — each needing its own step.
- Recorded in `decisions.md` (2026-07-25): ruff 0.16 findings are fixed at the source, never
    suppressed; docstring-only `.pyi` bodies are now the convention and were validated against
    mypy/pyright because the stub is a shipped artifact.
- `learnings.md` was pruned back under budget this review (208 → 199 lines): a second batch of
    settled `gen_*_v0` API-parameter facts moved to `learnings-archive.md`.
- No API, performance, or conformance surface was touched — no semver or benchmark run was owed.
