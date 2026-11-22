## 2026-11-22 — `prepare-release` writes the vcpkg portfile SHA512s from the built FFI archives before tagging

**Done:** Added `scripts/update_vcpkg_hashes.py` (stdlib, write/`--check` modes). The portfile is
the single target list; the version comes from `vcpkg.json`. Ran it on the five real v0.7.0 archives
to fix `portfile.cmake`. Rewired `release.yml` so `prepare-release` needs `build-ffi`, downloads
`ffi-*`, runs the script and commits the portfile with `Package.swift` before tagging. `publish-ffi`
and `pack-nuget` need `prepare-release`; `publish-ffi` has `overwrite_files: false`.

**Files changed:**

- `scripts/update_vcpkg_hashes.py`: new. Parses `if`/`elseif` branches (each must set `ISCC_TARGET`,
    `ISCC_ARCHIVE_EXT`, `ISCC_SHA512` exactly once). It exits 1 with each missing archive named and
    writes nothing, and rewrites only the hash spans (byte-level read/write)
- `tests/test_update_vcpkg_hashes.py`: new, 13 tests (7 script, 6 wiring against the real
    `release.yml`)
- `packages/cpp/portfile.cmake`: 5 SHA512s → v0.7.0 values (written by the script)
- `.github/workflows/release.yml`: `build-ffi` lost `needs`; `prepare-release` got
    `needs: [build-ffi]` (bare `if` kept), a download step and an update step. The commit step was
    renamed "Commit release checksums" and stages both files, with generalised message and subject.
    `needs` on `publish-ffi`/`pack-nuget` and `overwrite_files: false` added; header comments
    updated
- `packages/cpp/CLAUDE.md`: Publishing paragraph + "vcpkg SHA512 updates" pitfall rewritten (names
    the script, `--check`, `gh run rerun --failed`, the build-ffi-blocks-tag consequence)
- `.claude/context/specs/cpp-bindings.md`: one Distribution sentence on how the hashes are kept
- `.claude/context/decisions.md`: entry "vcpkg SHA512s are written by `prepare-release` …"

**Verification:**

- `gh release download v0.7.0 -p 'iscc-ffi-v0.7.0-*'` into an empty tmp dir D. Before the write:
    `--check D` → 5× `stale SHA512`, exit 1. After: `OK: 5 portfile hashes match v0.7.0 archives`,
    exit 0. The linux x64 hash starts `26f396edae983bc2f129` = `sha512sum` of that archive (and the
    issue's value)
- `uv run pytest tests/test_update_vcpkg_hashes.py tests/test_check_release_workflow.py` → 40
    passed. Full `uv run pytest tests` → 860 passed
- Wiring tests against `git show HEAD:.github/workflows/release.yml` (throwaway probe): all 5 wiring
    tests FAIL on the old workflow (`needs`, step order, download, ffi consumers, overwrite_files).
    The matrix-equality test passes on both, as it should
- `uv run python scripts/check_release_workflow.py` → OK, exit 0. `actionlint@v1.7.7` on
    `release.yml` → exit 0, no output
- `softprops/action-gh-release@v3` `action.yml` declares `overwrite_files` (default `'true'`),
    checked via `gh api`
- `grep -n "must be updated" packages/cpp/CLAUDE.md` → no output; the file names the script
- `mise run check` → exit 0 (the first run flagged ruff RUF007 `zip` → fixed with
    `itertools.pairwise`). `ty check` on both files → clean

**Next:** The `@iscc/lib` npm README result-type sentence (`normal`, docs-only), per the define-next
ranking.

**Notes:**

- `Target` is a `NamedTuple`, not a `@dataclass`: the tests load the script by path without
    registering it in `sys.modules`, and `dataclass` crashes on that (`'NoneType' … __dict__`)
- Unverifiable here: the real `release.yml` run. `build-ffi` checks out the dispatch sha while
    `prepare-release` checks out `main` (unchanged from before). Every pre-step `prepare-release`
    checks on `main` (version check) now runs only after the ~5 FFI builds finish, so a wrong
    version input fails later than before. `test-ffi` still runs in parallel with `prepare-release`.
    A failing smoke test therefore no longer stops the tag, only `publish-ffi`. That matches
    next.md's wiring, but the reviewer may want `test-ffi` in `prepare-release.needs` too (out of
    scope here)
- The script's `--check` with archives of another version reports them as missing (exit 1), same as
    write mode
- No Rust change: CRAP, iai, semver unaffected
