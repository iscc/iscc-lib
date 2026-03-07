# Next Work Package

## Step: C# / .NET documentation (howto guide, package README, root README section)

## Goal

Add complete C# / .NET documentation: a `docs/howto/dotnet.md` how-to guide, a
`packages/dotnet/README.md` for NuGet, C# sections in the root `README.md`, and the navigation entry
in `zensical.toml`. This makes the .NET binding discoverable and usable by C# developers.

## Scope

- **Create**:
    - `docs/howto/dotnet.md` — comprehensive how-to guide (~400 lines, following ruby.md/java.md
        pattern)
    - `packages/dotnet/README.md` — package README for NuGet (~80-90 lines, following iscc-rb/iscc-jni
        README pattern)
- **Modify**:
    - `README.md` — add C# / .NET install subsection under `## Installation` (between Ruby and WASM)
        and C# quickstart subsection under `## Quick Start` (between Ruby and WASM); add NuGet badge
        to badge block; update "Polyglot" bullet to include C#
    - `zensical.toml` — add `{ "C# / .NET" = "howto/dotnet.md" }` entry to How-to Guides nav (after
        Java, before C / C++)
- **Reference**:
    - `docs/howto/ruby.md` — primary template for howto structure and section ordering
    - `docs/howto/java.md` — secondary template (similar P/Invoke-style native binding)
    - `crates/iscc-rb/README.md` — template for per-package README
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — public API surface (methods, signatures, docstrings)
    - `packages/dotnet/Iscc.Lib/Results.cs` — result record types
    - `packages/dotnet/Iscc.Lib/IsccDataHasher.cs` — streaming hasher API
    - `packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs` — streaming hasher API
    - `packages/dotnet/Iscc.Lib/IsccException.cs` — exception type
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — verified usage examples

## Not In Scope

- NuGet publish pipeline in `release.yml` — separate step
- Version sync integration for `.csproj` — separate step
- `SafeHandles.cs` extraction — separate step
- C# API reference page (`docs/dotnet-api.md`) — can follow later as a separate step like
    `ruby-api.md` and `java-api.md`
- Adding NuGet badge URL that resolves (package not published yet) — use a placeholder URL that will
    become live after publish
- Modifying any C# source code

## Implementation Notes

### `docs/howto/dotnet.md`

Follow the exact structure from `ruby.md` and `java.md`. Sections in order:

1. **Frontmatter**: `icon: lucide/hash` (or `lucide/code`), description
2. **Title**: `# C# / .NET`
3. **Intro paragraph**: P/Invoke bindings via csbindgen, static methods on `IsccLib` class,
    `Iscc.Lib` namespace
4. **Installation**: `dotnet add package Iscc.Lib` (with "build from source" admonition showing
    `cargo build -p iscc-ffi` + `dotnet build`)
5. **Code generation**: All 10 `gen_*_v0` functions with C# code examples. Methods use PascalCase
    (`GenMetaCodeV0`). Results are typed records (`MetaCodeResult`, `TextCodeResult`, etc.) with
    `.Iscc` property. Show optional parameters with named args (`bits: 64`)
6. **Streaming**: `IsccDataHasher` and `IsccInstanceHasher` — `using` pattern (IDisposable),
    `Update(ReadOnlySpan<byte>)`, `Finalize()` returning `DataCodeResult`/`InstanceCodeResult`
7. **Codec operations**: `IsccDecode`, `IsccDecompose`, `EncodeComponent`
8. **Text utilities**: `TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse`
9. **Encoding utilities**: `EncodeBase64`, `JsonToDataUrl`
10. **Algorithm primitives**: `AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`,
    `SlidingWindow`
11. **Constants**: `MetaTrimName`, `MetaTrimDescription`, `MetaTrimMeta`, `IoReadSize`,
    `TextNgramSize` (properties, not fields)
12. **Conformance testing**: `ConformanceSelftest()` returns count of passed vectors

All code examples must use `Iscc.Lib` namespace, `using Iscc.Lib;`, and PascalCase method names.
Byte arrays use `ReadOnlySpan<byte>` or `byte[]` as appropriate per the actual API.

### `packages/dotnet/README.md`

Follow `crates/iscc-rb/README.md` pattern:

- Badges (NuGet version placeholder, CI, license)
- One-line description
- What is ISCC (2-3 sentences)
- Installation (`dotnet add package Iscc.Lib`)
- Quick start (gen_meta_code_v0 example)
- API overview (list of 10 gen functions + key utilities)
- Links to lib.iscc.codes, repository, ISO spec
- License: Apache-2.0

### `README.md` modifications

1. Add NuGet badge after Maven Central badge:
    `[![NuGet](https://img.shields.io/nuget/v/Iscc.Lib.svg)](https://www.nuget.org/packages/Iscc.Lib)`
2. Update "Polyglot" bullet in Key Features to include "C#"
3. Update "What is iscc-lib" paragraph to include "C#"
4. Add `### C# / .NET` subsection under Installation (between Ruby and WASM):
    ```
    dotnet add package Iscc.Lib
    ```
5. Add `### C# / .NET` quickstart under Quick Start (between Ruby and WASM):
    ```csharp
    using Iscc.Lib;

    var result = IsccLib.GenMetaCodeV0("ISCC Test Document!");
    Console.WriteLine($"Meta-Code: {result.Iscc}");
    ```

### `zensical.toml`

Add to nav after Java entry:

```toml
{ "C# / .NET" = "howto/dotnet.md" },
```

## Verification

- `uv run zensical build` exits 0 (docs site builds with new page)
- `test -f docs/howto/dotnet.md` exits 0
- `test -f packages/dotnet/README.md` exits 0
- `grep -c 'C# / .NET' README.md` returns at least 2 (install + quickstart sections)
- `grep -c 'NuGet' README.md` returns at least 1 (badge)
- `grep -c 'dotnet.md' zensical.toml` returns 1 (nav entry)
- `grep -c 'GenMetaCodeV0' docs/howto/dotnet.md` returns at least 1
- `grep -c 'IsccDataHasher' docs/howto/dotnet.md` returns at least 1
- `grep -c 'GenMetaCodeV0' packages/dotnet/README.md` returns at least 1
- `mise run lint` passes (formatting clean)

## Done When

All verification checks pass: docs site builds, howto guide and package README exist with complete
C# examples, root README includes C# install/quickstart sections and NuGet badge, and zensical.toml
navigation includes the C# / .NET entry.
