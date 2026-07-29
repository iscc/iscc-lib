## 2026-07-29 — Review of: Widen Python `VS` IntEnum to accept ISCC-IDv1 decode

**Verdict:** PASS

**Summary:** Added `V1 = 1` to the Python `VS` IntEnum so
`iscc_lib.iscc_decode(gen_iscc_id_v1(...))` round-trips instead of raising `1 is not a valid VS`.
One-line production edit plus two tests; faithful to next.md, no scope creep, no gate weakening.
This closes the functional half of #43 — all 11 surfaces now both mint and decode ISCC-IDv1.

**Verification:**

- [x] `iscc_decode(gen_iscc_id_v1(1751831876325218, 1, 0)["iscc"])` returns without raising, version
    element equals `VS.V1` — confirmed directly (`decode: 6 1`) and by the new round-trip test.
- [x] `pytest tests/test_iscc_id_v1.py tests/test_new_symbols.py` → **47 passed** (after
    `maturin develop`).
- [x] `VS.V1 == 1` and `VS.V0 == 0` both hold — confirmed directly and in `test_vs_values`.
- [x] `ruff check crates/iscc-py tests` → All checks passed; `ruff format --check` → 25 files
    already formatted.
- [x] `mise run check` — all prek hooks Passed.
- [x] (probe) Gate-circumvention scan of unpushed commits — pure Python wrapper edit, no
    suppressions/skips/threshold or hook weakening, no native/Rust change (`src/lib.rs`,
    `_lowlevel.pyi` untouched per Not-In-Scope), no API break (additive enum member).

**Issues found:**

- (none) — Python was the only surface with a version *enum*; the other 10 return a bare int/byte
    and already decoded V1, so no fan-out remains for decode.

**Codex review:** Clean — confirms the V1 enum value is what was needed and the regression coverage
is focused; the targeted suite passes (47 tests). No findings.

**Next:** The remaining #43 item (v0.6.0 blocker) is the repo-wide **Tier-1 32→33 doc/count sweep**
\+ `gen_iscc_id_v1` per-symbol API-doc entries (see the updated #43 issue). Mechanical and
non-functional; a good dedicated sweep step. Watch the four-list parity check in
`scripts/check_docs_nav.py` if a new docs page is added.

**Notes:** Rebuilding the extension only reloads the updated wrapper — the `.so` binary is unchanged
(no Rust source touched). The #43 issue has been rewritten to describe only the remaining doc/count
problem; its minting/decode fan-out is complete.
