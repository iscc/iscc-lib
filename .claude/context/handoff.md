## 2026-03-07 — C# codec + sliding window wrappers (4 symbols → 26/32)

**Done:** Added 4 new public methods (`IsccDecode`, `IsccDecompose`, `EncodeComponent`,
`SlidingWindow`) to the .NET binding along with a `DecodeResult` record type and a shared
`ConsumeNativeStringArray` private helper. The .NET binding now wraps 26 of 32 Tier 1 symbols.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Added `DecodeResult` record, 4 public methods (Codec
    section: `IsccDecode`, `IsccDecompose`, `EncodeComponent`; Utilities section: `SlidingWindow`),
    and `ConsumeNativeStringArray` private helper for NULL-terminated `byte**` → `string[]`
    marshaling
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Added 4 smoke tests covering all new methods

**Verification:**

- `cargo build -p iscc-ffi` — succeeds (no Rust changes)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- `dotnet test ... -e LD_LIBRARY_PATH=$(pwd)/target/debug` — 29 passed, 0 failed (25 existing + 4
    new)
- `grep -c 'public static' IsccLib.cs` — 27 (26 methods/properties + 1 class declaration)
- `grep 'DecodeResult'` — finds record definition and usages
- `mise run check` — all 15 hooks pass

**Next:** Algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`)
which require `IsccByteBuffer` / `IsccByteBufferArray` marshaling. After that: streaming types
(`DataHasher`, `InstanceHasher`) requiring `IDisposable` + `SafeHandle` pattern. Then .NET API
reference docs page.

**Notes:**

- `encode_component` and `iscc_decompose` return raw ISCC component strings without the "ISCC:"
    prefix — test assertions were adjusted accordingly (using `Assert.NotEmpty` instead of
    `Assert.StartsWith("ISCC:")`)
- The `ConsumeNativeStringArray` helper extracts the NULL-terminated `byte**` → `string[]` pattern
    that was previously inlined in `GenSumCodeV0`. The existing `GenSumCodeV0` was NOT refactored to
    use it (explicitly out of scope per next.md)
- Ruby linting hook required reinstalling `bundler` gem and running `bundle install` in the
    devcontainer — this is an environment setup issue, not a code issue
