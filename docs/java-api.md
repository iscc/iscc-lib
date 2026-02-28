---
icon: lucide/book-open
description: Java API reference for the ISCC library via JNI.
---

# Java API Reference

Java library for ISCC (ISO 24138:2024) code generation via JNI. All 30 Tier 1 symbols are exposed as
static methods on the `IsccLib` class. The native library is loaded automatically from the JAR's
`META-INF/native/` directory, with fallback to `System.loadLibrary` for development environments.

## Installation

=== "Maven"

    ```xml
    <dependency>
      <groupId>io.iscc</groupId>
      <artifactId>iscc-lib</artifactId>
      <version>0.0.2</version>
    </dependency>
    ```

=== "Gradle"

    ```groovy
    implementation 'io.iscc:iscc-lib:0.0.2'
    ```

## Quick Example

```java
import io.iscc.iscc_lib.IsccLib;

String iscc = IsccLib.genMetaCodeV0("Die Unendliche Geschichte", null, null, 64);
System.out.println(iscc); // e.g. "ISCC:AAA..."
```

---

## Constants

Algorithm constants exposed as `public static final int` fields on `IsccLib`.

| Constant                | Value       | Description                                             |
| ----------------------- | ----------- | ------------------------------------------------------- |
| `META_TRIM_NAME`        | `128`       | Maximum UTF-8 byte length for the name parameter        |
| `META_TRIM_DESCRIPTION` | `4096`      | Maximum UTF-8 byte length for the description parameter |
| `IO_READ_SIZE`          | `4_194_304` | Default read buffer size for streaming I/O (4 MB)       |
| `TEXT_NGRAM_SIZE`       | `13`        | N-gram window size for text similarity hashing          |

---

## Classes

### IsccDecodeResult

Result of decoding an ISCC unit string via `IsccLib.isccDecode()`. All fields are `public final`.

```java
package io.iscc.iscc_lib;

public class IsccDecodeResult {
    public final int maintype;
    public final int subtype;
    public final int version;
    public final int length;
    public final byte[] digest;
}
```

| Field      | Type     | Description                       |
| ---------- | -------- | --------------------------------- |
| `maintype` | `int`    | MainType enum value (0–7)         |
| `subtype`  | `int`    | SubType enum value (0–7)          |
| `version`  | `int`    | Version enum value (0–255)        |
| `length`   | `int`    | Length index from the ISCC header |
| `digest`   | `byte[]` | Raw digest bytes                  |

---

## Conformance

### conformanceSelftest

Run all ISCC conformance tests against vendored test vectors.

```java
public static native boolean conformanceSelftest();
```

Returns `true` if all tests pass, `false` otherwise.

---

## Code Generation

All 9 `gen*V0` methods return an ISCC string (prefixed with `ISCC:`) on success. All throw
`IllegalArgumentException` on invalid input.

### genMetaCodeV0

Generate a Meta-Code from name and optional metadata.

```java
public static native String genMetaCodeV0(
    String name,
    String description,
    String meta,
    int bits
);
```

| Parameter     | Type     | Description                                                    |
| ------------- | -------- | -------------------------------------------------------------- |
| `name`        | `String` | Title or name of the content (required, trimmed to 128 bytes)  |
| `description` | `String` | Optional description text (nullable, trimmed to 4096 bytes)    |
| `meta`        | `String` | Optional JSON string or `data:` URL (nullable)                 |
| `bits`        | `int`    | Bit length of the generated code (default: 64, multiple of 32) |

```java
String iscc = IsccLib.genMetaCodeV0(
    "Die Unendliche Geschichte", "Von Michael Ende", null, 64
);
```

---

### genTextCodeV0

Generate a Text-Code from plain text content.

```java
public static native String genTextCodeV0(String text, int bits);
```

| Parameter | Type     | Description                                                    |
| --------- | -------- | -------------------------------------------------------------- |
| `text`    | `String` | Plain text content (required)                                  |
| `bits`    | `int`    | Bit length of the generated code (default: 64, multiple of 32) |

```java
String iscc = IsccLib.genTextCodeV0("Hello World", 64);
```

---

### genImageCodeV0

Generate an Image-Code from 1024 grayscale pixel bytes.

```java
public static native String genImageCodeV0(byte[] pixels, int bits);
```

| Parameter | Type     | Description                                             |
| --------- | -------- | ------------------------------------------------------- |
| `pixels`  | `byte[]` | 32×32 grayscale pixel array (1024 bytes, values 0–255)  |
| `bits`    | `int`    | Bit length of the generated code (default: 64, max 256) |

```java
byte[] pixels = new byte[1024]; // 32×32 grayscale thumbnail
String iscc = IsccLib.genImageCodeV0(pixels, 64);
```

---

### genAudioCodeV0

Generate an Audio-Code from a Chromaprint feature vector.

```java
public static native String genAudioCodeV0(int[] cv, int bits);
```

| Parameter | Type    | Description                                                    |
| --------- | ------- | -------------------------------------------------------------- |
| `cv`      | `int[]` | Signed 32-bit Chromaprint feature values                       |
| `bits`    | `int`   | Bit length of the generated code (default: 64, multiple of 32) |

```java
int[] fingerprint = {123456, -789012, 345678};
String iscc = IsccLib.genAudioCodeV0(fingerprint, 64);
```

---

### genVideoCodeV0

Generate a Video-Code from frame signature data.

```java
public static native String genVideoCodeV0(int[][] frameSigs, int bits);
```

| Parameter   | Type      | Description                                                    |
| ----------- | --------- | -------------------------------------------------------------- |
| `frameSigs` | `int[][]` | Array of MPEG-7 frame signature vectors (each `int[]`)         |
| `bits`      | `int`     | Bit length of the generated code (default: 64, multiple of 32) |

Each frame signature is a 380-element `int[]` array.

```java
int[][] frameSigs = {new int[380], new int[380]};
String iscc = IsccLib.genVideoCodeV0(frameSigs, 64);
```

---

### genMixedCodeV0

Generate a Mixed-Code from multiple Content-Code strings.

```java
public static native String genMixedCodeV0(String[] codes, int bits);
```

| Parameter | Type       | Description                                                    |
| --------- | ---------- | -------------------------------------------------------------- |
| `codes`   | `String[]` | Array of ISCC Content-Code strings (requires >= 2)             |
| `bits`    | `int`      | Bit length of the generated code (default: 64, multiple of 32) |

```java
String[] codes = {"ISCC:EAA...", "ISCC:EEA..."};
String iscc = IsccLib.genMixedCodeV0(codes, 64);
```

---

### genDataCodeV0

Generate a Data-Code from raw byte data.

```java
public static native String genDataCodeV0(byte[] data, int bits);
```

| Parameter | Type     | Description                                                    |
| --------- | -------- | -------------------------------------------------------------- |
| `data`    | `byte[]` | Raw byte content                                               |
| `bits`    | `int`    | Bit length of the generated code (default: 64, multiple of 32) |

```java
byte[] data = "Hello World".getBytes();
String iscc = IsccLib.genDataCodeV0(data, 64);
```

---

### genInstanceCodeV0

Generate an Instance-Code from raw byte data.

```java
public static native String genInstanceCodeV0(byte[] data, int bits);
```

| Parameter | Type     | Description                                                    |
| --------- | -------- | -------------------------------------------------------------- |
| `data`    | `byte[]` | Raw byte content                                               |
| `bits`    | `int`    | Bit length of the generated code (default: 64, multiple of 32) |

```java
byte[] data = "Hello World".getBytes();
String iscc = IsccLib.genInstanceCodeV0(data, 64);
```

---

### genIsccCodeV0

Generate a composite ISCC-CODE from individual unit codes.

```java
public static native String genIsccCodeV0(String[] codes, boolean wide);
```

| Parameter | Type       | Description                                                    |
| --------- | ---------- | -------------------------------------------------------------- |
| `codes`   | `String[]` | Array of ISCC unit code strings (must include Data + Instance) |
| `wide`    | `boolean`  | `true` for 256-bit output, `false` for 128-bit                 |

```java
String dataCode = IsccLib.genDataCodeV0(data, 64);
String instanceCode = IsccLib.genInstanceCodeV0(data, 64);
String isccCode = IsccLib.genIsccCodeV0(
    new String[]{dataCode, instanceCode}, false
);
```

---

## Text Utilities

Text processing functions for normalization and cleaning.

### textClean

Clean and normalize text for display. Applies NFKC normalization, removes control characters (except
newlines), normalizes CRLF to LF, collapses consecutive empty lines, and strips leading/trailing
whitespace.

```java
public static native String textClean(String text);
```

---

### textRemoveNewlines

Remove newlines and collapse whitespace to single spaces.

```java
public static native String textRemoveNewlines(String text);
```

---

### textTrim

Trim text so its UTF-8 encoded size does not exceed `nbytes`. Multi-byte characters that would be
split are dropped entirely. Leading/trailing whitespace is stripped from the result.

```java
public static native String textTrim(String text, int nbytes);
```

| Parameter | Type     | Description               |
| --------- | -------- | ------------------------- |
| `text`    | `String` | Input text                |
| `nbytes`  | `int`    | Maximum UTF-8 byte length |

---

### textCollapse

Normalize and simplify text for similarity hashing. Applies NFD normalization, lowercasing, removes
whitespace and characters in Unicode categories C, M, and P, then recombines with NFKC
normalization.

```java
public static native String textCollapse(String text);
```

---

## Encoding

### encodeBase64

Encode bytes as base64url (RFC 4648 section 5, no padding).

```java
public static native String encodeBase64(byte[] data);
```

---

### jsonToDataUrl

Convert a JSON string to a data URL with base64 encoding. Uses `application/ld+json` media type when
the JSON contains an `@context` key, otherwise `application/json`.

```java
public static native String jsonToDataUrl(String json);
```

Throws `IllegalArgumentException` on invalid JSON input.

---

## Codec

### encodeComponent

Encode header fields and a raw digest into a base32-encoded ISCC unit string (without `ISCC:`
prefix).

```java
public static native String encodeComponent(
    int mtype,
    int stype,
    int version,
    int bitLength,
    byte[] digest
);
```

| Parameter   | Type     | Description                                |
| ----------- | -------- | ------------------------------------------ |
| `mtype`     | `int`    | MainType enum value (0–255)                |
| `stype`     | `int`    | SubType enum value (0–255)                 |
| `version`   | `int`    | Version enum value (0–255)                 |
| `bitLength` | `int`    | Digest bit length (multiple of 32)         |
| `digest`    | `byte[]` | Raw digest bytes (length >= bitLength / 8) |

Throws `IllegalArgumentException` on invalid input.

---

### isccDecode

Decode an ISCC unit string into its header components and raw digest. Strips an optional `ISCC:`
prefix before decoding.

```java
public static native IsccDecodeResult isccDecode(String isccUnit);
```

Returns an `IsccDecodeResult` with `maintype`, `subtype`, `version`, `length`, and `digest` fields.
Throws `IllegalArgumentException` on invalid input.

```java
IsccDecodeResult result = IsccLib.isccDecode("ISCC:EAASKDNZNYGUUF5A");
System.out.println(result.maintype); // 2 (Content)
System.out.println(result.subtype);  // 0 (Text)
```

---

### isccDecompose

Decompose a composite ISCC-CODE into individual ISCC-UNITs.

```java
public static native String[] isccDecompose(String isccCode);
```

Returns an array of base32-encoded ISCC-UNIT strings (without `ISCC:` prefix). Throws
`IllegalArgumentException` on invalid input.

---

## Sliding Window

### slidingWindow

Generate sliding window n-grams from a string.

```java
public static native String[] slidingWindow(String seq, int width);
```

| Parameter | Type     | Description                                       |
| --------- | -------- | ------------------------------------------------- |
| `seq`     | `String` | Input string                                      |
| `width`   | `int`    | Window width in Unicode characters (must be >= 2) |

Throws `IllegalArgumentException` if width is less than 2.

```java
String[] ngrams = IsccLib.slidingWindow("Hello World", 3);
// ["Hel", "ell", "llo", "lo ", "o W", " Wo", "Wor", "orl", "rld"]
```

---

## Algorithm Primitives

Low-level algorithm functions for SimHash, MinHash, content-defined chunking, and video hashing.

### algSimhash

Compute a SimHash from a sequence of equal-length hash digests. The output length matches the input
digest length.

```java
public static native byte[] algSimhash(byte[][] hashDigests);
```

Throws `IllegalArgumentException` on invalid input (empty array or unequal digest lengths).

---

### algMinhash256

Compute a 256-bit MinHash digest from 32-bit integer features. Uses 64 universal hash functions with
bit-interleaved compression.

```java
public static native byte[] algMinhash256(int[] features);
```

Returns a 32-byte MinHash digest.

---

### algCdcChunks

Split data into content-defined chunks using gear rolling hash.

```java
public static native byte[][] algCdcChunks(byte[] data, boolean utf32, int avgChunkSize);
```

| Parameter      | Type      | Description                                      |
| -------------- | --------- | ------------------------------------------------ |
| `data`         | `byte[]`  | Raw byte data to split                           |
| `utf32`        | `boolean` | If `true`, align cut points to 4-byte boundaries |
| `avgChunkSize` | `int`     | Target average chunk size (default 1024)         |

---

### softHashVideoV0

Compute a similarity-preserving hash from video frame signatures.

```java
public static native byte[] softHashVideoV0(int[][] frameSigs, int bits);
```

| Parameter   | Type      | Description                             |
| ----------- | --------- | --------------------------------------- |
| `frameSigs` | `int[][]` | Array of MPEG-7 frame signature vectors |
| `bits`      | `int`     | Output bit length                       |

Returns a byte array of length `bits / 8`. Throws `IllegalArgumentException` if input is empty.

---

## Streaming Hashers

Streaming hashers process large data incrementally without loading the entire content into memory.
Each hasher follows the lifecycle: `*New()` → `*Update()` (repeated) → `*Finalize()` → `*Free()`.

Streaming hashers use opaque `long` handles for native memory management. Callers **must** call the
corresponding `*Free` method to release native memory.

!!! warning

    `*Finalize()` consumes the hasher state. After finalizing, subsequent `*Update()` or `*Finalize()`
    calls will throw `IllegalArgumentException`. You must still call `*Free()` to release the wrapper.

### DataHasher

Streaming Data-Code hasher using content-defined chunking and MinHash.

```java
// Create a new DataHasher — returns an opaque handle
long hasher = IsccLib.dataHasherNew();

// Push data into the hasher (repeatable)
IsccLib.dataHasherUpdate(hasher, chunk1);
IsccLib.dataHasherUpdate(hasher, chunk2);

// Finalize and get the ISCC string
String iscc = IsccLib.dataHasherFinalize(hasher, 64);

// Free the native memory (required)
IsccLib.dataHasherFree(hasher);
```

#### dataHasherNew

```java
public static native long dataHasherNew();
```

Returns an opaque `long` handle to the hasher.

#### dataHasherUpdate

```java
public static native void dataHasherUpdate(long ptr, byte[] data);
```

Throws `IllegalArgumentException` if the hasher has been finalized.

#### dataHasherFinalize

```java
public static native String dataHasherFinalize(long ptr, int bits);
```

Throws `IllegalArgumentException` if already finalized or on error.

#### dataHasherFree

```java
public static native void dataHasherFree(long ptr);
```

Zero/null handle is a no-op. Each handle must be freed exactly once.

---

### InstanceHasher

Streaming Instance-Code hasher using BLAKE3.

```java
// Create a new InstanceHasher — returns an opaque handle
long hasher = IsccLib.instanceHasherNew();

// Push data into the hasher (repeatable)
IsccLib.instanceHasherUpdate(hasher, chunk1);
IsccLib.instanceHasherUpdate(hasher, chunk2);

// Finalize and get the ISCC string
String iscc = IsccLib.instanceHasherFinalize(hasher, 64);

// Free the native memory (required)
IsccLib.instanceHasherFree(hasher);
```

#### instanceHasherNew

```java
public static native long instanceHasherNew();
```

Returns an opaque `long` handle to the hasher.

#### instanceHasherUpdate

```java
public static native void instanceHasherUpdate(long ptr, byte[] data);
```

Throws `IllegalArgumentException` if the hasher has been finalized.

#### instanceHasherFinalize

```java
public static native String instanceHasherFinalize(long ptr, int bits);
```

Throws `IllegalArgumentException` if already finalized or on error.

#### instanceHasherFree

```java
public static native void instanceHasherFree(long ptr);
```

Zero/null handle is a no-op. Each handle must be freed exactly once.

---

## Error Handling

All methods that accept user input throw `IllegalArgumentException` on invalid arguments:

- Empty or null required parameters (e.g., empty `name` in `genMetaCodeV0`)
- Invalid `bits` values (must be a multiple of 32, range 32–256)
- Malformed ISCC strings passed to `isccDecode` or `isccDecompose`
- Streaming hasher operations after finalization

```java
try {
    String iscc = IsccLib.genTextCodeV0("Hello World", 64);
    System.out.println(iscc);
} catch (IllegalArgumentException e) {
    System.err.println("Error: " + e.getMessage());
}
```

## Memory Management

Most Java types (`String`, `byte[]`, `int[]`) are managed by the JVM garbage collector. The only
resources requiring manual cleanup are streaming hasher handles:

- `dataHasherFree(long ptr)` — release a DataHasher
- `instanceHasherFree(long ptr)` — release an InstanceHasher

Use try-finally to ensure cleanup:

```java
long hasher = IsccLib.dataHasherNew();
try {
    IsccLib.dataHasherUpdate(hasher, data);
    String iscc = IsccLib.dataHasherFinalize(hasher, 64);
} finally {
    IsccLib.dataHasherFree(hasher);
}
```
