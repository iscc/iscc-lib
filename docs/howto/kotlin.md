---
icon: lucide/compass
description: Guide to using iscc-lib from Kotlin — code generation, streaming, codec operations, constants, and utilities.
---

# Kotlin

A guide to using iscc-lib from Kotlin/JVM. The `uniffi.iscc_uniffi` package provides
UniFFI-generated bindings to the Rust-compiled ISCC library. All functions are free functions in the
`uniffi.iscc_uniffi` package — add the dependency and `import uniffi.iscc_uniffi.*` to get started.
The native library is loaded at runtime via JNA.

---

## Installation

Add the dependency to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation("io.iscc:iscc-lib-kotlin:0.3.1")
    implementation("net.java.dev.jna:jna:5.16.0")
}
```

The JNA dependency is required for loading the native Rust library at runtime. The native
`libiscc_uniffi` shared library must be available on `java.library.path` and `jna.library.path`.

!!! note "Not yet published to Maven Central"

    The Kotlin package is not yet published to Maven Central. For now, build from source (see below).

??? tip "Build from source"

    To build from source instead of using a Maven dependency:

    ```bash
    # Build the Rust UniFFI library
    cargo build -p iscc-uniffi

    # Run Kotlin tests (link against the Rust library)
    cd packages/kotlin
    ./gradlew test
    ```

    The Gradle build is configured to find the native library in `target/debug/`. The shared library
    (`libiscc_uniffi.so` on Linux, `libiscc_uniffi.dylib` on macOS, `iscc_uniffi.dll` on Windows) must
    be available at runtime via `java.library.path`, `jna.library.path`, or `LD_LIBRARY_PATH`.

## Code generation

All 10 `gen*V0` functions are free functions, return typed data classes, and throw
`IsccUniException` on invalid input. Parameters use Kotlin named arguments (`bits = 64u`,
`description = null`, etc.).

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```kotlin
import uniffi.iscc_uniffi.*

val result = genMetaCodeV0(
    name = "Die Unendliche Geschichte",
    description = null,
    meta = null,
    bits = 64u
)
println(result.iscc) // "ISCC:AAA..."

// With description
val result2 = genMetaCodeV0(
    name = "Die Unendliche Geschichte",
    description = "Von Michael Ende",
    meta = null,
    bits = 64u
)

// With structured metadata (JSON string)
val result3 = genMetaCodeV0(
    name = "Example Title",
    description = null,
    meta = """{"title": "Example", "author": "Author"}""",
    bits = 64u
)
```

Parameters: `name: String`, `description: String?`, `meta: String?`, `bits: UInt`. Returns
`MetaCodeResult` with `iscc`, `name`, `description`, `meta`, `metahash` fields.

### Text-Code

Generate a Text-Code from plain text content:

```kotlin
val result = genTextCodeV0(text = "Hello World", bits = 64u)
println(result.iscc)       // "ISCC:EAA..."
println(result.characters) // Character count after collapse
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```kotlin
// Pre-process your image to 32x32 grayscale externally
val pixels = ByteArray(1024) { 128.toByte() } // Placeholder: uniform gray
val result = genImageCodeV0(pixels = pixels, bits = 64u)
println(result.iscc) // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```kotlin
// Obtain Chromaprint features externally
val fingerprint = listOf(123456, -789012, 345678, 901234)
val result = genAudioCodeV0(cv = fingerprint, bits = 64u)
println(result.iscc) // "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```kotlin
// Each frame signature is a list of 380 integers
val frame1 = List(380) { 0 }
val frame2 = List(380) { 1 }
val frameSigs = listOf(frame1, frame2)

val result = genVideoCodeV0(frameSigs = frameSigs, bits = 64u)
println(result.iscc) // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```kotlin
val textResult = genTextCodeV0(text = "Hello World", bits = 64u)
val pixels = ByteArray(1024)
val imageResult = genImageCodeV0(pixels = pixels, bits = 64u)

val result = genMixedCodeV0(
    codes = listOf(textResult.iscc, imageResult.iscc),
    bits = 64u
)
println(result.iscc) // "ISCC:EQA..."
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```kotlin
val text = "Hello World".repeat(1000)
val data = text.toByteArray()
val result = genDataCodeV0(data = data, bits = 64u)
println(result.iscc) // "ISCC:GAA..."
```

For file data, read the file as a `ByteArray`:

```kotlin
import java.io.File

val data = File("document.pdf").readBytes()
val result = genDataCodeV0(data = data, bits = 64u)
println(result.iscc)
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```kotlin
val data = "Hello World".toByteArray()
val result = genInstanceCodeV0(data = data, bits = 64u)
println(result.iscc) // "ISCC:IAA..."
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```kotlin
val text = "Hello World".repeat(1000)
val data = text.toByteArray()
val dataResult = genDataCodeV0(data = data, bits = 64u)
val instanceResult = genInstanceCodeV0(data = data, bits = 64u)

val result = genIsccCodeV0(
    codes = listOf(dataResult.iscc, instanceResult.iscc),
    wide = false
)
println(result.iscc) // "ISCC:KAA..."
```

Parameters: `codes: List<String>`, `wide: Boolean` (`true` for 256-bit output, `false` for 128-bit).

### Sum-Code

Generate a composite ISCC-CODE from a file in a single pass:

```kotlin
val result = genSumCodeV0(
    path = "/path/to/file.bin",
    bits = 64u,
    wide = false,
    addUnits = false
)
println(result.iscc)     // "ISCC:KAA..."
println(result.datahash) // Multihash of the data
println(result.filesize) // Size in bytes

// With individual unit codes
val withUnits = genSumCodeV0(
    path = "/path/to/file.bin",
    bits = 64u,
    wide = false,
    addUnits = true
)
withUnits.units?.forEach { unit ->
    println(unit)
}
```

## Structured results

Every `gen*V0` function returns a typed Kotlin data class carrying the ISCC code string plus any
additional fields:

```kotlin
val result = genMetaCodeV0(
    name = "Example Title",
    description = null,
    meta = null,
    bits = 64u
)
println(result.iscc)     // The ISCC code string
println(result.metahash) // BLAKE3 multihash of metadata
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

Fields marked with `?` are optional and may be `null`.

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory. Both implement `AutoCloseable` for resource management.

### DataHasher

```kotlin
val hasher = DataHasher()

val file = java.io.File("large_file.bin")
file.inputStream().use { stream ->
    val buffer = ByteArray(65536)
    while (true) {
        val bytesRead = stream.read(buffer)
        if (bytesRead == -1) break
        hasher.update(data = buffer.copyOf(bytesRead))
    }
}

val result = hasher.finalize(bits = 64u)
println(result.iscc) // Identical to genDataCodeV0(data = entireFile, bits = 64u)
```

### InstanceHasher

```kotlin
val hasher = InstanceHasher()

val file = java.io.File("large_file.bin")
file.inputStream().use { stream ->
    val buffer = ByteArray(65536)
    while (true) {
        val bytesRead = stream.read(buffer)
        if (bytesRead == -1) break
        hasher.update(data = buffer.copyOf(bytesRead))
    }
}

val result = hasher.finalize(bits = 64u)
println(result.iscc) // Identical to genInstanceCodeV0(data = entireFile, bits = 64u)
```

Both hashers are single-use: after calling `finalize(bits)`, further calls to `update(data)` or
`finalize(bits)` throw `IsccUniException`. The native resources are freed when the object is closed
or garbage-collected.

## Codec operations

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```kotlin
// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
val digest = byteArrayOf(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08)
val code = encodeComponent(
    mtype = 0u,
    stype = 0u,
    version = 0u,
    bitLength = 64u,
    digest = digest
)
println(code) // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into header components and digest
val decoded = isccDecode(iscc = code)
println("Maintype: ${decoded.maintype}, Subtype: ${decoded.subtype}")
println("Version: ${decoded.version}, Length: ${decoded.length}")
println("Digest: ${decoded.digest.joinToString("") { "%02x".format(it) }}")
```

`isccDecode` returns a `DecodeResult` data class with `UByte` fields `maintype`, `subtype`,
`version`, `length` (length index), and a `ByteArray` field `digest`.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```kotlin
val data = "Hello World".toByteArray()
val dataCode = genDataCodeV0(data = data, bits = 64u)
val instanceCode = genInstanceCodeV0(data = data, bits = 64u)
val isccCode = genIsccCodeV0(
    codes = listOf(dataCode.iscc, instanceCode.iscc),
    wide = false
)

// Decompose into individual units
val units = isccDecompose(isccCode = isccCode.iscc)
for (unit in units) {
    println(unit) // Each unit code (without "ISCC:" prefix)
}
```

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs:

```kotlin
// Normalize text for display (NFKC, control char removal, line ending normalization)
val cleaned = textClean(text = "  Hello\r\n\r\n\r\nWorld  ")
println(cleaned) // "Hello\n\nWorld"

// Remove newlines and collapse whitespace to single spaces
val singleLine = textRemoveNewlines(text = "Hello\nWorld\nFoo")
println(singleLine) // "Hello World Foo"

// Trim text so UTF-8 byte size does not exceed a limit
val trimmed = textTrim(text = "Hello World", nbytes = 5u)
println(trimmed) // "Hello"

// Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
val collapsed = textCollapse(text = "Hello, World!")
println(collapsed) // "helloworld"
```

## Encoding utilities

```kotlin
// Encode bytes to base64url (no padding)
val encoded = encodeBase64(data = "Hello".toByteArray())
println(encoded) // "SGVsbG8"

// Convert JSON to data: URL with JCS canonicalization
val url = jsonToDataUrl(json = """{"key":"value"}""")
println(url) // "data:application/json;base64,..."
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```kotlin
// Sliding window: overlapping substrings of width Unicode characters
val windows = slidingWindow(seq = "Hello World", width = 4u)
// ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `algSimhash(hashDigests: List<ByteArray>)` — compute a SimHash digest from equal-length byte
    arrays, returns `ByteArray`
- `algMinhash256(features: List<UInt>)` — compute a 256-bit MinHash digest from 32-bit features,
    returns `ByteArray`
- `algCdcChunks(data: ByteArray, utf32: Boolean, avgChunkSize: UInt)` — split data into
    content-defined chunks, returns `List<ByteArray>`
- `softHashVideoV0(frameSigs: List<List<Int>>, bits: UInt)` — compute a similarity-preserving hash
    from video frame signatures, returns `ByteArray`

## Constants

Getter functions returning algorithm parameters used by the ISCC algorithms. These are functions
(not properties) due to a UniFFI constraint:

```kotlin
import uniffi.iscc_uniffi.*

metaTrimName()        // 128u — max byte length for name normalization
metaTrimDescription() // 4096u — max byte length for description normalization
metaTrimMeta()        // 128_000u — max byte length for meta normalization
ioReadSize()          // 4_194_304u — default read buffer size (4 MB)
textNgramSize()       // 13u — n-gram size for text similarity hashing
```

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```kotlin
val ok = conformanceSelftest()
println("Conformance: $ok") // true
```

## Error handling

Functions that can fail are annotated with `@Throws(IsccUniException::class)` and throw
`IsccUniException` on invalid input (wrong bit count, empty data where content is required, etc.).
Streaming hashers throw `IsccUniException` when used after finalization. Use try-catch for error
handling:

```kotlin
try {
    genTextCodeV0(text = "Hello", bits = 13u) // bits must be a multiple of 32
} catch (e: IsccUniException) {
    println("Invalid input: ${e.message}")
}
```
