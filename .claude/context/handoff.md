## 2026-03-07 — C# algorithm primitives (4 symbols → 30/32)

**Done:** Added 4 algorithm primitive wrappers (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`,
`SoftHashVideoV0`) plus 2 private helpers (`ConsumeByteBuffer`, `ConsumeByteBufferArray`) to the C#
binding, advancing from 26 to 30 of 32 Tier 1 symbols.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Added `// ── Algorithm Primitives ──` section with 4 public
    methods + 2 private `Consume*` helpers for `IsccByteBuffer`/`IsccByteBufferArray` marshaling
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Added 6 tests covering all 4 new methods including
    edge cases (empty input for SimHash, empty data for CDC)

**Verification:**

- `cargo build -p iscc-ffi` — succeeds
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 35 passed, 0 failed (29
    existing + 6 new)
- `grep -c 'public static' IsccLib.cs` — 31 (30 symbols + 1 class decl)
- `mise run check` — all 15 hooks pass

**Next:** Streaming types (`IsccDataHasher`, `IsccInstanceHasher`) requiring `IDisposable` +
`SafeHandle` pattern for the final 2 of 32 Tier 1 symbols. After that: conformance tests, howto
documentation, and API reference page.

**Notes:**

- `ConsumeByteBuffer` and `ConsumeByteBufferArray` follow the established `ConsumeNativeString` /
    `ConsumeNativeStringArray` pattern — null check + data copy + free in finally
- `AlgSimhash` and `SoftHashVideoV0` use the same `GCHandle` pinning pattern already established by
    `GenVideoCodeV0` — consistent marshaling across all jagged array parameters
- `AlgSimhash` with empty input returns 32 zero bytes (FFI passes through to Rust which returns
    `vec![0u8; 32]` for empty)
- `dotnet` is at `/usr/share/dotnet/dotnet` in the current devcontainer (not `/home/dev/.dotnet`)
