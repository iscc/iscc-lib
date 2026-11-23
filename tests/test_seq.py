"""Tests for ISCC-SEQ (IEP-0020) `encode_seq` / `decode_seq`, including differentials against iscc_core.

Divergences from iscc_core (strict base32, ASCII-only casing) live in
`crates/iscc-lib/tests/data-edge.json` and are checked by `tests/test_data_edge.py`.
"""

import contextlib
import json
import unicodedata
from pathlib import Path

import iscc_core
import pytest

import iscc_lib

DATA_JSON = Path(__file__).parent.parent / "crates" / "iscc-lib" / "tests" / "data.json"

UNIT_32 = "ISCC:GAAAAAAAAA"
UNIT_32_HEX = "300000000000"
UNIT_256 = "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI"
UNIT_256_HEX = "2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305"
ISCC_CODE = "ISCC:KUAALNA7GRZESMRFUURK6SRMTS7WU"


def load_vectors(function_name):
    """Load conformance vectors for a given function from data.json."""
    data = json.loads(DATA_JSON.read_text(encoding="utf-8"))
    return [pytest.param(tc, id=name) for name, tc in data[function_name].items()]


def from_hex(value):
    """Decode a `bytes:<hex>` fixture value to bytes."""
    assert value.startswith("bytes:")
    return bytes.fromhex(value.removeprefix("bytes:"))


ENCODE_VECTORS = load_vectors("encode_seq")
DECODE_VECTORS = load_vectors("decode_seq")


def test_fixture_sections_not_empty():
    """Both seq sections carry vectors, so parametrized tests cannot silently vanish."""
    assert ENCODE_VECTORS
    assert DECODE_VECTORS


# ── Conformance ──────────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", ENCODE_VECTORS)
def test_encode_seq_conformance(tc):
    """encode_seq matches the data.json vector bytes."""
    result = iscc_lib.encode_seq(tc["inputs"][0])
    assert isinstance(result, bytes)
    assert result == from_hex(tc["outputs"])


@pytest.mark.parametrize("tc", DECODE_VECTORS)
def test_decode_seq_conformance(tc):
    """decode_seq matches the data.json vector unit strings."""
    assert iscc_lib.decode_seq(from_hex(tc["inputs"][0])) == tc["outputs"]


@pytest.mark.parametrize("tc", DECODE_VECTORS)
def test_seq_round_trip(tc):
    """encode_seq(decode_seq(b)) reproduces b."""
    data = from_hex(tc["inputs"][0])
    assert iscc_lib.encode_seq(iscc_lib.decode_seq(data)) == data


# ── Matching input forms ─────────────────────────────────────────────────────


@pytest.mark.parametrize(
    "unit",
    ["GAAAAAAAAA", "iscc:gaaaaaaaaa", "iscc:gaaa-aaaaaa", "ISCC:GAAA-AAAAAA"],
)
def test_encode_seq_unit_forms(unit):
    """Prefix-less, lowercase and hyphenated units encode like the canonical unit."""
    assert iscc_lib.encode_seq([unit]) == bytes.fromhex(UNIT_32_HEX)


def test_encode_seq_accepts_tuple_and_generator():
    """Any non-string sequence or iterable of units is accepted."""
    expected = bytes.fromhex(UNIT_32_HEX + UNIT_256_HEX)
    assert iscc_lib.encode_seq((UNIT_32, UNIT_256)) == expected
    assert iscc_lib.encode_seq(u for u in [UNIT_32, UNIT_256]) == expected


def test_encode_seq_preserves_order_and_duplicates():
    """Units are neither sorted nor deduplicated."""
    expected = bytes.fromhex(UNIT_256_HEX + UNIT_32_HEX + UNIT_32_HEX)
    assert iscc_lib.encode_seq([UNIT_256, UNIT_32, UNIT_32]) == expected


@pytest.mark.parametrize(
    "wrap", [bytes, bytearray, memoryview], ids=lambda f: f.__name__
)
def test_decode_seq_bytes_like(wrap):
    """decode_seq accepts bytes, bytearray and memoryview."""
    data = wrap(bytes.fromhex(UNIT_32_HEX + UNIT_256_HEX))
    assert iscc_lib.decode_seq(data) == [UNIT_32, UNIT_256]


# ── Error arms (one bad input per test case) ─────────────────────────────────


def test_encode_seq_empty_list():
    """An empty unit list raises ValueError."""
    with pytest.raises(ValueError, match="at least one"):
        iscc_lib.encode_seq([])


@pytest.mark.parametrize("unit", ["", "   "])
def test_encode_seq_empty_unit(unit):
    """An empty or blank unit raises ValueError."""
    with pytest.raises(ValueError, match="must not be empty"):
        iscc_lib.encode_seq([unit])


def test_encode_seq_iscc_code_unit():
    """An ISCC-CODE (MainType 5) is not permitted in an ISCC-SEQ."""
    with pytest.raises(ValueError, match="MainType 5"):
        iscc_lib.encode_seq([ISCC_CODE])


def test_encode_seq_trailing_data():
    """Two units concatenated in one string raise ValueError."""
    with pytest.raises(ValueError, match="Trailing data"):
        iscc_lib.encode_seq(["ISCC:GAAAAAAAAAAAAAAAAAAA"])


@pytest.mark.parametrize("units", [UNIT_32, UNIT_32.encode()], ids=["str", "bytes"])
def test_encode_seq_bare_string_type_error(units):
    """A bare str or bytes is rejected instead of being iterated."""
    with pytest.raises(TypeError):
        iscc_lib.encode_seq(units)


@pytest.mark.parametrize("units", [[1]], ids=["int_unit"])
def test_encode_seq_non_string_unit_type_error(units):
    """A non-string unit raises TypeError."""
    with pytest.raises(TypeError):
        iscc_lib.encode_seq(units)


def test_decode_seq_empty():
    """Empty data raises ValueError."""
    with pytest.raises(ValueError, match="Empty ISCC-SEQ"):
        iscc_lib.decode_seq(b"")


@pytest.mark.parametrize("data", ["abc"], ids=["str"])
def test_decode_seq_str_type_error(data):
    """A str is not a bytes-like ISCC-SEQ."""
    with pytest.raises(TypeError):
        iscc_lib.decode_seq(data)


def test_decode_seq_truncated():
    """A unit cut short of its declared length raises ValueError."""
    with pytest.raises(ValueError):
        iscc_lib.decode_seq(bytes.fromhex(UNIT_32_HEX)[:-1])


# ── Differential against iscc_core 1.5.0 ─────────────────────────────────────


@pytest.mark.parametrize("tc", ENCODE_VECTORS)
def test_differential_encode_agrees_on_vectors(tc):
    """iscc_lib and iscc_core produce identical bytes for canonical units."""
    units = tc["inputs"][0]
    assert iscc_lib.encode_seq(units) == iscc_core.encode_seq(units)


@pytest.mark.parametrize("tc", DECODE_VECTORS)
def test_differential_decode_agrees_on_vectors(tc):
    """iscc_lib and iscc_core produce identical units for canonical sequences."""
    data = from_hex(tc["inputs"][0])
    assert iscc_lib.decode_seq(data) == iscc_core.decode_seq(data)


# Inputs both libraries reject with the same exception class (rust-core.md "Verified when").
SHARED_ENCODE_REJECTIONS = [
    pytest.param([], ValueError, id="empty_list"),
    pytest.param([""], ValueError, id="empty_unit"),
    pytest.param(["   "], ValueError, id="blank_unit"),
    pytest.param([ISCC_CODE], ValueError, id="iscc_code_unit"),
    pytest.param(["ISCC:MAIAAAAAAAAAAAAA"], ValueError, id="iscc_id_unit"),
    pytest.param(["ISCC:GAAAAAAAAAAAAAAAAAAA"], ValueError, id="trailing_data"),
    pytest.param(["foo:GAAAAAAAAA"], ValueError, id="invalid_scheme"),
    pytest.param(["ISCC:GAAA:AAAA"], ValueError, id="two_colons"),
    pytest.param([1], TypeError, id="int_unit"),
    pytest.param(UNIT_32, TypeError, id="bare_str"),
    pytest.param(UNIT_32.encode(), TypeError, id="bare_bytes"),
]

# IEP-0020 Table 2 rejection vectors plus a non-bytes input.
SHARED_DECODE_REJECTIONS = [
    pytest.param(b"", ValueError, id="empty"),
    pytest.param(bytes.fromhex("28000100000000"), ValueError, id="noncanonical_header"),
    pytest.param(bytes.fromhex("200800") + bytes(36), ValueError, id="length_above_7"),
    pytest.param(bytes.fromhex("3000000000"), ValueError, id="truncated"),
    pytest.param(bytes.fromhex("30000000000000"), ValueError, id="trailing_byte"),
    pytest.param(
        bytes.fromhex(
            "5005578c7c8fbfbdb58fa88248c53a852c8d05b41f3472493225a522af4a2c9cbf6a"
        ),
        ValueError,
        id="iscc_code",
    ),
    pytest.param(bytes.fromhex("60100000000000000000"), ValueError, id="maintype_6"),
    pytest.param("abc", TypeError, id="str"),
]


@pytest.mark.parametrize(("units", "exc"), SHARED_ENCODE_REJECTIONS)
def test_differential_encode_rejects_same_inputs(units, exc):
    """iscc_core and iscc_lib both reject a malformed encode_seq input with the same class."""
    with pytest.raises(exc):
        iscc_core.encode_seq(units)
    with pytest.raises(exc):
        iscc_lib.encode_seq(units)


@pytest.mark.parametrize(("data", "exc"), SHARED_DECODE_REJECTIONS)
def test_differential_decode_rejects_same_inputs(data, exc):
    """iscc_core and iscc_lib both reject a malformed decode_seq input with the same class."""
    with pytest.raises(exc):
        iscc_core.decode_seq(data)
    with pytest.raises(exc):
        iscc_lib.decode_seq(data)


# ── Whitespace trimming parity ───────────────────────────────────────────────

# Every code point Python's `str.strip()` removes, derived live from the interpreter.
PY_STRIP_CHARS = [c for c in map(chr, range(0x110000)) if c.strip() == ""]
INFO_SEPARATORS = ["\x1c", "\x1d", "\x1e", "\x1f"]


def test_strip_set_contains_info_separators():
    """The live-derived strip set includes U+001C..U+001F, which Rust's `trim` keeps."""
    assert set(INFO_SEPARATORS) <= set(PY_STRIP_CHARS)


def test_differential_encode_strips_python_whitespace():
    """Both libraries strip every `str.strip()` char around a unit to identical bytes."""
    mismatches = []
    for ch in PY_STRIP_CHARS:
        unit = ch + UNIT_32 + ch
        expected = iscc_core.encode_seq([unit])
        try:
            actual = iscc_lib.encode_seq([unit])
        except ValueError as e:
            actual = e
        if actual != expected:
            mismatches.append((hex(ord(ch)), actual))
    assert not mismatches


def test_differential_rejects_space_like_chars_python_keeps():
    """Space-like chars outside the `str.strip()` set are kept, so both libraries reject them.

    Covers the reverse direction of the strip-set parity: a char that Rust's `char::is_whitespace`
    trims but Python keeps would be accepted here. Candidates are derived live (categories `Z*`,
    `Cc`, `Cf` minus the strip set), which contain every Unicode `White_Space` code point.
    """
    strip = set(PY_STRIP_CHARS)
    candidates = [
        c
        for c in map(chr, range(0x110000))
        if unicodedata.category(c) in ("Zs", "Zl", "Zp", "Cc", "Cf") and c not in strip
    ]
    assert "\u200b" in candidates
    accepted = []
    for ch in candidates:
        for lib in (iscc_core, iscc_lib):
            with contextlib.suppress(ValueError):
                lib.encode_seq([ch + UNIT_32])
                accepted.append((lib.__name__, hex(ord(ch))))
    assert not accepted


@pytest.mark.parametrize("sep", INFO_SEPARATORS, ids=lambda s: f"U+{ord(s):04X}")
def test_differential_encode_strips_separator_around_colon(sep):
    """Separators before and after the scheme colon are stripped by both libraries."""
    for unit in (f"ISCC{sep}:GAAAAAAAAA", f"ISCC:{sep}GAAAAAAAAA"):
        assert iscc_lib.encode_seq([unit]) == iscc_core.encode_seq([unit])


@pytest.mark.parametrize("sep", INFO_SEPARATORS, ids=lambda s: f"U+{ord(s):04X}")
def test_differential_separator_only_unit_rejected(sep):
    """A unit consisting only of a separator is rejected by both libraries."""
    with pytest.raises(ValueError):
        iscc_core.encode_seq([sep])
    with pytest.raises(ValueError, match="must not be empty"):
        iscc_lib.encode_seq([sep])


def test_differential_zero_width_space_rejected():
    """U+200B is not stripped by `str.strip()`, so both libraries reject it."""
    assert "\u200b".strip() == "\u200b"
    with pytest.raises(ValueError):
        iscc_core.encode_seq(["\u200b" + UNIT_32])
    with pytest.raises(ValueError):
        iscc_lib.encode_seq(["\u200b" + UNIT_32])
