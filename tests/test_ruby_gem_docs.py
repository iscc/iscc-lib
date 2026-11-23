"""Parity test: the Ruby docs name the Ruby series that ``release.yml`` precompiles gems for.

The cross-gem step in ``.github/workflows/release.yml`` lists the Ruby series that get a
precompiled platform gem (``ruby-versions``). Both ``docs/howto/ruby.md`` and
``crates/iscc-rb/README.md`` must state that range as ``Ruby {min} to {max}``, so a change
to the release matrix fails here until the docs follow.

The Ruby floor must also agree in three places: the gemspec ``required_ruby_version``, the
``ruby-version`` the ``ci.yml`` ``ruby`` job tests, and the lowest series ``release.yml``
precompiles.

The development Ruby is pinned once, in the root ``mise.toml`` ``[tools]`` table, at or above
the gemspec floor; the standardrb hooks and the development docs use that pin and name no
``ruby@<version>`` of their own. No network, no mocks.
"""

import re
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[1]
RELEASE_WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"
CI_WORKFLOW = ROOT / ".github" / "workflows" / "ci.yml"
GEMSPEC = ROOT / "crates" / "iscc-rb" / "iscc-lib.gemspec"
MISE_TOML = ROOT / "mise.toml"
DEV_RUBY_FILES = [
    ROOT / ".pre-commit-config.yaml",
    ROOT / "crates" / "iscc-rb" / "CLAUDE.md",
    ROOT / ".claude" / "context" / "specs" / "ruby-bindings.md",
]
INLINE_RUBY_PIN_RE = re.compile(r"ruby@\d[\w.]*")
# Regex parsing instead of tomllib: CI's python-test matrix includes 3.10.
MISE_TOOLS_TABLE_RE = re.compile(
    r"^\[tools\][ \t]*$(.*?)(?=^\[|\Z)", re.MULTILINE | re.DOTALL
)
MISE_RUBY_PIN_RE = re.compile(r"""^ruby[ \t]*=[ \t]*["']([^"'\s]+)["']""", re.MULTILINE)
RUBY_DOCS = [
    ROOT / "docs" / "howto" / "ruby.md",
    ROOT / "crates" / "iscc-rb" / "README.md",
]
CROSS_GEM_ACTION = "oxidize-rb/actions/cross-gem"
SETUP_RUBY_ACTION = "ruby/setup-ruby"
GEMSPEC_FLOOR_RE = re.compile(
    r"""^\s*spec\.required_ruby_version\s*=\s*["']>=\s*(\d+)\.(\d+)(?:\.\d+)?["']""",
    re.MULTILINE,
)


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


def gemspec_floor(text):
    """Return the ``major.minor`` floor of the gemspec ``required_ruby_version``.

    Raises ``ValueError`` when the gemspec declares no ``>=`` floor.
    """
    match = GEMSPEC_FLOOR_RE.search(text)
    if match is None:
        raise ValueError("gemspec declares no required_ruby_version >= floor")
    return f"{match.group(1)}.{match.group(2)}"


def ci_ruby_job_version(workflow):
    """Return the ``ruby-version`` the ``ruby`` job of ``ci.yml`` passes to setup-ruby.

    Raises ``ValueError`` when the job or its setup-ruby version is missing.
    """
    job = (workflow.get("jobs") or {}).get("ruby")
    if job is None:
        raise ValueError("ci.yml has no ruby job")
    for step in job.get("steps") or []:
        if str(step.get("uses", "")).startswith(SETUP_RUBY_ACTION):
            version = str((step.get("with") or {}).get("ruby-version", "")).strip()
            if version:
                return version
    raise ValueError("ci.yml ruby job sets no setup-ruby ruby-version")


def check_floor_parity(gemspec, ci, release):
    """Raise ``ValueError`` unless the gemspec, CI and release floors are the same series."""
    floors = {
        "gemspec": gemspec,
        "ci.yml ruby job": ci,
        "release.yml cross-gem": release,
    }
    if len({version_key(value) for value in floors.values()}) != 1:
        raise ValueError(f"Ruby floor mismatch: {floors}")


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


def test_ruby_floor_parity():
    """The gemspec floor, the CI-tested Ruby and the lowest precompiled series agree."""
    gemspec = gemspec_floor(GEMSPEC.read_text(encoding="utf-8"))
    ci = ci_ruby_job_version(yaml.safe_load(CI_WORKFLOW.read_text(encoding="utf-8")))
    release = min(cross_gem_ruby_series(load_release_workflow()), key=version_key)
    check_floor_parity(gemspec, ci, release)


def test_floor_parity_accepts_equal_trio():
    """Three equal series pass the parity check."""
    check_floor_parity("3.3", "3.3", "3.3")


def test_floor_parity_rejects_gemspec_mismatch():
    """A gemspec floor below the CI and release floors fails."""
    with pytest.raises(ValueError, match="Ruby floor mismatch"):
        check_floor_parity("3.1", "3.3", "3.3")


def test_floor_parity_rejects_ci_mismatch():
    """A CI-tested Ruby other than the floor fails."""
    with pytest.raises(ValueError, match="Ruby floor mismatch"):
        check_floor_parity("3.3", "3.4", "3.3")


def test_floor_parity_rejects_release_mismatch():
    """A lowest precompiled series other than the floor fails."""
    with pytest.raises(ValueError, match="Ruby floor mismatch"):
        check_floor_parity("3.3", "3.3", "3.1")


def test_gemspec_floor_reads_major_minor():
    """The gemspec floor drops the patch level."""
    assert gemspec_floor('  spec.required_ruby_version = ">= 3.3.0"\n') == "3.3"


def test_gemspec_floor_missing_raises():
    """A gemspec without a floor is an error, not a pass."""
    with pytest.raises(ValueError, match="no required_ruby_version"):
        gemspec_floor('  spec.name = "iscc-lib"\n')


def test_ci_ruby_job_without_version_raises():
    """A ruby job whose setup-ruby step names no version is an error."""
    workflow = {"jobs": {"ruby": {"steps": [{"uses": f"{SETUP_RUBY_ACTION}@v1"}]}}}
    with pytest.raises(ValueError, match="no setup-ruby ruby-version"):
        ci_ruby_job_version(workflow)


def test_ci_without_ruby_job_raises():
    """A ci.yml without a ruby job is an error."""
    with pytest.raises(ValueError, match="no ruby job"):
        ci_ruby_job_version({"jobs": {"go": {"steps": []}}})


def mise_ruby_pin(text):
    """Return the ``[tools]`` ``ruby`` version from ``mise.toml`` text.

    Raises ``ValueError`` when ``mise.toml`` pins no Ruby.
    """
    table = MISE_TOOLS_TABLE_RE.search(text)
    pin = MISE_RUBY_PIN_RE.search(table.group(1)) if table else None
    if pin is None:
        raise ValueError("mise.toml [tools] pins no ruby")
    return pin.group(1)


def check_dev_ruby_pin(pin, floor):
    """Raise ``ValueError`` unless the development Ruby pin is at or above the floor series."""
    series = ".".join(pin.split(".")[:2])
    if version_key(series) < version_key(floor):
        raise ValueError(f"mise.toml ruby {pin} is below the gem floor {floor}")


def inline_ruby_pins(text):
    """Return every ``ruby@<version>`` spelling in a text, in order."""
    return INLINE_RUBY_PIN_RE.findall(text)


def test_mise_pins_dev_ruby_at_or_above_floor():
    """The root mise.toml pins a development Ruby no lower than the gemspec floor."""
    pin = mise_ruby_pin(MISE_TOML.read_text(encoding="utf-8"))
    check_dev_ruby_pin(pin, gemspec_floor(GEMSPEC.read_text(encoding="utf-8")))


@pytest.mark.parametrize("path", DEV_RUBY_FILES, ids=lambda p: p.name)
def test_dev_ruby_files_use_the_mise_pin(path):
    """The hooks and development docs name no Ruby version of their own."""
    assert inline_ruby_pins(path.read_text(encoding="utf-8")) == []


def test_inline_ruby_pin_in_hook_is_found():
    """A hook entry that pins its own Ruby is caught."""
    entry = "entry: bash -c 'cd crates/iscc-rb && mise exec ruby@3.4 -- bundle exec standardrb'"
    assert inline_ruby_pins(entry) == ["ruby@3.4"]


def test_mise_without_ruby_pin_raises():
    """A mise.toml without a Ruby pin is an error, not a pass."""
    with pytest.raises(ValueError, match="pins no ruby"):
        mise_ruby_pin('[tasks.check]\nrun = "true"\n')


def test_mise_ruby_outside_tools_table_raises():
    """A ruby key in another table is not the development pin."""
    with pytest.raises(ValueError, match="pins no ruby"):
        mise_ruby_pin('[tools]\nnode = "22"\n\n[env]\nruby = "3.4.13"\n')


def test_mise_ruby_pin_reads_tools_table():
    """The pin is read from the [tools] table, whatever follows it."""
    assert (
        mise_ruby_pin('[tools]\nruby = "3.4.13"\n\n[tasks.check]\nrun = "x"\n')
        == "3.4.13"
    )


def test_dev_ruby_pin_below_floor_raises():
    """A development Ruby below the gem floor fails."""
    with pytest.raises(ValueError, match="below the gem floor"):
        check_dev_ruby_pin("3.2.9", "3.3")


def test_dev_ruby_pin_compares_as_versions():
    """Ruby 3.10 is above a 3.9 floor (numeric, not string, comparison)."""
    check_dev_ruby_pin("3.10.0", "3.9")
