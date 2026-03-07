## 2026-03-07 — C# structured result records (Results.cs + gen function return type refactor)

**Done:** Created `Results.cs` with 11 sealed record types (9 gen-function results + relocated
`SumCodeResult` and `DecodeResult`). Refactored all 9 gen functions in `IsccLib.cs` to return typed
result records instead of raw strings. Fixed 4 remaining empty-span NULL pointer bugs in
`GenImageCodeV0`, `AlgMinhash256`, `AlgCdcChunks`, and `EncodeBase64`.

**Files changed:**

- `packages/dotnet/Iscc.Lib/Results.cs`: Created — 11 sealed record types in the `Iscc.Lib`
    namespace
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Removed `SumCodeResult` and `DecodeResult` definitions
    (moved to Results.cs); changed 9 gen function return types from `string` to typed records; added
    empty-span guards to 4 functions
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Updated 14 tests to use `.Iscc` accessor on gen
    function results, including streaming hasher comparison tests and codec tests that chain gen
    calls
- `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs`: Updated all 9 Theory tests to use `.Iscc`
    accessor

**Verification:** All 91 tests pass (41 smoke + 50 conformance). `grep -c 'sealed record'` returns
11 for Results.cs and 0 for IsccLib.cs. `cargo build -p iscc-ffi` succeeds. `dotnet build` succeeds.
`mise run check` passes all 15 hooks.

**Next:** Streaming hasher `Finalize()` return types — change `IsccDataHasher.Finalize()` and
`IsccInstanceHasher.Finalize()` to return `DataCodeResult`/`InstanceCodeResult` instead of `string`.
Then NuGet publish pipeline in release.yml, version sync integration, and documentation
(docs/howto/dotnet.md, packages/dotnet/README.md, README C# section).

**Notes:** The streaming hashers still return `string` from `Finalize()` — intentionally deferred
per next.md scope (would exceed 3-file limit). The 9 new gen-function records only have `Iscc` field
since the C FFI only returns the ISCC string; additional fields (Name, Characters, DataHash, etc.)
will be populated when FFI structured result support is added.
