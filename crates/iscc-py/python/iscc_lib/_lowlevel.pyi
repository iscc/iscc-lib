"""Type stubs for the native Rust extension module `iscc_lib._lowlevel`."""

from collections.abc import Sequence
from typing import Any

def conformance_selftest() -> bool:
    """Run all conformance tests against vendored test vectors.

    Iterates through all 9 ``gen_*_v0`` function sections in the conformance
    data, calls each function with the specified inputs, and compares results
    against expected output. Returns ``True`` if all tests pass.

    :return: ``True`` if all conformance tests pass, ``False`` otherwise.
    """
    ...

def text_clean(text: str) -> str:
    """Clean and normalize text for display.

    Applies NFKC normalization, removes control characters (except newlines),
    normalizes ``\\r\\n`` to ``\\n``, collapses consecutive empty lines to at
    most one, and strips leading/trailing whitespace.

    :param text: Input text to clean.
    :return: Cleaned text.
    """
    ...

def text_remove_newlines(text: str) -> str:
    """Remove newlines and collapse whitespace to single spaces.

    Converts multi-line text into a single normalized line by splitting on
    whitespace boundaries and joining with a single space.

    :param text: Input text with newlines.
    :return: Single-line text with collapsed whitespace.
    """
    ...

def text_trim(text: str, nbytes: int) -> str:
    """Trim text so its UTF-8 encoded size does not exceed ``nbytes``.

    Finds the largest valid UTF-8 prefix within ``nbytes``, then strips
    leading/trailing whitespace from the result. Multi-byte characters that
    would be split are dropped entirely.

    :param text: Input text to trim.
    :param nbytes: Maximum byte length of the result.
    :return: Trimmed text.
    """
    ...

def text_collapse(text: str) -> str:
    """Normalize and simplify text for similarity hashing.

    Applies NFD normalization, lowercasing, removes whitespace and characters
    in Unicode categories C (control), M (mark), and P (punctuation), then
    recombines with NFKC normalization.

    :param text: Input text to collapse.
    :return: Collapsed text suitable for similarity hashing.
    """
    ...

def encode_base64(data: bytes) -> str:
    """Encode bytes as base64url (RFC 4648 §5, no padding).

    Returns a URL-safe base64 encoded string without padding characters.

    :param data: Raw bytes to encode.
    :return: Base64url encoded string without padding.
    """
    ...

def iscc_decompose(iscc_code: str) -> list[str]:
    """Decompose a composite ISCC-CODE into individual ISCC-UNITs.

    Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
    The optional ``"ISCC:"`` prefix is stripped before decoding. Returns
    a list of base32-encoded ISCC-UNIT strings (without prefix).

    :param iscc_code: ISCC-CODE or ISCC-UNIT sequence string.
    :return: List of base32-encoded ISCC-UNIT strings.
    :raises ValueError: If the input is not a valid ISCC string.
    """
    ...

def sliding_window(seq: str, width: int) -> list[str]:
    """Generate sliding window n-grams from a string.

    Returns overlapping substrings of ``width`` Unicode characters,
    advancing by one character at a time. If the input is shorter than
    ``width``, returns a single element containing the full input.

    :param seq: Input string to slide over.
    :param width: Width of sliding window (must be >= 2).
    :return: List of window-sized substrings.
    :raises ValueError: If ``width`` is less than 2.
    """
    ...

def gen_meta_code_v0(
    name: str,
    description: str | None = None,
    meta: str | None = None,
    bits: int = 64,
) -> dict[str, Any]:
    """Generate an ISCC Meta-Code from content metadata.

    Produces a similarity-preserving fingerprint by hashing the provided name,
    description, and metadata fields using SimHash. When ``meta`` is provided,
    it is treated as a Data-URL (if starting with ``"data:"``) or a JSON string.

    :param name: Title or name of the content (max 128 chars after normalization).
    :param description: Optional text description (max 4096 chars after normalization).
    :param meta: Optional Data-URL or JSON metadata for deterministic encoding.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc``, ``name``, ``metahash``, and optionally
        ``description`` and ``meta`` keys.
    """
    ...

def gen_text_code_v0(text: str, bits: int = 64) -> dict[str, Any]:
    """Generate an ISCC Text-Code from plain text content.

    Produces a Content-Code for text by collapsing the input, extracting
    character n-gram features, and applying MinHash to create a
    similarity-preserving fingerprint.

    :param text: Plain text content to fingerprint.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` and ``characters`` keys.
    """
    ...

def gen_image_code_v0(pixels: bytes, bits: int = 64) -> dict[str, Any]:
    """Generate an ISCC Image-Code from pixel data.

    Produces a Content-Code for images from a sequence of 1024 grayscale
    pixel values (32x32, values 0-255) using a DCT-based perceptual hash.

    :param pixels: Raw grayscale pixel bytes (exactly 1024 bytes for 32x32).
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` key.
    """
    ...

def gen_audio_code_v0(cv: list[int], bits: int = 64) -> dict[str, Any]:
    """Generate an ISCC Audio-Code from a Chromaprint feature vector.

    Produces a Content-Code for audio from a Chromaprint signed integer
    fingerprint vector using multi-stage SimHash.

    :param cv: Chromaprint signed integer (i32) feature vector.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` key.
    """
    ...

def gen_video_code_v0(
    frame_sigs: Sequence[Sequence[int]], bits: int = 64
) -> dict[str, Any]:
    """Generate an ISCC Video-Code from frame signature data.

    Produces a Content-Code for video from a sequence of frame signatures.
    Each frame signature is a list of signed integers (typically 380 elements).
    Deduplicates frames and applies WTA-Hash.

    :param frame_sigs: List of frame signatures, each a list of signed integers.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` key.
    """
    ...

def gen_mixed_code_v0(codes: list[str], bits: int = 64) -> dict[str, Any]:
    """Generate an ISCC Mixed-Code from multiple Content-Code strings.

    Produces a Mixed Content-Code by combining multiple ISCC Content-Codes of
    different types (text, image, audio, video) using SimHash. Input codes may
    optionally include the ``"ISCC:"`` prefix.

    :param codes: List of ISCC Content-Code strings to combine.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` and ``parts`` keys.
    """
    ...

def gen_data_code_v0(data: bytes, bits: int = 64) -> dict[str, Any]:
    """Generate an ISCC Data-Code from raw byte data.

    Produces a Data-Code by splitting data into content-defined chunks,
    hashing each chunk, and applying MinHash to create a similarity-preserving
    fingerprint.

    :param data: Raw binary data to fingerprint.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` key.
    """
    ...

def gen_instance_code_v0(data: bytes, bits: int = 64) -> dict[str, Any]:
    """Generate an ISCC Instance-Code from raw byte data.

    Produces an Instance-Code by hashing the complete byte stream with BLAKE3.
    Captures the exact binary identity of the data.

    :param data: Raw binary data to hash.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc``, ``datahash``, and ``filesize`` keys.
    """
    ...

def alg_simhash(hash_digests: list[bytes]) -> bytes:
    """Compute a SimHash from a sequence of equal-length hash digests.

    Produces a similarity-preserving hash by counting bit frequencies across
    all input digests. Each output bit is set when its frequency meets or
    exceeds half the input count. Returns 32 zero bytes for empty input.

    :param hash_digests: List of equal-length byte hash digests.
    :return: Similarity-preserving hash as bytes (same length as input digests).
    """
    ...

def alg_minhash_256(features: list[int]) -> bytes:
    """Compute a 256-bit MinHash digest from 32-bit integer features.

    Uses 64 universal hash functions with bit-interleaved compression to
    produce a 32-byte similarity-preserving digest.

    :param features: List of 32-bit unsigned integer features.
    :return: 32-byte MinHash digest.
    """
    ...

def alg_cdc_chunks(data: bytes, utf32: bool, avg_chunk_size: int = 1024) -> list[bytes]:
    """Split data into content-defined chunks using gear rolling hash.

    Uses a FastCDC-inspired algorithm to find content-dependent boundaries.
    Returns at least one chunk (empty bytes for empty input). When ``utf32``
    is true, aligns cut points to 4-byte boundaries.

    :param data: Raw binary data to chunk.
    :param utf32: Whether to align cut points to 4-byte boundaries.
    :param avg_chunk_size: Target average chunk size in bytes (default 1024).
    :return: List of byte chunks that concatenate to the original data.
    """
    ...

def gen_iscc_code_v0(codes: list[str], wide: bool = False) -> dict[str, Any]:
    """Generate a composite ISCC-CODE from individual ISCC unit codes.

    Combines multiple ISCC unit codes (Meta-Code, Content-Code, Data-Code,
    Instance-Code) into a single composite ISCC-CODE. At least two codes are
    required (Data-Code and Instance-Code). Input codes may optionally include
    the ``"ISCC:"`` prefix. When ``wide`` is true and exactly two 128-bit+
    codes are provided, produces a 256-bit wide-mode code.

    :param codes: List of ISCC unit code strings to combine.
    :param wide: Whether to produce a wide (256-bit) code (default False).
    :return: Dict with ``iscc`` key.
    """
    ...

def soft_hash_video_v0(frame_sigs: Sequence[Sequence[int]], bits: int = 64) -> bytes:
    """Compute a similarity-preserving hash from video frame signatures.

    Deduplicates frame signatures, computes column-wise sums across all
    unique frames, then applies WTA-Hash to produce a digest.

    :param frame_sigs: List of frame signatures, each a list of signed integers.
    :param bits: Bit length of the output hash (default 64).
    :return: Raw hash bytes of length ``bits / 8``.
    :raises ValueError: If ``frame_sigs`` is empty.
    """
    ...

def gen_video_code_v0_flat(
    data: bytes, num_frames: int, frame_len: int, bits: int = 64
) -> dict[str, Any]:
    """Generate a Video-Code from a flat byte buffer of i32 frame signatures.

    Accepts pre-flattened frame data as raw bytes (native-endian i32 values)
    for callers that already have typed data (numpy, array.array).

    :param data: Flat byte buffer of native-endian i32 frame signature values.
    :param num_frames: Number of frames in the buffer.
    :param frame_len: Number of i32 elements per frame.
    :param bits: Bit length of the code body (default 64).
    :return: Dict with ``iscc`` key.
    """
    ...

def soft_hash_video_v0_flat(
    data: bytes, num_frames: int, frame_len: int, bits: int = 64
) -> bytes:
    """Compute a video hash from a flat byte buffer of i32 frame signatures.

    Accepts pre-flattened frame data as raw bytes (native-endian i32 values)
    for callers that already have typed data (numpy, array.array).

    :param data: Flat byte buffer of native-endian i32 frame signature values.
    :param num_frames: Number of frames in the buffer.
    :param frame_len: Number of i32 elements per frame.
    :param bits: Bit length of the output hash (default 64).
    :return: Raw hash bytes of length ``bits / 8``.
    """
    ...

class DataHasher:
    """Streaming Data-Code generator backed by Rust.

    Incrementally processes data with content-defined chunking and MinHash
    to produce results identical to ``gen_data_code_v0``.
    """

    def __init__(self) -> None:
        """Create a new DataHasher."""
        ...

    def update(self, data: bytes) -> None:
        """Push data into the hasher.

        :param data: Raw binary data to process.
        :raises ValueError: If the hasher has already been finalized.
        """
        ...

    def finalize(self, bits: int = 64) -> dict[str, Any]:
        """Consume the hasher and produce a Data-Code result dict.

        :param bits: Bit length of the code body (default 64).
        :return: Dict with ``iscc`` key.
        :raises ValueError: If the hasher has already been finalized.
        """
        ...

class InstanceHasher:
    """Streaming Instance-Code generator backed by Rust.

    Incrementally hashes data with BLAKE3 to produce results identical
    to ``gen_instance_code_v0``.
    """

    def __init__(self) -> None:
        """Create a new InstanceHasher."""
        ...

    def update(self, data: bytes) -> None:
        """Push data into the hasher.

        :param data: Raw binary data to process.
        :raises ValueError: If the hasher has already been finalized.
        """
        ...

    def finalize(self, bits: int = 64) -> dict[str, Any]:
        """Consume the hasher and produce an Instance-Code result dict.

        :param bits: Bit length of the code body (default 64).
        :return: Dict with ``iscc``, ``datahash``, and ``filesize`` keys.
        :raises ValueError: If the hasher has already been finalized.
        """
        ...
