# Next Work Package

## Step: `prepare-release` writes the vcpkg portfile SHA512s from the built FFI archives before tagging

## Goal

Fix the `normal` `[audit]` issue "vcpkg portfile pins the SHA512 hashes of the v0.2.0 tarballs". The
portfile now carries the v0.7.0 hashes. From now on the release workflow writes the hashes from the
archives it publishes, in the commit it tags, so the port works for every future release.

## Alternatives Considered

- **Chosen:** release-time hash writing plus a stdlib script that has a check mode. This copies the
    existing Swift checksum pattern. The other suggested fix (drop the port) would amend
    `specs/cpp-bindings.md` → Distribution, and that needs Titusz.
- **Rejected:** the `@iscc/lib` README result-type sentence (`normal`, docs-only). It is cheaper but
    less valuable: the vcpkg channel has been broken for five releases. It is the next step.
- **Rejected design:** a job after publishing that commits the hashes to `main`. The tagged commit
    would still carry stale hashes, and vcpkg consumers pin tags.

## Scope

- **Create**: `scripts/update_vcpkg_hashes.py`, `tests/test_update_vcpkg_hashes.py`
- **Modify**: `packages/cpp/portfile.cmake` (v0.7.0 hashes, written by the script),
    `.github/workflows/release.yml`, `packages/cpp/CLAUDE.md` (Publishing + "vcpkg SHA512 updates"
    pitfall), `.claude/context/specs/cpp-bindings.md` (one Distribution line on how the hashes are
    kept)
- **Reference**: `scripts/check_release_workflow.py` (guard and needs invariants),
    `tests/test_check_release_workflow.py` (how tests load `release.yml`), `release.yml`
    `prepare-release` (lines 55-122), `build-ffi` / `test-ffi` / `publish-ffi` / `pack-nuget`

## Not In Scope

- The Conan recipe (it does no hash check). Submitting the port to the vcpkg registry
- Making the FFI archives reproducible, and any other `release.yml` restructuring
- vcpkg usage docs for users (no page documents vcpkg install today). Do not add one
- `version_sync.py` changes: the hashes are not a version field

## Implementation Notes

**Script** (stdlib only: it runs as `python3` on the `macos-15` runner; docstrings per project
style): `update_vcpkg_hashes.py <archive-dir> [--check] [--portfile P] [--manifest M]`. The defaults
are `packages/cpp/portfile.cmake` and `packages/cpp/vcpkg.json`.

- Read the version from `vcpkg.json`. Read the target branches from the portfile: each
    `set(ISCC_TARGET "…")` with its `ISCC_ARCHIVE_EXT` and `ISCC_SHA512` in the same `if`/`elseif`
    block. The portfile is the single list of targets, so nothing is hardcoded
- Each target needs `<dir>/iscc-ffi-v{version}-{target}{ext}`. If any archive is missing, exit 1,
    name it, and write nothing. A file for another version counts as missing
- Without `--check`: compute the SHA512 hex (lowercase) and rewrite only the `ISCC_SHA512` values.
    With `--check`: write nothing, and exit 1 listing every target whose hash differs
- Fix the current portfile by running the script on the real v0.7.0 assets:
    `gh release download v0.7.0 -p 'iscc-ffi-v0.7.0-*' -D <empty tmp dir>`. The issue measured the
    linux x64 value as `26f396ed…`

**release.yml** (it is never run by CI, so the wiring test below is the gate):

- `build-ffi`: drop `needs: [prepare-release]`. Keep its guard `if` unchanged
- `prepare-release`: add `needs: [build-ffi]` and keep the bare `if: inputs.version != ''` (the
    checker requires it). Before "Commit Package.swift", add `actions/download-artifact@v8` with
    `pattern: ffi-*`, `path: ffi-staging` and `merge-multiple: true` (copy `publish-ffi`). Then add
    a step that runs `python3 scripts/update_vcpkg_hashes.py ffi-staging`. The commit step stages
    `packages/cpp/portfile.cmake` with `Package.swift`. Generalise its "unchanged" message and its
    commit subject
- `publish-ffi` and `pack-nuget` got the tag ordering through `build-ffi`; add `prepare-release` to
    their `needs`. `publish-ffi`: set `overwrite_files: false` on `softprops/action-gh-release@v3`
    (declared in its `action.yml`, default `'true'`). A later `ffi` re-trigger then cannot replace
    archives whose hashes are committed. Recovery stays `gh run rerun --failed`, which reuses the
    run's artifacts
- Consequence to document in the cpp `CLAUDE.md`: a failing `build-ffi` leg now blocks the tag and,
    through `!failure()`, every publish job. This is intended: no tag with hashes that do not match

**Tests** (pytest, no network; fake archives in `tmp_path`, with a copy of the portfile and a
`vcpkg.json` written in `tmp_path`). One bad case per call:

- The rewrite sets each hash to `hashlib.sha512` of its fake archive. The rest of the portfile stays
    byte-identical
- A missing archive exits non-zero and leaves the portfile byte-identical. So does an archive named
    for another version
- `--check` passes after a rewrite and fails on a stale hash
- The portfile target set equals the `build-ffi` `strategy.matrix.include[].target` set in the real
    `release.yml`
- Wiring in the real `release.yml`: `prepare-release.needs` contains `build-ffi`; a step `run` that
    calls `update_vcpkg_hashes.py` comes before the step whose `run` stages
    `packages/cpp/portfile.cmake`, and that step comes before "Create tags"; `build-ffi` has no
    `needs` on `prepare-release`; `publish-ffi` and `pack-nuget` need `prepare-release`;
    `overwrite_files` on `publish-ffi` is `false`

Record a decisions.md entry: "vcpkg SHA512s are written by `prepare-release` from the `build-ffi`
artifacts, before the tag". No Rust change: CRAP, iai and semver are unaffected.

## Verification

- With the five v0.7.0 FFI assets downloaded to an empty directory D,
    `python3 scripts/update_vcpkg_hashes.py --check D` exits 0
- `uv run pytest tests/test_update_vcpkg_hashes.py tests/test_check_release_workflow.py` passes
- `uv run python scripts/check_release_workflow.py` exits 0
- `grep -n "must be updated" packages/cpp/CLAUDE.md` prints nothing, and the cpp `CLAUDE.md` names
    `scripts/update_vcpkg_hashes.py`
- `mise run check` exits 0

## Done When

The portfile matches the v0.7.0 release archives, `release.yml` writes the hashes from the built
archives into the tagged commit, and the tests above pin that wiring.
