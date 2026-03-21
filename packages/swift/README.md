# IsccLib — Swift Package

Swift bindings for [iscc-lib](https://github.com/iscc/iscc-lib), a high-performance implementation
of [ISO 24138:2024](https://www.iso.org/standard/77899.html) International Standard Content Code
(ISCC).

## Installation

Add the package dependency to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.3.0"),
]
```

Then add `"IsccLib"` to your target's dependencies.

## Usage

```swift
import IsccLib

// Generate a Meta-Code
let meta = try genMetaCodeV0(name: "Title", description: "A description", meta: nil, bits: 64)
print(meta.iscc)  // "ISCC:..."

// Generate a Data-Code
let data = Data([0x68, 0x65, 0x6c, 0x6c, 0x6f])
let dataCode = try genDataCodeV0(data: data, bits: 64)
print(dataCode.iscc)

// Streaming Data-Code for large files
let hasher = DataHasher()
hasher.update(data: chunk1)
hasher.update(data: chunk2)
let result = try hasher.finalize(bits: 64)

// Conformance self-test
assert(conformanceSelftest())
```

## Requirements

- Swift 5.9+
- macOS 13+ / iOS 16+
- Pre-built `libiscc_uniffi` static library (XCFramework)

## Building from Source

The Swift package requires the UniFFI-generated native library. To build from source:

```bash
# Build the Rust library
cargo build -p iscc-uniffi --release

# Generate Swift bindings
cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- \
    generate --library target/release/libiscc_uniffi.so \
    --language swift --out-dir packages/swift/Sources/IsccLib/

# Run tests (requires libiscc_uniffi in library path)
cd packages/swift
swift test -Xlinker -L../../target/release
```

## License

Licensed under the Apache License, Version 2.0. See
[LICENSE](https://github.com/iscc/iscc-lib/blob/main/LICENSE) for details.
