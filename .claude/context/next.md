# Next Work Package

## Step: Add Swift version sync to version_sync.py

## Goal

Create a `Constants.swift` file with a `VERSION` string constant and add it as a sync target in
`scripts/version_sync.py`, so the Swift package version stays in sync with releases — the last
remaining gap before the Swift bindings issue can be closed.

## Scope

- **Create**: `packages/swift/Sources/IsccLib/Constants.swift`
- **Modify**: `scripts/version_sync.py`
- **Reference**: `crates/iscc-rb/lib/iscc_lib/version.rb` (Ruby version constant pattern),
    `scripts/version_sync.py` (existing sync target patterns)

## Not In Scope

- Updating `Package.swift` — SPM versioning uses Git tags, not a manifest field
- Closing the Swift issue in issues.md — the review agent handles that after verification
- Starting Kotlin bindings work — that's the next issue after Swift is fully done
- Updating `packages/swift/CLAUDE.md` — the version sync pitfall note already exists and can be
    refined in a future step if needed

## Implementation Notes

1. **`Constants.swift`** — create at `packages/swift/Sources/IsccLib/Constants.swift`:

    - Define a `public let isccLibVersion = "0.3.1"` (or `public static let version` on a namespace
        enum)
    - Follow Swift conventions: use `let` for immutable values, public access
    - A simple pattern like the Ruby `VERSION = "0.3.1"` but idiomatic Swift
    - Consider using an enum namespace:
        `public enum IsccLibConstants { public static let version =   "0.3.1" }` — but a top-level
        `public let isccLibVersion` is simpler and consistent with the UniFFI free-function style
        used throughout

2. **`version_sync.py`** — add a new sync target:

    - Create `_get_swift_version(text)` that extracts the version from the `Constants.swift` file
        using a regex matching the version string pattern
    - Create `_sync_swift_version(text, version)` that updates the version string
    - Add the target tuple to the `TARGETS` list:
        `("packages/swift/Sources/IsccLib/Constants.swift", _get_swift_version, _sync_swift_version)`
    - Update the module docstring to include the Swift target in the "Synced targets" list
    - Pattern to follow: the Ruby version sync (`_get_ruby_version` / `_sync_ruby_version`) is the
        closest analog — both extract a version string from a simple constant declaration

3. **Version format** — use a regex that matches `"X.Y.Z"` (3-component semver) like all other sync
    targets. The exact regex depends on the chosen Swift syntax:

    - For `let isccLibVersion = "0.3.1"`: regex `r'=\s*"(\d+\.\d+\.\d+)"'`
    - Keep it specific enough to avoid false matches with the UniFFI-generated code's own version
        references

## Verification

- `test -f packages/swift/Sources/IsccLib/Constants.swift` exits 0
- `grep -q '0.3.1' packages/swift/Sources/IsccLib/Constants.swift` exits 0
- `grep -q 'swift' scripts/version_sync.py` exits 0 (Swift target present)
- `uv run scripts/version_sync.py --check` exits 0 (all targets including Swift are in sync)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean
- `mise run check` passes (all pre-commit/pre-push hooks)

## Done When

All verification criteria pass — the Swift version constant exists, is synced by version_sync.py,
and all quality gates remain green.
