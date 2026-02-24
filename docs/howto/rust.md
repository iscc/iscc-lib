---
icon: lucide/cog
description: Guide to using iscc-lib from Rust — code generation, streaming, and text utilities.
---

# Rust

A guide to using iscc-lib from Rust. Covers installation, code generation, structured results,
streaming, and text utilities.

---

## Installation

```bash
cargo add iscc-lib
```

iscc-lib is a pure Rust crate with no system dependencies.

## Code generation

All 9 `gen_*_v0` functions follow the same pattern: pass content-specific input and a `bits`
parameter (typically 64), and receive a typed result struct with an `.iscc` field containing the
ISCC code string.

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```rust
use iscc_lib::gen_meta_code_v0;

let result = gen_meta_code_v0(
    "Die Unendliche Geschichte",
    Some("Von Michael Ende"),
    None,
    64,
)?;
println!("{}", result.iscc);      // "ISCC:AAA..."
println!("{}", result.name);      // Normalized name
println!("{}", result.metahash);  // BLAKE3 hash of metadata
```

The `meta` parameter accepts a JSON string or a `data:` URL with base64-encoded payload. When
provided, `meta` takes precedence over `description` for the similarity digest.

### Text-Code

Generate a Text-Code from plain text content:

```rust
use iscc_lib::gen_text_code_v0;

let result = gen_text_code_v0("Hello World", 64)?;
println!("{}", result.iscc);        // "ISCC:EAA..."
println!("{}", result.characters);  // Number of characters processed
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```rust
use iscc_lib::gen_image_code_v0;

// Pre-process your image to 32x32 grayscale (e.g., with the `image` crate)
let pixels = vec![128u8; 1024];  // Placeholder: uniform gray
let result = gen_image_code_v0(&pixels, 64)?;
println!("{}", result.iscc);  // "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```rust
use iscc_lib::gen_audio_code_v0;

// Obtain Chromaprint features externally (e.g., via FFI to libchromaprint)
let fingerprint: Vec<i32> = vec![123456, -789012, 345678, 901234];
let result = gen_audio_code_v0(&fingerprint, 64)?;
println!("{}", result.iscc);  // "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```rust
use iscc_lib::gen_video_code_v0;

// Each frame signature is a Vec of 380 integers
let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380], vec![1i32; 380]];
let result = gen_video_code_v0(&frame_sigs, 64)?;
println!("{}", result.iscc);  // "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```rust
use iscc_lib::{gen_text_code_v0, gen_image_code_v0, gen_mixed_code_v0};

let text_result = gen_text_code_v0("Hello World", 64)?;
let image_result = gen_image_code_v0(&vec![128u8; 1024], 64)?;

let codes: Vec<&str> = vec![&text_result.iscc, &image_result.iscc];
let result = gen_mixed_code_v0(&codes, 64)?;
println!("{}", result.iscc);   // "ISCC:EQA..."
println!("{:?}", result.parts);  // Input code strings
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```rust
use iscc_lib::gen_data_code_v0;

let data = b"Hello World".repeat(1000);
let result = gen_data_code_v0(&data, 64)?;
println!("{}", result.iscc);  // "ISCC:GAA..."
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```rust
use iscc_lib::gen_instance_code_v0;

let data = b"Hello World";
let result = gen_instance_code_v0(data, 64)?;
println!("{}", result.iscc);       // "ISCC:IAA..."
println!("{}", result.datahash);   // Multihash of the data
println!("{}", result.filesize);   // Size in bytes
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```rust
use iscc_lib::{gen_data_code_v0, gen_instance_code_v0, gen_iscc_code_v0};

let data = b"Hello World".repeat(1000);
let data_result = gen_data_code_v0(&data, 64)?;
let instance_result = gen_instance_code_v0(&data, 64)?;

let codes: Vec<&str> = vec![&data_result.iscc, &instance_result.iscc];
let result = gen_iscc_code_v0(&codes, false)?;
println!("{}", result.iscc);  // "ISCC:KAA..."
```

## Structured results

Every `gen_*_v0` function returns a dedicated result struct carrying the ISCC code string plus
additional fields specific to each code type. This differs from the Python API which returns
dict-like objects.

```rust
use iscc_lib::gen_meta_code_v0;

let result = gen_meta_code_v0(
    "Example Title",
    Some("Example description"),
    None,
    64,
)?;

// Access fields directly on the struct
println!("{}", result.iscc);
println!("{}", result.name);
println!("{}", result.metahash);

// Optional fields use Option<T>
if let Some(desc) = &result.description {
    println!("Description: {desc}");
}
if let Some(meta) = &result.meta {
    println!("Meta: {meta}");
}
```

Result types and their fields:

| Result type          | Fields                                              |
| -------------------- | --------------------------------------------------- |
| `MetaCodeResult`     | `iscc`, `name`, `metahash`, `description`?, `meta`? |
| `TextCodeResult`     | `iscc`, `characters`                                |
| `ImageCodeResult`    | `iscc`                                              |
| `AudioCodeResult`    | `iscc`                                              |
| `VideoCodeResult`    | `iscc`                                              |
| `MixedCodeResult`    | `iscc`, `parts`                                     |
| `DataCodeResult`     | `iscc`                                              |
| `InstanceCodeResult` | `iscc`, `datahash`, `filesize`                      |
| `IsccCodeResult`     | `iscc`                                              |

Fields marked with `?` are `Option<String>` — present only when the corresponding input was
provided.

All result structs are marked `#[non_exhaustive]`, which means fields may be added in future
versions without a breaking semver change. This is why result structs cannot be constructed directly
outside the crate.

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data incrementally without loading
everything into memory. Both follow the `new() -> update(&[u8]) -> finalize()` pattern.

### DataHasher

```rust
use iscc_lib::DataHasher;
use std::io::Read;

let mut hasher = DataHasher::new();

let mut file = std::fs::File::open("large_file.bin")?;
let mut buf = [0u8; 65536];
loop {
    let n = file.read(&mut buf)?;
    if n == 0 {
        break;
    }
    hasher.update(&buf[..n]);
}

let result = hasher.finalize(64)?;
println!("{}", result.iscc);  // Identical to gen_data_code_v0(entire_file, 64)
```

### InstanceHasher

`InstanceHasher` follows the same pattern. Its result includes `datahash` and `filesize` fields:

```rust
use iscc_lib::InstanceHasher;
use std::io::Read;

let mut hasher = InstanceHasher::new();
let mut file = std::fs::File::open("large_file.bin")?;
let mut buf = [0u8; 65536];
loop {
    let n = file.read(&mut buf)?;
    if n == 0 { break; }
    hasher.update(&buf[..n]);
}
let result = hasher.finalize(64)?;
println!("{}", result.iscc);       // Identical to gen_instance_code_v0(entire_file, 64)
println!("{}", result.datahash);   // Multihash of the complete data
println!("{}", result.filesize);   // Total bytes processed
```

Both hashers produce results identical to their one-shot counterparts for the same byte stream.
After calling `finalize()`, the hasher is consumed by Rust's ownership system — no runtime error is
needed to prevent reuse.

## Text utilities

iscc-lib provides text normalization functions used internally by the code generation pipeline.
These are available for preprocessing your own text inputs.

### text_clean

Normalize text for display: applies NFKC normalization, removes control characters (except
newlines), normalizes line endings, collapses consecutive empty lines, and strips leading/trailing
whitespace.

```rust
use iscc_lib::text_clean;

let cleaned = text_clean("  Hello\r\n\r\n\r\nWorld  ");
assert_eq!(cleaned, "Hello\n\nWorld");
```

### text_collapse

Simplify text for similarity hashing: lowercases, strips whitespace, punctuation, and diacritics.
Used internally by `gen_text_code_v0`.

```rust
use iscc_lib::text_collapse;

let collapsed = text_collapse("Hello, World!");
assert_eq!(collapsed, "helloworld");
```

### text_remove_newlines

Remove newlines and collapse whitespace to single spaces:

```rust
use iscc_lib::text_remove_newlines;

let single_line = text_remove_newlines("Hello\nWorld\nFoo");
assert_eq!(single_line, "Hello World Foo");
```

### text_trim

Trim text so its UTF-8 byte size does not exceed a limit. Multi-byte characters that would be split
are dropped entirely:

```rust
use iscc_lib::text_trim;

let trimmed = text_trim("Hello World", 5);
assert_eq!(trimmed, "Hello");
```

## Algorithm primitives

iscc-lib exposes low-level algorithm functions as part of the public Rust API. These are building
blocks used internally by `gen_*_v0` functions.

- **`sliding_window(text: &str, width: usize) -> Vec<String>`** — Generate character n-grams from
    text using a sliding window of the given width.

- **`alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8>`** — Compute a SimHash fingerprint
    from a collection of byte digests. Used by Meta-Code, Audio-Code, and Mixed-Code generation.

- **`alg_minhash_256(features: &[u32]) -> Vec<u8>`** — Compute a 256-bit MinHash fingerprint from a
    collection of u32 feature hashes. Used by Text-Code and Data-Code generation.

- **`alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<&[u8]>`** — Split data
    into content-defined chunks using a rolling hash. Used by Data-Code generation.

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```rust
use iscc_lib::conformance_selftest;

assert!(conformance_selftest());
```

## Error handling

All `gen_*_v0` functions return `IsccResult<T>`, which is an alias for `Result<T, IsccError>`. The
only error variant is `IsccError::InvalidInput`, used for malformed or out-of-range parameters. Use
standard Rust error handling with `?` or `match`:

```rust
use iscc_lib::{gen_text_code_v0, IsccError};

match gen_text_code_v0("Hello World", 64) {
    Ok(result) => println!("Generated: {}", result.iscc),
    Err(IsccError::InvalidInput(msg)) => eprintln!("Invalid input: {msg}"),
}
```

The `?` operator propagates errors in any function returning a compatible `Result` type.
