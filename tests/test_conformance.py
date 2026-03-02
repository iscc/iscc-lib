"""Conformance tests for all 9 gen_*_v0 functions against data.json vectors."""

import json
from pathlib import Path

import pytest

from iscc_lib import (
    gen_audio_code_v0,
    gen_data_code_v0,
    gen_image_code_v0,
    gen_instance_code_v0,
    gen_iscc_code_v0,
    gen_meta_code_v0,
    gen_mixed_code_v0,
    gen_text_code_v0,
    gen_video_code_v0,
)

DATA_JSON = Path(__file__).parent.parent / "crates" / "iscc-lib" / "tests" / "data.json"


def load_vectors(function_name):
    """Load conformance vectors for a given function from data.json."""
    data = json.loads(DATA_JSON.read_text(encoding="utf-8"))
    section = data[function_name]
    return [pytest.param(tc, id=name) for name, tc in section.items()]


# ── gen_meta_code_v0 ─────────────────────────────────────────────────────────


def _prepare_meta_arg(meta_val):
    """Convert meta input from JSON value to Python argument for gen_meta_code_v0."""
    if meta_val is None:
        return None
    if isinstance(meta_val, str):
        return meta_val
    if isinstance(meta_val, dict):
        return json.dumps(meta_val, sort_keys=True)
    raise ValueError(f"unexpected meta type: {type(meta_val)}")


@pytest.mark.parametrize("tc", load_vectors("gen_meta_code_v0"))
def test_gen_meta_code_v0(tc):
    """Verify gen_meta_code_v0 returns dict with all fields matching conformance vectors."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    name = inputs[0]
    description = inputs[1]
    meta = _prepare_meta_arg(inputs[2])
    bits = inputs[3]

    result = gen_meta_code_v0(
        name, description=description or None, meta=meta, bits=bits
    )
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]
    assert result["name"] == outputs["name"]
    assert result["metahash"] == outputs["metahash"]
    if "description" in outputs:
        assert result["description"] == outputs["description"]
    else:
        assert "description" not in result
    if "meta" in outputs:
        assert result["meta"] == outputs["meta"]
    else:
        assert "meta" not in result


# ── gen_text_code_v0 ─────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_text_code_v0"))
def test_gen_text_code_v0(tc):
    """Verify gen_text_code_v0 returns dict with iscc and characters."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    result = gen_text_code_v0(inputs[0], bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]
    assert result["characters"] == outputs["characters"]


# ── gen_image_code_v0 ────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_image_code_v0"))
def test_gen_image_code_v0(tc):
    """Verify gen_image_code_v0 returns dict with iscc."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    pixels = bytes(inputs[0])
    result = gen_image_code_v0(pixels, bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]


# ── gen_audio_code_v0 ────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_audio_code_v0"))
def test_gen_audio_code_v0(tc):
    """Verify gen_audio_code_v0 returns dict with iscc."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    result = gen_audio_code_v0(inputs[0], bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]


# ── gen_video_code_v0 ────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_video_code_v0"))
def test_gen_video_code_v0(tc):
    """Verify gen_video_code_v0 returns dict with iscc."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    result = gen_video_code_v0(inputs[0], bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]


# ── gen_mixed_code_v0 ────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_mixed_code_v0"))
def test_gen_mixed_code_v0(tc):
    """Verify gen_mixed_code_v0 returns dict with iscc and parts."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    result = gen_mixed_code_v0(inputs[0], bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]
    assert result["parts"] == outputs["parts"]


# ── gen_data_code_v0 ─────────────────────────────────────────────────────────


def _decode_stream(stream_str):
    """Decode 'stream:<hex>' input to bytes."""
    hex_data = stream_str.removeprefix("stream:")
    return bytes.fromhex(hex_data)


@pytest.mark.parametrize("tc", load_vectors("gen_data_code_v0"))
def test_gen_data_code_v0(tc):
    """Verify gen_data_code_v0 returns dict with iscc."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    data = _decode_stream(inputs[0])
    result = gen_data_code_v0(data, bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]


# ── gen_instance_code_v0 ─────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_instance_code_v0"))
def test_gen_instance_code_v0(tc):
    """Verify gen_instance_code_v0 returns dict with iscc, datahash, and filesize."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    data = _decode_stream(inputs[0])
    result = gen_instance_code_v0(data, bits=inputs[1])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]
    assert result["datahash"] == outputs["datahash"]
    assert result["filesize"] == outputs["filesize"]


# ── gen_iscc_code_v0 ─────────────────────────────────────────────────────────


@pytest.mark.parametrize("tc", load_vectors("gen_iscc_code_v0"))
def test_gen_iscc_code_v0(tc):
    """Verify gen_iscc_code_v0 returns dict with iscc."""
    inputs = tc["inputs"]
    outputs = tc["outputs"]
    result = gen_iscc_code_v0(inputs[0])
    assert isinstance(result, dict)
    assert result["iscc"] == outputs["iscc"]
