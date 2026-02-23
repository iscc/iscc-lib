"""Tests for algorithm primitive functions exposed via Python bindings."""

import iscc_core
from iscc_lib import alg_cdc_chunks, alg_minhash_256, alg_simhash


# ── alg_simhash ────────────────────────────────────────────────────────────


def test_alg_simhash_empty_returns_zero_bytes():
    """Verify empty input returns 32 zero bytes."""
    result = alg_simhash([])
    assert result == b"\x00" * 32


def test_alg_simhash_single_digest():
    """Verify single digest is returned unchanged."""
    digest = b"\xab" * 32
    result = alg_simhash([digest])
    assert result == digest


def test_alg_simhash_returns_bytes_of_correct_length():
    """Verify output length matches input digest length."""
    digests = [b"\xff" * 16, b"\x00" * 16]
    result = alg_simhash(digests)
    assert isinstance(result, bytes)
    assert len(result) == 16


def test_alg_simhash_identical_digests():
    """Verify identical digests produce the same output."""
    digest = b"\xca\xfe" * 16
    result = alg_simhash([digest, digest, digest])
    assert result == digest


def test_alg_simhash_cross_validate_with_iscc_core():
    """Verify output matches iscc-core for known inputs."""
    import hashlib

    digests = [hashlib.sha256(s.encode()).digest() for s in ["hello", "world", "test"]]
    rust_result = alg_simhash(digests)
    core_result = iscc_core.alg_simhash(digests)
    assert rust_result == core_result


def test_alg_simhash_4byte_digests():
    """Verify SimHash works with 4-byte digests (used in audio)."""
    import struct

    digests = [struct.pack(">I", v) for v in [0xAABBCCDD, 0x11223344, 0xFF00FF00]]
    rust_result = alg_simhash(digests)
    core_result = iscc_core.alg_simhash(digests)
    assert len(rust_result) == 4
    assert rust_result == core_result


# ── alg_minhash_256 ────────────────────────────────────────────────────────


def test_alg_minhash_256_returns_32_bytes():
    """Verify output is always 32 bytes."""
    result = alg_minhash_256([100, 200, 300])
    assert isinstance(result, bytes)
    assert len(result) == 32


def test_alg_minhash_256_empty_features():
    """Verify empty features return 32 bytes of 0xFF."""
    result = alg_minhash_256([])
    assert result == b"\xff" * 32


def test_alg_minhash_256_deterministic():
    """Verify same input produces same output."""
    features = [1, 2, 3, 4, 5]
    assert alg_minhash_256(features) == alg_minhash_256(features)


def test_alg_minhash_256_cross_validate_with_iscc_core():
    """Verify output matches iscc-core for known inputs."""
    features = [42, 1337, 9999, 123456]
    rust_result = alg_minhash_256(features)
    core_result = iscc_core.alg_minhash_256(features)
    assert rust_result == core_result


def test_alg_minhash_256_single_feature():
    """Verify single feature produces valid 32-byte digest."""
    rust_result = alg_minhash_256([1])
    core_result = iscc_core.alg_minhash_256([1])
    assert len(rust_result) == 32
    assert rust_result == core_result


# ── alg_cdc_chunks ─────────────────────────────────────────────────────────


def test_alg_cdc_chunks_empty_data():
    """Verify empty data returns list with one empty bytes."""
    result = alg_cdc_chunks(b"", False)
    assert result == [b""]


def test_alg_cdc_chunks_small_data():
    """Verify data smaller than min chunk size returns one chunk."""
    data = b"hello"
    result = alg_cdc_chunks(data, False)
    assert len(result) == 1
    assert result[0] == data


def test_alg_cdc_chunks_reassembly():
    """Verify chunks concatenate back to original data."""
    data = bytes(range(256)) * 16  # 4096 bytes
    chunks = alg_cdc_chunks(data, False)
    reassembled = b"".join(chunks)
    assert reassembled == data


def test_alg_cdc_chunks_default_avg_chunk_size():
    """Verify default avg_chunk_size parameter works (omit parameter)."""
    data = bytes(range(256)) * 16
    chunks_default = alg_cdc_chunks(data, False)
    chunks_explicit = alg_cdc_chunks(data, False, 1024)
    assert chunks_default == chunks_explicit


def test_alg_cdc_chunks_multiple_chunks():
    """Verify large data produces multiple chunks."""
    data = bytes(range(256)) * 32  # 8192 bytes
    chunks = alg_cdc_chunks(data, False)
    assert len(chunks) > 1


def test_alg_cdc_chunks_cross_validate_with_iscc_core():
    """Verify chunk boundaries match iscc-core for known input."""
    data = bytes(range(256)) * 16  # 4096 bytes
    rust_chunks = alg_cdc_chunks(data, False)
    core_chunks = list(iscc_core.alg_cdc_chunks(data, False))
    assert len(rust_chunks) == len(core_chunks)
    for r, c in zip(rust_chunks, core_chunks):
        assert r == c


def test_alg_cdc_chunks_utf32_cross_validate():
    """Verify UTF-32 mode chunk boundaries match iscc-core."""
    # Create data that looks like UTF-32 encoded text (4-byte aligned)
    data = bytes(range(256)) * 16
    rust_chunks = alg_cdc_chunks(data, True)
    core_chunks = list(iscc_core.alg_cdc_chunks(data, True))
    assert len(rust_chunks) == len(core_chunks)
    for r, c in zip(rust_chunks, core_chunks):
        assert r == c
