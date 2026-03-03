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

  def test_encode_base64
    result = IsccLib.encode_base64("Hello".b)
    assert result.is_a?(String)
    refute result.empty?, "base64 output should not be empty"
    assert_equal "SGVsbG8", result
  end

  def test_iscc_decompose
    units = IsccLib.iscc_decompose("ISCC:AAAWKLHFXM75OAMK")
    assert units.is_a?(Array), "iscc_decompose should return an Array"
    assert units.length.positive?, "should contain at least one unit"
    assert units.all? { |u| u.is_a?(String) }, "all units should be Strings"
  end

  def test_iscc_decompose_error
    assert_raises(RuntimeError) { IsccLib.iscc_decompose("INVALID") }
  end

  def test_encode_component
    digest = ("\x00" * 8).b
    result = IsccLib.encode_component(0, 0, 0, 64, digest)
    assert result.is_a?(String)
    refute result.empty?
  end

  def test_iscc_decode
    result = IsccLib.iscc_decode("AAAWKLHFXM75OAMK")
    assert result.is_a?(Array), "iscc_decode should return an Array"
    assert_equal 5, result.length, "should have 5 elements"
    assert result[0].is_a?(Integer), "maintype should be an integer"
    assert result[1].is_a?(Integer), "subtype should be an integer"
    assert result[2].is_a?(Integer), "version should be an integer"
    assert result[3].is_a?(Integer), "length_index should be an integer"
    assert result[4].is_a?(String), "digest should be a string"
  end

  def test_encode_decode_roundtrip
    digest = ("\xAB\xCD" * 4).b
    encoded = IsccLib.encode_component(0, 0, 0, 64, digest)
    decoded = IsccLib.iscc_decode(encoded)
    assert_equal 0, decoded[0], "maintype should round-trip"
    assert_equal 0, decoded[1], "subtype should round-trip"
    assert_equal 0, decoded[2], "version should round-trip"
    assert_equal digest[0, 8], decoded[4], "digest should round-trip"
  end

  def test_json_to_data_url
    result = IsccLib.json_to_data_url('{"key":"value"}')
    assert result.start_with?("data:application/json;base64,")
  end

  def test_json_to_data_url_ld
    result = IsccLib.json_to_data_url('{"@context":"https://schema.org"}')
    assert result.start_with?("data:application/ld+json;base64,")
  end

  def test_conformance_selftest
    assert_equal true, IsccLib.conformance_selftest
  end

  def test_version
    assert IsccLib::VERSION.is_a?(String)
    assert_match(/\A\d+\.\d+\.\d+\z/, IsccLib::VERSION)
  end
end
