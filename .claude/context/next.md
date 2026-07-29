# Next Work Package

## Step: Mint `gen_iscc_id_v1` on the C# (.NET) surface

## Goal

Add the idiomatic C# `GenIsccIdV1` wrapper (+ `IsccIdResult` record) over the already-generated
`iscc_gen_iscc_id_v1` P/Invoke decl, taking the C# (.NET) surface from 32→33 Tier 1 symbols. This is
the 10th of 11 IDv1 fan-out surfaces (#43); only C++ then remains.

## Alternatives Considered

- **Chosen:** C# `GenIsccIdV1` — the P/Invoke decl already exists (`NativeMethods.g.cs:293`, no FFI
    regen needed) and `dotnet test` is locally verifiable (dotnet 8.0.423 on PATH), so it is the
    lowest-risk of the two remaining minting surfaces.
- **Rejected:** C++ `iscc.hpp` wrapper — equally needed but a separate step (different tech, needs
    `uv run --with cmake cmake`); pick it up next iteration.

## Scope

- **Modify**:
    - `packages/dotnet/Iscc.Lib/Results.cs` — add `public sealed record IsccIdResult(string Iscc);`
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — add
        `GenIsccIdV1(ulong timestamp, ushort hubId, byte realm)`
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — golden + validation-error + round-trip tests
        (test file, off-budget)
- **Reference**: `crates/iscc-ffi/src/lib.rs:585-620` (FFI contract),
    `packages/dotnet/Iscc.Lib/IsccLib.cs:93-108` (`GenMetaCodeV0` pattern),
    `packages/swift/Tests/IsccLibTests/ConformanceTests.swift:228-235` (golden),
    `specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)"

## Not In Scope

- The deferred repo-wide Tier-1 32→33 doc/count sweep (`packages/dotnet/CLAUDE.md`/`README.md`
    counts, `docs/` API pages) — a distinct #43 step; do not touch doc counts here.
- The C++ (`iscc.hpp`) wrapper — separate step.
- Any change to `NativeMethods.g.cs` (P/Invoke decl already correct — do not regenerate) or the FFI
    crate.
- Adding a version enum / widening decode — C# `DecodeResult.Version` is a raw `byte`, so the
    generic decode path already accepts Version 1; no widening needed.

## Implementation Notes

- Mirror `GenMetaCodeV0`: call `NativeMethods.iscc_gen_iscc_id_v1(timestamp, hubId, realm)` in an
    `unsafe` block, wrap the `byte*` with `ConsumeNativeString(result)`, return
    `new IsccIdResult(...)`.
- The FFI takes exact-width unsigned types (`ulong`/`ushort`/`byte`), so out-of-width values are
    impossible at the call site — **no binding-side guard** (core re-validates ranges). Out-of-range
    input returns NULL from FFI; `ConsumeNativeString` already throws
    `IsccException(GetLastError())`. This surface is exempt from the open Ruby i64 marshalling gap.
- Golden: `GenIsccIdV1(1751831876325218, 1, 0).Iscc == "ISCC:MAIGHFECJMOPMIAB"`.
- Validation-error test: an out-of-range timestamp (e.g. `1UL << 52`) throws `IsccException`.
- Round-trip test: `IsccLib.IsccDecode(golden.Iscc)` returns `Maintype == 6`, `Version == 1`, an
    8-byte `Digest`; reconstruct `timestamp = (BE digest >> 12)`, `hubId = digest & 0xFFF`,
    `realm = Subtype` and assert they equal the inputs.
- XML doc-comment the method (`/// <summary>…</summary>`) as the surrounding methods do.

## Verification

- `dotnet test packages/dotnet/Iscc.Lib.Tests` passes (existing tests + new
    golden/error/round-trip).
- `IsccLib.GenIsccIdV1(1751831876325218, 1, 0).Iscc` equals `"ISCC:MAIGHFECJMOPMIAB"`.
- `IsccLib.GenIsccIdV1(1UL << 52, 0, 0)` throws `IsccException`.
- `grep -c "GenIsccIdV1" packages/dotnet/Iscc.Lib/IsccLib.cs` ≥ 1 and `IsccIdResult` present in
    `Results.cs`.
- `git diff --quiet -- packages/dotnet/Iscc.Lib/NativeMethods.g.cs` (P/Invoke layer untouched).

## Done When

The C# surface mints IDv1 via `GenIsccIdV1` (33 usable Tier 1 symbols) and `dotnet test` passes with
the new golden, validation-error, and decode round-trip coverage.
