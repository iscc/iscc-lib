## 2026-03-07 — Review of: C# streaming hasher Finalize() return types

**Verdict:** PASS

**Summary:** Clean, minimal, exactly-scoped change. Both streaming hashers now return typed result
records (`DataCodeResult` / `InstanceCodeResult`) instead of raw `string`, completing the structured
return type pattern across the entire .NET API surface. All 91 tests pass, all quality gates green.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — compiled in 1.01s
- [x] `dotnet build packages/dotnet/Iscc.Lib/` succeeds — 0 warnings, 0 errors
- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests/` — 91 passed, 0 failed, 0 skipped
- [x] `grep -c 'DataCodeResult Finalize' IsccDataHasher.cs` → 1
- [x] `grep -c 'InstanceCodeResult Finalize' IsccInstanceHasher.cs` → 1
- [x] `grep -c 'string Finalize' IsccDataHasher.cs IsccInstanceHasher.cs` → 0:0 (none remain)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** No defects found. Codex confirmed the change is consistent and the public API
change is intentional. No actionable findings.

**Next:** .NET bindings are now feature-complete for the current API surface. Priority order for
next steps: (1) Documentation — `docs/howto/dotnet.md`, `packages/dotnet/README.md`, and README C#
section; (2) NuGet publish pipeline in `release.yml`; (3) Version sync integration for .NET project
version. Documentation is the natural next step since it's self-contained and the API is now stable.

**Notes:** The 9 pre-existing xUnit1026 warnings (unused `vectorName` parameter in conformance test
Theory methods) remain harmless — the parameter provides test identification in output. The .NET
binding now has complete typed returns for all 9 gen functions + 2 streaming hashers, with 91 tests
covering conformance and smoke scenarios.
