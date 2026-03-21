## 2026-03-21 — Add Swift version sync to version_sync.py

**Done:** Created `Constants.swift` with a `public let isccLibVersion = "0.3.1"` constant, and added
it as a sync target in `scripts/version_sync.py` with `_get_swift_version` / `_sync_swift_version`
helper functions. The Swift package version now stays in sync with the workspace version from root
Cargo.toml.

**Files changed:**

- `packages/swift/Sources/IsccLib/Constants.swift`: New file — Swift version constant
- `scripts/version_sync.py`: Added Swift sync target (getter, sync fn, TARGETS entry, docstring)

**Verification:** All criteria pass:

- `test -f packages/swift/Sources/IsccLib/Constants.swift` exits 0
- `grep -q '0.3.1' packages/swift/Sources/IsccLib/Constants.swift` exits 0
- `grep -q 'swift' scripts/version_sync.py` exits 0
- `uv run scripts/version_sync.py --check` exits 0 (all 14 targets in sync)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean
- `mise run check` passes (15/15 hooks)

**Next:** The Swift bindings issue can now be closed — all sub-tasks (UniFFI crate, Swift package,
CI, documentation, version sync) are complete. The next priority issue is Kotlin bindings.

**Notes:** Simple, clean implementation. Used top-level `public let isccLibVersion` (matching the
UniFFI free-function style) rather than an enum namespace — simpler and consistent with the project
pattern. The regex matches `isccLibVersion = "X.Y.Z"` which is specific enough to avoid false
matches with the UniFFI-generated code.
