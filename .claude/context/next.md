# Next Work Package

## Step: Widen Python `VS` IntEnum to accept ISCC-IDv1 decode

## Goal

Close the functional half of #43's decode-widening: `iscc_lib.iscc_decode(gen_iscc_id_v1(...))`
currently raises `1 is not a valid VS` because the Python `VS` IntEnum defines only `V0`. Adding
`V1 = 1` makes Python round-trip an ISCC-IDv1 — the last surface that rejects V1 decode.

## Alternatives Considered

- **Chosen:** Python `VS` widening — the more functional #43 gap, and a survey shows it is the
    *only* remaining rejecting surface (napi/wasm/ffi/jni/rb/uniffi/dotnet/cpp/Go all return a bare
    int/byte version and already decode V1; uniffi's test asserts `version == 1`, Go/ffi carry
    `VSV1`). One small change finishes the whole decode-widening item.
- **Rejected:** the repo-wide Tier-1 32→33 doc/count sweep (#43 part 2) — mechanical, spans many
    files/doc surfaces, and non-functional; better as its own dedicated sweep step after the
    functional gap is closed.

## Scope

- **Modify**: `crates/iscc-py/python/iscc_lib/__init__.py` — add `V1 = 1` to the `VS` IntEnum
    (currently lines 82-85, only `V0 = 0`).
- **Reference**: `crates/iscc-py/python/iscc_lib/__init__.py:102` (`iscc_decode` wraps `VS(vs)`);
    `tests/test_iscc_id_v1.py` (IDv1 test module, add the round-trip test here);
    `tests/test_new_symbols.py:277` (`test_vs_values`).

## Not In Scope

- The Tier-1 32→33 doc/count sweep and per-symbol API-doc entries (#43 part 2) — separate step.
- Any Rust/native change: the core decode already returns `version = 1`; this is a pure Python
    wrapper fix. Do not touch `crates/iscc-py/src/lib.rs` or `_lowlevel.pyi`.
- The other 10 surfaces — they already decode V1 (verified); do not "widen" their bare-int paths.
- The `ST`/realm SubType handling — `ST` already covers 0-7, so realm 0/1 decodes fine.

## Implementation Notes

- Add `V1 = 1` under `V0 = 0` in the `VS` IntEnum. No other production edit is needed —
    `iscc_decode` already calls `VS(vs)` and will now succeed for `vs == 1`.
- Add a round-trip test to `tests/test_iscc_id_v1.py`: mint with
    `gen_iscc_id_v1(1751831876325218, 1, 0)`, feed `["iscc"]` to `iscc_decode`, assert the returned
    version element is `VS.V1` (value `1`) and the MainType is `MT.ID` (6). Import `VS`, `MT`,
    `iscc_decode` from `iscc_lib`.
- Extend `test_vs_values` (or add a sibling) to assert `VS.V1 == 1`. `VS.V0 == 0` must still hold.
- Requires a fresh extension build (`maturin develop -m crates/iscc-py/Cargo.toml`) so pytest loads
    the installed wrapper — but no Rust source changes, so the `.so` binary itself is unchanged.

## Verification

- `iscc_lib.iscc_decode(iscc_lib.gen_iscc_id_v1(1751831876325218, 1, 0)["iscc"])` returns without
    raising and its version element equals `VS.V1` (asserted by the new round-trip test).
- `pytest tests/test_iscc_id_v1.py tests/test_new_symbols.py` passes (existing + new tests).
- `VS.V1 == 1` and `VS.V0 == 0` both hold.
- `ruff check crates/iscc-py tests` and `ruff format --check crates/iscc-py tests` clean.

## Done When

Python's `VS` IntEnum accepts `V1`, a round-trip test proves `iscc_decode(gen_iscc_id_v1(...))`
returns `VS.V1`/`MT.ID` without raising, and the pytest + ruff checks above pass.
