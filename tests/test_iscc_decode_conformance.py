"""Differential conformance tests for `iscc_decode` against the iscc-core reference.

`iscc_decode` normalizes its input before decoding (decompose into ISCC-UNITs,
recompose into an ISCC-CODE when two or more are present), mirroring
`iscc_core.codec.iscc_decode`. That normalization is what makes a concatenated
unit sequence and its composite form decode identically, and it is only
observable end-to-end — the Rust unit tests pin structure, these pin the exact
bytes against the reference.

Not covered here: truncated bodies. `iscc_core` returns a short digest without
complaining (e.g. 1 byte where the header declares 8); iscc-lib rejects that as
`ValueError`, deliberately, since a silently short digest is a data-integrity
hazard. See `test_iscc_decode_rejects_truncated_body`.
"""

import io

import iscc_core
import pytest
from iscc_core.codec import iscc_decode as ref_iscc_decode

from iscc_lib import iscc_decode

PAYLOAD = b"x" * 2000


@pytest.fixture(scope="module")
def units():
    """Return a dict of reference-generated ISCC-UNIT and composite strings."""
    meta = iscc_core.gen_meta_code_v0("Hello")["iscc"]
    text = iscc_core.gen_text_code_v0("Hello World")["iscc"]
    data = iscc_core.gen_data_code_v0(io.BytesIO(PAYLOAD))["iscc"]
    instance = iscc_core.gen_instance_code_v0(io.BytesIO(PAYLOAD))["iscc"]
    data128 = iscc_core.gen_data_code_v0(io.BytesIO(PAYLOAD), bits=128)["iscc"]
    instance128 = iscc_core.gen_instance_code_v0(io.BytesIO(PAYLOAD), bits=128)["iscc"]
    return {
        "meta": meta,
        "text": text,
        "data": data,
        "instance": instance,
        "composite": iscc_core.gen_iscc_code_v0([meta, data, instance])["iscc"],
        "wide": iscc_core.gen_iscc_code_v0([data128, instance128], wide=True)["iscc"],
    }


def _bare(code):
    """Strip the ``ISCC:`` prefix from a code."""
    return code.removeprefix("ISCC:")


def _reference(code):
    """Return the reference decode as plain ints plus a hex digest."""
    mt, st, vs, ln, digest = ref_iscc_decode(code)
    return int(mt), int(st), int(vs), int(ln), bytes(digest).hex()


def _subject(code):
    """Return the iscc-lib decode as plain ints plus a hex digest."""
    mt, st, vs, ln, digest = iscc_decode(code)
    return int(mt), int(st), int(vs), int(ln), bytes(digest).hex()


def _case_ids(units):
    """Return (label, code) pairs covering every normalization shape."""
    m, t, d, i = units["meta"], units["text"], units["data"], units["instance"]
    return [
        ("single unit", m),
        ("single unit without prefix", _bare(m)),
        ("composite", units["composite"]),
        ("composite with trailing base32", units["composite"] + "AA"),
        ("wide composite", units["wide"]),
        ("sequence data+instance", _bare(d) + _bare(i)),
        ("sequence meta+data+instance", _bare(m) + _bare(d) + _bare(i)),
        ("uncomposable sequence meta+text", _bare(m) + _bare(t)),
        ("unit with trailing base32", m + "AAAA"),
    ]


def test_iscc_decode_matches_reference(units):
    """Verify every normalization shape decodes identically to iscc-core."""
    for label, code in _case_ids(units):
        try:
            expected = _reference(code)
        except ValueError:
            with pytest.raises(ValueError):
                iscc_decode(code)
            continue
        assert _subject(code) == expected, f"divergence on {label!r}: {code}"


def test_iscc_decode_normalizes_sequence_to_composite(units):
    """Verify a unit sequence and its composite decode to the same tuple."""
    sequence = _bare(units["data"]) + _bare(units["instance"])
    composite = iscc_core.gen_iscc_code_v0([units["data"], units["instance"]])["iscc"]
    assert _subject(sequence) == _subject(composite)
    # MainType is ISCC (5), not the leading Data unit's 3.
    assert _subject(sequence)[0] == 5


def test_iscc_decode_rejects_truncated_body(units):
    """Verify a truncated body is rejected, where the reference returns it short."""
    truncated = units["meta"][:10]
    mt, _st, _vs, ln, digest = ref_iscc_decode(truncated)
    assert int(mt) == 0
    # The reference hands back fewer digest bytes than the header declares.
    assert len(bytes(digest)) < 8 <= (int(ln) + 1) * 4
    with pytest.raises(ValueError, match="truncated ISCC body"):
        iscc_decode(truncated)
