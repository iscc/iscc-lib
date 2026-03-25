---
icon: lucide/house
description: High-performance polyglot implementation of ISO 24138 International Standard Content Code.
---

# iscc-lib

**High-performance polyglot implementation of
[ISO 24138:2024](https://www.iso.org/standard/77899.html) International Standard Content Code
(ISCC).**

---

## What is iscc-lib?

iscc-lib is a Rust implementation of the ISCC standard with bindings for multiple programming
languages. It provides all 10 `gen_*_v0` code generation functions specified by ISO 24138, fully
conformant with the official [iscc-core](https://github.com/iscc/iscc-core) Python reference
implementation.

The [ISCC](https://iscc.codes) is a content-derived identifier for digital media assets. It enables
decentralized, content-based identification without a central registry.

## Key Features

- **Complete ISO 24138 coverage** — all 10 `gen_*_v0` functions implemented
- **Full conformance** — passes all official test vectors from iscc-core
- **High performance** — pure Rust core delivers significant speedups over the Python reference
- **Multi-language** — use from Rust, Python, Java, Go, Ruby, C#, C++, Swift, Kotlin, Node.js,
    WebAssembly, or C
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
| `gen_sum_code_v0`      | ISCC-SUM      | Single-pass file processing    |

## Quick Start

=== "Python"

    ```bash
    pip install iscc-lib
    ```

    ```python
    from iscc_lib import gen_text_code_v0

    result = gen_text_code_v0("Hello World")
    print(result["iscc"])
    ```

=== "Rust"

    ```bash
    cargo add iscc-lib
    ```

    ```rust
    use iscc_lib::gen_text_code_v0;

    let result = gen_text_code_v0("Hello World", 64)?;
    println!("{}", result.iscc);
    ```

=== "Ruby"

    ```bash
    gem install iscc-lib
    ```

    ```ruby
    require "iscc_lib"

    result = IsccLib.gen_text_code_v0("Hello World")
    puts result.iscc # "ISCC:EAA..."
    ```

=== "Node.js"

    ```bash
    npm install @iscc/lib
    ```

    ```javascript
    import {
        gen_text_code_v0
    } from "@iscc/lib";

    const iscc = gen_text_code_v0("Hello World");
    console.log(iscc); // "ISCC:EAA..."
    ```

=== "WASM"

    ```bash
    npm install @iscc/wasm
    ```

    ```javascript
    import init, {
        gen_text_code_v0
    } from "@iscc/wasm";

    await init();
    const iscc = gen_text_code_v0("Hello World");
    console.log(iscc); // "ISCC:EAA..."
    ```

=== "Go"

    ```bash
    go get github.com/iscc/iscc-lib/packages/go
    ```

    ```go
    import iscc "github.com/iscc/iscc-lib/packages/go"

    result, _ := iscc.GenTextCodeV0("Hello World", 64)
    fmt.Println(result.Iscc) // "ISCC:EAA..."
    ```

=== "Java"

    ```xml
    <dependency>
      <groupId>io.iscc</groupId>
      <artifactId>iscc-lib</artifactId>
      <version>0.4.0</version>
    </dependency>
    ```

    ```java
    import io.iscc.iscc_lib.IsccLib;

    String iscc = IsccLib.genTextCodeV0("Hello World", 64);
    System.out.println(iscc); // "ISCC:EAA..."
    ```

=== "C#"

    ```bash
    dotnet add package Iscc.Lib
    ```

    ```csharp
    using Iscc.Lib;

    var result = IsccLib.GenTextCodeV0("Hello World");
    Console.WriteLine(result.Iscc); // "ISCC:EAA..."
    ```

=== "C++"

    Download pre-built libraries from [GitHub Releases](https://github.com/iscc/iscc-lib/releases).

    ```cpp
    #include <iscc/iscc.hpp>

    auto result = iscc::gen_text_code_v0("Hello World");
    std::cout << result.iscc << std::endl; // "ISCC:EAA..."
    ```

=== "Swift"

    ```swift
    // Package.swift dependency
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.4.0")
    ```

    ```swift
    import IsccLib

    let result = try genTextCodeV0(text: "Hello World", bits: 64)
    print(result.iscc) // "ISCC:EAA..."
    ```

=== "Kotlin"

    ```kotlin
    // build.gradle.kts
    implementation("io.iscc:iscc-lib-kotlin:0.4.0")
    ```

    ```kotlin
    import uniffi.iscc_uniffi.*

    val result = genTextCodeV0(text = "Hello World", bits = 64u)
    println(result.iscc) // "ISCC:EAA..."
    ```

## Available Bindings

| Platform    | Package                                                                        | Install                                         |
| ----------- | ------------------------------------------------------------------------------ | ----------------------------------------------- |
| Rust        | [crates.io](https://crates.io/crates/iscc-lib)                                 | `cargo add iscc-lib`                            |
| Python      | [PyPI](https://pypi.org/project/iscc-lib/)                                     | `pip install iscc-lib`                          |
| Node.js     | [npm](https://www.npmjs.com/package/@iscc/lib)                                 | `npm install @iscc/lib`                         |
| Java        | [Maven Central](https://central.sonatype.com/artifact/io.iscc/iscc-lib)        | See Quick Start above                           |
| Go          | [Go module](https://pkg.go.dev/github.com/iscc/iscc-lib/packages/go)           | `go get github.com/iscc/iscc-lib/packages/go`   |
| Ruby        | [RubyGems](https://rubygems.org/gems/iscc-lib)                                 | `gem install iscc-lib`                          |
| C# / .NET   | [NuGet](https://www.nuget.org/packages/Iscc.Lib)                               | `dotnet add package Iscc.Lib`                   |
| C / C++     | [GitHub Releases](https://github.com/iscc/iscc-lib/releases)                   | Pre-built tarballs per platform                 |
| Swift       | [SPM](https://github.com/iscc/iscc-lib)                                        | `.package(url: "...", from: "0.4.0")`           |
| Kotlin      | [Maven Central](https://central.sonatype.com/artifact/io.iscc/iscc-lib-kotlin) | `implementation("io.iscc:iscc-lib-kotlin:...")` |
| WebAssembly | [npm](https://www.npmjs.com/package/@iscc/wasm)                                | `npm install @iscc/wasm`                        |

## Links

- [ISO 24138:2024](https://www.iso.org/standard/77899.html) — the ISCC international standard
- [ISCC Foundation](https://iscc.io) — stewards of the ISCC standard
- [iscc-core](https://github.com/iscc/iscc-core) — Python reference implementation
- [Source Code](https://github.com/iscc/iscc-lib) — iscc-lib on GitHub
