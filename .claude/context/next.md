# Next Work Package

## Step: C# structured result records (Results.cs + gen function return type refactor)

## Goal

Refactor all 9 `string`-returning gen functions in `IsccLib.cs` to return typed result records,
matching the Rust core's structured return types. This establishes the idiomatic C# API surface
specified in the dotnet-bindings spec and also fixes the 4 remaining empty-span NULL pointer bugs.

## Scope

- **Create**:
    - `packages/dotnet/Iscc.Lib/Results.cs` — 11 `sealed record` types: 9 new gen-function results
        (`MetaCodeResult`, `TextCodeResult`, `ImageCodeResult`, `AudioCodeResult`, `VideoCodeResult`,
        `MixedCodeResult`, `DataCodeResult`, `InstanceCodeResult`, `IsccCodeResult`) + 2 relocated
        from `IsccLib.cs` (`SumCodeResult`, `DecodeResult`)
- **Modify**:
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — (a) change 9 gen function return types from `string` to
        their respective record types; (b) remove the `SumCodeResult` and `DecodeResult` definitions
        (moved to `Results.cs`); (c) fix 4 remaining empty-span NULL pointer bugs in `GenImageCodeV0`,
        `AlgMinhash256`, `AlgCdcChunks`, `EncodeBase64`
- **Reference**:
    - `crates/iscc-lib/src/types.rs` — Rust core result struct definitions (field names and types)
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — current implementation and existing empty-span fix
        pattern (see `GenAudioCodeV0`, `GenDataCodeV0`, `GenInstanceCodeV0`)
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — tests that will need `.Iscc` accessor
    - `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` — conformance tests that will need `.Iscc`
        accessor
    - `.claude/context/specs/dotnet-bindings.md` — target API shape and record type examples

## Not In Scope

- **Streaming hasher `Finalize()` return types** — `IsccDataHasher.Finalize()` and
    `IsccInstanceHasher.Finalize()` currently return `string`; changing them to return
    `DataCodeResult`/`InstanceCodeResult` is a separate step (would exceed the 3-file limit)
- **FFI-level structured results** — the C FFI currently returns only the ISCC string via
    `.map(|r| r.iscc)`. Adding FFI structs to expose additional fields (e.g., `MetaCodeResult.Name`,
    `TextCodeResult.Characters`, `InstanceCodeResult.DataHash`) requires
    `crates/iscc-ffi/src/lib.rs` changes + csbindgen regeneration — separate step(s)
- **Populating additional record fields** — for records whose Rust core type has extra fields (Meta,
    Text, Mixed, Instance), only the `Iscc` field is populated from FFI. Additional fields will be
    added when FFI structured result support is implemented
- **Documentation** — `docs/howto/dotnet.md`, `packages/dotnet/README.md`, README C# section
- **NuGet publish pipeline** — separate step

## Implementation Notes

### Record Type Design

Use positional `sealed record` syntax matching the existing `SumCodeResult` pattern. All 9 new
records start with only the `Iscc` field since the C FFI only provides the ISCC string:

```csharp
/// <summary>Result of GenMetaCodeV0.</summary>
public sealed record MetaCodeResult(string Iscc);

/// <summary>Result of GenTextCodeV0.</summary>
public sealed record TextCodeResult(string Iscc);

/// <summary>Result of GenImageCodeV0.</summary>
public sealed record ImageCodeResult(string Iscc);

/// <summary>Result of GenAudioCodeV0.</summary>
public sealed record AudioCodeResult(string Iscc);

/// <summary>Result of GenVideoCodeV0.</summary>
public sealed record VideoCodeResult(string Iscc);

/// <summary>Result of GenMixedCodeV0.</summary>
public sealed record MixedCodeResult(string Iscc);

/// <summary>Result of GenDataCodeV0.</summary>
public sealed record DataCodeResult(string Iscc);

/// <summary>Result of GenInstanceCodeV0.</summary>
public sealed record InstanceCodeResult(string Iscc);

/// <summary>Result of GenIsccCodeV0.</summary>
public sealed record IsccCodeResult(string Iscc);
```

For Image, Audio, Video, Data, and IsccCode — these match the Rust core exactly (the Rust types only
have `iscc`). For Meta, Text, Mixed, and Instance — these are intentionally incomplete and will be
expanded when FFI structured result support is added.

Also relocate the existing `SumCodeResult` and `DecodeResult` from `IsccLib.cs` into `Results.cs`
unchanged. They remain in the `Iscc.Lib` namespace so no consumer code breaks.

### Gen Function Return Type Changes

Each gen function wraps its string return in the corresponding record:

```csharp
// Before:
public static string GenMetaCodeV0(...) { ... return ConsumeNativeString(result); }

// After:
public static MetaCodeResult GenMetaCodeV0(...)
{
    ...
    return new MetaCodeResult(ConsumeNativeString(result));
}
```

### Empty-Span NULL Pointer Fixes

Apply the same guard pattern already used in `GenAudioCodeV0`, `GenDataCodeV0`, and
`GenInstanceCodeV0` to the 4 remaining affected functions:

- `GenImageCodeV0` — `ReadOnlySpan<byte> pixels`
- `AlgMinhash256` — `ReadOnlySpan<uint> features`
- `AlgCdcChunks` — `ReadOnlySpan<byte> data`
- `EncodeBase64` — `ReadOnlySpan<byte> data`

Pattern: check `span.IsEmpty`, if so use a stack-allocated sentinel variable instead of `fixed`.

### Test Updates (excluded from file count)

Both `SmokeTests.cs` and `ConformanceTests.cs` need updates:

- **SmokeTests.cs**: Change `string result = IsccLib.GenFooV0(...)` to
    `var result = IsccLib.GenFooV0(...)` and assertions from `result` to `result.Iscc`. The
    `GenMixedCodeV0` and `GenIsccCodeV0` tests use intermediate gen calls — those intermediate
    results also need `.Iscc` to extract the string for input to the next function.
- **ConformanceTests.cs**: Change `string result = IsccLib.GenFooV0(...)` to
    `var result = IsccLib.GenFooV0(...)` and `Assert.Equal(expected, result)` to
    `Assert.Equal(expected, result.Iscc)`.
- **Streaming hasher comparison tests**: `DataHasher_MatchesGenDataCodeV0` and
    `InstanceHasher_MatchesGenInstanceCodeV0` compare gen result with hasher result. Since hasher
    still returns `string`, use `expected.Iscc` in the comparison.

## Verification

- `cargo build -p iscc-ffi` succeeds (no Rust changes, but confirms FFI library builds)
- `dotnet build packages/dotnet/Iscc.Lib/` succeeds (new Results.cs compiles, modified IsccLib.cs
    compiles)
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug`
    passes — all 91 tests (41 smoke + 50 conformance)
- `grep -c 'sealed record' packages/dotnet/Iscc.Lib/Results.cs` outputs `11` (9 new + 2 relocated)
- `grep -c 'sealed record' packages/dotnet/Iscc.Lib/IsccLib.cs` outputs `0` (all moved to
    Results.cs)

## Done When

All verification criteria pass — 9 gen functions return typed result records, 4 empty-span bugs
fixed, all 91 tests pass with the new return types.
