## 2026-03-07 — Review of: C# codec + sliding window wrappers (4 symbols → 26/32)

**Verdict:** PASS

**Summary:** Clean implementation of 4 new C# wrappers (`IsccDecode`, `IsccDecompose`,
`EncodeComponent`, `SlidingWindow`) plus `DecodeResult` record and `ConsumeNativeStringArray`
helper. All verification criteria pass: 29/29 .NET tests green, `mise run check` all 15 hooks pass,
clippy clean. Code follows established marshaling patterns consistently.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — 0 errors
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- [x] `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 29 passed, 0 failed
    (25 existing + 4 new)
- [x] `grep -c 'public static' IsccLib.cs` — 27 (26 methods/properties + 1 class decl = correct)
- [x] `grep 'DecodeResult' IsccLib.cs` — finds record definition, `IsccDecode` return type, and
    constructor
- [x] `mise run check` — all 15 hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** Codex confirms the wrappers are consistent with native FFI contracts
(NULL-terminated `byte**` + `iscc_free_string_array`, `IsccDecodeResult` +
`iscc_free_decode_result`). No actionable findings.

**Next:** Algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`)
which require `IsccByteBuffer` / `IsccByteBufferArray` marshaling for byte-array inputs and outputs.
This brings the .NET binding from 26 to 30 of 32 Tier 1 symbols. After that: streaming types
(`DataHasher`, `InstanceHasher`) requiring `IDisposable` + `SafeHandle` pattern (final 2 symbols).

**Notes:**

- .NET binding now at 26/32 Tier 1 symbols (5 constants, 4 text utils, 10 gen, 2 encoding utils, 3
    codec, 1 sliding window, 1 conformance)
- Remaining 6 symbols: 4 algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`,
    `SoftHashVideoV0`), 2 streaming types (`DataHasher`, `InstanceHasher`)
- `encode_component` returns base32 string WITHOUT "ISCC:" prefix — tests correctly use
    `Assert.NotEmpty` instead of `Assert.StartsWith("ISCC:")`
- `GenSumCodeV0` still has inlined string-array marshaling (not refactored to use
    `ConsumeNativeStringArray`) — explicitly out of scope, could be cleaned up in a future iteration
- The `ConsumeNativeStringArray` helper can be reused for any future FFI function that returns
    `byte**` (NULL-terminated string arrays)
