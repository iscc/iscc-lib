# Next Work Package

## Step: Generate Swift bindings and create SPM package structure

## Goal

Create `packages/swift/` with UniFFI-generated Swift bindings, SPM manifest, and XCTest conformance
tests. This establishes the Swift package foundation — the key deliverable of the Swift bindings
issue.

## Scope

- **Create**:
    - `crates/iscc-uniffi/uniffi-bindgen.rs` — 3-line entry point for binding generation
    - `packages/swift/Package.swift` — SPM manifest with IsccLibFFI + IsccLib + test targets
    - `packages/swift/Sources/IsccLibFFI/iscc_uniffiFFI.h` — generated C header (uniffi-bindgen
        output)
    - `packages/swift/Sources/IsccLibFFI/module.modulemap` — module map for FFI target (rename from
        generated `iscc_uniffiFFI.modulemap`)
    - `packages/swift/Sources/IsccLib/iscc_uniffi.swift` — generated Swift bindings (uniffi-bindgen
        output)
    - `packages/swift/Tests/IsccLibTests/ConformanceTests.swift` — XCTest against data.json vectors
    - `packages/swift/Tests/IsccLibTests/data.json` — vendored conformance vectors (copy from
        `crates/iscc-lib/tests/data.json`)
    - `packages/swift/README.md` — per-package README
- **Modify**:
    - `crates/iscc-uniffi/Cargo.toml` — add `[[bin]]` section and `bindgen` feature for CLI
- **Reference**:
    - `crates/iscc-uniffi/src/lib.rs` — UniFFI interface definition (all 32 symbols)
    - `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` — reference conformance test pattern
    - `packages/go/conformance_test.go` — alternate conformance test pattern
    - `.claude/context/specs/swift-bindings.md` — full Swift spec
    - `crates/iscc-lib/tests/data.json` — conformance vectors source

## Not In Scope

- CI job (macOS runner, `swift build` + `swift test`) — separate step after package verified
- Documentation (`docs/howto/swift.md`, README Swift install/quickstart sections)
- Version sync integration (`scripts/version_sync.py` target for Swift)
- XCFramework build for release distribution
- Idiomatic Swift wrapper layer (`IsccLib.swift` with `Codable`/`Sendable`) — the generated UniFFI
    code already provides `camelCase`, `throws`, and reasonable Swift types; extra wrapper can come
    later if needed
- SwiftFormat/SwiftLint integration
- Release workflow updates

## Implementation Notes

### Binding generation mechanism

Add a `bindgen` feature to `iscc-uniffi` that enables `uniffi/cli`, and a `[[bin]]` with
`required-features = ["bindgen"]`:

```toml
[features]
bindgen = ["uniffi/cli"]

[[bin]]
name = "uniffi-bindgen"
path = "uniffi-bindgen.rs"
required-features = ["bindgen"]
```

The binary is trivial:

```rust
fn main() {
    uniffi::uniffi_bindgen_main()
}
```

### Generating Swift code

1. Build the cdylib first: `cargo build -p iscc-uniffi` (produces `target/debug/libiscc_uniffi.so`)
2. Run:
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- generate --library target/debug/libiscc_uniffi.so --language swift --out-dir /tmp/swift-gen/`
3. This generates three files: `iscc_uniffi.swift`, `iscc_uniffiFFI.h`, `iscc_uniffiFFI.modulemap`
4. Place them in the package structure:
    - `iscc_uniffi.swift` → `packages/swift/Sources/IsccLib/`
    - `iscc_uniffiFFI.h` → `packages/swift/Sources/IsccLibFFI/`
    - `iscc_uniffiFFI.modulemap` → rename to `module.modulemap` in
        `packages/swift/Sources/IsccLibFFI/`

### Package.swift structure

```swift
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "IsccLib",
    products: [
        .library(name: "IsccLib", targets: ["IsccLib"]),
    ],
    targets: [
        .target(
            name: "IsccLibFFI",
            path: "Sources/IsccLibFFI",
            publicHeadersPath: ".",
            linkerSettings: [
                .linkedLibrary("iscc_uniffi"),
            ]
        ),
        .target(
            name: "IsccLib",
            dependencies: ["IsccLibFFI"],
            path: "Sources/IsccLib"
        ),
        .testTarget(
            name: "IsccLibTests",
            dependencies: ["IsccLib"],
            path: "Tests/IsccLibTests",
            resources: [.copy("data.json")]
        ),
    ]
)
```

### Conformance tests pattern

Follow the .NET/Go pattern: load `data.json`, iterate per-function groups, decode inputs per
function signature, compare `.iscc` output. Key considerations:

- Use `JSONSerialization` for flexible JSON parsing (like .NET's `JsonElement`)
- `"stream:<hex>"` prefix: extract hex after prefix, convert to `Data`
- `gen_iscc_code_v0` vectors have no `wide` parameter — always pass `false`
- `_metadata` key in data.json: skip non-vector top-level keys
- Empty `description`/`meta` strings → pass `nil` (not empty string)
- 9 test functions covering 50 vectors (no `gen_sum_code_v0` vectors in data.json)
- Test method per gen function (9 methods) — each iterates its vectors and asserts `.iscc` match

### data.json note

Copy from `crates/iscc-lib/tests/data.json`. This is the same file vendored by Go and .NET. All
copies must stay identical.

## Verification

- `cargo build -p iscc-uniffi` compiles successfully (existing crate unaffected)
- `cargo test -p iscc-uniffi` passes (21 existing tests)
- `cargo clippy -p iscc-uniffi -- -D warnings` is clean
- `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- --help` shows usage
    (uniffi-bindgen binary works)
- `packages/swift/Package.swift` exists
- `packages/swift/Sources/IsccLib/iscc_uniffi.swift` exists and contains `genMetaCodeV0`
- `packages/swift/Sources/IsccLibFFI/iscc_uniffiFFI.h` exists
- `packages/swift/Sources/IsccLibFFI/module.modulemap` exists
- `packages/swift/Tests/IsccLibTests/ConformanceTests.swift` exists
- `packages/swift/Tests/IsccLibTests/data.json` exists and is identical to
    `crates/iscc-lib/tests/data.json`

## Done When

All verification criteria pass — the Swift package structure is complete with generated bindings,
conformance tests, and vendored vectors, ready for CI validation on a macOS runner.
