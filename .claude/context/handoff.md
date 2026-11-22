## 2026-11-22 — Review of: `prepare-release` writes the vcpkg portfile SHA512s from the built FFI archives before tagging

**Verdict:** PASS_WITH_NOTES

**Summary:** The portfile now carries the v0.7.0 hashes, written by the new stdlib script
`scripts/update_vcpkg_hashes.py`, and `release.yml` writes them from the `build-ffi` archives into
the commit it tags. The work matches next.md, the code is small and fails closed, and 13 tests pin
the script and the wiring. One recovery gap in the new `overwrite_files: false` setting is filed as
a `normal` issue (found by Codex).

**Verification:**

- [x] With the five v0.7.0 FFI assets in an empty directory D,
    `python3 scripts/update_vcpkg_hashes.py --check D` exits 0 — "OK: 5 portfile hashes match v0.7.0
    archives"; the linux x64 hash starts `26f396ed…` and equals `sha512sum` of the archive
- [x] `uv run pytest tests/test_update_vcpkg_hashes.py tests/test_check_release_workflow.py` passes
    — 40 passed
- [x] `uv run python scripts/check_release_workflow.py` exits 0 — and the CI-only
    `--check-action-inputs` run resolves 19 of 19 action refs
- [x] `grep -n "must be updated" packages/cpp/CLAUDE.md` prints nothing, and the file names
    `scripts/update_vcpkg_hashes.py` (2 mentions)
- [x] `mise run check` exits 0 — also `uv run ty check` and ruff `S`/`C901` on both new files

Probes beyond next.md (2 of 3 used):

- **The port installs end to end.** A fresh vcpkg on x64-linux ran
    `vcpkg install iscc --overlay-ports=<portfile.cmake + vcpkg.json from HEAD>`: download of the
    v0.7.0 archive, hash check, headers and libraries installed, exit 0. Before this step the hash
    check failed. It prints one post-build warning (filed `low`, see below)
- **The script runs on the release runner.** It uses `itertools.pairwise` (Python 3.10+) and runs as
    bare `python3` on `macos-15`; the runner image readme lists Python 3.15.2

**Issues found:**

- Filed `normal`: "`publish-ffi` skips a half-uploaded FFI archive on a rerun" (Codex, confirmed in
    the action source). The entry also covers two sentences in `docs/development.md` that still
    describe the old FFI upload behaviour
- Filed `low`, untriaged: "vcpkg port fails one post-build check" (debug/release binary count; only
    matters for a submission to the vcpkg registry). Pre-existing
- Deleted: "vcpkg portfile pins the SHA512 hashes of the v0.2.0 tarballs" (resolved)
- Fixed in this review (facts the step made stale): `specs/swift-bindings.md` → Release Workflow
    Integration said each registry chain starts with a job that needs `prepare-release` (`build-ffi`
    and `test-ffi` no longer do); `specs/ci-cd.md` → Job Conditions and `docs/development.md` said
    `prepare-release` commits only the `Package.swift` checksum
- Not filed, minor: write mode rewrites the portfile even when no hash changed; `single_match` has
    no return annotation

**Codex review:** One finding (P2), accepted. With `overwrite_files: false`,
`softprops/action-gh-release@v3` skips a file whose name is already on the release and does not
check the asset's state (`src/github.ts`, `upload`). GitHub documents that a failed upload can leave
an empty `starter` asset under that name, so `gh run rerun --failed` would report success and leave
the archive unusable. Not reproducible here; filed as the `normal` issue above.

**Next:** The `@iscc/lib` npm README result-type sentence (`normal`, docs-only), as define-next
ranked it. The new `publish-ffi` issue is a small follow-up (`release.yml`, a wiring test,
`docs/development.md`) and fits well while this wiring is fresh; define-next decides the order.

**Notes:**

- decisions.md was 410 lines; the runner's `archive_overflow` rotated one entry, now 397. No new
    entry from this review: the advance entry of 2026-11-22 covers the design
- `failure()` sees every ancestor. Because `prepare-release` now needs `build-ffi`, a failed FFI leg
    also skips other chains in a combined re-trigger (`-f ffi=true -f pypi=true`). It fails safe and
    a separate dispatch works around it; not filed
- `test-ffi` still does not gate the tag (it never did: the tag used to be created before any
    build). Adding it to `prepare-release.needs` would be cheap if Titusz wants the smoke test
    before the tag; not filed
- The first real run of this wiring is the v0.8.0 release. A dispatch must come from `main`:
    `build-ffi` names its archives from the dispatch commit's `Cargo.toml`, and `prepare-release`
    reads the version from `vcpkg.json` on `main`; a mismatch fails closed ("missing archive")
- Gate idea, not scoped: the port can only install while the release named in `vcpkg.json` exists,
    so it cannot be a CI job on `develop`. `mise run test:install` could run
    `update_vcpkg_hashes.py --check` on the downloaded release assets after a release
- No Rust change: CRAP, iai and semver are unaffected
