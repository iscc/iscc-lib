# Next Work Package

## Step: iai-callgrind perf gate slice 2b — committed Ir baseline + >10% regression gate

## Goal

Close the performance-regression gate (issue #3, slice 2b): commit a frozen instruction-count
baseline and make the `Perf (iai-callgrind)` CI job fail when any benchmarked path's instruction
count (Ir) regresses more than 10% versus that baseline. This flips the last unchecked perf-gate
criteria in `rust-core.md` / `ci-cd.md` and lets issue #3 close.

## Goal context (issue)

Picks up issue **"Add `iai-callgrind` performance-regression CI gate" `normal` [human]**. Slice 2a
(harness + Perf job collecting real non-zero counts) already landed and CI is GREEN; this is the
final slice — the committed baseline + the enforcing >10% regression comparison.

## Scope

- **Create**: `scripts/iai_regression.py` — a stdlib-only (no third-party deps) Python script with
    two modes: build/update the baseline from a directory of callgrind `.out` files, and check the
    current `target/iai/` run against the committed baseline, exiting non-zero on a >10% Ir
    regression.
- **Create (generated data, excluded from file budget)**: `.iai-baseline.json` at the repo root —
    the committed baseline, built from the CI artifact (see Implementation Notes), keyed by bench id
    → Ir count. Mirrors the existing committed `.crap-baseline.json`.
- **Modify**: `.github/workflows/ci.yml` — add a regression-check step to the `perf` job that runs
    the script (keep the existing zero-collection guard).
- **Modify**: `mise.toml` — add a `bench:iai:baseline` task that regenerates `.iai-baseline.json`
    from a local run (deliberate reviewed refresh, mirroring `crap:baseline`); optionally a
    `bench:iai:check` task for local verification.
- **Modify (docs/spec, excluded from file budget)**: `.claude/context/specs/rust-core.md` (flip the
    two perf checkboxes near lines 374-376), `.claude/context/specs/ci-cd.md` (flip the perf
    checkbox near lines 424-425).
- **Reference**: `crates/iscc-lib/benches/iai_benches.rs` (16 bench cases, do not edit),
    `.github/workflows/ci.yml` lines 280-324 (existing `perf` job), `mise.toml` lines 124-133
    (existing `bench:iai`), `scripts/version_sync.py` (Python script style: module docstring, type
    hints, ruff/ty compliance), `.crap-baseline.json` + `mise.toml` `crap:baseline` task (the
    committed-baseline precedent to mirror).

## Not In Scope

- **Do NOT gate on any metric other than Ir** (instructions). The `summary:` line is
    `Ir Dr Dw I1mr D1mr D1mw ILmr DLmr DLmw`; only the first value (Ir) is deterministic. Cache/miss
    counts are ASLR/cache-state noisy and must not be part of the gate.
- **Do NOT remove the zero-collection guard.** An all-zero (stripped-binary) collection looks like a
    *decrease*, which the >10%-increase regression check would happily pass — so the guard is still
    needed to catch that failure mode.
- **Do NOT use iai-callgrind's native `--save-baseline`/`RegressionConfig` with restored
    `target/iai/` state across CI runs.** `target/` is gitignored and the `.out` files carry
    machine-specific paths; use the committed JSON + script approach (matches the `.crap-baseline`
    precedent and keeps the baseline portable/reviewable).
- **Do NOT edit the bench harness** (`iai_benches.rs`) — it is correct as of slice 2a.
- **Do NOT flip the `Semver (cargo-semver-checks)` gate to enforcing** — that is deliberately tied
    to the v1.0.0 cut.
- **Do NOT touch the other two normal issues** (CRAP `--fail-above 30`, cargo-deny/audit gate) —
    both are HUMAN-REVIEW-REQUESTED spec amendments.
- Do not start v1.0.0 release prep.

## Implementation Notes

**On-disk layout** (one group `iscc_benches`, 16 cases):
`target/iai/iscc-lib/iai_benches/iscc_benches/<bench_fn>.<bench_id>/callgrind.<bench_fn>.<bench_id>.out`.
Each `.out` has a line `summary: <Ir> <Dr> <Dw> ...`; the first integer is Ir. Read only `.out`
files (ignore `.out.old` re-run companions). Use the leaf dir name `<bench_fn>.<bench_id>` (e.g.
`bench_cdc_chunks.bytes_1m`) as the JSON key — there are 16 unique ones.

**Script (`scripts/iai_regression.py`):**

- Default/`--check`: scan `target/iai/`, parse each bench's Ir, load `.iai-baseline.json`, and for
    every bench present in BOTH, fail (exit 1) if `current_Ir > baseline_Ir * 1.10`. Print a table
    (bench, baseline, current, delta%). Benches in the run but not the baseline → warn, do not fail
    (regression gate compares against baseline only). Error clearly if `.iai-baseline.json` is
    missing.
- `--update` (or `--baseline-out PATH`): write `.iai-baseline.json` from a source dir (`--from-dir`,
    default `target/iai/`). JSON shape suggestion:
    `{"metric": "Ir", "tolerance_pct": 10.0, "benches": {"<id>": <Ir>, ...}}`.
- Keep it stdlib-only (`json`, `pathlib`, `argparse`, `re`/`glob`) so the CI `perf` job needs no
    `uv`/Python setup — `python3 scripts/iai_regression.py --check` runs on ubuntu-latest as-is. Add
    module + function docstrings and type hints so `ruff`/`ty` pre-push hooks stay clean.

**Building the COMMITTED baseline from CI (avoids local-rustc drift):** local rustc is 1.96.0 but CI
uses `dtolnay/rust-toolchain@stable`; instruction counts can differ across rustc versions. Build the
committed `.iai-baseline.json` from the CI artifact so it matches what the CI gate will measure:
`gh run download <latest-green-Perf-run> -n iai-baseline -D /tmp/ci-iai` (the last known green run
is 27746693860 on `1463edb`; prefer the latest successful `Perf` run on develop), then
`python3 scripts/iai_regression.py --update --from-dir /tmp/ci-iai`. Do this LAST so it is the file
that gets committed (a local-run baseline generated during script testing must be overwritten by the
CI-sourced one).

**CI wiring (`perf` job):** add a step after the zero-collection guard, before upload:
`Check perf regression` → `python3 scripts/iai_regression.py --check`. The job already has no
`continue-on-error`, so this enforces. Consider `if: always()` on the upload step so the artifact is
preserved even when the regression step fails (aids debugging a real regression).

**mise task:** `bench:iai:baseline` should run the benches then `--update` (mirror `crap:baseline`'s
`depends`/run shape and keep the `IAI_CALLGRIND_ALLOW_ASLR=true` env from `bench:iai`).

## Verification

- `mise run bench:iai` completes locally and populates `target/iai/` with non-zero `summary:` lines
    (valgrind 3.19.0 + iai-callgrind-runner are present in the devcontainer).
- `mise run bench:iai:baseline` regenerates `.iai-baseline.json` containing exactly 16 bench Ir
    entries; the file is valid JSON.
- Self-consistency: `python3 scripts/iai_regression.py --check` against a baseline generated from
    the same local run exits 0 (0% delta).
- Failure path: temporarily lowering one baseline Ir value by 50% makes
    `python3 scripts/iai_regression.py --check` exit 1 and name the regressed bench.
- `.iai-baseline.json` is committed at the repo root (sourced from the CI `iai-baseline` artifact,
    not the local run) and is not gitignored (`git check-ignore .iai-baseline.json` prints nothing).
- `.github/workflows/ci.yml` `perf` job contains a regression-check step invoking the script AND
    still contains the `Assert non-zero instruction collection` guard.
- `mise.toml` has a `bench:iai:baseline` task.
- `.claude/context/specs/rust-core.md` perf checkboxes ("committed baseline" and ">10% regression")
    are `[x]`; `.claude/context/specs/ci-cd.md` perf checkbox (line ~424) is `[x]`.
- `mise run check` passes (pre-commit hooks: YAML/TOML/JSON validation + ruff/ty on the new script).
- `cargo test -p iscc-lib` still passes (no harness change).
- **CI-only (confirm next cycle):** the post-push `Perf` job's regression-check step passes against
    the committed baseline (counts within 10%).

## Done When

`scripts/iai_regression.py` + a CI-sourced committed `.iai-baseline.json` + a wired `perf`-job
regression step + a `bench:iai:baseline` refresh task are in place, the script is locally verified
(generates a 16-entry baseline, passes self-check, fails on a tampered baseline), `mise run check`
is clean, and the `rust-core.md` / `ci-cd.md` perf-gate checkboxes are flipped — completing slice 2b
of issue #3.
