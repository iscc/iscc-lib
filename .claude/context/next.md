# Next Work Package

## Step: Harden iai-callgrind regression gate against zero-count and disappeared-bench false greens

## Goal

Close the two narrow false-green escape hatches Codex flagged in `scripts/iai_regression.py` (issue
"Harden iai-callgrind regression gate against false-green edge cases" `normal` [review]) so the
enforcing perf gate fails when a shared benchmark collects a zero instruction count or when a
baselined benchmark disappears from the run — keeping the gate trustworthy without any spec change.

## Scope

- **Modify**: `scripts/iai_regression.py` — tighten `check_regressions` (and its `main` wiring) to
    fail on the two edge cases; update the module + function docstrings to describe the new fail
    conditions and the `--allow-missing` opt-out.
- **Create**: `tests/test_iai_regression.py` — synthetic-fixture pytest coverage for the new and
    existing gate behavior.
- **Reference**: `.claude/context/issues.md` (the [review] hardening issue, lines 58-77),
    `.claude/context/handoff.md` (Codex findings 1-2 and the mid-flush-race note),
    `tests/test_cid.py` (the `importlib.util.spec_from_file_location` load-script-by-path pattern to
    mirror), `.iai-baseline.json` (real baseline shape: `metric`/`tolerance_pct`/`benches`).

## Not In Scope

- Do NOT touch the CRAP `--fail-above 30` gate or wire up `cargo deny`/`cargo audit` — both are
    HUMAN-REVIEW-REQUESTED spec amendments, not this step.
- Do NOT change `only_run` (new-bench) behavior — a benchmark present only in the run must keep
    *warning* (not failing) until a deliberate baseline refresh; only the missing/zero directions
    change.
- Do NOT edit the CI `Perf` job YAML or the `bench:iai*` mise tasks — the default `--check`
    invocation simply becomes stricter; no workflow/task change is needed.
- Do NOT regenerate `.iai-baseline.json`, change the 10% tolerance, or add new metrics.
- Do NOT flip the `Semver (cargo-semver-checks)` gate to enforcing or start any v1.0.0 prep.

## Implementation Notes

- `check_regressions(run_dir, baseline_path)` → add an `allow_missing: bool = False` parameter; wire
    a new `--allow-missing` argparse flag in `main()` and pass `args.allow_missing` through.
- **Zero-count fix (Codex finding 1):** among `shared` benches, collect any whose *current* Ir is 0
    into a `zero_benches` list. A zero Ir means callgrind collected nothing for that case (partial
    strip / harness regression), so it must FAIL — currently `cur > base * limit` is False for
    `cur == 0`, reading it as a giant improvement. Fail the gate when `zero_benches` is non-empty
    and print which benches collected zero. (The CI `grep -rEq '^summary: [1-9]'` guard only catches
    the *all-zero* case, so this is the per-bench complement.)
- **Disappeared-bench fix (Codex finding 2):** when `only_baseline` is non-empty, FAIL unless
    `allow_missing` is set — a baselined bench that stops emitting a `.out` must not silently pass.
    With `--allow-missing`, keep the existing warning behavior so an intentional shrink is still
    possible without a refresh. The committed baseline should only shrink via a deliberate
    `--update` refresh.
- Have `check_regressions` return False if ANY of: `regressions`, `zero_benches`, or
    (`only_baseline` and not `allow_missing`). Print a clear `FAIL:` summary for each cause; keep
    the existing per-bench table and the final `OK:` line for the success path.
- Update the module docstring (the `--check` bullet) and the `check_regressions` docstring to state
    that zero-count shared benches and missing baselined benches now fail, and mention
    `--allow-missing`.
- **Mid-flush-race awareness (do not "fix"):** the handoff notes that running `--check` in the *same
    shell command* immediately after `cargo bench` once matched only 9/16 benches before the flush
    completed. In CI the check is its own step after the bench step fully finishes, so making
    missing benches fail is safe there. Use synthetic temp dirs in tests (never a live bench run) so
    the tests are deterministic.
- **Test fixtures:** mirror `tests/test_cid.py` — load the script by path via
    `importlib.util.spec_from_file_location("iai_regression", scripts/iai_regression.py)`. Write a
    helper that materializes a run dir of `<bench>/<bench>.out` files each containing a
    `summary: <Ir> 0 0 ...` line, plus a temp baseline JSON via the module's `write_baseline` or a
    hand-built dict. Cover: (a) all within tolerance → True; (b) one bench regressed >10% → False;
    (c) a shared bench with current Ir 0 → False; (d) a baselined bench missing from the run →
    False; (e) same missing case with `allow_missing=True` → True; (f) a bench only in the run (not
    the baseline) → still True (warns only).

## Verification

- `uv run pytest tests/test_iai_regression.py -q` passes (>= 6 new tests covering cases a-f).
- `uv run pytest -q` full suite stays green (no regressions in the existing tests).
- `uv run ruff check scripts/iai_regression.py tests/test_iai_regression.py` clean.
- `uv run ruff format --check scripts/iai_regression.py tests/test_iai_regression.py` clean.
- `uv run ty check scripts/iai_regression.py` clean.
- `cargo test -p iscc-lib` still passes (script-only change must not affect the Rust core).
- Manual sanity: `python3 scripts/iai_regression.py --check` against an unchanged baseline + a
    matching synthetic run dir (e.g. via `--from-dir`) exits 0; tampering one `.out` to
    `summary: 0 ...` makes it exit 1.

## Done When

`check_regressions` fails (exit 1) on a zero-Ir shared bench and on a missing baselined bench
(unless `--allow-missing`) while keeping the normal pass and >10%-regression-fail behavior, and all
verification checks above pass.
