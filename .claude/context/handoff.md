# Handoff

## 2026-07-26 — Gate byte-identity of the vendored test-vector copies (pytest anchor)

**Done:** Added `tests/test_vendored_fixtures.py`, a pytest anchor that gates all five vendored
fixture copies against their two canonical sources: parametrized byte-identity per pair, a
non-vacuity floor on the table, a `git ls-files`-based check that no tracked
`data.json`/`unicode_boundary.json` exists outside the registered table, and an ASCII-only guard on
the boundary fixture. Added one line to `crates/iscc-lib/CLAUDE.md` Conformance Rules naming the
gate so the next propagation slice registers its new copies.

**Files changed:**

- `tests/test_vendored_fixtures.py`: new — 4 test functions, 8 collected cases; explicit
    `VENDORED_COPIES` table (2 canonicals → 5 copies); discovery via
    `git ls-files -z -- '*data.json' '*unicode_boundary.json'` filtered to exact basenames (no
    `rglob`, no `pytest.skip`, no `import iscc_lib`)
- `crates/iscc-lib/CLAUDE.md`: one bullet in Conformance Rules pointing at the gate and the
    register-new-copies rule

**Verification:** (all observed in this session)

- `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed, 0 skips**
- `--collect-only -q` — 8 tests; **5** parametrized byte-identity ids (one per copy path)
- `uv run pytest` (full suite) — **379 passed** in 23.6s
- All **7** tracked fixture paths appear literally in the test file (`grep -F -c` ≥ 1 for each;
    `git ls-files` count = 7)
- **Drift probe:** appended `x` to `packages/go/testdata/data.json` → suite reds with
    `FAILED …[packages/go/testdata/data.json]` and message
    `fix with: cp crates/iscc-lib/tests/data.json packages/go/testdata/data.json`; restored via
    `cp /tmp/dj.bak …`, `cmp` confirms identical to canonical. No git index operation used.
- `git status --porcelain -- packages/ crates/iscc-lib/tests/ .crap-baseline.json .iai-baseline.json`
    — empty; `git status --porcelain -- 'crates/*/src'` — empty
- `uv run ruff check .` — "All checks passed"; `uv run ruff format --check .` — 170 files already
    formatted
- `mise run check` — exit 0, all hooks Passed, no reformats
- `grep -F -c 'test_vendored_fixtures.py' crates/iscc-lib/CLAUDE.md` = 1

**Next:** Resume Unicode boundary propagation with the **C FFI + JNI/Java** slice (cheapest
in-container pair per iteration 151's review). When the later `packages/{dotnet,kotlin,swift}` slice
vendors `unicode_boundary.json` copies, `test_no_unregistered_tracked_copy` will red until each new
copy is added to `VENDORED_COPIES` — that is the gate working as designed, not a bug.

**Notes:**

- The unregistered-copy failure message explicitly says *register the copy in VENDORED_COPIES*, not
    *delete the file*, and the module docstring explains why the copies exist (per-language test
    trees can't reach across the repo root when consumed standalone) — both per next.md.
- The ASCII guard applies **only** to `unicode_boundary.json` (2,344 bytes, stores `\uXXXX`
    escapes); `data.json` legitimately contains raw non-ASCII and is deliberately not covered by it.
- No prek hook added (deliberate — a `files:`-scoped hook never fires on deletion; pytest at
    pre-push + `python-test` CI is the sole mechanism, per next.md Not-In-Scope).
- `git` failure in discovery raises (`check=True`) rather than skipping — a silently skipped gate is
    the failure mode this issue exists to prevent.
- No API, hot path, baseline, or crate source touched; no `decisions.md`-worthy trade-off made.
