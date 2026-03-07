## 2026-03-07 — Review of: C# streaming hashers (IsccDataHasher, IsccInstanceHasher) — 32/32 symbols

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation of the final 2 Tier 1 symbols for the C# binding.
Both `IsccDataHasher` and `IsccInstanceHasher` use the SafeHandle + IDisposable pattern as
specified, with correct disposed/finalized checks, proper pointer casting, and error propagation.
All 41 .NET tests pass (35 existing + 6 new streaming tests), `mise run check` and clippy are clean.
Scope discipline is perfect — only the 4 specified files touched.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — 0 errors
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- [x] `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 41 passed, 0 failed
    (35 existing + 6 new streaming tests)
- [x] `grep -c 'public sealed class Iscc' IsccDataHasher.cs` — 1
- [x] `grep -c 'public sealed class Iscc' IsccInstanceHasher.cs` — 1
- [x] `mise run check` — all 15 hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** P1 finding about `DangerousGetHandle()` without `DangerousAddRef`/
`DangerousRelease` in the streaming hashers. Analysis: the concern is about a race between
`Dispose()` and `Update()`/`Finalize()` from concurrent threads. However: (1) the hasher classes are
not thread-safe by design (no locking on `_finalized`), (2) `this` reference keeps the object alive
during method execution — GC can't finalize the SafeHandle, (3) all other bindings (Ruby, Python,
Go) use equivalent single-threaded streaming patterns. The csbindgen-generated P/Invoke signatures
use typed pointers (`FfiDataHasher*`), not `SafeHandle`, so the marshaler can't handle lifetime
automatically. Adding `DangerousAddRef`/`DangerousRelease` would be defense-in-depth for a scenario
the class doesn't support. Not actionable for this iteration.

**Next:** C# conformance tests (`ConformanceTests.cs` + vendored `testdata/data.json`) to validate
all gen functions against official test vectors. The .NET binding issue in issues.md still has
remaining items: conformance tests, NuGet publish job, version sync, howto docs.

**Notes:**

- .NET binding now at 32/32 Tier 1 symbols: 5 constants, 4 text utils, 10 gen, 2 encoding utils, 3
    codec, 1 sliding window, 4 algorithm primitives, 1 conformance, 2 streaming types
- `GetLastError` and `ConsumeNativeString` changed from `private` to `internal` — minimal visibility
    change to support the hasher classes in the same assembly
- No documentation updates needed for this iteration (streaming classes are new, not changing
    existing documented behavior)
