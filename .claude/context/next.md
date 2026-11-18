# Next Work Package

## Step: Bind `EncodeSeq` / `DecodeSeq` in the .NET wrapper

## Goal

Expose ISCC-SEQ (`encode_seq` / `decode_seq`, IEP-0020, GitHub #45) as idiomatic C# methods on
`IsccLib` over the existing `NativeMethods.iscc_encode_seq` / `iscc_decode_seq` P/Invoke
declarations, and run the seq vectors in the .NET conformance harness. That brings ISCC-SEQ to 6 of
11 surfaces.

## Alternatives Considered

- **Chosen:** .NET seq. The FFI symbols and generated P/Invoke declarations landed in 205, so this
    is a single wrapper file plus tests. It continues the v0.8.0 feature set in the order the
    handoff gives.
- **Rejected:** bundling C++ `iscc.hpp` seq into the same step. The C++ harness needs its own design
    (RAII ownership of the returned buffer and string array), and it would push the criteria past 6.
    C++ is next. The small `normal` issues (py docstring, wasm metadata, Ruby json/locale, uv.lock)
    stay queued behind the coherent seq fan-out.

## Scope

- **Modify**: `packages/dotnet/Iscc.Lib/IsccLib.cs`
- **Modify (tests)**: `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs`, plus seq unit tests in
    `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` (or a new `SeqTests.cs`)
- **Reference**: `.claude/context/specs/rust-core.md` → "encode_seq / decode_seq (ISCC-SEQ,
    IEP-0020)"; `IsccLib.cs` `GenIsccCodeV0` (pin loop), `AlgMinhash256` (empty sentinel),
    `ConsumeByteBuffer`, `ConsumeNativeStringArray`; `crates/iscc-ffi/src/lib.rs` `iscc_encode_seq`
    (L1075), `iscc_decode_seq` (L1126), and its seq tests (~L2075–2220) for error substrings

## Not In Scope

- Any `iscc-ffi` edit. `NativeMethods.g.cs` must not change (it is generated, and already declares
    both symbols)
- Fixing the same NULL-on-empty shape in `GenIsccCodeV0` / `GenMixedCodeV0`, and extracting a shared
    pin-loop helper. Copy the loop into `EncodeSeq` and leave the two gen call sites untouched
- The 33→35 doc sweep: `packages/dotnet/README.md`, `packages/dotnet/CLAUDE.md`,
    `docs/howto/dotnet.md`, `docs/` API pages. This waits until all 11 surfaces bind seq
- C++, JNI, Ruby or UniFFI seq

## Implementation Notes

- Signatures: `public static byte[] EncodeSeq(string[] units)` and
    `public static string[] DecodeSeq(ReadOnlySpan<byte> data)`, placed after `EncodeComponent` in
    the codec block, each with an XML `<summary>`. Errors surface as `IsccException` through the
    existing `Consume*` helpers. Throw `ArgumentNullException` for a null `units` array or a null
    element, because `ToNativeUtf8(null)` returns null and must never reach `GCHandle.Alloc`.
- **Empty-list trap (EncodeSeq):** `fixed (byte** p = ptrs)` on a zero-length array yields NULL, so
    the FFI would report "units must not be NULL" instead of the core's "at least one ISCC-UNIT".
    For `count == 0`, pass a non-null stack sentinel (as `AlgMinhash256` does) with `num_units = 0`,
    so the core error surfaces.
- **No sentinel for DecodeSeq:** `iscc_decode_seq` checks `data_len == 0` before the NULL check, so
    an empty span already reaches the core's "Empty ISCC-SEQ" error. Plain `fixed` is enough.
- Ownership: wrap the `iscc_encode_seq` result in `ConsumeByteBuffer`, which frees it with
    `iscc_free_byte_buffer`. Wrap the `iscc_decode_seq` result in `ConsumeNativeStringArray`, which
    frees it with `iscc_free_string_array`.
- Harness shape: both data.json sections use `bytes:<hex>`, not `stream:`. The `encode_seq` case has
    `inputs[0]` = string array and `outputs` = a bare `"bytes:…"` string. The `decode_seq` case has
    `inputs[0]` = `"bytes:…"` and `outputs` = a string array. Neither `outputs` is an object, unlike
    the nine gen theories. Add a `DecodeBytes` helper next to `DecodeStream`, and add two `[Theory]`
    \+ `MemberData` pairs in the existing style.
- **Round trip is BYTES:** add a theory over the `decode_seq` vectors asserting
    `EncodeSeq(DecodeSeq(x))` equals `x` byte-for-byte. Decode-of-encode alone is not enough.
- Negatives, one bad input per test, each asserting its `IsccException.Message` contains:
    `EncodeSeq([])` → "at least one ISCC-UNIT"; `["ISCC:NOT-A-VALID-UNIT!"]` → "base32 decode
    error"; the ISCC-CODE `ISCC:KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY` → "not
    permitted in an ISCC-SEQ"; `DecodeSeq([])` → "Empty ISCC-SEQ"; a vector's bytes minus the last
    byte → "Truncated". Also add one null-element test (`ArgumentNullException`) and one lower-case,
    prefix-free unit that encodes the same bytes as its `ISCC:` form (FFI test L2138).
- The `ConformanceTests.cs` header comment and class summary say "all 9 gen functions … (50 total)".
    Reword them evergreen to cover the seq sections, with no hard-coded total.
- Binding-only step: no core or hot-path code, so the iai Perf and CRAP gates are unaffected.

## Verification

- `cargo build -p iscc-ffi && dotnet restore packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj --locked-mode && dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj --no-restore`
    exits 0 with 0 warnings
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ --no-restore -e LD_LIBRARY_PATH=$PWD/target/debug`
    exits 0, and the output lists the encode/decode seq theories and the byte round trip
- `IsccLib.cs` declares `public static byte[] EncodeSeq(string[] units)` and
    `public static string[] DecodeSeq(ReadOnlySpan<byte> data)`
- `EncodeSeq` with an empty array throws `IsccException` whose message contains "at least one
    ISCC-UNIT" (asserted by a test)
- `git status --porcelain packages/dotnet/Iscc.Lib/NativeMethods.g.cs crates/iscc-ffi` is empty
    after the build
- `mise run check` exits 0

## Done When

The advance agent is done when the .NET wrapper exposes `EncodeSeq` / `DecodeSeq`, the seq vectors
and the byte round trip pass in `dotnet test`, and all verification criteria pass.
