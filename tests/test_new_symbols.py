"""Tests for Tier 1 Python symbols, dict meta, PIL pixel data, enums, and core_opts."""

import json

import pytest

from iscc_lib import (
    IO_READ_SIZE,
    META_TRIM_DESCRIPTION,
    META_TRIM_NAME,
    MT,
    ST,
    TEXT_NGRAM_SIZE,
    VS,
    core_opts,
    encode_component,
    gen_image_code_v0,
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


# ── PIL pixel data (gen_image_code_v0) tests ─────────────────────────────────


def test_gen_image_code_v0_list_int():
    """gen_image_code_v0 accepts list[int] (Sequence[int]) like PIL getdata()."""
    pixels = list(range(256)) * 4  # 1024 ints, each 0-255
    result = gen_image_code_v0(pixels)
    assert "iscc" in result
    assert result["iscc"].startswith("ISCC:")


def test_gen_image_code_v0_bytearray():
    """gen_image_code_v0 accepts bytearray input."""
    pixels = bytearray(b"\x80" * 1024)
    result = gen_image_code_v0(pixels)
    assert "iscc" in result
    assert result["iscc"].startswith("ISCC:")


def test_gen_image_code_v0_memoryview():
    """gen_image_code_v0 accepts memoryview input."""
    pixels = memoryview(b"\x80" * 1024)
    result = gen_image_code_v0(pixels)
    assert "iscc" in result
    assert result["iscc"].startswith("ISCC:")


def test_gen_image_code_v0_bytes_regression():
    """Regression: gen_image_code_v0 still accepts bytes."""
    pixels = b"\x80" * 1024
    result = gen_image_code_v0(pixels)
    assert "iscc" in result
    assert result["iscc"].startswith("ISCC:")


def test_gen_image_code_v0_list_matches_bytes():
    """list[int] and bytes produce identical ISCC for equivalent pixel data."""
    pixel_values = list(range(256)) * 4  # 1024 ints
    result_list = gen_image_code_v0(pixel_values)
    result_bytes = gen_image_code_v0(bytes(pixel_values))
    assert result_list["iscc"] == result_bytes["iscc"]


def test_gen_image_code_v0_bytearray_matches_bytes():
    """bytearray and bytes produce identical ISCC for same pixel data."""
    raw = b"\x80" * 1024
    result_ba = gen_image_code_v0(bytearray(raw))
    result_bytes = gen_image_code_v0(raw)
    assert result_ba["iscc"] == result_bytes["iscc"]


def test_gen_image_code_v0_memoryview_matches_bytes():
    """memoryview and bytes produce identical ISCC for same pixel data."""
    raw = b"\x80" * 1024
    result_mv = gen_image_code_v0(memoryview(raw))
    result_bytes = gen_image_code_v0(raw)
    assert result_mv["iscc"] == result_bytes["iscc"]


# ── MT IntEnum tests ─────────────────────────────────────────────────────────


def test_mt_values():
    """MT IntEnum has correct values for all 8 MainTypes."""
    assert MT.META == 0
    assert MT.SEMANTIC == 1
    assert MT.CONTENT == 2
    assert MT.DATA == 3
    assert MT.INSTANCE == 4
    assert MT.ISCC == 5
    assert MT.ID == 6
    assert MT.FLAKE == 7


def test_mt_is_int():
    """MT members are integers (IntEnum inherits from int)."""
    assert isinstance(MT.DATA, int)
    assert isinstance(MT.META, int)


# ── ST IntEnum tests ─────────────────────────────────────────────────────────


def test_st_values():
    """ST IntEnum has correct values for all SubTypes."""
    assert ST.NONE == 0
    assert ST.IMAGE == 1
    assert ST.AUDIO == 2
    assert ST.VIDEO == 3
    assert ST.MIXED == 4
    assert ST.SUM == 5
    assert ST.ISCC_NONE == 6
    assert ST.WIDE == 7


def test_st_text_alias():
    """ST.TEXT is an alias for ST.NONE (both are value 0)."""
    assert ST.TEXT == 0
    assert ST.TEXT == ST.NONE


# ── VS IntEnum tests ─────────────────────────────────────────────────────────


def test_vs_values():
    """VS IntEnum has correct value."""
    assert VS.V0 == 0


# ── core_opts tests ──────────────────────────────────────────────────────────


def test_core_opts_meta_trim_name():
    """core_opts.meta_trim_name matches the module-level constant."""
    assert core_opts.meta_trim_name == 128
    assert core_opts.meta_trim_name == META_TRIM_NAME


def test_core_opts_meta_trim_description():
    """core_opts.meta_trim_description matches the module-level constant."""
    assert core_opts.meta_trim_description == 4096
    assert core_opts.meta_trim_description == META_TRIM_DESCRIPTION


def test_core_opts_io_read_size():
    """core_opts.io_read_size matches the module-level constant."""
    assert core_opts.io_read_size == 4_194_304
    assert core_opts.io_read_size == IO_READ_SIZE


def test_core_opts_text_ngram_size():
    """core_opts.text_ngram_size matches the module-level constant."""
    assert core_opts.text_ngram_size == 13
    assert core_opts.text_ngram_size == TEXT_NGRAM_SIZE


# ── iscc_decode IntEnum wrapping tests ───────────────────────────────────────


def test_iscc_decode_returns_mt():
    """iscc_decode returns MT IntEnum for the maintype field."""
    digest = bytes(range(8))
    encoded = encode_component(0, 0, 0, 64, digest)
    result = iscc_decode(encoded)
    assert isinstance(result[0], MT)


def test_iscc_decode_returns_st():
    """iscc_decode returns ST IntEnum for the subtype field."""
    digest = bytes(range(8))
    encoded = encode_component(0, 0, 0, 64, digest)
    result = iscc_decode(encoded)
    assert isinstance(result[1], ST)


def test_iscc_decode_returns_vs():
    """iscc_decode returns VS IntEnum for the version field."""
    digest = bytes(range(8))
    encoded = encode_component(0, 0, 0, 64, digest)
    result = iscc_decode(encoded)
    assert isinstance(result[2], VS)


def test_iscc_decode_roundtrip_with_enums():
    """Round-trip: encode_component with enum values, decode, verify match."""
    digest = b"\xab\xcd\xef\x01\x23\x45\x67\x89"
    encoded = encode_component(MT.DATA, ST.NONE, VS.V0, 64, digest)
    mt, st, vs, length, decoded_digest = iscc_decode(encoded)
    assert mt == MT.DATA
    assert st == ST.NONE
    assert vs == VS.V0
    assert decoded_digest == digest


def test_iscc_decode_known_code():
    """Decode a known ISCC code and verify IntEnum types."""
    # GAA2XTPPAERUKZ4J is a Data-Code (MT=3, ST=0, VS=0, 64-bit)
    mt, st, vs, length, digest = iscc_decode("GAA2XTPPAERUKZ4J")
    assert mt == MT.DATA
    assert st == ST.NONE
    assert vs == VS.V0
    assert isinstance(mt, MT)
    assert isinstance(st, ST)
    assert isinstance(vs, VS)
    assert digest == b"\xab\xcd\xef\x01\x23\x45\x67\x89"
