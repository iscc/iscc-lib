---
icon: lucide/house
description: High-performance polyglot implementation of ISO 24138 International Standard Content Code.
---

# iscc-lib

**High-performance polyglot implementation of
[ISO 24138:2024](https://www.iso.org/standard/77899.html) International Standard Content Code
(ISCC).**

!!! warning "Experimental"

    This library is in early development (v0.0.x). APIs may change without notice. Not recommended for
    production use yet.

---

## What is iscc-lib?

iscc-lib is a Rust implementation of the ISCC standard with bindings for multiple programming
languages. It provides all 9 `gen_*_v0` code generation functions specified by ISO 24138, fully
conformant with the official [iscc-core](https://github.com/iscc/iscc-core) Python reference
implementation.

The [ISCC](https://iscc.codes) is a content-derived identifier for digital media assets. It enables
decentralized, content-based identification without a central registry.

## Key Features

- **Complete ISO 24138 coverage** — all 9 `gen_*_v0` functions implemented
- **Full conformance** — passes all official test vectors from iscc-core
- **High performance** — pure Rust core delivers significant speedups over the Python reference
- **Multi-language** — use from Rust, Python, Node.js, WebAssembly, or C
- **Cross-platform** — runs on Linux, macOS, and Windows

## Supported Code Types

| Function               | Code Type     | Description                    |
| ---------------------- | ------------- | ------------------------------ |
| `gen_meta_code_v0`     | Meta-Code     | Content metadata similarity    |
| `gen_text_code_v0`     | Text-Code     | Text content similarity        |
| `gen_image_code_v0`    | Image-Code    | Image content similarity       |
| `gen_audio_code_v0`    | Audio-Code    | Audio content similarity       |
| `gen_video_code_v0`    | Video-Code    | Video content similarity       |
| `gen_mixed_code_v0`    | Mixed-Code    | Cross-type content similarity  |
| `gen_data_code_v0`     | Data-Code     | Raw binary data similarity     |
| `gen_instance_code_v0` | Instance-Code | Exact binary data identity     |
| `gen_iscc_code_v0`     | ISCC-CODE     | Composite code combining units |

## Quick Start

=== "Rust"

    ```bash
    cargo add iscc-lib
    ```

    ```rust
    use iscc_lib::gen_text_code_v0;

    let result = gen_text_code_v0("Hello World", 64)?;
    println!("{result}"); // JSON string
    ```

=== "Python"

    ```bash
    pip install iscc-lib
    ```

    ```python
    import json
    from iscc_lib import gen_text_code_v0

    result = json.loads(gen_text_code_v0("Hello World"))
    print(result["iscc"])
    ```

## Available Bindings

| Platform    | Package                                         | Install                     |
| ----------- | ----------------------------------------------- | --------------------------- |
| Rust        | [crates.io](https://crates.io/crates/iscc-lib)  | `cargo add iscc-lib`        |
| Python      | [PyPI](https://pypi.org/project/iscc-lib/)      | `pip install iscc-lib`      |
| Node.js     | [npm](https://www.npmjs.com/package/@iscc/lib)  | `npm install @iscc/lib`     |
| WebAssembly | [npm](https://www.npmjs.com/package/@iscc/wasm) | `npm install @iscc/wasm`    |
| C / C++     | Source                                          | Via C FFI header (`iscc.h`) |

## Links

- [ISO 24138:2024](https://www.iso.org/standard/77899.html) — the ISCC international standard
- [ISCC Foundation](https://iscc.io) — stewards of the ISCC standard
- [iscc-core](https://github.com/iscc/iscc-core) — Python reference implementation
- [Source Code](https://github.com/iscc/iscc-lib) — iscc-lib on GitHub
