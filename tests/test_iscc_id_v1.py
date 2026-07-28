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

from iscc_lib import IsccIdResult, IsccResult, gen_iscc_id_v1

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
