"""Tests for soft_hash_video_v0, DataHasher, and InstanceHasher Python bindings."""

import io
import json
from pathlib import Path

import pytest

from iscc_lib import (
    DataCodeResult,
    DataHasher,
    InstanceCodeResult,
    InstanceHasher,
    gen_data_code_v0,
    gen_instance_code_v0,
    soft_hash_video_v0,
)

DATA_JSON = Path(__file__).parent.parent / "crates" / "iscc-lib" / "tests" / "data.json"


def _decode_stream(stream_str):
    """Decode 'stream:<hex>' input to bytes."""
    hex_data = stream_str.removeprefix("stream:")
    return bytes.fromhex(hex_data)


def load_vectors(function_name):
    """Load conformance vectors for a given function from data.json."""
    data = json.loads(DATA_JSON.read_text(encoding="utf-8"))
    section = data[function_name]
    return [pytest.param(tc, id=name) for name, tc in section.items()]


# ── soft_hash_video_v0 ─────────────────────────────────────────────────────


def test_soft_hash_video_v0_zero_vectors():
    """Verify zero-vector input returns zero bytes."""
    frame_sigs = [[0] * 380, [0] * 380]
    result = soft_hash_video_v0(frame_sigs)
    assert isinstance(result, bytes)
    assert len(result) == 8  # 64 bits / 8
    assert result == b"\x00" * 8


def test_soft_hash_video_v0_returns_correct_length():
    """Verify output length matches bits / 8."""
    frame_sigs = [[1] * 380]
    result_64 = soft_hash_video_v0(frame_sigs, bits=64)
    assert len(result_64) == 8
    result_128 = soft_hash_video_v0(frame_sigs, bits=128)
    assert len(result_128) == 16
    result_256 = soft_hash_video_v0(frame_sigs, bits=256)
    assert len(result_256) == 32


def test_soft_hash_video_v0_empty_raises_value_error():
    """Verify empty frame_sigs raises ValueError."""
    with pytest.raises(ValueError, match="must not be empty"):
        soft_hash_video_v0([])


def test_soft_hash_video_v0_range_vectors():
    """Verify range-vector input matches iscc-core reference."""
    import iscc_core

    frame_sigs = [list(range(380)), list(range(1, 381))]
    rust_result = soft_hash_video_v0(frame_sigs, bits=64)
    core_result = iscc_core.soft_hash_video_v0(frame_sigs, bits=64)
    assert rust_result == core_result


def test_soft_hash_video_v0_cross_validate_various_inputs():
    """Verify multiple inputs match iscc-core."""
    import iscc_core

    # Three identical frames (dedup reduces to one)
    frame_sigs = [list(range(380))] * 3
    rust_result = soft_hash_video_v0(frame_sigs, bits=64)
    core_result = iscc_core.soft_hash_video_v0(frame_sigs, bits=64)
    assert rust_result == core_result


def test_soft_hash_video_v0_default_bits():
    """Verify default bits parameter is 64."""
    frame_sigs = [[i % 256 for i in range(380)]]
    result_default = soft_hash_video_v0(frame_sigs)
    result_explicit = soft_hash_video_v0(frame_sigs, bits=64)
    assert result_default == result_explicit


# ── DataHasher ──────────────────────────────────────────────────────────────


def test_data_hasher_empty_matches_oneshot():
    """Verify empty DataHasher matches gen_data_code_v0(b'')."""
    dh = DataHasher()
    result = dh.finalize()
    oneshot = gen_data_code_v0(b"")
    assert result["iscc"] == oneshot["iscc"]
    assert isinstance(result, DataCodeResult)


def test_data_hasher_single_update_matches_oneshot():
    """Verify single update matches one-shot for small data."""
    data = b"Hello, ISCC World!"
    dh = DataHasher()
    dh.update(data)
    result = dh.finalize()
    oneshot = gen_data_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]


def test_data_hasher_multi_chunk_matches_oneshot():
    """Verify multi-chunk streaming matches one-shot."""
    data = b"The quick brown fox jumps over the lazy dog" * 100
    dh = DataHasher()
    dh.update(data[:500])
    dh.update(data[500:1000])
    dh.update(data[1000:])
    result = dh.finalize()
    oneshot = gen_data_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]


def test_data_hasher_binaryio_input():
    """Verify DataHasher.update() accepts BinaryIO (BytesIO)."""
    data = b"Hello World via stream"
    dh = DataHasher()
    dh.update(io.BytesIO(data))
    result = dh.finalize()
    oneshot = gen_data_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]


def test_data_hasher_constructor_with_data():
    """Verify DataHasher(data=...) processes initial data."""
    data = b"constructor data"
    dh = DataHasher(data=data)
    result = dh.finalize()
    oneshot = gen_data_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]


def test_data_hasher_constructor_with_binaryio():
    """Verify DataHasher(data=BytesIO(...)) processes initial stream."""
    data = b"constructor stream data"
    dh = DataHasher(data=io.BytesIO(data))
    result = dh.finalize()
    oneshot = gen_data_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]


def test_data_hasher_double_finalize_raises():
    """Verify calling finalize() twice raises ValueError."""
    dh = DataHasher()
    dh.finalize()
    with pytest.raises(ValueError, match="already finalized"):
        dh.finalize()


def test_data_hasher_update_after_finalize_raises():
    """Verify calling update() after finalize raises ValueError."""
    dh = DataHasher()
    dh.finalize()
    with pytest.raises(ValueError, match="already finalized"):
        dh.update(b"more data")


def test_data_hasher_various_bits():
    """Verify DataHasher works with different bit widths."""
    data = b"test various bit widths"
    for bits in [64, 128, 256]:
        dh = DataHasher()
        dh.update(data)
        result = dh.finalize(bits=bits)
        oneshot = gen_data_code_v0(data, bits=bits)
        assert result["iscc"] == oneshot["iscc"], f"bits={bits}"


@pytest.mark.parametrize("tc", load_vectors("gen_data_code_v0"))
def test_data_hasher_conformance(tc):
    """Verify DataHasher streaming matches gen_data_code_v0 for conformance vectors."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    data = _decode_stream(inputs[0])
    bits = inputs[1]

    # Single update
    dh = DataHasher()
    dh.update(data)
    result = dh.finalize(bits=bits)
    assert result["iscc"] == outputs["iscc"]

    # Multi-chunk (256-byte chunks)
    dh2 = DataHasher()
    for i in range(0, len(data), 256):
        dh2.update(data[i : i + 256])
    result2 = dh2.finalize(bits=bits)
    assert result2["iscc"] == outputs["iscc"]


# ── InstanceHasher ──────────────────────────────────────────────────────────


def test_instance_hasher_empty_matches_oneshot():
    """Verify empty InstanceHasher matches gen_instance_code_v0(b'')."""
    ih = InstanceHasher()
    result = ih.finalize()
    oneshot = gen_instance_code_v0(b"")
    assert result["iscc"] == oneshot["iscc"]
    assert result["datahash"] == oneshot["datahash"]
    assert result["filesize"] == oneshot["filesize"]
    assert isinstance(result, InstanceCodeResult)


def test_instance_hasher_single_update_matches_oneshot():
    """Verify single update matches one-shot."""
    data = b"Hello, ISCC World!"
    ih = InstanceHasher()
    ih.update(data)
    result = ih.finalize()
    oneshot = gen_instance_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]
    assert result["datahash"] == oneshot["datahash"]
    assert result["filesize"] == oneshot["filesize"]


def test_instance_hasher_multi_chunk_matches_oneshot():
    """Verify multi-chunk streaming produces identical results."""
    data = b"The quick brown fox jumps over the lazy dog"
    ih = InstanceHasher()
    ih.update(data[:10])
    ih.update(data[10:25])
    ih.update(data[25:])
    result = ih.finalize()
    oneshot = gen_instance_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]
    assert result["datahash"] == oneshot["datahash"]
    assert result["filesize"] == oneshot["filesize"]


def test_instance_hasher_binaryio_input():
    """Verify InstanceHasher.update() accepts BinaryIO (BytesIO)."""
    data = b"Hello World via stream"
    ih = InstanceHasher()
    ih.update(io.BytesIO(data))
    result = ih.finalize()
    oneshot = gen_instance_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]
    assert result["datahash"] == oneshot["datahash"]
    assert result["filesize"] == oneshot["filesize"]


def test_instance_hasher_constructor_with_data():
    """Verify InstanceHasher(data=...) processes initial data."""
    data = b"constructor data"
    ih = InstanceHasher(data=data)
    result = ih.finalize()
    oneshot = gen_instance_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]
    assert result["datahash"] == oneshot["datahash"]
    assert result["filesize"] == oneshot["filesize"]


def test_instance_hasher_constructor_with_binaryio():
    """Verify InstanceHasher(data=BytesIO(...)) processes initial stream."""
    data = b"constructor stream data"
    ih = InstanceHasher(data=io.BytesIO(data))
    result = ih.finalize()
    oneshot = gen_instance_code_v0(data)
    assert result["iscc"] == oneshot["iscc"]


def test_instance_hasher_double_finalize_raises():
    """Verify calling finalize() twice raises ValueError."""
    ih = InstanceHasher()
    ih.finalize()
    with pytest.raises(ValueError, match="already finalized"):
        ih.finalize()


def test_instance_hasher_update_after_finalize_raises():
    """Verify calling update() after finalize raises ValueError."""
    ih = InstanceHasher()
    ih.finalize()
    with pytest.raises(ValueError, match="already finalized"):
        ih.update(b"more data")


def test_instance_hasher_various_bits():
    """Verify InstanceHasher works with different bit widths."""
    data = b"test various bit widths"
    for bits in [64, 128, 256]:
        ih = InstanceHasher()
        ih.update(data)
        result = ih.finalize(bits=bits)
        oneshot = gen_instance_code_v0(data, bits=bits)
        assert result["iscc"] == oneshot["iscc"], f"bits={bits}"
        assert result["datahash"] == oneshot["datahash"], f"bits={bits}"


@pytest.mark.parametrize("tc", load_vectors("gen_instance_code_v0"))
def test_instance_hasher_conformance(tc):
    """Verify InstanceHasher streaming matches gen_instance_code_v0 for conformance vectors."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    data = _decode_stream(inputs[0])
    bits = inputs[1]

    # Single update
    ih = InstanceHasher()
    ih.update(data)
    result = ih.finalize(bits=bits)
    assert result["iscc"] == outputs["iscc"]
    assert result["datahash"] == outputs["datahash"]
    assert result["filesize"] == outputs["filesize"]

    # Multi-chunk (256-byte chunks)
    ih2 = InstanceHasher()
    for i in range(0, len(data), 256):
        ih2.update(data[i : i + 256])
    result2 = ih2.finalize(bits=bits)
    assert result2["iscc"] == outputs["iscc"]
    assert result2["datahash"] == outputs["datahash"]
    assert result2["filesize"] == outputs["filesize"]


# ── Bytes-like input tests ─────────────────────────────────────────────────


def test_gen_data_code_v0_bytearray():
    """Verify bytearray input matches bytes input for gen_data_code_v0."""
    data = b"Hello, ISCC bytearray test!"
    expected = gen_data_code_v0(data)
    result = gen_data_code_v0(bytearray(data))
    assert result["iscc"] == expected["iscc"]


def test_gen_data_code_v0_memoryview():
    """Verify memoryview input matches bytes input for gen_data_code_v0."""
    data = b"Hello, ISCC memoryview test!"
    expected = gen_data_code_v0(data)
    result = gen_data_code_v0(memoryview(data))
    assert result["iscc"] == expected["iscc"]


def test_gen_instance_code_v0_bytearray():
    """Verify bytearray input matches bytes input for gen_instance_code_v0."""
    data = b"Hello, ISCC bytearray instance test!"
    expected = gen_instance_code_v0(data)
    result = gen_instance_code_v0(bytearray(data))
    assert result["iscc"] == expected["iscc"]
    assert result["datahash"] == expected["datahash"]
    assert result["filesize"] == expected["filesize"]


def test_gen_instance_code_v0_memoryview():
    """Verify memoryview input matches bytes input for gen_instance_code_v0."""
    data = b"Hello, ISCC memoryview instance test!"
    expected = gen_instance_code_v0(data)
    result = gen_instance_code_v0(memoryview(data))
    assert result["iscc"] == expected["iscc"]
    assert result["datahash"] == expected["datahash"]
    assert result["filesize"] == expected["filesize"]


def test_data_hasher_bytearray():
    """Verify DataHasher.update(bytearray(...)) works correctly."""
    data = b"DataHasher bytearray test"
    expected = gen_data_code_v0(data)
    dh = DataHasher()
    dh.update(bytearray(data))
    result = dh.finalize()
    assert result["iscc"] == expected["iscc"]


def test_data_hasher_memoryview():
    """Verify DataHasher.update(memoryview(...)) works correctly."""
    data = b"DataHasher memoryview test"
    expected = gen_data_code_v0(data)
    dh = DataHasher()
    dh.update(memoryview(data))
    result = dh.finalize()
    assert result["iscc"] == expected["iscc"]


def test_instance_hasher_bytearray():
    """Verify InstanceHasher.update(bytearray(...)) works correctly."""
    data = b"InstanceHasher bytearray test"
    expected = gen_instance_code_v0(data)
    ih = InstanceHasher()
    ih.update(bytearray(data))
    result = ih.finalize()
    assert result["iscc"] == expected["iscc"]
    assert result["datahash"] == expected["datahash"]
    assert result["filesize"] == expected["filesize"]


def test_instance_hasher_memoryview():
    """Verify InstanceHasher.update(memoryview(...)) works correctly."""
    data = b"InstanceHasher memoryview test"
    expected = gen_instance_code_v0(data)
    ih = InstanceHasher()
    ih.update(memoryview(data))
    result = ih.finalize()
    assert result["iscc"] == expected["iscc"]
    assert result["datahash"] == expected["datahash"]
    assert result["filesize"] == expected["filesize"]


# ── Chunked streaming tests ───────────────────────────────────────────────


def test_gen_data_code_v0_stream_chunked():
    """Verify a large BytesIO stream produces the same result as one-shot bytes."""
    data = b"The quick brown fox jumps over the lazy dog" * 5000
    expected = gen_data_code_v0(data)
    result = gen_data_code_v0(io.BytesIO(data))
    assert result["iscc"] == expected["iscc"]


def test_gen_instance_code_v0_stream_chunked():
    """Verify a large BytesIO stream produces the same result as one-shot bytes."""
    data = b"The quick brown fox jumps over the lazy dog" * 5000
    expected = gen_instance_code_v0(data)
    result = gen_instance_code_v0(io.BytesIO(data))
    assert result["iscc"] == expected["iscc"]
    assert result["datahash"] == expected["datahash"]
    assert result["filesize"] == expected["filesize"]
