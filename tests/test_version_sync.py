"""Tests for ``scripts/version_sync.py``.

Every getter reports each version occurrence its sync function rewrites. These tests cover the napi
lockfile (two root versions, rewritten together, byte-identical no-op), the multi-site getters (docs
snippets, ``mise.toml``, ``test_install.py``), and ``check_mode``/``sync_mode`` on synthetic trees
(``vs.ROOT`` and ``vs.TARGETS`` monkeypatched): each fails on exactly one stale occurrence, missing
target or unmatched pattern. The real repository tree must pass ``--check``.
"""

import importlib.util
import json
from pathlib import Path

import pytest

# Load scripts/version_sync.py by path — it is a repo script, not a package.
_SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "version_sync.py"
_spec = importlib.util.spec_from_file_location("version_sync", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
vs = importlib.util.module_from_spec(_spec)
_loader.exec_module(vs)

LOCK_REL = "crates/iscc-napi/package-lock.json"
LOCK_PATH = vs.ROOT / LOCK_REL


def _lock_text():
    """Return the committed napi lockfile text."""
    return LOCK_PATH.read_text(encoding="utf-8")


def _lock_with(top, root):
    """Return lockfile text with the given top-level and packages[""] versions."""
    data = json.loads(_lock_text())
    data["version"] = top
    data["packages"][""]["version"] = root
    return json.dumps(data, indent=2) + "\n"


def test_lock_is_registered_target():
    """The napi lockfile is a TARGETS entry wired to the lockfile helpers."""
    entries = [t for t in vs.TARGETS if t[0] == LOCK_REL]
    assert entries == [(LOCK_REL, vs._get_package_lock_version, vs._sync_package_lock)]


def test_sync_rewrites_both_fields():
    """Syncing a stale lock updates the top-level and packages[""] versions."""
    synced = vs._sync_package_lock(_lock_with("0.0.1", "0.0.1"), "9.8.7")
    data = json.loads(synced)
    assert data["version"] == "9.8.7"
    assert data["packages"][""]["version"] == "9.8.7"
    assert vs._get_package_lock_version(synced) == ["9.8.7", "9.8.7"]


def test_get_reports_stale_root_package():
    """A lock whose packages[""] version alone is stale reports that stale version."""
    assert vs._get_package_lock_version(_lock_with("9.8.7", "0.0.1")) == [
        "9.8.7",
        "0.0.1",
    ]


def test_get_reports_stale_top_level():
    """A lock whose top-level version alone is stale reports that stale version."""
    assert vs._get_package_lock_version(_lock_with("0.0.1", "9.8.7")) == [
        "0.0.1",
        "9.8.7",
    ]


def test_sync_keeps_current_lock_byte_identical():
    """Syncing the committed lock to its own version reproduces npm's exact bytes."""
    text = _lock_text()
    version = vs._get_package_lock_version(text)[0]
    assert vs._sync_package_lock(text, version) == text


def test_real_tree_passes_check(capsys):
    """Every version target in the repository matches the workspace version."""
    assert vs.check_mode(vs.read_workspace_version())
    assert f"OK: {LOCK_REL}" in capsys.readouterr().out


WS = "9.8.7"
STALE = "0.0.1"

DOC_TEMPLATE = """# Install

```xml
<dependency>
  <groupId>io.iscc</groupId>
  <artifactId>iscc-lib</artifactId>
  <version>{maven}</version>
</dependency>
```

```swift
.package(url: "https://github.com/iscc/iscc-lib", from: "{spm}"),
```
"""

MISE_TEMPLATE = """[tasks."test:install"]
arg "[flags]" help="Flags forwarded to test_install.py (default: --version {first})"
run = 'uv run scripts/test_install.py ${{usage_flags:---version {second}}}'
"""


def _tree(tmp_path, monkeypatch, files, targets):
    """Write `files` (rel path -> text) under tmp_path and point vs.ROOT/vs.TARGETS at them."""
    for rel, text in files.items():
        path = tmp_path / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    monkeypatch.setattr(vs, "ROOT", tmp_path)
    monkeypatch.setattr(vs, "TARGETS", targets)


def _doc_target(rel="doc.md"):
    """Return a TARGETS entry for a docs snippet file."""
    return (rel, vs._get_doc_dep_version, vs._sync_doc_deps)


def _mise_target():
    """Return the TARGETS entry for mise.toml."""
    return ("mise.toml", vs._get_mise_version, vs._sync_mise)


def test_doc_getter_reports_every_snippet():
    """The docs getter returns the Maven, Gradle and every SPM version of docs/index.md."""
    text = (vs.ROOT / "docs/index.md").read_text(encoding="utf-8")
    assert len(vs._get_doc_dep_version(text)) == 4


def test_mise_getter_reports_every_flag():
    """The mise getter returns both --version defaults, not only the first."""
    assert vs._get_mise_version(MISE_TEMPLATE.format(first=WS, second=STALE)) == [
        WS,
        STALE,
    ]


def test_test_install_getter_reports_docstring_flag():
    """The test_install getter returns the fallback and the docstring --version."""
    text = f'"""Usage: test_install.py --version {STALE}"""\nv = version or "{WS}"\n'
    assert vs._get_test_install_version(text) == [WS, STALE]


def test_check_passes_current_synthetic_tree(tmp_path, monkeypatch, capsys):
    """A synthetic tree whose every occurrence is current passes with one OK line per target."""
    files = {
        "doc.md": DOC_TEMPLATE.format(maven=WS, spm=WS),
        "mise.toml": MISE_TEMPLATE.format(first=WS, second=WS),
    }
    _tree(tmp_path, monkeypatch, files, [_doc_target(), _mise_target()])
    assert vs.check_mode(WS)
    assert capsys.readouterr().out.count("OK: ") == 2


def test_check_fails_on_stale_second_doc_snippet(tmp_path, monkeypatch):
    """A doc with a current Maven snippet and a stale SPM snippet fails the check."""
    _tree(
        tmp_path,
        monkeypatch,
        {"doc.md": DOC_TEMPLATE.format(maven=WS, spm=STALE)},
        [_doc_target()],
    )
    assert not vs.check_mode(WS)


def test_check_fails_on_missing_target(tmp_path, monkeypatch, capsys):
    """A registered target file that does not exist fails the check."""
    _tree(tmp_path, monkeypatch, {}, [_doc_target("gone.md")])
    assert not vs.check_mode(WS)
    assert "MISMATCH: gone.md missing" in capsys.readouterr().out


def test_check_fails_on_unmatched_pattern(tmp_path, monkeypatch, capsys):
    """A target file in which the getter finds no version fails the check."""
    _tree(
        tmp_path,
        monkeypatch,
        {"doc.md": "# No install snippet here\n"},
        [_doc_target()],
    )
    assert not vs.check_mode(WS)
    assert "MISMATCH: doc.md pattern not found" in capsys.readouterr().out


def test_check_fails_on_stale_second_mise_flag(tmp_path, monkeypatch):
    """A mise.toml whose second --version default is stale fails the check."""
    _tree(
        tmp_path,
        monkeypatch,
        {"mise.toml": MISE_TEMPLATE.format(first=WS, second=STALE)},
        [_mise_target()],
    )
    assert not vs.check_mode(WS)


def test_sync_rewrites_stale_second_doc_snippet(tmp_path, monkeypatch):
    """Syncing a doc with a current first and a stale second snippet leaves only the workspace version."""
    _tree(
        tmp_path,
        monkeypatch,
        {"doc.md": DOC_TEMPLATE.format(maven=WS, spm=STALE)},
        [_doc_target()],
    )
    assert vs.sync_mode(WS)
    text = (tmp_path / "doc.md").read_text(encoding="utf-8")
    assert text == DOC_TEMPLATE.format(maven=WS, spm=WS)
    assert vs._get_doc_dep_version(text) == [WS, WS]


def test_sync_keeps_current_file_byte_identical(tmp_path, monkeypatch):
    """Syncing a current file does not rewrite it."""
    text = DOC_TEMPLATE.format(maven=WS, spm=WS)
    _tree(tmp_path, monkeypatch, {"doc.md": text}, [_doc_target()])
    mtime = (tmp_path / "doc.md").stat().st_mtime_ns
    assert vs.sync_mode(WS)
    assert (tmp_path / "doc.md").read_text(encoding="utf-8") == text
    assert (tmp_path / "doc.md").stat().st_mtime_ns == mtime


def test_sync_fails_on_missing_target(tmp_path, monkeypatch, capsys):
    """Syncing a tree with a missing target reports it and returns False."""
    _tree(tmp_path, monkeypatch, {}, [_doc_target("gone.md")])
    assert not vs.sync_mode(WS)
    assert "FAILED: gone.md missing" in capsys.readouterr().out


def test_sync_fails_on_unmatched_pattern(tmp_path, monkeypatch, capsys):
    """Syncing a target whose pattern is not found reports it, leaves it alone and returns False."""
    _tree(tmp_path, monkeypatch, {"doc.md": "no snippet\n"}, [_doc_target()])
    assert not vs.sync_mode(WS)
    assert "FAILED: doc.md pattern not found" in capsys.readouterr().out
    assert (tmp_path / "doc.md").read_text(encoding="utf-8") == "no snippet\n"


def test_main_exits_1_when_sync_fails(tmp_path, monkeypatch):
    """main() without --check exits 1 when a sync target is missing."""
    _tree(tmp_path, monkeypatch, {}, [_doc_target("gone.md")])
    monkeypatch.setattr(vs, "read_workspace_version", lambda: WS)
    monkeypatch.setattr(vs.sys, "argv", ["version_sync.py"])
    with pytest.raises(SystemExit) as exc:
        vs.main()
    assert exc.value.code == 1
