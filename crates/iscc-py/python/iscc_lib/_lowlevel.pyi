"""Type stubs for the native Rust extension module `iscc_lib._lowlevel`."""

def gen_meta_code_v0(
    name: str,
    description: str | None = None,
    meta: str | None = None,
    bits: int = 64,
) -> str:
    """Generate an ISCC Meta-Code from content metadata.

    Produces a similarity-preserving fingerprint by hashing the provided name,
    description, and metadata fields using SimHash. When ``meta`` is provided,
    it is treated as a Data-URL (if starting with ``"data:"``) or a JSON string.

    :param name: Title or name of the content (max 128 chars after normalization).
    :param description: Optional text description (max 4096 chars after normalization).
    :param meta: Optional Data-URL or JSON metadata for deterministic encoding.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing ``iscc`` and ``metahash`` fields.
    """
    ...

def gen_text_code_v0(text: str, bits: int = 64) -> str:
    """Generate an ISCC Text-Code from plain text content.

    Produces a Content-Code for text by collapsing the input, extracting
    character n-gram features, and applying MinHash to create a
    similarity-preserving fingerprint.

    :param text: Plain text content to fingerprint.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_image_code_v0(pixels: bytes, bits: int = 64) -> str:
    """Generate an ISCC Image-Code from pixel data.

    Produces a Content-Code for images from a sequence of 1024 grayscale
    pixel values (32x32, values 0-255) using a DCT-based perceptual hash.

    :param pixels: Raw grayscale pixel bytes (exactly 1024 bytes for 32x32).
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_audio_code_v0(cv: list[int], bits: int = 64) -> str:
    """Generate an ISCC Audio-Code from a Chromaprint feature vector.

    Produces a Content-Code for audio from a Chromaprint signed integer
    fingerprint vector using multi-stage SimHash.

    :param cv: Chromaprint signed integer (i32) feature vector.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_video_code_v0(frame_sigs: list[list[int]], bits: int = 64) -> str:
    """Generate an ISCC Video-Code from frame signature data.

    Produces a Content-Code for video from a sequence of frame signatures.
    Each frame signature is a list of signed integers (typically 380 elements).
    Deduplicates frames and applies WTA-Hash.

    :param frame_sigs: List of frame signatures, each a list of signed integers.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_mixed_code_v0(codes: list[str], bits: int = 64) -> str:
    """Generate an ISCC Mixed-Code from multiple Content-Code strings.

    Produces a Mixed Content-Code by combining multiple ISCC Content-Codes of
    different types (text, image, audio, video) using SimHash. Input codes may
    optionally include the ``"ISCC:"`` prefix.

    :param codes: List of ISCC Content-Code strings to combine.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_data_code_v0(data: bytes, bits: int = 64) -> str:
    """Generate an ISCC Data-Code from raw byte data.

    Produces a Data-Code by splitting data into content-defined chunks,
    hashing each chunk, and applying MinHash to create a similarity-preserving
    fingerprint.

    :param data: Raw binary data to fingerprint.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_instance_code_v0(data: bytes, bits: int = 64) -> str:
    """Generate an ISCC Instance-Code from raw byte data.

    Produces an Instance-Code by hashing the complete byte stream with BLAKE3.
    Captures the exact binary identity of the data.

    :param data: Raw binary data to hash.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing the ``iscc`` field.
    """
    ...

def gen_iscc_code_v0(codes: list[str], wide: bool = False) -> str:
    """Generate a composite ISCC-CODE from individual ISCC unit codes.

    Combines multiple ISCC unit codes (Meta-Code, Content-Code, Data-Code,
    Instance-Code) into a single composite ISCC-CODE. At least two codes are
    required (Data-Code and Instance-Code). Input codes may optionally include
    the ``"ISCC:"`` prefix. When ``wide`` is true and exactly two 128-bit+
    codes are provided, produces a 256-bit wide-mode code.

    :param codes: List of ISCC unit code strings to combine.
    :param wide: Whether to produce a wide (256-bit) code (default False).
    :return: JSON string containing the ``iscc`` field.
    """
    ...
