# iscc-lib-kotlin

Kotlin/JVM bindings for [iscc-lib](https://github.com/iscc/iscc-lib), a high-performance
implementation of [ISO 24138:2024](https://www.iso.org/standard/77899.html) International Standard
Content Code (ISCC).

## Installation

Add the dependency to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation("io.iscc:iscc-lib-kotlin:0.3.1")
    implementation("net.java.dev.jna:jna:5.16.0")
}
```

The native `libiscc_uniffi` shared library must be available on `java.library.path` and
`jna.library.path` at runtime.

## Usage

```kotlin
import uniffi.iscc_uniffi.*

// Generate a Meta-Code
val meta = genMetaCodeV0(
    name = "Title",
    description = "A description",
    meta = null,
    bits = 64u
)
println(meta.iscc)  // "ISCC:..."

// Generate a Data-Code
val data = byteArrayOf(0x68, 0x65, 0x6c, 0x6c, 0x6f)
val dataCode = genDataCodeV0(data = data, bits = 64u)
println(dataCode.iscc)

// Streaming Data-Code for large files
val hasher = DataHasher()
hasher.update(data = chunk1)
hasher.update(data = chunk2)
val result = hasher.finalize(bits = 64u)

// Conformance self-test
assert(conformanceSelftest())
```

## API Overview

10 code generation functions covering all ISCC types:

- `genMetaCodeV0` — Meta-Code from title/description/metadata
- `genTextCodeV0` — Text-Code from plain text
- `genImageCodeV0` — Image-Code from 32x32 grayscale pixels
- `genAudioCodeV0` — Audio-Code from Chromaprint features
- `genVideoCodeV0` — Video-Code from MPEG-7 frame signatures
- `genMixedCodeV0` — Mixed-Code from multiple Content-Codes
- `genDataCodeV0` — Data-Code from raw bytes
- `genInstanceCodeV0` — Instance-Code from raw bytes (BLAKE3)
- `genIsccCodeV0` — Composite ISCC-CODE from unit codes
- `genSumCodeV0` — Single-pass ISCC-CODE from a file path

Plus codec operations (`encodeComponent`, `isccDecode`, `isccDecompose`), text utilities
(`textClean`, `textCollapse`, `textTrim`, `textRemoveNewlines`), encoding helpers (`encodeBase64`,
`jsonToDataUrl`), algorithm primitives, and constants.

## Building from Source

```bash
# Build the Rust UniFFI library
cargo build -p iscc-uniffi

# Run Kotlin tests
cd packages/kotlin
./gradlew test
```

## Links

- [Documentation](https://lib.iscc.codes/howto/kotlin/)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISO 24138:2024](https://www.iso.org/standard/77899.html)

## License

Licensed under the Apache License, Version 2.0. See
[LICENSE](https://github.com/iscc/iscc-lib/blob/main/LICENSE) for details.
