---
icon: lucide/package
description: Guide to using iscc-lib from Go — runtime setup, code generation, streaming, and utilities.
---

# Go

A guide to using iscc-lib from Go. The Go package uses [wazero](https://wazero.io/) — a pure-Go
WebAssembly runtime — to execute the Rust-compiled ISCC library. No cgo, no shared libraries, no
external files. The WASM binary is embedded at compile time via `//go:embed`.

---

## Installation

```bash
go get github.com/iscc/iscc-lib/packages/go
```

The WASM binary is embedded in the package — no manual setup or external files needed.

## Runtime setup

All ISCC operations require a `Runtime` instance. Create one with `NewRuntime` and release resources
with `Close` when done:

```go
package main

import (
	"context"
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

	// Use rt for all ISCC operations...
}
```

`Runtime` is the entry point for all ISCC operations. Every method on `Runtime` requires a
`context.Context` as its first argument.

## Code generation

All 9 `Gen*V0` methods are on `*Runtime`, accept `context.Context` as the first parameter, and
return `(string, error)`. The returned string is the ISCC code prefixed with `ISCC:`. Optional
parameters use `nil` (for pointer types) or a default value.

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```go
code, err := rt.GenMetaCodeV0(ctx, "Die Unendliche Geschichte", nil, nil, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:AAA..."

// With description
desc := "Von Michael Ende"
code, err = rt.GenMetaCodeV0(ctx, "Die Unendliche Geschichte", &desc, nil, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code)

// With structured metadata (JSON string)
meta := `{"title": "Example", "author": "Author"}`
code, err = rt.GenMetaCodeV0(ctx, "Example Title", nil, &meta, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code)
```

Parameters: `name string`, `description *string`, `meta *string`, `bits uint32`. Pass `nil` for
`description` or `meta` to omit those fields.

### Text-Code

Generate a Text-Code from plain text content:

```go
code, err := rt.GenTextCodeV0(ctx, "Hello World", 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:EAA..."
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```go
// Pre-process your image to 32x32 grayscale externally
pixels := make([]byte, 1024)
for i := range pixels {
	pixels[i] = 128 // Placeholder: uniform gray
}
code, err := rt.GenImageCodeV0(ctx, pixels, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```go
// Obtain Chromaprint features externally
fingerprint := []int32{123456, -789012, 345678, 901234}
code, err := rt.GenAudioCodeV0(ctx, fingerprint, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```go
// Each frame signature is a slice of 380 integers
frame1 := make([]int32, 380)
frame2 := make([]int32, 380)
for i := range frame2 {
	frame2[i] = 1
}
frameSigs := [][]int32{frame1, frame2}

code, err := rt.GenVideoCodeV0(ctx, frameSigs, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```go
textCode, err := rt.GenTextCodeV0(ctx, "Hello World", 64)
if err != nil {
	log.Fatal(err)
}

pixels := make([]byte, 1024)
for i := range pixels {
	pixels[i] = 128
}
imageCode, err := rt.GenImageCodeV0(ctx, pixels, 64)
if err != nil {
	log.Fatal(err)
}

code, err := rt.GenMixedCodeV0(ctx, []string{textCode, imageCode}, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:EQA..."
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```go
data := bytes.Repeat([]byte("Hello World"), 1000)
code, err := rt.GenDataCodeV0(ctx, data, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:GAA..."
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```go
data := []byte("Hello World")
code, err := rt.GenInstanceCodeV0(ctx, data, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:IAA..."
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```go
data := bytes.Repeat([]byte("Hello World"), 1000)
dataCode, err := rt.GenDataCodeV0(ctx, data, 64)
if err != nil {
	log.Fatal(err)
}
instanceCode, err := rt.GenInstanceCodeV0(ctx, data, 64)
if err != nil {
	log.Fatal(err)
}

code, err := rt.GenIsccCodeV0(ctx, []string{dataCode, instanceCode})
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // "ISCC:KAA..."
```

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory. Both follow the `NewHasher → Update → Finalize → Close` pattern.

### DataHasher

```go
hasher, err := rt.NewDataHasher(ctx)
if err != nil {
	log.Fatal(err)
}
defer hasher.Close(ctx)

f, err := os.Open("large_file.bin")
if err != nil {
	log.Fatal(err)
}
defer f.Close()

buf := make([]byte, 65536)
for {
	n, err := f.Read(buf)
	if n > 0 {
		if err := hasher.Update(ctx, buf[:n]); err != nil {
			log.Fatal(err)
		}
	}
	if err == io.EOF {
		break
	}
	if err != nil {
		log.Fatal(err)
	}
}

code, err := hasher.Finalize(ctx, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // Identical to rt.GenDataCodeV0(ctx, entireFile, 64)
```

### InstanceHasher

```go
hasher, err := rt.NewInstanceHasher(ctx)
if err != nil {
	log.Fatal(err)
}
defer hasher.Close(ctx)

f, err := os.Open("large_file.bin")
if err != nil {
	log.Fatal(err)
}
defer f.Close()

buf := make([]byte, 65536)
for {
	n, err := f.Read(buf)
	if n > 0 {
		if err := hasher.Update(ctx, buf[:n]); err != nil {
			log.Fatal(err)
		}
	}
	if err == io.EOF {
		break
	}
	if err != nil {
		log.Fatal(err)
	}
}

code, err := hasher.Finalize(ctx, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // Identical to rt.GenInstanceCodeV0(ctx, entireFile, 64)
```

Both hashers must be closed with `Close(ctx)` to release WASM-side memory, even after calling
`Finalize`. After calling `Finalize`, further calls to `Update` or `Finalize` return an error.

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs.

```go
// Normalize text for display (NFKC, control char removal, line ending normalization)
cleaned, err := rt.TextClean(ctx, "  Hello\r\n\r\n\r\nWorld  ")
if err != nil {
	log.Fatal(err)
}
fmt.Println(cleaned) // "Hello\n\nWorld"

// Remove newlines and collapse whitespace to single spaces
singleLine, err := rt.TextRemoveNewlines(ctx, "Hello\nWorld\nFoo")
if err != nil {
	log.Fatal(err)
}
fmt.Println(singleLine) // "Hello World Foo"

// Trim text so UTF-8 byte size does not exceed a limit
trimmed, err := rt.TextTrim(ctx, "Hello World", 5)
if err != nil {
	log.Fatal(err)
}
fmt.Println(trimmed) // "Hello"

// Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
collapsed, err := rt.TextCollapse(ctx, "Hello, World!")
if err != nil {
	log.Fatal(err)
}
fmt.Println(collapsed) // "helloworld"
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```go
// Sliding window: overlapping substrings of width Unicode characters
windows, err := rt.SlidingWindow(ctx, "Hello World", 4)
if err != nil {
	log.Fatal(err)
}
fmt.Println(windows) // ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `AlgMinhash256(ctx, features)` — compute a 256-bit MinHash digest from `[]uint32` features
- `AlgCdcChunks(ctx, data, utf32, avgChunkSize)` — split data into content-defined chunks
- `AlgSimhash(ctx, digests)` — compute a SimHash from equal-length byte digests

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```go
ok, err := rt.ConformanceSelftest(ctx)
if err != nil {
	log.Fatal(err)
}
fmt.Println("Conformance:", ok) // true
```

## Error handling

All methods return `error` as the second return value, following idiomatic Go error handling. Errors
indicate invalid input (wrong bit count, empty data where content is required, etc.) or WASM runtime
failures:

```go
_, err := rt.GenTextCodeV0(ctx, "Hello", 13) // bits must be a multiple of 32
if err != nil {
	fmt.Printf("Invalid input: %v\n", err)
}
```
