"""High-performance ISCC (ISO 24138:2024) implementation backed by Rust."""

from __future__ import annotations

import enum
import json as _json
from collections.abc import Sequence
from importlib.metadata import version
from types import SimpleNamespace
from typing import BinaryIO

__version__ = version("iscc-lib")

from iscc_lib._lowlevel import (
    IO_READ_SIZE as IO_READ_SIZE,
    META_TRIM_DESCRIPTION as META_TRIM_DESCRIPTION,
    META_TRIM_META as META_TRIM_META,
    META_TRIM_NAME as META_TRIM_NAME,
    TEXT_NGRAM_SIZE as TEXT_NGRAM_SIZE,
    DataHasher as _DataHasher,
    InstanceHasher as _InstanceHasher,
    alg_cdc_chunks as alg_cdc_chunks,
    alg_minhash_256 as alg_minhash_256,
    alg_simhash as alg_simhash,
    conformance_selftest as conformance_selftest,
    encode_base64 as encode_base64,
    encode_component as encode_component,
    gen_audio_code_v0 as _gen_audio_code_v0,
    gen_data_code_v0 as _gen_data_code_v0,
    gen_image_code_v0 as _gen_image_code_v0,
    gen_instance_code_v0 as _gen_instance_code_v0,
    gen_iscc_code_v0 as _gen_iscc_code_v0,
    gen_meta_code_v0 as _gen_meta_code_v0,
    gen_mixed_code_v0 as _gen_mixed_code_v0,
    gen_text_code_v0 as _gen_text_code_v0,
    gen_video_code_v0 as _gen_video_code_v0,
    iscc_decode as _iscc_decode,
    iscc_decompose as iscc_decompose,
    json_to_data_url as json_to_data_url,
    sliding_window as sliding_window,
    soft_hash_video_v0 as soft_hash_video_v0,
    text_clean as text_clean,
    text_collapse as text_collapse,
    text_remove_newlines as text_remove_newlines,
    text_trim as text_trim,
)


# ── Type Enums ──────────────────────────────────────────────────────────────


class MT(enum.IntEnum):
    """ISCC MainType identifiers."""

    META = 0
    SEMANTIC = 1
    CONTENT = 2
    DATA = 3
    INSTANCE = 4
    ISCC = 5
    ID = 6
    FLAKE = 7


class ST(enum.IntEnum):
    """ISCC SubType identifiers."""

    NONE = 0
    IMAGE = 1
    AUDIO = 2
    VIDEO = 3
    MIXED = 4
    SUM = 5
    ISCC_NONE = 6
    WIDE = 7
    TEXT = 0  # Alias — IntEnum allows duplicate values as aliases


class VS(enum.IntEnum):
    """ISCC Version identifiers."""

    V0 = 0


# ── Algorithm configuration namespace ───────────────────────────────────────

core_opts = SimpleNamespace(
    meta_trim_name=META_TRIM_NAME,
    meta_trim_description=META_TRIM_DESCRIPTION,
    meta_trim_meta=META_TRIM_META,
    io_read_size=IO_READ_SIZE,
    text_ngram_size=TEXT_NGRAM_SIZE,
)


# ── Codec helpers ───────────────────────────────────────────────────────────


def iscc_decode(iscc: str) -> tuple[MT, ST, VS, int, bytes]:
    """Decode an ISCC unit string into header components and raw digest."""
    mt, st, vs, length, digest = _iscc_decode(iscc)
    return MT(mt), ST(st), VS(vs), length, digest


_CHUNK_SIZE = 65536  # 64 KiB read chunks


# ── Result types ─────────────────────────────────────────────────────────────


class IsccResult(dict):
    """ISCC result with both dict-style and attribute-style access."""

    def __getattr__(self, name):
        try:
            return self[name]
        except KeyError:
            raise AttributeError(name) from None


class MetaCodeResult(IsccResult):
    """Result of gen_meta_code_v0."""

    iscc: str
    name: str
    metahash: str
    description: str | None
    meta: str | None


class TextCodeResult(IsccResult):
    """Result of gen_text_code_v0."""

    iscc: str
    characters: int


class ImageCodeResult(IsccResult):
    """Result of gen_image_code_v0."""

    iscc: str


class AudioCodeResult(IsccResult):
    """Result of gen_audio_code_v0."""

    iscc: str


class VideoCodeResult(IsccResult):
    """Result of gen_video_code_v0."""

    iscc: str


class MixedCodeResult(IsccResult):
    """Result of gen_mixed_code_v0."""

    iscc: str
    parts: list[str]


class DataCodeResult(IsccResult):
    """Result of gen_data_code_v0."""

    iscc: str


class InstanceCodeResult(IsccResult):
    """Result of gen_instance_code_v0."""

    iscc: str
    datahash: str
    filesize: int


class IsccCodeResult(IsccResult):
    """Result of gen_iscc_code_v0."""

    iscc: str


# ── Wrapper functions ────────────────────────────────────────────────────────


def gen_meta_code_v0(
    name: str,
    description: str | None = None,
    meta: str | dict | None = None,
    bits: int = 64,
) -> MetaCodeResult:
    """Generate an ISCC Meta-Code from content metadata."""
    if isinstance(meta, dict):
        meta = json_to_data_url(
            _json.dumps(meta, separators=(",", ":"), ensure_ascii=False)
        )
    return MetaCodeResult(_gen_meta_code_v0(name, description, meta, bits))


def gen_text_code_v0(text: str, bits: int = 64) -> TextCodeResult:
    """Generate an ISCC Text-Code from plain text content."""
    return TextCodeResult(_gen_text_code_v0(text, bits))


def gen_image_code_v0(
    pixels: bytes | bytearray | memoryview | Sequence[int], bits: int = 64
) -> ImageCodeResult:
    """Generate an ISCC Image-Code from pixel data."""
    if not isinstance(pixels, bytes):
        pixels = bytes(pixels)
    return ImageCodeResult(_gen_image_code_v0(pixels, bits))


def gen_audio_code_v0(cv: list[int], bits: int = 64) -> AudioCodeResult:
    """Generate an ISCC Audio-Code from a Chromaprint feature vector."""
    return AudioCodeResult(_gen_audio_code_v0(cv, bits))


def gen_video_code_v0(
    frame_sigs: Sequence[Sequence[int]], bits: int = 64
) -> VideoCodeResult:
    """Generate an ISCC Video-Code from frame signature data."""
    return VideoCodeResult(_gen_video_code_v0(frame_sigs, bits))


def gen_mixed_code_v0(codes: list[str], bits: int = 64) -> MixedCodeResult:
    """Generate an ISCC Mixed-Code from multiple Content-Code strings."""
    return MixedCodeResult(_gen_mixed_code_v0(codes, bits))


def gen_data_code_v0(
    data: bytes | bytearray | memoryview | BinaryIO, bits: int = 64
) -> DataCodeResult:
    """Generate an ISCC Data-Code from raw byte data or a file-like stream."""
    if not isinstance(data, (bytes, bytearray, memoryview)):
        hasher = _DataHasher()
        while chunk := data.read(_CHUNK_SIZE):
            hasher.update(chunk)
        return DataCodeResult(hasher.finalize(bits))
    if not isinstance(data, bytes):
        data = bytes(data)
    return DataCodeResult(_gen_data_code_v0(data, bits))


def gen_instance_code_v0(
    data: bytes | bytearray | memoryview | BinaryIO, bits: int = 64
) -> InstanceCodeResult:
    """Generate an ISCC Instance-Code from raw byte data or a file-like stream."""
    if not isinstance(data, (bytes, bytearray, memoryview)):
        hasher = _InstanceHasher()
        while chunk := data.read(_CHUNK_SIZE):
            hasher.update(chunk)
        return InstanceCodeResult(hasher.finalize(bits))
    if not isinstance(data, bytes):
        data = bytes(data)
    return InstanceCodeResult(_gen_instance_code_v0(data, bits))


def gen_iscc_code_v0(codes: list[str], wide: bool = False) -> IsccCodeResult:
    """Generate a composite ISCC-CODE from individual ISCC unit codes."""
    return IsccCodeResult(_gen_iscc_code_v0(codes, wide))


# ── Streaming hashers ──────────────────────────────────────────────────────


class DataHasher:
    """Streaming Data-Code generator.

    Incrementally processes data with content-defined chunking and MinHash
    to produce results identical to ``gen_data_code_v0``.
    """

    def __init__(
        self, data: bytes | bytearray | memoryview | BinaryIO | None = None
    ) -> None:
        """Create a new DataHasher with optional initial data."""
        self._inner = _DataHasher()
        if data is not None:
            self.update(data)

    def update(self, data: bytes | bytearray | memoryview | BinaryIO) -> None:
        """Push data into the hasher."""
        if not isinstance(data, (bytes, bytearray, memoryview)):
            while chunk := data.read(_CHUNK_SIZE):
                self._inner.update(chunk)
        else:
            if not isinstance(data, bytes):
                data = bytes(data)
            self._inner.update(data)

    def finalize(self, bits: int = 64) -> DataCodeResult:
        """Consume the hasher and return a Data-Code result."""
        return DataCodeResult(self._inner.finalize(bits))


class InstanceHasher:
    """Streaming Instance-Code generator.

    Incrementally hashes data with BLAKE3 to produce results identical
    to ``gen_instance_code_v0``.
    """

    def __init__(
        self, data: bytes | bytearray | memoryview | BinaryIO | None = None
    ) -> None:
        """Create a new InstanceHasher with optional initial data."""
        self._inner = _InstanceHasher()
        if data is not None:
            self.update(data)

    def update(self, data: bytes | bytearray | memoryview | BinaryIO) -> None:
        """Push data into the hasher."""
        if not isinstance(data, (bytes, bytearray, memoryview)):
            while chunk := data.read(_CHUNK_SIZE):
                self._inner.update(chunk)
        else:
            if not isinstance(data, bytes):
                data = bytes(data)
            self._inner.update(data)

    def finalize(self, bits: int = 64) -> InstanceCodeResult:
        """Consume the hasher and return an Instance-Code result."""
        return InstanceCodeResult(self._inner.finalize(bits))


__all__ = [
    "__version__",
    "IO_READ_SIZE",
    "META_TRIM_DESCRIPTION",
    "META_TRIM_META",
    "META_TRIM_NAME",
    "MT",
    "ST",
    "TEXT_NGRAM_SIZE",
    "VS",
    "IsccResult",
    "AudioCodeResult",
    "DataCodeResult",
    "DataHasher",
    "ImageCodeResult",
    "InstanceCodeResult",
    "InstanceHasher",
    "IsccCodeResult",
    "MetaCodeResult",
    "MixedCodeResult",
    "TextCodeResult",
    "VideoCodeResult",
    "alg_cdc_chunks",
    "alg_minhash_256",
    "alg_simhash",
    "conformance_selftest",
    "core_opts",
    "encode_base64",
    "encode_component",
    "gen_audio_code_v0",
    "gen_data_code_v0",
    "gen_image_code_v0",
    "gen_instance_code_v0",
    "gen_iscc_code_v0",
    "gen_meta_code_v0",
    "gen_mixed_code_v0",
    "gen_text_code_v0",
    "gen_video_code_v0",
    "iscc_decode",
    "iscc_decompose",
    "json_to_data_url",
    "sliding_window",
    "soft_hash_video_v0",
    "text_clean",
    "text_collapse",
    "text_remove_newlines",
    "text_trim",
]
