"""Benchmark tests comparing iscc_lib (Rust) vs iscc_core (Python) for all 9 gen_*_v0 functions."""

import io
import json
from pathlib import Path

import pytest

import iscc_core as ic
import iscc_lib as il

DATA_JSON = Path(__file__).parent.parent / "crates" / "iscc-lib" / "tests" / "data.json"


def load_vectors(function_name):
    """Load conformance vectors for a given function from data.json."""
    data = json.loads(DATA_JSON.read_text(encoding="utf-8"))
    return data[function_name]


def _decode_stream(stream_str):
    """Decode 'stream:<hex>' input to bytes."""
    return bytes.fromhex(stream_str.removeprefix("stream:"))


def _prepare_meta_arg(meta_val):
    """Convert meta input from JSON value to Python argument for gen_meta_code_v0."""
    if meta_val is None:
        return None
    if isinstance(meta_val, str):
        return meta_val
    if isinstance(meta_val, dict):
        return json.dumps(meta_val, sort_keys=True)
    raise ValueError(f"unexpected meta type: {type(meta_val)}")


# ── Shared test data (loaded once per module) ────────────────────────────────

_vectors = {
    fn: load_vectors(fn)
    for fn in [
        "gen_meta_code_v0",
        "gen_text_code_v0",
        "gen_image_code_v0",
        "gen_audio_code_v0",
        "gen_video_code_v0",
        "gen_mixed_code_v0",
        "gen_data_code_v0",
        "gen_instance_code_v0",
        "gen_iscc_code_v0",
    ]
}


def _pick(fn_name, key):
    """Pick a specific conformance vector by function name and key."""
    return _vectors[fn_name][key]


# ── gen_meta_code_v0 ─────────────────────────────────────────────────────────

_meta_tc = _pick("gen_meta_code_v0", "test_0013_norm_i18n_256")
_meta_name = _meta_tc["inputs"][0]
_meta_desc = _meta_tc["inputs"][1] or None
_meta_meta = _prepare_meta_arg(_meta_tc["inputs"][2])
_meta_bits = _meta_tc["inputs"][3]


@pytest.mark.benchmark(group="gen_meta_code_v0")
def test_bench_meta_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_meta_code_v0."""
    result = benchmark(
        il.gen_meta_code_v0,
        _meta_name,
        description=_meta_desc,
        meta=_meta_meta,
        bits=_meta_bits,
    )
    assert result["iscc"] == _meta_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_meta_code_v0")
def test_bench_meta_iscc_core(benchmark):
    """Benchmark iscc_core.gen_meta_code_v0."""
    result = benchmark(
        ic.gen_meta_code_v0,
        _meta_name,
        description=_meta_desc,
        meta=_meta_meta,
        bits=_meta_bits,
    )
    assert result["iscc"] == _meta_tc["outputs"]["iscc"]


# ── gen_text_code_v0 ─────────────────────────────────────────────────────────

_text_tc = _pick("gen_text_code_v0", "test_0004_more")
_text_input = _text_tc["inputs"][0]
_text_bits = _text_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_text_code_v0")
def test_bench_text_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_text_code_v0."""
    result = benchmark(il.gen_text_code_v0, _text_input, bits=_text_bits)
    assert result["iscc"] == _text_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_text_code_v0")
def test_bench_text_iscc_core(benchmark):
    """Benchmark iscc_core.gen_text_code_v0."""
    result = benchmark(ic.gen_text_code_v0, _text_input, bits=_text_bits)
    assert result["iscc"] == _text_tc["outputs"]["iscc"]


# ── gen_image_code_v0 ────────────────────────────────────────────────────────

_image_tc = _pick("gen_image_code_v0", "test_0003_img_256")
_image_pixels = bytes(_image_tc["inputs"][0])
_image_bits = _image_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_image_code_v0")
def test_bench_image_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_image_code_v0."""
    result = benchmark(il.gen_image_code_v0, _image_pixels, bits=_image_bits)
    assert result["iscc"] == _image_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_image_code_v0")
def test_bench_image_iscc_core(benchmark):
    """Benchmark iscc_core.gen_image_code_v0."""
    result = benchmark(ic.gen_image_code_v0, list(_image_pixels), bits=_image_bits)
    assert result["iscc"] == _image_tc["outputs"]["iscc"]


# ── gen_audio_code_v0 ────────────────────────────────────────────────────────

_audio_tc = _pick("gen_audio_code_v0", "test_0004_cv_256")
_audio_input = _audio_tc["inputs"][0]
_audio_bits = _audio_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_audio_code_v0")
def test_bench_audio_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_audio_code_v0."""
    result = benchmark(il.gen_audio_code_v0, _audio_input, bits=_audio_bits)
    assert result["iscc"] == _audio_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_audio_code_v0")
def test_bench_audio_iscc_core(benchmark):
    """Benchmark iscc_core.gen_audio_code_v0."""
    result = benchmark(ic.gen_audio_code_v0, _audio_input, bits=_audio_bits)
    assert result["iscc"] == _audio_tc["outputs"]["iscc"]


# ── gen_video_code_v0 ────────────────────────────────────────────────────────

_video_tc = _pick("gen_video_code_v0", "test_0003_range_256")
_video_input = _video_tc["inputs"][0]
_video_bits = _video_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_video_code_v0")
def test_bench_video_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_video_code_v0."""
    result = benchmark(il.gen_video_code_v0, _video_input, bits=_video_bits)
    assert result["iscc"] == _video_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_video_code_v0")
def test_bench_video_iscc_core(benchmark):
    """Benchmark iscc_core.gen_video_code_v0."""
    result = benchmark(ic.gen_video_code_v0, _video_input, bits=_video_bits)
    assert result["iscc"] == _video_tc["outputs"]["iscc"]


# ── gen_mixed_code_v0 ────────────────────────────────────────────────────────

_mixed_tc = _pick("gen_mixed_code_v0", "test_0000_std_64")
_mixed_input = _mixed_tc["inputs"][0]
_mixed_bits = _mixed_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_mixed_code_v0")
def test_bench_mixed_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_mixed_code_v0."""
    result = benchmark(il.gen_mixed_code_v0, _mixed_input, bits=_mixed_bits)
    assert result["iscc"] == _mixed_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_mixed_code_v0")
def test_bench_mixed_iscc_core(benchmark):
    """Benchmark iscc_core.gen_mixed_code_v0."""
    result = benchmark(ic.gen_mixed_code_v0, _mixed_input, bits=_mixed_bits)
    assert result["iscc"] == _mixed_tc["outputs"]["iscc"]


# ── gen_data_code_v0 ─────────────────────────────────────────────────────────

_data_tc = _pick("gen_data_code_v0", "test_0003_static_256")
_data_bytes = _decode_stream(_data_tc["inputs"][0])
_data_bits = _data_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_data_code_v0")
def test_bench_data_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_data_code_v0."""
    result = benchmark(il.gen_data_code_v0, _data_bytes, bits=_data_bits)
    assert result["iscc"] == _data_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_data_code_v0")
def test_bench_data_iscc_core(benchmark):
    """Benchmark iscc_core.gen_data_code_v0."""

    def call():
        """Wrap call to create fresh BytesIO stream each iteration."""
        return ic.gen_data_code_v0(io.BytesIO(_data_bytes), bits=_data_bits)

    result = benchmark(call)
    assert result["iscc"] == _data_tc["outputs"]["iscc"]


# ── gen_instance_code_v0 ─────────────────────────────────────────────────────

_instance_tc = _pick("gen_instance_code_v0", "test_0002_static_256")
_instance_bytes = _decode_stream(_instance_tc["inputs"][0])
_instance_bits = _instance_tc["inputs"][1]


@pytest.mark.benchmark(group="gen_instance_code_v0")
def test_bench_instance_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_instance_code_v0."""
    result = benchmark(il.gen_instance_code_v0, _instance_bytes, bits=_instance_bits)
    assert result["iscc"] == _instance_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_instance_code_v0")
def test_bench_instance_iscc_core(benchmark):
    """Benchmark iscc_core.gen_instance_code_v0."""

    def call():
        """Wrap call to create fresh BytesIO stream each iteration."""
        return ic.gen_instance_code_v0(io.BytesIO(_instance_bytes), bits=_instance_bits)

    result = benchmark(call)
    assert result["iscc"] == _instance_tc["outputs"]["iscc"]


# ── gen_iscc_code_v0 ─────────────────────────────────────────────────────────

_iscc_tc = _pick("gen_iscc_code_v0", "test_0000_standard")
_iscc_codes = _iscc_tc["inputs"][0]


@pytest.mark.benchmark(group="gen_iscc_code_v0")
def test_bench_iscc_iscc_lib(benchmark):
    """Benchmark iscc_lib.gen_iscc_code_v0."""
    result = benchmark(il.gen_iscc_code_v0, _iscc_codes)
    assert result["iscc"] == _iscc_tc["outputs"]["iscc"]


@pytest.mark.benchmark(group="gen_iscc_code_v0")
def test_bench_iscc_iscc_core(benchmark):
    """Benchmark iscc_core.gen_iscc_code_v0."""
    result = benchmark(ic.gen_iscc_code_v0, _iscc_codes)
    assert result["iscc"] == _iscc_tc["outputs"]["iscc"]
