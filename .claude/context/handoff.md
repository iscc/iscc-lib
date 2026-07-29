## 2026-07-29 — Widen Python `VS` IntEnum to accept ISCC-IDv1 decode

**Done:** Added `V1 = 1` to the Python `VS` IntEnum so `iscc_lib.iscc_decode(gen_iscc_id_v1(...))`
round-trips instead of raising `1 is not a valid VS`. This closes the functional half of #43's
decode-widening — Python was the last surface rejecting V1 decode.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: added `V1 = 1` under `V0 = 0` in the `VS` IntEnum.
    No other production edit — `iscc_decode` already calls `VS(vs)`.
- `tests/test_iscc_id_v1.py`: added `test_iscc_decode_roundtrips_iscc_id_v1` (mints
    `gen_iscc_id_v1(1751831876325218, 1, 0)`, decodes, asserts `MT.ID` and `VS.V1`/`== 1`); widened
    the import to include `MT`, `VS`, `iscc_decode`.
- `tests/test_new_symbols.py`: extended `test_vs_values` to assert `VS.V1 == 1` (kept `VS.V0 == 0`).

**Verification:**

- Rebuilt the extension (`maturin develop -m crates/iscc-py/Cargo.toml`) — no Rust source change,
    `.so` binary logically unchanged; rebuild only to load the updated wrapper.
- `pytest tests/test_iscc_id_v1.py tests/test_new_symbols.py` → **47 passed**.
- `ruff check crates/iscc-py tests` → All checks passed; `ruff format --check` → clean.
- `mise run check` → all prek hooks Passed.

**Next:** The remaining #43 item (v0.6.0 blocker) is the repo-wide **Tier-1 32→33 doc/count sweep**
— stale `32` symbol counts across docs plus `gen_iscc_id_v1` API-doc entries. It is mechanical and
non-functional; a good dedicated sweep step. With this change, all 11 surfaces both mint and decode
ISCC-IDv1.

**Notes:** Pure Python wrapper fix, no native/Rust change (did not touch `src/lib.rs` or
`_lowlevel.pyi`, per Not-In-Scope). Only Python has a version *enum*; the other 10 surfaces return a
bare int/byte version and already decoded V1 — none touched. `ST`/realm already covers 0-7 so realm
0/1 decode was never the blocker.
