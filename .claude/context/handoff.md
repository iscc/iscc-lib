## 2026-07-29 — Mint `gen_iscc_id_v1` on the C++ surface

**Done:** Added `iscc::gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` returning `IsccIdResult` to the
header-only C++ wrapper, delegating to the already-shipped `iscc_gen_iscc_id_v1` FFI symbol.
Advances cpp 32→33 Tier 1 symbols — the 11th and final minting surface of the #43 IDv1 fan-out.

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: added `struct IsccIdResult { std::string iscc; }` (after
    `IsccCodeResult`) + `gen_iscc_id_v1` wrapper in the Gen-functions section (mirrors
    `gen_meta_code_v0`: `UniqueString` + `check_ptr` + return `IsccIdResult`; no default args, no
    binding-side guard).
- `packages/cpp/tests/test_iscc.cpp`: added tests 36 (golden `MAIGHFECJMOPMIAB`), 37 (realm=2 throws
    `IsccError`), 38 (decode round-trip `.version == 1`); renumbered Unicode section to 39.

**Verification:**

- `cargo build -p iscc-ffi` then
    `git diff --quiet -- crates/iscc-ffi/include/iscc.h   packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    → CLEAN (no FFI/P-Invoke change, as expected).
- `uv run --with cmake cmake -B packages/cpp/build -DFFI_LIB_DIR=target/debug packages/cpp` +
    `cmake --build` succeed; `LD_LIBRARY_PATH=target/debug ./packages/cpp/build/tests/test_iscc` →
    **72 passed, 0 failed**, exit 0 (build showed benign "Clock skew detected" warnings only).
- `grep -c gen_iscc_id_v1 …/iscc.hpp` = 3; `struct IsccIdResult` present.
- `mise run check` → all prek hooks Passed (hygiene, formatting, release/docs/CI-table/Ruby parity).

**Next:** #43 fan-out minting is now complete on all 11 surfaces. Remaining #43 work is the deferred
repo-wide Tier-1 **32→33 doc/count sweep** (stale `32` counts in CLAUDE.md/README files,
`gen_iscc_id_v1` API-doc entries per issues.md first bullet). #43 stays open (v0.6.0 blocker) until
that sweep lands.

**Notes:** Pure additive change, no scope creep — count text and cpp README left untouched per
Not-In-Scope. Exact-width unsigned args (`uint64_t`/`uint16_t`/`uint8_t`) mean callers cannot pass
out-of-width values, so no `checked()`/range guard is needed; core re-validates in ts→hub→realm
order and a no-guard passthrough cannot reorder (matches ffi/dotnet pattern). `DecodeResult.version`
is a raw `uint8_t`, so the round-trip needed no enum widening.
