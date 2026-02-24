---
icon: lucide/terminal
description: Guide to using iscc-lib from Python — code generation, streaming, and text utilities.
---

# Python

A guide to using iscc-lib from Python. Covers installation, code generation, structured results,
streaming, and text utilities.

---

## Installation

```bash
pip install iscc-lib
```

iscc-lib ships as a pre-built wheel with a compiled Rust extension. No Rust toolchain is required
for installation.

## Code generation

All 9 `gen_*_v0` functions follow the same pattern: pass content-specific input and an optional
`bits` parameter (default 64), and receive a structured result with an `.iscc` field containing the
ISCC code string.

### Meta-Code

Generate a Meta-Code from content metadata (title, description, structured metadata):

```python
from iscc_lib import gen_meta_code_v0

result = gen_meta_code_v0(
    name="Die Unendliche Geschichte",
    description="Von Michael Ende",
)
print(result.iscc)  # "ISCC:AAA..."
print(result.name)  # Normalized name
print(result.metahash)  # BLAKE3 hash of metadata
```

The `meta` parameter accepts a JSON string or a `data:` URL with base64-encoded payload. When
provided, `meta` takes precedence over `description` for the similarity digest:

```python
import json

metadata = json.dumps({"title": "Example", "author": "Author"})
result = gen_meta_code_v0("Example Title", meta=metadata)
print(result.meta)  # data: URL with base64-encoded JSON
```

### Text-Code

Generate a Text-Code from plain text content:

```python
from iscc_lib import gen_text_code_v0

result = gen_text_code_v0("Hello World")
print(result.iscc)  # "ISCC:EAA..."
print(result.characters)  # Number of characters processed
```

### Image-Code

Generate an Image-Code from a 32x32 grayscale thumbnail (1024 bytes):

```python
from iscc_lib import gen_image_code_v0

# Pre-process your image to 32x32 grayscale (e.g., with Pillow)
pixels = bytes([128] * 1024)  # Placeholder: uniform gray
result = gen_image_code_v0(pixels)
print(result.iscc)  # "ISCC:EEA..."
```

### Audio-Code

Generate an Audio-Code from a Chromaprint fingerprint vector (signed integers):

```python
from iscc_lib import gen_audio_code_v0

# Obtain Chromaprint features externally (e.g., with pyacoustid)
fingerprint = [123456, -789012, 345678, 901234]
result = gen_audio_code_v0(fingerprint)
print(result.iscc)  # "ISCC:EIA..."
```

### Video-Code

Generate a Video-Code from MPEG-7 frame signature vectors:

```python
from iscc_lib import gen_video_code_v0

# Each frame signature is a list of 380 integers
frame_sigs = [[0] * 380, [1] * 380]
result = gen_video_code_v0(frame_sigs)
print(result.iscc)  # "ISCC:EMA..."
```

### Mixed-Code

Combine multiple Content-Codes of different types into a Mixed-Code:

```python
from iscc_lib import gen_text_code_v0, gen_image_code_v0, gen_mixed_code_v0

text_result = gen_text_code_v0("Hello World")
image_result = gen_image_code_v0(bytes([128] * 1024))

result = gen_mixed_code_v0([text_result.iscc, image_result.iscc])
print(result.iscc)  # "ISCC:EQA..."
print(result.parts)  # List of input code strings
```

### Data-Code

Generate a Data-Code from raw bytes using content-defined chunking and MinHash:

```python
from iscc_lib import gen_data_code_v0

data = b"Hello World" * 1000
result = gen_data_code_v0(data)
print(result.iscc)  # "ISCC:GAA..."
```

Data-Code also accepts file-like objects:

```python
with open("document.pdf", "rb") as f:
    result = gen_data_code_v0(f)
print(result.iscc)
```

### Instance-Code

Generate an Instance-Code from raw bytes using BLAKE3 hashing:

```python
from iscc_lib import gen_instance_code_v0

data = b"Hello World"
result = gen_instance_code_v0(data)
print(result.iscc)  # "ISCC:IAA..."
print(result.datahash)  # Multihash of the data
print(result.filesize)  # Size in bytes
```

Instance-Code also accepts file-like objects:

```python
with open("document.pdf", "rb") as f:
    result = gen_instance_code_v0(f)
print(result.datahash)
```

### ISCC-CODE

Combine individual ISCC unit codes into a composite ISCC-CODE:

```python
from iscc_lib import gen_data_code_v0, gen_instance_code_v0, gen_iscc_code_v0

data = b"Hello World" * 1000
data_result = gen_data_code_v0(data)
instance_result = gen_instance_code_v0(data)

result = gen_iscc_code_v0([data_result.iscc, instance_result.iscc])
print(result.iscc)  # "ISCC:KAA..."
```

## Structured results

Every `gen_*_v0` function returns a typed result object that supports both dict-style and
attribute-style access:

```python
from iscc_lib import gen_meta_code_v0

result = gen_meta_code_v0("Example Title", description="Example description")

# Attribute access
print(result.iscc)
print(result.name)
print(result.metahash)

# Dict access
print(result["iscc"])
print(result["name"])

# Iterate over keys
for key, value in result.items():
    print(f"{key}: {value}")

# JSON serialization
import json

print(json.dumps(result, indent=2))
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

Fields marked with `?` are optional and only present when the corresponding input was provided.

## Streaming

For large files, use `DataHasher` and `InstanceHasher` to process data incrementally without loading
everything into memory. Both follow the `new() -> update() -> finalize()` pattern.

### DataHasher

```python
from iscc_lib import DataHasher

hasher = DataHasher()

with open("large_file.bin", "rb") as f:
    while chunk := f.read(65536):
        hasher.update(chunk)

result = hasher.finalize()
print(result.iscc)  # Identical to gen_data_code_v0(entire_file)
```

You can also pass initial data or a file-like object to the constructor:

```python
from iscc_lib import DataHasher

# From bytes
hasher = DataHasher(b"initial data")
hasher.update(b"more data")
result = hasher.finalize()

# From file
with open("file.bin", "rb") as f:
    hasher = DataHasher(f)
    result = hasher.finalize()
```

### InstanceHasher

```python
from iscc_lib import InstanceHasher

hasher = InstanceHasher()

with open("large_file.bin", "rb") as f:
    while chunk := f.read(65536):
        hasher.update(chunk)

result = hasher.finalize()
print(result.iscc)  # Identical to gen_instance_code_v0(entire_file)
print(result.datahash)  # Multihash of the complete data
print(result.filesize)  # Total bytes processed
```

Both hashers accept `bytes` or file-like objects in `update()`. After calling `finalize()`, the
hasher is consumed and further calls to `update()` or `finalize()` raise an error.

## Text utilities

iscc-lib provides text normalization functions used internally by the code generation pipeline.
These are available for preprocessing your own text inputs.

### text_clean

Normalize text for display: applies NFKC normalization, removes control characters (except
newlines), normalizes line endings, collapses consecutive empty lines, and strips leading/trailing
whitespace.

```python
from iscc_lib import text_clean

cleaned = text_clean("  Hello\r\n\r\n\r\nWorld  ")
print(repr(cleaned))  # 'Hello\n\nWorld'
```

### text_collapse

Simplify text for similarity hashing: lowercases, strips whitespace, punctuation, and diacritics.
Used internally by `gen_text_code_v0`.

```python
from iscc_lib import text_collapse

collapsed = text_collapse("Hello, World!")
print(collapsed)  # 'helloworld'
```

### text_remove_newlines

Remove newlines and collapse whitespace to single spaces:

```python
from iscc_lib import text_remove_newlines

single_line = text_remove_newlines("Hello\nWorld\nFoo")
print(single_line)  # 'Hello World Foo'
```

### text_trim

Trim text so its UTF-8 byte size does not exceed a limit. Multi-byte characters that would be split
are dropped entirely:

```python
from iscc_lib import text_trim

trimmed = text_trim("Hello World", 5)
print(trimmed)  # 'Hello'
```

## Conformance testing

Verify that the library produces correct results for all official test vectors:

```python
from iscc_lib import conformance_selftest

assert conformance_selftest() is True
```

## Error handling

All `gen_*_v0` functions raise `ValueError` on invalid input:

```python
from iscc_lib import gen_text_code_v0

try:
    gen_text_code_v0("Hello", bits=13)  # bits must be a multiple of 32
except ValueError as e:
    print(f"Invalid input: {e}")
```
