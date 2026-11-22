"""Tests for the napi lockfile target in ``scripts/version_sync.py``.

The npm lockfile repeats the package version in two places: the top-level ``version`` and
``packages[""].version``. These tests cover that the sync helper rewrites both, that the get helper
reports a mismatch when only one is stale, that syncing a current lock leaves it byte-identical, and
that the real repository tree passes ``--check``. Fixtures are built from the committed lockfile.
"""

import importlib.util
import json
from pathlib import Path

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
    assert vs._get_package_lock_version(synced) == "9.8.7"


def test_get_reports_stale_root_package():
    """A lock whose packages[""] version alone is stale does not report the workspace version."""
    found = vs._get_package_lock_version(_lock_with("9.8.7", "0.0.1"))
    assert found != "9.8.7"


def test_get_reports_stale_top_level():
    """A lock whose top-level version alone is stale does not report the workspace version."""
    found = vs._get_package_lock_version(_lock_with("0.0.1", "9.8.7"))
    assert found != "9.8.7"


def test_sync_keeps_current_lock_byte_identical():
    """Syncing the committed lock to its own version reproduces npm's exact bytes."""
    text = _lock_text()
    version = vs._get_package_lock_version(text)
    assert vs._sync_package_lock(text, version) == text


def test_real_tree_passes_check(capsys):
    """Every version target in the repository matches the workspace version."""
    assert vs.check_mode(vs.read_workspace_version())
    assert f"OK: {LOCK_REL}" in capsys.readouterr().out
