"""Tests for the docs page-list parity gate in ``scripts/check_docs_nav.py``.

The anchor test runs the checks against the real repository tree and asserts zero
errors — this is what carries the gate into CI, since CI runs pytest but never prek.
The mutation tests build a small fixture tree in ``tmp_path`` (two pages plus an
``includes/`` partial) and drop or add a page in exactly one of the three hand-wired
lists, asserting the specific mismatch is reported. No network, no mocks beyond
fixture files.
"""

import importlib.util
from pathlib import Path

# Load scripts/check_docs_nav.py by path — it is a repo gate script, not a package.
_SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "check_docs_nav.py"
_spec = importlib.util.spec_from_file_location("check_docs_nav", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
cdn = importlib.util.module_from_spec(_spec)
_loader.exec_module(cdn)

# Fixture page sets: PAGES is the ground truth on disk; each mutation test removes
# or adds one entry in exactly one list.
PAGES = ["index.md", "howto/rust.md"]


def _write_fixtures(tmp_path, nav=PAGES, ordered=PAGES, llms=PAGES):
    """Build a docs tree plus the three lists in tmp_path; return check_docs_nav args.

    Disk always contains PAGES plus an ``includes/abbreviations.md`` partial that
    must not count as a page.
    """
    docs = tmp_path / "docs"
    for page in PAGES:
        path = docs / page
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(f"# {page}\n", encoding="utf-8")
    includes = docs / "includes" / "abbreviations.md"
    includes.parent.mkdir(parents=True, exist_ok=True)
    includes.write_text("*[ISCC]: International Standard Content Code\n")

    toml_path = tmp_path / "zensical.toml"
    nav_lines = "".join(f'  "{page}",\n' for page in nav)
    toml_path.write_text(f"nav = [\n{nav_lines}]\n", encoding="utf-8")

    script_path = tmp_path / "gen_llms_full.py"
    script_path.write_text(f"ORDERED_PAGES = {ordered!r}\n", encoding="utf-8")

    llms_path = docs / "llms.txt"
    llms_lines = "".join(
        f"- [{page}](https://lib.iscc.codes/{page}): desc\n" for page in llms
    )
    llms_path.write_text(llms_lines, encoding="utf-8")
    return docs, toml_path, script_path, llms_path


def test_real_repo_passes():
    # The gate must pass on the repository exactly as committed, and main() (the
    # prek hook entry point) must exit 0.
    assert (
        cdn.run_checks(cdn.DOCS_DIR, cdn.ZENSICAL_TOML, cdn.GEN_LLMS_FULL, cdn.LLMS_TXT)
        == []
    )
    assert cdn.main() == 0


def test_fixture_tree_passes(tmp_path):
    # A consistent fixture tree yields no errors; includes/ does not count as a page.
    assert cdn.run_checks(*_write_fixtures(tmp_path)) == []


def test_page_missing_from_nav_fires(tmp_path):
    # Dropping a page from the zensical.toml nav is reported for that list only.
    errors = cdn.run_checks(*_write_fixtures(tmp_path, nav=["index.md"]))
    assert errors == ["zensical.toml nav: missing 1 page(s): ['howto/rust.md']"]


def test_page_missing_from_ordered_pages_fires(tmp_path):
    # Dropping a page from ORDERED_PAGES is reported for that list only.
    errors = cdn.run_checks(*_write_fixtures(tmp_path, ordered=["index.md"]))
    assert errors == ["ORDERED_PAGES: missing 1 page(s): ['howto/rust.md']"]


def test_page_missing_from_llms_txt_fires(tmp_path):
    # Dropping a page from docs/llms.txt is reported for that list only.
    errors = cdn.run_checks(*_write_fixtures(tmp_path, llms=["index.md"]))
    assert errors == ["llms.txt: missing 1 page(s): ['howto/rust.md']"]


def test_main_exits_nonzero_on_missing_page(tmp_path, monkeypatch, capsys):
    # main() (the prek hook entry point) returns 1 and prints the mismatch when a
    # list is missing a page.
    docs, toml_path, script_path, llms_path = _write_fixtures(
        tmp_path, llms=["index.md"]
    )
    monkeypatch.setattr(cdn, "DOCS_DIR", docs)
    monkeypatch.setattr(cdn, "ZENSICAL_TOML", toml_path)
    monkeypatch.setattr(cdn, "GEN_LLMS_FULL", script_path)
    monkeypatch.setattr(cdn, "LLMS_TXT", llms_path)
    assert cdn.main() == 1
    assert "llms.txt: missing 1 page(s)" in capsys.readouterr().out


def test_page_absent_from_disk_fires(tmp_path):
    # A page linked in a list but absent from disk is reported as unexpected.
    errors = cdn.run_checks(*_write_fixtures(tmp_path, llms=[*PAGES, "ghost.md"]))
    assert errors == ["llms.txt: unexpected 1 page(s): ['ghost.md']"]


def test_includes_partial_is_not_a_page(tmp_path):
    # Listing includes/abbreviations.md in a list is an error: partials are not pages.
    errors = cdn.run_checks(
        *_write_fixtures(tmp_path, nav=[*PAGES, "includes/abbreviations.md"])
    )
    assert errors == [
        "zensical.toml nav: unexpected 1 page(s): ['includes/abbreviations.md']"
    ]
