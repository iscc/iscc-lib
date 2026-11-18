## 2026-11-18 — Bind `EncodeSeq` / `DecodeSeq` in the .NET wrapper

**Done:** `IsccLib` now exposes `public static byte[] EncodeSeq(string[] units)` and
`public static string[] DecodeSeq(ReadOnlySpan<byte> data)`. Both sit right after `EncodeComponent`
and wrap the existing `NativeMethods.iscc_encode_seq` / `iscc_decode_seq` through
`ConsumeByteBuffer` / `ConsumeNativeStringArray`. The .NET conformance harness runs both data.json
seq sections plus the BYTE round trip, and a new `SeqTests.cs` covers the negatives. ISCC-SEQ now
covers 6 of 11 surfaces.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccLib.cs`: adds `EncodeSeq` (pin loop copied from `GenIsccCodeV0`) and
    `DecodeSeq`. `EncodeSeq` throws `ArgumentNullException` for a null array or element. When
    `count == 0` it passes a non-null `byte*` stack sentinel with `num_units = 0`, so the core
    message "at least one ISCC-UNIT" surfaces. `DecodeSeq` uses plain `fixed`
- `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs`: adds the `DecodeBytes` and `ToStringArray`
    helpers and three theories: `EncodeSeq` and `DecodeSeq` (one per section), plus
    `SeqRoundTripBytes`, which asserts `EncodeSeq(DecodeSeq(x)) == x` over the `decode_seq` vectors.
    The header comment and class summary are reworded evergreen, with no hard-coded total
- `packages/dotnet/Iscc.Lib.Tests/SeqTests.cs` (new file): 10 facts. Seven negatives, one bad input
    each, assert a message substring or exception type: empty → "at least one ISCC-UNIT",
    `ISCC:NOT-A-VALID-UNIT!` → "base32 decode error", the ISCC-CODE → "not permitted in an
    ISCC-SEQ", `DecodeSeq([])` → "Empty ISCC-SEQ", vector bytes minus the last byte → "Truncated", a
    null array and a null element → `ArgumentNullException`. Three positives: a lower-case
    prefix-free unit gives the same bytes as its `ISCC:` form, a single-unit round trip, and the
    four-unit vector decodes to 4 units

**Verification:**

- `cargo build -p iscc-ffi && dotnet restore … --locked-mode && dotnet build … --no-restore`: Build
    succeeded, 0 Warning(s), 0 Error(s)
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ --no-restore -e LD_LIBRARY_PATH=$PWD/target/debug`:
    exit 0, Total 116, Passed 116, 0 failed. The output lists `ConformanceTests.EncodeSeq` ×4,
    `DecodeSeq` ×4, `SeqRoundTripBytes` ×4 and the 10 `SeqTests` facts. Breakdown from the log: 50
    `ConformanceTests.Gen*` rows (all nine gen theories intact) + 44 `SmokeTests` facts + 22 seq
    tests (12 theory rows + 10 facts) = 116
- `git status --porcelain packages/dotnet/Iscc.Lib/NativeMethods.g.cs crates/iscc-ffi`: empty after
    the build
- `mise run check`: exit 0, all hooks Passed

**Next:** C++ `iscc.hpp` seq, which needs RAII ownership of the returned `IsccByteBuffer` and the
string array, and must include the BYTE round trip. After that come JNI, Ruby and UniFFI, then the
33→35 doc sweep. That sweep includes `packages/dotnet/CLAUDE.md`, which still says "all 9 gen
functions (50 vectors total)", and `packages/dotnet/README.md` / `docs/howto/dotnet.md`. The same
CLAUDE.md's File Layout and Test Patterns sections also do not list `SeqTests.cs` yet.

**Notes:** binding-only step. No core, FFI or hot-path code changed, so iai, CRAP and the generated
files are unaffected. The empty-array sentinel is a `byte* sentinel = null` local whose address is
passed as `byte**`. The FFI only calls `from_raw_parts(units, 0)`, which needs a non-null, aligned
pointer and never dereferences it. `GenIsccCodeV0` / `GenMixedCodeV0` keep their NULL-on-empty
shape, and the pin loop stays duplicated (three copies now), as next.md asked.
