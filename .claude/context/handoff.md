## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the C# (.NET) surface

**Verdict:** PASS

**Summary:** The C# surface now mints IDv1 via idiomatic `GenIsccIdV1(ulong, ushort, byte)` +
`IsccIdResult` record over the already-generated P/Invoke decl, advancing dotnet from 32→33 Tier 1
symbols (10th of 11 IDv1 fan-out surfaces, #43). Clean, faithful to next.md, no scope creep, no gate
weakening. Purely additive: 2 non-test source files + 1 test file.

**Verification:**

- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests` (LD_LIBRARY_PATH=target/debug) — **107 passed, 0
    failed** (+3 new: golden, out-of-range-ts throws, decode round-trip).
- [x] `GenIsccIdV1(1751831876325218, 1, 0).Iscc == "ISCC:MAIGHFECJMOPMIAB"` — matches `iscc_core`
    1.3.0 oracle byte-for-byte (cross-checked, `--python 3.13`).
- [x] `GenIsccIdV1(1UL << 52, 0, 0)` throws `IsccException` — covered by passing test.
- [x] `grep -c GenIsccIdV1 IsccLib.cs` = 1; `IsccIdResult` present in `Results.cs`.
- [x] `git diff --quiet -- NativeMethods.g.cs` clean after `cargo build -p iscc-ffi` (P/Invoke layer
    untouched).
- [x] `mise run check` — all prek hygiene + parity + lint hooks Passed; working tree clean after
    (only runner-owned `iterations.jsonl` dirty).
- [x] (probe) Scope + circumvention: `@{upstream}..HEAD` scan of all 4 unpushed commits clean — no
    suppressions, no test skips, no threshold/hook weakening. No API break, no hot path.
- [x] (probe) Validation order: dotnet has no binding guard, delegates entirely to core; core
    reports ts→hub→realm first-failure (confirmed against reference for `(2^52, 5000, 0)` →
    "Timestamp overflow"). A no-guard passthrough can't reorder — correct pattern, unlike jni's
    earlier partial guard.

**Issues found:**

- (none) — exact-width unsigned args (`ulong`/`ushort`/`byte`) mean callers can't pass out-of-width
    values, so dotnet is genuinely exempt from the open Ruby i64 marshalling gap. No enum-widening
    needed (`DecodeResult.Version` is a raw `byte`). No new issue warranted.

**Codex review:** Clean — no findings. Confirms correct delegation to the P/Invoke decl, native
error handling via the established helper, correct result type, and 107 tests + warning-as-error
release build pass.

**Next:** #43 fan-out has **one minting surface left**: C++ (`packages/cpp/include/iscc.hpp`) over
the same fixed-width FFI (needs `uv run --with cmake cmake` to verify). After it lands, do the
deferred repo-wide Tier-1 32→33 doc/count sweep (issues.md `#43` first bullet enumerates every stale
site, `gen_iscc_id_v1` API-doc entries, `packages/dotnet`/other `CLAUDE.md`+`README.md` counts). #43
stays open (v0.6.0 blocker).

**Notes:** dotnet is a pure passthrough (`unsafe` call + `ConsumeNativeString`, which throws on
NULL), so it inherits core's ordered validation and needs no binding-side guard — matches the
uniffi/ffi exact-width pattern. Doc counts left untouched per Not-In-Scope. Updated the #43 issue's
minting enumeration to reflect cpp as the only remaining minting surface.
