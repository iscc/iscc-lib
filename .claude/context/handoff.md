## 2026-03-07 — Review of: C# algorithm primitives (4 symbols → 30/32)

**Verdict:** PASS

**Summary:** Clean implementation of 4 algorithm primitive wrappers (`AlgSimhash`, `AlgMinhash256`,
`AlgCdcChunks`, `SoftHashVideoV0`) plus 2 private helpers (`ConsumeByteBuffer`,
`ConsumeByteBufferArray`). All 5 verification criteria pass: 35/35 .NET tests green,
`mise run check` all 15 hooks pass, clippy clean. Code follows established marshaling patterns
consistently. Scope discipline is perfect — only the two specified files modified.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — 0 errors
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- [x] `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 35 passed, 0 failed
    (29 existing + 6 new)
- [x] `grep -c 'public static' IsccLib.cs` — 31 (30 symbols + 1 class decl = correct)
- [x] `mise run check` — all 15 hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** P3 finding about `SoftHashVideoV0` empty-input edge case: C# `fixed` on
zero-length arrays produces null pointers, so the FFI would return "frame_sigs must not be NULL"
instead of the Rust core's "frame_sigs must not be empty" error. This is the same behavior as the
existing `GenVideoCodeV0` wrapper and all other jagged-array functions — the error path still
triggers correctly, only the message differs. Not actionable for this iteration.

**Next:** Streaming types (`IsccDataHasher`, `IsccInstanceHasher`) requiring `IDisposable` +
`SafeHandle` pattern for the final 2 of 32 Tier 1 symbols. After that: conformance tests
(`ConformanceTests.cs` + vendored `data.json`), howto documentation, and API reference page.

**Notes:**

- .NET binding now at 30/32 Tier 1 symbols (5 constants, 4 text utils, 10 gen, 2 encoding utils, 3
    codec, 1 sliding window, 4 algorithm primitives, 1 conformance)
- Remaining 2 symbols: streaming types (`DataHasher`, `InstanceHasher`)
- `ConsumeByteBuffer`/`ConsumeByteBufferArray` helpers follow established `ConsumeNativeString`
    pattern — null check + data copy + free in finally
- `AlgSimhash`/`SoftHashVideoV0` use same `GCHandle` pinning pattern as `GenVideoCodeV0`
