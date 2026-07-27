"""Byte-identity gate for vendored copies of the canonical conformance fixtures.

Per-language test trees (Go, .NET, Kotlin, Swift) cannot read files across the
repo root once the package is consumed as a standalone module/package, so they
carry vendored copies of the canonical fixtures. Those copies must stay
byte-identical to their canonical sources — drift would silently leave a
binding testing stale vectors. Do not "fix" the duplication by deleting a copy;
register every tracked copy in VENDORED_COPIES instead.

This gate is about files on disk and deliberately does not import iscc_lib.
"""

import subprocess
from pathlib import Path

import pytest

REPO_ROOT = Path(__file__).parent.parent

FIXTURE_BASENAMES = {"data.json", "unicode_boundary.json"}

# Canonical fixture -> tracked vendored copies (repo-relative POSIX paths).
VENDORED_COPIES = {
    "crates/iscc-lib/tests/data.json": [
        "packages/dotnet/Iscc.Lib.Tests/testdata/data.json",
        "packages/go/testdata/data.json",
        "packages/kotlin/src/test/resources/data.json",
        "packages/swift/Tests/IsccLibTests/data.json",
    ],
    "crates/iscc-lib/tests/unicode_boundary.json": [
        "packages/go/testdata/unicode_boundary.json",
        "packages/swift/Tests/IsccLibTests/unicode_boundary.json",
    ],
}

PAIRS = [
    pytest.param(canonical, copy, id=copy)
    for canonical, copies in VENDORED_COPIES.items()
    for copy in copies
]


def tracked_fixture_paths():
    """Return repo-relative paths of all git-tracked conformance fixture files."""
    out = subprocess.run(
        ["git", "ls-files", "-z", "--", "*data.json", "*unicode_boundary.json"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    paths = [p for p in out.stdout.split("\0") if p]
    # The '*data.json' pathspec also matches names like 'foo_data.json', so
    # filter to exact basename matches rather than trusting the glob.
    return {p for p in paths if Path(p).name in FIXTURE_BASENAMES}


def test_table_is_not_vacuous():
    """Guard against an emptied or mistyped table collecting zero identity cases."""
    assert len(PAIRS) >= 5
    for canonical in VENDORED_COPIES:
        assert (REPO_ROOT / canonical).is_file(), f"canonical missing: {canonical}"


@pytest.mark.parametrize(("canonical", "copy"), PAIRS)
def test_vendored_copy_is_byte_identical(canonical, copy):
    """Verify a vendored fixture copy is byte-identical to its canonical source."""
    canonical_bytes = (REPO_ROOT / canonical).read_bytes()
    copy_bytes = (REPO_ROOT / copy).read_bytes()
    assert copy_bytes == canonical_bytes, (
        f"vendored fixture drifted from its canonical source; "
        f"fix with: cp {canonical} {copy}"
    )


def test_no_unregistered_tracked_copy():
    """Verify every tracked fixture file is registered in VENDORED_COPIES."""
    registered = set(VENDORED_COPIES) | {
        copy for copies in VENDORED_COPIES.values() for copy in copies
    }
    tracked = tracked_fixture_paths()
    assert tracked == registered, (
        "tracked fixture files do not match the registered table; register each "
        "new copy in VENDORED_COPIES in this file (do not delete the file): "
        f"unregistered={sorted(tracked - registered)}, "
        f"gone={sorted(registered - tracked)}"
    )


def test_boundary_fixture_is_pure_ascii():
    """Verify the boundary fixture stays pure ASCII (it stores \\uXXXX escapes).

    A tool that decodes the escapes into literal UTF-8 would silently corrupt
    the fixture while remaining valid JSON; this guard catches that directly.
    """
    fixture = REPO_ROOT / "crates/iscc-lib/tests/unicode_boundary.json"
    assert fixture.read_bytes().isascii(), (
        "unicode_boundary.json must contain only ASCII bytes "
        "(\\uXXXX escapes, never literal non-ASCII characters)"
    )
