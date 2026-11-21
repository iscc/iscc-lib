# Spec: Swift Bindings — Prebuilt XCFramework via SPM

A Swift package providing idiomatic Swift access to all ISCC functions via UniFFI-generated
bindings. Distributed via Swift Package Manager (SPM) using Git tags with a prebuilt XCFramework —
consumers need only add the SPM dependency, no Rust toolchain or manual library management required.
Shares the UniFFI scaffolding crate (`crates/iscc-uniffi/`) with Kotlin bindings.

**Reference model:** [Ferrostar](https://github.com/stadiamaps/ferrostar) — production-proven
monorepo pattern with UniFFI, variable toggle in `Package.swift`, force-update tag for checksum.
Also informed by Matrix/Element, Mozilla application-services, BDK, Sparkle, and Firebase XCF.

## Core DX Goal

```swift
// Consumer's Package.swift — this is all that's needed
dependencies: [
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.5.0"),
]
```

Zero-friction: `swift package resolve` fetches the prebuilt XCFramework from GitHub Releases. No
Rust, no `-Xlinker` flags, no build-from-source steps. Matches the DX of every other binding (Python
wheels, Ruby gems, npm packages, Maven JARs, NuGet packages).

## Architecture

**Three-layer target structure** (industry standard for UniFFI Swift packages):

1. **Binary target** (`.binaryTarget`): Prebuilt Rust static libraries in an XCFramework, downloaded
    from GitHub Releases.
2. **FFI bridge target** (`.target`): UniFFI-generated Swift source + C header + modulemap. Depends
    on the binary target.
3. **Public API target** (`.target`): The `IsccLib` module consumers import. Depends on the FFI
    bridge target.

**Why UniFFI (not swift-bridge or manual C FFI):**

- Production-proven by Mozilla (Firefox for iOS/Android), Ferrostar, Matrix/Element, BDK
- One scaffolding crate generates both Swift and Kotlin — two-for-one
- Handles memory management, error propagation, and type marshalling automatically
- Supports `#[uniffi::Object]` for streaming hashers with proper reference counting

## Package Structure

```
Package.swift (root)                    # Single manifest for both dev and distribution
packages/swift/
├── Package.swift                       # Development-only manifest (CI tests use this)
├── Sources/
│   ├── IsccLib/
│   │   ├── iscc_uniffi.swift           # UniFFI-generated Swift bindings (DO NOT EDIT)
│   │   └── Constants.swift             # Version constant for diagnostics
│   └── iscc_uniffiFFI/
│       ├── iscc_uniffiFFI.h            # UniFFI-generated C header (DO NOT EDIT)
│       └── module.modulemap            # Simplified modulemap for SPM
├── Tests/
│   └── IsccLibTests/
│       ├── ConformanceTests.swift      # data.json conformance vectors
│       └── data.json                   # Vendored ISCC conformance test vectors
└── README.md
scripts/
└── build_xcframework.sh                # Portable build script (macOS + CI)
```

The tree lists the load-bearing entries, not every file.

## Root Package.swift — Variable Toggle Pattern

The root `Package.swift` serves both development and distribution using a variable toggle, following
the pattern established by Ferrostar, YSwift, and Automerge:

```swift
// swift-tools-version: 5.9
import PackageDescription

let useLocalFramework = false

let binaryTarget: Target
if useLocalFramework {
    binaryTarget = .binaryTarget(
        name: "iscc_uniffiFFI",
        path: "target/ios/IsccLib.xcframework"
    )
} else {
    let releaseTag = "0.5.0"
    let releaseChecksum = "abc123..."
    binaryTarget = .binaryTarget(
        name: "iscc_uniffiFFI",
        url: "https://github.com/iscc/iscc-lib/releases/download/v\(releaseTag)/IsccLib.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "IsccLib",
    platforms: [.macOS(.v13), .iOS(.v16)],
    products: [
        .library(name: "IsccLib", targets: ["IsccLib"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "IsccLib",
            dependencies: ["iscc_uniffiFFI"],
            path: "packages/swift/Sources/IsccLib"
        ),
    ]
)
```

**Why a variable toggle (not two files):**

- Single source of truth — one `Package.swift` for SPM resolution
- `useLocalFramework = true` for local development on macOS (after `cargo build -p iscc-uniffi`)
- `useLocalFramework = false` (default) for consumers — fetches prebuilt XCFramework
- `releaseTag` and `releaseChecksum` updated by CI during release via `sed`

**Development Package.swift** (`packages/swift/Package.swift`) is retained for CI tests — it uses
`.linkedLibrary("iscc_uniffi")` with `-Xlinker` flags, which is faster for iterating on the bindings
without building a full XCFramework.

## Idiomatic Swift API

```swift
import IsccLib

// Code generation
let result = try genMetaCodeV0(name: "Title", description: "A description", meta: nil, bits: 64)
print(result.iscc)       // "ISCC:..."
print(result.name)       // "Title"

// Streaming
let hasher = DataHasher()
hasher.update(data: chunk1)
hasher.update(data: chunk2)
let code = try hasher.finalize(bits: 64)

// Conformance selftest
assert(conformanceSelftest())
```

### Swift Conventions

- `camelCase` method names (auto-converted by UniFFI)
- `throws` for fallible functions (mapped from `Result<T, IsccUniError>`)
- `Data` for binary data (converted from `Vec<u8>`)
- Named parameters without defaults: the UniFFI exports declare no `default(...)`, so Swift and
    Kotlin callers pass every argument, e.g. `genMetaCodeV0(name:description:meta:bits:)`
- Constants are getter functions (UniFFI limitation): `metaTrimName()`, `ioReadSize()`, etc.

## XCFramework Build

Prebuilt static libraries for Apple platforms, assembled via `xcodebuild -create-xcframework`.

### Target Platforms

| Platform        | Rust Target                                    | Architecture       |
| --------------- | ---------------------------------------------- | ------------------ |
| macOS           | `aarch64-apple-darwin` + `x86_64-apple-darwin` | arm64 + x86_64 fat |
| iOS (device)    | `aarch64-apple-ios`                            | arm64              |
| iOS (simulator) | `aarch64-apple-ios-sim` + `x86_64-apple-ios`   | arm64 + x86_64 fat |

The Rust targets are merged into one slice per platform row via `lipo`.

### Build Script (`scripts/build_xcframework.sh`)

Encapsulates the complete XCFramework build pipeline. Runs on local macOS and in CI:

```bash
#!/bin/bash
set -euo pipefail
# Usage: ./scripts/build_xcframework.sh [--release]
# Output: target/ios/IsccLib.xcframework.zip
# Prints: SHA256 checksum (via swift package compute-checksum)

PROFILE="${1:---release}"
TARGETS=(
    aarch64-apple-darwin
    x86_64-apple-darwin
    aarch64-apple-ios
    aarch64-apple-ios-sim
    x86_64-apple-ios
)

# 1. Cross-compile libiscc_uniffi.a for all targets
for target in "${TARGETS[@]}"; do
    cargo build -p iscc-uniffi $PROFILE --target "$target"
done

# 2. Create fat binaries with lipo
lipo -create \
    target/aarch64-apple-darwin/release/libiscc_uniffi.a \
    target/x86_64-apple-darwin/release/libiscc_uniffi.a \
    -output target/macos-fat/libiscc_uniffi.a

lipo -create \
    target/aarch64-apple-ios-sim/release/libiscc_uniffi.a \
    target/x86_64-apple-ios/release/libiscc_uniffi.a \
    -output target/ios-simulator-fat/libiscc_uniffi.a

# 3. Stage headers + modulemap for each slice
# (copy iscc_uniffiFFI.h + module.modulemap to staging dirs)

# 4. Assemble XCFramework
xcodebuild -create-xcframework \
    -library target/macos-fat/libiscc_uniffi.a \
    -headers target/xcframework-staging/headers \
    -library target/aarch64-apple-ios/release/libiscc_uniffi.a \
    -headers target/xcframework-staging/headers \
    -library target/ios-simulator-fat/libiscc_uniffi.a \
    -headers target/xcframework-staging/headers \
    -output target/ios/IsccLib.xcframework

# 5. Zip with ditto (preserves resource forks, standard for macOS)
ditto -c -k --sequesterRsrc --keepParent \
    target/ios/IsccLib.xcframework \
    target/ios/IsccLib.xcframework.zip

# 6. Compute checksum using Swift's standard tool
swift package compute-checksum target/ios/IsccLib.xcframework.zip
```

**Key tool choices:**

- **`ditto`** for zipping (not `zip`) — macOS standard, preserves resource forks, used by Ferrostar,
    Sparkle, and Apple's own tooling
- **`swift package compute-checksum`** for SHA256 — the canonical tool SPM itself uses for
    verification; ensures hash matches what SPM expects
- **`lipo`** for fat binaries — standard Apple tool for merging architectures

## Release Process — Force-Update Tag Pattern

The checksum-before-tag problem: SPM reads `Package.swift` from the tag commit, so the checksum must
be in the tag. But the checksum requires the built artifact, which requires the code to be final.
Every production UniFFI-to-Swift project solves this. The cleanest solution for monorepos is
Ferrostar's **force-update tag** approach.

### Release Flow

1. **Human dispatches `release.yml`** with a `version` input (e.g. `0.5.0`); `workflow_dispatch` is
    its only trigger and no tag exists yet
2. **`prepare-release` checks out `main`** and verifies that the root `Cargo.toml` version matches
    the input
3. **macOS job builds XCFramework** — runs `build_xcframework.sh --release` (skipped on a cache hit)
4. **`sed` updates `Package.swift`** — writes the input version to `releaseTag` and the
    `swift package compute-checksum` result to `releaseChecksum`
5. **Auto-commit** the updated `Package.swift` and push it to `main` (skipped when unchanged)
6. **Tag the checksum commit** and force-push the tags, so a stale remote tag of the same name is
    overwritten:
    ```bash
    git tag "v0.5.0"
    git tag "packages/go/v0.5.0"
    git push origin "v0.5.0" "packages/go/v0.5.0" --force
    ```
7. **Create the GitHub Release** for `v0.5.0` with `IsccLib.xcframework.zip` attached

Because the tag is created after the checksum commit, it points to a commit with the correct
checksum, and the release asset URL is deterministic from the tag name. SPM consumers see a
self-consistent state.

### Release Workflow Integration

The `prepare-release` job of `release.yml` (macos-15) implements the flow above. It runs only when
the `version` input is set: there is no separate `swift` checkbox, and a registry-only re-trigger
does not rebuild the XCFramework. Each registry chain starts with a job that has
`needs: [prepare-release]`, so the tags exist before any publish job starts.

## Caching Strategy

XCFramework builds are expensive (one cross-compilation per Rust target on a macOS runner). Cache
aggressively.

### CI Cache

```yaml
  - uses: actions/cache@v4
    id: xcf-cache
    with:
      path: IsccLib.xcframework.zip
      key: xcf-${{ hashFiles( 'crates/iscc-*/src/**', 'Cargo.lock') }}
```

**Cache hit** — skip the entire native build, use the cached zip directly.

**Cache miss** — full rebuild. Also use `Swatinem/rust-cache@v2` for Rust compilation caching
(incremental builds if only a few files changed).

### What Invalidates the Cache

- Any change to `crates/iscc-lib/src/**` (core library code)
- Any change to `crates/iscc-uniffi/src/**` (UniFFI interface)
- Any change to `Cargo.lock` (dependency versions)

### What Does NOT Invalidate the Cache

- Changes to Swift source files (`packages/swift/Sources/**`)
- Changes to tests (`packages/swift/Tests/**`)
- Changes to other binding crates
- Documentation or CI config changes

### Kotlin Artifact Reuse

The Kotlin release workflow already builds `libiscc_uniffi` for macOS targets. However, Kotlin needs
shared libraries (`cdylib`) while Swift XCFramework needs static libraries (`staticlib`). Keep the
builds separate for clarity and independence — the cache eliminates redundant work.

## CI Integration

### CI Workflow (Quality Gate)

The existing `swift` job in `ci.yml` continues to use the development `Package.swift` with
`.linkedLibrary` — it builds `iscc-uniffi` from source and runs `swift test` with `-Xlinker` flags.
This validates that the Swift bindings and tests are correct.

```yaml
# ci.yml — unchanged, uses development Package.swift
swift:
  runs-on: macos-15
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    - run: cargo build -p iscc-uniffi
    - run: swift build -Xlinker -L${{ github.workspace }}/target/debug
      working-directory: packages/swift
    - run: >-
        swift test
        -Xlinker -L${{ github.workspace }}/target/debug
        -Xlinker -rpath -Xlinker ${{ github.workspace }}/target/debug
      working-directory: packages/swift
```

## Binding Generation

The Swift bindings are generated from the `iscc-uniffi` crate using UniFFI's proc macro approach:

```bash
# Build the library first
cargo build -p iscc-uniffi

# Generate Swift bindings
cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- \
    generate --library target/debug/libiscc_uniffi.so \
    --language swift --out-dir packages/swift/Sources/IsccLib/
```

Generated files: `iscc_uniffi.swift` (Swift source), `iscc_uniffiFFI.h` (C header),
`iscc_uniffiFFI.modulemap` (module map — must be simplified for SPM by removing Darwin-specific
`use` directives).

## Version Sync

| Target                                           | What is synced                 |
| ------------------------------------------------ | ------------------------------ |
| `packages/swift/Sources/IsccLib/Constants.swift` | `static let version = "X.Y.Z"` |
| `Package.swift` (root)                           | `releaseTag` variable          |

The `releaseChecksum` in root `Package.swift` is updated by CI during the release workflow (not by
`version:sync`), since the checksum depends on the built artifact.

## Code Quality

- **SwiftFormat**: Opinionated formatting (matches project pattern of cargo fmt, ruff, biome)
- **SwiftLint**: Additional code quality checks
- Pre-commit: `swiftformat` auto-fix; pre-push: `swiftformat --lint` + `swift test`

## Documentation

- **How-to guide**: `docs/howto/swift.md`
- **Per-package README**: `packages/swift/README.md`

## Account Setup Required

No registry account needed — SPM pulls from the public Git repository. XCFramework hosted as GitHub
Release asset (uses existing `contents: write` permission).

## Verification Criteria

- `swift test` passes conformance vectors (CI, development Package.swift)
- Package resolves via SPM from Git tag using `.binaryTarget` (no Rust toolchain needed)
- All `gen_*_v0` functions accessible with idiomatic Swift types
- All Tier 1 symbols accessible from Swift
- XCFramework bundles static libraries for macOS (arm64 + x86_64), iOS device (arm64), iOS simulator
    (arm64 + x86_64)
- Root `Package.swift` uses variable toggle (`useLocalFramework`) with `releaseTag` +
    `releaseChecksum` for remote mode
- XCFramework build cached by `actions/cache` — cache hit skips native compilation
- Cache key covers `crates/iscc-uniffi/**`, `crates/iscc-lib/**`, `Cargo.lock`
- `build_xcframework.sh` script works on local macOS and in CI
- XCFramework zip uploaded as GitHub Release asset named `IsccLib.xcframework.zip`
- Release workflow force-updates tag after updating checksum in `Package.swift`
- Release workflow includes `swift` checkbox in `workflow_dispatch` inputs
- `swift package compute-checksum` used for checksum (not raw sha256sum)
- `ditto -c -k --sequesterRsrc --keepParent` used for zipping
- `DataHasher` / `InstanceHasher` streaming types work correctly
- `throws` error handling works for fallible functions
- Binary data uses Swift `Data` type
- `swiftformat --lint` passes
- Swift CI runs on macOS runner (`macos-15`)
- Version synced from root `Cargo.toml` via `mise run version:sync`
- `conformanceSelftest()` returns `true`

## Reference Projects

| Project                                                                        | Pattern                                     | Relevance                 |
| ------------------------------------------------------------------------------ | ------------------------------------------- | ------------------------- |
| [Ferrostar](https://github.com/stadiamaps/ferrostar)                           | Monorepo, variable toggle, force-update tag | Primary model             |
| [Matrix/Element](https://github.com/element-hq/matrix-rust-components-swift)   | Separate repo, CI rewrites Package.swift    | Checksum automation       |
| [BDK](https://github.com/bitcoindevkit/bdk-swift)                              | Separate repo, cross-repo CI                | Release orchestration     |
| [Sparkle](https://github.com/sparkle-project/Sparkle)                          | Same repo, GitHub Releases                  | Template-based generation |
| [Mozilla](https://github.com/mozilla/rust-components-swift)                    | Separate repo, wrapper targets              | Platform conditions       |
| [Firebase XCF](https://github.com/akaffenberger/firebase-ios-sdk-xcframeworks) | Separate repo, 70+ binary targets           | Scale patterns            |
