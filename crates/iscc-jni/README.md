# iscc-lib (Java)

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> **Experimental:** This library is in early development (v0.0.x). APIs may change without notice.
> Not recommended for production use yet.

Java/JVM bindings for [ISO 24138:2024](https://www.iso.org/standard/77899.html) -- International
Standard Content Code (ISCC). Built with Rust via JNI for near-native speed.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

```xml
<dependency>
  <groupId>io.iscc</groupId>
  <artifactId>iscc-lib</artifactId>
  <version>0.0.3</version>
</dependency>
```

The native library (`libiscc_jni.so`, `libiscc_jni.dylib`, or `iscc_jni.dll`) must be available on
`java.library.path` at runtime. An automatic native loader is not yet included.

## Quick Start

```java
import io.iscc.iscc_lib.IsccLib;

String iscc = IsccLib.genMetaCodeV0("ISCC Test Document!", null, null, 64);
System.out.println("Meta-Code: " + iscc);
```

## API Overview

### Code Generators

| Method              | Description                                  |
| ------------------- | -------------------------------------------- |
| `genMetaCodeV0`     | Generate a Meta-Code from metadata fields    |
| `genTextCodeV0`     | Generate a Text-Code from plain text         |
| `genImageCodeV0`    | Generate an Image-Code from pixel data       |
| `genAudioCodeV0`    | Generate an Audio-Code from Chromaprint data |
| `genVideoCodeV0`    | Generate a Video-Code from frame signatures  |
| `genMixedCodeV0`    | Generate a Mixed-Code from Content-Codes     |
| `genDataCodeV0`     | Generate a Data-Code from raw bytes          |
| `genInstanceCodeV0` | Generate an Instance-Code from raw bytes     |
| `genIsccCodeV0`     | Generate a composite ISCC-CODE               |

All methods are static and accessed via `IsccLib`. Methods that accept invalid input throw
`IllegalArgumentException`.

### Utilities

- **Text processing:** `textClean`, `textRemoveNewlines`, `textTrim`, `textCollapse`
- **Algorithm primitives:** `algSimhash`, `algMinhash256`, `algCdcChunks`, `slidingWindow`
- **Soft hashing:** `softHashVideoV0`
- **Encoding:** `encodeBase64`
- **Codec:** `isccDecompose`
- **Streaming:** `dataHasherNew`/`dataHasherUpdate`/`dataHasherFinalize`/`dataHasherFree`,
    `instanceHasherNew`/`instanceHasherUpdate`/`instanceHasherFinalize`/`instanceHasherFree`
- **Diagnostics:** `conformanceSelftest`

## Links

- [Documentation](https://lib.iscc.codes)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
