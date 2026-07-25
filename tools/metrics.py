"""Codebase-health metrics snapshotter for the CID loop.

Appends one JSON line per run to ``.claude/context/metrics.jsonl`` so the audit
role (and humans) can read trend lines instead of point-in-time impressions —
rot is invisible per-step and obvious as a slope. Stdlib-only by design: no new
tool dependencies, deterministic output, cross-platform.

Counted per component (a crate, package, or top-level dir) and in total:
files, non-blank LOC, function definitions, test definitions, lint
suppressions, ``unsafe`` uses, TODO/FIXME markers, and skipped tests. With
``--time-gates``, additionally times the read-only quality gates (cargo test,
clippy, pytest) — gate latency is itself a rot metric: unchecked, the loop
slows to a grind.

Usage: ``uv run tools/metrics.py [--time-gates]`` (or via ``mise run cid:metrics``).
"""

import argparse
import json
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

METRICS_FILE = Path(".claude/context/metrics.jsonl")

# Directories scanned for source files, relative to the repo root.
SCAN_ROOTS = ("crates", "packages", "tools", "tests")

# Directory names never descended into (build output, vendored code, caches).
EXCLUDE_DIRS = frozenset(
    {
        ".git",
        ".gradle",
        "__pycache__",
        "bin",
        "build",
        "dist",
        "node_modules",
        "obj",
        "target",
        "vendor",
    }
)

# File extensions considered source code, mapped to a language label.
EXT_LANG = {
    ".c": "c",
    ".cs": "csharp",
    ".go": "go",
    ".h": "c",
    ".java": "java",
    ".js": "javascript",
    ".kt": "kotlin",
    ".kts": "kotlin",
    ".py": "python",
    ".rb": "ruby",
    ".rs": "rust",
    ".swift": "swift",
    ".ts": "typescript",
}

# Substring markers counted per line. Deliberately simple: these are trend
# signals for the audit role, not precise static analysis.
SUPPRESSION_MARKERS = (
    "#[allow(",
    "#![allow(",
    "# noqa",
    "# type: ignore",
    "eslint-disable",
    "@SuppressWarnings",
    "#pragma warning disable",
)
UNSAFE_MARKERS = ("unsafe {", "unsafe fn", "unsafe impl", "unsafe trait")
TODO_MARKERS = ("TODO", "FIXME", "XXX")
SKIP_MARKERS = ("#[ignore]", "pytest.mark.skip", "@Disabled", "@Ignore")
FN_MARKERS = ("fn ", "def ", "func ")
TEST_MARKERS = ("#[test]", "#[tokio::test]", "def test_", "func Test", "@Test")

METRIC_KEYS = (
    "files",
    "loc",
    "fns",
    "tests",
    "suppressions",
    "unsafe",
    "todos",
    "skips",
)

# Read-only quality gates timed by --time-gates, matching how the loop runs
# them. Deliberately excludes `mise run check`: its auto-fix hooks can mutate
# the working tree, which a metrics run must never do.
GATE_COMMANDS = (
    ("cargo_test", ("cargo", "test", "-p", "iscc-lib", "--quiet")),
    ("clippy", ("cargo", "clippy", "--workspace", "--", "-D", "warnings")),
    ("pytest", ("uv", "run", "pytest", "-q", "--timeout=120")),
)

# Ceiling per timed gate so a hung command cannot stall a metrics run.
GATE_TIMEOUT_S = 900


def _count_markers(line, markers):
    """Return 1 if any marker appears in the line, else 0."""
    return 1 if any(m in line for m in markers) else 0


def scan_file(path):
    """Return a metrics dict for one source file (keys from METRIC_KEYS)."""
    counts = dict.fromkeys(METRIC_KEYS, 0)
    counts["files"] = 1
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return counts
    is_rust = path.suffix == ".rs"
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        counts["loc"] += 1
        counts["fns"] += _count_markers(stripped, FN_MARKERS)
        counts["tests"] += _count_markers(stripped, TEST_MARKERS)
        counts["suppressions"] += _count_markers(stripped, SUPPRESSION_MARKERS)
        counts["todos"] += _count_markers(stripped, TODO_MARKERS)
        counts["skips"] += _count_markers(stripped, SKIP_MARKERS)
        if is_rust:
            counts["unsafe"] += _count_markers(stripped, UNSAFE_MARKERS)
    return counts


def _component_key(root, rel_path):
    """Map a file's relative path to its component (e.g. crates/iscc-lib)."""
    parts = rel_path.parts
    if root in ("crates", "packages") and len(parts) >= 2:
        return f"{parts[0]}/{parts[1]}"
    return root


def iter_source_files(repo_root):
    """Yield (component, path) for every source file under the scan roots."""
    for root in SCAN_ROOTS:
        base = repo_root / root
        if not base.is_dir():
            continue
        for path in sorted(base.rglob("*")):
            if not path.is_file() or path.suffix not in EXT_LANG:
                continue
            rel = path.relative_to(repo_root)
            if any(part in EXCLUDE_DIRS for part in rel.parts):
                continue
            yield _component_key(root, rel), path


def collect_metrics(repo_root):
    """Aggregate per-component and total metrics for the repository."""
    components = {}
    totals = dict.fromkeys(METRIC_KEYS, 0)
    for component, path in iter_source_files(repo_root):
        counts = scan_file(path)
        comp = components.setdefault(component, dict.fromkeys(METRIC_KEYS, 0))
        for key in METRIC_KEYS:
            comp[key] += counts[key]
            totals[key] += counts[key]
    return {"totals": totals, "components": components}


def git_sha(repo_root):
    """Return the short HEAD sha, or None if git is unavailable."""
    try:
        # Fixed argv, no shell, trusted local git — safe subprocess use.
        out = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],  # noqa: S607
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError:
        return None
    return out.stdout.strip() or None


def time_gate(repo_root, argv):
    """Time one gate command; return {seconds, ok} (ok None if it can't run)."""
    start = time.monotonic()
    try:
        # Fixed argv from GATE_COMMANDS, no shell — safe subprocess use.
        proc = subprocess.run(  # noqa: S603
            list(argv),
            cwd=repo_root,
            capture_output=True,
            timeout=GATE_TIMEOUT_S,
            check=False,
        )
        ok = proc.returncode == 0
    except (OSError, subprocess.TimeoutExpired):
        ok = None  # tool missing or hung — recorded, never fatal
    return {"seconds": round(time.monotonic() - start, 1), "ok": ok}


def time_gates(repo_root):
    """Time every gate in GATE_COMMANDS; return {name: {seconds, ok}}."""
    return {name: time_gate(repo_root, argv) for name, argv in GATE_COMMANDS}


def snapshot(repo_root, include_gates=False):
    """Build one complete metrics snapshot dict for the repository."""
    data = collect_metrics(repo_root)
    snap = {
        "ts": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "git_sha": git_sha(repo_root),
        **data,
    }
    if include_gates:
        snap["gates"] = time_gates(repo_root)
    return snap


def append_snapshot(repo_root, snap):
    """Append the snapshot as one JSON line to the metrics log."""
    log_path = repo_root / METRICS_FILE
    log_path.parent.mkdir(parents=True, exist_ok=True)
    with log_path.open("a", encoding="utf-8") as f:
        f.write(json.dumps(snap, separators=(",", ":"), sort_keys=True) + "\n")


def main():
    """Snapshot the current working directory and append it to the log."""
    parser = argparse.ArgumentParser(description="Append a codebase-health snapshot")
    parser.add_argument(
        "--time-gates",
        action="store_true",
        default=False,
        help="Also time the read-only quality gates (adds minutes)",
    )
    args = parser.parse_args()
    repo_root = Path.cwd()
    snap = snapshot(repo_root, include_gates=args.time_gates)
    append_snapshot(repo_root, snap)
    t = snap["totals"]
    gates = "".join(
        f", {name} {g['seconds']}s" for name, g in snap.get("gates", {}).items()
    )
    print(
        f"metrics @ {snap['git_sha'] or '?'}: {t['files']} files, {t['loc']} loc, "
        f"{t['fns']} fns, {t['tests']} tests, {t['suppressions']} suppressions, "
        f"{t['unsafe']} unsafe, {t['todos']} todos, {t['skips']} skips"
        f"{gates} -> {METRICS_FILE}"
    )


if __name__ == "__main__":
    sys.exit(main())
