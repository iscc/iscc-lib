# Rust API Reference

Pure Rust crate for ISCC (ISO 24138:2024) code generation. All 9 `gen_*_v0` functions return
`IsccResult<String>` containing a JSON string with the generated ISCC code and metadata.

```bash
cargo add iscc-lib
```

```rust
use iscc_lib::gen_text_code_v0;

let result = gen_text_code_v0("Hello World", 64)?;
println!("{result}"); // JSON string containing "iscc" field
```

For full rustdoc documentation, see [docs.rs/iscc-lib](https://docs.rs/iscc-lib).

## Functions

### gen_meta_code_v0

Generate a Meta-Code from name and optional metadata.

```rust
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> IsccResult<String>
```

| Parameter     | Type           | Description                                                         |
| ------------- | -------------- | ------------------------------------------------------------------- |
| `name`        | `&str`         | Title or name of the content (normalized to 128 bytes)              |
| `description` | `Option<&str>` | Optional description text (normalized to 4096 bytes)                |
| `meta`        | `Option<&str>` | Optional metadata as JSON string or `data:` URL with base64 payload |
| `bits`        | `u32`          | Bit length of the generated code (default: 64, multiple of 32)      |

Returns an ISCC Meta-Code using SimHash for similarity hashing. When `meta` is provided, it takes
precedence over `description` for the similarity digest.

```rust
use iscc_lib::gen_meta_code_v0;

let iscc = gen_meta_code_v0("Die Unendliche Geschichte", Some("Von Michael Ende"), None, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_text_code_v0

Generate a Text-Code from plain text content.

```rust
pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<String>
```

| Parameter | Type   | Description                                                    |
| --------- | ------ | -------------------------------------------------------------- |
| `text`    | `&str` | Plain text content                                             |
| `bits`    | `u32`  | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Content-Code for text by collapsing the input, extracting character n-gram
features, and applying MinHash.

```rust
use iscc_lib::gen_text_code_v0;

let iscc = gen_text_code_v0("Hello World", 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_image_code_v0

Generate an Image-Code from pixel data.

```rust
pub fn gen_image_code_v0(pixels: &[u8], bits: u32) -> IsccResult<String>
```

| Parameter | Type    | Description                                             |
| --------- | ------- | ------------------------------------------------------- |
| `pixels`  | `&[u8]` | 1024 grayscale pixel values (32×32 image, values 0–255) |
| `bits`    | `u32`   | Bit length of the generated code (default: 64, max 256) |

Produces an ISCC Content-Code for images using a DCT-based perceptual hash. The caller must
pre-process the image to a 32×32 grayscale thumbnail before calling this function.

```rust
use iscc_lib::gen_image_code_v0;

let pixels = vec![128u8; 1024]; // 32×32 grayscale thumbnail
let iscc = gen_image_code_v0(&pixels, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_audio_code_v0

Generate an Audio-Code from a Chromaprint feature vector.

```rust
pub fn gen_audio_code_v0(cv: &[i32], bits: u32) -> IsccResult<String>
```

| Parameter | Type     | Description                                                    |
| --------- | -------- | -------------------------------------------------------------- |
| `cv`      | `&[i32]` | Chromaprint signed integer fingerprint vector                  |
| `bits`    | `u32`    | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Content-Code for audio using multi-stage SimHash over Chromaprint features. The
caller must generate the Chromaprint fingerprint externally before calling this function.

```rust
use iscc_lib::gen_audio_code_v0;

let fingerprint: Vec<i32> = vec![123456, -789012, 345678];
let iscc = gen_audio_code_v0(&fingerprint, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_video_code_v0

Generate a Video-Code from frame signature data.

```rust
pub fn gen_video_code_v0(frame_sigs: &[Vec<i32>], bits: u32) -> IsccResult<String>
```

| Parameter    | Type          | Description                                                    |
| ------------ | ------------- | -------------------------------------------------------------- |
| `frame_sigs` | `&[Vec<i32>]` | MPEG-7 frame signatures (each a 380-element integer vector)    |
| `bits`       | `u32`         | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Content-Code for video from MPEG-7 frame signatures using WTA-Hash. Deduplicates
frames and computes column-wise sums before hashing.

```rust
use iscc_lib::gen_video_code_v0;

let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380], vec![1i32; 380]];
let iscc = gen_video_code_v0(&frame_sigs, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_mixed_code_v0

Generate a Mixed-Code from multiple Content-Code strings.

```rust
pub fn gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<String>
```

| Parameter | Type      | Description                                                     |
| --------- | --------- | --------------------------------------------------------------- |
| `codes`   | `&[&str]` | Two or more ISCC Content-Code strings (optional `ISCC:` prefix) |
| `bits`    | `u32`     | Bit length of the generated code (default: 64, multiple of 32)  |

Combines multiple Content-Codes of different types (text, image, audio, video) into a single Mixed
Content-Code using SimHash. Requires at least 2 codes.

```rust
use iscc_lib::gen_mixed_code_v0;

let codes = &["ISCC:EAA...", "ISCC:EEA..."];
let iscc = gen_mixed_code_v0(codes, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_data_code_v0

Generate a Data-Code from raw byte data.

```rust
pub fn gen_data_code_v0(data: &[u8], bits: u32) -> IsccResult<String>
```

| Parameter | Type    | Description                                                    |
| --------- | ------- | -------------------------------------------------------------- |
| `data`    | `&[u8]` | Raw byte data                                                  |
| `bits`    | `u32`   | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Data-Code by splitting data into content-defined chunks, hashing each chunk, and
applying MinHash for similarity-preserving fingerprinting.

```rust
use iscc_lib::gen_data_code_v0;

let data = b"Hello World";
let iscc = gen_data_code_v0(data, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_instance_code_v0

Generate an Instance-Code from raw byte data.

```rust
pub fn gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<String>
```

| Parameter | Type    | Description                                                    |
| --------- | ------- | -------------------------------------------------------------- |
| `data`    | `&[u8]` | Raw byte data                                                  |
| `bits`    | `u32`   | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Instance-Code by hashing the complete byte stream with BLAKE3. Captures the exact
binary identity of the data — any change in the input produces a different code.

```rust
use iscc_lib::gen_instance_code_v0;

let data = b"Hello World";
let iscc = gen_instance_code_v0(data, 64)?;
assert!(iscc.starts_with("ISCC:"));
```

---

### gen_iscc_code_v0

Generate a composite ISCC-CODE from individual unit codes.

```rust
pub fn gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<String>
```

| Parameter | Type      | Description                                                    |
| --------- | --------- | -------------------------------------------------------------- |
| `codes`   | `&[&str]` | Two or more ISCC unit code strings (optional `ISCC:` prefix)   |
| `wide`    | `bool`    | Enable 256-bit wide mode (only with Data-Code + Instance-Code) |

Combines multiple ISCC unit codes (Meta-Code, Content-Code, Data-Code, Instance-Code) into a single
composite ISCC-CODE. Data-Code and Instance-Code are mandatory. When `wide` is `true` and exactly
two 128-bit+ codes (Data + Instance) are provided, produces a 256-bit wide-mode code.

```rust
use iscc_lib::{gen_data_code_v0, gen_instance_code_v0, gen_iscc_code_v0};

let data = b"Hello World";
let data_code = gen_data_code_v0(data, 64)?;
let instance_code = gen_instance_code_v0(data, 64)?;

// Extract ISCC strings from JSON results, then combine:
let codes = &[data_code_str, instance_code_str];
let iscc_code = gen_iscc_code_v0(codes, false)?;
```

## Types

The `codec` module provides Tier 2 types available to Rust consumers. These types are part of the
public Rust API but are not exposed through FFI bindings (Python, Node.js, WASM, C).

!!! note

    Import codec types with `use iscc_lib::codec::{MainType, SubType, ...}`.

### IsccError

Error type for ISCC operations.

```rust
pub enum IsccError {
    InvalidInput(String),
}
```

All `gen_*_v0` functions return `IsccResult<String>`, which is an alias for
`Result<String, IsccError>`. The only error variant is `InvalidInput`, used for malformed or
out-of-range parameters.

### MainType

ISCC MainType identifier. Integer values match the `iscc-core` Python reference.

| Variant    | Value | Description                        |
| ---------- | ----- | ---------------------------------- |
| `Meta`     | 0     | Metadata similarity code           |
| `Semantic` | 1     | Semantic similarity code           |
| `Content`  | 2     | Content similarity code            |
| `Data`     | 3     | Data similarity code               |
| `Instance` | 4     | Instance identity code             |
| `Iscc`     | 5     | Composite ISCC-CODE                |
| `Id`       | 6     | ISCC-ID (registry identifier)      |
| `Flake`    | 7     | ISCC-FLAKE (time-based identifier) |

### SubType

ISCC SubType identifier. Interpretation depends on the `MainType` context.

| Variant    | Value | Description                                |
| ---------- | ----- | ------------------------------------------ |
| `None`     | 0     | No specific subtype / Text (in CC context) |
| `Image`    | 1     | Image content                              |
| `Audio`    | 2     | Audio content                              |
| `Video`    | 3     | Video content                              |
| `Mixed`    | 4     | Mixed content                              |
| `Sum`      | 5     | ISCC composite summary                     |
| `IsccNone` | 6     | ISCC no specific content type              |
| `Wide`     | 7     | ISCC wide mode (256-bit)                   |

!!! tip

    `SubType::TEXT` is a constant alias for `SubType::None` (value 0), provided for readability when
    working with text Content-Codes.

### Codec Functions

The `codec` module also exposes encoding/decoding functions:

- **`encode_component`** — encode an ISCC unit (header + digest body) as a base32 string
- **`decode_header`** — decode ISCC header bytes into `(MainType, SubType, Version, length, tail)`
- **`encode_base32` / `decode_base32`** — RFC 4648 base32 encoding without padding

## Error Handling

All `gen_*_v0` functions use the `IsccResult<String>` return type. Errors are returned as
`IsccError::InvalidInput` with a descriptive message. Use standard Rust error handling:

```rust
use iscc_lib::{gen_text_code_v0, IsccError};

match gen_text_code_v0("Hello World", 64) {
    Ok(json) => println!("Generated: {json}"),
    Err(IsccError::InvalidInput(msg)) => eprintln!("Invalid input: {msg}"),
}
```
