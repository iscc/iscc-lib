"""Parity check for the CI job table in `.claude/context/specs/ci-cd.md`.

The spec's "CI Workflow — Quality Gates" table is hand-written and has repeatedly
drifted as jobs were added to `.github/workflows/ci.yml`. This gate parses the
`jobs:` keys from `ci.yml` and the backticked job keys from the first column of the
spec table (scanning only the table's own section, so no other table can feed it
rows) and asserts the two sets are equal. It reports every mismatch — job keys with
no table row and rows naming a job that does not exist — before exiting non-zero.

A count floor guards against a vacuous pass: two empty (or near-empty) sets compare
equal, so either side yielding fewer than 10 keys is an error in itself. The exact
job count is deliberately NOT asserted — it drifts with every legitimate CI change.

Requires PyYAML (declared in the `dev` dependency group).

Runs as a prek hook scoped to the two inputs and in CI via
`tests/test_check_ci_job_table.py`.

Usage:
    uv run scripts/check_ci_job_table.py
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
CI_YML = ROOT / ".github" / "workflows" / "ci.yml"
SPEC_MD = ROOT / ".claude" / "context" / "specs" / "ci-cd.md"

# Fewer keys than this on either side means a parse failure or a gutted input,
# not a legitimately small CI — fail loudly instead of comparing near-empty sets.
COUNT_FLOOR = 10

# The spec section owning the job table: from its heading to the next `## ` heading
# (or end of file). Scanning only this span keeps other tables out of the row set.
SECTION_RE = re.compile(
    r"^## CI Workflow — Quality Gates$(?P<body>.*?)(?=^## |\Z)",
    re.DOTALL | re.MULTILINE,
)

# A table row whose first cell is a single backticked job key, e.g. `| `rust` | … |`.
# The header (`| Job |`) and divider (`| --- |`) rows carry no backticks and never match.
JOB_ROW_RE = re.compile(r"^\|\s*`([^`]+)`\s*\|", re.MULTILINE)


def ci_jobs(ci_yml: Path) -> set[str]:
    """Return the `jobs:` keys of the CI workflow file."""
    data = yaml.safe_load(ci_yml.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or not isinstance(data.get("jobs"), dict):
        sys.exit(f"error: no `jobs:` mapping found in {ci_yml}")
    return set(data["jobs"])


def spec_jobs(spec_md: Path) -> set[str]:
    """Return the backticked job keys from the spec's quality-gates job table."""
    match = SECTION_RE.search(spec_md.read_text(encoding="utf-8"))
    if match is None:
        sys.exit(f"error: no `## CI Workflow — Quality Gates` section in {spec_md}")
    return set(JOB_ROW_RE.findall(match.group("body")))


def run_checks(ci_yml: Path, spec_md: Path) -> list[str]:
    """Compare the spec job table against the workflow jobs; return all errors."""
    workflow = ci_jobs(ci_yml)
    table = spec_jobs(spec_md)
    errors = []
    if len(workflow) < COUNT_FLOOR:
        errors.append(
            f"ci.yml: only {len(workflow)} job key(s) parsed "
            f"(floor {COUNT_FLOOR}) — parse failure or gutted workflow?"
        )
    if len(table) < COUNT_FLOOR:
        errors.append(
            f"spec table: only {len(table)} job row(s) parsed "
            f"(floor {COUNT_FLOOR}) — table moved or rows reformatted?"
        )
    missing = sorted(workflow - table)
    if missing:
        errors.append(f"spec table: missing {len(missing)} job row(s): {missing}")
    unexpected = sorted(table - workflow)
    if unexpected:
        errors.append(
            f"spec table: {len(unexpected)} row(s) naming nonexistent job(s): "
            f"{unexpected}"
        )
    return errors


def main() -> int:
    """Run the parity check against the repository and report the result."""
    errors = run_checks(CI_YML, SPEC_MD)
    for error in errors:
        print(f"error: {error}")
    if errors:
        return 1
    count = len(ci_jobs(CI_YML))
    print(f"OK: {count} CI jobs consistent between ci.yml and specs/ci-cd.md.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
