## 2026-11-22 — Conan recipe verifies the SHA256 of the FFI archive it downloads

**Done:** `conanfile.py` now passes `sha256=` from `conandata.yml` (keyed by target triple) to
`download()`. A target with no pinned hash raises `ConanInvalidConfiguration`, so `sha256=None`
never reaches `download()`. `scripts/update_vcpkg_hashes.py` computes SHA512 and SHA256 in one pass
over each archive and renders the whole `conandata.yml`. The missing-archive check is shared, so a
missing archive writes neither file. `prepare-release` stages the file in the release commit.

**Files changed:**

- `packages/cpp/conandata.yml` (new): 5 SHA256s of the v0.7.0 archives. Starts with `---` because
    yamlfix inserts one
- `packages/cpp/conanfile.py`: `_archive_sha256(target)`, `download(..., sha256=sha256)`, new
    docstring
- `scripts/update_vcpkg_hashes.py`: `file_hexdigests` (one pass gives SHA512 and SHA256, replacing
    `sha512_hex`), `render_conandata`, `stale_conan_targets`, `report_check`, `--conandata` option,
    module docstring. `--check` compares the rendered text with the file and names each stale target
    ("stale Conan SHA256 for X")
- `.github/workflows/release.yml`: step renamed "Update vcpkg and Conan hashes"; the `git add` line
    gains `packages/cpp/conandata.yml`; 3 comments updated
- `tests/test_update_vcpkg_hashes.py`: 8 new tests (one SHA256 written per target, missing archive
    writes no conandata, stale Conan hash, missing conandata, committed file equals the rendered
    form, conandata keys == matrix, `_target_triple` triples == matrix (via `ast`), `download` has a
    non-None `sha256=`). The commit-order test now parses the `git add` paths and requires conandata
- `packages/cpp/CLAUDE.md`, `.claude/context/specs/cpp-bindings.md`: Conan publishing and pitfall
    text added

**Verification:**

- `uv run pytest tests/test_update_vcpkg_hashes.py tests/test_ffi_release_assets.py`: 32 passed.
    `tests/test_check_release_workflow.py`: 27 passed
- Mutation probe: I dropped `sha256=`, the conandata `git add` entry and the Windows mapping entry,
    ran the tests, then restored the files from backup. The 3 matching tests failed, 17 passed
- The SHA256s match GitHub's own asset `digest` fields for v0.7.0 (`gh release view --json assets`)
- `python3 scripts/update_vcpkg_hashes.py /tmp/ffi070 --check` printed
    `OK: 5 portfile and Conan hashes match v0.7.0 archives` with rc=0.
    `git status --porcelain packages/cpp/portfile.cmake` printed nothing
- Conan 2.34.0, `CONAN_HOME=/tmp/conanhome`:
    - `conan create packages/cpp` exited 0. The log shows `exports: File 'conandata.yml' found`,
        `Sources correctly downloaded from …x86_64-unknown-linux-gnu.tar.gz`, and
        `Package '63fead08…' created`
    - Copy with the last digit `0`→`1` in the Linux x86_64 hash: exit 1,
        `ConanException: sha256 hash failed for 'iscc-ffi-v0.7.0-x86_64-unknown-linux-gnu.tar.gz' file. Provided hash: …c40c1 Computed hash: …c40c0`
    - Copy with the Linux x86_64 line deleted: exit 6,
        `Invalid configuration: No SHA256 pinned in conandata.yml for target x86_64-unknown-linux-gnu`
- The tampered copy came from `git ls-files -co --exclude-standard packages/cpp | tar`, not
    `git archive`, because conandata.yml was not committed yet. `packages/cpp` has no Conan output
    files, and `/tmp/conanhome` and `/tmp/conanbad` are removed
- `git grep -n "sha256=" packages/cpp/conanfile.py` →
    `97: download(self, url, archive_path, sha256=sha256)`
- `uv run scripts/check_release_workflow.py …` printed OK; `mise run version:check` rc=0
- `mise run check` rc=0 on the second run (the first run had mdformat reflow
    `packages/cpp/CLAUDE.md`). `conandata.yml` md5 is `cf57c25f…` before and after both runs.
    `ty check` and ruff `S,C901` on the touched Python files are clean

**Next:** Two `normal` `[audit]` issues still block v0.8.0: the Rust conformance runner duplication,
and the spec and API-doc drift.

**Notes:**

- No changes to `version_sync.py`: conandata has no version key, by design.
- `validate()` does not check for a pinned hash; only `build()` does, through `_archive_sha256`. I
    kept the change to the minimum. Moving the check into `validate()` would fail earlier, but it is
    optional.
- `--check` with every hash line present but other bytes different (for example a hand-edited
    comment) prints "conandata.yml differs from the rendered file" and exits 1.
- `issues.md` is untouched. The review can close "Conan recipe downloads the FFI archive without a
    checksum".
