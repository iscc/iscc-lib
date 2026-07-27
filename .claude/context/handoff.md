# Handoff

## 2026-07-27 — Harden the Unicode sweep gate — require a rebuild, bound the divergence list

**Done:** Closed both blind spots from the `normal` `[review]` issue "Harden the Unicode
differential sweep gate": (1) a bare `uv run scripts/unicode_sweep.py` now fails closed via a new
`check_rebuilt` guard requiring the `--rebuilt` caller assertion (passed only by
`mise run unicode:sweep` and the CI job, both right after their unconditional release build), and
the vacuous freshness pass is fixed (`check_extension_fresh` raises `SystemExit` naming the searched
directories when no `*.rs` sources are found — `default=0.0` is gone); (2) `sweep()` now returns a
`SweepResult(comparisons, divergences, samples)` NamedTuple that counts every divergence exactly but
retains at most `MAX_REPORTED_DIVERGENCES` (20) `Divergence` samples in sweep order, so a broad
regression can no longer OOM-kill CI before diagnostics print.

**Files changed:**

- `scripts/unicode_sweep.py`: added `REBUILD_FLAG` + `check_rebuilt(argv)` (called first in
    `main(argv)`, before `check_oracle`); `check_extension_fresh` collects `*.rs` paths first and
    fails on an empty set; `sweep()` returns `SweepResult` with bounded `samples`; `main()` prints
    samples, an on-failure-only "showing first N of M divergences" line, and the byte-identical
    `TOTAL … comparisons, … divergences` line; entrypoint is `sys.exit(main(sys.argv[1:]))`;
    docstring Usage now names `mise run unicode:sweep`
- `mise.toml`: `unicode:sweep` task's sweep line now passes `--rebuilt` (build line unchanged, still
    `&&`-chained)
- `.github/workflows/ci.yml`: `unicode-sweep` job's sweep step now passes `--rebuilt` (nothing else
    in the job changed)
- `tests/test_unicode_sweep.py`: updated the four `sweep()` call sites to the `SweepResult` shape
    (the delete-filter discrimination test keeps asserting the same three `(function, context)`
    pairs, unweakened — 16 comparisons < the 20 cap); added 4 new tests:
    `test_sweep_caps_retained_samples_with_wrong_oracle` (480/480/20 over 30 scalars),
    `test_check_rebuilt_missing_flag_fails_closed`, `test_check_rebuilt_accepts_flag`,
    `test_check_extension_fresh_rejects_missing_sources` (empty dir and missing dir both)
- `docs/unicode.md`: one sentence in "Differential sweep gate" — the script refuses a bare
    invocation; `mise run unicode:sweep` (or the CI job) is the only way to run it

**Verification:** All next.md criteria pass, each against a tool result from this session:

- `timeout 60 uv run scripts/unicode_sweep.py` exits **1** in ~2 s, stderr names
    `mise run unicode:sweep`, stdout contains zero `TOTAL` lines (grep-counted)
- `mise run unicode:sweep` exits 0; last stdout line verified byte-exact with `od -c`:
    `TOTAL 17793024 comparisons, 0 divergences` (provenance line
    `oracle: iscc-core 1.3.0, unidata 16.0.0, python 3.14.6` above it)
- `grep -c "default=0.0" scripts/unicode_sweep.py` → `0`
- `uv run pytest -q tests/test_unicode_sweep.py` → **14 passed in 0.72 s** (10 updated + 4 new);
    includes all three mandated guard assertions (480/480/`MAX_REPORTED_DIVERGENCES`;
    `check_rebuilt([])` SystemExit naming `mise run unicode:sweep` / `check_rebuilt([REBUILD_FLAG])`
    → `None`; empty **and** missing source dirs both raise)
- `uv run pytest -q` → **393 passed** (389 pre-existing + 4 new, none removed)
- next.md's PyYAML job-shape one-liner prints `OK` (exactly one `unicode_sweep.py` step, carrying
    `--rebuilt`)
- `uv run ruff check`, `uv run ruff format --check`, `uv run ty check` all exit 0; pre-push-only
    `ruff check --select S,C901` also run explicitly, clean
- `uv run zensical build` exits 0 with "No issues found"; `uv run scripts/check_docs_nav.py` →
    `OK: 23 documentation pages consistent`
- `mise run check` exits 0 (first run had mdformat auto-reflow my `docs/unicode.md` paragraph;
    second run all hooks Passed with the reflowed text kept)
- `git status --porcelain` over `crates/`, `.crap-baseline.json`, `.iai-baseline.json`,
    `.claude/context/specs/`, and `crates/iscc-lib/tests/unicode_boundary.json` — all empty; working
    tree holds exactly the 5 in-scope files + runner-owned `iterations.jsonl` (left unstaged)

**Next:** Propagation slice 5 remains the open thread: C FFI is the only autonomously-completable
surface (`tests/test_iscc.c` has no JSON reader — needs the design call between a generated vector
header and hand-pinned expected strings; `gcc` is present, `cmake`/`swift` are not). The "Harden the
Unicode differential sweep gate" issue this step implements can be closed by the review agent if
verification holds.

**Notes:**

- Design followed next.md exactly: the `--rebuilt` flag is a caller *assertion*, not an observation
    — a human passing it without rebuilding is trusted (stated in the `check_rebuilt` docstring,
    which also cites the decisions.md 2026-07-27 operating rule it hardens). `check_extension_fresh`
    is retained as the cheap redundant mtime observation behind it.
- The "showing first N of M divergences" line prints only when `divergences > len(samples)`, i.e.
    only on a >20-divergence failure — success output is byte-identical to before (verified: the
    green run prints exactly the provenance line + TOTAL line).
- No Rust source, no API surface, no benchmarked hot path touched; no baseline refresh needed and
    none made. No lint suppressions, no skips added.
- The mtime widening to `Cargo.toml`/`Cargo.lock`/`rust-toolchain*` and the in-script maturin
    shell-out were deliberately NOT done, per next.md Not-In-Scope and decisions.md 2026-07-27.
