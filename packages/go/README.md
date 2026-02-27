# iscc-lib (Go)

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> **Experimental:** This library is in early development (v0.0.x). APIs may change without notice.
> Not recommended for production use yet.

Go bindings for [ISO 24138:2024](https://www.iso.org/standard/77899.html) — International Standard
Content Code (ISCC). Pure Go with no cgo — uses [wazero](https://wazero.io/) to run Rust-compiled
WebAssembly. The WASM binary is embedded via `//go:embed`, so there are zero external dependencies
for consumers.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

```bash
go get github.com/iscc/iscc-lib/packages/go
```

The WASM binary is embedded in the package — no manual setup or external files needed.

## Quick Start

```go
package main

import (
	"context"
	"fmt"
	"log"

	iscc "github.com/iscc/iscc-lib/packages/go"
)

func main() {
	ctx := context.Background()

	rt, err := iscc.NewRuntime(ctx)
	if err != nil {
		log.Fatal(err)
	}
	defer rt.Close(ctx)

	code, err := rt.GenMetaCodeV0(ctx, "ISCC Test Document!", nil, nil, 64)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Meta-Code:", code)
}
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
| `GenDataCodeV0`     | Generate a Data-Code from raw bytes          |
| `GenInstanceCodeV0` | Generate an Instance-Code from raw bytes     |
| `GenIsccCodeV0`     | Generate a composite ISCC-CODE               |

All methods are on `*Runtime` and accept a `context.Context` as the first parameter. Functions
return `(string, error)` following idiomatic Go error handling.

### Text Processing

| Function             | Description                                              |
| -------------------- | -------------------------------------------------------- |
| `TextClean`          | Normalize text (NFKC, control chars, line endings)       |
| `TextRemoveNewlines` | Remove newlines and collapse whitespace to single spaces |
| `TextTrim`           | Trim text to a UTF-8 byte length limit                   |
| `TextCollapse`       | Simplify text for similarity hashing                     |

### Algorithm Primitives

| Function          | Description                                             |
| ----------------- | ------------------------------------------------------- |
| `SlidingWindow`   | Generate overlapping substrings of a given width        |
| `AlgMinhash256`   | Compute a 256-bit MinHash digest from `uint32` features |
| `AlgCdcChunks`    | Split data into content-defined chunks                  |
| `AlgSimhash`      | Compute a SimHash from equal-length byte digests        |
| `SoftHashVideoV0` | Compute video similarity hash from frame signatures     |

### Codec Operations

| Function          | Description                                                |
| ----------------- | ---------------------------------------------------------- |
| `EncodeBase64`    | Encode bytes to base64                                     |
| `JsonToDataUrl`   | Convert JSON string to `data:` URL                         |
| `EncodeComponent` | Construct an ISCC unit from header fields and digest       |
| `IsccDecode`      | Decode an ISCC unit string into header components + digest |
| `IsccDecompose`   | Decompose a composite ISCC-CODE into individual units      |

### Streaming

| Type / Function     | Description                                           |
| ------------------- | ----------------------------------------------------- |
| `NewDataHasher`     | Create a streaming Data-Code hasher                   |
| `NewInstanceHasher` | Create a streaming Instance-Code hasher               |
| `DataHasher`        | Streaming hasher with `Update` → `Finalize` → `Close` |
| `InstanceHasher`    | Streaming hasher with `Update` → `Finalize` → `Close` |

### Constants

| Constant              | Value     | Description                                   |
| --------------------- | --------- | --------------------------------------------- |
| `MetaTrimName`        | 128       | Max byte length for name normalization        |
| `MetaTrimDescription` | 4096      | Max byte length for description normalization |
| `IoReadSize`          | 4,194,304 | Default read buffer size (4 MB)               |
| `TextNgramSize`       | 13        | N-gram size for text similarity hashing       |

### Diagnostics

| Function              | Description                                      |
| --------------------- | ------------------------------------------------ |
| `ConformanceSelftest` | Verify correctness against official test vectors |

## Architecture

The package uses [wazero](https://wazero.io/), a pure-Go WebAssembly runtime, to execute
Rust-compiled WASM. This means:

- **No cgo** — builds with `CGO_ENABLED=0`
- **No shared libraries** — no `.so`, `.dylib`, or `.dll` to manage
- **Cross-compilation works** — `GOOS`/`GOARCH` just works, no C toolchain needed
- **Single binary deployment** — the WASM module is embedded at compile time

## Links

- [Documentation](https://lib.iscc.codes)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
