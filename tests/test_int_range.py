"""Pin the exception class for out-of-range integer arguments.

PyO3 converts a Python `int` to a Rust integer type itself and raises
`OverflowError` when the value does not fit, before any binding code runs.
Video frame signatures are converted by the binding itself, which raises the
same class on every platform, whatever the width of the C `long`.
"""

import pytest

import iscc_lib


@pytest.mark.parametrize("bits", [-1, 2**32 + 64])
def test_gen_text_code_v0_out_of_range_bits_raises_overflow_error(bits):
    """Reject a `bits` value outside the u32 range with OverflowError."""
    with pytest.raises(OverflowError):
        iscc_lib.gen_text_code_v0("text", bits=bits)


@pytest.mark.parametrize("bits", [-1, 2**32 + 64])
def test_data_hasher_finalize_out_of_range_bits_raises_overflow_error(bits):
    """Reject an out-of-range `bits` on streaming finalize with OverflowError."""
    with pytest.raises(OverflowError):
        iscc_lib.DataHasher().finalize(bits=bits)


def test_text_trim_negative_nbytes_raises_overflow_error():
    """Reject a negative `nbytes` with OverflowError."""
    with pytest.raises(OverflowError):
        iscc_lib.text_trim("hello", -1)


def test_gen_audio_code_v0_out_of_range_element_raises_overflow_error():
    """Reject a feature outside the i32 range with OverflowError."""
    with pytest.raises(OverflowError):
        iscc_lib.gen_audio_code_v0([2**40])


VIDEO_OUT_OF_RANGE = [2**31, -(2**31) - 1, 2**63]
VIDEO_BOUNDARY = [2**31 - 1, -(2**31)]


def _frame(value):
    """Return one 380-value frame signature whose first value is `value`."""
    return [value] + [0] * 379


@pytest.mark.parametrize("value", VIDEO_OUT_OF_RANGE)
def test_gen_video_code_v0_out_of_range_value_raises_overflow_error(value):
    """Reject a frame value outside the i32 range with OverflowError."""
    frames = [_frame(value)]
    with pytest.raises(OverflowError):
        iscc_lib.gen_video_code_v0(frames)


@pytest.mark.parametrize("value", VIDEO_OUT_OF_RANGE)
def test_soft_hash_video_v0_out_of_range_value_raises_overflow_error(value):
    """Reject a frame value outside the i32 range with OverflowError."""
    frames = [_frame(value)]
    with pytest.raises(OverflowError):
        iscc_lib.soft_hash_video_v0(frames)


@pytest.mark.parametrize("value", VIDEO_BOUNDARY)
def test_gen_video_code_v0_i32_boundary_value_is_accepted(value):
    """Accept the i32 minimum and maximum as frame values."""
    result = iscc_lib.gen_video_code_v0([_frame(value)])
    assert isinstance(result, dict)
    assert result["iscc"].startswith("ISCC:")


@pytest.mark.parametrize("value", VIDEO_BOUNDARY)
def test_soft_hash_video_v0_i32_boundary_value_is_accepted(value):
    """Accept the i32 minimum and maximum as frame values."""
    digest = iscc_lib.soft_hash_video_v0([_frame(value)])
    assert isinstance(digest, bytes)
    assert len(digest) == 8
