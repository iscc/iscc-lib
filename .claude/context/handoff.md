## 2026-03-07 — C# streaming hashers (IsccDataHasher, IsccInstanceHasher) — 32/32 symbols

**Done:** Implemented `IsccDataHasher` and `IsccInstanceHasher` streaming classes using SafeHandle +
IDisposable pattern for deterministic native resource cleanup. Changed `GetLastError` and
`ConsumeNativeString` visibility from `private` to `internal` in `IsccLib.cs` so the hasher classes
can reuse them. This completes 32/32 Tier 1 symbols for the C#/.NET binding.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccDataHasher.cs`: New file — streaming Data-Code hasher with nested
    `DataHasherHandle : SafeHandle`, `Update(ReadOnlySpan<byte>)`, `Finalize(uint bits = 64)`,
    `IDisposable`
- `packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs`: New file — streaming Instance-Code hasher with
    nested `InstanceHasherHandle : SafeHandle`, same API pattern as DataHasher
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Changed `ConsumeNativeString` and `GetLastError` from
    `private` to `internal` (one-word change each, no behavioral modification)
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Added 6 streaming tests

**Verification:**

- `cargo build -p iscc-ffi` — succeeds (no FFI changes)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 41 passed, 0 failed (35
    existing + 6 new streaming tests)
- `grep -c 'public sealed class Iscc' IsccDataHasher.cs` — 1
- `grep -c 'public sealed class Iscc' IsccInstanceHasher.cs` — 1
- `mise run check` — all 15 hooks pass

**Next:** Conformance tests (`ConformanceTests.cs` + vendored `testdata/data.json`) to validate all
10 gen functions against official test vectors. After that: howto documentation
(`docs/howto/dotnet.md`), API reference page, and NuGet publish job in `release.yml`.

**Notes:**

- .NET binding now at 32/32 Tier 1 symbols: 5 constants, 4 text utils, 10 gen, 2 encoding utils, 3
    codec, 1 sliding window, 4 algorithm primitives, 1 conformance, 2 streaming types
- Both hashers use `ObjectDisposedException.ThrowIf` for disposed checks and
    `InvalidOperationException` for finalized checks — consistent with .NET conventions
- The `_finalized` flag is checked before the disposed check in `Update`/`Finalize` — order is
    disposed-first because `ObjectDisposedException.ThrowIf` is checked first in code flow, matching
    the behavior specified in next.md
