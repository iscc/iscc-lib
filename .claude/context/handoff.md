## 2026-06-18 — Harden iai-callgrind regression gate against zero-count and disappeared-bench false greens

**Done:** Closed the two false-green escape hatches Codex flagged in `scripts/iai_regression.py`:
`check_regressions` now FAILS when any shared benchmark collects a zero instruction count (partial
strip / harness regression that the relative comparison would read as a giant improvement) and when
a baselined benchmark disappears from the run, with a new `allow_missing`/`--allow-missing` opt-out
that downgrades the disappeared-bench failure back to a warning for an intentional shrink. Added
synthetic-fixture pytest coverage.

**Files changed:**

- `scripts/iai_regression.py`: `check_regressions(run_dir, baseline_path, allow_missing=False)` —
    collects shared benches with current Ir 0 into `zero_benches` (fail), fails on non-empty
    `only_baseline` unless `allow_missing`, and aggregates all causes into a `failed` flag with
    distinct `FAIL:` blocks per cause; the `only_baseline` warning hints at `--allow-missing`. Added
    `--allow-missing` argparse flag wired through `main()`. Updated module + function docstrings to
    describe the new fail conditions. Existing >10%-regression and normal-pass behavior unchanged;
    `only_run` (new-bench) still warns only, per Not-In-Scope.
- `tests/test_iai_regression.py` (new): 11 tests loading the script by path via
    `importlib.util.spec_from_file_location` (mirrors `test_cid.py`). Helpers materialize synthetic
    `<bench>/<bench>.out` run dirs and baseline JSON — no live bench run. Covers cases a-f from
    next.md plus boundary (exactly +10% passes, +1 Ir over fails), zero-with-allow-missing still
    fails, write/check roundtrip, and `parse_ir`.

**Verification:**

- `uv run pytest tests/test_iai_regression.py -q` → 11 passed.
- `uv run pytest -q` full suite → 297 passed, 1 warning (pre-existing benchmark legend).
- `uv run ruff check` + `uv run ruff format --check` → clean on both files.
- `uv run ty check scripts/iai_regression.py` → clean.
- `cargo test -p iscc-lib` → passes (script-only change, Rust core unaffected).
- `mise run check` → all 15 pre-commit hooks pass.
- Manual sanity (synthetic run dir built from the committed 16-entry `.iai-baseline.json`):
    `--check` on a matching run exits 0; tampering one `.out` to `summary: 0` exits 1 with
    `FAIL: 1 benchmark(s) collected a zero Ir count`; removing a bench dir exits 1 by default and
    exits 0 with `--allow-missing`.

**Next:** Issue #3 (iai-callgrind perf gate) and this hardening follow-up are both functionally
complete. The remaining `normal` issues (CRAP `--fail-above 30`, cargo-deny/audit gate) are
HUMAN-REVIEW-REQUESTED spec amendments — do not start autonomously. update-state should still
confirm the post-push CI `Perf` job is green. No new CID-actionable script work is queued; the
natural next milestone is human-driven v1.0.0 prep (do NOT cut autonomously).

**Notes:**

- The CI `Perf` job's default `--check` invocation simply becomes stricter — no workflow/task YAML
    change needed (per Not-In-Scope), and none was made. The committed `.iai-baseline.json`,
    tolerance, and metric set are untouched.
- The mid-flush race the prior handoff noted (running `--check` in the same shell command right
    after `cargo bench` once matched only 9/16 benches before the flush completed) means the
    now-failing disappeared-bench path relies on the CI step running *after* the bench step fully
    finishes — which it does (separate step). Tests use synthetic temp dirs only, so they are
    deterministic and never touch a live run. If a future CI flake ever shows a transient missing
    bench, `--allow-missing` is the deliberate escape hatch (do not weaken the default).
- The zero-count guard is independent of `--allow-missing` (a zero Ir always fails), complementing
    the CI `grep -rEq '^summary: [1-9]'` guard which only catches the all-zero case.
