# IsccLib — Swift Package

Swift bindings for [iscc-lib](https://github.com/iscc/iscc-lib), a high-performance implementation
of [ISO 24138:2024](https://www.iso.org/standard/77899.html) International Standard Content Code
(ISCC).

## Installation

Add the package dependency to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.4.0"),
]
```

Then add `"IsccLib"` to your target's dependencies.

> **Note:** The native `libiscc_uniffi` library must be built from source before building your
> project. See [Building from Source](#building-from-source) below for instructions. Pre-built
> XCFramework distribution is planned for a future release.

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
- Rust toolchain (for building the native `libiscc_uniffi` library from source)

## Building from Source

The Swift package requires the native `libiscc_uniffi` library. Build it from the repository root:

```bash
# 1. Build the Rust native library
cargo build -p iscc-uniffi --release

# 2. Build the Swift package (link against the native library)
cd packages/swift
swift build -Xlinker -L../../target/release

# 3. Run tests
swift test \
    -Xlinker -L../../target/release \
    -Xlinker -rpath -Xlinker ../../target/release
```

For your own project, point the linker at the directory containing `libiscc_uniffi`:

```bash
swift build -Xlinker -L/path/to/target/release
```

## License

Licensed under the Apache License, Version 2.0. See
[LICENSE](https://github.com/iscc/iscc-lib/blob/main/LICENSE) for details.
