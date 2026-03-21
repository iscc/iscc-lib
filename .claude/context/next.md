# Next Work Package

## Step: Create XCFramework build script and restructure root Package.swift

## Goal

Create the foundational infrastructure for Swift XCFramework distribution: a portable build script
that cross-compiles for all Apple platforms and assembles an XCFramework, plus a restructured root
`Package.swift` with the Ferrostar-style variable toggle for local/remote binary targets. This
addresses the only `normal` priority issue (Swift package does not vend native library).

## Scope

- **Create**: `scripts/build_xcframework.sh`
- **Modify**: `Package.swift` (root)
- **Reference**:
    - `.claude/context/specs/swift-bindings.md` — comprehensive spec with build script pseudocode,
        Package.swift template, and architecture decisions
    - `packages/swift/Package.swift` — development manifest (read-only, do NOT modify)
    - `packages/swift/Sources/` — verify directory structure for target paths

## Not In Scope

- Modifying `packages/swift/Package.swift` (development manifest used by CI — must stay unchanged)
- Updating `release.yml` or `ci.yml` workflows (separate step)
- Updating `docs/howto/swift.md` (separate step after XCFramework is verified working)
- Adding version sync entries for `releaseTag`/`releaseChecksum` to `version_sync.py` (separate
    step)
- Fixing the wrong reproduction path in `docs/benchmarks.md` (unrelated)
- Actually building or testing the XCFramework (requires macOS — will be verified in CI)

## Implementation Notes

### Build Script (`scripts/build_xcframework.sh`)

Follow the spec pseudocode in `swift-bindings.md` precisely. Key requirements:

1. **Shebang + strict mode**: `#!/bin/bash` with `set -euo pipefail`
2. **5 Rust targets**: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `aarch64-apple-ios`,
    `aarch64-apple-ios-sim`, `x86_64-apple-ios`
3. **Profile handling**: Accept `--release` flag (default to release). Use `PROFILE_DIR` variable
    for path selection (`release` vs `debug`)
4. **`cargo build -p iscc-uniffi`** for each target (not `iscc-lib` — the UniFFI crate produces
    `libiscc_uniffi.a`)
5. **Fat binaries via `lipo`**: macOS (arm64 + x86_64), iOS simulator (arm64 + x86_64)
6. **Header staging**: Copy `iscc_uniffiFFI.h` and `module.modulemap` from
    `packages/swift/Sources/iscc_uniffiFFI/` to a staging directory. The headers dir must have the
    header file and a modulemap for `xcodebuild -create-xcframework`
7. **`xcodebuild -create-xcframework`**: 3 library slices (macOS fat, iOS device, iOS simulator
    fat) with headers
8. **`ditto`** for zipping (not `zip`) — preserves resource forks
9. **`swift package compute-checksum`** — print the SHA256 checksum at the end
10. **Output path**: `target/ios/IsccLib.xcframework` and `target/ios/IsccLib.xcframework.zip`
11. **Clean before build**: Remove existing XCFramework output directory before creating new one

### Root Package.swift

Follow the Ferrostar-pattern template from the spec:

1. **`let useLocalFramework = false`** — variable toggle at the top
2. **`platforms: [.macOS(.v13), .iOS(.v16)]`** — minimum deployment targets
3. **Three targets**:
    - `binaryTarget` (conditional): `useLocalFramework` →
        `.binaryTarget(name: "iscc_uniffiFFI", path: "target/ios/IsccLib.xcframework")`, else →
        `.binaryTarget(name: "iscc_uniffiFFI", url: "https://github.com/iscc/iscc-lib/releases/download/v\(releaseTag)/IsccLib.xcframework.zip", checksum: releaseChecksum)`
    - `.target(name: "IsccLib", dependencies: ["iscc_uniffiFFI"], path: "packages/swift/Sources/IsccLib")`
4. **`releaseTag` and `releaseChecksum`** variables with placeholder values (e.g., `"0.3.1"` and
    `"PLACEHOLDER"`)
5. **Single product**: `.library(name: "IsccLib", targets: ["IsccLib"])`

Key difference from current Package.swift: the `iscc_uniffiFFI` target is now a `.binaryTarget` (not
a `.target` with `.linkedLibrary`), so it no longer has `publicHeadersPath` or `linkerSettings` —
those are baked into the XCFramework.

## Verification

- `test -x scripts/build_xcframework.sh` exits 0 (script exists and is executable)
- `bash -n scripts/build_xcframework.sh` exits 0 (valid shell syntax)
- `grep -q 'aarch64-apple-darwin' scripts/build_xcframework.sh` exits 0 (contains macOS arm64
    target)
- `grep -q 'x86_64-apple-ios' scripts/build_xcframework.sh` exits 0 (contains iOS simulator x86_64
    target)
- `grep -q 'xcodebuild -create-xcframework' scripts/build_xcframework.sh` exits 0
- `grep -q 'ditto' scripts/build_xcframework.sh` exits 0
- `grep -q 'swift package compute-checksum' scripts/build_xcframework.sh` exits 0
- `grep -q 'useLocalFramework' Package.swift` exits 0 (variable toggle present)
- `grep -q 'binaryTarget' Package.swift` exits 0 (uses binary target)
- `grep -q 'releaseTag' Package.swift` exits 0 (release tag variable present)
- `grep -q 'releaseChecksum' Package.swift` exits 0 (checksum variable present)
- `grep -q '.macOS' Package.swift` exits 0 (platform constraints present)
- `diff packages/swift/Package.swift <(git show HEAD:packages/swift/Package.swift)` exits 0
    (development Package.swift unchanged)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (Rust code unaffected)
- `cargo test -p iscc-lib` passes (core unaffected)

## Done When

All verification criteria pass — the build script and root Package.swift are correctly structured
following the spec, the development Package.swift is unchanged, and all existing tests continue to
pass.
