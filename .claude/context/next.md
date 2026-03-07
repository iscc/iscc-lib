# Next Work Package

## Step: C# algorithm primitives (4 symbols → 30/32)

## Goal

Add the 4 remaining algorithm primitive wrappers (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`,
`SoftHashVideoV0`) to the C# binding, advancing from 26 to 30 of 32 Tier 1 symbols. These require
new `IsccByteBuffer` / `IsccByteBufferArray` marshaling helpers that return `byte[]` and `byte[][]`
instead of strings.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — add 4 public methods + 2 private helpers
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — add tests for each new method
- **Reference**:
    - `crates/iscc-ffi/src/lib.rs` — FFI signatures for `iscc_alg_simhash`, `iscc_alg_minhash_256`,
        `iscc_alg_cdc_chunks`, `iscc_soft_hash_video_v0`, `iscc_free_byte_buffer`,
        `iscc_free_byte_buffer_array`
    - `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` — generated P/Invoke declarations and
        `IsccByteBuffer` / `IsccByteBufferArray` struct definitions

## Not In Scope

- Streaming types (`IsccDataHasher`, `IsccInstanceHasher`) — that's the next step (symbols 31-32)
- Structured result records for gen functions (`MetaCodeResult`, `TextCodeResult`, etc.)
- Conformance tests (`ConformanceTests.cs` + vendored `data.json`)
- Documentation (`docs/howto/dotnet.md`, README C# section)
- Refactoring existing `GenSumCodeV0` to use the new helpers — it has its own inline marshaling that
    works fine

## Implementation Notes

### New private helpers (add to IsccLib.cs private section)

1. **`ConsumeByteBuffer(IsccByteBuffer buf)`** → `byte[]`: Check `buf.data` for null (throw
    `IsccException(GetLastError())`), copy `buf.len` bytes via
    `new Span<byte>(buf.data,  (int)buf.len).ToArray()`, then call
    `NativeMethods.iscc_free_byte_buffer(buf)` in `finally`. Pattern mirrors `ConsumeNativeString`
    but returns `byte[]` instead of `string`.

2. **`ConsumeByteBufferArray(IsccByteBufferArray arr)`** → `byte[][]`: Check `arr.buffers` for null
    (throw), iterate `arr.count` elements copying each buffer's data to `byte[]`, then call
    `NativeMethods.iscc_free_byte_buffer_array(arr)` in `finally`.

### 4 public wrapper methods

1. **`AlgSimhash(byte[][] digests)`** → `byte[]`:

    - Pin each inner `byte[]` with `GCHandle.Alloc(GCHandleType.Pinned)` (same pattern as
        `GenVideoCodeV0` but with `byte**` instead of `int**`)
    - Build `byte*[]` pointer array and `nuint[]` lengths array
    - Call `NativeMethods.iscc_alg_simhash(pPtrs, pLens, numDigests)`
    - Return via `ConsumeByteBuffer`
    - Free GCHandles in `finally`

2. **`AlgMinhash256(ReadOnlySpan<uint> features)`** → `byte[]`:

    - Simplest — just `fixed (uint* pFeatures = features)` and call
        `NativeMethods.iscc_alg_minhash_256(pFeatures, (nuint)features.Length)`
    - Return via `ConsumeByteBuffer`

3. **`AlgCdcChunks(ReadOnlySpan<byte> data, bool utf32 = false, uint avgChunkSize = 1024)`** →
    `byte[][]`:

    - `fixed (byte* pData = data)` and call
        `NativeMethods.iscc_alg_cdc_chunks(pData, (nuint)data.Length, utf32, avgChunkSize)`
    - Return via `ConsumeByteBufferArray`

4. **`SoftHashVideoV0(int[][] frameSigs, uint bits = 64)`** → `byte[]`:

    - Same GCHandle pinning pattern as `GenVideoCodeV0` (already implemented)
    - Call `NativeMethods.iscc_soft_hash_video_v0(pPtrs, pLens, numFrames, bits)`
    - Return via `ConsumeByteBuffer`

### Section organization

Add a new `// ── Algorithm Primitives ──` section in IsccLib.cs between Utilities and Diagnostics.
Place the 4 methods in order: `AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`.

### Tests

Add to SmokeTests.cs an `// ── Algorithm Primitives ──` section with tests:

- `AlgSimhash_ReturnsByteArray` — 2 digests of 4 bytes each → result is 4 bytes (matches input
    digest length)
- `AlgSimhash_EmptyInput_Returns32Bytes` — empty array → 32 zero bytes
- `AlgMinhash256_ReturnsByteArray` — `[1u, 2u, 3u, 4u, 5u]` features → 32 bytes result
- `AlgCdcChunks_SplitsData` — "Hello World" → at least 1 chunk, concatenated chunks == original
- `AlgCdcChunks_EmptyData` — empty span → 1 chunk of 0 bytes
- `SoftHashVideoV0_ReturnsByteArray` — 2 frames of 380 i32 each, bits=64 → 8 bytes result

## Verification

- `cargo build -p iscc-ffi` succeeds (FFI lib must compile)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds with 0 errors, 0 warnings
- `dotnet test packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug`
    — all tests pass (29 existing + 6 new = 35 total)
- `grep -c 'public static' packages/dotnet/Iscc.Lib/IsccLib.cs` shows 31 (30 symbols + 1 class decl)
- `mise run check` — all hooks pass (formatting, clippy, etc.)

## Done When

All 5 verification criteria pass, confirming the C# binding has 30 of 32 Tier 1 symbols with clean
builds, passing tests, and clean linting.
