"""Compare the staged FFI archives with the assets on their GitHub Release.

The release workflow's `publish-ffi` job uploads `ffi-staging/iscc-ffi-v*.*` with
`overwrite_files: false`, which skips an asset by NAME. A failed upload (502) leaves an
empty asset in state `starter`, so a `gh run rerun --failed` would skip it and report
success. This script reads the release's assets (the output of
`gh release view <tag> --json assets`) and the staging directory, and does no network
I/O itself:

- `stale` prints the `apiUrl` of every staged archive's asset whose `state` is not
  `uploaded`, one per line, so the job can delete it before the upload.
- `verify` exits 1 naming every staged archive that is missing from the release or not
  in state `uploaded`, and exits 0 otherwise.

Assets that are not staged (the XCFramework zip, other targets) are ignored. An empty
staging directory is an error in both modes: a check over an empty set proves nothing.

Stdlib only: it runs as `python3` on the release runner.

Usage:
    python3 scripts/ffi_release_assets.py {stale,verify} <staging-dir> <assets.json>
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

STAGED_GLOB = "iscc-ffi-v*.*"
UPLOADED = "uploaded"


def staged_names(staging_dir: Path) -> list[str]:
    """Return the sorted names of the staged archives (the upload step's glob)."""
    return sorted(p.name for p in staging_dir.glob(STAGED_GLOB) if p.is_file())


def read_assets(assets_json: Path) -> list[dict]:
    """Return the asset list from a `gh release view --json assets` output file."""
    return json.loads(assets_json.read_text(encoding="utf-8"))["assets"]


def stale_urls(names: list[str], assets: list[dict]) -> list[str]:
    """Return the `apiUrl` of each staged-name asset whose state is not `uploaded`."""
    wanted = set(names)
    return [
        a["apiUrl"] for a in assets if a["name"] in wanted and a["state"] != UPLOADED
    ]


def verify_errors(names: list[str], assets: list[dict]) -> list[str]:
    """Return one message per staged name that is missing or not `uploaded`."""
    states = {a["name"]: a["state"] for a in assets}
    errors = []
    for name in names:
        state = states.get(name)
        if state is None:
            errors.append(f"missing from the release: {name}")
        elif state != UPLOADED:
            errors.append(f"asset in state {state!r}, not {UPLOADED!r}: {name}")
    return errors


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    """Parse the command line."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("mode", choices=["stale", "verify"])
    parser.add_argument("staging_dir", type=Path, help="directory with the archives")
    parser.add_argument("assets_json", type=Path, help="gh release view --json assets")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    """Run `stale` or `verify` and return the exit code."""
    args = parse_args(argv)
    names = staged_names(args.staging_dir)
    if not names:
        print(f"error: no {STAGED_GLOB} archive in {args.staging_dir}", file=sys.stderr)
        return 1
    assets = read_assets(args.assets_json)
    if args.mode == "stale":
        for url in stale_urls(names, assets):
            print(url)
        return 0
    errors = verify_errors(names, assets)
    for message in errors:
        print(f"error: {message}", file=sys.stderr)
    if errors:
        return 1
    print(f"OK: all {len(names)} staged FFI archives are uploaded")
    return 0


if __name__ == "__main__":
    sys.exit(main())
