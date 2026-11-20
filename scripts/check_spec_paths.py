"""Path-existence check for the target specs in `.claude/context/specs/*.md`.

The binding specs name repository paths in two forms: fenced structure trees
(`├── go.mod` under a `packages/go/` root line) and backticked spans
(`` `crates/iscc-ffi/build.rs` ``). Both have repeatedly drifted from the tree —
files renamed, moved, or never created. This gate resolves every such path against
the set of TRACKED paths and reports each one that does not resolve.

Ground truth is `git ls-files` (the index, so files staged in the same commit
count), plus every ancestor directory of a tracked file. The disk is NOT used:
untracked build output (napi `index.js`, wasm `pkg/`) exists locally but not on a
CI checkout, so a disk check would be green here and red in CI.

Rules:

- Tree entries live in fenced code blocks only. A root line is a fence line that
  ends in `/` and holds no whitespace (`packages/swift/`); it resets the path
  stack. An entry line is `[│ ]*` indentation, a `├── `/`└── ` connector, and a
  first token: depth = indentation // 4, path = root + the stack up to that depth.
  An entry with no root line before it in its fence is an error.
- Backticked paths match `` `(crates|packages|scripts|docs|tests)/…` `` anywhere
  in the file; a `:…` suffix (line number, symbol) and a trailing `/` are dropped.
- A path containing `*` resolves when `fnmatch` matches some tracked path.
- Exemptions live in two dicts below, each entry with its reason — there is no
  in-spec marker. `GENERATED` holds build/release output prefixes (children are
  exempt too); `NOT_PATHS` holds exact strings that look like paths but are not.

Floors: finding zero trees, zero tree entries, or zero backticked paths is an
error — a parse that silently matches nothing must not read as a pass.

Runs as a prek hook scoped to the specs and this script, and in CI via
`tests/test_check_spec_paths.py`.

Usage:
    uv run scripts/check_spec_paths.py
"""

from __future__ import annotations

import fnmatch
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC_DIR = ROOT / ".claude" / "context" / "specs"

# Build or release output: never tracked, legitimately named by the specs. A prefix
# exempts the path itself and everything below it.
GENERATED = {
    "crates/iscc-wasm/pkg": "wasm-pack output directory",
    "crates/iscc-napi/index.js": "napi-rs generated JS loader",
    "crates/iscc-napi/index.d.ts": "napi-rs generated TypeScript declarations",
    "packages/dotnet/Iscc.Lib/runtimes": "native libraries staged by release.yml",
    "packages/kotlin/src/main/resources": "JNA natives staged by release.yml",
}

# Exact strings that match the path pattern but do not name a repository path.
NOT_PATHS = {
    "packages/go/vX.Y.Z": "Go module git tag form, not a directory",
    "scripts/gen_ffi_boundary_vectors.py": "removed script, cited as history",
}

FENCE_RE = re.compile(r"^\s*```")
ENTRY_RE = re.compile(r"^([│ ]*)(?:├── |└── )(\S+)")
BACKTICK_RE = re.compile(r"`((?:crates|packages|scripts|docs|tests)/[^`\s]+)`")


def tracked_paths(root: Path) -> set[str]:
    """Return every tracked path under `root` plus all of its ancestor directories."""
    git = shutil.which("git") or sys.exit("error: git not found on PATH")
    # S603: the argv is fixed (`git ls-files -z`), no shell and no external input.
    out = subprocess.run(  # noqa: S603
        [git, "ls-files", "-z"],
        cwd=root,
        capture_output=True,
        check=True,
    ).stdout.decode("utf-8")
    known = set()
    for path in filter(None, out.split("\0")):
        parts = path.split("/")
        known.update("/".join(parts[:i]) for i in range(1, len(parts) + 1))
    return known


def is_root_line(line: str) -> bool:
    """Return True if a fenced line is a tree root such as `packages/go/`."""
    text = line.strip()
    return text.endswith("/") and not any(ch.isspace() for ch in text)


def is_generated(path: str) -> bool:
    """Return True if `path` equals or lies below a `GENERATED` prefix."""
    return any(path == p or path.startswith(p + "/") for p in GENERATED)


def resolves(path: str, known: set[str]) -> bool:
    """Return True if `path` is tracked, generated, exempt, or a matching glob."""
    if path in known or path in NOT_PATHS or is_generated(path):
        return True
    if "*" in path:
        return any(fnmatch.fnmatchcase(k, path) for k in known)
    return False


def tree_entries(text: str) -> tuple[int, list[tuple[int, str | None]]]:
    """Return (root count, [(line number, resolved path or None)]) for fenced trees.

    The path is None for an entry with no root line before it in its fence.
    """
    roots, entries = 0, []
    in_fence, root, stack = False, None, []
    for lineno, line in enumerate(text.splitlines(), start=1):
        if FENCE_RE.match(line):
            in_fence, root, stack = not in_fence, None, []
            continue
        if not in_fence:
            continue
        if is_root_line(line):
            roots, root, stack = roots + 1, line.strip().rstrip("/"), []
            continue
        match = ENTRY_RE.match(line)
        if match is None:
            continue
        depth = len(match.group(1)) // 4
        stack = [*stack[:depth], match.group(2).rstrip("/")]
        entries.append((lineno, None if root is None else "/".join([root, *stack])))
    return roots, entries


def backticked_paths(text: str) -> list[tuple[int, str]]:
    """Return (line number, path) for every backticked repository path."""
    found = []
    for lineno, line in enumerate(text.splitlines(), start=1):
        for span in BACKTICK_RE.findall(line):
            found.append((lineno, span.split(":", 1)[0].rstrip("/")))
    return found


def check_spec(spec: Path, known: set[str]) -> tuple[list[str], tuple[int, int, int]]:
    """Check one spec file; return (errors, (roots, tree entries, backticked paths))."""
    text = spec.read_text(encoding="utf-8")
    roots, entries = tree_entries(text)
    spans = backticked_paths(text)
    errors = []
    for lineno, path in entries:
        if path is None:
            errors.append(f"{spec.name}:{lineno}: tree entry with no root line")
        elif not resolves(path, known):
            errors.append(f"{spec.name}:{lineno}: tree entry `{path}` is not tracked")
    for lineno, path in spans:
        if not resolves(path, known):
            errors.append(f"{spec.name}:{lineno}: backticked `{path}` is not tracked")
    return errors, (roots, len(entries), len(spans))


def run_checks(specs: list[Path], known: set[str]) -> list[str]:
    """Check every spec against the known path set; return all errors."""
    errors, totals = [], [0, 0, 0]
    for spec in specs:
        spec_errors, counts = check_spec(spec, known)
        errors.extend(spec_errors)
        totals = [a + b for a, b in zip(totals, counts, strict=True)]
    for label, total in zip(
        ("tree root lines", "tree entries", "backticked paths"), totals, strict=True
    ):
        if total == 0:
            errors.append(f"floor: zero {label} found — parse failure or moved specs?")
    return errors


def main() -> int:
    """Run the check against the repository specs and report the result."""
    specs = sorted(SPEC_DIR.glob("*.md"))
    errors = run_checks(specs, tracked_paths(ROOT))
    for error in errors:
        print(f"error: {error}")
    if errors:
        return 1
    print(f"OK: every tree entry and backticked path in {len(specs)} specs resolves.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
