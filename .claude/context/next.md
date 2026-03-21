# Next Work Package

## Step: Add missing language tabs to docs landing page

## Goal

Add Ruby, C#, C++, Swift, and Kotlin to the `docs/index.md` Quick Start tabbed code examples and
Available Bindings table. The target requires all 11 languages in tabbed code examples, but the
landing page currently only shows 6 (Python, Rust, Node.js, Java, Go, WASM).

## Scope

- **Create**: (none)
- **Modify**:
    - `docs/index.md` — add 5 language tabs to Quick Start section; add Swift and Kotlin rows to
        Available Bindings table
- **Reference**:
    - `docs/howto/ruby.md` — Ruby API patterns (`IsccLib.gen_text_code_v0`)
    - `docs/howto/dotnet.md` — C# API patterns (`IsccLib.GenTextCodeV0`)
    - `docs/howto/c-cpp.md` — C++ API patterns (`iscc::gen_text_code_v0`)
    - `docs/howto/swift.md` — Swift API patterns (`genTextCodeV0(text:bits:)`)
    - `docs/howto/kotlin.md` — Kotlin API patterns (`genTextCodeV0(text=, bits=)`)

## Not In Scope

- Updating `docs/tutorials/getting-started.md` tabs — that has 7 tab groups with complex examples
    (streaming, data code, etc.) and should be a separate follow-up step
- Adding language logos to docs headers — that's a `low` issue reserved for human direction
- Fixing the GITHUB_REF_NAME bug — blocked on human review
- Modifying `scripts/gen_llms_full.py` or `zensical.toml` — no navigation changes needed
- Adding new howto guide pages — all 11 already exist

## Implementation Notes

### Quick Start tabs

The existing Quick Start section has 6 tabs showing `gen_text_code_v0("Hello World")`. Add 5 new
tabs after the "WASM" tab, in this order: Ruby, C# / .NET, C / C++, Swift, Kotlin.

Each new tab needs:

1. An install command
2. A minimal code example calling `gen_text_code_v0`

**Ruby tab:**

````markdown
=== "Ruby"

    ```bash
    gem install iscc-lib
    ```

    ```ruby
    require "iscc_lib"

    result = IsccLib.gen_text_code_v0("Hello World")
    puts result.iscc # "ISCC:EAA..."
    ```
````

**C# tab:**

````markdown
=== "C#"

    ```bash
    dotnet add package Iscc.Lib
    ```

    ```csharp
    using Iscc.Lib;

    var result = IsccLib.GenTextCodeV0("Hello World");
    Console.WriteLine(result.Iscc); // "ISCC:EAA..."
    ```
````

**C++ tab:**

The C++ wrapper uses `iscc::gen_text_code_v0`. Install is via pre-built tarballs from GitHub
Releases. Include `#include <iscc/iscc.hpp>`:

````markdown
=== "C++"

    Download pre-built libraries from
    [GitHub Releases](https://github.com/iscc/iscc-lib/releases).

    ```cpp
    #include <iscc/iscc.hpp>

    auto result = iscc::gen_text_code_v0("Hello World");
    std::cout << result.iscc << std::endl; // "ISCC:EAA..."
    ```
````

**Swift tab:**

Swift uses UniFFI-generated free functions with named parameters:

````markdown
=== "Swift"

    ```swift
    // Package.swift dependency
    .package(url: "https://github.com/iscc/iscc-lib", from: "0.3.1")
    ```

    ```swift
    import IsccLib

    let result = try genTextCodeV0(text: "Hello World", bits: 64)
    print(result.iscc) // "ISCC:EAA..."
    ```
````

**Kotlin tab:**

Kotlin uses UniFFI-generated free functions with `UInt` parameters:

````markdown
=== "Kotlin"

    ```kotlin
    // build.gradle.kts
    implementation("io.iscc:iscc-lib-kotlin:0.3.1")
    ```

    ```kotlin
    import uniffi.iscc_uniffi.*

    val result = genTextCodeV0(text = "Hello World", bits = 64u)
    println(result.iscc) // "ISCC:EAA..."
    ```
````

### Available Bindings table

Add two rows at the end of the table (before the WebAssembly row or at the logical position):

| Swift | [SPM](https://github.com/iscc/iscc-lib) | `.package(url: "...", from: "0.3.1")` | | Kotlin
| [Maven Central](https://central.sonatype.com/artifact/io.iscc/iscc-lib-kotlin) |
`implementation("io.iscc:iscc-lib-kotlin:...")` |

Place Swift and Kotlin rows logically — after C / C++ and before WebAssembly, or at the end.

### Tab ordering

Follow the order used in the target spec: Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++,
Swift, Kotlin. OR keep existing order and append the 5 new tabs. Either is acceptable — consistency
with target spec is preferred.

## Verification

- `grep -c '=== "' docs/index.md` returns 11 (was 6 — one tab per language)
- `grep '=== "Ruby"' docs/index.md` returns a match
- `grep '=== "C#"' docs/index.md` returns a match
- `grep '=== "C++"' docs/index.md` returns a match
- `grep '=== "Swift"' docs/index.md` returns a match
- `grep '=== "Kotlin"' docs/index.md` returns a match
- `grep -c "Swift" docs/index.md` returns at least 2 (tab + Available Bindings table)
- `grep -c "Kotlin" docs/index.md` returns at least 2 (tab + Available Bindings table)
- `uv run zensical build` exits 0 (docs site builds successfully)
- `mise run format` produces no changes (formatting is clean)

## Done When

All 10 verification criteria pass — `docs/index.md` Quick Start has 11 language tabs and the
Available Bindings table includes all 11 languages including Swift and Kotlin.
