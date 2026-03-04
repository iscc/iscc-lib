# frozen_string_literal: true

# Tests for DataHasher and InstanceHasher streaming types.
# Verifies the new → update → finalize interface and equivalence with one-shot functions.

require "test_helper"

class TestDataHasher < Minitest::Test
  def test_basic_usage
    hasher = IsccLib::DataHasher.new
    hasher.update("Hello ISCC World!".b)
    result = hasher.finalize
    assert_kind_of IsccLib::DataCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_streaming_matches_oneshot
    data = ("Hello World" * 100).b
    oneshot = IsccLib.gen_data_code_v0(data)
    hasher = IsccLib::DataHasher.new
    hasher.update(data)
    streaming = hasher.finalize
    assert_equal oneshot["iscc"], streaming["iscc"],
      "streaming result should match one-shot result"
  end

  def test_multi_update_matches_oneshot
    data = ("Streaming test data for CDC chunking" * 100).b
    oneshot = IsccLib.gen_data_code_v0(data)
    hasher = IsccLib::DataHasher.new
    # Split data across multiple update calls
    hasher.update(data[0, 500])
    hasher.update(data[500, 1000])
    hasher.update(data[1500..])
    streaming = hasher.finalize
    assert_equal oneshot["iscc"], streaming["iscc"],
      "multi-update result should match one-shot result"
  end

  def test_double_finalize_error
    hasher = IsccLib::DataHasher.new
    hasher.update("test".b)
    hasher.finalize
    assert_raises(RuntimeError) { hasher.finalize }
  end

  def test_update_after_finalize_error
    hasher = IsccLib::DataHasher.new
    hasher.update("test".b)
    hasher.finalize
    assert_raises(RuntimeError) { hasher.update("more".b) }
  end

  def test_method_chaining
    data1 = ("chunk one " * 50).b
    data2 = ("chunk two " * 50).b
    result = IsccLib::DataHasher.new.update(data1).update(data2).finalize
    assert_kind_of IsccLib::DataCodeResult, result
    assert result.iscc.start_with?("ISCC:")
  end

  def test_attribute_access
    hasher = IsccLib::DataHasher.new
    hasher.update("attribute test".b)
    result = hasher.finalize
    assert result.iscc.start_with?("ISCC:")
    assert result.iscc.is_a?(String)
  end
end

class TestInstanceHasher < Minitest::Test
  def test_basic_usage
    hasher = IsccLib::InstanceHasher.new
    hasher.update("Hello ISCC World!".b)
    result = hasher.finalize
    assert_kind_of IsccLib::InstanceCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_result_has_datahash_and_filesize
    data = ("Instance test data" * 50).b
    hasher = IsccLib::InstanceHasher.new
    hasher.update(data)
    result = hasher.finalize
    assert result.key?("datahash"), "result should contain datahash"
    assert result.key?("filesize"), "result should contain filesize"
    assert result["datahash"].is_a?(String)
    assert_equal data.bytesize, result["filesize"]
  end

  def test_streaming_matches_oneshot
    data = ("Hello Instance" * 100).b
    oneshot = IsccLib.gen_instance_code_v0(data)
    hasher = IsccLib::InstanceHasher.new
    hasher.update(data)
    streaming = hasher.finalize
    assert_equal oneshot["iscc"], streaming["iscc"],
      "streaming ISCC should match one-shot"
    assert_equal oneshot["datahash"], streaming["datahash"],
      "streaming datahash should match one-shot"
    assert_equal oneshot["filesize"], streaming["filesize"],
      "streaming filesize should match one-shot"
  end

  def test_multi_update_matches_oneshot
    data = ("Streaming instance hash" * 100).b
    oneshot = IsccLib.gen_instance_code_v0(data)
    hasher = IsccLib::InstanceHasher.new
    hasher.update(data[0, 500])
    hasher.update(data[500, 1000])
    hasher.update(data[1500..])
    streaming = hasher.finalize
    assert_equal oneshot["iscc"], streaming["iscc"],
      "multi-update result should match one-shot"
    assert_equal oneshot["datahash"], streaming["datahash"]
    assert_equal oneshot["filesize"], streaming["filesize"]
  end

  def test_double_finalize_error
    hasher = IsccLib::InstanceHasher.new
    hasher.update("test".b)
    hasher.finalize
    assert_raises(RuntimeError) { hasher.finalize }
  end

  def test_update_after_finalize_error
    hasher = IsccLib::InstanceHasher.new
    hasher.update("test".b)
    hasher.finalize
    assert_raises(RuntimeError) { hasher.update("more".b) }
  end

  def test_method_chaining
    data1 = ("chunk one " * 50).b
    data2 = ("chunk two " * 50).b
    result = IsccLib::InstanceHasher.new.update(data1).update(data2).finalize
    assert_kind_of IsccLib::InstanceCodeResult, result
    assert result.iscc.start_with?("ISCC:")
    assert result.key?("datahash")
    assert result.key?("filesize")
  end

  def test_attribute_access
    data = ("attribute test" * 50).b
    hasher = IsccLib::InstanceHasher.new
    hasher.update(data)
    result = hasher.finalize
    assert result.iscc.start_with?("ISCC:")
    assert result.datahash.is_a?(String)
    assert_equal data.bytesize, result.filesize
  end
end
