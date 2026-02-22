"""Smoke tests for the iscc Python package."""

from iscc_lib import gen_instance_code_v0


def test_gen_instance_code_v0_empty():
    """Verify gen_instance_code_v0 produces correct ISCC for empty bytes."""
    result = gen_instance_code_v0(b"", 64)
    assert result == "ISCC:IAA26E2JXH27TING"


def test_gen_instance_code_v0_default_bits():
    """Verify gen_instance_code_v0 defaults to 64 bits."""
    result = gen_instance_code_v0(b"")
    assert result == "ISCC:IAA26E2JXH27TING"


def test_gen_instance_code_v0_hello():
    """Verify gen_instance_code_v0 with sample data."""
    result = gen_instance_code_v0(b"Hello World", 64)
    assert result.startswith("ISCC:IAA")
