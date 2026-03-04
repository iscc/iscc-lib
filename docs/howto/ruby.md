---
icon: lucide/gem
description: Guide to using iscc-lib from Ruby — code generation, streaming, codec operations, constants, and utilities.
---

# Ruby

A guide to using iscc-lib from Ruby. The Ruby gem provides a native Rust extension via
[Magnus](https://github.com/matsadler/magnus), with precompiled gems for common platforms. All
functions are module-level calls on `IsccLib` — install the gem and `require "iscc_lib"`.

---

## Installation

```bash
gem install iscc-lib
```

Or add to your `Gemfile`:

```ruby
gem "iscc-lib"
```

Then run `bundle install`.

!!! note "Build from source"

    If no precompiled gem is available for your platform, the gem builds from source automatically. This
    requires a Rust toolchain (`rustup`), `libclang-dev`, and Ruby development headers:

    ```bash
    # Clone the repository
    git clone https://github.com/iscc/iscc-lib.git
    cd iscc-lib/crates/iscc-rb

    # Install dependencies and compile
    bundle install
    bundle exec rake compile
    ```

## Code generation

All 10 `gen_*_v0` functions are module-level methods on `IsccLib` that return typed `Result`
objects. Results support both attribute-style (`result.iscc`) and hash-style (`result["iscc"]`)
access. Optional parameters use keyword arguments with sensible defaults (`bits: 64`).

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```ruby
require "iscc_lib"

result = IsccLib.gen_meta_code_v0("Die Unendliche Geschichte")
puts result.iscc     # "ISCC:AAA..."
puts result.name     # Normalized name
puts result.metahash # BLAKE3 hash of metadata

# With description
result = IsccLib.gen_meta_code_v0(
  "Die Unendliche Geschichte",
  description: "Von Michael Ende"
)
puts result.description # "Von Michael Ende"

# With structured metadata (JSON string)
require "json"

meta = JSON.generate({ title: "Example", author: "Author" })
result = IsccLib.gen_meta_code_v0("Example Title", meta: meta)
puts result.meta # data: URL with base64-encoded JSON
```

Parameters: `name` (String, required), `description:` (String, nil), `meta:` (String, nil), `bits:`
(Integer, default 64). Returns `MetaCodeResult` with fields `iscc`, `name`, `metahash`, and
optionally `description`, `meta`.

### Text-Code

Generate a Text-Code from plain text content:

```ruby
result = IsccLib.gen_text_code_v0("Hello World")
puts result.iscc       # "ISCC:EAA..."
puts result.characters # Number of characters processed
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```ruby
# Pre-process your image to 32x32 grayscale externally
pixels = ("\x80" * 1024).b  # Placeholder: uniform gray
result = IsccLib.gen_image_code_v0(pixels)
puts result.iscc # "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```ruby
# Obtain Chromaprint features externally
fingerprint = [123_456, -789_012, 345_678, 901_234]
result = IsccLib.gen_audio_code_v0(fingerprint)
puts result.iscc # "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```ruby
# Each frame signature is an array of 380 integers
frame1 = Array.new(380, 0)
frame2 = Array.new(380, 1)
frame_sigs = [frame1, frame2]

result = IsccLib.gen_video_code_v0(frame_sigs)
puts result.iscc # "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```ruby
text_result = IsccLib.gen_text_code_v0("Hello World")
pixels = ("\x80" * 1024).b
image_result = IsccLib.gen_image_code_v0(pixels)

result = IsccLib.gen_mixed_code_v0([text_result.iscc, image_result.iscc])
puts result.iscc  # "ISCC:EQA..."
puts result.parts # Array of input code strings
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```ruby
data = ("Hello World" * 1000).b
result = IsccLib.gen_data_code_v0(data)
puts result.iscc # "ISCC:GAA..."
```

For file data, read the file as binary:

```ruby
data = File.binread("document.pdf")
result = IsccLib.gen_data_code_v0(data)
puts result.iscc
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```ruby
data = "Hello World".b
result = IsccLib.gen_instance_code_v0(data)
puts result.iscc     # "ISCC:IAA..."
puts result.datahash # Multihash of the data
puts result.filesize # Size in bytes
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```ruby
data = ("Hello World" * 1000).b
data_result = IsccLib.gen_data_code_v0(data)
instance_result = IsccLib.gen_instance_code_v0(data)

result = IsccLib.gen_iscc_code_v0([data_result.iscc, instance_result.iscc])
puts result.iscc # "ISCC:KAA..."
```

Parameters: `codes` (Array of Strings), `wide:` (Boolean, default `false` for 128-bit output, `true`
for 256-bit).

### Sum-Code

Generate a composite ISCC-CODE from a file in a single pass:

```ruby
File.binwrite("example.bin", "Hello World" * 1000)

result = IsccLib.gen_sum_code_v0("example.bin")
puts result.iscc     # "ISCC:KAA..."
puts result.datahash # Multihash of the data
puts result.filesize # Size in bytes

# With individual unit codes
result = IsccLib.gen_sum_code_v0("example.bin", add_units: true)
puts result["units"] # Array of unit code strings
```

## Structured results

Every `gen_*_v0` function returns a typed `Result` object (a `Hash` subclass) that supports both
attribute-style and hash-style access:

```ruby
result = IsccLib.gen_meta_code_v0("Example Title", description: "Example description")

# Attribute access
puts result.iscc
puts result.name
puts result.metahash

# Hash access
puts result["iscc"]
puts result["name"]

# Iterate over keys
result.each { |key, value| puts "#{key}: #{value}" }

# JSON serialization
require "json"

puts JSON.generate(result, indent: "  ")
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
| `SumCodeResult`      | `iscc`, `datahash`, `filesize`, `units`?            |

Fields marked with `?` are optional and only present when the corresponding input was provided.

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory. Both support method chaining on `update` and follow the `new` → `update` →
`finalize` pattern.

### DataHasher

```ruby
hasher = IsccLib::DataHasher.new

File.open("large_file.bin", "rb") do |f|
  while (chunk = f.read(65_536))
    hasher.update(chunk)
  end
end

result = hasher.finalize
puts result.iscc # Identical to IsccLib.gen_data_code_v0(entire_file)
```

Method chaining:

```ruby
result = IsccLib::DataHasher.new
  .update(chunk1)
  .update(chunk2)
  .finalize(bits: 64)
```

### InstanceHasher

```ruby
hasher = IsccLib::InstanceHasher.new

File.open("large_file.bin", "rb") do |f|
  while (chunk = f.read(65_536))
    hasher.update(chunk)
  end
end

result = hasher.finalize
puts result.iscc     # Identical to IsccLib.gen_instance_code_v0(entire_file)
puts result.datahash # Multihash of the complete data
puts result.filesize # Total bytes processed
```

Both hashers are single-use: after calling `finalize`, further calls to `update` or `finalize` raise
`RuntimeError`.

## Codec and diagnostics

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```ruby
# Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
digest = ("\x01\x02\x03\x04\x05\x06\x07\x08").b
code = IsccLib.encode_component(0, 0, 0, 64, digest)
puts code # ISCC unit string (without "ISCC:" prefix)

# Decode: parse an ISCC unit string back into header components and digest
maintype, subtype, version, length, raw_digest = IsccLib.iscc_decode(code)
puts "Maintype: #{maintype}, Subtype: #{subtype}, Version: #{version}, Length: #{length}"
puts "Digest: #{raw_digest.unpack1("H*")}"
```

`iscc_decode` returns a 5-element Array: `[maintype, subtype, version, length_index, digest]` where
`digest` is a binary String.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```ruby
data = ("Hello World" * 1000).b
data_result = IsccLib.gen_data_code_v0(data)
instance_result = IsccLib.gen_instance_code_v0(data)
iscc_result = IsccLib.gen_iscc_code_v0([data_result.iscc, instance_result.iscc])

# Decompose into individual units
units = IsccLib.iscc_decompose(iscc_result.iscc)
units.each { |unit| puts unit } # Each unit code (without "ISCC:" prefix)
```

### Other codec functions

```ruby
# Encode bytes to base64url (no padding)
encoded = IsccLib.encode_base64("Hello".b)
puts encoded # "SGVsbG8"

# Convert JSON to data: URL with JCS canonicalization
url = IsccLib.json_to_data_url('{"key":"value"}')
puts url # "data:application/json;base64,..."
```

### Conformance testing

Verify that the library produces correct results for all official test vectors:

```ruby
puts IsccLib.conformance_selftest # true
```

## Text utilities

Text normalization functions used internally by the code generation pipeline are available for
preprocessing your own text inputs:

```ruby
# Normalize text for display (NFKC, control char removal, line ending normalization)
cleaned = IsccLib.text_clean("  Hello\r\n\r\n\r\nWorld  ")
puts cleaned # "Hello\n\nWorld"

# Remove newlines and collapse whitespace to single spaces
single_line = IsccLib.text_remove_newlines("Hello\nWorld\nFoo")
puts single_line # "Hello World Foo"

# Trim text so UTF-8 byte size does not exceed a limit
trimmed = IsccLib.text_trim("Hello World", 5)
puts trimmed # "Hello"

# Simplify text for similarity hashing (lowercase, strip whitespace/punctuation)
collapsed = IsccLib.text_collapse("Hello, World!")
puts collapsed # "helloworld"
```

## Algorithm primitives

Low-level algorithm functions are available for advanced use cases:

```ruby
# Sliding window: overlapping substrings of width Unicode characters
windows = IsccLib.sliding_window("Hello World", 4)
puts windows.inspect
# ["Hell", "ello", "llo ", "lo W", "o Wo", " Wor", "Worl", "orld"]
```

Additional primitives:

- `IsccLib.alg_minhash_256(features)` — compute a 256-bit MinHash digest from an Array of 32-bit
    integers, returns a binary String (32 bytes)
- `IsccLib.alg_cdc_chunks(data, utf32, avg_chunk_size)` — split binary data into content-defined
    chunks, returns an Array of binary Strings
- `IsccLib.alg_simhash(digests)` — compute a SimHash from an Array of equal-length binary Strings,
    returns a binary String
- `IsccLib.soft_hash_video_v0(frame_sigs, bits)` — compute a similarity-preserving hash from video
    frame signatures (Array of Arrays of integers), returns a binary String

## Constants

Module-level constants used by the ISCC algorithms:

```ruby
require "iscc_lib"

IsccLib::META_TRIM_NAME        # 128 — max byte length for name normalization
IsccLib::META_TRIM_DESCRIPTION # 4096 — max byte length for description normalization
IsccLib::META_TRIM_META        # 128_000 — max byte length for meta normalization
IsccLib::IO_READ_SIZE          # 4_194_304 — default read buffer size (4 MB)
IsccLib::TEXT_NGRAM_SIZE       # 13 — n-gram size for text similarity hashing
```

## Error handling

Code generation methods raise `RuntimeError` on invalid input (wrong bit count, empty data where
content is required, etc.):

```ruby
begin
  IsccLib.gen_text_code_v0("Hello", bits: 13) # bits must be a multiple of 32
rescue RuntimeError => e
  puts "Invalid input: #{e.message}"
end
```
