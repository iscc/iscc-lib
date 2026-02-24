---
icon: lucide/rocket
description: Install iscc-lib, generate your first ISCC code, and understand the result.
---

# Getting Started

This tutorial walks you through installing iscc-lib, generating your first ISCC code, and
understanding the result. By the end, you will know how to create content identifiers for metadata,
text, and binary data.

## Prerequisites

- **Python 3.10+** with pip or [uv](https://docs.astral.sh/uv/)

!!! tip "Other languages"

    iscc-lib also has bindings for [Node.js](../howto/nodejs.md), [WebAssembly](../howto/wasm.md), and
    Rust. This tutorial uses Python for simplicity — the concepts apply to all languages.

## Install

Install iscc-lib from PyPI:

```bash
pip install iscc-lib
```

Or with uv:

```bash
uv add iscc-lib
```

Verify the installation by running the built-in conformance self-test:

```python
from iscc_lib import conformance_selftest

print(conformance_selftest())  # True
```

If this prints `True`, the library is installed correctly and passes all official ISO 24138 test
vectors.

## Generate your first ISCC code

The simplest way to create an ISCC is from content metadata. Use `gen_meta_code_v0` with a title and
optional description:

```python
from iscc_lib import gen_meta_code_v0

result = gen_meta_code_v0(
    name="The Neverending Story",
    description="A novel by Michael Ende",
)

print(result.iscc)  # ISCC code string, e.g. "ISCC:AAA..."
print(result.name)  # Normalized name
print(result.metahash)  # BLAKE3 hash of the metadata
```

The result object supports both attribute access (`result.iscc`) and dict-style access
(`result["iscc"]`). It is also JSON-serializable:

```python
import json

print(json.dumps(result, indent=2))
```

## Understand the ISCC structure

Every ISCC code encodes its type, subtype, version, and length in a self-describing header. Use
`iscc_decompose` to inspect the components of a composite code:

```python
from iscc_lib import gen_meta_code_v0, iscc_decompose

result = gen_meta_code_v0("The Neverending Story")
print(result.iscc)  # e.g. "ISCC:AAAWN3GYR2PBKSE4"

units = iscc_decompose(result.iscc)
print(units)  # List of individual ISCC-UNIT strings
```

Each ISCC-UNIT header encodes:

- **MainType** — the kind of code (META, TEXT, IMAGE, AUDIO, VIDEO, MIXED, DATA, INSTANCE, ISCC)
- **SubType** — content type qualifier (e.g., TEXT, IMAGE, NONE)
- **Version** — algorithm version (currently V0 for all types)
- **Length** — bit length of the body (default 64 bits)

## Try other code types

ISCC supports different code types for different content. Here are two more examples.

### Text-Code

Generate a similarity hash from text content:

```python
from iscc_lib import gen_text_code_v0

result = gen_text_code_v0("Hello World")
print(result.iscc)  # "ISCC:EAA..."
print(result.characters)  # Number of characters processed
```

Text-Codes capture the semantic fingerprint of text. Similar texts produce similar codes, enabling
fuzzy matching.

### Instance-Code

Generate an exact identity hash from raw bytes:

```python
from iscc_lib import gen_instance_code_v0

result = gen_instance_code_v0(b"Hello World")
print(result.iscc)  # "ISCC:IAA..."
print(result.datahash)  # Multihash of the data
print(result.filesize)  # Size in bytes
```

For large files, use `DataHasher` or `InstanceHasher` to process data in chunks without loading
everything into memory:

```python
from iscc_lib import InstanceHasher

hasher = InstanceHasher()

with open("large_file.bin", "rb") as f:
    while chunk := f.read(65536):
        hasher.update(chunk)

result = hasher.finalize()
print(result.iscc)  # Identical to gen_instance_code_v0(entire_file)
```

## Next steps

Now that you have generated your first ISCC codes, explore further:

- **[Python how-to guide](../howto/python.md)** — all 9 code types, structured results, streaming,
    and text utilities
- **[Node.js how-to guide](../howto/nodejs.md)** — use iscc-lib from JavaScript
- **[WebAssembly how-to guide](../howto/wasm.md)** — run ISCC in the browser
- **[Architecture](../architecture.md)** — understand the hub-and-spoke crate model and internal
    design
- **[Python API reference](../api.md)** — complete function signatures and docstrings
- **[Rust API reference](../rust-api.md)** — Rust-level API documentation
