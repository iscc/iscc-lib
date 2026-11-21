"""Pin the exception class for out-of-range integer arguments.

PyO3 converts a Python `int` to a Rust integer type itself and raises
`OverflowError` when the value does not fit, before any binding code runs.
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
