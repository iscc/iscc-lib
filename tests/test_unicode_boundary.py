"""Unicode 16.0.0 boundary conformance tests for the Python binding.

Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json)
against text_clean and text_collapse, gating the declared-Unicode-version freeze
rule outside the Rust crate: code points unassigned in Unicode 16.0.0 are mapped
to the noncharacter sentinel U+FFFF before normalization and removed by the
unchanged category-C filter, while assigned code points flow through the regular
pipeline. The sequence vectors additionally pin the sentinel map against the
superseded delete-filter design.
"""

import json
from pathlib import Path

import pytest

from iscc_lib import text_clean, text_collapse

FIXTURE = (
    Path(__file__).parent.parent
    / "crates"
    / "iscc-lib"
    / "tests"
    / "unicode_boundary.json"
)


def load_cases(section):
    """Load boundary vectors for a fixture section as pytest params."""
    data = json.loads(FIXTURE.read_text(encoding="utf-8"))
    return [pytest.param(tc, id=name) for name, tc in data[section].items()]


def test_boundary_fixture_metadata():
    """Guard against a truncated or renamed fixture silently collecting zero cases."""
    data = json.loads(FIXTURE.read_text(encoding="utf-8"))
    assert data["_metadata"]["unicode_data_version"] == "16.0.0"
    assert len(data["text_clean"]) == 7
    assert len(data["text_collapse"]) == 5


@pytest.mark.parametrize("tc", load_cases("text_clean"))
def test_text_clean_boundary(tc):
    """Verify text_clean matches every Unicode 16.0 boundary vector."""
    assert text_clean(*tc["inputs"]) == tc["outputs"]["result"]


@pytest.mark.parametrize("tc", load_cases("text_collapse"))
def test_text_collapse_boundary(tc):
    """Verify text_collapse matches every Unicode 16.0 boundary vector."""
    assert text_collapse(*tc["inputs"]) == tc["outputs"]["result"]
