# iscc-lib

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/iscc/iscc-lib)

[![Crate](https://img.shields.io/crates/v/iscc-lib.svg)](https://crates.io/crates/iscc-lib)
[![PyPI](https://img.shields.io/pypi/v/iscc-lib.svg)](https://pypi.org/project/iscc-lib/)
[![npm](https://img.shields.io/npm/v/@iscc/lib.svg)](https://www.npmjs.com/package/@iscc/lib)
[![Go Reference](https://pkg.go.dev/badge/github.com/iscc/iscc-lib/packages/go.svg)](https://pkg.go.dev/github.com/iscc/iscc-lib/packages/go)
[![Gem](https://img.shields.io/gem/v/iscc-lib.svg)](https://rubygems.org/gems/iscc-lib)
[![Maven Central](https://img.shields.io/maven-central/v/io.iscc/iscc-lib.svg)](https://central.sonatype.com/artifact/io.iscc/iscc-lib)
[![NuGet](https://img.shields.io/nuget/v/Iscc.Lib.svg)](https://www.nuget.org/packages/Iscc.Lib)
[![npm wasm](https://img.shields.io/npm/v/@iscc/wasm.svg)](https://www.npmjs.com/package/@iscc/wasm)

**High-performance polyglot implementation of
[ISO 24138:2024](https://www.iso.org/standard/77899.html) — International Standard Content Code
(ISCC)**

## Key Features

- **Similarity-Preserving**: Detect similar content even after modifications
- **Multi-Level Identification**: Identify content at metadata, semantic, perceptual, and data
    levels
- **Self-Describing**: Each component contains its own type and version information
- **ISO Standardized**: Implements the official ISO 24138:2024 specification
- **Polyglot**: Rust core with bindings for Python, Java, Go, Ruby, C#, C++, Swift, Kotlin, Node.js,
    WASM, and C FFI
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

`iscc-lib` is a high-performance polyglot implementation of the ISCC core algorithms
([ISO 24138](https://www.iso.org/standard/77899.html)). Built in Rust with language bindings for
Python, Java, Go, Ruby, C#, C++, Kotlin, Node.js, WebAssembly, and C, it serves developers across
multiple ecosystems who need fast, reliable content identification.

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

### <img src="https://cdn.simpleicons.org/rust/CE412B" width="20" height="20" alt="Rust"> Rust

```bash
cargo add iscc-lib
```

### <img src="https://cdn.simpleicons.org/python/3776AB" width="20" height="20" alt="Python"> Python

```bash
pip install iscc-lib
```

### <img src="https://cdn.simpleicons.org/nodedotjs/5FA04E" width="20" height="20" alt="Node.js"> Node.js

```bash
npm install @iscc/lib
```

### <img src="https://cdn.simpleicons.org/openjdk/ED8B00" width="20" height="20" alt="Java"> Java

```xml
<dependency>
  <groupId>io.iscc</groupId>
  <artifactId>iscc-lib</artifactId>
  <version>0.3.1</version>
</dependency>
```

The native library must be available on `java.library.path` at runtime.

### <img src="https://cdn.simpleicons.org/go/00ADD8" width="20" height="20" alt="Go"> Go

```bash
go get github.com/iscc/iscc-lib/packages/go
```

### <img src="https://cdn.simpleicons.org/ruby/CC342D" width="20" height="20" alt="Ruby"> Ruby

```bash
gem install iscc-lib
```

### <img src="https://cdn.simpleicons.org/dotnet/512BD4" width="20" height="20" alt="C# / .NET"> C# / .NET

```bash
dotnet add package Iscc.Lib
```

### <img src="https://cdn.simpleicons.org/cplusplus/00599C" width="20" height="20" alt="C / C++"> C / C++

Pre-built release tarballs are attached to each
[GitHub Release](https://github.com/iscc/iscc-lib/releases). Download for your platform — includes
shared library, static library, `iscc.h` header, and `iscc.hpp` C++ wrapper.

### <img src="https://cdn.simpleicons.org/swift/F05138" width="20" height="20" alt="Swift"> Swift

Add the package dependency to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.3.0"),
]
```

### <img src="https://cdn.simpleicons.org/kotlin/7F52FF" width="20" height="20" alt="Kotlin"> Kotlin

```kotlin
dependencies {
    implementation("io.iscc:iscc-lib-kotlin:0.3.1")
    implementation("net.java.dev.jna:jna:5.16.0")
}
```

The native library must be available on `java.library.path` and `jna.library.path` at runtime.

### <img src="https://cdn.simpleicons.org/webassembly/654FF0" width="20" height="20" alt="WASM"> WASM

```bash
npm install @iscc/wasm
```

## Quick Start

### <img src="https://cdn.simpleicons.org/rust/CE412B" width="20" height="20" alt="Rust"> Rust

```rust
use iscc_lib::gen_meta_code_v0;

let result = gen_meta_code_v0("ISCC Test Document!", None, None, 64).unwrap();
println!("Meta-Code: {}", result.iscc);
```

### <img src="https://cdn.simpleicons.org/python/3776AB" width="20" height="20" alt="Python"> Python

```python
import iscc_lib as ic

result = ic.gen_meta_code_v0("ISCC Test Document!")
print(f"Meta-Code: {result['iscc']}")
```

### <img src="https://cdn.simpleicons.org/nodedotjs/5FA04E" width="20" height="20" alt="Node.js"> Node.js

```javascript
const ic = require("@iscc/lib");

const result = ic.gen_meta_code_v0("ISCC Test Document!");
console.log(`Meta-Code: ${result.iscc}`);
```

### <img src="https://cdn.simpleicons.org/openjdk/ED8B00" width="20" height="20" alt="Java"> Java

```java
import io.iscc.iscc_lib.IsccLib;

String result = IsccLib.genMetaCodeV0("ISCC Test Document!", null, null, 64);
System.out.println("Meta-Code: " + result);
```

### <img src="https://cdn.simpleicons.org/go/00ADD8" width="20" height="20" alt="Go"> Go

```go
package main

import (
	"fmt"
	"log"

	iscc "github.com/iscc/iscc-lib/packages/go"
)

func main() {
	result, err := iscc.GenMetaCodeV0("ISCC Test Document!", nil, nil, 64)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Meta-Code:", result.Iscc)
}
```

### <img src="https://cdn.simpleicons.org/ruby/CC342D" width="20" height="20" alt="Ruby"> Ruby

```ruby
require "iscc_lib"

result = IsccLib.gen_meta_code_v0("ISCC Test Document!")
puts "Meta-Code: #{result.iscc}"
```

### <img src="https://cdn.simpleicons.org/dotnet/512BD4" width="20" height="20" alt="C# / .NET"> C# / .NET

```csharp
using Iscc.Lib;

var result = IsccLib.GenMetaCodeV0("ISCC Test Document!");
Console.WriteLine($"Meta-Code: {result.Iscc}");
```

### <img src="https://cdn.simpleicons.org/cplusplus/00599C" width="20" height="20" alt="C++"> C++

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

int main() {
    auto result = iscc::gen_meta_code_v0("ISCC Test Document!");
    std::cout << "Meta-Code: " << result.iscc << std::endl;
}
```

### <img src="https://cdn.simpleicons.org/swift/F05138" width="20" height="20" alt="Swift"> Swift

```swift
import IsccLib

let result = try genMetaCodeV0(name: "ISCC Test Document!",
                               description: nil, meta: nil, bits: 64)
print("Meta-Code: \(result.iscc)")
```

### <img src="https://cdn.simpleicons.org/kotlin/7F52FF" width="20" height="20" alt="Kotlin"> Kotlin

```kotlin
import uniffi.iscc_uniffi.*

val result = genMetaCodeV0(
    name = "ISCC Test Document!",
    description = null,
    meta = null,
    bits = 64u
)
println("Meta-Code: ${result.iscc}")
```

### <img src="https://cdn.simpleicons.org/webassembly/654FF0" width="20" height="20" alt="WASM"> WASM

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
gen_sum_code_v0
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
