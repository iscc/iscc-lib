---
icon: lucide/package
description: Guide to using iscc-lib from Go — code generation, streaming, and utilities.
---

# Go

A guide to using iscc-lib from Go. The Go package is a pure Go implementation of all ISCC algorithms
— no cgo, no shared libraries, no external files. Install via `go get` and call package-level
functions directly.

---

## Installation

```bash
go get github.com/iscc/iscc-lib/packages/go
```

## Code generation

All 9 `Gen*V0` functions are package-level functions that return typed result structs and `error`.
The result struct's `Iscc` field contains the ISCC code string prefixed with `ISCC:`. Optional
parameters use `nil` (for pointer types) or a default value.

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```go
result, err := iscc.GenMetaCodeV0("Die Unendliche Geschichte", nil, nil, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:AAA..."

// With description
desc := "Von Michael Ende"
result, err = iscc.GenMetaCodeV0("Die Unendliche Geschichte", &desc, nil, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc)

// With structured metadata (JSON string)
meta := `{"title": "Example", "author": "Author"}`
result, err = iscc.GenMetaCodeV0("Example Title", nil, &meta, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc)
```

Parameters: `name string`, `description *string`, `meta *string`, `bits uint32`. Pass `nil` for
`description` or `meta` to omit those fields. Returns `*MetaCodeResult` with fields `Iscc`, `Name`,
`Description`, `Meta`, `Metahash`.

### Text-Code

Generate a Text-Code from plain text content:

```go
result, err := iscc.GenTextCodeV0("Hello World", 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:EAA..."
```

Returns `*TextCodeResult` with fields `Iscc` and `Characters`.

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```go
// Pre-process your image to 32x32 grayscale externally
pixels := make([]byte, 1024)
for i := range pixels {
	pixels[i] = 128 // Placeholder: uniform gray
}
result, err := iscc.GenImageCodeV0(pixels, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```go
// Obtain Chromaprint features externally
fingerprint := []int32{123456, -789012, 345678, 901234}
result, err := iscc.GenAudioCodeV0(fingerprint, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:EIA..."
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

result, err := iscc.GenVideoCodeV0(frameSigs, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```go
textResult, err := iscc.GenTextCodeV0("Hello World", 64)
if err != nil {
	log.Fatal(err)
}

pixels := make([]byte, 1024)
for i := range pixels {
	pixels[i] = 128
}
imageResult, err := iscc.GenImageCodeV0(pixels, 64)
if err != nil {
	log.Fatal(err)
}

result, err := iscc.GenMixedCodeV0([]string{textResult.Iscc, imageResult.Iscc}, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:EQA..."
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```go
data := bytes.Repeat([]byte("Hello World"), 1000)
result, err := iscc.GenDataCodeV0(data, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:GAA..."
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```go
data := []byte("Hello World")
result, err := iscc.GenInstanceCodeV0(data, 64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:IAA..."
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```go
data := bytes.Repeat([]byte("Hello World"), 1000)
dataResult, err := iscc.GenDataCodeV0(data, 64)
if err != nil {
	log.Fatal(err)
}
instanceResult, err := iscc.GenInstanceCodeV0(data, 64)
if err != nil {
	log.Fatal(err)
}

result, err := iscc.GenIsccCodeV0(
	[]string{dataResult.Iscc, instanceResult.Iscc}, false,
)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // "ISCC:KAA..."
```

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory. Both follow the `NewHasher` → `Push` → `Finalize` pattern.

### DataHasher

```go
hasher := iscc.NewDataHasher()

f, err := os.Open("large_file.bin")
if err != nil {
	log.Fatal(err)
}
defer f.Close()

buf := make([]byte, 65536)
for {
	n, err := f.Read(buf)
	if n > 0 {
		hasher.Push(buf[:n])
	}
	if err == io.EOF {
		break
	}
	if err != nil {
		log.Fatal(err)
	}
}

result, err := hasher.Finalize(64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // Identical to iscc.GenDataCodeV0(entireFile, 64)
```

### InstanceHasher

```go
hasher := iscc.NewInstanceHasher()

f, err := os.Open("large_file.bin")
if err != nil {
	log.Fatal(err)
}
defer f.Close()

buf := make([]byte, 65536)
for {
	n, err := f.Read(buf)
	if n > 0 {
		hasher.Push(buf[:n])
	}
	if err == io.EOF {
		break
	}
	if err != nil {
		log.Fatal(err)
	}
}

result, err := hasher.Finalize(64)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result.Iscc) // Identical to iscc.GenInstanceCodeV0(entireFile, 64)
```

Both hashers are single-use: after calling `Finalize`, do not call `Push` or `Finalize` again.

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs. These are pure functions that return a value directly (no
error).

```go
// Normalize text for display (NFKC, control char removal, line ending normalization)
cleaned := iscc.TextClean("  Hello\r\n\r\n\r\nWorld  ")
fmt.Println(cleaned) // "Hello\n\nWorld"

// Remove newlines and collapse whitespace to single spaces
singleLine := iscc.TextRemoveNewlines("Hello\nWorld\nFoo")
fmt.Println(singleLine) // "Hello World Foo"

// Trim text so UTF-8 byte size does not exceed a limit
trimmed := iscc.TextTrim("Hello World", 5)
fmt.Println(trimmed) // "Hello"

// Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
collapsed := iscc.TextCollapse("Hello, World!")
fmt.Println(collapsed) // "helloworld"
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```go
// Sliding window: overlapping substrings of width Unicode characters
windows, err := iscc.SlidingWindow("Hello World", 4)
if err != nil {
	log.Fatal(err)
}
fmt.Println(windows) // ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `AlgMinhash256(features []uint32) []byte` — compute a 256-bit MinHash digest from features
- `AlgCdcChunks(data []byte, utf32 bool, avgChunkSize uint32) [][]byte` — split data into
    content-defined chunks
- `AlgSimhash(digests [][]byte) ([]byte, error)` — compute a SimHash from equal-length byte digests

## Codec operations

Functions for encoding, decoding, and decomposing ISCC codes. These operate on the ISCC binary
format defined in ISO 24138.

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```go
// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
digest := []byte{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08}
code, err := iscc.EncodeComponent(0, 0, 0, 64, digest)
if err != nil {
	log.Fatal(err)
}
fmt.Println(code) // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into its header components and digest
decoded, err := iscc.IsccDecode(code)
if err != nil {
	log.Fatal(err)
}
fmt.Printf("Maintype: %d, Subtype: %d, Version: %d, Length: %d\n",
	decoded.Maintype, decoded.Subtype, decoded.Version, decoded.Length)
fmt.Printf("Digest: %x\n", decoded.Digest)
```

`IsccDecode` returns a `*DecodeResult` with fields `Maintype`, `Subtype`, `Version`, `Length`
(length index), and `Digest` (raw bytes).

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```go
// Generate a composite ISCC-CODE first
data := bytes.Repeat([]byte("Hello World"), 1000)
dataResult, _ := iscc.GenDataCodeV0(data, 64)
instanceResult, _ := iscc.GenInstanceCodeV0(data, 64)
isccResult, _ := iscc.GenIsccCodeV0(
	[]string{dataResult.Iscc, instanceResult.Iscc}, false,
)

// Decompose into individual units
units, err := iscc.IsccDecompose(isccResult.Iscc)
if err != nil {
	log.Fatal(err)
}
for _, unit := range units {
	fmt.Println(unit) // Each unit code (without "ISCC:" prefix)
}
```

### Other codec functions

- `EncodeBase64(data []byte) string` — encode bytes to base64url (no padding)
- `JsonToDataUrl(jsonStr string) (string, error)` — convert a JSON string to a
    `data:application/json;base64,...` URL
- `SoftHashVideoV0(frameSigs [][]int32, bits uint32) ([]byte, error)` — compute a video similarity
    hash from MPEG-7 frame signatures

## Constants

Package-level constants used by the ISCC algorithms:

```go
import iscc "github.com/iscc/iscc-lib/packages/go"

iscc.MetaTrimName        // 128 — max byte length for name normalization
iscc.MetaTrimDescription // 4096 — max byte length for description normalization
iscc.IoReadSize          // 4_194_304 — default read buffer size (4 MB)
iscc.TextNgramSize       // 13 — n-gram size for text similarity hashing
```

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```go
ok, err := iscc.ConformanceSelftest()
if err != nil {
	log.Fatal(err)
}
fmt.Println("Conformance:", ok) // true
```

## Error handling

All code generation functions return `error` as the last return value, following idiomatic Go error
handling. Errors indicate invalid input (wrong bit count, empty data where content is required,
etc.):

```go
_, err := iscc.GenTextCodeV0("Hello", 13) // bits must be a multiple of 32
if err != nil {
	fmt.Printf("Invalid input: %v\n", err)
}
```
