"""Smoke tests for the iscc_lib Python package."""

import json

from iscc_lib import (
    IsccResult,
    InstanceCodeResult,
    MetaCodeResult,
    TextCodeResult,
    gen_instance_code_v0,
    gen_meta_code_v0,
    gen_text_code_v0,
)


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
