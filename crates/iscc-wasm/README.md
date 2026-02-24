# @iscc/wasm

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@iscc/wasm.svg)](https://www.npmjs.com/package/@iscc/wasm)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> **Experimental:** This library is in early development (v0.0.x). APIs may change without notice.
> Not recommended for production use yet.

Browser-compatible WebAssembly bindings for
[ISO 24138:2024](https://www.iso.org/standard/77899.html) -- International Standard Content Code
(ISCC). Built with Rust and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for use in
browsers and Node.js.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

```bash
npm install @iscc/wasm
```

## Quick Start

```javascript
import init, {
    gen_meta_code_v0
} from "@iscc/wasm";

await init();
const iscc = gen_meta_code_v0("ISCC Test Document!");
console.log(`Meta-Code: ${iscc}`);
```

Both browser and Node.js targets are supported.

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
| `gen_data_code_v0`     | Generate a Data-Code from raw bytes          |
| `gen_instance_code_v0` | Generate an Instance-Code from raw bytes     |
| `gen_iscc_code_v0`     | Generate a composite ISCC-CODE               |

All code generators return ISCC strings directly.

### Utilities

- **Text processing:** `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- **Algorithm primitives:** `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `sliding_window`
- **Soft hashing:** `soft_hash_video_v0`
- **Encoding:** `encode_base64`
- **Codec:** `iscc_decompose`
- **Streaming:** `DataHasher`, `InstanceHasher` classes for incremental processing
- **Diagnostics:** `conformance_selftest`

## Links

- [Documentation](https://lib.iscc.codes)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
