"""Tests for the codebase-health snapshotter in ``tools/metrics.py``.

Covers per-file marker counting, component mapping, build-dir exclusion,
snapshot/append behavior, and gate-timing resilience. The counters are trend
signals for the audit role, so the tests pin the counting rules rather than
chase static-analysis precision.
"""

import importlib.util
import json
import sys
from pathlib import Path

# Load tools/metrics.py by path — it is a script, not an installed package.
_METRICS_PATH = Path(__file__).resolve().parents[1] / "tools" / "metrics.py"
_spec = importlib.util.spec_from_file_location("metrics_tool", _METRICS_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
metrics = importlib.util.module_from_spec(_spec)
_loader.exec_module(metrics)


def _write(root, rel, text):
    """Write text to root/rel, creating parent directories."""
    path = root / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")
    return path


def test_scan_file_counts_markers(tmp_path):
    path = _write(
        tmp_path,
        "lib.rs",
        "pub fn one() {}\n"
        "#[test]\n"
        "fn test_helper() {}\n"
        "unsafe { touch() }\n"
        "// TODO: tighten\n"
        "#[allow(dead_code)]\n"
        "\n"
        "let x = 1;\n",
    )
    c = metrics.scan_file(path)
    assert c["files"] == 1
    assert c["loc"] == 7  # blank line excluded
    assert c["fns"] == 2
    assert c["tests"] == 1
    assert c["unsafe"] == 1
    assert c["todos"] == 1
    assert c["suppressions"] == 1
    assert c["skips"] == 0


def test_scan_file_unsafe_only_counted_for_rust(tmp_path):
    path = _write(tmp_path, "notes.py", "# unsafe { not rust }\n")
    assert metrics.scan_file(path)["unsafe"] == 0


def test_component_key_mapping():
    key = metrics._component_key("crates", Path("crates/iscc-lib/src/lib.rs"))
    assert key == "crates/iscc-lib"
    assert metrics._component_key("tools", Path("tools/cid.py")) == "tools"
    assert metrics._component_key("tests", Path("tests/test_algo.py")) == "tests"


def test_collect_metrics_excludes_build_dirs(tmp_path):
    _write(tmp_path, "crates/foo/src/lib.rs", "fn a() {}\n")
    _write(tmp_path, "crates/foo/target/debug/gen.rs", "fn junk() {}\n")
    _write(tmp_path, "packages/go/x.go", "func A() {}\n")
    _write(tmp_path, "packages/go/node_modules/dep/x.js", "function j() {}\n")
    _write(tmp_path, "crates/foo/src/data.json", "{}\n")  # not a source extension
    data = metrics.collect_metrics(tmp_path)
    assert set(data["components"]) == {"crates/foo", "packages/go"}
    assert data["totals"]["files"] == 2
    assert data["totals"]["fns"] == 2


def test_snapshot_and_append(tmp_path):
    _write(tmp_path, "crates/foo/src/lib.rs", "fn a() {}\n")
    snap = metrics.snapshot(tmp_path)
    assert "ts" in snap
    assert "git_sha" in snap  # None outside a git repo — key still present
    assert "gates" not in snap  # gate timing is opt-in
    metrics.append_snapshot(tmp_path, snap)
    metrics.append_snapshot(tmp_path, snap)
    lines = (tmp_path / metrics.METRICS_FILE).read_text(encoding="utf-8").splitlines()
    assert len(lines) == 2  # append-only, one JSON line per snapshot
    assert json.loads(lines[0])["totals"]["files"] == 1


def test_time_gate_ok_and_missing_tool(tmp_path):
    ok = metrics.time_gate(tmp_path, (sys.executable, "-c", "pass"))
    assert ok["ok"] is True
    assert ok["seconds"] >= 0
    failing = metrics.time_gate(tmp_path, (sys.executable, "-c", "raise SystemExit(1)"))
    assert failing["ok"] is False
    missing = metrics.time_gate(tmp_path, ("definitely-not-a-real-tool-xyz",))
    assert missing["ok"] is None  # tool missing is recorded, never fatal
