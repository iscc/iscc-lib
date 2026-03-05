## 2026-03-05 — C# wrappers for remaining gen functions + encoding utilities

**Done:** Added idiomatic C# wrappers for the 6 remaining gen functions (GenImageCodeV0,
GenAudioCodeV0, GenVideoCodeV0, GenMixedCodeV0, GenIsccCodeV0, GenSumCodeV0) plus 2 encoding
utilities (EncodeBase64, JsonToDataUrl). Introduced SumCodeResult record type and 4 new marshaling
patterns: int[] input, int[][] jagged array, string[] input, and struct return+free.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Added 8 public methods (GenImageCodeV0, GenAudioCodeV0,
    GenVideoCodeV0, GenMixedCodeV0, GenIsccCodeV0, GenSumCodeV0, EncodeBase64, JsonToDataUrl) and
    SumCodeResult record type. All 10 gen functions are now wrapped.
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Added 9 smoke tests for all 8 new symbols
    (GenSumCodeV0 has 2 tests: basic and with units). Total: 25 tests.

**Verification:** cargo build -p iscc-ffi OK, dotnet build 0 errors 0 warnings, 25/25 tests pass (16
existing + 9 new), grep confirms 10 gen functions and 1 SumCodeResult record, mise run check all
hooks pass.

**Next:** Codec functions (IsccDecode, IsccDecompose, EncodeComponent) with DecodeResult record type
and string-array return marshaling, or algorithm primitives (AlgSimhash, AlgMinhash256,
AlgCdcChunks, SoftHashVideoV0) with IsccByteBuffer struct marshaling.

**Notes:** The CallWithStringArray generic helper was attempted but C# does not allow pointer types
(byte\*\*) as generic type arguments, so the string array marshaling pattern is inlined in
GenMixedCodeV0 and GenIsccCodeV0. The IsccSumCodeResult struct is at the namespace level in
NativeMethods.g.cs (not nested inside the NativeMethods class), so it is referenced without the
NativeMethods prefix.
