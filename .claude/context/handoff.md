## 2026-03-07 — Review of: C# conformance tests (ConformanceTests.cs + vendored data.json)

**Verdict:** PASS

**Summary:** Clean, well-structured implementation of 50 xUnit conformance tests across 9 gen
functions. All tests pass (91 total: 41 smoke + 50 conformance). The advance agent also fixed a real
bug in `IsccLib.cs` where `fixed` on empty `ReadOnlySpan<T>` produces NULL pointers — this was
technically out of scope but was a necessary bug fix to pass 3 conformance vectors. The fix is
minimal, well-commented, and doesn't change any API signatures.

**Verification:**

- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=...` — 91 passed (41 smoke +
    50 conformance), 0 failed
- [x] `grep -c '\[Theory\]' ConformanceTests.cs` → 9 (one per gen function)
- [x] `diff data.json` — no differences (identical copy, 579 lines each)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `mise run check` — all 15 hooks pass

**Issues found:**

- (none) — the out-of-scope `IsccLib.cs` modification is a legitimate bug fix (empty spans → NULL
    pointers via `fixed`), not an API change. Pragmatic and well-justified.

**Codex review:** No correctness issues identified. The empty-span fix is consistent with the native
FFI contract that rejects NULL pointers.

**Next:** Continue C# binding completion. Priority order: (1) Structured result record types
(`Results.cs` — `MetaCodeResult`, `TextCodeResult`, etc.) so gen functions return records instead of
`string`; (2) NuGet publish job in `release.yml`; (3) Version sync integration; (4) Documentation
(`docs/howto/dotnet.md`, `packages/dotnet/README.md`, README C# section). Note: `GenImageCodeV0`,
`AlgMinhash256`, `AlgCdcChunks`, and `EncodeBase64` have the same latent empty-span NULL pointer bug
— can be fixed as part of the structured records refactor.

**Notes:**

- The `JsonDocument` in `LoadDataJson()` is not explicitly disposed (returned `RootElement` outlives
    parent). This is acceptable in test code — the `Lazy<>` keeps the document alive for the process
    lifetime and GC handles cleanup.
- The `dotnet test` command requires the project path, not the solution directory
    (`packages/dotnet/Iscc.Lib.Tests/` not `packages/dotnet/`).
