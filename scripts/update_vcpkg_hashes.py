"""Write or check the hashes that the vcpkg and Conan recipes pin for the FFI archives.

`packages/cpp/portfile.cmake` (vcpkg) downloads one pre-built FFI archive per platform
from the GitHub Release and verifies it against a pinned SHA512; `packages/cpp/conanfile.py`
(Conan) verifies the same archive against the SHA256 in `packages/cpp/conandata.yml`. The
release workflow's `prepare-release` job runs this script on the `build-ffi` artifacts
before it tags, so the tagged commit carries the hashes of exactly the archives
`publish-ffi` uploads.

The portfile is the single list of targets: every `if`/`elseif` block that sets
`ISCC_TARGET` must also set `ISCC_ARCHIVE_EXT` and `ISCC_SHA512`. The version comes from
`packages/cpp/vcpkg.json`. Each target needs `<archive-dir>/iscc-ffi-v{version}-{target}{ext}`;
if any archive is missing, the script names it, writes neither file and exits 1.

Without `--check` the script rewrites only the `ISCC_SHA512` values of the portfile and
renders the whole `conandata.yml` (one SHA256 per portfile target, sorted by triple). With
`--check` it writes nothing and exits 1 listing every target whose pinned hash differs.

Stdlib only: it runs as `python3` on the release runner.

Usage:
    python3 scripts/update_vcpkg_hashes.py <archive-dir> [--check] [--portfile P]
        [--manifest M] [--conandata C]
"""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import re
import sys
from pathlib import Path
from typing import NamedTuple

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_PORTFILE = ROOT / "packages" / "cpp" / "portfile.cmake"
DEFAULT_MANIFEST = ROOT / "packages" / "cpp" / "vcpkg.json"
DEFAULT_CONANDATA = ROOT / "packages" / "cpp" / "conandata.yml"
CONANDATA_HEADER = (
    "---\n"
    "# Written by scripts/update_vcpkg_hashes.py in prepare-release. Do not edit.\n"
    "sha256:\n"
)

BLOCK_RE = re.compile(r"^[ \t]*(?:if|elseif|else|endif)[ \t]*\(", re.MULTILINE)
TARGET_RE = re.compile(r'set\(ISCC_TARGET "(?P<value>[^"]+)"\)')
EXT_RE = re.compile(r'set\(ISCC_ARCHIVE_EXT "(?P<value>[^"]+)"\)')
SHA_RE = re.compile(r'set\(ISCC_SHA512 "(?P<value>[^"]*)"\)')


class Target(NamedTuple):
    """One portfile platform branch: target triple, archive extension and pinned hash."""

    name: str
    ext: str
    sha512: str
    sha_start: int
    sha_end: int


def block_spans(text: str) -> list[tuple[int, int]]:
    """Return the `(start, end)` offsets of the text between consecutive cmake branches."""
    starts = [m.start() for m in BLOCK_RE.finditer(text)]
    bounds = [0, *starts, len(text)]
    return list(itertools.pairwise(bounds))


def single_match(pattern: re.Pattern, text: str, start: int, end: int, what: str):
    """Return the one match of `pattern` in `text[start:end]`, exiting if not exactly one."""
    matches = list(pattern.finditer(text, start, end))
    if len(matches) != 1:
        sys.exit(f"error: portfile branch must set {what} exactly once")
    return matches[0]


def parse_targets(text: str) -> list[Target]:
    """Parse every target branch of the portfile, exiting if none is found."""
    targets: list[Target] = []
    for start, end in block_spans(text):
        if TARGET_RE.search(text, start, end) is None:
            continue
        name = single_match(TARGET_RE, text, start, end, "ISCC_TARGET")
        ext = single_match(EXT_RE, text, start, end, "ISCC_ARCHIVE_EXT")
        sha = single_match(SHA_RE, text, start, end, "ISCC_SHA512")
        targets.append(
            Target(
                name=name.group("value"),
                ext=ext.group("value"),
                sha512=sha.group("value"),
                sha_start=sha.start("value"),
                sha_end=sha.end("value"),
            )
        )
    if not targets:
        sys.exit("error: no `set(ISCC_TARGET ...)` branch found in the portfile")
    return targets


def read_version(manifest: Path) -> str:
    """Read the port version from the vcpkg manifest."""
    return json.loads(manifest.read_text(encoding="utf-8"))["version"]


def archive_path(archive_dir: Path, version: str, target: Target) -> Path:
    """Return the path of the release archive for one target."""
    return archive_dir / f"iscc-ffi-v{version}-{target.name}{target.ext}"


def file_hexdigests(path: Path) -> tuple[str, str]:
    """Return the lowercase `(sha512, sha256)` hex digests of a file, read in one pass."""
    sha512 = hashlib.sha512()
    sha256 = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            sha512.update(chunk)
            sha256.update(chunk)
    return sha512.hexdigest(), sha256.hexdigest()


def conandata_line(name: str, sha256: str) -> str:
    """Return the `conandata.yml` line that pins one target's SHA256."""
    return f"  {name}: {sha256}\n"


def render_conandata(names: list[str], hashes: list[str]) -> str:
    """Return the full `conandata.yml` text: a header, then one line per sorted target."""
    lines = sorted(conandata_line(n, h) for n, h in zip(names, hashes))
    return CONANDATA_HEADER + "".join(lines)


def read_text_or_empty(path: Path) -> str:
    """Return the UTF-8 text of `path`, or an empty string if the file does not exist."""
    return path.read_bytes().decode("utf-8") if path.is_file() else ""


def stale_conan_targets(current: str, names: list[str], hashes: list[str]) -> list[str]:
    """Return the targets whose SHA256 line is absent from the current `conandata.yml`."""
    present = set(current.splitlines(keepends=True))
    return [n for n, h in zip(names, hashes) if conandata_line(n, h) not in present]


def rewrite(text: str, targets: list[Target], hashes: list[str]) -> str:
    """Return the portfile text with each target's SHA512 value replaced."""
    out = text
    for target, value in sorted(
        zip(targets, hashes), key=lambda pair: pair[0].sha_start, reverse=True
    ):
        out = out[: target.sha_start] + value + out[target.sha_end :]
    return out


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    """Parse the command line."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "archive_dir", type=Path, help="directory with the FFI archives"
    )
    parser.add_argument("--check", action="store_true", help="verify, write nothing")
    parser.add_argument("--portfile", type=Path, default=DEFAULT_PORTFILE)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--conandata", type=Path, default=DEFAULT_CONANDATA)
    return parser.parse_args(argv)


def report_check(
    stale: list[str], stale_conan: list[str], conan_ok: bool, count: int, version: str
) -> int:
    """Print the `--check` result and return its exit code."""
    for name in stale:
        print(f"error: stale SHA512 for {name}", file=sys.stderr)
    for name in stale_conan:
        print(f"error: stale Conan SHA256 for {name}", file=sys.stderr)
    if not conan_ok and not stale_conan:
        print("error: conandata.yml differs from the rendered file", file=sys.stderr)
    if stale or not conan_ok:
        return 1
    print(f"OK: {count} portfile and Conan hashes match v{version} archives")
    return 0


def main(argv: list[str] | None = None) -> int:
    """Write (or with `--check` verify) the vcpkg and Conan hashes; return the exit code."""
    args = parse_args(argv)
    text = args.portfile.read_bytes().decode("utf-8")
    targets = parse_targets(text)
    version = read_version(args.manifest)
    paths = [archive_path(args.archive_dir, version, t) for t in targets]
    missing = [p for p in paths if not p.is_file()]
    if missing:
        for path in missing:
            print(f"error: missing archive: {path}", file=sys.stderr)
        return 1
    digests = [file_hexdigests(p) for p in paths]
    hashes = [sha512 for sha512, _ in digests]
    names = [t.name for t in targets]
    sha256s = [sha256 for _, sha256 in digests]
    stale = [t.name for t, h in zip(targets, hashes) if t.sha512 != h]
    conandata = render_conandata(names, sha256s)
    current = read_text_or_empty(args.conandata)
    stale_conan = stale_conan_targets(current, names, sha256s)
    if args.check:
        ok = current == conandata
        return report_check(stale, stale_conan, ok, len(targets), version)
    args.portfile.write_bytes(rewrite(text, targets, hashes).encode("utf-8"))
    args.conandata.write_bytes(conandata.encode("utf-8"))
    print(
        f"updated {len(stale)} of {len(targets)} portfile hashes and "
        f"{len(stale_conan)} Conan hashes for v{version}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
