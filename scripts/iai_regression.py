"""Check iai-callgrind instruction counts against a committed regression baseline.

This is the enforcing half of the v1.0.0 performance-regression gate (issue #3). The
`Perf (iai-callgrind)` CI job runs `cargo bench -p iscc-lib --bench iai_benches`, which
writes one callgrind `.out` file per benchmark case under `target/iai/`. Each `.out` file
carries a `summary: <Ir> <Dr> <Dw> ...` line whose first value is the instruction count
(Ir) — the only deterministic, machine-stable metric callgrind reports, so it is the sole
metric this gate compares.

Two modes:

- ``--check`` (default): parse the current run directory, load `.iai-baseline.json`, and
  exit non-zero if any benchmark present in BOTH the run and the baseline regresses more
  than the tolerance (default 10%) versus its committed baseline Ir. Benchmarks only in the
  run (not the baseline) are warned about but never fail the gate.
- ``--update``: rebuild `.iai-baseline.json` from a run directory. The committed baseline is
  built from the CI artifact (not a local run) so it matches the rustc the gate measures
  with; refresh it deliberately in a reviewed commit, mirroring `.crap-baseline.json`.

Stdlib-only (no third-party deps) so the CI `perf` job can run it with bare `python3`,
without a `uv`/Python toolchain setup step.

Usage:
    python3 scripts/iai_regression.py --check               # gate the current run
    python3 scripts/iai_regression.py --update              # rebuild baseline from target/iai/
    python3 scripts/iai_regression.py --update --from-dir /tmp/ci-iai
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_BASELINE = ROOT / ".iai-baseline.json"
DEFAULT_RUN_DIR = ROOT / "target" / "iai"
METRIC = "Ir"
DEFAULT_TOLERANCE_PCT = 10.0


def find_out_files(run_dir: Path) -> list[Path]:
    """Return the callgrind `.out` result files under a run directory.

    Excludes `.out.old` re-run companions, which `pathlib`'s `*.out` glob already
    skips because their names do not end in `.out`.
    """
    return sorted(run_dir.glob("**/*.out"))


def parse_ir(out_file: Path) -> int:
    """Parse the instruction count (Ir) from a callgrind `.out` file.

    The `summary:` line is `summary: <Ir> <Dr> <Dw> ...`; the first integer is Ir.
    Raises `ValueError` if no summary line is found.
    """
    for line in out_file.read_text(encoding="utf-8").splitlines():
        if line.startswith("summary:"):
            return int(line.split()[1])
    raise ValueError(f"no 'summary:' line found in {out_file}")


def bench_id(out_file: Path) -> str:
    """Return the benchmark id for a `.out` file (its leaf directory name).

    The leaf directory is `<bench_fn>.<bench_id>` (e.g. `bench_cdc_chunks.bytes_1m`),
    which is unique across the suite and used as the baseline JSON key.
    """
    return out_file.parent.name


def collect_irs(run_dir: Path) -> dict[str, int]:
    """Map each benchmark id to its instruction count from a run directory."""
    out_files = find_out_files(run_dir)
    if not out_files:
        sys.exit(f"error: no callgrind '.out' files found under {run_dir}")
    return {bench_id(f): parse_ir(f) for f in out_files}


def write_baseline(run_dir: Path, baseline_path: Path, tolerance_pct: float) -> None:
    """Write a baseline JSON file from a run directory's instruction counts."""
    benches = collect_irs(run_dir)
    payload = {
        "metric": METRIC,
        "tolerance_pct": tolerance_pct,
        "benches": dict(sorted(benches.items())),
    }
    baseline_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    print(
        f"Wrote {baseline_path} with {len(benches)} {METRIC} entries (from {run_dir})."
    )


def load_baseline(baseline_path: Path) -> tuple[dict[str, int], float]:
    """Load benchmark Ir values and the regression tolerance from a baseline file."""
    if not baseline_path.exists():
        sys.exit(
            f"error: baseline {baseline_path} not found; regenerate it with "
            f"'python3 scripts/iai_regression.py --update'"
        )
    data = json.loads(baseline_path.read_text(encoding="utf-8"))
    benches = {str(k): int(v) for k, v in data.get("benches", {}).items()}
    tolerance_pct = float(data.get("tolerance_pct", DEFAULT_TOLERANCE_PCT))
    return benches, tolerance_pct


def check_regressions(run_dir: Path, baseline_path: Path) -> bool:
    """Compare a run against the baseline; return True if no benchmark regressed.

    Fails (returns False) when any benchmark present in BOTH the run and the baseline
    has an instruction count more than `tolerance_pct` above its baseline value.
    Benchmarks only in the run warn but do not fail the gate.
    """
    baseline, tolerance_pct = load_baseline(baseline_path)
    current = collect_irs(run_dir)
    limit = 1.0 + tolerance_pct / 100.0

    shared = sorted(set(baseline) & set(current))
    only_run = sorted(set(current) - set(baseline))
    only_baseline = sorted(set(baseline) - set(current))

    print(f"{'benchmark':<36} {'baseline':>14} {'current':>14} {'delta%':>9}")
    print("-" * 76)
    regressions: list[tuple[str, int, int, float]] = []
    for name in shared:
        base = baseline[name]
        cur = current[name]
        delta_pct = (cur - base) / base * 100.0 if base else 0.0
        flag = "  REGRESSION" if cur > base * limit else ""
        if flag:
            regressions.append((name, base, cur, delta_pct))
        print(f"{name:<36} {base:>14,} {cur:>14,} {delta_pct:>+8.2f}%{flag}")

    for name in only_run:
        print(f"warning: '{name}' is in the run but not the baseline (not gated).")
    for name in only_baseline:
        print(f"warning: '{name}' is in the baseline but not the run.")

    print()
    if regressions:
        print(f"FAIL: {len(regressions)} benchmark(s) regressed > {tolerance_pct:g}%:")
        for name, base, cur, delta_pct in regressions:
            print(f"  {name}: {base:,} -> {cur:,} ({delta_pct:+.2f}%)")
        return False

    print(
        f"OK: {len(shared)} benchmark(s) within {tolerance_pct:g}% of baseline ({METRIC})."
    )
    return True


def main() -> None:
    """Entry point: build or check the iai-callgrind instruction-count baseline."""
    parser = argparse.ArgumentParser(
        description="Check iai-callgrind instruction counts against a committed baseline."
    )
    parser.add_argument(
        "--update",
        action="store_true",
        help="rebuild the baseline from --from-dir instead of checking",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="check the current run against the baseline (default)",
    )
    parser.add_argument(
        "--from-dir",
        type=Path,
        default=DEFAULT_RUN_DIR,
        help=f"directory of callgrind .out files (default: {DEFAULT_RUN_DIR})",
    )
    parser.add_argument(
        "--baseline",
        type=Path,
        default=DEFAULT_BASELINE,
        help=f"baseline JSON file (default: {DEFAULT_BASELINE})",
    )
    parser.add_argument(
        "--tolerance-pct",
        type=float,
        default=DEFAULT_TOLERANCE_PCT,
        help=f"regression tolerance percent for --update (default: {DEFAULT_TOLERANCE_PCT})",
    )
    args = parser.parse_args()

    if args.update:
        write_baseline(args.from_dir, args.baseline, args.tolerance_pct)
    elif not check_regressions(args.from_dir, args.baseline):
        sys.exit(1)


if __name__ == "__main__":
    main()
