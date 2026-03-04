---
icon: lucide/book-open
description: Ruby API reference for the ISCC library via native Rust extension.
---

# Ruby API Reference

Ruby gem for ISCC (ISO 24138:2024) code generation via a native Rust extension
([Magnus](https://github.com/matsadler/magnus)). All 32 Tier 1 symbols are exposed as module-level
methods, constants, and classes on `IsccLib`.

## Installation

```bash
gem install iscc-lib
```

Or add to your `Gemfile`:

```ruby
gem "iscc-lib"
```

## Quick Example

```ruby
require "iscc_lib"

result = IsccLib.gen_meta_code_v0("Die Unendliche Geschichte")
puts result.iscc # "ISCC:AAA..."
```

---

## Constants

Algorithm constants exposed as module-level constants on `IsccLib`.

| Constant                         | Value       | Description                                             |
| -------------------------------- | ----------- | ------------------------------------------------------- |
| `IsccLib::META_TRIM_NAME`        | `128`       | Maximum UTF-8 byte length for the name parameter        |
| `IsccLib::META_TRIM_DESCRIPTION` | `4096`      | Maximum UTF-8 byte length for the description parameter |
| `IsccLib::META_TRIM_META`        | `128_000`   | Maximum UTF-8 byte length for the meta parameter        |
| `IsccLib::IO_READ_SIZE`          | `4_194_304` | Default read buffer size for streaming I/O (4 MB)       |
| `IsccLib::TEXT_NGRAM_SIZE`       | `13`        | N-gram window size for text similarity hashing          |

---

## Result Classes

All `gen_*_v0` functions return typed result objects — subclasses of `IsccLib::Result`, which itself
extends `Hash`. Results support both attribute-style (`result.iscc`) and hash-style
(`result["iscc"]`) access.

| Class                         | Returned by            | Fields                                              |
| ----------------------------- | ---------------------- | --------------------------------------------------- |
| `IsccLib::MetaCodeResult`     | `gen_meta_code_v0`     | `iscc`, `name`, `metahash`, `description`?, `meta`? |
| `IsccLib::TextCodeResult`     | `gen_text_code_v0`     | `iscc`, `characters`                                |
| `IsccLib::ImageCodeResult`    | `gen_image_code_v0`    | `iscc`                                              |
| `IsccLib::AudioCodeResult`    | `gen_audio_code_v0`    | `iscc`                                              |
| `IsccLib::VideoCodeResult`    | `gen_video_code_v0`    | `iscc`                                              |
| `IsccLib::MixedCodeResult`    | `gen_mixed_code_v0`    | `iscc`, `parts`                                     |
| `IsccLib::DataCodeResult`     | `gen_data_code_v0`     | `iscc`                                              |
| `IsccLib::InstanceCodeResult` | `gen_instance_code_v0` | `iscc`, `datahash`, `filesize`                      |
| `IsccLib::IsccCodeResult`     | `gen_iscc_code_v0`     | `iscc`                                              |
| `IsccLib::SumCodeResult`      | `gen_sum_code_v0`      | `iscc`, `datahash`, `filesize`, `units`?            |

Fields marked with `?` are optional — present only when the corresponding input was provided.

```ruby
result = IsccLib.gen_meta_code_v0("Example", description: "A description")
result.iscc        # attribute-style access
result["iscc"]     # hash-style access
result.keys        # list all present fields
```

---

## Code Generation Functions

All 10 `gen_*_v0` functions return typed `Result` objects and raise `RuntimeError` on invalid input.

### gen_meta_code_v0

Generate a Meta-Code from name and optional metadata.

```ruby
IsccLib.gen_meta_code_v0(name, description: nil, meta: nil, bits: 64)
```

| Parameter      | Type          | Description                                                    |
| -------------- | ------------- | -------------------------------------------------------------- |
| `name`         | `String`      | Title or name of the content (required, trimmed to 128 bytes)  |
| `description:` | `String, nil` | Optional description text (trimmed to 4096 bytes)              |
| `meta:`        | `String, nil` | Optional JSON string or `data:` URL                            |
| `bits:`        | `Integer`     | Bit length of the generated code (default: 64, multiple of 32) |

Returns `MetaCodeResult` with `iscc`, `name`, `metahash`, and optionally `description`, `meta`.

```ruby
result = IsccLib.gen_meta_code_v0(
  "Die Unendliche Geschichte",
  description: "Von Michael Ende"
)
puts result.iscc
```

---

### gen_text_code_v0

Generate a Text-Code from plain text content using MinHash over character n-grams.

```ruby
IsccLib.gen_text_code_v0(text, bits: 64)
```

| Parameter | Type      | Description                                                    |
| --------- | --------- | -------------------------------------------------------------- |
| `text`    | `String`  | Plain text content (required)                                  |
| `bits:`   | `Integer` | Bit length of the generated code (default: 64, multiple of 32) |

Returns `TextCodeResult` with `iscc` and `characters`.

```ruby
result = IsccLib.gen_text_code_v0("Hello World")
puts result.characters # number of characters processed
```

---

### gen_image_code_v0

Generate an Image-Code from 1024 grayscale pixel bytes using DCT-based perceptual hashing.

```ruby
IsccLib.gen_image_code_v0(pixels, bits: 64)
```

| Parameter | Type      | Description                                             |
| --------- | --------- | ------------------------------------------------------- |
| `pixels`  | `String`  | 32×32 grayscale pixel data (1024 bytes, binary encoded) |
| `bits:`   | `Integer` | Bit length of the generated code (default: 64, max 256) |

Returns `ImageCodeResult` with `iscc`.

```ruby
pixels = ("\x80" * 1024).b
result = IsccLib.gen_image_code_v0(pixels)
```

---

### gen_audio_code_v0

Generate an Audio-Code from a Chromaprint feature vector using multi-stage SimHash.

```ruby
IsccLib.gen_audio_code_v0(cv, bits: 64)
```

| Parameter | Type             | Description                                                    |
| --------- | ---------------- | -------------------------------------------------------------- |
| `cv`      | `Array<Integer>` | Signed 32-bit Chromaprint feature values                       |
| `bits:`   | `Integer`        | Bit length of the generated code (default: 64, multiple of 32) |

Returns `AudioCodeResult` with `iscc`.

```ruby
fingerprint = [123_456, -789_012, 345_678]
result = IsccLib.gen_audio_code_v0(fingerprint)
```

---

### gen_video_code_v0

Generate a Video-Code from MPEG-7 frame signature vectors using WTA-Hash and SimHash.

```ruby
IsccLib.gen_video_code_v0(frame_sigs, bits: 64)
```

| Parameter    | Type                    | Description                                                    |
| ------------ | ----------------------- | -------------------------------------------------------------- |
| `frame_sigs` | `Array<Array<Integer>>` | Array of MPEG-7 frame signature vectors (each 380 integers)    |
| `bits:`      | `Integer`               | Bit length of the generated code (default: 64, multiple of 32) |

Returns `VideoCodeResult` with `iscc`.

```ruby
frame_sigs = [Array.new(380, 0), Array.new(380, 1)]
result = IsccLib.gen_video_code_v0(frame_sigs)
```

---

### gen_mixed_code_v0

Generate a Mixed-Code from multiple Content-Code strings using SimHash combination.

```ruby
IsccLib.gen_mixed_code_v0(codes, bits: 64)
```

| Parameter | Type            | Description                                                    |
| --------- | --------------- | -------------------------------------------------------------- |
| `codes`   | `Array<String>` | Array of ISCC Content-Code strings (requires >= 2)             |
| `bits:`   | `Integer`       | Bit length of the generated code (default: 64, multiple of 32) |

Returns `MixedCodeResult` with `iscc` and `parts`.

```ruby
result = IsccLib.gen_mixed_code_v0([text_code, image_code])
puts result.parts # Array of input code strings
```

---

### gen_data_code_v0

Generate a Data-Code from raw byte data using content-defined chunking and MinHash.

```ruby
IsccLib.gen_data_code_v0(data, bits: 64)
```

| Parameter | Type      | Description                                                    |
| --------- | --------- | -------------------------------------------------------------- |
| `data`    | `String`  | Raw binary data                                                |
| `bits:`   | `Integer` | Bit length of the generated code (default: 64, multiple of 32) |

Returns `DataCodeResult` with `iscc`.

```ruby
data = File.binread("document.pdf")
result = IsccLib.gen_data_code_v0(data)
```

---

### gen_instance_code_v0

Generate an Instance-Code from raw byte data using BLAKE3 hashing.

```ruby
IsccLib.gen_instance_code_v0(data, bits: 64)
```

| Parameter | Type      | Description                                                    |
| --------- | --------- | -------------------------------------------------------------- |
| `data`    | `String`  | Raw binary data                                                |
| `bits:`   | `Integer` | Bit length of the generated code (default: 64, multiple of 32) |

Returns `InstanceCodeResult` with `iscc`, `datahash`, and `filesize`.

```ruby
result = IsccLib.gen_instance_code_v0("Hello World".b)
puts result.datahash # Multihash of the data
puts result.filesize # Size in bytes
```

---

### gen_iscc_code_v0

Generate a composite ISCC-CODE from individual unit codes.

```ruby
IsccLib.gen_iscc_code_v0(codes, wide: false)
```

| Parameter | Type            | Description                                                    |
| --------- | --------------- | -------------------------------------------------------------- |
| `codes`   | `Array<String>` | Array of ISCC unit code strings (must include Data + Instance) |
| `wide:`   | `Boolean`       | `true` for 256-bit output, `false` for 128-bit (default)       |

Returns `IsccCodeResult` with `iscc`.

```ruby
data = ("Hello World" * 1000).b
data_code = IsccLib.gen_data_code_v0(data).iscc
instance_code = IsccLib.gen_instance_code_v0(data).iscc
result = IsccLib.gen_iscc_code_v0([data_code, instance_code])
```

---

### gen_sum_code_v0

Generate a composite ISCC-CODE from a file in a single pass. Reads the file once, feeding both a
DataHasher (CDC/MinHash) and an InstanceHasher (BLAKE3).

```ruby
IsccLib.gen_sum_code_v0(path, bits: 64, wide: false, add_units: false)
```

| Parameter    | Type      | Description                                                |
| ------------ | --------- | ---------------------------------------------------------- |
| `path`       | `String`  | File path                                                  |
| `bits:`      | `Integer` | Bit length of the generated code (default: 64)             |
| `wide:`      | `Boolean` | Enable 256-bit wide mode (default: `false`)                |
| `add_units:` | `Boolean` | Include individual unit codes in result (default: `false`) |

Returns `SumCodeResult` with `iscc`, `datahash`, `filesize`, and optionally `units`.

```ruby
result = IsccLib.gen_sum_code_v0("document.pdf", add_units: true)
puts result.iscc
puts result["units"] # Array of unit code strings
```

---

## Text Utilities

Text processing functions for normalization and cleaning. Used internally by code generation but
available for preprocessing.

### text_clean

Clean and normalize text for display. Applies NFKC normalization, removes control characters (except
newlines), normalizes CRLF to LF, collapses consecutive empty lines, and strips leading/trailing
whitespace.

```ruby
IsccLib.text_clean(text)
```

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `text`    | `String` | Input text  |

```ruby
cleaned = IsccLib.text_clean("  Hello\r\n\r\n\r\nWorld  ")
# => "Hello\n\nWorld"
```

---

### text_remove_newlines

Remove newlines and collapse whitespace to single spaces.

```ruby
IsccLib.text_remove_newlines(text)
```

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `text`    | `String` | Input text  |

```ruby
IsccLib.text_remove_newlines("Hello\nWorld\nFoo")
# => "Hello World Foo"
```

---

### text_trim

Trim text so its UTF-8 encoded size does not exceed `nbytes`. Multi-byte characters that would be
split are dropped entirely. Leading/trailing whitespace is stripped from the result.

```ruby
IsccLib.text_trim(text, nbytes)
```

| Parameter | Type      | Description               |
| --------- | --------- | ------------------------- |
| `text`    | `String`  | Input text                |
| `nbytes`  | `Integer` | Maximum UTF-8 byte length |

```ruby
IsccLib.text_trim("Hello World", 5)
# => "Hello"
```

---

### text_collapse

Normalize and simplify text for similarity hashing. Applies NFD normalization, lowercasing, removes
whitespace and characters in Unicode categories C, M, and P, then recombines with NFKC.

```ruby
IsccLib.text_collapse(text)
```

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `text`    | `String` | Input text  |

```ruby
IsccLib.text_collapse("Hello, World!")
# => "helloworld"
```

---

## Encoding & Codec

### encode_base64

Encode bytes as base64url (RFC 4648 section 5, no padding).

```ruby
IsccLib.encode_base64(data)
```

| Parameter | Type     | Description                |
| --------- | -------- | -------------------------- |
| `data`    | `String` | Binary data (`.b` encoded) |

```ruby
IsccLib.encode_base64("Hello".b)
# => "SGVsbG8"
```

---

### json_to_data_url

Convert a JSON string to a data URL with base64 encoding. Uses `application/ld+json` media type when
the JSON contains an `@context` key, otherwise `application/json`.

```ruby
IsccLib.json_to_data_url(json)
```

| Parameter | Type     | Description       |
| --------- | -------- | ----------------- |
| `json`    | `String` | Valid JSON string |

Raises `RuntimeError` on invalid JSON input.

```ruby
IsccLib.json_to_data_url('{"key":"value"}')
# => "data:application/json;base64,..."
```

---

### encode_component

Encode header fields and a raw digest into a base32-encoded ISCC unit string (without `ISCC:`
prefix).

```ruby
IsccLib.encode_component(mtype, stype, version, bit_length, digest)
```

| Parameter    | Type      | Description                                 |
| ------------ | --------- | ------------------------------------------- |
| `mtype`      | `Integer` | MainType enum value (0–255)                 |
| `stype`      | `Integer` | SubType enum value (0–255)                  |
| `version`    | `Integer` | Version enum value (0–255)                  |
| `bit_length` | `Integer` | Digest bit length (multiple of 32)          |
| `digest`     | `String`  | Raw digest bytes (length >= bit_length / 8) |

Raises `RuntimeError` on invalid input.

```ruby
digest = ("\x01\x02\x03\x04\x05\x06\x07\x08").b
code = IsccLib.encode_component(0, 0, 0, 64, digest)
```

---

### iscc_decompose

Decompose a composite ISCC-CODE into individual ISCC-UNITs.

```ruby
IsccLib.iscc_decompose(iscc_code)
```

| Parameter   | Type     | Description                |
| ----------- | -------- | -------------------------- |
| `iscc_code` | `String` | Composite ISCC-CODE string |

Returns an `Array<String>` of base32-encoded ISCC-UNIT strings (without `ISCC:` prefix). Raises
`RuntimeError` on invalid input.

```ruby
units = IsccLib.iscc_decompose("ISCC:KAA...")
units.each { |unit| puts unit }
```

---

### iscc_decode

Decode an ISCC unit string into its header components and raw digest.

```ruby
IsccLib.iscc_decode(iscc_unit)
```

| Parameter   | Type     | Description      |
| ----------- | -------- | ---------------- |
| `iscc_unit` | `String` | ISCC unit string |

Returns a 5-element `Array`: `[maintype, subtype, version, length, digest]` where `digest` is a
binary `String`. Raises `RuntimeError` on invalid input.

```ruby
maintype, subtype, version, length, digest = IsccLib.iscc_decode("ISCC:EAASKDNZNYGUUF5A")
puts "Maintype: #{maintype}, Subtype: #{subtype}" # "Maintype: 2, Subtype: 0"
puts "Digest: #{digest.unpack1("H*")}"
```

---

## Algorithm Primitives

Low-level algorithm functions for SimHash, MinHash, content-defined chunking, video hashing, and
sliding window n-grams.

### sliding_window

Generate sliding window n-grams from a string.

```ruby
IsccLib.sliding_window(text, width)
```

| Parameter | Type      | Description                                       |
| --------- | --------- | ------------------------------------------------- |
| `text`    | `String`  | Input string                                      |
| `width`   | `Integer` | Window width in Unicode characters (must be >= 2) |

Returns `Array<String>`. Raises `RuntimeError` if width < 2.

```ruby
IsccLib.sliding_window("Hello World", 3)
# => ["Hel", "ell", "llo", "lo ", "o W", " Wo", "Wor", "orl", "rld"]
```

---

### alg_simhash

Compute a SimHash from a sequence of equal-length hash digests. Output length matches the input
digest length.

```ruby
IsccLib.alg_simhash(hash_digests)
```

| Parameter      | Type            | Description                                 |
| -------------- | --------------- | ------------------------------------------- |
| `hash_digests` | `Array<String>` | Array of equal-length binary digest strings |

Returns a binary `String`. Raises `RuntimeError` on empty input or unequal digest lengths.

```ruby
digests = ["\x01\x02\x03\x04".b, "\xFF\xFE\xFD\xFC".b]
result = IsccLib.alg_simhash(digests)
```

---

### alg_minhash_256

Compute a 256-bit MinHash digest from 32-bit integer features. Uses 64 universal hash functions with
bit-interleaved compression.

```ruby
IsccLib.alg_minhash_256(features)
```

| Parameter  | Type             | Description                         |
| ---------- | ---------------- | ----------------------------------- |
| `features` | `Array<Integer>` | Array of 32-bit integer hash values |

Returns a 32-byte binary `String`.

```ruby
features = [100, 200, 300, 400]
digest = IsccLib.alg_minhash_256(features)
```

---

### alg_cdc_chunks

Split data into content-defined chunks using gear rolling hash.

```ruby
IsccLib.alg_cdc_chunks(data, utf32, avg_chunk_size)
```

| Parameter        | Type      | Description                                      |
| ---------------- | --------- | ------------------------------------------------ |
| `data`           | `String`  | Raw binary data                                  |
| `utf32`          | `Boolean` | If `true`, align cut points to 4-byte boundaries |
| `avg_chunk_size` | `Integer` | Target average chunk size (default 1024)         |

Returns `Array<String>` of binary chunk strings.

```ruby
chunks = IsccLib.alg_cdc_chunks("Hello World".b * 100, false, 1024)
```

---

### soft_hash_video_v0

Compute a similarity-preserving hash from video frame signatures.

```ruby
IsccLib.soft_hash_video_v0(frame_sigs, bits)
```

| Parameter    | Type                    | Description                             |
| ------------ | ----------------------- | --------------------------------------- |
| `frame_sigs` | `Array<Array<Integer>>` | Array of MPEG-7 frame signature vectors |
| `bits`       | `Integer`               | Output bit length                       |

Returns a binary `String` of length `bits / 8`. Raises `RuntimeError` if input is empty.

```ruby
frame_sigs = [Array.new(380, 0), Array.new(380, 1)]
hash = IsccLib.soft_hash_video_v0(frame_sigs, 256)
```

---

## Streaming Hashers

Streaming hashers process large data incrementally without loading the entire content into memory.
Each hasher follows the lifecycle: `new` → `update` (repeated) → `finalize`.

Both hashers are single-use: after calling `finalize`, subsequent calls to `update` or `finalize`
raise `RuntimeError`. The `update` method returns `self` for method chaining.

### DataHasher

Streaming Data-Code generator using content-defined chunking and MinHash.

```ruby
hasher = IsccLib::DataHasher.new
```

#### update

```ruby
hasher.update(data) # => self
```

| Parameter | Type     | Description                |
| --------- | -------- | -------------------------- |
| `data`    | `String` | Binary data (`.b` encoded) |

Returns `self` for method chaining.

#### finalize

```ruby
hasher.finalize(bits: 64)
```

| Parameter | Type      | Description                                    |
| --------- | --------- | ---------------------------------------------- |
| `bits:`   | `Integer` | Bit length of the generated code (default: 64) |

Returns `DataCodeResult`.

#### Example

```ruby
hasher = IsccLib::DataHasher.new

File.open("large_file.bin", "rb") do |f|
  while (chunk = f.read(65_536))
    hasher.update(chunk)
  end
end

result = hasher.finalize(bits: 64)
puts result.iscc
```

Method chaining:

```ruby
result = IsccLib::DataHasher.new
  .update(chunk1)
  .update(chunk2)
  .finalize(bits: 64)
```

---

### InstanceHasher

Streaming Instance-Code generator using BLAKE3.

```ruby
hasher = IsccLib::InstanceHasher.new
```

#### update

```ruby
hasher.update(data) # => self
```

| Parameter | Type     | Description                |
| --------- | -------- | -------------------------- |
| `data`    | `String` | Binary data (`.b` encoded) |

Returns `self` for method chaining.

#### finalize

```ruby
hasher.finalize(bits: 64)
```

| Parameter | Type      | Description                                    |
| --------- | --------- | ---------------------------------------------- |
| `bits:`   | `Integer` | Bit length of the generated code (default: 64) |

Returns `InstanceCodeResult` with `iscc`, `datahash`, and `filesize`.

#### Example

```ruby
hasher = IsccLib::InstanceHasher.new

File.open("large_file.bin", "rb") do |f|
  while (chunk = f.read(65_536))
    hasher.update(chunk)
  end
end

result = hasher.finalize
puts result.iscc
puts result.datahash
puts result.filesize
```

---

## Diagnostics

### conformance_selftest

Run all ISCC conformance tests against vendored test vectors.

```ruby
IsccLib.conformance_selftest
```

Returns `true` if all tests pass, `false` otherwise.

```ruby
puts IsccLib.conformance_selftest # true
```

---

## Error Handling

All methods that accept user input raise `RuntimeError` on invalid arguments:

- Empty or nil required parameters (e.g., empty `name` in `gen_meta_code_v0`)
- Invalid `bits` values (must be a multiple of 32, range 32–256)
- Malformed ISCC strings passed to `iscc_decode` or `iscc_decompose`
- Streaming hasher operations after finalization

```ruby
begin
  IsccLib.gen_text_code_v0("Hello", bits: 13)
rescue RuntimeError => e
  puts "Error: #{e.message}"
end
```
