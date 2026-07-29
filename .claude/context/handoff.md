## 2026-07-29 — Mint `gen_iscc_id_v1` on the C# (.NET) surface

**Done:** Added idiomatic C# `GenIsccIdV1(ulong, ushort, byte)` wrapper + `IsccIdResult` record over
the already-generated `iscc_gen_iscc_id_v1` P/Invoke decl, taking the C# surface from 32→33 Tier 1
symbols (10th of 11 IDv1 fan-out surfaces, #43). No FFI regen, no P/Invoke change.

**Files changed:**

- `packages/dotnet/Iscc.Lib/Results.cs`: added `public sealed record IsccIdResult(string Iscc);`.
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: added `GenIsccIdV1` mirroring `GenMetaCodeV0` — `unsafe`
    call to `NativeMethods.iscc_gen_iscc_id_v1`, wrapped with `ConsumeNativeString` (throws
    `IsccException(GetLastError())` on NULL). No binding-side range guard (exact-width unsigned
    args; core re-validates ts→hub→realm).
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` (test, off-budget): golden, out-of-range-timestamp
    error, and decode round-trip tests.

**Verification:**

- `dotnet test packages/dotnet/Iscc.Lib.Tests` (LD_LIBRARY_PATH=target/debug): **Passed 107, Failed
    0** (was 104; +3 new). dotnet 8.0 SDK.
- Golden `GenIsccIdV1(1751831876325218, 1, 0).Iscc == "ISCC:MAIGHFECJMOPMIAB"` ✓.
- `GenIsccIdV1(1UL << 52, 0, 0)` throws `IsccException` ✓.
- Round-trip: `IsccDecode` → Maintype 6, Version 1, 8-byte Digest; `ts = body>>12`,
    `hub = body&0xFFF`, `realm = Subtype` all equal inputs ✓.
- `grep -c GenIsccIdV1 IsccLib.cs` = 1; `IsccIdResult` present in Results.cs ✓.
- `git diff --quiet -- NativeMethods.g.cs` clean after `cargo build -p iscc-ffi` (P/Invoke
    untouched) ✓.
- `mise run check` — all 18 prek hygiene + parity + lint hooks Passed.

**Next:** #43 has **one minting surface left**: C++ (`packages/cpp/include/iscc.hpp`) wrapper over
the same fixed-width FFI (needs `uv run --with cmake cmake` to verify). After that lands, do the
deferred repo-wide Tier-1 32→33 doc/count sweep (issues.md, `gen_iscc_id_v1` API-doc entries,
`packages/dotnet/CLAUDE.md`/`README.md` counts). #43 stays open (v0.6.0 blocker).

**Notes:** No enum-widening needed — C# `DecodeResult.Version` is a raw `byte`, so the generic
decode path already accepts Version 1 (confirmed by the passing round-trip). This surface is exempt
from the open Ruby i64 marshalling gap: the FFI takes exact-width unsigned types, so callers cannot
pass out-of-width values and core does the ordered range validation. Doc counts left untouched per
Not-In-Scope.
