## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the C++ surface

**Verdict:** PASS

**Summary:** The header-only C++ wrapper now mints IDv1 via idiomatic
`iscc::gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` + `IsccIdResult`, delegating to the
already-shipped `iscc_gen_iscc_id_v1` FFI symbol. This is the 11th and final minting surface of the
#43 fan-out. Purely additive (1 header + 1 test file), faithful to next.md, no scope creep, no gate
weakening.

**Verification:**

- [x] `cargo build -p iscc-ffi` then `git diff --quiet` on `iscc.h` + `NativeMethods.g.cs` → CLEAN —
    no FFI/P-Invoke change, as expected (symbol shipped iter 182).
- [x] cmake configure + build succeed (only benign "Clock skew detected" warnings).
- [x] `LD_LIBRARY_PATH=target/debug ./…/test_iscc` → **72 passed, 0 failed**, exit 0.
- [x] `grep -c gen_iscc_id_v1 …/iscc.hpp` = 3; `struct IsccIdResult` present (1 match).
- [x] `mise run check` — all prek hooks Passed (hygiene, formatting, release/docs/CI-table/Ruby
    parity). Working tree clean after (only runner-owned `iterations.jsonl` dirty).
- [x] (probe) Golden oracle: `iscc_core` 1.3.0 (`--python 3.13`)
    `gen_iscc_id_v1(1751831876325218,   1, 0)` → `ISCC:MAIGHFECJMOPMIAB`, byte-for-byte matches the
    C++ golden test.
- [x] (probe) Gate circumvention: `@{upstream}..HEAD` scan of all 4 unpushed commits — source
    changes are cpp-only, no suppressions/skips/threshold or hook weakening, no API break.

**Issues found:**

- (none) — exact-width unsigned args (`uint64_t`/`uint16_t`/`uint8_t`) mean callers can't pass
    out-of-width values, so no binding guard is needed; core re-checks in ts→hub→realm order and a
    no-guard passthrough can't reorder (matches ffi/dotnet). `DecodeResult.version` is a raw
    `uint8_t`, so no enum-widening. No new issue warranted.

**Codex review:** Clean — no findings. Confirms the wrapper matches the FFI signature and the
established RAII/error-handling patterns; a clean ASAN-enabled build passed with all 72 assertions.

**Next:** #43 minting fan-out is now complete on all 11 surfaces. Two #43 items remain (v0.6.0
blocker stays open): (1) the **decode enum-widening** — surfaces with their own version enum still
reject V1 (Python `VS` IntEnum defines only `V0`, so `iscc_lib.iscc_decode(gen_iscc_id_v1(...))`
raises `1 is not a valid VS`; each such surface needs the widening + a round-trip test); (2) the
repo-wide **Tier-1 32→33 doc/count sweep** (stale `32` counts + `gen_iscc_id_v1` API-doc entries per
issues.md #43). Either is a good next step; the enum-widening is the more functional gap.

**Notes:** cpp is a pure passthrough over a fixed-width FFI, so it inherits core's ordered
validation with no binding-side guard. Count text and cpp README left untouched per Not-In-Scope.
Updated the #43 issue's minting bullet to record cpp as done (all 11 surfaces mint).
