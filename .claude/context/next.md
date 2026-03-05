# Next Work Package

## Step: Idiomatic C# wrappers — infrastructure, constants, text utilities, and simple gen functions

## Goal

Establish the C# marshaling infrastructure (exception type, UTF-8 string conversion helpers, error
checking) and wrap the first 14 of 32 Tier 1 symbols with idiomatic PascalCase static methods. This
is the foundation — subsequent steps add remaining symbols mechanically.

## Scope

- **Create**: `packages/dotnet/Iscc.Lib/IsccException.cs`
- **Modify**: `packages/dotnet/Iscc.Lib/IsccLib.cs`
- **Reference**: `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`,
    `.claude/context/specs/dotnet-bindings.md`, `crates/iscc-ffi/src/lib.rs`

## Not In Scope

- Result record types (`MetaCodeResult`, `SumCodeResult`, etc.) — those are needed for
    `GenSumCodeV0` and `IsccDecode`, which are more complex and belong in a separate step
- Streaming types (`IsccDataHasher`, `IsccInstanceHasher` implementing `IDisposable`) — separate
    step after all static methods are wrapped
- Complex gen functions that take array-of-arrays or return structs: `GenVideoCodeV0`,
    `GenMixedCodeV0`, `GenIsccCodeV0`, `GenSumCodeV0` — defer to next step
- Codec functions (`IsccDecode`, `IsccDecompose`, `EncodeComponent`) — defer to next step
- Algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`) — defer
- Encoding utilities (`EncodeBase64`, `JsonToDataUrl`) — defer
- `SlidingWindow` — defer
- Conformance tests against `data.json` — separate step (needs result records for Gen functions)
- SafeHandle pattern — needed for streaming types, not for string-returning functions
- NuGet packaging and release pipeline
- Documentation (howto guide, README section)

## Implementation Notes

### IsccException.cs

Simple exception class for ISCC errors:

```csharp
namespace Iscc.Lib;

public class IsccException : Exception
{
    public IsccException(string message) : base(message) { }
}
```

### Marshaling Infrastructure in IsccLib.cs

Add private helper methods inside the `IsccLib` class. All string-returning FFI functions follow the
same pattern: call native → check null → marshal UTF-8 → free native string → return managed string.

**Key helpers (private, inside IsccLib):**

1. **`ToNativeUtf8(string? s)`** — converts a C# string to a pinned UTF-8 `byte[]` for passing to
    native code. Returns `null` for null input. Uses `System.Text.Encoding.UTF8.GetBytes` + null
    terminator.

2. **`ConsumeNativeString(byte* ptr)`** — marshals a native UTF-8 `byte*` to a managed `string`,
    then calls `NativeMethods.iscc_free_string(ptr)`. Throws `IsccException` if ptr is null (reads
    `NativeMethods.iscc_last_error()` for the message).

3. **`GetLastError()`** — reads `NativeMethods.iscc_last_error()`, returns the error string or a
    generic fallback.

**Pattern for every string-returning wrapper:**

```csharp
public static string GenTextCodeV0(string text, uint bits = 64)
{
    byte[] nativeText = ToNativeUtf8(text);
    unsafe
    {
        fixed (byte* pText = nativeText)
        {
            byte* result = NativeMethods.iscc_gen_text_code_v0(pText, bits);
            return ConsumeNativeString(result);
        }
    }
}
```

### Constants (5 symbols)

Expose as `public static uint` read-only properties delegating to `NativeMethods`:

- `MetaTrimName` → `NativeMethods.iscc_meta_trim_name()`
- `MetaTrimDescription` → `NativeMethods.iscc_meta_trim_description()`
- `MetaTrimMeta` → `NativeMethods.iscc_meta_trim_meta()`
- `IoReadSize` → `NativeMethods.iscc_io_read_size()`
- `TextNgramSize` → `NativeMethods.iscc_text_ngram_size()`

### Text Utilities (4 symbols)

All take `string` input, return `string`:

- `TextClean(string text)` → `iscc_text_clean`
- `TextRemoveNewlines(string text)` → `iscc_text_remove_newlines`
- `TextTrim(string text, uint nbytes)` → `iscc_text_trim`
- `TextCollapse(string text)` → `iscc_text_collapse`

### Gen Functions (4 symbols)

String-returning gen functions with simple input types:

- `GenMetaCodeV0(string name, string? description, string? meta, uint bits)` — 3 string inputs
    (description and meta are nullable, pass null pointer for null C# string)
- `GenTextCodeV0(string text, uint bits)` — single string input
- `GenDataCodeV0(ReadOnlySpan<byte> data, uint bits)` — byte span input, uses `fixed` pinning
- `GenInstanceCodeV0(ReadOnlySpan<byte> data, uint bits)` — same pattern as DataCode

### ConformanceSelftest Refactoring

Refactor the existing hand-written `DllImport` in `IsccLib.cs` to delegate to
`NativeMethods.iscc_conformance_selftest()` instead of its own `DllImport`. Remove the duplicate
`DllImport` and `LibName` constant.

### AllowUnsafeBlocks

The `.csproj` already has `<AllowUnsafeBlocks>true</AllowUnsafeBlocks>` — no change needed.

### Using Directives

Add `using System.Text;` for `Encoding.UTF8` and `using System.Runtime.InteropServices;` (already
present). Also add `using System.Runtime.CompilerServices;` if using `Unsafe.` helpers (likely not
needed).

### Byte Span to Native Pointer

For `GenDataCodeV0` and `GenInstanceCodeV0`, accept `ReadOnlySpan<byte>`:

```csharp
public static string GenDataCodeV0(ReadOnlySpan<byte> data, uint bits = 64)
{
    unsafe
    {
        fixed (byte* pData = data)
        {
            byte* result = NativeMethods.iscc_gen_data_code_v0(pData, (nuint)data.Length, bits);
            return ConsumeNativeString(result);
        }
    }
}
```

### Default Parameter Values

Match other bindings: `bits = 64` default for all gen/text functions. `GenMetaCodeV0` has
`description = null, meta = null, bits = 64`.

### Error Handling

All FFI functions that return `byte*` (string) return NULL on error and set a thread-local error
message readable via `iscc_last_error()`. The `ConsumeNativeString` helper must check for null, read
the error, and throw `IsccException`.

`iscc_last_error()` returns a pointer that must NOT be freed — it's thread-local static storage. Use
`Marshal.PtrToStringUTF8` to read it without freeing.

## Verification

- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds with 0 errors, 0 warnings
- `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` succeeds
- `dotnet test packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj -e LD_LIBRARY_PATH=target/debug`
    passes (existing ConformanceSelftest smoke test still works after refactoring to NativeMethods)
- `IsccLib.cs` contains at least these public methods: `ConformanceSelftest`, `TextClean`,
    `TextRemoveNewlines`, `TextTrim`, `TextCollapse`, `GenMetaCodeV0`, `GenTextCodeV0`,
    `GenDataCodeV0`, `GenInstanceCodeV0`
- `IsccLib.cs` contains at least these public properties: `MetaTrimName`, `MetaTrimDescription`,
    `MetaTrimMeta`, `IoReadSize`, `TextNgramSize`
- `IsccException.cs` exists in `packages/dotnet/Iscc.Lib/`
- No duplicate `DllImport` declarations in `IsccLib.cs` — all P/Invoke calls go through
    `NativeMethods`
- `cargo clippy -p iscc-ffi -- -D warnings` is still clean (no Rust changes in this step)

## Done When

All seven verification criteria pass — the C# project builds, the existing smoke test still passes,
and 14 Tier 1 symbols (5 constants + 4 text utilities + 4 gen functions + ConformanceSelftest) are
wrapped with idiomatic PascalCase methods delegating to `NativeMethods`.
