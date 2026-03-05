## 2026-03-05 — Review of: C# wrappers — remaining gen functions + encoding utilities

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation adding 8 public methods and the SumCodeResult record
type. Four new marshaling patterns (int[], int[][] jagged array, string[], struct return+free) are
correctly implemented with proper GCHandle pinning and finally-block cleanup. All 10 gen functions
are now callable from C#. 25/25 tests pass, builds are clean, hooks pass.

**Verification:**

- [x] `cargo build -p iscc-ffi` — succeeds (no Rust changes)
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- [x] `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — succeeds
- [x] `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 25 passed, 0 failed
- [x] `grep` for gen function signatures — returns 10 (all 10 gen functions wrapped)
- [x] `grep` for SumCodeResult record — returns 1
- [x] `mise run check` — all 15 hooks pass

**Issues found:**

- Minor: `GenMixedCodeV0` and `GenIsccCodeV0` have near-identical string array marshaling code (~25
    lines each). C# language limitation (no pointer types in generics) prevents extraction to a
    shared helper. Acceptable technical debt — documented in learnings.

**Next:** Codec functions (`IsccDecode`, `IsccDecompose`, `EncodeComponent`) with `DecodeResult`
record type and string-array return marshaling, or algorithm primitives (`AlgSimhash`,
`AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`) with `IsccByteBuffer` struct marshaling. The
jagged array and struct return patterns from this iteration are reusable foundations.

**Notes:**

- The `IsccSumCodeResult` struct is at namespace level in `NativeMethods.g.cs` (not nested in the
    `NativeMethods` class), so it is referenced without the `NativeMethods.` prefix
- 22 of 32 Tier 1 symbols are now wrapped in C# (5 constants, 4 text utilities, 10 gen functions, 2
    encoding utilities, 1 conformance selftest). Remaining: 3 codec functions, 4 algorithm
    primitives, 1 sliding window, 2 streaming types
