# Next Work Package

## Step: C# wrappers — remaining gen functions + encoding utilities

## Goal

Add idiomatic C# wrappers for the 6 remaining gen functions (`GenImageCodeV0`, `GenAudioCodeV0`,
`GenVideoCodeV0`, `GenMixedCodeV0`, `GenIsccCodeV0`, `GenSumCodeV0`) plus 2 encoding utilities
(`EncodeBase64`, `JsonToDataUrl`). This completes all 10 gen functions in C# and introduces 4 new
marshaling patterns (int arrays, jagged int arrays, string arrays, struct returns) that enable the
remaining wrapper steps.

## Scope

- **Create**: (none)
- **Modify**:
    1. `packages/dotnet/Iscc.Lib/IsccLib.cs` — add 8 new public methods + `SumCodeResult` record type
        \+ private helpers for new marshaling patterns
    2. `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — add tests for all 8 new symbols
- **Reference**:
    - `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` — P/Invoke signatures and struct definitions
    - `packages/dotnet/Iscc.Lib/IsccException.cs` — exception type (already exists)
    - `crates/iscc-ffi/src/lib.rs` — FFI implementation for behavior reference
    - `crates/iscc-jni/src/lib.rs` — Java JNI wrappers for pattern reference (especially
        GenVideoCodeV0 jagged array handling)

## Not In Scope

- Codec functions (`IsccDecode`, `IsccDecompose`, `EncodeComponent`) — separate step with
    `DecodeResult` record type and string-array return marshaling
- Algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`) — these
    return `IsccByteBuffer` structs, different marshaling from gen functions
- `SlidingWindow` — returns string array (same step as codec)
- Streaming types (`DataHasher`, `InstanceHasher` with `IDisposable`) — separate step
- Conformance tests against `data.json` — separate step after all 32 symbols are wrapped
- Documentation (`docs/howto/dotnet.md`, README C# section) — after wrapper layer is complete
- NuGet packaging and release pipeline — separate step

## Implementation Notes

### New marshaling patterns needed

1. **`int[]` input** (`GenAudioCodeV0`): Pin with `fixed (int* pCv = cv)`, pass length as
    `(nuint)cv.Length`. Same pattern as `byte[]` in `GenDataCodeV0` but with `int*` pointer type.

2. **`int[][]` jagged array** (`GenVideoCodeV0`): Most complex pattern. Each inner `int[]` must be
    pinned simultaneously. Use `GCHandle.Alloc(arr, GCHandleType.Pinned)` for each inner array,
    build `int*[]` and `nuint[]` arrays from pinned addresses, then `fixed` on those outer arrays.
    Free all handles in a `finally` block. NativeMethods signature:
    `iscc_gen_video_code_v0(int** frame_sigs, nuint* frame_lens, nuint num_frames, uint bits)`

3. **`string[]` input** (`GenMixedCodeV0`, `GenIsccCodeV0`): Convert each string to UTF-8 `byte[]`
    via `ToNativeUtf8`, pin each with `GCHandle`, build `byte*[]` from pinned addresses, `fixed` on
    the pointer array. Free handles in `finally`. NativeMethods signatures use
    `byte** codes, nuint num_codes`.

4. **Struct return + free** (`GenSumCodeV0`): Returns `IsccSumCodeResult` struct (defined in
    NativeMethods.g.cs). Create a managed `SumCodeResult` record class to hold the result. Marshal
    fields: check `ok` field (throw on false via `GetLastError`), read `iscc`/`datahash` strings
    via `Marshal.PtrToStringUTF8`, read `filesize`, optionally read `units` NULL-terminated string
    array. Always call `iscc_free_sum_code_result(result)` in a `finally` block — this function
    takes the struct **by value** (not by pointer).

### SumCodeResult record type

Define inside or alongside `IsccLib` in `IsccLib.cs`:

```csharp
/// <summary>Result of GenSumCodeV0 — composite ISCC-CODE with file metadata.</summary>
public sealed record SumCodeResult(
    string Iscc,
    string Datahash,
    ulong Filesize,
    string[]? Units);
```

### Simple wrappers (follow existing patterns)

- **`GenImageCodeV0(ReadOnlySpan<byte> pixels, uint bits = 64)`**: Same pattern as `GenDataCodeV0` —
    `fixed (byte* p = pixels)`, call native, `ConsumeNativeString`.
- **`EncodeBase64(ReadOnlySpan<byte> data)`**: Same pattern — `fixed`, call, consume.
- **`JsonToDataUrl(string json)`**: Same pattern as text utilities — `ToNativeUtf8`, `fixed`, call,
    consume.

### GenAudioCodeV0 signature

Takes `ReadOnlySpan<int> cv` + `uint bits = 64`. NativeMethods signature:
`iscc_gen_audio_code_v0(int* cv, nuint cv_len, uint bits)`.

### GenIsccCodeV0 signature note

Unlike other gen functions, `GenIsccCodeV0` takes `bool wide` instead of `uint bits`. The
NativeMethods signature is: `iscc_gen_iscc_code_v0(byte** codes, nuint num_codes, bool wide)`.
Default: `wide = false`.

### GenSumCodeV0 signature

Takes file path (string) + bits + wide + addUnits. NativeMethods signature:
`iscc_gen_sum_code_v0(byte* path, uint bits, bool wide, bool add_units)`. Returns
`IsccSumCodeResult` struct. Must free with `iscc_free_sum_code_result()`. Default:
`bits = 64, wide = false, addUnits = false`.

### Units array marshaling in SumCodeResult

The `units` field is `byte** units` — a NULL-terminated array of UTF-8 strings. Walk until null:

```csharp
List<string> unitsList = new();
for (int i = 0; result.units[i] != null; i++)
    unitsList.Add(Marshal.PtrToStringUTF8((IntPtr)result.units[i])!);
```

Pass `null` for `Units` when `addUnits` is `false` (units pointer will be NULL).

### Test patterns

Each new wrapper should have at least one smoke test. For content-code gen functions that need
specific input data (pixels, audio features, video frames), use small synthetic arrays — the goal is
to verify P/Invoke marshaling works, not to test algorithm correctness (that's `ConformanceSelftest`
and future data.json conformance tests). Examples:

- `GenImageCodeV0`: 1024-byte array of zeros (32x32 grayscale)
- `GenAudioCodeV0`: small `int[]` of arbitrary values (minimum ~32 elements)
- `GenVideoCodeV0`: 2-3 frames of small `int[]` arrays
- `GenMixedCodeV0`/`GenIsccCodeV0`: use outputs from other gen functions as input codes
- `GenSumCodeV0`: create a temp file with some content, pass its path — verify `SumCodeResult`
    fields are populated (Iscc starts with "ISCC:", Filesize > 0)
- `EncodeBase64`: encode known bytes, verify output matches expected base64url string
- `JsonToDataUrl`: pass valid JSON, verify result starts with `data:`

## Verification

- `cargo build -p iscc-ffi` succeeds (no Rust changes expected, confirms FFI lib builds)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds with 0 errors, 0 warnings
- `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` succeeds
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug` — all tests
    pass (16 existing + at least 8 new = 24+ total)
- `grep -c 'public static string Gen.*CodeV0\|public static SumCodeResult Gen' packages/dotnet/Iscc.Lib/IsccLib.cs`
    returns 10 (all 10 gen functions wrapped)
- `grep -c 'public sealed record SumCodeResult' packages/dotnet/Iscc.Lib/IsccLib.cs` returns 1
- `mise run check` — all hooks pass

## Done When

All verification criteria pass — all 10 gen functions and 2 encoding utilities are callable from C#
with correct marshaling, `SumCodeResult` record type exists, and 24+ tests pass including smoke
tests for all 8 new symbols.
