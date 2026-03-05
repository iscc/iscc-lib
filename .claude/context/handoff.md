## 2026-03-05 — Review of: Idiomatic C# wrappers — infrastructure, constants, text utilities, and simple gen functions

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation that exactly matches the next.md specification. Added
`IsccException.cs`, rewrote `IsccLib.cs` with 3 private marshaling helpers and 14 idiomatic
PascalCase public wrappers (5 constants, 4 text utilities, 4 gen functions, 1 refactored
ConformanceSelftest), and expanded tests from 1 to 16. All verification criteria pass.

**Verification:**

- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds with 0 errors, 0 warnings
- [x] `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` succeeds
- [x] `dotnet test ... -e LD_LIBRARY_PATH=target/debug` — 16 passed, 0 failed (note: requires
    absolute path in devcontainer; CI uses `env:` on the test step which handles this)
- [x] `IsccLib.cs` contains all required public methods: `ConformanceSelftest`, `TextClean`,
    `TextRemoveNewlines`, `TextTrim`, `TextCollapse`, `GenMetaCodeV0`, `GenTextCodeV0`,
    `GenDataCodeV0`, `GenInstanceCodeV0`
- [x] `IsccLib.cs` contains all required public properties: `MetaTrimName`, `MetaTrimDescription`,
    `MetaTrimMeta`, `IoReadSize`, `TextNgramSize`
- [x] `IsccException.cs` exists in `packages/dotnet/Iscc.Lib/`
- [x] No `DllImport` declarations in `IsccLib.cs` — all P/Invoke through `NativeMethods` (grep
    returns 0 matches)
- [x] `cargo clippy -p iscc-ffi -- -D warnings` — clean (no Rust changes)
- [x] `mise run check` — all 15 hooks pass

**Issues found:**

- (none)

**Codex review:** No correctness, security, or maintainability bugs found. Marshaling patterns
(UTF-8 conversion, `iscc_free_string` for owned strings, `iscc_last_error` left unfreed) are all
correct.

**Next:** Add remaining Tier 1 symbols to complete the C# wrapper layer: result record types
(`MetaCodeResult`, `SumCodeResult`, `DecodeResult`), complex gen functions (`GenVideoCodeV0`,
`GenMixedCodeV0`, `GenIsccCodeV0`, `GenSumCodeV0`), codec functions (`IsccDecode`, `IsccDecompose`,
`EncodeComponent`), algorithm primitives, encoding utilities, and streaming types (`IsccDataHasher`,
`IsccInstanceHasher` with `IDisposable`). The marshaling infrastructure established in this
iteration makes these additions mechanical.

**Notes:**

- `dotnet test` with relative `LD_LIBRARY_PATH=target/debug` fails in devcontainer — must use
    absolute path `/workspace/iscc-lib/target/debug`. CI is unaffected (uses `env:` on the step)
- The `ToNativeUtf8` + `ConsumeNativeString` + `GetLastError` helpers are a clean, reusable
    foundation for all remaining string-returning wrappers
- Constants are expression-bodied properties (`=>`) calling NativeMethods — correct for values that
    could theoretically change across library versions (no hardcoded values in C#)
