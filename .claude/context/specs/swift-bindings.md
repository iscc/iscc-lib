# Spec: Swift Bindings — UniFFI-Generated Swift Package

A Swift package providing idiomatic Swift access to all ISCC functions via UniFFI-generated
bindings. Distributed via Swift Package Manager (SPM) using Git tags. Shares a UniFFI scaffolding
crate (`crates/iscc-uniffi/`) with the Kotlin Multiplatform bindings.

## Architecture

**Three-layer design:**

1. **UniFFI scaffolding crate** (`crates/iscc-uniffi/`): Rust crate with `#[uniffi::export]` proc
    macros defining the shared interface for both Swift and Kotlin. Compiles to `cdylib` +
    `staticlib`.
2. **Generated Swift code** (`packages/swift/Sources/IsccLib/Generated/`): `uniffi-bindgen`
    generates Swift FFI bridge code from the scaffolding crate.
3. **Idiomatic Swift wrapper** (`packages/swift/Sources/IsccLib/IsccLib.swift`): Thin layer adding
    Swift conventions — `camelCase`, `throws`, `Data` types, `Codable` conformance.

**Why UniFFI (not swift-bridge or manual C FFI):**

- Production-proven by Mozilla (Firefox for iOS/Android)
- One scaffolding crate generates both Swift and Kotlin — two-for-one
- Handles memory management, error propagation, and type marshalling automatically
- Supports `#[uniffi::Object]` for streaming hashers with proper reference counting

## Shared UniFFI Crate

```
crates/iscc-uniffi/
├── Cargo.toml              # cdylib + staticlib, depends on iscc-lib + uniffi
├── src/
│   └── lib.rs              # UniFFI interface (~600-800 lines)
├── uniffi.toml             # Binding generation config
└── build.rs                # uniffi::generate_scaffolding()
```

### UniFFI Interface Definition

Uses proc macros (not UDL files) for simpler maintenance:

```rust
#[derive(uniffi::Error)]
pub enum IsccUniError {
    IsccError { msg: String },
}

#[derive(uniffi::Record)]
pub struct MetaCodeResult {
    pub iscc: String,
    pub name: String,
    pub metahash: String,
    pub description: Option<String>,
    pub meta: Option<String>,
}

#[uniffi::export]
pub fn gen_meta_code_v0(
    name: String, description: Option<String>,
    meta: Option<String>, bits: u32,
) -> Result<MetaCodeResult, IsccUniError> {
    let result = iscc_lib::gen_meta_code_v0(&name, ...)?;
    Ok(MetaCodeResult { /* map fields */ })
}

#[derive(uniffi::Object)]
pub struct DataHasher {
    inner: std::sync::Mutex<Option<iscc_lib::DataHasher>>,
}

#[uniffi::export]
impl DataHasher {
    #[uniffi::constructor]
    pub fn new() -> Self { ... }
    pub fn update(&self, data: Vec<u8>) { ... }
    pub fn finalize(&self, bits: u32) -> Result<DataCodeResult, IsccUniError> { ... }
}
```

All 32 Tier 1 symbols are exposed. Constants use getter functions (UniFFI doesn't support `const`
exports directly).

## Package Structure

```
packages/swift/
├── Package.swift                       # SPM manifest
├── Sources/
│   └── IsccLib/
│       ├── Generated/                  # uniffi-bindgen output
│       │   ├── iscc_uniffi.swift
│       │   └── iscc_uniffiFFI.h
│       ├── IsccLib.swift               # Idiomatic Swift wrapper
│       └── Constants.swift             # Swift-idiomatic constant properties
├── Tests/
│   └── IsccLibTests/
│       ├── ConformanceTests.swift      # XCTest against data.json
│       └── data.json                   # Vendored conformance vectors
├── IsccLib.xcframework/               # Pre-built binary framework (release)
└── README.md                           # Per-package README
```

## Idiomatic Swift API

```swift
import IsccLib

// Code generation
let result = try IsccLib.genMetaCodeV0(name: "Title", description: "A description")
print(result.iscc)       // "ISCC:..."
print(result.name)       // "Title"

// Streaming
let hasher = DataHasher()
hasher.update(chunk1)
hasher.update(chunk2)
let code = try hasher.finalize(bits: 64)

// ISCC-SUM from file
let sum = try IsccLib.genSumCodeV0(path: "/path/to/file.bin")

// Conformance selftest
assert(IsccLib.conformanceSelftest())
```

### Swift Conventions

- `camelCase` method names (auto-converted by UniFFI)
- `throws` for fallible functions (mapped from `Result<T, IsccUniError>`)
- `Data` for binary data (converted from `Vec<u8>`)
- `Codable` + `Sendable` conformance on result types
- Named parameters with defaults: `genMetaCodeV0(name:description:meta:bits:)`

## XCFramework Build

Pre-built static libraries for Apple platforms, assembled via `xcodebuild -create-xcframework`:

| Platform        | Rust Target                                    | Architecture       |
| --------------- | ---------------------------------------------- | ------------------ |
| iOS (device)    | `aarch64-apple-ios`                            | arm64              |
| iOS (simulator) | `aarch64-apple-ios-sim` + `x86_64-apple-ios`   | arm64 + x86_64 fat |
| macOS           | `aarch64-apple-darwin` + `x86_64-apple-darwin` | arm64 + x86_64 fat |

## Distribution

Distributed via **Swift Package Manager (Git tags)** — no upload registry.

```swift
// Consumer's Package.swift
dependencies: [
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.1.0"),
]
```

SPM resolves the package directly from the Git repository. The `Package.swift` references the
XCFramework as a binary target.

## CI Integration

Requires **macOS runner** for XCFramework building and `swift test`:

```yaml
swift:
  runs-on: macos-14
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: aarch64-apple-darwin,x86_64-apple-darwin
    - run: cargo build -p iscc-uniffi --release
    - run: >-
        cargo run -p iscc-uniffi -- generate --language swift
        --out-dir packages/swift/Sources/IsccLib/Generated/
    - run: swift test
      working-directory: packages/swift
```

## Code Quality

- **SwiftFormat**: Opinionated formatting (matches project pattern of cargo fmt, ruff, biome)
- **SwiftLint**: Additional code quality checks
- Pre-commit: `swiftformat` auto-fix; pre-push: `swiftformat --lint` + `swift test`

## Version Sync

| Target                                           | What is synced                 |
| ------------------------------------------------ | ------------------------------ |
| `packages/swift/Sources/IsccLib/Constants.swift` | `static let version = "X.Y.Z"` |

SPM version is determined by Git tags. Embedded version constant for diagnostics.

## Documentation

- **How-to guide**: `docs/howto/swift.md`
- **Per-package README**: `packages/swift/README.md`

## Account Setup Required

No registry account needed — SPM pulls from the public Git repository.

## Verification Criteria

- [ ] `swift test` passes conformance vectors
- [ ] Package resolves via SPM from Git tag
- [ ] All 10 `gen_*_v0` functions accessible with idiomatic Swift types
- [ ] All 32 Tier 1 symbols accessible from Swift
- [ ] Works on iOS and macOS targets
- [ ] `DataHasher` / `InstanceHasher` streaming types work correctly
- [ ] `throws` error handling works for fallible functions
- [ ] Binary data uses Swift `Data` type
- [ ] XCFramework builds for iOS (device + simulator) and macOS
- [ ] Result types conform to `Codable` and `Sendable`
- [ ] `swiftformat --lint` passes
- [ ] Swift CI runs on macOS runner (`macos-14`)
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
- [ ] `conformanceSelftest()` returns `true`
