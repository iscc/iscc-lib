# Next Work Package

## Step: C# codec + sliding window wrappers (4 symbols → 26/32)

## Goal

Add idiomatic C# wrappers for `IsccDecode`, `IsccDecompose`, `EncodeComponent`, and `SlidingWindow`
— the codec and utility functions that bring the .NET binding from 22 to 26 of 32 Tier 1 symbols.
This introduces a `DecodeResult` record type and a shared NULL-terminated string array marshaling
helper.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — add `DecodeResult` record, 4 new public methods, and a
        private `ConsumeNativeStringArray` helper
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — add smoke tests for the 4 new wrappers
- **Reference**:
    - `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` — P/Invoke signatures for `iscc_decode`,
        `iscc_decompose`, `iscc_sliding_window`, `iscc_encode_component`, `iscc_free_decode_result`,
        `iscc_free_string_array`, and struct types `IsccDecodeResult`, `IsccByteBuffer`
    - `crates/iscc-ffi/src/lib.rs` — Rust FFI function implementations (for understanding semantics)

## Not In Scope

- Structured result records for gen functions (`MetaCodeResult`, `TextCodeResult`, etc.) — these are
    a separate refactoring step that changes existing API surface
- Algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`) — these
    need `IsccByteBuffer` / `IsccByteBufferArray` marshaling and are a separate step
- Streaming types (`IsccDataHasher`, `IsccInstanceHasher`) — require `IDisposable` + `SafeHandle`
    pattern, separate step
- Conformance tests, NuGet publishing, or documentation pages
- Refactoring existing `GenSumCodeV0` to use the new `ConsumeNativeStringArray` helper (nice-to-have
    but out of scope to keep the diff focused)

## Implementation Notes

### `DecodeResult` record

Add at namespace level (next to `SumCodeResult`):

```csharp
public sealed record DecodeResult(
    byte Maintype, byte Subtype, byte Version, byte Length, byte[] Digest);
```

### `IsccDecode` wrapper

- Call `NativeMethods.iscc_decode(pIscc)` → returns `IsccDecodeResult` struct
- Check `.ok` field — throw `IsccException(GetLastError())` on failure
- Copy `.digest.data` to managed `byte[]` using
    `new Span<byte>(result.digest.data, (int)result.digest.len).ToArray()`
- Free with `NativeMethods.iscc_free_decode_result(result)` in `finally` block
- Return
    `new DecodeResult(result.maintype, result.subtype, result.version, result.length, digestBytes)`

### `ConsumeNativeStringArray` private helper

Extract the NULL-terminated `byte**` → `string[]` pattern (already inlined in `GenSumCodeV0`):

```csharp
private static unsafe string[] ConsumeNativeStringArray(byte** arr)
{
    if (arr is null)
        throw new IsccException(GetLastError());
    try
    {
        var list = new List<string>();
        for (int i = 0; arr[i] != null; i++)
            list.Add(Marshal.PtrToStringUTF8((IntPtr)arr[i])!);
        return list.ToArray();
    }
    finally
    {
        NativeMethods.iscc_free_string_array(arr);
    }
}
```

### `IsccDecompose` and `SlidingWindow` wrappers

Both return `string[]` via the `ConsumeNativeStringArray` helper:

- `IsccDecompose(string isccCode)` → `iscc_decompose(pCode)` → `ConsumeNativeStringArray`
- `SlidingWindow(string seq, uint width)` → `iscc_sliding_window(pSeq, width)` →
    `ConsumeNativeStringArray`

### `EncodeComponent` wrapper

- Signature:
    `EncodeComponent(byte mtype, byte stype, byte version, uint bitLength,   ReadOnlySpan<byte> digest)`
- Call `iscc_encode_component(mtype, stype, version, bitLength, pDigest, digestLen)`
- Use existing `ConsumeNativeString` pattern — standard string return

### Tests to add (~4 tests)

- `IsccDecode_ReturnsDecodedComponents`: decode a known ISCC string (e.g., from `GenMetaCodeV0`),
    verify maintype/subtype are reasonable bytes, digest is non-empty byte array
- `IsccDecompose_ReturnsUnitArray`: compose an ISCC-CODE from data+instance codes, decompose it,
    verify result has 2+ entries each starting with "ISCC:"
- `EncodeComponent_ReturnsIsccString`: encode a component with known params (e.g., mtype=0, stype=0,
    version=0, bitLength=64, digest=8 zero bytes), verify result is a non-empty string
- `SlidingWindow_ReturnsNgrams`: call with "Hello World" and width 4, verify returns 8 n-grams
    ("Hell", "ello", "llo ", ...)

## Verification

- `cargo build -p iscc-ffi` succeeds (no Rust changes, just confirming FFI library builds)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds with 0 errors, 0 warnings
- `dotnet test packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj -e LD_LIBRARY_PATH=$(pwd)/target/debug`
    — all tests pass (25 existing + 4+ new)
- `grep -c 'public static' packages/dotnet/Iscc.Lib/IsccLib.cs` shows 26+ (22 existing + 4 new
    public methods)
- `grep 'DecodeResult' packages/dotnet/Iscc.Lib/IsccLib.cs` finds the record definition
- `mise run check` passes (all pre-commit/pre-push hooks green)

## Done When

All verification criteria pass: the .NET binding has 26 of 32 Tier 1 symbols wrapped with passing
smoke tests, including `IsccDecode` returning a `DecodeResult` record, `IsccDecompose` and
`SlidingWindow` returning `string[]`, and `EncodeComponent` returning a string.
