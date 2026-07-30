"""Differential conformance tests for `gen_iscc_id_v1` against iscc-core.

The ISCC-IDv1 (experimental) packs a 52-bit microsecond timestamp and a 12-bit
HUB-ID into a 64-bit ISCC-ID unit, encoded with the realm as SubType. These
tests pin the exact output bytes against the `iscc_core` reference over an
explicit grid of inputs, plus a golden value and the validation contract.

Timestamps are always explicit — passing ``timestamp=None`` would make the
reference read the system clock, which is not deterministic.
"""

import iscc_core
import pytest

from iscc_lib import MT, VS, IsccIdResult, IsccResult, gen_iscc_id_v1, iscc_decode

TIMESTAMPS = [0, 1, 1751831876325218, (1 << 52) - 1]
HUB_IDS = [0, 1, 4095]
REALMS = [0, 1]


def test_gen_iscc_id_v1_matches_reference():
    """Verify every input combination matches iscc-core byte-for-byte."""
    for ts in TIMESTAMPS:
        for hub in HUB_IDS:
            for realm in REALMS:
                expected = iscc_core.gen_iscc_id_v1(ts, hub, realm)["iscc"]
                assert gen_iscc_id_v1(ts, hub, realm)["iscc"] == expected, (
                    f"divergence at ts={ts} hub={hub} realm={realm}"
                )


def test_gen_iscc_id_v1_golden():
    """Verify the pinned golden value."""
    assert gen_iscc_id_v1(1751831876325218, 1, 0)["iscc"] == "ISCC:MAIGHFECJMOPMIAB"


def test_gen_iscc_id_v1_result_type():
    """Verify the result is an IsccIdResult with dict and attribute access."""
    result = gen_iscc_id_v1(1751831876325218, 1, 0)
    assert isinstance(result, IsccIdResult)
    assert isinstance(result, IsccResult)
    assert isinstance(result, dict)
    assert result["iscc"] == "ISCC:MAIGHFECJMOPMIAB"
    assert result.iscc == "ISCC:MAIGHFECJMOPMIAB"


def test_gen_iscc_id_v1_defaults():
    """Verify hub_id and realm_id default to 0, matching the reference."""
    assert gen_iscc_id_v1(0)["iscc"] == iscc_core.gen_iscc_id_v1(0, 0, 0)["iscc"]


def test_gen_iscc_id_v1_rejects_invalid_realm():
    """Verify an out-of-range realm raises ValueError."""
    with pytest.raises(ValueError):
        gen_iscc_id_v1(0, 0, 2)


def test_gen_iscc_id_v1_rejects_timestamp_overflow():
    """Verify a timestamp at or beyond 2**52 raises ValueError."""
    with pytest.raises(ValueError):
        gen_iscc_id_v1(1 << 52, 0, 0)


def test_gen_iscc_id_v1_rejects_hub_overflow():
    """Verify a hub_id at or beyond 2**12 raises ValueError."""
    with pytest.raises(ValueError):
        gen_iscc_id_v1(0, 4096, 0)


def test_gen_iscc_id_v1_validation_order_arbitrary_precision():
    """Verify the first failing check wins even beyond fixed-width ranges.

    Values exceeding u64/u16/u8 must not short-circuit into OverflowError
    during argument conversion; the normative timestamp -> hub_id -> realm
    order applies to any int magnitude, matching the reference.
    """
    with pytest.raises(ValueError, match="Timestamp overflow"):
        gen_iscc_id_v1(1 << 52, 1 << 100, 2)
    with pytest.raises(ValueError, match="Timestamp overflow"):
        gen_iscc_id_v1(1 << 70, 0, 0)
    with pytest.raises(ValueError, match="HUB-ID overflow"):
        gen_iscc_id_v1(0, 1 << 100, 2)
    with pytest.raises(ValueError, match="Realm-ID"):
        gen_iscc_id_v1(0, 0, 1 << 100)


def test_gen_iscc_id_v1_rejects_non_int():
    """Verify non-int arguments keep raising TypeError."""
    with pytest.raises(TypeError):
        gen_iscc_id_v1("not-an-int", 0, 0)  # ty: ignore[invalid-argument-type]


def test_gen_iscc_id_v1_rejects_explicit_none():
    """Verify explicit None raises instead of silently defaulting to 0.

    The reference rejects None for every parameter (it never mints an ID),
    and an earlier failing check still wins over a later None, so a caller
    with missing application data cannot accidentally produce a hub/realm-0
    identifier.
    """
    with pytest.raises(TypeError):
        gen_iscc_id_v1(None, 0, 0)  # ty: ignore[invalid-argument-type]
    with pytest.raises(TypeError):
        gen_iscc_id_v1(0, None, 0)  # ty: ignore[invalid-argument-type]
    with pytest.raises(TypeError):
        gen_iscc_id_v1(0, 0, None)  # ty: ignore[invalid-argument-type]
    with pytest.raises(ValueError, match="Timestamp overflow"):
        gen_iscc_id_v1(1 << 52, None, 0)  # ty: ignore[invalid-argument-type]


def test_iscc_decode_roundtrips_iscc_id_v1():
    """Verify iscc_decode accepts a minted ISCC-IDv1 and reports MT.ID / VS.V1."""
    iscc = gen_iscc_id_v1(1751831876325218, 1, 0)["iscc"]
    mt, _st, vs, _length, _digest = iscc_decode(iscc)
    assert mt == MT.ID
    assert vs == VS.V1
    assert vs == 1
