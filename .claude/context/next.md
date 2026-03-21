# Next Work Package

## Step: Add Package.swift version sync and update Swift install docs

## Goal

Add the root `Package.swift` `releaseTag` as a version sync target and update `docs/howto/swift.md`
to document zero-friction SPM installation. Together, these close the "Swift package does not vend
the native library" normal issue.

## Scope

- **Create**: (none)
- **Modify**:
    - `scripts/version_sync.py` — add `_get_package_swift_release_tag` /
        `_sync_package_swift_release_tag` functions and TARGETS entry for `Package.swift`
    - `docs/howto/swift.md` — replace "Build the native library" manual build section with
        zero-friction SPM install note (XCFramework is resolved automatically)
- **Reference**:
    - `Package.swift` — line 16: `let releaseTag = "0.3.1"` (the sync target pattern)
    - `docs/howto/dotnet.md` — installation section pattern (collapsible "Build from source" tip)
    - `docs/howto/kotlin.md` — similar pattern for reference

## Not In Scope

- Fixing the `GITHUB_REF_NAME` bug in the Swift release job — marked `HUMAN REVIEW REQUESTED`,
    requires spec update
- Adding Swift version snippet sync to doc files (like the Maven doc version sync) — not needed yet
- Modifying `packages/swift/Package.swift` (the dev/CI version) — only root `Package.swift` is the
    sync target
- Changing the `releaseChecksum` value (that's handled by the release workflow's
    `swift package compute-checksum` step)

## Implementation Notes

### Version sync (`version_sync.py`)

The `releaseTag` line in root `Package.swift` looks like:

```swift
let releaseTag = "0.3.1"
```

Add two functions following the existing pattern:

```python
def _get_package_swift_release_tag(text):
    """Extract releaseTag version from root Package.swift."""
    m = re.search(r'releaseTag\s*=\s*"(\d+\.\d+\.\d+)"', text)
    return m.group(1) if m else ""


def _sync_package_swift_release_tag(text, version):
    """Update releaseTag version in root Package.swift."""
    return re.sub(
        r'(releaseTag\s*=\s*")\d+\.\d+\.\d+(")',
        rf"\g<1>{version}\2",
        text,
        count=1,
    )
```

Add to TARGETS list:

```python
(("Package.swift", _get_package_swift_release_tag, _sync_package_swift_release_tag),)
```

Also update the module docstring to mention `Package.swift` — add a line like:
`- Package.swift — SPM release tag version`

### Docs update (`docs/howto/swift.md`)

Replace lines 35-59 (the "Build the native library" section with the manual cargo/linker workflow)
with a collapsible "Build from source" tip, following the pattern from `docs/howto/dotnet.md`:

````markdown
??? tip "Build from source"

    To build from source instead of using the prebuilt XCFramework:

    ```bash
    # Clone and build the native library
    git clone https://github.com/iscc/iscc-lib.git
    cd iscc-lib

    # Build the XCFramework (macOS only)
    ./scripts/build_xcframework.sh

    # Or build just the native library for local development
    cargo build -p iscc-uniffi --release
    ```

    Then toggle `useLocalFramework = true` in the root `Package.swift` and reference
    the local package in your project.
````

The existing SPM dependency snippet (lines 16-33) is already correct and should remain as-is. The
key change is removing the "planned for a future release" text and the manual linker instructions,
replacing them with the collapsible build-from-source tip.

## Verification

- `uv run scripts/version_sync.py --check` exits 0 (all 16 targets match, was 15)
- `grep -c "Package.swift" scripts/version_sync.py` returns at least 2 (TARGETS entry + docstring)
- `grep "planned for a future release" docs/howto/swift.md` returns no matches (old text removed)
- `grep "Build from source" docs/howto/swift.md` returns a match (new collapsible section present)
- `mise run check` passes (all pre-commit/pre-push hooks green)

## Done When

All 5 verification criteria pass — version sync covers 16 targets including `Package.swift`
`releaseTag`, and `docs/howto/swift.md` documents zero-friction SPM installation without manual
native library build steps.
