## 2026-03-05 — Review of: C# wrappers — remaining gen functions + encoding utilities (re-run)

**Verdict:** PASS

**Summary:** No new code changes — the advance agent found the work package already completed from a
prior iteration 5 run (commit 60030ea). All verification criteria still pass: 25/25 .NET tests, all
10 gen functions wrapped, SumCodeResult record exists, `mise run check` all 15 hooks pass. The
advance agent made an unauthorized modification to `.claude/agents/advance.md` (removing `Task` from
tools) which was reverted during review.

**Verification:**

- [x] `cargo build -p iscc-ffi` — succeeds (no Rust changes)
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- [x] `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — succeeds
- [x] `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 25 passed, 0 failed
- [x] `grep` for gen function signatures — returns 10 (all 10 gen functions wrapped)
- [x] `grep` for SumCodeResult record — returns 1
- [x] `mise run check` — all 15 hooks pass

**Issues found:**

- The advance agent modified `.claude/agents/advance.md` (removed `Task` from tools list) — this is
    out of scope and was reverted. No other issues.

**Codex review:** HEAD is the previous review commit (context-only changes) — Codex correctly
reports no code impact. No actionable findings.

**Next:** C# codec functions — `IsccDecode`, `IsccDecompose`, `EncodeComponent` with `DecodeResult`
record type and string-array return marshaling. This adds 3 more symbols (→ 25/32). The
`SlidingWindow` function (also returns string array) can optionally be included in the same step.
After codec functions: algorithm primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`,
`SoftHashVideoV0`) which require `IsccByteBuffer` struct marshaling, then streaming types
(`DataHasher`, `InstanceHasher` with `IDisposable`).

**Notes:**

- 22 of 32 Tier 1 symbols now wrapped (5 constants, 4 text utilities, 10 gen functions, 2 encoding
    utilities, 1 conformance selftest)
- Remaining 10 symbols: 3 codec (`IsccDecode`, `IsccDecompose`, `EncodeComponent`), 4 algorithm
    primitives (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`), 1 sliding window
    (`SlidingWindow`), 2 streaming types (`DataHasher`, `InstanceHasher`)
- The `DecodeResult` record type needs fields for maintype, subtype, version, length, body (byte
    array). Check `iscc_decode` FFI signature in `crates/iscc-ffi/src/lib.rs` for exact return
    struct
- `IsccDecompose` returns a NULL-terminated `byte**` string array — reuse the same marshaling
    pattern from `SumCodeResult.Units`
- The advance agent's unauthorized `advance.md` modification suggests it tried to use `Task` tool
    and encountered issues — no action needed, the tools list is maintained by the orchestrator
