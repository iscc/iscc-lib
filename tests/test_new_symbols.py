"""Tests for the 7 new Tier 1 Python symbols: 3 functions + 4 constants."""

import pytest

from iscc_lib import (
    IO_READ_SIZE,
    META_TRIM_DESCRIPTION,
    META_TRIM_NAME,
    TEXT_NGRAM_SIZE,
    encode_component,
    iscc_decode,
    json_to_data_url,
)


def test_constants():
    """Verify algorithm configuration constants match iscc-core values."""
    assert META_TRIM_NAME == 128
    assert META_TRIM_DESCRIPTION == 4096
    assert IO_READ_SIZE == 4_194_304
    assert TEXT_NGRAM_SIZE == 13


def test_constants_are_int():
    """Verify constants are plain integers."""
    assert isinstance(META_TRIM_NAME, int)
    assert isinstance(META_TRIM_DESCRIPTION, int)
    assert isinstance(IO_READ_SIZE, int)
    assert isinstance(TEXT_NGRAM_SIZE, int)


def test_encode_component_roundtrip():
    """Encode a known digest and verify output is a valid ISCC string."""
    # mtype=0 (META), stype=0, version=0, 64 bits, 8-byte digest
    digest = b"\x00" * 8
    result = encode_component(0, 0, 0, 64, digest)
    assert isinstance(result, str)
    assert len(result) > 0
    # Verify it starts with a valid ISCC component prefix (base32 encoded)
    assert result.isalnum() or "-" in result


def test_iscc_decode_roundtrip():
    """Encode then decode, verify components match."""
    mtype, stype, version, bit_length = 0, 0, 0, 64
    digest = bytes(range(8))
    encoded = encode_component(mtype, stype, version, bit_length, digest)
    # Decode with ISCC: prefix
    mt, st, vs, li, decoded_digest = iscc_decode(f"ISCC:{encoded}")
    assert mt == mtype
    assert st == stype
    assert vs == version
    assert decoded_digest == digest


def test_iscc_decode_without_prefix():
    """Decode without ISCC: prefix works too."""
    digest = bytes(range(8))
    encoded = encode_component(0, 0, 0, 64, digest)
    mt, st, vs, li, decoded_digest = iscc_decode(encoded)
    assert mt == 0
    assert decoded_digest == digest


def test_iscc_decode_returns_bytes():
    """Verify iscc_decode returns bytes for the digest component."""
    digest = bytes(range(8))
    encoded = encode_component(0, 0, 0, 64, digest)
    result = iscc_decode(encoded)
    assert isinstance(result, tuple)
    assert len(result) == 5
    assert isinstance(result[4], bytes)


def test_json_to_data_url_plain():
    """JSON without @context returns data:application/json;base64,..."""
    result = json_to_data_url('{"key": "value"}')
    assert result.startswith("data:application/json;base64,")


def test_json_to_data_url_ld():
    """JSON with @context returns data:application/ld+json;base64,..."""
    result = json_to_data_url('{"@context": "https://schema.org"}')
    assert result.startswith("data:application/ld+json;base64,")


def test_encode_component_invalid_mtype():
    """encode_component with invalid mtype raises ValueError."""
    with pytest.raises(ValueError):
        encode_component(99, 0, 0, 64, b"\x00" * 8)


def test_iscc_decode_invalid_string():
    """iscc_decode with invalid string raises ValueError."""
    with pytest.raises(ValueError):
        iscc_decode("NOT_VALID_ISCC")


def test_json_to_data_url_invalid_json():
    """json_to_data_url with invalid JSON raises ValueError."""
    with pytest.raises(ValueError):
        json_to_data_url("not json {{{")


def test_encode_component_digest_too_short():
    """encode_component with digest shorter than bit_length/8 raises ValueError."""
    with pytest.raises(ValueError):
        encode_component(0, 0, 0, 64, b"\x00" * 4)  # need 8, only 4


def test_encode_component_iscc_mtype_rejected():
    """encode_component rejects MainType::Iscc (5) as mtype."""
    with pytest.raises(ValueError):
        encode_component(5, 0, 0, 64, b"\x00" * 8)
