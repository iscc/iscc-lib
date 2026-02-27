---
icon: lucide/coffee
description: Guide to using iscc-lib from Java — JNI setup, code generation, streaming, codec operations, constants, and utilities.
---

# Java

A guide to using iscc-lib from Java. The Java package provides JNI (Java Native Interface) bindings
to the Rust-compiled ISCC library. All methods are static calls on the `IsccLib` class — the native
shared library is loaded via `System.loadLibrary("iscc_jni")` and must be available on
`java.library.path` at runtime.

---

## Installation

Add the Maven dependency to your `pom.xml`:

```xml
<dependency>
  <groupId>io.iscc</groupId>
  <artifactId>iscc-lib</artifactId>
  <version>0.0.1</version>
</dependency>
```

!!! note "Build from source"

    Maven Central publishing is not yet available. Currently, the native library must be built from
    source:

    ```bash
    # Build the native JNI shared library
    cargo build -p iscc-jni

    # Run Java tests (Maven picks up the native library from target/debug)
    cd crates/iscc-jni/java
    mvn test
    ```

    The native library (`libiscc_jni.so` on Linux, `libiscc_jni.dylib` on macOS, `iscc_jni.dll` on
    Windows) is built to `target/debug/` (or `target/release/` with `--release`).

## Setup

The `IsccLib` class loads the native library automatically in its static initializer:

```java
import io.iscc.iscc_lib.IsccLib;

// The native library is loaded when the class is first referenced.
// Ensure java.library.path includes the directory containing the native library:
//   java -Djava.library.path=target/debug -cp ... YourApp
```

If the native library is not found on `java.library.path`, the JVM throws `UnsatisfiedLinkError` at
class load time. Set the path when launching your application:

```bash
java -Djava.library.path=/path/to/native/libs -cp your-app.jar com.example.Main
```

All ISCC methods are static — there is no runtime object to create or close.

## Code generation

All 9 `gen*V0` methods are static on `IsccLib`, return a `String` (the ISCC code prefixed with
`ISCC:`), and throw `IllegalArgumentException` on invalid input. Optional parameters use `null`. The
default `bits` value is `64`.

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```java
String code = IsccLib.genMetaCodeV0("Die Unendliche Geschichte", null, null, 64);
System.out.println(code); // "ISCC:AAA..."

// With description
String code2 = IsccLib.genMetaCodeV0(
    "Die Unendliche Geschichte", "Von Michael Ende", null, 64
);
System.out.println(code2);

// With structured metadata (JSON string)
String meta = "{\"title\": \"Example\", \"author\": \"Author\"}";
String code3 = IsccLib.genMetaCodeV0("Example Title", null, meta, 64);
System.out.println(code3);
```

Parameters: `String name`, `String description` (nullable), `String meta` (nullable), `int bits`.

### Text-Code

Generate a Text-Code from plain text content:

```java
String code = IsccLib.genTextCodeV0("Hello World", 64);
System.out.println(code); // "ISCC:EAA..."
```

### Image-Code

Generate an Image-Code from a 32×32 grayscale thumbnail (1024 bytes):

```java
// Pre-process your image to 32x32 grayscale externally
byte[] pixels = new byte[1024];
java.util.Arrays.fill(pixels, (byte) 128); // Placeholder: uniform gray
String code = IsccLib.genImageCodeV0(pixels, 64);
System.out.println(code); // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```java
// Obtain Chromaprint features externally
int[] fingerprint = {123456, -789012, 345678, 901234};
String code = IsccLib.genAudioCodeV0(fingerprint, 64);
System.out.println(code); // "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```java
// Each frame signature is an array of 380 integers
int[] frame1 = new int[380];
int[] frame2 = new int[380];
java.util.Arrays.fill(frame2, 1);
int[][] frameSigs = {frame1, frame2};

String code = IsccLib.genVideoCodeV0(frameSigs, 64);
System.out.println(code); // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```java
String textCode = IsccLib.genTextCodeV0("Hello World", 64);

byte[] pixels = new byte[1024];
java.util.Arrays.fill(pixels, (byte) 128);
String imageCode = IsccLib.genImageCodeV0(pixels, 64);

String code = IsccLib.genMixedCodeV0(
    new String[]{textCode, imageCode}, 64
);
System.out.println(code); // "ISCC:EQA..."
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```java
byte[] data = "Hello World".repeat(1000).getBytes();
String code = IsccLib.genDataCodeV0(data, 64);
System.out.println(code); // "ISCC:GAA..."
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```java
byte[] data = "Hello World".getBytes();
String code = IsccLib.genInstanceCodeV0(data, 64);
System.out.println(code); // "ISCC:IAA..."
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```java
byte[] data = "Hello World".repeat(1000).getBytes();
String dataCode = IsccLib.genDataCodeV0(data, 64);
String instanceCode = IsccLib.genInstanceCodeV0(data, 64);

String code = IsccLib.genIsccCodeV0(
    new String[]{dataCode, instanceCode}, false
);
System.out.println(code); // "ISCC:KAA..."
```

Parameters: `String[] codes`, `boolean wide` (`true` for 256-bit output, `false` for 128-bit).

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory. Both use opaque `long` handles for memory management and follow the `*New` →
`*Update` → `*Finalize` → `*Free` lifecycle.

!!! warning "Memory management"

    Every handle returned by `dataHasherNew()` or `instanceHasherNew()` must be freed with the
    corresponding `*Free` method. Always use try-finally to ensure cleanup, even if an exception occurs.

### DataHasher

```java
long ptr = IsccLib.dataHasherNew();
try {
    java.io.FileInputStream fis = new java.io.FileInputStream("large_file.bin");
    try {
        byte[] buf = new byte[65536];
        int n;
        while ((n = fis.read(buf)) != -1) {
            byte[] chunk = java.util.Arrays.copyOf(buf, n);
            IsccLib.dataHasherUpdate(ptr, chunk);
        }
    } finally {
        fis.close();
    }
    String code = IsccLib.dataHasherFinalize(ptr, 64);
    System.out.println(code); // Identical to IsccLib.genDataCodeV0(entireFile, 64)
} finally {
    IsccLib.dataHasherFree(ptr);
}
```

### InstanceHasher

```java
long ptr = IsccLib.instanceHasherNew();
try {
    java.io.FileInputStream fis = new java.io.FileInputStream("large_file.bin");
    try {
        byte[] buf = new byte[65536];
        int n;
        while ((n = fis.read(buf)) != -1) {
            byte[] chunk = java.util.Arrays.copyOf(buf, n);
            IsccLib.instanceHasherUpdate(ptr, chunk);
        }
    } finally {
        fis.close();
    }
    String code = IsccLib.instanceHasherFinalize(ptr, 64);
    System.out.println(code); // Identical to IsccLib.genInstanceCodeV0(entireFile, 64)
} finally {
    IsccLib.instanceHasherFree(ptr);
}
```

After calling `*Finalize`, further calls to `*Update` or `*Finalize` throw
`IllegalArgumentException`. The handle must still be freed with `*Free` after finalization. Calling
`*Free` with a zero/null handle is a no-op — each handle must be freed exactly once.

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs.

```java
// Normalize text for display (NFKC, control char removal, line ending normalization)
String cleaned = IsccLib.textClean("  Hello\r\n\r\n\r\nWorld  ");
System.out.println(cleaned); // "Hello\n\nWorld"

// Remove newlines and collapse whitespace to single spaces
String singleLine = IsccLib.textRemoveNewlines("Hello\nWorld\nFoo");
System.out.println(singleLine); // "Hello World Foo"

// Trim text so UTF-8 byte size does not exceed a limit
String trimmed = IsccLib.textTrim("Hello World", 5);
System.out.println(trimmed); // "Hello"

// Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
String collapsed = IsccLib.textCollapse("Hello, World!");
System.out.println(collapsed); // "helloworld"
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```java
// Sliding window: overlapping substrings of width Unicode characters
String[] windows = IsccLib.slidingWindow("Hello World", 4);
// ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `IsccLib.algSimhash(byte[][] hashDigests)` — compute a SimHash from equal-length byte digests,
    returns `byte[]`
- `IsccLib.algMinhash256(int[] features)` — compute a 256-bit MinHash digest from 32-bit features,
    returns `byte[]`
- `IsccLib.algCdcChunks(byte[] data, boolean utf32, int avgChunkSize)` — split data into
    content-defined chunks, returns `byte[][]`
- `IsccLib.softHashVideoV0(int[][] frameSigs, int bits)` — compute a similarity-preserving hash from
    video frame signatures, returns `byte[]`

## Codec operations

Methods for encoding, decoding, and decomposing ISCC codes. These operate on the ISCC binary format
defined in ISO 24138.

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```java
// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
byte[] digest = {0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};
String code = IsccLib.encodeComponent(0, 0, 0, 64, digest);
System.out.println(code); // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into header components and digest
IsccDecodeResult result = IsccLib.isccDecode(code);
System.out.printf("Maintype: %d, Subtype: %d, Version: %d, Length: %d%n",
    result.maintype, result.subtype, result.version, result.length);
System.out.printf("Digest: %s%n", java.util.HexFormat.of().formatHex(result.digest));
```

`isccDecode` returns an `IsccDecodeResult` with `int` fields `maintype`, `subtype`, `version`,
`length` (length index), and a `byte[]` field `digest`.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```java
byte[] data = "Hello World".repeat(1000).getBytes();
String dataCode = IsccLib.genDataCodeV0(data, 64);
String instanceCode = IsccLib.genInstanceCodeV0(data, 64);
String isccCode = IsccLib.genIsccCodeV0(
    new String[]{dataCode, instanceCode}, false
);

// Decompose into individual units
String[] units = IsccLib.isccDecompose(isccCode);
for (String unit : units) {
    System.out.println(unit); // Each unit code (without "ISCC:" prefix)
}
```

### Other codec methods

- `IsccLib.encodeBase64(byte[] data)` — encode bytes to base64 string
- `IsccLib.jsonToDataUrl(String json)` — convert a JSON string to a
    `data:application/json;base64,...` URL
- `IsccLib.softHashVideoV0(int[][] frameSigs, int bits)` — compute a video similarity hash from
    MPEG-7 frame signatures, returns `byte[]`

## Constants

Static constants on the `IsccLib` class used by the ISCC algorithms:

```java
import io.iscc.iscc_lib.IsccLib;

IsccLib.META_TRIM_NAME;        // 128 — max byte length for name normalization
IsccLib.META_TRIM_DESCRIPTION; // 4096 — max byte length for description normalization
IsccLib.IO_READ_SIZE;          // 4_194_304 — default read buffer size (4 MB)
IsccLib.TEXT_NGRAM_SIZE;       // 13 — n-gram size for text similarity hashing
```

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```java
boolean ok = IsccLib.conformanceSelftest();
System.out.println("Conformance: " + ok); // true
```

## Error handling

Methods throw `IllegalArgumentException` on invalid input (wrong bit count, empty data where content
is required, etc.). Use try-catch for error handling:

```java
try {
    IsccLib.genTextCodeV0("Hello", 13); // bits must be a multiple of 32
} catch (IllegalArgumentException e) {
    System.out.println("Invalid input: " + e.getMessage());
}
```
