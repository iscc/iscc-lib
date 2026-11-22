---
icon: lucide/book-open
description: Rust API reference for all ISCC code generation functions and types.
---

# Rust API Reference

Pure Rust crate for ISCC (ISO 24138:2024) code generation. Every `gen_*` function returns
`IsccResult<T>`, where `T` is a result struct specific to that function (`MetaCodeResult`,
`TextCodeResult`, …). Each result struct has an `iscc` field holding the ISCC code string (prefixed
with `ISCC:`); some carry additional fields.

```bash
cargo add iscc-lib
```

```rust
use iscc_lib::gen_text_code_v0;

let result = gen_text_code_v0("Hello World", 64)?;
println!("{}", result.iscc); // "ISCC:EAASKDNZNYGUUF5A"
```

For full rustdoc documentation, see [docs.rs/iscc-lib](https://docs.rs/iscc-lib).

## Code Generation

### gen_meta_code_v0

Generate a Meta-Code from name and optional metadata.

```rust
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> IsccResult<MetaCodeResult>
```

| Parameter     | Type           | Description                                                         |
| ------------- | -------------- | ------------------------------------------------------------------- |
| `name`        | `&str`         | Title or name of the content (normalized to 128 bytes)              |
| `description` | `Option<&str>` | Optional description text (normalized to 4096 bytes)                |
| `meta`        | `Option<&str>` | Optional metadata as JSON string or `data:` URL with base64 payload |
| `bits`        | `u32`          | Bit length of the generated code (default: 64, multiple of 32)      |

Returns a `MetaCodeResult` with `iscc`, `name`, `metahash` and the optional `description` and `meta`
fields. Uses SimHash for similarity hashing. When `meta` is provided, it takes precedence over
`description` for the similarity digest.

```rust
use iscc_lib::gen_meta_code_v0;

let result = gen_meta_code_v0("Die Unendliche Geschichte", Some("Von Michael Ende"), None, 64)?;
assert_eq!(result.iscc, "ISCC:AAAZXZ6OU4E45RB5");
```

---

### gen_text_code_v0

Generate a Text-Code from plain text content.

```rust
pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<TextCodeResult>
```

| Parameter | Type   | Description                                                    |
| --------- | ------ | -------------------------------------------------------------- |
| `text`    | `&str` | Plain text content                                             |
| `bits`    | `u32`  | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Content-Code for text by collapsing the input, extracting character n-gram
features, and applying MinHash. Returns a `TextCodeResult` with `iscc` and `characters` (the number
of characters after collapsing).

```rust
use iscc_lib::gen_text_code_v0;

let result = gen_text_code_v0("Hello World", 64)?;
assert_eq!(result.iscc, "ISCC:EAASKDNZNYGUUF5A");
```

---

### gen_image_code_v0

Generate an Image-Code from pixel data.

```rust
pub fn gen_image_code_v0(pixels: &[u8], bits: u32) -> IsccResult<ImageCodeResult>
```

| Parameter | Type    | Description                                             |
| --------- | ------- | ------------------------------------------------------- |
| `pixels`  | `&[u8]` | 1024 grayscale pixel values (32×32 image, values 0–255) |
| `bits`    | `u32`   | Bit length of the generated code (default: 64, max 256) |

Produces an ISCC Content-Code for images using a DCT-based perceptual hash. The caller must
pre-process the image to a 32×32 grayscale thumbnail before calling this function. Returns an
`ImageCodeResult` with `iscc`.

```rust
use iscc_lib::gen_image_code_v0;

let pixels = vec![128u8; 1024]; // 32×32 grayscale thumbnail
let result = gen_image_code_v0(&pixels, 64)?;
assert_eq!(result.iscc, "ISCC:EEAYAAAAAAAAAAAA");
```

---

### gen_audio_code_v0

Generate an Audio-Code from a Chromaprint feature vector.

```rust
pub fn gen_audio_code_v0(cv: &[i32], bits: u32) -> IsccResult<AudioCodeResult>
```

| Parameter | Type     | Description                                                    |
| --------- | -------- | -------------------------------------------------------------- |
| `cv`      | `&[i32]` | Chromaprint signed integer fingerprint vector                  |
| `bits`    | `u32`    | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Content-Code for audio using multi-stage SimHash over Chromaprint features. The
caller must generate the Chromaprint fingerprint externally before calling this function. Returns an
`AudioCodeResult` with `iscc`.

```rust
use iscc_lib::gen_audio_code_v0;

let fingerprint: Vec<i32> = vec![123456, -789012, 345678];
let result = gen_audio_code_v0(&fingerprint, 64)?;
assert_eq!(result.iscc, "ISCC:EIAQAAPGJQAADYSA");
```

---

### gen_video_code_v0

Generate a Video-Code from frame signature data.

```rust
pub fn gen_video_code_v0<S: AsRef<[i32]> + Ord>(
    frame_sigs: &[S],
    bits: u32,
) -> IsccResult<VideoCodeResult>
```

| Parameter    | Type   | Description                                                                  |
| ------------ | ------ | ---------------------------------------------------------------------------- |
| `frame_sigs` | `&[S]` | MPEG-7 frame signatures (each a 380-element integer vector, e.g. `Vec<i32>`) |
| `bits`       | `u32`  | Bit length of the generated code (default: 64, multiple of 32)               |

Produces an ISCC Content-Code for video from MPEG-7 frame signatures using WTA-Hash. Deduplicates
frames and computes column-wise sums before hashing. Returns a `VideoCodeResult` with `iscc`.
Returns `IsccError::InvalidInput` for an empty `frame_sigs` or for frame signatures of unequal
length.

```rust
use iscc_lib::gen_video_code_v0;

let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380], vec![1i32; 380]];
let result = gen_video_code_v0(&frame_sigs, 64)?;
assert_eq!(result.iscc, "ISCC:EMAQAAAAAAAAAAAA");
```

---

### gen_mixed_code_v0

Generate a Mixed-Code from multiple Content-Code strings.

```rust
pub fn gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<MixedCodeResult>
```

| Parameter | Type      | Description                                                     |
| --------- | --------- | --------------------------------------------------------------- |
| `codes`   | `&[&str]` | Two or more ISCC Content-Code strings (optional `ISCC:` prefix) |
| `bits`    | `u32`     | Bit length of the generated code (default: 64, multiple of 32)  |

Combines multiple Content-Codes of different types (text, image, audio, video) into a single Mixed
Content-Code using SimHash. Requires at least 2 codes. Returns a `MixedCodeResult` with `iscc` and
`parts` (the input code strings).

```rust
use iscc_lib::gen_mixed_code_v0;

let codes = &["ISCC:EAASKDNZNYGUUF5A", "ISCC:EEAYAAAAAAAAAAAA"];
let result = gen_mixed_code_v0(codes, 64)?;
assert_eq!(result.iscc, "ISCC:EQASDJINXFXA2SQX");
```

---

### gen_data_code_v0

Generate a Data-Code from raw byte data.

```rust
pub fn gen_data_code_v0(data: &[u8], bits: u32) -> IsccResult<DataCodeResult>
```

| Parameter | Type    | Description                                                    |
| --------- | ------- | -------------------------------------------------------------- |
| `data`    | `&[u8]` | Raw byte data                                                  |
| `bits`    | `u32`   | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Data-Code by splitting data into content-defined chunks, hashing each chunk, and
applying MinHash for similarity-preserving fingerprinting. Returns a `DataCodeResult` with `iscc`.

```rust
use iscc_lib::gen_data_code_v0;

let data = b"Hello World";
let result = gen_data_code_v0(data, 64)?;
assert_eq!(result.iscc, "ISCC:GAAW53FRSZTRHOFE");
```

---

### gen_instance_code_v0

Generate an Instance-Code from raw byte data.

```rust
pub fn gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<InstanceCodeResult>
```

| Parameter | Type    | Description                                                    |
| --------- | ------- | -------------------------------------------------------------- |
| `data`    | `&[u8]` | Raw byte data                                                  |
| `bits`    | `u32`   | Bit length of the generated code (default: 64, multiple of 32) |

Produces an ISCC Instance-Code by hashing the complete byte stream with BLAKE3. Captures the exact
binary identity of the data — any change in the input produces a different code. Returns an
`InstanceCodeResult` with `iscc`, `datahash` (BLAKE3 multihash) and `filesize`.

```rust
use iscc_lib::gen_instance_code_v0;

let data = b"Hello World";
let result = gen_instance_code_v0(data, 64)?;
assert_eq!(result.iscc, "ISCC:IAAUD6BZIEI6W4J2");
assert_eq!(result.filesize, 11);
```

---

### gen_iscc_code_v0

Generate a composite ISCC-CODE from individual unit codes.

```rust
pub fn gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<IsccCodeResult>
```

| Parameter | Type      | Description                                                    |
| --------- | --------- | -------------------------------------------------------------- |
| `codes`   | `&[&str]` | Two or more ISCC unit code strings (optional `ISCC:` prefix)   |
| `wide`    | `bool`    | Enable 256-bit wide mode (only with Data-Code + Instance-Code) |

Combines multiple ISCC unit codes (Meta-Code, Content-Code, Data-Code, Instance-Code) into a single
composite ISCC-CODE. Data-Code and Instance-Code are mandatory. When `wide` is `true` and exactly
two 128-bit+ codes (Data + Instance) are provided, produces a 256-bit wide-mode code. Returns an
`IsccCodeResult` with `iscc`.

```rust
use iscc_lib::{gen_data_code_v0, gen_instance_code_v0, gen_iscc_code_v0};

let data = b"Hello World";
let data_code = gen_data_code_v0(data, 64)?;
let instance_code = gen_instance_code_v0(data, 64)?;

// Combine the ISCC code strings:
let codes = &[data_code.iscc.as_str(), instance_code.iscc.as_str()];
let result = gen_iscc_code_v0(codes, false)?;
assert_eq!(result.iscc, "ISCC:KUAG53FRSZTRHOFEIH4DSQIR5NYTU");
```

---

### gen_sum_code_v0

Generate an ISCC-SUM from a file path in a single pass.

```rust
pub fn gen_sum_code_v0(
    path: &std::path::Path,
    bits: u32,
    wide: bool,
    add_units: bool,
) -> IsccResult<SumCodeResult>
```

| Parameter   | Type    | Description                                                           |
| ----------- | ------- | --------------------------------------------------------------------- |
| `path`      | `&Path` | Path to the file (`std::path::Path`)                                  |
| `bits`      | `u32`   | Bit length of the generated code (default: 64)                        |
| `wide`      | `bool`  | Enable 256-bit wide mode for ISCC-CODE combination                    |
| `add_units` | `bool`  | Include individual Data-Code and Instance-Code ISCC strings in result |

Reads the file once, feeding both a `DataHasher` (CDC/MinHash) and an `InstanceHasher` (BLAKE3) from
the same buffer, then composes the result into an ISCC-CODE. Returns a `SumCodeResult` with `iscc`,
`datahash`, `filesize`, and optionally `units` (when `add_units` is `true`).

```rust
use iscc_lib::gen_sum_code_v0;
use std::path::Path;

let result = gen_sum_code_v0(Path::new("document.pdf"), 64, false, false)?;
println!("{}", result.iscc);      // Composite ISCC-CODE
println!("{}", result.datahash);  // BLAKE3 multihash
println!("{}", result.filesize);  // File size in bytes
```

### gen_iscc_id_v1

Mint an experimental ISCC-IDv1 from an explicit timestamp.

!!! warning "Experimental"

    ISCC-IDv1 is not part of ISO 24138 and may change in a minor release. It is exempt from the v1.0.0
    stability lock.

```rust
pub fn gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8) -> IsccResult<IsccIdResult>
```

| Parameter   | Type  | Description                                            |
| ----------- | ----- | ------------------------------------------------------ |
| `timestamp` | `u64` | Microsecond timestamp, 52-bit range (`0 ..= 2^52 - 1`) |
| `hub_id`    | `u16` | HUB-ID, 12-bit range (`0 ..= 4095`)                    |
| `realm`     | `u8`  | Realm identifier, encoded as the SubType (`0` or `1`)  |

Produces an 80-bit ISCC-IDv1 (a 52-bit microsecond timestamp plus a 12-bit HUB-ID). Returns an
`IsccIdResult` with a single `iscc` field, mirroring iscc-core's `{"iscc": ...}`.

There is no dedicated decoder — recover the fields with the generic `iscc_decode` and unpack the
8-byte big-endian body:

```rust
use iscc_lib::{gen_iscc_id_v1, iscc_decode};

let result = gen_iscc_id_v1(1751831876325218, 1, 0)?;
// result.iscc == "ISCC:MAIGHFECJMOPMIAB"

let (_maintype, subtype, version, _length, digest) = iscc_decode(&result.iscc)?;
assert_eq!(version, 1); // ISCC-IDv1
let n = u64::from_be_bytes(digest[..8].try_into().unwrap());
let timestamp = n >> 12;          // 1751831876325218
let hub_id = (n & 0xFFF) as u16;  // 1
let realm = subtype;              // 0
```

## Text Utilities

Text normalization functions used by the code generation pipeline, available for preprocessing your
own text inputs. `text_clean` and `text_collapse` require the `text-processing` feature (on by
default).

```rust
pub fn text_clean(text: &str) -> String
pub fn text_remove_newlines(text: &str) -> String
pub fn text_trim(text: &str, nbytes: usize) -> String
pub fn text_collapse(text: &str) -> String
```

- **`text_clean`** — applies NFKC normalization, removes control characters (except newlines),
    normalizes `\r\n` to `\n`, collapses consecutive empty lines to at most one, and strips
    leading/trailing whitespace.
- **`text_remove_newlines`** — splits on whitespace and joins the parts with a single space.
- **`text_trim`** — returns the longest prefix whose UTF-8 encoding fits in `nbytes` (a character
    that would be split is dropped), stripped of leading/trailing whitespace.
- **`text_collapse`** — NFD normalization, lowercasing, removal of whitespace and of characters in
    Unicode categories C, M and P, then NFKC normalization. Used by `gen_text_code_v0`.

```rust
use iscc_lib::{text_clean, text_collapse, text_remove_newlines, text_trim};

assert_eq!(text_clean("  Hello\r\n\r\n\r\nWorld  "), "Hello\n\nWorld");
assert_eq!(text_remove_newlines("Hello\nWorld\nFoo"), "Hello World Foo");
assert_eq!(text_trim("Hello World", 5), "Hello");
assert_eq!(text_collapse("Hello, World!"), "helloworld");
```

## Algorithm Primitives

Low-level building blocks used by the `gen_*` functions.

### sliding_window

```rust
pub fn sliding_window(seq: &str, width: usize) -> IsccResult<Vec<String>>
```

Returns overlapping substrings of `width` Unicode characters, advancing one character at a time. An
input shorter than `width` yields a single element holding the whole input. Returns
`IsccError::InvalidInput` if `width < 2`.

```rust
use iscc_lib::sliding_window;

assert_eq!(sliding_window("Hello", 3)?, ["Hel", "ell", "llo"]);
```

### alg_simhash

```rust
pub fn alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> IsccResult<Vec<u8>>
```

Computes a SimHash from a sequence of equal-length digests; the output has the length of one input
digest. Returns `IsccError::InvalidInput` for an empty list or digests of unequal length. Used by
Meta-Code, Audio-Code and Mixed-Code generation.

```rust
use iscc_lib::alg_simhash;

let digest = alg_simhash(&[[0xFFu8, 0x00], [0xFF, 0x0F], [0x00, 0x0F]])?;
assert_eq!(digest, [0xFF, 0x0F]);
```

### alg_minhash_256

```rust
pub fn alg_minhash_256(features: &[u32]) -> IsccResult<Vec<u8>>
```

Computes a 256-bit (32-byte) MinHash digest from 32-bit integer features. Returns
`IsccError::InvalidInput` for an empty feature list. Used by Text-Code and Data-Code generation.

```rust
use iscc_lib::alg_minhash_256;

assert_eq!(alg_minhash_256(&[1, 2, 3]).unwrap().len(), 32);
assert!(alg_minhash_256(&[]).is_err());
```

### alg_cdc_chunks

```rust
pub fn alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> IsccResult<Vec<&[u8]>>
```

Splits data into content-defined chunks with a gear rolling hash. Returns at least one chunk (an
empty slice for empty input). When `utf32` is `true`, cut points are aligned to 4-byte boundaries.
Returns `IsccError::InvalidInput` if `avg_chunk_size < 2`. Used by Data-Code generation.

```rust
use iscc_lib::alg_cdc_chunks;

let chunks = alg_cdc_chunks(b"Hello World", false, 1024)?;
assert_eq!(chunks, [b"Hello World".as_slice()]);
```

### soft_hash_video_v0

```rust
pub fn soft_hash_video_v0<S: AsRef<[i32]> + Ord>(
    frame_sigs: &[S],
    bits: u32,
) -> IsccResult<Vec<u8>>
```

Computes the raw similarity digest (`bits / 8` bytes) behind `gen_video_code_v0`: deduplicates the
frame signatures, sums them column-wise and applies WTA-Hash. Returns `IsccError::InvalidInput` for
an empty `frame_sigs` or for frame signatures of unequal length (iscc-core truncates to the shortest
signature instead).

```rust
use iscc_lib::soft_hash_video_v0;

let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380], vec![1i32; 380]];
let digest = soft_hash_video_v0(&frame_sigs, 64)?;
assert_eq!(digest.len(), 8);
```

## Encoding

### encode_base64

```rust
pub fn encode_base64(data: &[u8]) -> String
```

Encodes bytes as base64url (RFC 4648 §5) without padding.

```rust
use iscc_lib::encode_base64;

assert_eq!(encode_base64(b"Hello World"), "SGVsbG8gV29ybGQ");
```

### json_to_data_url

```rust
pub fn json_to_data_url(json: &str) -> IsccResult<String>
```

Parses a JSON string, re-serializes it in RFC 8785 (JCS) canonical form, base64-encodes it and wraps
it in a `data:` URL. The media type is `application/ld+json` when the JSON has an `@context` key,
otherwise `application/json`. Returns `IsccError::InvalidInput` for invalid JSON. Requires the
`meta-code` feature (on by default). The result can be passed as the `meta` argument of
`gen_meta_code_v0`.

```rust
use iscc_lib::json_to_data_url;

let url = json_to_data_url(r#"{"key": "value"}"#)?;
assert_eq!(url, "data:application/json;base64,eyJrZXkiOiJ2YWx1ZSJ9");
```

## Codec

### iscc_decode

```rust
pub fn iscc_decode(iscc: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>
```

Decodes an ISCC string (optional `ISCC:` prefix, dashes allowed) into
`(maintype, subtype, version, length_index, digest)`. The integer fields match the
`codec::MainType`, `codec::SubType` and `codec::Version` values. Returns `IsccError::InvalidInput`
for invalid base32 or a malformed header.

```rust
use iscc_lib::iscc_decode;

let (maintype, subtype, version, length_index, digest) = iscc_decode("ISCC:GAAW53FRSZTRHOFE")?;
assert_eq!((maintype, subtype, version, length_index), (3, 0, 0, 1)); // Data-Code, 64 bits
assert_eq!(digest.len(), 8);
```

### iscc_decompose

```rust
pub fn iscc_decompose(iscc_code: &str) -> IsccResult<Vec<String>>
```

Splits a composite ISCC-CODE, or the base32 form of concatenated ISCC-UNIT bytes, into its
ISCC-UNITs. Returns base32 unit strings without the `ISCC:` prefix.

```rust
use iscc_lib::iscc_decompose;

let units = iscc_decompose("ISCC:KUAG53FRSZTRHOFEIH4DSQIR5NYTU")?;
assert_eq!(units, ["GAAW53FRSZTRHOFE", "IAAUD6BZIEI6W4J2"]);
```

### encode_component

```rust
pub fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: &[u8],
) -> IsccResult<String>
```

Encodes a raw digest as an ISCC-UNIT string (base32, without the `ISCC:` prefix). `mtype`, `stype`
and `version` take the integer values of the `codec::MainType`, `codec::SubType` and
`codec::Version` enums. Returns `IsccError::InvalidInput` for out-of-range values, for
`MainType::Iscc` (5), or when `digest` is shorter than `bit_length / 8` bytes.

```rust
use iscc_lib::encode_component;

let unit = encode_component(3, 0, 0, 64, &[0u8; 8])?; // Data-Code, 64 bits
assert_eq!(unit, "GAAQAAAAAAAAAAAA");
```

### ISCC-SEQ

`encode_seq` and `decode_seq` (IEP-0020) are Tier 1 functions re-exported at the crate root:

```rust
pub fn encode_seq(units: &[&str]) -> IsccResult<Vec<u8>>
pub fn decode_seq(data: &[u8]) -> IsccResult<Vec<String>>
```

`encode_seq` concatenates the header and body bytes of each ISCC-UNIT in the given order (never
sorted, deduplicated or dropped). Units may omit the `ISCC:` prefix and may be lowercase or contain
hyphens. Base32 input is strict (no padding, zero spare bits) and upper-casing is ASCII-only.
ISCC-CODE, ISCC-ID and ISCC-FLAKE units are rejected. `decode_seq` is a strict byte-level reader
that returns `ISCC:`-prefixed unit strings in wire order and rejects trailing bytes. Both return
`IsccError::InvalidInput` for empty input or a malformed unit.

```rust
use iscc_lib::{decode_seq, encode_seq};

let data = encode_seq(&["ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI"])?;
assert_eq!(data.len(), 34); // starts with 0x20 0x07 0xa8 0x82 0x48 0xc5

let units = decode_seq(&data)?;
// units == ["ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI"]

// The round trip is defined on bytes: encode_seq(decode_seq(data)) == data
let refs: Vec<&str> = units.iter().map(String::as_str).collect();
assert_eq!(encode_seq(&refs)?, data);
```

## Streaming

`DataHasher` and `InstanceHasher` compute a Data-Code and an Instance-Code incrementally, for input
that does not fit in memory. Both produce the same result as their one-shot counterparts for the
same byte stream. `finalize` consumes the hasher.

```rust
impl DataHasher {
    pub fn new() -> Self
    pub fn update(&mut self, data: &[u8])
    pub fn finalize(self, bits: u32) -> IsccResult<DataCodeResult>
}

impl InstanceHasher {
    pub fn new() -> Self
    pub fn update(&mut self, data: &[u8])
    pub fn finalize(self, bits: u32) -> IsccResult<InstanceCodeResult>
}
```

```rust
use iscc_lib::{DataHasher, InstanceHasher};

let mut data_hasher = DataHasher::new();
let mut instance_hasher = InstanceHasher::new();
for chunk in [b"Hello ".as_slice(), b"World".as_slice()] {
    data_hasher.update(chunk);
    instance_hasher.update(chunk);
}
assert_eq!(data_hasher.finalize(64)?.iscc, "ISCC:GAAW53FRSZTRHOFE");
assert_eq!(instance_hasher.finalize(64)?.iscc, "ISCC:IAAUD6BZIEI6W4J2");
```

### SumHasher

`iscc_lib::streaming::SumHasher` is Tier 2: public Rust API, but not re-exported at the crate root
and not bound in every language (Python and WASM wrap it). It feeds each chunk to an inner
`DataHasher` and `InstanceHasher` in one pass and composes the ISCC-SUM. The result is identical to
`gen_sum_code_v0` on a file holding the same bytes.

```rust
impl SumHasher {
    pub fn new() -> Self
    pub fn update(&mut self, data: &[u8])
    pub fn finalize(self, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult>
}
```

```rust
use iscc_lib::streaming::SumHasher;

let mut hasher = SumHasher::new();
for chunk in [b"Hello ".as_slice(), b"World".as_slice()] {
    hasher.update(chunk);
}
let result = hasher.finalize(64, false, true)?;
assert_eq!(result.filesize, 11);
assert_eq!(
    result.units,
    Some(vec![
        "ISCC:GAAW53FRSZTRHOFE".to_string(),
        "ISCC:IAAUD6BZIEI6W4J2".to_string(),
    ])
);
```

## Constants

Algorithm parameters shared with the reference implementation. The `META_TRIM_*` constants require
the `meta-code` feature (on by default).

```rust
pub const META_TRIM_NAME: usize = 128;
pub const META_TRIM_DESCRIPTION: usize = 4096;
pub const META_TRIM_META: usize = 128_000;
pub const IO_READ_SIZE: usize = 4_194_304;
pub const TEXT_NGRAM_SIZE: usize = 13;
```

| Constant                | Meaning                                                 |
| ----------------------- | ------------------------------------------------------- |
| `META_TRIM_NAME`        | Max UTF-8 byte length of the normalized `name`          |
| `META_TRIM_DESCRIPTION` | Max UTF-8 byte length of the normalized `description`   |
| `META_TRIM_META`        | Max decoded payload size in bytes of the `meta` element |
| `IO_READ_SIZE`          | Buffer size in bytes for streaming file reads (4 MB)    |
| `TEXT_NGRAM_SIZE`       | Character n-gram width for Text-Code features           |

## Diagnostics

### conformance_selftest

```rust
pub fn conformance_selftest() -> bool
```

Runs the vendored conformance vectors (every `gen_*_v0` section plus `encode_seq` / `decode_seq`)
and returns `true` if all enabled tests pass. Sections whose feature flags are disabled are skipped.
Does not panic: failures are logged to stderr.

```rust
use iscc_lib::conformance_selftest;

assert!(conformance_selftest());
```

## Types

### IsccError

Error type for ISCC operations, at the crate root (`use iscc_lib::IsccError;`). It supports the Tier
1 API: every fallible Tier 1 function returns it, and each binding maps it to its own error idiom.

```rust
pub enum IsccError {
    InvalidInput(String),
}
```

Every fallible function returns `IsccResult<T>`, an alias for `Result<T, IsccError>`. The only error
variant is `InvalidInput`, used for malformed or out-of-range parameters.

### Codec types

The `codec` module is Tier 2: part of the public Rust API, but not exposed through the bindings.
Import its items from the module path, for example `use iscc_lib::codec::{MainType, SubType};`.

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

- **`encode_component`** — enum-typed variant of the crate-root `encode_component`
- **`decode_header`** — decode ISCC header bytes into `(MainType, SubType, Version, length, tail)`
- **`encode_base32` / `decode_base32`** — RFC 4648 base32 encoding without padding

## Error Handling

Every `gen_*` function returns `IsccResult<T>` with its own result struct as `T`. Errors are
returned as `IsccError::InvalidInput` with a descriptive message. Use standard Rust error handling:

```rust
use iscc_lib::{gen_text_code_v0, IsccError};

match gen_text_code_v0("Hello World", 64) {
    Ok(result) => println!("Generated: {}", result.iscc),
    Err(IsccError::InvalidInput(msg)) => eprintln!("Invalid input: {msg}"),
}
```
