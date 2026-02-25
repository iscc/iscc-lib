"""Smoke tests for the iscc_lib Python package."""

import io
import json

import iscc_lib
from iscc_lib import (
    DataCodeResult,
    IsccResult,
    InstanceCodeResult,
    MetaCodeResult,
    TextCodeResult,
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_meta_code_v0,
    gen_text_code_v0,
)


def test_version_exists_and_correct():
    """Verify __version__ is a string matching the workspace version."""
    assert hasattr(iscc_lib, "__version__")
    assert isinstance(iscc_lib.__version__, str)
    assert iscc_lib.__version__ == "0.0.1"


def test_version_in_all():
    """Verify __version__ is included in __all__."""
    assert "__version__" in iscc_lib.__all__


def test_gen_instance_code_v0_empty():
    """Verify gen_instance_code_v0 returns dict with correct fields for empty bytes."""
    result = gen_instance_code_v0(b"", 64)
    assert isinstance(result, dict)
    assert result["iscc"] == "ISCC:IAA26E2JXH27TING"
    assert result["datahash"].startswith("1e20")
    assert result["filesize"] == 0


def test_gen_instance_code_v0_default_bits():
    """Verify gen_instance_code_v0 defaults to 64 bits."""
    result = gen_instance_code_v0(b"")
    assert result["iscc"] == "ISCC:IAA26E2JXH27TING"


def test_gen_instance_code_v0_hello():
    """Verify gen_instance_code_v0 with sample data."""
    result = gen_instance_code_v0(b"Hello World", 64)
    assert result["iscc"].startswith("ISCC:IAA")
    assert result["filesize"] == 11


def test_attribute_access():
    """Verify attribute-style access works on result objects."""
    result = gen_meta_code_v0("Test")
    assert result.iscc == result["iscc"]
    assert isinstance(result.iscc, str)
    assert result.iscc.startswith("ISCC:")


def test_isinstance_checks():
    """Verify isinstance checks for IsccResult hierarchy."""
    result = gen_instance_code_v0(b"")
    assert isinstance(result, dict)
    assert isinstance(result, IsccResult)
    assert isinstance(result, InstanceCodeResult)


def test_isinstance_meta_code():
    """Verify isinstance check for MetaCodeResult."""
    result = gen_meta_code_v0("Test")
    assert isinstance(result, dict)
    assert isinstance(result, IsccResult)
    assert isinstance(result, MetaCodeResult)


def test_json_serialization():
    """Verify json.dumps works on result objects."""
    result = gen_meta_code_v0("Test")
    serialized = json.dumps(result)
    parsed = json.loads(serialized)
    assert parsed["iscc"] == result["iscc"]
    assert parsed["name"] == result["name"]


def test_attribute_access_instance():
    """Verify attribute access on InstanceCodeResult-specific fields."""
    result = gen_instance_code_v0(b"")
    assert result.datahash.startswith("1e20")
    assert result.filesize == 0


def test_attribute_access_text():
    """Verify attribute access on TextCodeResult-specific fields."""
    result = gen_text_code_v0("test")
    assert isinstance(result, TextCodeResult)
    assert isinstance(result.characters, int)


def test_attribute_error_missing():
    """Verify AttributeError is raised for missing attributes."""
    result = gen_instance_code_v0(b"")
    try:
        _ = result.nonexistent
        assert False, "should have raised AttributeError"
    except AttributeError:
        pass


def test_dict_and_attribute_equal():
    """Verify dict access and attribute access return the same values."""
    result = gen_meta_code_v0("Test")
    assert result["iscc"] == result.iscc
    assert result["name"] == result.name
    assert result["metahash"] == result.metahash


# ── Streaming (BinaryIO) tests ─────────────────────────────────────────────


def test_gen_data_code_v0_stream_matches_bytes():
    """Verify gen_data_code_v0 with BytesIO produces same result as bytes."""
    data = b"Hello World"
    result_bytes = gen_data_code_v0(data)
    result_stream = gen_data_code_v0(io.BytesIO(data))
    assert result_stream["iscc"] == result_bytes["iscc"]
    assert isinstance(result_stream, DataCodeResult)


def test_gen_instance_code_v0_stream_matches_bytes():
    """Verify gen_instance_code_v0 with BytesIO produces same result as bytes."""
    data = b"Hello World"
    result_bytes = gen_instance_code_v0(data)
    result_stream = gen_instance_code_v0(io.BytesIO(data))
    assert result_stream["iscc"] == result_bytes["iscc"]
    assert result_stream["datahash"] == result_bytes["datahash"]
    assert result_stream["filesize"] == result_bytes["filesize"]
    assert isinstance(result_stream, InstanceCodeResult)


def test_gen_data_code_v0_empty_stream():
    """Verify gen_data_code_v0 handles empty BytesIO correctly."""
    result_bytes = gen_data_code_v0(b"")
    result_stream = gen_data_code_v0(io.BytesIO(b""))
    assert result_stream["iscc"] == result_bytes["iscc"]


def test_gen_instance_code_v0_empty_stream():
    """Verify gen_instance_code_v0 handles empty BytesIO correctly."""
    result_bytes = gen_instance_code_v0(b"")
    result_stream = gen_instance_code_v0(io.BytesIO(b""))
    assert result_stream["iscc"] == result_bytes["iscc"]
    assert result_stream["datahash"] == result_bytes["datahash"]
    assert result_stream["filesize"] == result_bytes["filesize"]


def test_gen_data_code_v0_bytes_still_works():
    """Verify gen_data_code_v0 still accepts plain bytes after adding stream support."""
    result = gen_data_code_v0(b"test data")
    assert result["iscc"].startswith("ISCC:")
    assert isinstance(result, DataCodeResult)


def test_gen_instance_code_v0_bytes_still_works():
    """Verify gen_instance_code_v0 still accepts plain bytes after adding stream support."""
    result = gen_instance_code_v0(b"test data")
    assert result["iscc"].startswith("ISCC:")
    assert result["filesize"] == 9
    assert isinstance(result, InstanceCodeResult)
