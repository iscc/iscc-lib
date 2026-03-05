## 2026-03-05 — Idiomatic C# wrappers — infrastructure, constants, text utilities, and simple gen functions

**Done:** Created `IsccException.cs` and rewrote `IsccLib.cs` with UTF-8 marshaling infrastructure
(ToNativeUtf8, ConsumeNativeString, GetLastError helpers) and 14 idiomatic PascalCase wrappers: 5
constants, 4 text utilities, 4 gen functions, and refactored ConformanceSelftest to delegate to
NativeMethods instead of a duplicate DllImport.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccException.cs`: Created — simple exception class for ISCC errors
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Replaced manual DllImport with NativeMethods delegation,
    added 3 private marshaling helpers and 14 public Tier 1 symbol wrappers
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Expanded from 1 test to 16 — covers constants,
    text utilities, gen functions, and exception type

**Verification:**

- `dotnet build Iscc.Lib.csproj` — 0 warnings, 0 errors ✓
- `dotnet build Iscc.Lib.Tests.csproj` — 0 warnings, 0 errors ✓
- `dotnet test` — 16 passed, 0 failed ✓
- All 14 public methods/properties present in IsccLib.cs ✓
- No DllImport in IsccLib.cs — all P/Invoke through NativeMethods ✓
- `cargo clippy -p iscc-ffi -- -D warnings` — clean ✓
- `mise run check` — all 15 hooks pass ✓

**Next:** Add remaining Tier 1 symbols — result record types (MetaCodeResult, SumCodeResult, etc.)
for complex gen functions (GenVideoCodeV0, GenMixedCodeV0, GenIsccCodeV0, GenSumCodeV0), codec
functions (IsccDecode, IsccDecompose, EncodeComponent), algorithm primitives, and encoding
utilities. The marshaling infrastructure established here makes these additions mechanical.

**Notes:**

- `META_TRIM_META` is 128,000 (not 16,384 as I initially assumed from memory — verified against
    `crates/iscc-lib/src/lib.rs`)
- `dotnet` was not pre-installed in the devcontainer but was available at `/home/dev/.dotnet` after
    running the install script. CI uses `actions/setup-dotnet@v4` so this doesn't affect CI
- Empty string passed to `GenTextCodeV0` does NOT error at the FFI level — the Rust function handles
    it gracefully (returns a valid ISCC). Error handling via IsccException is functional but tested
    via the exception class itself rather than provoking an FFI error
- The `fixed (byte* p = nullArray)` pattern in C# correctly sets the pointer to null when the array
    is null — used for optional parameters in GenMetaCodeV0 (description, meta)
