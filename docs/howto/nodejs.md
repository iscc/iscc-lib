---
icon: lucide/hexagon
description: Guide to using iscc-lib from Node.js — code generation, streaming, codec operations, and constants.
---

# Node.js

A guide to using iscc-lib from Node.js via the `@iscc/lib` native addon. Covers installation, code
generation, streaming, codec operations, and constants.

---

## Installation

```bash
npm install @iscc/lib
```

The package includes pre-built native binaries for common platforms (Linux x64, macOS x64/arm64,
Windows x64). No Rust toolchain is required for installation.

## Import

```javascript
import {
    gen_text_code_v0,
    gen_meta_code_v0,
    gen_data_code_v0,
    gen_instance_code_v0,
    DataHasher,
    InstanceHasher,
} from "@iscc/lib";
```

All function names use `snake_case` to match the Python and Rust APIs.

## Code generation

All 9 `gen_*_v0` functions return the ISCC code as a string (prefixed with `ISCC:`). Optional
parameters use `null` or `undefined` for defaults.

### Meta-Code

```javascript
import {
    gen_meta_code_v0
} from "@iscc/lib";

const iscc = gen_meta_code_v0("Die Unendliche Geschichte", "Von Michael Ende");
console.log(iscc); // "ISCC:AAA..."

// With structured metadata (JSON string)
const meta = JSON.stringify({
    title: "Example",
    author: "Author"
});
const iscc2 = gen_meta_code_v0("Example Title", null, meta);
console.log(iscc2);
```

Parameters: `name`, `description?`, `meta?`, `bits?` (default 64).

### Text-Code

```javascript
import {
    gen_text_code_v0
} from "@iscc/lib";

const iscc = gen_text_code_v0("Hello World");
console.log(iscc); // "ISCC:EAA..."
```

### Image-Code

```javascript
import {
    gen_image_code_v0
} from "@iscc/lib";

// 32x32 grayscale thumbnail as Buffer (1024 bytes)
const pixels = Buffer.alloc(1024, 128);
const iscc = gen_image_code_v0(pixels);
console.log(iscc); // "ISCC:EEA..."
```

### Audio-Code

```javascript
import {
    gen_audio_code_v0
} from "@iscc/lib";

// Chromaprint feature vector (signed integers)
const fingerprint = [123456, -789012, 345678, 901234];
const iscc = gen_audio_code_v0(fingerprint);
console.log(iscc); // "ISCC:EIA..."
```

### Video-Code

```javascript
import {
    gen_video_code_v0
} from "@iscc/lib";

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
} from "@iscc/lib";

const textCode = gen_text_code_v0("Hello World");
const imageCode = gen_image_code_v0(Buffer.alloc(1024, 128));
const iscc = gen_mixed_code_v0([textCode, imageCode]);
console.log(iscc); // "ISCC:EQA..."
```

### Data-Code

```javascript
import {
    readFileSync
} from "node:fs";
import {
    gen_data_code_v0
} from "@iscc/lib";

const data = readFileSync("document.pdf");
const iscc = gen_data_code_v0(data);
console.log(iscc); // "ISCC:GAA..."
```

### Instance-Code

```javascript
import {
    readFileSync
} from "node:fs";
import {
    gen_instance_code_v0
} from "@iscc/lib";

const data = readFileSync("document.pdf");
const iscc = gen_instance_code_v0(data);
console.log(iscc); // "ISCC:IAA..."
```

### ISCC-CODE

```javascript
import {
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_iscc_code_v0
} from "@iscc/lib";

const data = Buffer.from("Hello World".repeat(1000));
const dataCode = gen_data_code_v0(data);
const instanceCode = gen_instance_code_v0(data);
const iscc = gen_iscc_code_v0([dataCode, instanceCode]);
console.log(iscc); // "ISCC:KAA..."
```

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data in chunks without loading
everything into memory.

### DataHasher

```javascript
import {
    createReadStream
} from "node:fs";
import {
    DataHasher
} from "@iscc/lib";

const hasher = new DataHasher();

const stream = createReadStream("large_file.bin");
for await (const chunk of stream) {
    hasher.update(chunk);
}

const iscc = hasher.finalize();
console.log(iscc); // Identical to gen_data_code_v0(entireFile)
```

### InstanceHasher

```javascript
import {
    createReadStream
} from "node:fs";
import {
    InstanceHasher
} from "@iscc/lib";

const hasher = new InstanceHasher();

const stream = createReadStream("large_file.bin");
for await (const chunk of stream) {
    hasher.update(chunk);
}

const iscc = hasher.finalize();
console.log(iscc); // Identical to gen_instance_code_v0(entireFile)
```

Both hashers accept `Buffer` input in `update()`. After calling `finalize()`, the hasher is consumed
and further calls throw an error.

The optional `bits` parameter can be passed to `finalize()`:

```javascript
const iscc = hasher.finalize(128); // 128-bit code
```

## Text utilities

Text normalization functions are available for preprocessing:

```javascript
import {
    text_clean,
    text_collapse,
    text_remove_newlines,
    text_trim
} from "@iscc/lib";

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
const {
    encode_component,
    iscc_decode
} = require("@iscc/lib");

// Encode: maintype=0 (Meta), subtype=0, version=0, 64 bits, 8-byte digest
const digest = Buffer.from([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
const code = encode_component(0, 0, 0, 64, digest);
console.log(code); // ISCC unit string (without "ISCC:" prefix)

// Decode: parse an ISCC unit string back into header components and digest
const result = iscc_decode(code);
console.log(`Maintype: ${result.maintype}, Subtype: ${result.subtype}`);
console.log(`Version: ${result.version}, Length: ${result.length}`);
console.log(`Digest: ${Buffer.from(result.digest).toString("hex")}`);
```

`iscc_decode` returns an `IsccDecodeResult` object with `maintype`, `subtype`, `version`, `length`
(length index), and `digest` (Buffer) fields.

### Decompose

Split a composite ISCC-CODE into its individual unit codes:

```javascript
const {
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_iscc_code_v0,
    iscc_decompose
} = require("@iscc/lib");

const data = Buffer.from("Hello World".repeat(1000));
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

- `encode_base64(data)` — encode a Buffer to base64 string
- `json_to_data_url(json)` — convert a JSON string to a `data:application/json;base64,...` URL
- `soft_hash_video_v0(frameSigs, bits?)` — compute a video similarity hash from MPEG-7 frame
    signatures, returns Buffer

## Constants

Exported constants used by the ISCC algorithms:

```javascript
const {
    META_TRIM_NAME,
    META_TRIM_DESCRIPTION,
    IO_READ_SIZE,
    TEXT_NGRAM_SIZE,
} = require("@iscc/lib");

META_TRIM_NAME; // 128 — max byte length for name normalization
META_TRIM_DESCRIPTION; // 4096 — max byte length for description normalization
IO_READ_SIZE; // 4_194_304 — default read buffer size (4 MB)
TEXT_NGRAM_SIZE; // 13 — n-gram size for text similarity hashing
```

## Conformance testing

Verify the library against official test vectors:

```javascript
import {
    conformance_selftest
} from "@iscc/lib";

console.log(conformance_selftest()); // true
```

## Error handling

Functions throw on invalid input:

```javascript
import {
    gen_text_code_v0
} from "@iscc/lib";

try {
    gen_text_code_v0("Hello", 13); // bits must be a multiple of 32
} catch (error) {
    console.error(`Invalid input: ${error.message}`);
}
```
