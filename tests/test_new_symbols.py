"""Tests for Tier 1 Python symbols and dict meta parameter support."""

import json

import pytest

from iscc_lib import (
    IO_READ_SIZE,
    META_TRIM_DESCRIPTION,
    META_TRIM_NAME,
    TEXT_NGRAM_SIZE,
    encode_component,
    gen_meta_code_v0,
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


# ── Dict meta parameter tests ────────────────────────────────────────────────


def test_gen_meta_dict_meta_basic():
    """Pass meta as dict, verify result meta starts with data:application/json;base64,."""
    result = gen_meta_code_v0("Test Name", meta={"key": "value"})
    assert "meta" in result
    assert result["meta"].startswith("data:application/json;base64,")


def test_gen_meta_dict_meta_ld_json():
    """Pass meta dict with @context, verify result uses application/ld+json media type."""
    result = gen_meta_code_v0(
        "Test Name", meta={"@context": "https://schema.org", "name": "Test"}
    )
    assert "meta" in result
    assert result["meta"].startswith("data:application/ld+json;base64,")


def test_gen_meta_dict_meta_matches_string():
    """Dict meta and pre-computed data URL string produce identical ISCC codes."""
    meta_dict = {"key": "value"}
    # Compute the data URL string the same way the wrapper does
    json_str = json.dumps(meta_dict, separators=(",", ":"), ensure_ascii=False)
    data_url = json_to_data_url(json_str)

    result_dict = gen_meta_code_v0("Test Name", meta=meta_dict)
    result_str = gen_meta_code_v0("Test Name", meta=data_url)
    assert result_dict["iscc"] == result_str["iscc"]
    assert result_dict["meta"] == result_str["meta"]
    assert result_dict["metahash"] == result_str["metahash"]


def test_gen_meta_str_meta_still_works():
    """Regression: meta as data URL string still works."""
    data_url = json_to_data_url('{"key":"value"}')
    result = gen_meta_code_v0("Test Name", meta=data_url)
    assert "iscc" in result
    assert result["meta"] == data_url


def test_gen_meta_dict_meta_none_still_works():
    """Regression: meta=None still works (no meta in output)."""
    result = gen_meta_code_v0("Test Name", meta=None)
    assert "iscc" in result
    assert "meta" not in result
