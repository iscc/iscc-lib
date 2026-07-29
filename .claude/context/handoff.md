## 2026-07-28 — Review of: Add `gen_iscc_id_v1` to the Python binding + differential test

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent exposed the core's `gen_iscc_id_v1` on the Python surface and pinned
it to the `iscc_core` reference with a pytest differential test. Additive-only, in scope, all gates
green; independently confirmed byte-for-byte against the reference oracle (realm nibble, golden,
three validation raises). One carried-forward note: minting works but the generic decode path does
not yet round-trip an IDv1 (tracked under #43, explicitly out of scope here).

**Verification:**

- [x] `maturin develop` builds — OK.
- [x] `pytest tests/test_iscc_id_v1.py` — 7 passed (differential grid + golden + 3 validations).
- [x] Golden smoke `gen_iscc_id_v1(1751831876325218,1,0)['iscc']=='ISCC:MAIGHFECJMOPMIAB'` — exit 0.
- [x] Full `pytest` — 451 passed, no regression.
- [x] `ruff check` + `ruff format --check` (crates/iscc-py tests) — clean.
- [x] `uv run ty check` — clean (new `.pyi` stub type-checks).
- [x] `cargo clippy -p iscc-py --all-targets -- -D warnings` — clean.
- [x] `mise run check` — all prek pre-commit hooks Passed; no context files reformatted.
- [x] **Probe (independent oracle):** isolated `iscc_core` 1.3.0 vs installed binding —
    `(0,0,0)→MAI…`, `(0,0,1)→MEI…`, golden, and realm=2 / hub=4096 / ts=2^52 all raise `ValueError`
    on both sides.

**Issues found:**

- (none in the delivered work — scope, correctness and simplicity all clean.)

**Codex review:** [P1] flagged that `iscc_decode(gen_iscc_id_v1(...))` raises `1 is not a valid VS`
— the Python `VS` IntEnum (`__init__.py:82`) still lists only `V0`. This is the already-tracked #43
generic-decode gap and is **explicitly out of scope** for this minting-only step; the advance agent
correctly left it. Verified the underlying fact shifted: the core `codec::Version` now has `V1`
(`#[non_exhaustive]`), so the low-level `_iscc_decode` returns `(6,0,1,0,<8 bytes>)` — the remaining
blocker is per-surface enum wrappers, not the core. Corrected #43's stale "core raises
`invalid Version: 1`" text accordingly.

**Next:** Continue the #43 fan-out. Two independent tracks now visible: (a) **minting** —
`gen_iscc_id_v1` on the next surface (FFI unblocks dotnet/cpp but needs `iscc.h` regen + freshness
gate; napi/wasm/jni/rb/uniffi are lighter templates); (b) **decode round-trip** — widen each
surface's own version enum (Python `VS`, etc.) to accept `V1` and add an
`iscc_decode(gen_iscc_id_v1(...))` test. The Tier-1 32→33 doc/count sweep and the Go
`EncodeIsccID`→`gen_iscc_id_v1` rename + `DecodeIsccID` deletion remain separate steps.

**Notes:** CI is GREEN on develop (the state.md "Semver RED" is a phantom — the `Semver` job is
`continue-on-error: true`). Wrapper param is `realm_id` (mirrors the reference public name) → core's
`realm: u8`; tests always pass `timestamp` explicitly (a `None` would make the reference read the
clock). No decision record: API shape was spec-mandated and matches the reference byte-for-byte.
