"""Tests for the iai-callgrind regression gate in ``scripts/iai_regression.py``.

Covers the enforcing ``check_regressions`` gate: the normal pass, the >10% regression
fail, and the two hardened false-green edge cases (a shared benchmark collecting a zero
instruction count, and a baselined benchmark disappearing from the run) plus the
``--allow-missing`` opt-out. All fixtures are synthetic temp dirs of fake callgrind
``.out`` files — never a live bench run — so the tests are deterministic.
"""

import importlib.util
import json
from pathlib import Path

# Load scripts/iai_regression.py by path — it is a stdlib-only CI script, not a package.
_IAI_PATH = Path(__file__).resolve().parents[1] / "scripts" / "iai_regression.py"
_spec = importlib.util.spec_from_file_location("iai_regression", _IAI_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
iai = importlib.util.module_from_spec(_spec)
_loader.exec_module(iai)


def _make_run_dir(tmp_path, benches):
    """Materialize a run dir of ``<bench>/<bench>.out`` files with summary lines.

    Each ``.out`` carries a ``summary: <Ir> 0 0 ...`` line so ``parse_ir`` reads the
    given instruction count. Returns the run directory path.
    """
    run_dir = tmp_path / "iai"
    for name, ir in benches.items():
        leaf = run_dir / name
        leaf.mkdir(parents=True, exist_ok=True)
        (leaf / f"{name}.out").write_text(
            f"summary: {ir} 0 0 0 0 0 0 0 0\n", encoding="utf-8"
        )
    return run_dir


def _make_baseline(tmp_path, benches, tolerance_pct=10.0):
    """Write a baseline JSON file with the given Ir values; return its path."""
    path = tmp_path / "baseline.json"
    payload = {
        "metric": "Ir",
        "tolerance_pct": tolerance_pct,
        "benches": dict(sorted(benches.items())),
    }
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    return path


def test_within_tolerance_passes(tmp_path):
    # Case (a): every shared bench is within the 10% tolerance -> True.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000, "bench_b.case1": 2000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1050, "bench_b.case1": 1900})
    assert iai.check_regressions(run, baseline) is True


def test_regression_over_tolerance_fails(tmp_path):
    # Case (b): one bench rises >10% over baseline -> False.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000, "bench_b.case1": 2000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1200, "bench_b.case1": 2000})
    assert iai.check_regressions(run, baseline) is False


def test_zero_current_count_fails(tmp_path):
    # Case (c): a shared bench reporting Ir 0 must fail, not read as an improvement.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000, "bench_b.case1": 2000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 0, "bench_b.case1": 2000})
    assert iai.check_regressions(run, baseline) is False


def test_zero_count_fails_even_with_allow_missing(tmp_path):
    # The zero-count guard is independent of --allow-missing (which only affects the
    # disappeared-bench direction).
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 0})
    assert iai.check_regressions(run, baseline, allow_missing=True) is False


def test_missing_baselined_bench_fails(tmp_path):
    # Case (d): a baselined bench absent from the run must fail by default.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000, "bench_b.case1": 2000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1000})
    assert iai.check_regressions(run, baseline) is False


def test_missing_baselined_bench_allowed_with_flag(tmp_path):
    # Case (e): the same missing bench passes when --allow-missing is set.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000, "bench_b.case1": 2000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1000})
    assert iai.check_regressions(run, baseline, allow_missing=True) is True


def test_only_run_bench_warns_only(tmp_path):
    # Case (f): a bench present only in the run (not the baseline) warns but passes.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1000, "bench_new.case1": 5000})
    assert iai.check_regressions(run, baseline) is True


def test_boundary_exactly_at_tolerance_passes(tmp_path):
    # Exactly +10% is within tolerance (strict `>` comparison) -> True.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1100})
    assert iai.check_regressions(run, baseline) is True


def test_just_over_boundary_fails(tmp_path):
    # One instruction past +10% trips the gate.
    baseline = _make_baseline(tmp_path, {"bench_a.case1": 1000})
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1101})
    assert iai.check_regressions(run, baseline) is False


def test_write_and_check_roundtrip(tmp_path):
    # write_baseline -> check against the same run is a clean 0% pass.
    run = _make_run_dir(tmp_path, {"bench_a.case1": 1000, "bench_b.case1": 2000})
    baseline = tmp_path / "baseline.json"
    iai.write_baseline(run, baseline, 10.0)
    assert iai.check_regressions(run, baseline) is True


def test_parse_ir_reads_first_summary_value(tmp_path):
    # parse_ir returns the first integer of the summary line (Ir).
    out = tmp_path / "bench.out"
    out.write_text("desc: foo\nsummary: 4607021 12 34\n", encoding="utf-8")
    assert iai.parse_ir(out) == 4607021
