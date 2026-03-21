---
icon: lucide/compass
description: Guide to using iscc-lib from Swift — code generation, streaming, codec operations, constants, and utilities.
---

# Swift

A guide to using iscc-lib from Swift. The `IsccLib` Swift package provides UniFFI-generated bindings
to the Rust-compiled ISCC library. All functions are free functions in the `IsccLib` module — add
the package dependency and `import IsccLib` to get started.

---

## Installation

Add the package dependency to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.3.0"),
]
```

Then add `"IsccLib"` to your target's dependencies:

```swift
.target(
    name: "YourTarget",
    dependencies: [
        .product(name: "IsccLib", package: "iscc-lib"),
    ]
)
```

??? tip "Build from source"

    To build from source instead of using the Swift package:

    ```bash
    # Build the Rust UniFFI library
    cargo build -p iscc-uniffi

    # Run Swift tests (link against the Rust library)
    cd packages/swift
    swift test \
        -Xlinker -L../../target/debug \
        -Xlinker -rpath -Xlinker ../../target/debug
    ```

    The native library (`libiscc_uniffi.so` on Linux, `libiscc_uniffi.dylib` on macOS) must be available
    at runtime. Swift resolves the library via the linker search path.

## Code generation

All 10 `gen*V0` functions are free functions, return typed result structs, and throw `IsccUniError`
on invalid input. Parameters use Swift named arguments (`bits: 64`, `description: nil`, etc.).

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```swift
import IsccLib

let result = try genMetaCodeV0(name: "Die Unendliche Geschichte",
                               description: nil, meta: nil, bits: 64)
print(result.iscc) // "ISCC:AAA..."

// With description
let result2 = try genMetaCodeV0(name: "Die Unendliche Geschichte",
                                description: "Von Michael Ende",
                                meta: nil, bits: 64)

// With structured metadata (JSON string)
let result3 = try genMetaCodeV0(
    name: "Example Title",
    description: nil,
    meta: "{\"title\": \"Example\", \"author\": \"Author\"}",
    bits: 64
)
```

Parameters: `name: String`, `description: String?`, `meta: String?`, `bits: UInt32`. Returns
`MetaCodeResult` with `iscc`, `name`, `description`, `meta`, `metahash` fields.

### Text-Code

Generate a Text-Code from plain text content:

```swift
let result = try genTextCodeV0(text: "Hello World", bits: 64)
print(result.iscc) // "ISCC:EAA..."
print(result.characters) // Character count after collapse
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```swift
// Pre-process your image to 32x32 grayscale externally
let pixels = Data(repeating: 128, count: 1024) // Placeholder: uniform gray
let result = try genImageCodeV0(pixels: pixels, bits: 64)
print(result.iscc) // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```swift
// Obtain Chromaprint features externally
let fingerprint: [Int32] = [123_456, -789_012, 345_678, 901_234]
let result = try genAudioCodeV0(cv: fingerprint, bits: 64)
print(result.iscc) // "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```swift
// Each frame signature is an array of 380 integers
let frame1 = [Int32](repeating: 0, count: 380)
let frame2 = [Int32](repeating: 1, count: 380)
let frameSigs = [frame1, frame2]

let result = try genVideoCodeV0(frameSigs: frameSigs, bits: 64)
print(result.iscc) // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```swift
let textResult = try genTextCodeV0(text: "Hello World", bits: 64)
let pixels = Data(repeating: 0, count: 1024)
let imageResult = try genImageCodeV0(pixels: pixels, bits: 64)

let result = try genMixedCodeV0(
    codes: [textResult.iscc, imageResult.iscc],
    bits: 64
)
print(result.iscc) // "ISCC:EQA..."
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```swift
let text = String(repeating: "Hello World", count: 1000)
let data = Data(text.utf8)
let result = try genDataCodeV0(data: data, bits: 64)
print(result.iscc) // "ISCC:GAA..."
```

For file data, read the file as `Data`:

```swift
let data = try Data(contentsOf: URL(fileURLWithPath: "document.pdf"))
let result = try genDataCodeV0(data: data, bits: 64)
print(result.iscc)
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```swift
let data = Data("Hello World".utf8)
let result = try genInstanceCodeV0(data: data, bits: 64)
print(result.iscc) // "ISCC:IAA..."
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```swift
let text = String(repeating: "Hello World", count: 1000)
let data = Data(text.utf8)
let dataResult = try genDataCodeV0(data: data, bits: 64)
let instanceResult = try genInstanceCodeV0(data: data, bits: 64)

let result = try genIsccCodeV0(
    codes: [dataResult.iscc, instanceResult.iscc],
    wide: false
)
print(result.iscc) // "ISCC:KAA..."
```

Parameters: `codes: [String]`, `wide: Bool` (`true` for 256-bit output, `false` for 128-bit).

### Sum-Code

Generate a composite ISCC-CODE from a file in a single pass:

```swift
let result = try genSumCodeV0(path: "/path/to/file.bin",
                              bits: 64, wide: false, addUnits: false)
print(result.iscc)     // "ISCC:KAA..."
print(result.datahash) // Multihash of the data
print(result.filesize) // Size in bytes

// With individual unit codes
let withUnits = try genSumCodeV0(path: "/path/to/file.bin",
                                 bits: 64, wide: false, addUnits: true)
if let units = withUnits.units {
    for unit in units {
        print(unit)
    }
}
```

## Structured results

Every `gen*V0` function returns a typed Swift struct carrying the ISCC code string plus any
additional fields:

```swift
let result = try genMetaCodeV0(name: "Example Title",
                               description: nil, meta: nil, bits: 64)
print(result.iscc)     // The ISCC code string
print(result.metahash) // BLAKE3 multihash of metadata
```

Result types and their fields:

| Result type          | Fields                                              |
| -------------------- | --------------------------------------------------- |
| `MetaCodeResult`     | `iscc`, `name`, `description`?, `meta`?, `metahash` |
| `TextCodeResult`     | `iscc`, `characters`                                |
| `ImageCodeResult`    | `iscc`                                              |
| `AudioCodeResult`    | `iscc`                                              |
| `VideoCodeResult`    | `iscc`                                              |
| `MixedCodeResult`    | `iscc`, `parts`                                     |
| `DataCodeResult`     | `iscc`                                              |
| `InstanceCodeResult` | `iscc`, `datahash`, `filesize`                      |
| `IsccCodeResult`     | `iscc`                                              |
| `SumCodeResult`      | `iscc`, `datahash`, `filesize`, `units`?            |

Fields marked with `?` are optional and may be `nil`.

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory. Both are reference types (classes, not structs) because they wrap native
UniFFI objects.

### DataHasher

```swift
let hasher = DataHasher()

let fileHandle = try FileHandle(forReadingFrom: URL(fileURLWithPath: "large_file.bin"))
while true {
    let chunk = fileHandle.readData(ofLength: 65536)
    if chunk.isEmpty { break }
    try hasher.update(data: chunk)
}
fileHandle.closeFile()

let result = try hasher.finalize(bits: 64)
print(result.iscc) // Identical to genDataCodeV0(data: entireFile, bits: 64)
```

### InstanceHasher

```swift
let hasher = InstanceHasher()

let fileHandle = try FileHandle(forReadingFrom: URL(fileURLWithPath: "large_file.bin"))
while true {
    let chunk = fileHandle.readData(ofLength: 65536)
    if chunk.isEmpty { break }
    try hasher.update(data: chunk)
}
fileHandle.closeFile()

let result = try hasher.finalize(bits: 64)
print(result.iscc) // Identical to genInstanceCodeV0(data: entireFile, bits: 64)
```

Both hashers are single-use: after calling `finalize(bits:)`, further calls to `update(data:)` or
`finalize(bits:)` throw `IsccUniError`. The native resources are freed when the object is
deallocated.

## Codec operations

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```swift
// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
let digest = Data([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08])
let code = try encodeComponent(mtype: 0, stype: 0, version: 0,
                               bitLength: 64, digest: digest)
print(code) // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into header components and digest
let decoded = try isccDecode(iscc: code)
print("Maintype: \(decoded.maintype), Subtype: \(decoded.subtype)")
print("Version: \(decoded.version), Length: \(decoded.length)")
print("Digest: \(decoded.digest.map { String(format: "%02x", $0) }.joined())")
```

`isccDecode` returns a `DecodeResult` struct with `UInt8` fields `maintype`, `subtype`, `version`,
`length` (length index), and a `Data` field `digest`.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```swift
let data = Data("Hello World".utf8)
let dataCode = try genDataCodeV0(data: data, bits: 64)
let instanceCode = try genInstanceCodeV0(data: data, bits: 64)
let isccCode = try genIsccCodeV0(
    codes: [dataCode.iscc, instanceCode.iscc],
    wide: false
)

// Decompose into individual units
let units = try isccDecompose(isccCode: isccCode.iscc)
for unit in units {
    print(unit) // Each unit code (without "ISCC:" prefix)
}
```

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs:

```swift
// Normalize text for display (NFKC, control char removal, line ending normalization)
let cleaned = textClean(text: "  Hello\r\n\r\n\r\nWorld  ")
print(cleaned) // "Hello\n\nWorld"

// Remove newlines and collapse whitespace to single spaces
let singleLine = textRemoveNewlines(text: "Hello\nWorld\nFoo")
print(singleLine) // "Hello World Foo"

// Trim text so UTF-8 byte size does not exceed a limit
let trimmed = textTrim(text: "Hello World", nbytes: 5)
print(trimmed) // "Hello"

// Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
let collapsed = textCollapse(text: "Hello, World!")
print(collapsed) // "helloworld"
```

## Encoding utilities

```swift
// Encode bytes to base64url (no padding)
let encoded = encodeBase64(data: Data("Hello".utf8))
print(encoded) // "SGVsbG8"

// Convert JSON to data: URL with JCS canonicalization
let url = try jsonToDataUrl(json: "{\"key\":\"value\"}")
print(url) // "data:application/json;base64,..."
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```swift
// Sliding window: overlapping substrings of width Unicode characters
let windows = try slidingWindow(seq: "Hello World", width: 4)
// ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `algSimhash(hashDigests: [Data])` — compute a SimHash digest from equal-length byte arrays,
    returns `Data`
- `algMinhash256(features: [UInt32])` — compute a 256-bit MinHash digest from 32-bit features,
    returns `Data`
- `algCdcChunks(data: Data, utf32: Bool, avgChunkSize: UInt32)` — split data into content-defined
    chunks, returns `[Data]`
- `softHashVideoV0(frameSigs: [[Int32]], bits: UInt32)` — compute a similarity-preserving hash from
    video frame signatures, returns `Data`

## Constants

Getter functions returning algorithm parameters used by the ISCC algorithms. These are functions
(not properties) due to a UniFFI constraint:

```swift
import IsccLib

metaTrimName()        // 128 — max byte length for name normalization
metaTrimDescription() // 4096 — max byte length for description normalization
metaTrimMeta()        // 128_000 — max byte length for meta normalization
ioReadSize()          // 4_194_304 — default read buffer size (4 MB)
textNgramSize()       // 13 — n-gram size for text similarity hashing
```

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```swift
let ok = conformanceSelftest()
print("Conformance: \(ok)") // true
```

## Error handling

Functions that can fail are marked `throws` and throw `IsccUniError` on invalid input (wrong bit
count, empty data where content is required, etc.). Streaming hashers throw `IsccUniError` when used
after finalization. Use do-catch for error handling:

```swift
do {
    let _ = try genTextCodeV0(text: "Hello", bits: 13) // bits must be a multiple of 32
} catch let error as IsccUniError {
    print("Invalid input: \(error.localizedDescription)")
}
```
