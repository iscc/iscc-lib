# iscc-lib

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![Gem Version](https://img.shields.io/gem/v/iscc-lib.svg)](https://rubygems.org/gems/iscc-lib)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

High-performance Ruby bindings for [ISO 24138:2024](https://www.iso.org/standard/77899.html) —
International Standard Content Code (ISCC). Built as a native Rust extension via
[Magnus](https://github.com/matsadler/magnus) for speed, with an idiomatic Ruby API featuring
keyword arguments and attribute-style result access.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

```bash
gem install iscc-lib
```

Or add to your `Gemfile`:

```ruby
gem "iscc-lib"
```

Precompiled gems are available for common platforms. If no precompiled gem matches your platform,
the gem builds from source (requires a Rust toolchain and `libclang-dev`).

## Quick Start

```ruby
require "iscc_lib"

# Generate a Meta-Code from content metadata
result = IsccLib.gen_meta_code_v0("Hello World", description: "A greeting")
puts result.iscc      # => "ISCC:..."
puts result.name      # => "Hello World"
puts result.metahash  # => "1e20..."

# Streaming for large files
hasher = IsccLib::DataHasher.new
File.open("large_file.bin", "rb") do |f|
  while (chunk = f.read(65_536))
    hasher.update(chunk)
  end
end
result = hasher.finalize
puts result.iscc
```

## API Overview

### Code Generators

| Function               | Description                                  |
| ---------------------- | -------------------------------------------- |
| `gen_meta_code_v0`     | Generate a Meta-Code from metadata fields    |
| `gen_text_code_v0`     | Generate a Text-Code from plain text         |
| `gen_image_code_v0`    | Generate an Image-Code from pixel data       |
| `gen_audio_code_v0`    | Generate an Audio-Code from Chromaprint data |
| `gen_video_code_v0`    | Generate a Video-Code from frame signatures  |
| `gen_mixed_code_v0`    | Generate a Mixed-Code from Content-Codes     |
| `gen_data_code_v0`     | Generate a Data-Code from binary data        |
| `gen_instance_code_v0` | Generate an Instance-Code from binary data   |
| `gen_iscc_code_v0`     | Generate a composite ISCC-CODE               |
| `gen_sum_code_v0`      | Generate an ISCC-SUM from a file path        |

### Utilities

- **Text processing:** `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- **Algorithm primitives:** `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `sliding_window`
- **Soft hashing:** `soft_hash_video_v0`
- **Encoding:** `encode_base64`, `encode_component`
- **Codec:** `iscc_decode`, `iscc_decompose`, `json_to_data_url`
- **Streaming:** `DataHasher`, `InstanceHasher` for incremental processing
- **Diagnostics:** `conformance_selftest`

## Links

- [Documentation](https://lib.iscc.codes)
- [How-to Guide](https://lib.iscc.codes/howto/ruby/)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
