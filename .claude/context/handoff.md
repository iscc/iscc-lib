## 2026-03-21 — Add Package.swift version sync and update Swift install docs

**Done:** Added `Package.swift` `releaseTag` as the 16th version sync target in
`scripts/version_sync.py`, and replaced the manual native library build section in
`docs/howto/swift.md` with a collapsible "Build from source" tip following the dotnet pattern.

**Files changed:**

- `scripts/version_sync.py`: Added `_get_package_swift_release_tag` /
    `_sync_package_swift_release_tag` functions, added TARGETS entry for `Package.swift`, updated
    module docstring to mention Package.swift
- `docs/howto/swift.md`: Replaced lines 35-60 (manual "Build the native library" section with
    cargo/linker instructions and "planned for a future release" text) with collapsible
    `??? tip "Build from source"` admonition pointing to `build_xcframework.sh` and
    `useLocalFramework` toggle

**Verification:** All 5 criteria pass:

- `uv run scripts/version_sync.py --check` exits 0 — 16 targets all match (was 15)
- `grep -c "Package.swift" scripts/version_sync.py` returns 4 (≥2: docstring + TARGETS entry + 2
    function names)
- `grep "planned for a future release" docs/howto/swift.md` returns no matches (old text removed)
- `grep "Build from source" docs/howto/swift.md` returns a match (new collapsible section present)
- `mise run check` passes — all 15 pre-commit hooks green

**Next:** The Swift vend issue in issues.md can be closed after this is merged. Remaining work
items: the `GITHUB_REF_NAME` bug in the Swift release job (filed in issues.md, marked HUMAN REVIEW
REQUESTED) is out of scope for this step and requires a spec update.

**Notes:** mdformat auto-rewrapped a long line in the collapsible section (line 51-52) — normal
behavior, no manual intervention needed. No tests needed for this step (Python script changes are
verified by `--check` mode; docs changes are verified by grep).
