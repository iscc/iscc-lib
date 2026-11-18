# Next Work Package

## Step: Python `encode_seq` / `decode_seq` binding with an `iscc_core` differential test

## Goal

Expose ISCC-SEQ (IEP-0020, #45) in the Python binding, the first of the 11 surfaces. The test also
pins the strict-base32 divergence from `iscc_core` 1.5.0 that decisions.md 2026-11-18 requires on
every surface.

## Alternatives Considered

- **Chosen:** Python seq binding. It is the first item of the v0.8.0 fan-out, it can be verified
    locally against the live reference (`iscc_core` 1.5.0 is in the dev group), and it is a
    binding-only step, so neither iai nor CRAP is touched.
- **Rejected:** Go `decodeBase32` spare-bit fix plus Rust/Go strict tests (`normal` issue). It only
    gates Go seq, as does the Go `isccClean` port, and Go comes later in the fan-out. Python has no
    such prerequisite. It remains the strongest next candidate.

## Scope

- **Create**: `tests/test_seq.py`
- **Modify**: `crates/iscc-py/src/lib.rs`, `crates/iscc-py/python/iscc_lib/__init__.py`,
    `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`
- **Reference**: `crates/iscc-lib/src/codec.rs` (`encode_seq` L808, `decode_seq` L847),
    `reference/iscc-core/iscc_core/codec.py` (L470-534, contract + docstrings), `iscc_decode` in
    `lib.rs` (~L508) + `__init__.py` (L103) as the wrapper pattern, `tests/test_conformance.py`
    (`DATA_JSON` + loader pattern)

## Not In Scope

- The Tier-1 33→35 count/doc sweep (crate `CLAUDE.md`, `notes/`, `docs/architecture.md`, binding
    docs). It runs once after all 11 surfaces land (IDv1 precedent).
- Rust strict-base32 rejection tests and the Go `decodeBase32` fix. Those belong to the Go issue
    step.
- `py.detach` for seq. These are lightweight paths, so they stay attached like meta/audio/mixed.
- The stale "return JSON strings" intro in `docs/api.md`. The page renders `::: iscc_lib` from the
    docstrings, so good docstrings are the whole doc change.
- Exposing Tier 2 `read_unit` / `SEQ_MAINTYPES` in Python. They are Rust-only.

## Implementation Notes

- **Rust side (`lib.rs`):** `encode_seq(units: Vec<String>)` must return `PyBytes`. A plain
    `Vec<u8>` return becomes `list[int]` in PyO3. The `iscc_decode` digest uses `PyBytes::new` and
    is the precedent. Convert to `Vec<&str>` for the core call.
    `decode_seq(data: &[u8]) ->   Vec<String>`. Map `IsccError` to `PyValueError` as every other
    function does. Register both in the module.
- **Wrapper (`__init__.py`)** carries the reference's type contract, which PyO3 extraction alone
    does not give:
    - `encode_seq`: raise `TypeError` for a bare `str` or `bytes`, else `list(units)`, so tuples and
        generators work.
    - `decode_seq`: accept `bytes`, `bytearray` and `memoryview` (`bytes(data)`). Raise `TypeError`
        otherwise. `&[u8]` accepts only `bytes`.
    - Add both to `__all__` and the `.pyi` stubs. Adapt the docstrings from `codec.py` L470-534, but
        say that base32 decoding is strict: `=` padding and non-zero spare bits are rejected.
- **Measured facts** (probed against `iscc_core` 1.5.0 and the installed `iscc_lib`):
    - `ic.encode_seq(["ISCC:GAAAAAAAAB"])` and `ic.encode_seq(["ISCC:GAAAAAAAAA======"])` both return
        `300000000000`. The same goes for `GAAAAAAAAA` and `iscc:gaaa-aaaaaa`.
    - The 256-bit unit `ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI` with its last
        char `I`→`J` is accepted by the reference and gives the same bytes as the canonical unit.
        `iscc_lib.iscc_decode` rejects it ("non-zero trailing bits at 54").
    - **Trap:** 64-bit units are exactly 16 chars and have no spare bits, so they have no alias. Use
        32- or 256-bit units for spare-bit cases. `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` is an
        ISCC-CODE, which the reference also rejects (for MainType), so it is not a seq divergence
        case.
- **Tests (`tests/test_seq.py`):**
    - Run the `encode_seq` and `decode_seq` sections of `crates/iscc-lib/tests/data.json`,
        parametrized from the fixture (no hard-coded counts). Strip `bytes:` and use `bytes.fromhex`.
    - Round trip: `encode_seq(decode_seq(b)) == b`.
    - Matching forms: no prefix, lowercase, hyphens, tuple input, `bytearray`/`memoryview` input.
    - Error arms: empty list and empty unit → `ValueError`; ISCC-CODE unit → `ValueError`; bare `str`
        and `bytes` → `TypeError`; `decode_seq(b"")` → `ValueError`; `decode_seq("abc")` →
        `TypeError`.
    - **Differential, one bad input per test case:** for each of the 32-bit spare-bit alias, the
        256-bit `…J` alias and the `======` padded form, assert BOTH that `iscc_core.encode_seq`
        accepts it and returns the canonical unit's bytes, AND that `iscc_lib.encode_seq` raises
        `ValueError`. Also add a differential that agrees on the canonical vectors (both libs give
        identical bytes and units).

## Verification

- `maturin develop -m crates/iscc-py/Cargo.toml` succeeds. Do this first: the installed `.so`
    predates the seq symbols.
- `uv run pytest tests/test_seq.py` passes, and the full `uv run pytest` passes.
- `uv run python -c "import iscc_lib as i; b=i.encode_seq(['ISCC:GAAAAAAAAA']); assert isinstance(b, bytes) and i.decode_seq(bytearray(b))==['ISCC:GAAAAAAAAA']"`
    exits 0
- `uv run python -c "import iscc_lib as i; assert {'encode_seq','decode_seq'} <= set(i.__all__)"`
    exits 0
- `uv run ty check` exits 0
- `mise run check` exits 0, and `cargo clippy -p iscc-py -- -D warnings` is clean.

## Done When

All verification criteria pass, and the Python binding exposes `encode_seq`/`decode_seq` with
fixture conformance and an explicit strict-base32 differential against `iscc_core` 1.5.0.
