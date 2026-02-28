# iscc-lib (Go)

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> **Experimental:** This library is in early development (v0.0.x). APIs may change without notice.
> Not recommended for production use yet.

Pure Go implementation of [ISO 24138:2024](https://www.iso.org/standard/77899.html) — International
Standard Content Code (ISCC). No cgo, no binary artifacts — all algorithms are implemented in native
Go for full portability and standard `go get` distribution.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

```bash
go get github.com/iscc/iscc-lib/packages/go
```

## Quick Start

```go
package main

import (
	"fmt"
	"log"

	iscc "github.com/iscc/iscc-lib/packages/go"
)

func main() {
	result, err := iscc.GenMetaCodeV0("ISCC Test Document!", nil, nil, 64)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Meta-Code:", result.Iscc)
}
```

## API Overview

### Code Generators

| Function            | Description                                  |
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

Package-level functions that return typed result structs (e.g., `*MetaCodeResult`,
`*TextCodeResult`) and `error`, following idiomatic Go error handling.

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

| Type / Function     | Description                               |
| ------------------- | ----------------------------------------- |
| `NewDataHasher`     | Create a streaming Data-Code hasher       |
| `NewInstanceHasher` | Create a streaming Instance-Code hasher   |
| `DataHasher`        | Streaming hasher with `Push` → `Finalize` |
| `InstanceHasher`    | Streaming hasher with `Push` → `Finalize` |

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

Pure Go implementation of all ISCC algorithms — native compiled code with no runtime overhead:

- **No cgo** — builds with `CGO_ENABLED=0`
- **No binary artifacts** — no shared libraries, no embedded binaries
- **Cross-compilation works** — `GOOS`/`GOARCH` just works, no C toolchain needed
- **Standard distribution** — install via `go get`, no extra setup

## Links

- [Documentation](https://lib.iscc.codes)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
