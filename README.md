# iscc-lib

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)

<!-- TODO: Add version badges once packages are published -->

<!-- [![Crate](https://img.shields.io/crates/v/iscc-lib.svg)](https://crates.io/crates/iscc-lib) -->

<!-- [![PyPI](https://img.shields.io/pypi/v/iscc-lib.svg)](https://pypi.org/project/iscc-lib/) -->

<!-- [![npm](https://img.shields.io/npm/v/@iscc/lib.svg)](https://www.npmjs.com/package/@iscc/lib) -->

**High-performance polyglot implementation of
[ISO 24138:2024](https://www.iso.org/standard/77899.html) --- International Standard Content Code
(ISCC)**

## Key Features

- **Similarity-Preserving**: Detect similar content even after modifications
- **Multi-Level Identification**: Identify content at metadata, semantic, perceptual, and data
    levels
- **Self-Describing**: Each component contains its own type and version information
- **ISO Standardized**: Implements the official ISO 24138:2024 specification
- **Polyglot**: Rust core with bindings for Python, Node.js, WASM, and C FFI
- **Conformance-Tested**: Validated against the official
    [iscc-core](https://github.com/iscc/iscc-core) reference implementation

## What is the ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets.

ISCCs are generated algorithmically from digital content, just like cryptographic hashes. However,
instead of using a single cryptographic hash function to identify data only, the ISCC uses various
algorithms to create a composite identifier that exhibits similarity-preserving properties (soft
hash).

The component-based structure of the ISCC identifies content at multiple levels of abstraction. Each
component is self-describing, modular, and can be used separately or with others to aid in various
content identification tasks. The algorithmic design supports content deduplication, database
synchronization, indexing, integrity verification, timestamping, versioning, data provenance,
similarity clustering, anomaly detection, usage tracking, allocation of royalties, fact-checking,
and general digital asset management use-cases.

## What is iscc-lib

`iscc-lib` is a high-performance polyglot implementation of the ISCC core algorithms as defined by
[ISO 24138](https://www.iso.org/standard/77899.html). Built in Rust with bindings for Python,
Node.js, WebAssembly, and C, it serves developers across multiple ecosystems who need fast, reliable
content identification.

`iscc-lib` is conformance-tested against the official Python reference implementation
[iscc-core](https://github.com/iscc/iscc-core) and produces identical results for all test vectors.

> **Note:** This is a low-level codec and algorithm library. It does not include features like
> media-type detection, metadata extraction, or file-format-specific content extraction. For
> higher-level features, see [iscc-sdk](https://github.com/iscc/iscc-sdk) which builds on top of the
> core algorithms.

## ISCC Architecture

![ISCC Architecture](https://raw.githubusercontent.com/iscc/iscc-core/master/docs/images/iscc-codec-light.png)

## ISCC MainTypes

| Idx | Slug     | Bits | Purpose                                                |
| --- | :------- | ---- | ------------------------------------------------------ |
| 0   | META     | 0000 | Match on metadata similarity                           |
| 1   | SEMANTIC | 0001 | Match on semantic content similarity                   |
| 2   | CONTENT  | 0010 | Match on perceptual content similarity                 |
| 3   | DATA     | 0011 | Match on data similarity                               |
| 4   | INSTANCE | 0100 | Match on data identity                                 |
| 5   | ISCC     | 0101 | Composite of two or more components with common header |

## Installation

Packages are not yet published. The install commands below will work once the initial release is
available.

### Rust

```bash
cargo add iscc-lib
```

### Python

```bash
pip install iscc-lib
```

### Node.js

```bash
npm install @iscc/lib
```

### WASM

```bash
npm install @iscc/wasm
```

## Quick Start

### Rust

```rust
use iscc_lib::gen_meta_code_v0;

let result = gen_meta_code_v0("ISCC Test Document!", None, None, 64).unwrap();
println!("Meta-Code: {}", result.iscc);
```

### Python

```python
import iscc_lib as ic

result = ic.gen_meta_code_v0("ISCC Test Document!")
print(f"Meta-Code: {result['iscc']}")
```

### Node.js

```javascript
const ic = require("@iscc/lib");

const result = ic.gen_meta_code_v0("ISCC Test Document!");
console.log(`Meta-Code: ${result.iscc}`);
```

### WASM

```javascript
import {
    gen_meta_code_v0
} from "@iscc/wasm";

const result = gen_meta_code_v0("ISCC Test Document!");
console.log(`Meta-Code: ${result.iscc}`);
```

## Implementors Guide

To build a conformant ISCC implementation, work through the following top-level entry-point
functions:

```
gen_meta_code_v0
gen_text_code_v0
gen_image_code_v0
gen_audio_code_v0
gen_video_code_v0
gen_mixed_code_v0
gen_data_code_v0
gen_instance_code_v0
gen_iscc_code_v0
```

The corresponding conformance test vectors can be found in
[`iscc_core/data.json`](https://github.com/iscc/iscc-core/blob/master/iscc_core/data.json).

For detailed per-language API guides, see the [documentation site](https://lib.iscc.codes).

## Documentation

Documentation is published at <https://lib.iscc.codes>

## Contributing

Pull requests are welcome. For significant changes, please open an issue first to discuss your
plans. Please make sure to update tests as appropriate.

You may also want to join our developer chat on Telegram at <https://t.me/iscc_dev>.

## License

Apache-2.0

## Maintainers

[@titusz](https://github.com/titusz)
