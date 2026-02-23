"""High-performance ISCC (ISO 24138:2024) implementation backed by Rust."""

from __future__ import annotations

from typing import BinaryIO

from iscc_lib._lowlevel import (
    conformance_selftest as conformance_selftest,
    gen_audio_code_v0 as _gen_audio_code_v0,
    gen_data_code_v0 as _gen_data_code_v0,
    gen_image_code_v0 as _gen_image_code_v0,
    gen_instance_code_v0 as _gen_instance_code_v0,
    gen_iscc_code_v0 as _gen_iscc_code_v0,
    gen_meta_code_v0 as _gen_meta_code_v0,
    gen_mixed_code_v0 as _gen_mixed_code_v0,
    gen_text_code_v0 as _gen_text_code_v0,
    gen_video_code_v0 as _gen_video_code_v0,
    text_clean as text_clean,
    text_collapse as text_collapse,
    text_remove_newlines as text_remove_newlines,
    text_trim as text_trim,
)


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
    meta: str | None = None,
    bits: int = 64,
) -> MetaCodeResult:
    """Generate an ISCC Meta-Code from content metadata."""
    return MetaCodeResult(_gen_meta_code_v0(name, description, meta, bits))


def gen_text_code_v0(text: str, bits: int = 64) -> TextCodeResult:
    """Generate an ISCC Text-Code from plain text content."""
    return TextCodeResult(_gen_text_code_v0(text, bits))


def gen_image_code_v0(pixels: bytes, bits: int = 64) -> ImageCodeResult:
    """Generate an ISCC Image-Code from pixel data."""
    return ImageCodeResult(_gen_image_code_v0(pixels, bits))


def gen_audio_code_v0(cv: list[int], bits: int = 64) -> AudioCodeResult:
    """Generate an ISCC Audio-Code from a Chromaprint feature vector."""
    return AudioCodeResult(_gen_audio_code_v0(cv, bits))


def gen_video_code_v0(frame_sigs: list[list[int]], bits: int = 64) -> VideoCodeResult:
    """Generate an ISCC Video-Code from frame signature data."""
    return VideoCodeResult(_gen_video_code_v0(frame_sigs, bits))


def gen_mixed_code_v0(codes: list[str], bits: int = 64) -> MixedCodeResult:
    """Generate an ISCC Mixed-Code from multiple Content-Code strings."""
    return MixedCodeResult(_gen_mixed_code_v0(codes, bits))


def gen_data_code_v0(data: bytes | BinaryIO, bits: int = 64) -> DataCodeResult:
    """Generate an ISCC Data-Code from raw byte data or a file-like stream."""
    if not isinstance(data, bytes):
        data = data.read()
    return DataCodeResult(_gen_data_code_v0(data, bits))


def gen_instance_code_v0(data: bytes | BinaryIO, bits: int = 64) -> InstanceCodeResult:
    """Generate an ISCC Instance-Code from raw byte data or a file-like stream."""
    if not isinstance(data, bytes):
        data = data.read()
    return InstanceCodeResult(_gen_instance_code_v0(data, bits))


def gen_iscc_code_v0(codes: list[str], wide: bool = False) -> IsccCodeResult:
    """Generate a composite ISCC-CODE from individual ISCC unit codes."""
    return IsccCodeResult(_gen_iscc_code_v0(codes, wide))


__all__ = [
    "IsccResult",
    "AudioCodeResult",
    "DataCodeResult",
    "ImageCodeResult",
    "InstanceCodeResult",
    "IsccCodeResult",
    "MetaCodeResult",
    "MixedCodeResult",
    "TextCodeResult",
    "VideoCodeResult",
    "conformance_selftest",
    "gen_audio_code_v0",
    "gen_data_code_v0",
    "gen_image_code_v0",
    "gen_instance_code_v0",
    "gen_iscc_code_v0",
    "gen_meta_code_v0",
    "gen_mixed_code_v0",
    "gen_text_code_v0",
    "gen_video_code_v0",
    "text_clean",
    "text_collapse",
    "text_remove_newlines",
    "text_trim",
]
