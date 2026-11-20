"""Parity test: the Ruby docs name the Ruby series that ``release.yml`` precompiles gems for.

The cross-gem step in ``.github/workflows/release.yml`` lists the Ruby series that get a
precompiled platform gem (``ruby-versions``). Both ``docs/howto/ruby.md`` and
``crates/iscc-rb/README.md`` must state that range as ``Ruby {min} to {max}``, so a change
to the release matrix fails here until the docs follow. No network, no mocks.
"""

import re
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[1]
RELEASE_WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"
RUBY_DOCS = [
    ROOT / "docs" / "howto" / "ruby.md",
    ROOT / "crates" / "iscc-rb" / "README.md",
]
CROSS_GEM_ACTION = "oxidize-rb/actions/cross-gem"


def cross_gem_ruby_series(workflow):
    """Return the Ruby series listed in the workflow's cross-gem step, in file order.

    Raises ``ValueError`` when no cross-gem step exists or its list parses to nothing.
    """
    for job in (workflow.get("jobs") or {}).values():
        for step in job.get("steps") or []:
            if str(step.get("uses", "")).startswith(CROSS_GEM_ACTION):
                raw = str((step.get("with") or {}).get("ruby-versions", ""))
                series = [item.strip() for item in raw.split(",") if item.strip()]
                if not series:
                    raise ValueError(f"cross-gem step has no ruby-versions: {raw!r}")
                return series
    raise ValueError(f"no step uses {CROSS_GEM_ACTION}")


def version_key(series):
    """Return a sortable integer tuple for a Ruby series string like ``3.10``."""
    return tuple(int(part) for part in series.split("."))


def ruby_range_phrase(series):
    """Return the ``Ruby {min} to {max}`` phrase for a list of Ruby series."""
    ordered = sorted(series, key=version_key)
    return f"Ruby {ordered[0]} to {ordered[-1]}"


def normalise_whitespace(text):
    """Collapse every whitespace run to one space (mdformat reflows prose at 100 columns)."""
    return re.sub(r"\s+", " ", text)


def doc_names_range(text, phrase):
    """Return True if the doc text contains the range phrase, ignoring line wrapping."""
    return phrase in normalise_whitespace(text)


def load_release_workflow():
    """Load ``release.yml`` as a dict."""
    return yaml.safe_load(RELEASE_WORKFLOW.read_text(encoding="utf-8"))


def test_release_workflow_lists_ruby_series():
    """The real cross-gem step lists at least one parseable Ruby series."""
    series = cross_gem_ruby_series(load_release_workflow())
    assert len(series) >= 1
    assert all(version_key(item) for item in series)


@pytest.mark.parametrize("doc", RUBY_DOCS, ids=lambda p: p.relative_to(ROOT).as_posix())
def test_ruby_doc_states_precompiled_range(doc):
    """Each Ruby doc states the precompiled range derived from release.yml."""
    phrase = ruby_range_phrase(cross_gem_ruby_series(load_release_workflow()))
    assert doc_names_range(doc.read_text(encoding="utf-8"), phrase), (
        f"{doc.relative_to(ROOT)} must state {phrase!r}"
    )


def test_range_phrase_sorts_as_versions():
    """The range sorts numerically, so 3.10 ranks above 3.9."""
    assert ruby_range_phrase(["3.10", "3.9", "3.1"]) == "Ruby 3.1 to 3.10"


def test_range_phrase_from_float_list():
    """A single unquoted YAML series (a float) still yields a range."""
    workflow = {
        "jobs": {
            "gem": {
                "steps": [
                    {"uses": f"{CROSS_GEM_ACTION}@v1", "with": {"ruby-versions": 3.3}}
                ]
            }
        }
    }
    assert ruby_range_phrase(cross_gem_ruby_series(workflow)) == "Ruby 3.3 to 3.3"


def test_matching_matches_wrapped_phrase():
    """A phrase split across lines by mdformat still matches."""
    assert doc_names_range("built for Ruby\n3.1 to 3.3 on common", "Ruby 3.1 to 3.3")


def test_matching_rejects_other_range():
    """A doc naming a different range fails the match."""
    assert not doc_names_range(
        "Precompiled gems are built for Ruby 3.1 to 3.4.", "Ruby 3.1 to 3.3"
    )


def test_missing_cross_gem_step_raises():
    """A workflow without a cross-gem step is an error, not an empty range."""
    workflow = {"jobs": {"build": {"steps": [{"uses": "actions/checkout@v6"}]}}}
    with pytest.raises(ValueError, match="no step uses"):
        cross_gem_ruby_series(workflow)


def test_empty_ruby_versions_raises():
    """A cross-gem step with an empty ruby-versions list is an error."""
    workflow = {
        "jobs": {"gem": {"steps": [{"uses": f"{CROSS_GEM_ACTION}@v1", "with": {}}]}}
    }
    with pytest.raises(ValueError, match="no ruby-versions"):
        cross_gem_ruby_series(workflow)
