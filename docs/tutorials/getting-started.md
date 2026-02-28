---
icon: lucide/rocket
description: Install iscc-lib, generate your first ISCC code, and understand the result.
---

# Getting Started

This tutorial walks you through installing iscc-lib, generating your first ISCC code, and
understanding the result. By the end, you will know how to create content identifiers for metadata,
text, and binary data.

## Prerequisites

You need a working development environment for your language of choice. iscc-lib supports Python
3.10+, Rust, Node.js, Java 11+, Go 1.21+, and WebAssembly.

## Install

=== "Python"

    ```bash
    pip install iscc-lib
    ```

=== "Rust"

    ```bash
    cargo add iscc-lib
    ```

=== "Node.js"

    ```bash
    npm install @iscc/lib
    ```

=== "Java"

    ```bash
    cargo build -p iscc-jni --release
    ```

=== "Go"

    ```bash
    go get github.com/iscc/iscc-lib/packages/go
    ```

=== "WASM"

    ```bash
    npm install @iscc/wasm
    ```

Verify the installation by running the built-in conformance self-test:

=== "Python"

    ```python
    from iscc_lib import conformance_selftest

    print(conformance_selftest())  # True
    ```

=== "Rust"

    ```rust
    use iscc_lib::conformance_selftest;

    assert!(conformance_selftest());
    ```

=== "Node.js"

    ```javascript
    import {
        conformance_selftest
    } from "@iscc/lib";

    console.log(conformance_selftest()); // true
    ```

=== "Java"

    ```java
    import io.iscc.iscc_lib.IsccLib;

    boolean ok = IsccLib.conformanceSelftest();
    System.out.println(ok); // true
    ```

=== "Go"

    ```go
    ok, _ := iscc.ConformanceSelftest()
    fmt.Println(ok) // true
    ```

=== "WASM"

    ```javascript
    import init, {
        conformance_selftest
    } from "@iscc/wasm";

    await init();
    console.log(conformance_selftest()); // true
    ```

If this prints `true`, the library is installed correctly and passes all official ISO 24138 test
vectors.

## Generate your first ISCC code

The simplest way to create an ISCC is from content metadata. Use `gen_meta_code_v0` with a title and
optional description:

=== "Python"

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

=== "Rust"

    ```rust
    use iscc_lib::gen_meta_code_v0;

    let result = gen_meta_code_v0(
        "The Neverending Story",
        Some("A novel by Michael Ende"),
        None, 64,
    )?;
    println!("{}", result.iscc);      // "ISCC:AAA..."
    println!("{}", result.name);      // Normalized name
    println!("{}", result.metahash);  // BLAKE3 hash
    ```

=== "Node.js"

    ```javascript
    import {
        gen_meta_code_v0
    } from "@iscc/lib";

    const iscc = gen_meta_code_v0(
        "The Neverending Story",
        "A novel by Michael Ende",
    );
    console.log(iscc); // "ISCC:AAA..."
    ```

=== "Java"

    ```java
    import io.iscc.iscc_lib.IsccLib;

    String iscc = IsccLib.genMetaCodeV0(
        "The Neverending Story", "A novel by Michael Ende", null, 64
    );
    System.out.println(iscc); // "ISCC:AAA..."
    ```

=== "Go"

    ```go
    desc := "A novel by Michael Ende"
    result, _ := iscc.GenMetaCodeV0(
        "The Neverending Story", &desc, nil, 64,
    )
    fmt.Println(result.Iscc) // "ISCC:AAA..."
    ```

=== "WASM"

    ```javascript
    import {
        gen_meta_code_v0
    } from "@iscc/wasm";

    const iscc = gen_meta_code_v0(
        "The Neverending Story",
        "A novel by Michael Ende",
    );
    console.log(iscc); // "ISCC:AAA..."
    ```

!!! tip "JSON serialization (Python)"

    The Python result object supports both attribute access (`result.iscc`) and dict-style access
    (`result["iscc"]`). It is also JSON-serializable:

    ```python
    import json

    print(json.dumps(result, indent=2))
    ```

## Understand the ISCC structure

Every ISCC code encodes its type, subtype, version, and length in a self-describing header. Use
`iscc_decompose` to inspect the components of a composite code:

=== "Python"

    ```python
    from iscc_lib import gen_meta_code_v0, iscc_decompose

    result = gen_meta_code_v0("The Neverending Story")
    units = iscc_decompose(result.iscc)
    print(units)  # List of individual ISCC-UNIT strings
    ```

=== "Rust"

    ```rust
    use iscc_lib::{gen_meta_code_v0, iscc_decompose};

    let result = gen_meta_code_v0("The Neverending Story", None, None, 64)?;
    let units = iscc_decompose(&result.iscc)?;
    for unit in &units {
        println!("{unit}");
    }
    ```

=== "Node.js"

    ```javascript
    import {
        gen_meta_code_v0,
        iscc_decompose
    } from "@iscc/lib";

    const iscc = gen_meta_code_v0("The Neverending Story");
    const units = iscc_decompose(iscc);
    console.log(units);
    ```

=== "Java"

    ```java
    String iscc = IsccLib.genMetaCodeV0("The Neverending Story", null, null, 64);
    String[] units = IsccLib.isccDecompose(iscc);
    for (String unit : units) {
        System.out.println(unit);
    }
    ```

=== "Go"

    ```go
    result, _ := iscc.GenMetaCodeV0("The Neverending Story", nil, nil, 64)
    units, _ := iscc.IsccDecompose(result.Iscc)
    for _, unit := range units {
        fmt.Println(unit)
    }
    ```

=== "WASM"

    ```javascript
    import {
        gen_meta_code_v0,
        iscc_decompose
    } from "@iscc/wasm";

    const iscc = gen_meta_code_v0("The Neverending Story");
    const units = iscc_decompose(iscc);
    console.log(units);
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

=== "Python"

    ```python
    from iscc_lib import gen_text_code_v0

    result = gen_text_code_v0("Hello World")
    print(result.iscc)  # "ISCC:EAA..."
    print(result.characters)  # Number of characters processed
    ```

=== "Rust"

    ```rust
    use iscc_lib::gen_text_code_v0;

    let result = gen_text_code_v0("Hello World", 64)?;
    println!("{}", result.iscc);        // "ISCC:EAA..."
    println!("{}", result.characters);  // Characters processed
    ```

=== "Node.js"

    ```javascript
    import {
        gen_text_code_v0
    } from "@iscc/lib";

    const iscc = gen_text_code_v0("Hello World");
    console.log(iscc); // "ISCC:EAA..."
    ```

=== "Java"

    ```java
    String iscc = IsccLib.genTextCodeV0("Hello World", 64);
    System.out.println(iscc); // "ISCC:EAA..."
    ```

=== "Go"

    ```go
    result, _ := iscc.GenTextCodeV0("Hello World", 64)
    fmt.Println(result.Iscc) // "ISCC:EAA..."
    ```

=== "WASM"

    ```javascript
    import {
        gen_text_code_v0
    } from "@iscc/wasm";

    const iscc = gen_text_code_v0("Hello World");
    console.log(iscc); // "ISCC:EAA..."
    ```

Text-Codes capture the semantic fingerprint of text. Similar texts produce similar codes, enabling
fuzzy matching.

### Instance-Code

Generate an exact identity hash from raw bytes:

=== "Python"

    ```python
    from iscc_lib import gen_instance_code_v0

    result = gen_instance_code_v0(b"Hello World")
    print(result.iscc)  # "ISCC:IAA..."
    print(result.datahash)  # Multihash of the data
    print(result.filesize)  # Size in bytes
    ```

=== "Rust"

    ```rust
    use iscc_lib::gen_instance_code_v0;

    let result = gen_instance_code_v0(b"Hello World", 64)?;
    println!("{}", result.iscc);      // "ISCC:IAA..."
    println!("{}", result.datahash);  // Multihash
    println!("{}", result.filesize);  // Size in bytes
    ```

=== "Node.js"

    ```javascript
    import {
        gen_instance_code_v0
    } from "@iscc/lib";

    const data = Buffer.from("Hello World");
    const iscc = gen_instance_code_v0(data);
    console.log(iscc); // "ISCC:IAA..."
    ```

=== "Java"

    ```java
    byte[] data = "Hello World".getBytes();
    String iscc = IsccLib.genInstanceCodeV0(data, 64);
    System.out.println(iscc); // "ISCC:IAA..."
    ```

=== "Go"

    ```go
    data := []byte("Hello World")
    result, _ := iscc.GenInstanceCodeV0(data, 64)
    fmt.Println(result.Iscc) // "ISCC:IAA..."
    ```

=== "WASM"

    ```javascript
    import {
        gen_instance_code_v0
    } from "@iscc/wasm";

    const data = new TextEncoder().encode("Hello World");
    const iscc = gen_instance_code_v0(data);
    console.log(iscc); // "ISCC:IAA..."
    ```

For large files, use `InstanceHasher` to process data in chunks without loading everything into
memory:

=== "Python"

    ```python
    from iscc_lib import InstanceHasher

    hasher = InstanceHasher()
    with open("large_file.bin", "rb") as f:
        while chunk := f.read(65536):
            hasher.update(chunk)
    result = hasher.finalize()
    print(result.iscc)
    ```

=== "Rust"

    ```rust
    use iscc_lib::InstanceHasher;
    use std::io::Read;

    let mut hasher = InstanceHasher::new();
    let mut file = std::fs::File::open("large_file.bin")?;
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    let result = hasher.finalize(64)?;
    ```

=== "Node.js"

    ```javascript
    import {
        createReadStream
    } from "node:fs";
    import {
        InstanceHasher
    } from "@iscc/lib";

    const hasher = new InstanceHasher();
    for await (const chunk of createReadStream("large_file.bin")) {
        hasher.update(chunk);
    }
    console.log(hasher.finalize());
    ```

=== "Java"

    ```java
    long ptr = IsccLib.instanceHasherNew();
    try {
        var fis = new java.io.FileInputStream("large_file.bin");
        byte[] buf = new byte[65536];
        int n;
        while ((n = fis.read(buf)) != -1) {
            IsccLib.instanceHasherUpdate(ptr, java.util.Arrays.copyOf(buf, n));
        }
        System.out.println(IsccLib.instanceHasherFinalize(ptr, 64));
    } finally {
        IsccLib.instanceHasherFree(ptr);
    }
    ```

=== "Go"

    ```go
    hasher := iscc.NewInstanceHasher()
    f, _ := os.Open("large_file.bin")
    defer f.Close()
    buf := make([]byte, 65536)
    for {
        n, err := f.Read(buf)
        if n > 0 { hasher.Push(buf[:n]) }
        if err != nil { break }
    }
    result, _ := hasher.Finalize(64)
    fmt.Println(result.Iscc)
    ```

=== "WASM"

    ```javascript
    import {
        InstanceHasher
    } from "@iscc/wasm";

    const hasher = new InstanceHasher();
    // Feed Uint8Array chunks from a ReadableStream or File
    hasher.update(new Uint8Array([72, 101, 108, 108, 111]));
    hasher.update(new Uint8Array([32, 87, 111, 114, 108, 100]));
    console.log(hasher.finalize());
    ```

## Next steps

Now that you have generated your first ISCC codes, explore further:

- **[Python how-to guide](../howto/python.md)** — all 9 code types, structured results, streaming,
    and text utilities
- **[Rust how-to guide](../howto/rust.md)** — Rust-level API with typed results and ownership
- **[Node.js how-to guide](../howto/nodejs.md)** — use iscc-lib from JavaScript
- **[Java how-to guide](../howto/java.md)** — JNI bindings for Java applications
- **[Go how-to guide](../howto/go.md)** — pure Go implementation
- **[WebAssembly how-to guide](../howto/wasm.md)** — run ISCC in the browser
- **[Architecture](../architecture.md)** — understand the hub-and-spoke crate model and internal
    design
- **[Python API reference](../api.md)** — complete function signatures and docstrings
- **[Rust API reference](../rust-api.md)** — Rust-level API documentation
