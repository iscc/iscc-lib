# frozen_string_literal: true

# Smoke tests for iscc-lib Ruby bindings.
# Verifies the Magnus bridge loads and core functions produce valid output.

require "test_helper"

class TestSmoke < Minitest::Test
  def test_gen_meta_code_v0_basic
    result = IsccLib.gen_meta_code_v0("Hello World")
    assert_kind_of IsccLib::MetaCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
    assert_equal "Hello World", result["name"]
    assert result.key?("metahash"), "Result should contain metahash"
  end

  def test_gen_meta_code_v0_attribute_access
    result = IsccLib.gen_meta_code_v0("Test")
    assert result.iscc.start_with?("ISCC:")
    assert_equal "Test", result.name
    assert result.metahash.is_a?(String)
  end

  def test_gen_meta_code_v0_with_description
    result = IsccLib.gen_meta_code_v0("Title", description: "A description")
    assert result.key?("description")
    assert_equal "A description", result["description"]
  end

  def test_gen_meta_code_v0_without_description
    result = IsccLib.gen_meta_code_v0("Title")
    refute result.key?("description"), "description should be omitted when nil"
  end

  def test_text_clean
    cleaned = IsccLib.text_clean("  Hello  World  ")
    assert_equal "Hello  World", cleaned
  end

  def test_text_remove_newlines
    result = IsccLib.text_remove_newlines("Hello\nWorld")
    assert_equal "Hello World", result
  end

  def test_text_trim
    result = IsccLib.text_trim("Hello World", 5)
    assert result.bytesize <= 5
  end

  def test_text_collapse
    result = IsccLib.text_collapse("Hello, World!")
    assert result.is_a?(String)
    refute result.include?(","), "Punctuation should be removed"
    refute result.include?("!"), "Punctuation should be removed"
  end

  def test_constants
    assert_equal 128, IsccLib::META_TRIM_NAME
    assert_equal 4096, IsccLib::META_TRIM_DESCRIPTION
    assert_equal 128_000, IsccLib::META_TRIM_META
    assert_equal 4_194_304, IsccLib::IO_READ_SIZE
    assert_equal 13, IsccLib::TEXT_NGRAM_SIZE
  end

  def test_version
    assert IsccLib::VERSION.is_a?(String)
    assert_match(/\A\d+\.\d+\.\d+\z/, IsccLib::VERSION)
  end
end
