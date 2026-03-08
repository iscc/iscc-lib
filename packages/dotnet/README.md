# Iscc.Lib

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![NuGet](https://img.shields.io/nuget/v/Iscc.Lib.svg)](https://www.nuget.org/packages/Iscc.Lib)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

High-performance C# / .NET bindings for [ISO 24138:2024](https://www.iso.org/standard/77899.html) —
International Standard Content Code (ISCC). Built as P/Invoke bindings to the Rust-compiled ISCC
library via [csbindgen](https://github.com/Cysharp/csbindgen), with an idiomatic C# API featuring
typed result records, `IDisposable` streaming hashers, and PascalCase method names.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

```bash
dotnet add package Iscc.Lib
```

## Quick Start

```csharp
using Iscc.Lib;

// Generate a Meta-Code from content metadata
var result = IsccLib.GenMetaCodeV0("Hello World");
Console.WriteLine(result.Iscc); // "ISCC:..."

// Streaming for large files
using var hasher = new IsccDataHasher();
using var fs = File.OpenRead("large_file.bin");
byte[] buf = new byte[65536];
int n;
while ((n = fs.Read(buf, 0, buf.Length)) > 0)
    hasher.Update(buf.AsSpan(0, n));
DataCodeResult dataResult = hasher.Finalize();
Console.WriteLine(dataResult.Iscc);
```

## API Overview

### Code Generators

| Method              | Description                                  |
| ------------------- | -------------------------------------------- |
| `GenMetaCodeV0`     | Generate a Meta-Code from metadata fields    |
| `GenTextCodeV0`     | Generate a Text-Code from plain text         |
| `GenImageCodeV0`    | Generate an Image-Code from pixel data       |
| `GenAudioCodeV0`    | Generate an Audio-Code from Chromaprint data |
| `GenVideoCodeV0`    | Generate a Video-Code from frame signatures  |
| `GenMixedCodeV0`    | Generate a Mixed-Code from Content-Codes     |
| `GenDataCodeV0`     | Generate a Data-Code from binary data        |
| `GenInstanceCodeV0` | Generate an Instance-Code from binary data   |
| `GenIsccCodeV0`     | Generate a composite ISCC-CODE               |
| `GenSumCodeV0`      | Generate an ISCC-SUM from a file path        |

### Utilities

- **Text processing:** `TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse`
- **Algorithm primitives:** `AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SlidingWindow`
- **Soft hashing:** `SoftHashVideoV0`
- **Encoding:** `EncodeBase64`, `EncodeComponent`
- **Codec:** `IsccDecode`, `IsccDecompose`, `JsonToDataUrl`
- **Streaming:** `IsccDataHasher`, `IsccInstanceHasher` for incremental processing
- **Diagnostics:** `ConformanceSelftest`

## Links

- [Documentation](https://lib.iscc.codes)
- [How-to Guide](https://lib.iscc.codes/howto/dotnet/)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
