"""Smoke tests for the iscc_lib Python package."""

import io
import json
import re
from pathlib import Path

import pytest

import iscc_lib
from iscc_lib import (
    DataCodeResult,
    IsccResult,
    InstanceCodeResult,
    MetaCodeResult,
    SumCodeResult,
    TextCodeResult,
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_meta_code_v0,
    gen_sum_code_v0,
    gen_text_code_v0,
)


def test_version_exists_and_correct():
    """Verify __version__ is a string matching the workspace version."""
    cargo_toml = Path(__file__).resolve().parent.parent / "Cargo.toml"
    match = re.search(r'^version\s*=\s*"(.+?)"', cargo_toml.read_text(), re.MULTILINE)
    assert match is not None, "version not found in Cargo.toml"
    expected = match.group(1)
    assert hasattr(iscc_lib, "__version__")
    assert isinstance(iscc_lib.__version__, str)
    assert iscc_lib.__version__ == expected


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


# ── gen_sum_code_v0 tests ──────────────────────────────────────────────────


def test_gen_sum_code_v0_equivalence(tmp_path):
    """Verify gen_sum_code_v0 matches gen_instance_code_v0 datahash and filesize."""
    data = b"Hello World"
    file = tmp_path / "test.bin"
    file.write_bytes(data)
    sum_result = gen_sum_code_v0(str(file))
    inst_result = gen_instance_code_v0(data)
    assert sum_result["datahash"] == inst_result["datahash"]
    assert sum_result["filesize"] == inst_result["filesize"]


def test_gen_sum_code_v0_pathlike(tmp_path):
    """Verify gen_sum_code_v0 accepts pathlib.Path via os.fspath conversion."""
    file = tmp_path / "pathlike.bin"
    file.write_bytes(b"test")
    result = gen_sum_code_v0(file)
    assert result["iscc"].startswith("ISCC:")


def test_gen_sum_code_v0_error_nonexistent(tmp_path):
    """Verify gen_sum_code_v0 raises ValueError for nonexistent path."""
    with pytest.raises(ValueError):
        gen_sum_code_v0(tmp_path / "missing.bin")


def test_gen_sum_code_v0_result_type(tmp_path):
    """Verify gen_sum_code_v0 returns SumCodeResult which is a dict."""
    file = tmp_path / "type.bin"
    file.write_bytes(b"data")
    result = gen_sum_code_v0(str(file))
    assert isinstance(result, SumCodeResult)
    assert isinstance(result, dict)


def test_gen_sum_code_v0_attribute_access(tmp_path):
    """Verify attribute-style access works on SumCodeResult."""
    file = tmp_path / "attr.bin"
    file.write_bytes(b"data")
    result = gen_sum_code_v0(str(file))
    assert result.iscc == result["iscc"]
    assert result.datahash == result["datahash"]
    assert result.filesize == result["filesize"]


def test_gen_sum_code_v0_wide_mode(tmp_path):
    """Verify wide=True produces a different ISCC string than wide=False."""
    file = tmp_path / "wide.bin"
    file.write_bytes(b"test data for wide mode")
    # Wide mode requires 128-bit+ codes to produce a different result
    result_normal = gen_sum_code_v0(str(file), bits=128, wide=False)
    result_wide = gen_sum_code_v0(str(file), bits=128, wide=True)
    assert result_normal["iscc"] != result_wide["iscc"]
