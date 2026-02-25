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

### Utilities

- **Text processing:** `TextClean`
- **Diagnostics:** `ConformanceSelftest`

Additional utilities (text processing, algorithm primitives, streaming hashers) are planned.

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
