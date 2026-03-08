---
icon: lucide/hash
description: Guide to using iscc-lib from C# / .NET — code generation, streaming, codec operations, constants, and utilities.
---

# C# / .NET

A guide to using iscc-lib from C# / .NET. The `Iscc.Lib` NuGet package provides P/Invoke bindings to
the Rust-compiled ISCC library via [csbindgen](https://github.com/Cysharp/csbindgen). All methods
are static calls on the `IsccLib` class in the `Iscc.Lib` namespace — add the package reference and
`using Iscc.Lib;` to get started.

---

## Installation

```bash
dotnet add package Iscc.Lib
```

??? tip "Build from source"

    To build from source instead of using the NuGet package:

    ```bash
    # Build the native FFI shared library
    cargo build -p iscc-ffi

    # Build the .NET class library
    dotnet build packages/dotnet/Iscc.Lib/

    # Run tests (set native library path)
    dotnet test packages/dotnet/Iscc.Lib.Tests/ \
        -e LD_LIBRARY_PATH=$(pwd)/target/debug
    ```

    The native library (`libiscc_ffi.so` on Linux, `libiscc_ffi.dylib` on macOS, `iscc_ffi.dll` on
    Windows) must be available at runtime. .NET resolves the DLL name `iscc_ffi` to the
    platform-appropriate filename automatically.

## Code generation

All 10 `Gen*V0` methods are static on `IsccLib`, return typed result records, and throw
`IsccException` on invalid input. Optional parameters use C# default values (`bits: 64`,
`description: null`, etc.).

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```csharp
using Iscc.Lib;

var result = IsccLib.GenMetaCodeV0("Die Unendliche Geschichte");
Console.WriteLine(result.Iscc); // "ISCC:AAA..."

// With description
var result2 = IsccLib.GenMetaCodeV0(
    "Die Unendliche Geschichte",
    description: "Von Michael Ende"
);

// With structured metadata (JSON string)
var result3 = IsccLib.GenMetaCodeV0(
    "Example Title",
    meta: "{\"title\": \"Example\", \"author\": \"Author\"}"
);
```

Parameters: `string name`, `string? description = null`, `string? meta = null`, `uint bits = 64`.
Returns `MetaCodeResult` with `Iscc` property.

### Text-Code

Generate a Text-Code from plain text content:

```csharp
var result = IsccLib.GenTextCodeV0("Hello World");
Console.WriteLine(result.Iscc); // "ISCC:EAA..."
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```csharp
// Pre-process your image to 32x32 grayscale externally
byte[] pixels = new byte[1024];
Array.Fill(pixels, (byte)128); // Placeholder: uniform gray
var result = IsccLib.GenImageCodeV0(pixels);
Console.WriteLine(result.Iscc); // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```csharp
// Obtain Chromaprint features externally
int[] fingerprint = [123_456, -789_012, 345_678, 901_234];
var result = IsccLib.GenAudioCodeV0(fingerprint);
Console.WriteLine(result.Iscc); // "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```csharp
// Each frame signature is an array of 380 integers
int[] frame1 = new int[380];
int[] frame2 = new int[380];
Array.Fill(frame2, 1);
int[][] frameSigs = [frame1, frame2];

var result = IsccLib.GenVideoCodeV0(frameSigs);
Console.WriteLine(result.Iscc); // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```csharp
var textResult = IsccLib.GenTextCodeV0("Hello World");
byte[] pixels = new byte[1024];
var imageResult = IsccLib.GenImageCodeV0(pixels);

var result = IsccLib.GenMixedCodeV0(
    [textResult.Iscc, imageResult.Iscc]
);
Console.WriteLine(result.Iscc); // "ISCC:EQA..."
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```csharp
using System.Text;

byte[] data = Encoding.UTF8.GetBytes(
    string.Concat(Enumerable.Repeat("Hello World", 1000))
);
var result = IsccLib.GenDataCodeV0(data);
Console.WriteLine(result.Iscc); // "ISCC:GAA..."
```

For file data, read the file as bytes:

```csharp
byte[] data = File.ReadAllBytes("document.pdf");
var result = IsccLib.GenDataCodeV0(data);
Console.WriteLine(result.Iscc);
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```csharp
byte[] data = "Hello World"u8.ToArray();
var result = IsccLib.GenInstanceCodeV0(data);
Console.WriteLine(result.Iscc); // "ISCC:IAA..."
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```csharp
byte[] data = Encoding.UTF8.GetBytes(
    string.Concat(Enumerable.Repeat("Hello World", 1000))
);
var dataResult = IsccLib.GenDataCodeV0(data);
var instanceResult = IsccLib.GenInstanceCodeV0(data);

var result = IsccLib.GenIsccCodeV0(
    [dataResult.Iscc, instanceResult.Iscc]
);
Console.WriteLine(result.Iscc); // "ISCC:KAA..."
```

Parameters: `string[] codes`, `bool wide = false` (`true` for 256-bit output, `false` for 128-bit).

### Sum-Code

Generate a composite ISCC-CODE from a file in a single pass:

```csharp
File.WriteAllBytes("example.bin",
    Encoding.UTF8.GetBytes(
        string.Concat(Enumerable.Repeat("Hello World", 1000))));

SumCodeResult result = IsccLib.GenSumCodeV0("example.bin");
Console.WriteLine(result.Iscc);     // "ISCC:KAA..."
Console.WriteLine(result.Datahash); // Multihash of the data
Console.WriteLine(result.Filesize); // Size in bytes

// With individual unit codes
SumCodeResult withUnits = IsccLib.GenSumCodeV0("example.bin", addUnits: true);
if (withUnits.Units is not null)
{
    foreach (string unit in withUnits.Units)
        Console.WriteLine(unit);
}
```

## Structured results

Every `Gen*V0` method returns a typed C# record carrying the ISCC code string plus any additional
fields:

```csharp
var result = IsccLib.GenMetaCodeV0("Example Title");
Console.WriteLine(result.Iscc);     // The ISCC code string

// Records support with-expressions for copying
var copy = result with { Iscc = "modified" };
```

Result types and their fields:

| Result type          | Fields                                   |
| -------------------- | ---------------------------------------- |
| `MetaCodeResult`     | `Iscc`                                   |
| `TextCodeResult`     | `Iscc`                                   |
| `ImageCodeResult`    | `Iscc`                                   |
| `AudioCodeResult`    | `Iscc`                                   |
| `VideoCodeResult`    | `Iscc`                                   |
| `MixedCodeResult`    | `Iscc`                                   |
| `DataCodeResult`     | `Iscc`                                   |
| `InstanceCodeResult` | `Iscc`                                   |
| `IsccCodeResult`     | `Iscc`                                   |
| `SumCodeResult`      | `Iscc`, `Datahash`, `Filesize`, `Units`? |

Fields marked with `?` are optional and may be `null`.

## Streaming

For large files, use `IsccDataHasher` and `IsccInstanceHasher` to process data in chunks without
loading everything into memory. Both implement `IDisposable` — use `using` statements for automatic
cleanup.

### IsccDataHasher

```csharp
using var hasher = new IsccDataHasher();

using var fs = File.OpenRead("large_file.bin");
byte[] buf = new byte[65536];
int n;
while ((n = fs.Read(buf, 0, buf.Length)) > 0)
{
    hasher.Update(buf.AsSpan(0, n));
}

DataCodeResult result = hasher.Finalize();
Console.WriteLine(result.Iscc); // Identical to IsccLib.GenDataCodeV0(entireFile)
```

### IsccInstanceHasher

```csharp
using var hasher = new IsccInstanceHasher();

using var fs = File.OpenRead("large_file.bin");
byte[] buf = new byte[65536];
int n;
while ((n = fs.Read(buf, 0, buf.Length)) > 0)
{
    hasher.Update(buf.AsSpan(0, n));
}

InstanceCodeResult result = hasher.Finalize();
Console.WriteLine(result.Iscc); // Identical to IsccLib.GenInstanceCodeV0(entireFile)
```

Both hashers are single-use: after calling `Finalize()`, further calls to `Update()` or `Finalize()`
throw `InvalidOperationException`. The `using` statement ensures the native resources are freed even
if an exception occurs.

## Codec operations

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```csharp
// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
byte[] digest = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
string code = IsccLib.EncodeComponent(0, 0, 0, 64, digest);
Console.WriteLine(code); // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into header components and digest
DecodeResult decoded = IsccLib.IsccDecode(code);
Console.WriteLine($"Maintype: {decoded.Maintype}, Subtype: {decoded.Subtype}");
Console.WriteLine($"Version: {decoded.Version}, Length: {decoded.Length}");
Console.WriteLine($"Digest: {Convert.ToHexString(decoded.Digest)}");
```

`IsccDecode` returns a `DecodeResult` record with `byte` fields `Maintype`, `Subtype`, `Version`,
`Length` (length index), and a `byte[]` field `Digest`.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```csharp
byte[] data = "Hello World"u8.ToArray();
var dataCode = IsccLib.GenDataCodeV0(data);
var instanceCode = IsccLib.GenInstanceCodeV0(data);
var isccCode = IsccLib.GenIsccCodeV0(
    [dataCode.Iscc, instanceCode.Iscc]
);

// Decompose into individual units
string[] units = IsccLib.IsccDecompose(isccCode.Iscc);
foreach (string unit in units)
    Console.WriteLine(unit); // Each unit code (without "ISCC:" prefix)
```

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs:

```csharp
// Normalize text for display (NFKC, control char removal, line ending normalization)
string cleaned = IsccLib.TextClean("  Hello\r\n\r\n\r\nWorld  ");
Console.WriteLine(cleaned); // "Hello\n\nWorld"

// Remove newlines and collapse whitespace to single spaces
string singleLine = IsccLib.TextRemoveNewlines("Hello\nWorld\nFoo");
Console.WriteLine(singleLine); // "Hello World Foo"

// Trim text so UTF-8 byte size does not exceed a limit
string trimmed = IsccLib.TextTrim("Hello World", 5);
Console.WriteLine(trimmed); // "Hello"

// Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
string collapsed = IsccLib.TextCollapse("Hello, World!");
Console.WriteLine(collapsed); // "helloworld"
```

## Encoding utilities

```csharp
// Encode bytes to base64url (no padding)
string encoded = IsccLib.EncodeBase64("Hello"u8);
Console.WriteLine(encoded); // "SGVsbG8"

// Convert JSON to data: URL with JCS canonicalization
string url = IsccLib.JsonToDataUrl("{\"key\":\"value\"}");
Console.WriteLine(url); // "data:application/json;base64,..."
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```csharp
// Sliding window: overlapping substrings of width Unicode characters
string[] windows = IsccLib.SlidingWindow("Hello World", 4);
// ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `IsccLib.AlgSimhash(byte[][] digests)` — compute a SimHash digest from equal-length byte arrays,
    returns `byte[]`
- `IsccLib.AlgMinhash256(ReadOnlySpan<uint> features)` — compute a 256-bit MinHash digest from
    32-bit features, returns `byte[]`
- `IsccLib.AlgCdcChunks(ReadOnlySpan<byte> data, bool utf32, uint avgChunkSize)` — split data into
    content-defined chunks, returns `byte[][]`
- `IsccLib.SoftHashVideoV0(int[][] frameSigs, uint bits)` — compute a similarity-preserving hash
    from video frame signatures, returns `byte[]`

## Constants

Static properties on the `IsccLib` class used by the ISCC algorithms:

```csharp
using Iscc.Lib;

IsccLib.MetaTrimName;        // 128 — max byte length for name normalization
IsccLib.MetaTrimDescription; // 4096 — max byte length for description normalization
IsccLib.MetaTrimMeta;        // 128_000 — max byte length for meta normalization
IsccLib.IoReadSize;          // 4_194_304 — default read buffer size (4 MB)
IsccLib.TextNgramSize;       // 13 — n-gram size for text similarity hashing
```

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```csharp
bool ok = IsccLib.ConformanceSelftest();
Console.WriteLine($"Conformance: {ok}"); // true
```

## Error handling

Methods throw `IsccException` on invalid input (wrong bit count, empty data where content is
required, etc.). Streaming hashers throw `InvalidOperationException` when used after finalization.
Use try-catch for error handling:

```csharp
try
{
    IsccLib.GenTextCodeV0("Hello", bits: 13); // bits must be a multiple of 32
}
catch (IsccException ex)
{
    Console.WriteLine($"Invalid input: {ex.Message}");
}
```
