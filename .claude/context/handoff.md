## 2026-03-08 — Add SHA512 checksums to vcpkg portfile

**Done:** Downloaded all 5 v0.2.0 FFI release tarballs from GitHub Releases, computed SHA512
checksums for each, added `set(ISCC_SHA512 "...")` lines in each platform branch of
`portfile.cmake`, and replaced `SKIP_SHA512` with `SHA512 "${ISCC_SHA512}"` in the
`vcpkg_download_distfile` call.

**Files changed:**

- `packages/cpp/portfile.cmake`: Added per-platform SHA512 checksums (5 `set()` calls) and replaced
    `SKIP_SHA512` with `SHA512 "${ISCC_SHA512}"` in the download call

**Verification:** All 5 criteria pass:

- `SKIP_SHA512` no longer present in the file
- `ISCC_SHA512` appears exactly 6 times (5 `set()` + 1 `SHA512` usage)
- `SHA512 "${ISCC_SHA512}"` pattern present in download call
- All 5 SHA512 hashes are 128 lowercase hex characters
- CMake syntax is valid (errors on vcpkg functions as expected, not on syntax)
- All 15 pre-commit hooks pass

**Next:** Only one `normal` priority issue remains: language logos in README/docs. After that, the
CID loop should check issues.md for any remaining items or signal idle.

**Notes:** The SHA512 checksums are pinned to the v0.2.0 release tarballs. Future releases will need
these checksums updated — the next.md explicitly noted that release workflow automation for
computing SHA512 on future releases is out of scope and should be a follow-up step. The Conan recipe
(`conanfile.py`) also lacks SHA512 verification but that's a separate concern per the scope
definition.
