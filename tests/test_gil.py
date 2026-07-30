"""Concurrency-correctness tests for GIL release during Python hashing.

The streaming hasher `update()` methods and the one-shot byte-data hashing
functions release the GIL around their pure-Rust compute (issue #39), as do
the text and video compute paths (issue #41). These tests verify that
releasing the GIL does not change output: many threads producing
byte-identical results to the single-threaded path. They assert correctness
under concurrency, not a speedup.
"""

from concurrent.futures import ThreadPoolExecutor

from iscc_lib import (
    DataHasher,
    InstanceHasher,
    SumHasher,
    gen_data_code_v0,
    gen_image_code_v0,
    gen_instance_code_v0,
    gen_text_code_v0,
    gen_video_code_v0,
    soft_hash_video_v0,
)

THREADS = 16

# A payload large enough that each update() spends real time in the GIL-released
# compute, increasing the chance of overlapping threads exercising the release.
PAYLOAD = bytes(i % 256 for i in range(200_000))

# A text payload large enough that cleaning/collapsing + n-gram minhashing
# spends real time in the GIL-released compute.
TEXT_PAYLOAD = "Hello Wörld — ISCC text concurrency test. " * 5_000

# Synthetic nested frame signatures: 64 frames of 380-element i32 rows. These
# check output-consistency under contention, not conformance.
FRAME_SIGS = [[(f * 380 + i) % 256 - 128 for i in range(380)] for f in range(64)]


def _run_concurrent(fn, count=THREADS):
    """Run `fn(index)` across a thread pool and return results in submit order."""
    with ThreadPoolExecutor(max_workers=count) as pool:
        return list(pool.map(fn, range(count)))


def test_gen_data_code_v0_concurrent_matches_single_thread():
    """Verify gen_data_code_v0 yields identical output under thread contention."""
    expected = gen_data_code_v0(PAYLOAD)["iscc"]
    results = _run_concurrent(lambda _: gen_data_code_v0(PAYLOAD)["iscc"])
    assert all(r == expected for r in results)


def test_gen_instance_code_v0_concurrent_matches_single_thread():
    """Verify gen_instance_code_v0 yields identical output under thread contention."""
    single = gen_instance_code_v0(PAYLOAD)
    results = _run_concurrent(lambda _: gen_instance_code_v0(PAYLOAD))
    for r in results:
        assert r["iscc"] == single["iscc"]
        assert r["datahash"] == single["datahash"]
        assert r["filesize"] == single["filesize"]


def test_gen_image_code_v0_concurrent_matches_single_thread():
    """Verify gen_image_code_v0 yields identical output under thread contention."""
    pixels = bytes(i % 256 for i in range(1024))  # 32x32 grayscale
    expected = gen_image_code_v0(pixels)["iscc"]
    results = _run_concurrent(lambda _: gen_image_code_v0(pixels)["iscc"])
    assert all(r == expected for r in results)


def test_gen_text_code_v0_concurrent_matches_single_thread():
    """Verify gen_text_code_v0 yields identical output under thread contention."""
    single = gen_text_code_v0(TEXT_PAYLOAD)
    results = _run_concurrent(lambda _: gen_text_code_v0(TEXT_PAYLOAD))
    for r in results:
        assert r["iscc"] == single["iscc"]
        assert r["characters"] == single["characters"]


def test_gen_video_code_v0_concurrent_matches_single_thread():
    """Verify gen_video_code_v0 yields identical output under thread contention."""
    expected = gen_video_code_v0(FRAME_SIGS)["iscc"]
    results = _run_concurrent(lambda _: gen_video_code_v0(FRAME_SIGS)["iscc"])
    assert all(r == expected for r in results)


def test_soft_hash_video_v0_concurrent_matches_single_thread():
    """Verify soft_hash_video_v0 yields identical bytes under thread contention."""
    expected = soft_hash_video_v0(FRAME_SIGS, 256)
    results = _run_concurrent(lambda _: soft_hash_video_v0(FRAME_SIGS, 256))
    assert all(r == expected for r in results)


def test_gen_data_code_v0_concurrent_distinct_payloads():
    """Verify distinct concurrent payloads each match their single-threaded code."""
    payloads = [bytes([i]) * (1000 + i * 137) for i in range(THREADS)]
    expected = [gen_data_code_v0(p)["iscc"] for p in payloads]
    results = _run_concurrent(lambda i: gen_data_code_v0(payloads[i])["iscc"])
    assert results == expected


def test_data_hasher_concurrent_matches_single_thread():
    """Verify streaming DataHasher.update() is correct under thread contention."""
    expected = gen_data_code_v0(PAYLOAD)["iscc"]

    def hash_chunked(_):
        """Hash PAYLOAD via a fresh DataHasher fed in 16 KiB chunks."""
        hasher = DataHasher()
        for off in range(0, len(PAYLOAD), 16_384):
            hasher.update(PAYLOAD[off : off + 16_384])
        return hasher.finalize()["iscc"]

    results = _run_concurrent(hash_chunked)
    assert all(r == expected for r in results)


def test_instance_hasher_concurrent_matches_single_thread():
    """Verify streaming InstanceHasher.update() is correct under thread contention."""
    single = gen_instance_code_v0(PAYLOAD)

    def hash_chunked(_):
        """Hash PAYLOAD via a fresh InstanceHasher fed in 16 KiB chunks."""
        hasher = InstanceHasher()
        for off in range(0, len(PAYLOAD), 16_384):
            hasher.update(PAYLOAD[off : off + 16_384])
        return hasher.finalize()

    results = _run_concurrent(hash_chunked)
    for r in results:
        assert r["iscc"] == single["iscc"]
        assert r["datahash"] == single["datahash"]
        assert r["filesize"] == single["filesize"]


def test_sum_hasher_concurrent_matches_single_thread():
    """Verify streaming SumHasher.update() is correct under thread contention."""

    def hash_chunked(_):
        """Hash PAYLOAD via a fresh SumHasher fed in 16 KiB chunks."""
        hasher = SumHasher()
        for off in range(0, len(PAYLOAD), 16_384):
            hasher.update(PAYLOAD[off : off + 16_384])
        return hasher.finalize(add_units=True)

    expected = hash_chunked(0)
    results = _run_concurrent(hash_chunked)
    for r in results:
        assert r["iscc"] == expected["iscc"]
        assert r["datahash"] == expected["datahash"]
        assert r["filesize"] == expected["filesize"]
        assert r["units"] == expected["units"]
