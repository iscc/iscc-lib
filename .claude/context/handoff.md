# Handoff

## 2026-07-25 — Review of: ruff 0.16 slice B — enforce `S` + `C901` project-wide via `extend-select`, clear all RUF100

**Verdict:** PASS

**Summary:** A tight, disciplined gate-strengthening step:
`[tool.ruff.lint] extend-select = ["S", "C901"]` promotes the security and complexity scans from two
pre-push-only hooks into every `ruff check` invocation (pre-commit, `mise run lint`, CI), which
simultaneously clears all 15 ruff-0.16 `RUF100` findings without deleting a single load-bearing
`# noqa`. Two genuinely dead directives were removed and the docs were kept accurate. Every next.md
criterion reproduces green, the diff is exactly at the 3-file budget, and no gate was weakened
anywhere in the unpushed range.

**Verification:**

- [x] `uv run ruff check` exits 0 — `All checks passed!` (now covering `S` + `C901`)
- [x] `uv run ruff format --check` exits 0 — `25 files already formatted`
- [x] `uv run ruff check --select S --force-exclude` exits 0 (pre-push security gate intact)
- [x] `uv run ruff check --select C901 --force-exclude` exits 0 (pre-push complexity gate intact)
- [x] `uvx ruff@0.16.0 check . --output-format concise` → `Found 12 errors.` — breakdown matches
    next.md exactly: `I001` ×8, `RUF022` ×1, `RUF007` ×1, `PLW1510` ×1, `EXE001` ×1
- [x] `… | grep -c RUF100` → `0`
- [x] tomllib assertion exits 0 — `{'S','C901'} <= extend-select` and **no `select` key** (the
    important half: `select` would have replaced ruff's `E4`/`E7`/`E9`/`F` defaults)
- [x] `grep -q 'ruff<0.16' pyproject.toml` exits 0 — hold-back pin retained
- [x] `grep -c 'noqa: S603' tools/cid.py` → 6, `grep -c 'noqa: S607' tools/cid.py` → 4
- [x] `uv run python -m compileall -q tools/cid.py tools/metrics.py` exits 0
- [x] `mise run cid:status` exits 0
- [x] `uv run pytest -q` → **314 passed**
- [x] `mise run check` — all 15 hooks Passed, nothing rewritten
- [x] `grep -q 'C901' docs/development.md` exits 0; the pre-commit bullet names both `S` and `C901`
- [x] **Independent probe of the two deletions** (not in next.md):
    `uv run ruff check --select S --ignore-noqa` lists 13 real violations, and
    `tools/metrics.py:174` is **not** among them — the deleted `S603` suppressed nothing, so no hole
    was opened. `uv run ruff check --extend-select RUF100` and `--select S,RUF100` are both clean,
    so no other directive is stale under the pinned 0.15.22
- [x] Scope discipline — 3 non-test/non-doc files (`pyproject.toml`, `tools/cid.py`,
    `tools/metrics.py`) exactly at budget, plus the sanctioned `docs/development.md`. Every
    Not-In-Scope fence held: pin untouched, no `uv lock`, no `--fix` under 0.16, pre-push hooks
    verbatim, no new rule families, no Unicode work, `issues.md` untouched by advance
- [x] Quality-gate integrity across `@{upstream}..HEAD` — no suppressions added, no skips, no
    threshold or hook changes. The two `# noqa` **removals** move in the strengthening direction,
    and `S`/`C901` now run in strictly more places (CI `ruff check` enforced them for the first
    time)
- [x] Extras: `cargo clippy --workspace --all-targets -- -D warnings` clean (only the known dev-only
    `proc-macro-error2` future-incompat note); `uv run zensical build` → "No issues found"

**Issues found:**

- **Rationale inaccuracy (documentation only, no code impact).** next.md and the advance handoff
    attribute the `tools/metrics.py` deletion to "ruff 0.16 refined `S603` to skip static
    list-literal argv". Probed with an isolated repro: `S603` does **not** fire on a fully static
    list literal in 0.15.22 *either*, and `--select S,RUF100` reports the directive as unused
    ("unused: `S603`") in **both** versions. What actually changed in 0.16 is that `RUF100` is in
    the default select ("non-enabled: `S603`" under `--isolated`). The deletion is correct; only the
    stated cause is wrong. Corrected in `learnings.md`, `issues.md` and review memory so it does not
    become folklore.
- **Minor fix applied by this review:** the `# held:` comment on the `ruff<0.16` pin still read "104
    new errors, mostly `_lowlevel.pyi`" — stale by two sub-slices. Rewritten to name the 12 findings
    that actually hold the pin. Comment-only, no behaviour change.
- Note (no issue filed): `.claude/context/specs/ci-cd.md` and `CLAUDE.md` still describe the Ruff
    `S`/`C901` scans under "Pre-push stage". That remains literally true — the hooks still run — so
    it is not misleading; no spec edit needed.

**Codex review:** Clean — "The Ruff configuration correctly extends the default rule set, and the
suppression removals do not affect runtime behavior. Relevant lint, formatting, and focused tests
pass." No actionable findings; it did not independently probe the `S603` attribution.

**Next:** ruff 0.16 **sub-slice C — the isort cluster**: `I001` ×8 (7 test modules +
`crates/iscc-py/python/iscc_lib/__init__.py`) + `RUF022` (`__all__` not sorted in the same
`__init__.py`). This needs one real decision: whether to add `[tool.ruff.lint.isort]` with
`known-first-party`/`src` entries so `iscc_lib` sorts as first-party (the package lives under
`crates/iscc-py/python/`, not the repo root), then let `ruff check --fix --extend-select I,RUF022`
apply the mechanical fixes. **Bundle two things with it:**

1. Add `RUF100` to `extend-select`. It is green at HEAD *today* (verified this review) and is the
    check that would have prevented the stale-directive cluster this slice just cleaned up —
    crystallising the lesson instead of writing it down again.
2. The three remaining one-liners can ride along or take their own step: `RUF007` in
    `scripts/gen_unicode16_unassigned.py` (`zip(...)` → `itertools.pairwise`; the file is a
    checked-in generator, so re-run it and assert `git status --porcelain` on the generated Rust is
    empty), `PLW1510` in `scripts/test_install.py` (add explicit `check=False`), and `EXE001` on
    `tools/cid.py` (shebang without exec bit — either `chmod +x` or drop the shebang; note the
    Windows-bind-mount exec-bit caveat in the devcontainer memory before choosing `chmod`).

Sub-slice D (`uv lock --upgrade-package ruff`, drop the pin) is unblocked only once
`uvx ruff@0.16.0 check .` exits 0.

**Notes:**

- The Unicode work is still parked on the open `normal` `[review]` issue "Freeze-rule ordering
    diverges from iscc-core on sequences" (HUMAN REVIEW REQUESTED). Step (a2) the differential sweep
    and step (b) boundary vectors in 11 bindings both depend on that spec-wording ruling — do not
    start either. Slice C above is genuine, unblocked autonomous work, so this is **not** an IDLE or
    pause iteration.
- The redundancy between the pre-push `--select S`/`--select C901` hooks and the new project
    selection is deliberate and now recorded in `decisions.md` (2026-07-25): the hooks pass
    `--select` on the command line, so a future `pyproject.toml` regression cannot silently disable
    the security scan, and they name the failing gate in push output. A future "simplify the hooks"
    step should be rejected on those grounds.
- Behavioural consequence worth knowing: `S` and `C901` now fire at **commit** time via the
    `ruff check --fix` pre-commit hook, not just at push. New Python code with `assert` outside
    `tests/**`, a `subprocess` call with dynamic argv, or a function over complexity 15 will block a
    commit. That is the intended tightening, not a bug.
- `.claude/context/iterations.jsonl` is modified in the working tree (runner-owned) and was left
    unstaged.
