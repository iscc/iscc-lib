"""Tests for the CI job table parity gate in ``scripts/check_ci_job_table.py``.

The anchor test runs the checks against the real repository tree and asserts zero
errors — this is what carries the gate into CI, since CI runs pytest but never prek.
The mutation tests build a throwaway ``ci.yml`` plus spec markdown in ``tmp_path``
and cover the failure modes: a job key with no table row, a row naming a job that
does not exist, and the count floor tripping on near-empty inputs. No network, no
mocks beyond fixture files.
"""

import importlib.util
from pathlib import Path

import pytest

# Load scripts/check_ci_job_table.py by path — it is a repo gate script, not a package.
_SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "check_ci_job_table.py"
_spec = importlib.util.spec_from_file_location("check_ci_job_table", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
cjt = importlib.util.module_from_spec(_spec)
_loader.exec_module(cjt)

# Fixture job keys: 12 entries, comfortably above the count floor of 10, so a
# consistent fixture passes and shrinking either side below 10 trips the floor.
JOBS = [
    "rust",
    "python-test",
    "python",
    "nodejs",
    "wasm",
    "c-ffi",
    "dotnet",
    "java",
    "go",
    "ruby",
    "bench",
    "perf",
]


def _write_fixtures(tmp_path, ci_jobs=JOBS, table_jobs=JOBS):
    """Write a minimal ci.yml and spec markdown into tmp_path; return their paths.

    The spec file always carries a second section with a backticked row
    (`ghost-job`) that must never feed the job table scan.
    """
    ci_yml = tmp_path / "ci.yml"
    jobs_yaml = "".join(f"  {job}:\n    runs-on: ubuntu-latest\n" for job in ci_jobs)
    ci_yml.write_text(f"name: CI\njobs:\n{jobs_yaml}", encoding="utf-8")

    spec_md = tmp_path / "ci-cd.md"
    rows = "".join(f"| `{job}` | checks {job} |\n" for job in table_jobs)
    spec_md.write_text(
        "# Spec\n\n## CI Workflow — Quality Gates\n\n"
        "| Job | What it checks |\n| --- | --- |\n"
        f"{rows}\n"
        "## Another Section\n\n"
        "| Key | Value |\n| --- | --- |\n| `ghost-job` | not a CI job |\n",
        encoding="utf-8",
    )
    return ci_yml, spec_md


def test_real_repo_passes():
    # The gate must pass on the repository exactly as committed, and main() (the
    # prek hook entry point) must exit 0. The floor keeps a near-empty job set
    # from reading as a pass: equal empty sets are "consistent" but prove nothing.
    assert len(cjt.ci_jobs(cjt.CI_YML)) >= cjt.COUNT_FLOOR
    assert len(cjt.spec_jobs(cjt.SPEC_MD)) >= cjt.COUNT_FLOOR
    assert cjt.run_checks(cjt.CI_YML, cjt.SPEC_MD) == []
    assert cjt.main() == 0


def test_fixture_tree_passes(tmp_path):
    # A consistent fixture yields no errors; the backticked row in the section
    # after the job table does not count as a job row.
    assert cjt.run_checks(*_write_fixtures(tmp_path)) == []


def test_missing_row_fires(tmp_path):
    # Deleting one table row is reported with that job key.
    table = [job for job in JOBS if job != "wasm"]
    errors = cjt.run_checks(*_write_fixtures(tmp_path, table_jobs=table))
    assert errors == ["spec table: missing 1 job row(s): ['wasm']"]


def test_nonexistent_job_row_fires(tmp_path):
    # A table row naming a job absent from ci.yml is reported with that key.
    errors = cjt.run_checks(*_write_fixtures(tmp_path, table_jobs=[*JOBS, "ghost"]))
    assert errors == ["spec table: 1 row(s) naming nonexistent job(s): ['ghost']"]


def test_duplicate_row_fires(tmp_path):
    # A key listed twice keeps both sets equal, so only row-order inspection can
    # see it — the table's invariant is exactly one row per job key.
    errors = cjt.run_checks(*_write_fixtures(tmp_path, table_jobs=[*JOBS, "wasm"]))
    assert errors == ["spec table: 1 duplicated job row(s): ['wasm']"]


def test_count_floor_fires_on_both_sides(tmp_path):
    # Two equal but near-empty sets must not pass vacuously: both floors trip
    # even though the set-equality comparison itself finds no mismatch.
    small = JOBS[:3]
    errors = cjt.run_checks(*_write_fixtures(tmp_path, ci_jobs=small, table_jobs=small))
    assert len(errors) == 2
    assert "ci.yml: only 3 job key(s) parsed" in errors[0]
    assert "spec table: only 3 job row(s) parsed" in errors[1]


def test_main_exits_nonzero_on_missing_row(tmp_path, monkeypatch, capsys):
    # main() (the prek hook entry point) returns 1 and prints the mismatch.
    ci_yml, spec_md = _write_fixtures(tmp_path, table_jobs=JOBS[:-1])
    monkeypatch.setattr(cjt, "CI_YML", ci_yml)
    monkeypatch.setattr(cjt, "SPEC_MD", spec_md)
    assert cjt.main() == 1
    assert "spec table: missing 1 job row(s): ['perf']" in capsys.readouterr().out


def test_missing_section_heading_exits(tmp_path):
    # A spec file without the quality-gates heading is a hard error, not an
    # empty row set (which would surface as a confusing floor failure).
    spec_md = tmp_path / "ci-cd.md"
    spec_md.write_text("# Spec\n\n## Some Other Section\n", encoding="utf-8")
    with pytest.raises(SystemExit, match="no `## CI Workflow — Quality Gates`"):
        cjt.spec_jobs(spec_md)


def test_missing_jobs_mapping_exits(tmp_path):
    # A workflow file without a `jobs:` mapping is a hard error.
    ci_yml = tmp_path / "ci.yml"
    ci_yml.write_text("name: CI\n", encoding="utf-8")
    with pytest.raises(SystemExit, match="no `jobs:` mapping"):
        cjt.ci_jobs(ci_yml)
