---
icon: lucide/globe
description: Guide to using iscc-lib via WebAssembly — code generation, streaming, codec operations, and constants.
---

# WebAssembly

A guide to using iscc-lib from the browser or Node.js via the `@iscc/wasm` WebAssembly package.
Covers installation, setup for different environments, code generation, streaming, codec operations,
and constants.

---

## Installation

```bash
npm install @iscc/wasm
```

## Setup

The WASM package supports three build targets. Choose the setup that matches your environment.

### Bundler (webpack, Vite)

For projects using a JavaScript bundler:

```javascript
import init, {
    gen_text_code_v0,
    gen_data_code_v0,
    DataHasher,
} from "@iscc/wasm";

// Initialize the WASM module before calling any functions
await init();

const iscc = gen_text_code_v0("Hello World");
console.log(iscc);
```

Most bundlers handle WASM initialization automatically. If not, call `init()` once at application
startup.

### Browser (native ESM)

For direct use in browsers without a bundler, build the package with `--target web` and serve the
files:

```html
<script type="module">
 import init, { gen_text_code_v0 } from "./pkg/iscc_wasm.js";

  await init();
  const iscc = gen_text_code_v0("Hello World");
  document.getElementById("result").textContent = iscc;
</script>
```

### Node.js

For Node.js environments where native addons are not available, build with `--target nodejs`:

```javascript
import {
    gen_text_code_v0,
    gen_data_code_v0
} from "@iscc/wasm";

const iscc = gen_text_code_v0("Hello World");
console.log(iscc);
```

!!! tip "Prefer @iscc/lib for Node.js"

    For Node.js server applications, the native addon [`@iscc/lib`](nodejs.md) provides better
    performance than the WASM package. Use `@iscc/wasm` for Node.js only when native addon installation
    is not possible (e.g., restricted build environments).

## Code generation

All 9 `gen_*_v0` functions return the ISCC code as a string. Optional parameters use `undefined` for
defaults.

### Meta-Code

```javascript
import {
    gen_meta_code_v0
} from "@iscc/wasm";

const iscc = gen_meta_code_v0("Die Unendliche Geschichte", "Von Michael Ende");
console.log(iscc); // "ISCC:AAA..."

// With structured metadata (JSON string)
const meta = JSON.stringify({
    title: "Example",
    author: "Author"
});
const iscc2 = gen_meta_code_v0("Example Title", undefined, meta);
console.log(iscc2);
```

### Text-Code

```javascript
import {
    gen_text_code_v0
} from "@iscc/wasm";

const iscc = gen_text_code_v0("Hello World");
console.log(iscc); // "ISCC:EAA..."
```

### Image-Code

```javascript
import {
    gen_image_code_v0
} from "@iscc/wasm";

// 32x32 grayscale thumbnail as Uint8Array (1024 bytes)
const pixels = new Uint8Array(1024).fill(128);
const iscc = gen_image_code_v0(pixels);
console.log(iscc); // "ISCC:EEA..."
```

### Audio-Code

```javascript
import {
    gen_audio_code_v0
} from "@iscc/wasm";

// Chromaprint feature vector (signed integers)
const fingerprint = [123456, -789012, 345678, 901234];
const iscc = gen_audio_code_v0(fingerprint);
console.log(iscc); // "ISCC:EIA..."
```

### Video-Code

```javascript
import {
    gen_video_code_v0
} from "@iscc/wasm";

// MPEG-7 frame signatures: array of arrays of integers
const frameSigs = [new Array(380).fill(0), new Array(380).fill(1)];
const iscc = gen_video_code_v0(frameSigs);
console.log(iscc); // "ISCC:EMA..."
```

### Mixed-Code

```javascript
import {
    gen_text_code_v0,
    gen_image_code_v0,
    gen_mixed_code_v0
} from "@iscc/wasm";

const textCode = gen_text_code_v0("Hello World");
const imageCode = gen_image_code_v0(new Uint8Array(1024).fill(128));
const iscc = gen_mixed_code_v0([textCode, imageCode]);
console.log(iscc); // "ISCC:EQA..."
```

### Data-Code

```javascript
import {
    gen_data_code_v0
} from "@iscc/wasm";

const encoder = new TextEncoder();
const data = encoder.encode("Hello World".repeat(1000));
const iscc = gen_data_code_v0(data);
console.log(iscc); // "ISCC:GAA..."
```

### Instance-Code

```javascript
import {
    gen_instance_code_v0
} from "@iscc/wasm";

const encoder = new TextEncoder();
const data = encoder.encode("Hello World");
const iscc = gen_instance_code_v0(data);
console.log(iscc); // "ISCC:IAA..."
```

### ISCC-CODE

```javascript
import {
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_iscc_code_v0
} from "@iscc/wasm";

const encoder = new TextEncoder();
const data = encoder.encode("Hello World".repeat(1000));
const dataCode = gen_data_code_v0(data);
const instanceCode = gen_instance_code_v0(data);
const iscc = gen_iscc_code_v0([dataCode, instanceCode]);
console.log(iscc); // "ISCC:KAA..."
```

## Binary data

WASM uses `Uint8Array` for binary data (not Node.js `Buffer`). When working with files in the
browser, convert `File` or `Blob` objects to `Uint8Array`:

```javascript
import {
    gen_data_code_v0
} from "@iscc/wasm";

// From a File input element
const file = document.getElementById("fileInput").files[0];
const arrayBuffer = await file.arrayBuffer();
const data = new Uint8Array(arrayBuffer);
const iscc = gen_data_code_v0(data);
```

When working with `fetch` responses:

```javascript
const response = await fetch("https://example.com/document.pdf");
const buffer = await response.arrayBuffer();
const data = new Uint8Array(buffer);
const iscc = gen_data_code_v0(data);
```

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks.

### DataHasher

```javascript
import {
    DataHasher
} from "@iscc/wasm";

const hasher = new DataHasher();

// Process file in chunks (e.g., from a ReadableStream)
const response = await fetch("https://example.com/large_file.bin");
const reader = response.body.getReader();

while (true) {
    const {
        done,
        value
    } = await reader.read();
    if (done) break;
    hasher.update(value); // value is a Uint8Array
}

const iscc = hasher.finalize();
console.log(iscc);
```

### InstanceHasher

```javascript
import {
    InstanceHasher
} from "@iscc/wasm";

const hasher = new InstanceHasher();

// Process data in chunks
hasher.update(new Uint8Array([1, 2, 3]));
hasher.update(new Uint8Array([4, 5, 6]));

const iscc = hasher.finalize();
console.log(iscc);
```

Both hashers accept `Uint8Array` input. After calling `finalize()`, the hasher is consumed and
further calls throw an error.

## Text utilities

Text normalization functions are available for preprocessing:

```javascript
import {
    text_clean,
    text_collapse,
    text_remove_newlines,
    text_trim
} from "@iscc/wasm";

// Normalize text for display
const cleaned = text_clean("  Hello\r\n\r\n\r\nWorld  ");

// Simplify text for similarity hashing
const collapsed = text_collapse("Hello, World!");

// Remove newlines, collapse whitespace
const singleLine = text_remove_newlines("Hello\nWorld");

// Trim to UTF-8 byte limit
const trimmed = text_trim("Hello World", 5);
```

## Codec operations

Functions for encoding, decoding, and decomposing ISCC codes. These operate on the ISCC binary
format defined in ISO 24138.

### Encode and decode

Construct an ISCC unit from raw header fields and digest, then decode it back:

```javascript
import {
    encode_component,
    iscc_decode
} from "@iscc/wasm";

// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
const digest = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
const code = encode_component(0, 0, 0, 64, digest);
console.log(code); // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into header components and digest
const result = iscc_decode(code);
console.log(`Maintype: ${result.maintype}, Subtype: ${result.subtype}`);
console.log(`Version: ${result.version}, Length: ${result.length}`);
console.log(`Digest:`, result.digest);
```

`iscc_decode` returns an `IsccDecodeResult` object with `maintype`, `subtype`, `version`, `length`
(length index), and `digest` (Uint8Array) fields.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```javascript
import {
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_iscc_code_v0,
    iscc_decompose
} from "@iscc/wasm";

const encoder = new TextEncoder();
const data = encoder.encode("Hello World".repeat(1000));
const dataCode = gen_data_code_v0(data);
const instanceCode = gen_instance_code_v0(data);
const isccCode = gen_iscc_code_v0([dataCode, instanceCode]);

// Decompose into individual units
const units = iscc_decompose(isccCode);
for (const unit of units) {
    console.log(unit); // Each unit code (without "ISCC:" prefix)
}
```

### Other codec functions

- `encode_base64(data)` — encode a Uint8Array to base64 string
- `json_to_data_url(json)` — convert a JSON string to a `data:application/json;base64,...` URL
- `soft_hash_video_v0(frameSigs, bits?)` — compute a video similarity hash from MPEG-7 frame
    signatures, returns Uint8Array

## Constants

Constants are exposed as getter functions:

```javascript
import {
    meta_trim_name,
    meta_trim_description,
    io_read_size,
    text_ngram_size,
} from "@iscc/wasm";

meta_trim_name(); // 128 — max byte length for name normalization
meta_trim_description(); // 4096 — max byte length for description normalization
io_read_size(); // 4_194_304 — default read buffer size (4 MB)
text_ngram_size(); // 13 — n-gram size for text similarity hashing
```

## Conformance testing

Verify the library against official test vectors:

```javascript
import {
    conformance_selftest
} from "@iscc/wasm";

console.log(conformance_selftest()); // true
```

## Error handling

Functions throw on invalid input:

```javascript
import {
    gen_text_code_v0
} from "@iscc/wasm";

try {
    gen_text_code_v0("Hello", 13); // bits must be a multiple of 32
} catch (error) {
    console.error(`Invalid input: ${error.message}`);
}
```
