## 2026-03-07 — Review of: C# structured result records (Results.cs + gen function return type refactor)

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation. Created `Results.cs` with 11 sealed record types,
refactored all 9 gen functions to return typed records instead of `string`, fixed 4 remaining
empty-span NULL pointer bugs, and updated all tests. All 91 tests pass. No quality gate
circumvention, no scope overreach.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — compiled in 1.13s
- [x] `dotnet build packages/dotnet/Iscc.Lib/` succeeds — 0 warnings, 0 errors
- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests/` — 91 passed (41 smoke + 50 conformance), 0
    failed, 0 skipped
- [x] `grep -c 'sealed record' Results.cs` → 11 (9 new + 2 relocated)
- [x] `grep -c 'sealed record' IsccLib.cs` → 0 (all moved to Results.cs)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Next:** Continue C# binding completion. Priority order: (1) Streaming hasher `Finalize()` return
types — change `IsccDataHasher.Finalize()` and `IsccInstanceHasher.Finalize()` to return
`DataCodeResult`/`InstanceCodeResult` instead of `string`; (2) Documentation
(`docs/howto/dotnet.md`, `packages/dotnet/README.md`, README C# section); (3) NuGet publish pipeline
in `release.yml`; (4) Version sync integration for .NET project version. The streaming hasher change
is small (2 files, 2 return type changes + 2 test updates) and should be paired with another item
for efficiency.

**Notes:**

- The 9 xUnit1026 warnings (unused `vectorName` parameter in Theory methods) are pre-existing from
    the conformance tests — `vectorName` provides test identification in output but xUnit flags it.
    Not worth suppressing since the parameter serves a purpose (test discoverability).
- state.md still lists the 4 empty-span functions as having the latent bug — the update-state agent
    will correct this in the next cycle.
- All 9 gen-function records currently only have `Iscc` field. Additional fields (Name, Characters,
    DataHash, etc.) will be populated when FFI structured result support is added — this is
    correctly documented as out of scope.
