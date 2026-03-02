"""Pytest-benchmark tests for all 9 iscc-core gen_*_v0 functions.

Establishes the Python reference performance baseline (Phase 0) for computing
speedup factors against the Rust implementation. Uses identical inputs to the
Rust criterion benchmarks in crates/iscc-lib/benches/benchmarks.rs.
"""

import io

import pytest
from iscc_core.code_content_audio import gen_audio_code_v0
from iscc_core.code_content_image import gen_image_code_v0
from iscc_core.code_content_mixed import gen_mixed_code_v0
from iscc_core.code_content_text import gen_text_code_v0
from iscc_core.code_content_video import gen_video_code_v0
from iscc_core.code_data import gen_data_code_v0
from iscc_core.code_instance import gen_instance_code_v0
from iscc_core.code_meta import gen_meta_code_v0
from iscc_core.iscc_code import gen_iscc_code_v0


def synthetic_text(chars):
    """Generate a synthetic text string of approximately the given character count."""
    base = "The quick brown fox jumps over the lazy dog. "
    repeats = (chars // len(base)) + 1
    return (base * repeats)[:chars]


# Pre-computed inputs (matching Rust criterion benchmarks)
TEXT_1000 = synthetic_text(1000)
PIXELS_1024 = [i % 256 for i in range(1024)]
AUDIO_CV_300 = list(range(300))
VIDEO_10_FRAMES = [tuple(range(f * 380, f * 380 + 380)) for f in range(10)]
MIXED_CODES = ["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"]
DATA_64K = bytes(i % 256 for i in range(65536))
ISCC_UNITS = [
    "AAAYPXW445FTYNJ3",
    "EAARMJLTQCUWAND2",
    "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
    "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ",
]


@pytest.mark.benchmark(group="gen_meta_code_v0")
def test_bench_gen_meta_code_v0(benchmark):
    """Benchmark gen_meta_code_v0 with name and description."""
    benchmark(
        gen_meta_code_v0,
        "Die Unendliche Geschichte",
        "Von Michael Ende",
        None,
        64,
    )


@pytest.mark.benchmark(group="gen_text_code_v0")
def test_bench_gen_text_code_v0(benchmark):
    """Benchmark gen_text_code_v0 with ~1000-character synthetic text."""
    benchmark(gen_text_code_v0, TEXT_1000, 64)


@pytest.mark.benchmark(group="gen_image_code_v0")
def test_bench_gen_image_code_v0(benchmark):
    """Benchmark gen_image_code_v0 with 1024-element pixel list."""
    benchmark(gen_image_code_v0, PIXELS_1024, 64)


@pytest.mark.benchmark(group="gen_audio_code_v0")
def test_bench_gen_audio_code_v0(benchmark):
    """Benchmark gen_audio_code_v0 with 300-element feature vector."""
    benchmark(gen_audio_code_v0, AUDIO_CV_300, 64)


@pytest.mark.benchmark(group="gen_video_code_v0")
def test_bench_gen_video_code_v0(benchmark):
    """Benchmark gen_video_code_v0 with 10 frames of 380 sequential ints."""
    benchmark(gen_video_code_v0, VIDEO_10_FRAMES, 64)


@pytest.mark.benchmark(group="gen_mixed_code_v0")
def test_bench_gen_mixed_code_v0(benchmark):
    """Benchmark gen_mixed_code_v0 with 2 Content-Code strings."""
    benchmark(gen_mixed_code_v0, MIXED_CODES, 64)


@pytest.mark.benchmark(group="gen_data_code_v0")
def test_bench_gen_data_code_v0(benchmark):
    """Benchmark gen_data_code_v0 with 64KB deterministic bytes."""
    benchmark(lambda: gen_data_code_v0(io.BytesIO(DATA_64K), 64))


@pytest.mark.benchmark(group="gen_instance_code_v0")
def test_bench_gen_instance_code_v0(benchmark):
    """Benchmark gen_instance_code_v0 with 64KB deterministic bytes."""
    benchmark(lambda: gen_instance_code_v0(io.BytesIO(DATA_64K), 64))


@pytest.mark.benchmark(group="gen_iscc_code_v0")
def test_bench_gen_iscc_code_v0(benchmark):
    """Benchmark gen_iscc_code_v0 with 4 ISCC unit strings."""
    benchmark(gen_iscc_code_v0, ISCC_UNITS, False)
