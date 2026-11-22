# Next Work Package

## Step: Conan recipe verifies the SHA256 of the FFI archive it downloads

## Goal

Picks up the `[audit]` issue "Conan recipe downloads the FFI archive without a checksum": the recipe
passes a pinned SHA256 to `download()`, `prepare-release` writes those hashes from the `build-ffi`
archives in the same pass as the vcpkg SHA512s, and tests tie the recipe's target table to the
`build-ffi` matrix. It is one of three `normal` issues that block v0.8.0.

## Alternatives Considered

- **Chosen:** Conan checksum — it has a security impact (unverified native binary on a user's
    machine), it is small (3 non-test files plus one generated data file), and the handoff ranks it
    first.
- **Rejected:** Rust conformance runner deduplication — test-only cleanup with no user impact; next.
- **Rejected:** a separate `scripts/update_conan_hashes.py` — it would duplicate the archive lookup,
    the missing-archive and `--check` logic of the vcpkg script. A rename to a neutral name would
    leave the decision of 2026-11-22 (which names `scripts/update_vcpkg_hashes.py`) pointing at a
    missing file, so the existing script is extended under its current name.

## Scope

- **Create**: `packages/cpp/conandata.yml` (data the script writes; Conan exports and loads it as
    `self.conan_data` on its own)
- **Modify**: `packages/cpp/conanfile.py`, `scripts/update_vcpkg_hashes.py`,
    `.github/workflows/release.yml` (the "Commit release checksums" `git add` adds
    `packages/cpp/conandata.yml`; rename the hash step to say vcpkg and Conan)
- **Modify (tests)**: `tests/test_update_vcpkg_hashes.py`
- **Modify (docs)**: `packages/cpp/CLAUDE.md` (Publishing → add Conan; Pitfalls → "never edit the
    hashes by hand" covers `conandata.yml`; a new platform also needs a `_target_triple` entry),
    `.claude/context/specs/cpp-bindings.md` → Distribution, the Conan line
- **Reference**: `tests/test_ffi_release_assets.py` (workflow-reading test helpers),
    `scripts/version_sync.py` (`_sync_conanfile`, must keep working)

## Not In Scope

- The Rust conformance duplication issue and the spec/API-doc drift issue (their own steps)
- The vcpkg post-build check `low` issue, any ConanCenter submission, `conan` as a dev dependency
- Moving the os/arch → triple/lib-name table out of `conanfile.py` into YAML; changing the vcpkg
    hash logic or the portfile beyond what sharing the archive pass requires
- Editing decisions.md (the decision still holds: the script still writes the vcpkg hashes)

## Implementation Notes

- **Layout** of `conandata.yml`, keyed by target triple only (no version key, so `version_sync.py`
    stays untouched; between a version bump and the release the hashes are stale, exactly as the
    portfile's are):

    ```text
    # Written by scripts/update_vcpkg_hashes.py in prepare-release. Do not edit.
    sha256:
      aarch64-apple-darwin: <64 hex>
      ...
    ```

    The script stays stdlib-only (`python3` on the macOS runner, no PyYAML): it renders the whole file
    from the portfile's target list (sorted triples), it never parses YAML on write. `--check`
    compares the rendered text with the file on disk and names each stale target. The file must be
    byte-stable under the prek hooks (`yamlfix`, `check-yaml`, end-of-file): generate, run
    `mise run format`, and make the renderer emit exactly what the hooks leave.

- One pass: compute SHA512 and SHA256 per archive; the missing-archive check stays shared, so a
    missing archive still writes neither file. Add a `--conandata` path option (default
    `packages/cpp/conandata.yml`) next to `--portfile`/`--manifest`; update the module docstring.

- **Recipe**: `download(self, url, archive_path, sha256=self.conan_data["sha256"][target])`
    (`conan/tools/files/files.py` in conan 2.34:
    `download(conanfile, url, filename, …, sha256=None)` and `conandata.yml` next to the recipe is
    exported and loaded by `conan/internal/api/export.py`). Never let `sha256=None` reach
    `download()` — that disables the check; a missing target must raise `ConanInvalidConfiguration`
    naming it. Fix the docstring ("Mirrors … portfile.cmake").

- **Initial hashes**: fill the file from the published v0.7.0 archives (recipe version is 0.7.0):
    `gh release download v0.7.0 -R iscc/iscc-lib -p 'iscc-ffi-v0.7.0-*' -D /tmp/ffi070`, then
    `python3 scripts/update_vcpkg_hashes.py /tmp/ffi070`. The portfile must come out unchanged (its
    SHA512s are already the 0.7.0 ones); if not, stop and report rather than commit portfile edits.

- **Tests** (stdlib `ast`/`yaml`, no `conan` import): the script writes a SHA256 per target and
    `--check` fails naming a stale Conan hash (one stale case per call); the `conandata.yml` keys
    and the triples in `_target_triple` each equal the `build-ffi` matrix targets (read the recipe
    with `ast`); the recipe's `download(...)` call passes a `sha256=` keyword; the release commit
    step stages `packages/cpp/conandata.yml`, after the hash step and before the tag.

- End-to-end, with a throwaway `CONAN_HOME` (network is fine outside the test suite):
    `CONAN_HOME=/tmp/conanhome uv run --no-project --with conan conan profile detect` then
    `… conan create packages/cpp`. Then edit one hex digit of the x86_64 Linux hash in a
    `git archive` copy of `packages/cpp` and confirm `conan create` on that copy fails on the hash.
    Report both outcomes in the handoff. Clean `packages/cpp` of any Conan output files afterwards.

## Verification

- `uv run pytest tests/test_update_vcpkg_hashes.py tests/test_ffi_release_assets.py` passes,
    including the new Conan tests
- `python3 scripts/update_vcpkg_hashes.py /tmp/ffi070 --check` exits 0 on the v0.7.0 archives, and
    `git status --porcelain packages/cpp/portfile.cmake` prints nothing
- `conan create packages/cpp` succeeds with the committed hashes and fails with one altered digit
    (both outputs in the handoff)
- `git grep -n "sha256=" packages/cpp/conanfile.py` shows the `download(` call's keyword
- `uv run scripts/check_release_workflow.py .github/workflows/release.yml` and
    `mise run version:check` exit 0
- `mise run check` exits 0, and running it again leaves `packages/cpp/conandata.yml` unchanged

## Done When

All six verification criteria pass, and the Conan recipe refuses an archive whose SHA256 differs
from the hash `prepare-release` wrote.
