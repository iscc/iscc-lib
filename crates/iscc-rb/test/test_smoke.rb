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
    refute_includes result, ",", "Punctuation should be removed"
    refute_includes result, "!", "Punctuation should be removed"
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
    refute_empty result, "base64 output should not be empty"
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
    refute_empty result
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
    assert IsccLib.conformance_selftest
  end

  def test_gen_text_code_v0_basic
    result = IsccLib.gen_text_code_v0("Hello World")
    assert_kind_of IsccLib::TextCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
    assert result.key?("characters"), "Result should contain characters"
  end

  def test_gen_text_code_v0_attribute_access
    result = IsccLib.gen_text_code_v0("Hello World")
    assert result.iscc.start_with?("ISCC:")
    assert result.characters.is_a?(Integer)
    assert result.characters.positive?, "characters should be > 0"
  end

  def test_gen_image_code_v0_basic
    pixels = ("\x00" * 1024).b
    result = IsccLib.gen_image_code_v0(pixels)
    assert_kind_of IsccLib::ImageCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_gen_image_code_v0_attribute_access
    pixels = ("\xFF" * 1024).b
    result = IsccLib.gen_image_code_v0(pixels)
    assert result.iscc.start_with?("ISCC:")
    assert result.iscc.is_a?(String)
  end

  def test_gen_audio_code_v0_basic
    cv = [1, 2, 3, 4, 5, 6, 7, 8]
    result = IsccLib.gen_audio_code_v0(cv)
    assert_kind_of IsccLib::AudioCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_gen_audio_code_v0_attribute_access
    cv = [100, 200, 300, 400]
    result = IsccLib.gen_audio_code_v0(cv)
    assert result.iscc.start_with?("ISCC:")
    assert result.iscc.is_a?(String)
  end

  def test_gen_video_code_v0_basic
    # WTA-Hash requires at least 380 elements per frame signature
    frame = (1..400).to_a
    frame_sigs = [frame, frame.map { |x| x + 1 }]
    result = IsccLib.gen_video_code_v0(frame_sigs)
    assert_kind_of IsccLib::VideoCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_gen_video_code_v0_attribute_access
    frame = (1..400).to_a
    frame_sigs = [frame, frame.reverse]
    result = IsccLib.gen_video_code_v0(frame_sigs)
    assert result.iscc.start_with?("ISCC:")
    assert result.iscc.is_a?(String)
  end

  def test_gen_mixed_code_v0_basic
    # gen_mixed_code_v0 requires Content-Codes (Text, Image, Audio, Video)
    text = IsccLib.gen_text_code_v0("Hello World " * 100)
    pixels = ("\x80" * 1024).b
    image = IsccLib.gen_image_code_v0(pixels)
    codes = [text.iscc, image.iscc]
    result = IsccLib.gen_mixed_code_v0(codes)
    assert_kind_of IsccLib::MixedCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
    assert result["parts"].is_a?(Array), "parts should be an Array"
  end

  def test_gen_mixed_code_v0_attribute_access
    text = IsccLib.gen_text_code_v0("Test Document " * 100)
    pixels = ("\xFF" * 1024).b
    image = IsccLib.gen_image_code_v0(pixels)
    result = IsccLib.gen_mixed_code_v0([text.iscc, image.iscc])
    assert result.iscc.start_with?("ISCC:")
    assert result.parts.is_a?(Array)
    assert result.parts.all? { |p| p.is_a?(String) }, "all parts should be Strings"
  end

  def test_gen_data_code_v0_basic
    data = ("Hello World" * 100).b
    result = IsccLib.gen_data_code_v0(data)
    assert_kind_of IsccLib::DataCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_gen_data_code_v0_attribute_access
    data = ("x" * 1000).b
    result = IsccLib.gen_data_code_v0(data)
    assert result.iscc.start_with?("ISCC:")
    assert result.iscc.is_a?(String)
  end

  def test_gen_instance_code_v0_basic
    data = ("Hello Instance" * 100).b
    result = IsccLib.gen_instance_code_v0(data)
    assert_kind_of IsccLib::InstanceCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
    assert result.key?("datahash"), "Result should contain datahash"
    assert result.key?("filesize"), "Result should contain filesize"
    assert_equal data.bytesize, result["filesize"]
  end

  def test_gen_instance_code_v0_attribute_access
    data = ("x" * 500).b
    result = IsccLib.gen_instance_code_v0(data)
    assert result.iscc.start_with?("ISCC:")
    assert result.datahash.is_a?(String)
    assert_equal data.bytesize, result.filesize
  end

  def test_gen_iscc_code_v0_basic
    data = ("ISCC code test data" * 100).b
    data_code = IsccLib.gen_data_code_v0(data)
    instance_code = IsccLib.gen_instance_code_v0(data)
    result = IsccLib.gen_iscc_code_v0([data_code.iscc, instance_code.iscc])
    assert_kind_of IsccLib::IsccCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
  end

  def test_gen_iscc_code_v0_attribute_access
    data = ("composite code test" * 100).b
    data_code = IsccLib.gen_data_code_v0(data)
    instance_code = IsccLib.gen_instance_code_v0(data)
    result = IsccLib.gen_iscc_code_v0([data_code.iscc, instance_code.iscc])
    assert result.iscc.start_with?("ISCC:")
    assert result.iscc.is_a?(String)
  end

  def test_gen_sum_code_v0_basic
    require "tempfile"
    file = Tempfile.new("iscc_test")
    file.binmode
    file.write("Sum code test content" * 100)
    file.flush
    result = IsccLib.gen_sum_code_v0(file.path)
    assert_kind_of IsccLib::SumCodeResult, result
    assert result["iscc"].start_with?("ISCC:"), "ISCC should start with 'ISCC:'"
    assert result.key?("datahash"), "Result should contain datahash"
    assert result.key?("filesize"), "Result should contain filesize"
  ensure
    file&.close
    file&.unlink
  end

  def test_gen_sum_code_v0_with_units
    require "tempfile"
    file = Tempfile.new("iscc_test_units")
    file.binmode
    file.write("Sum code with units" * 100)
    file.flush
    result = IsccLib.gen_sum_code_v0(file.path, add_units: true)
    assert result.iscc.start_with?("ISCC:")
    assert result.key?("units"), "Result should contain units when add_units is true"
    assert result["units"].is_a?(Array), "units should be an Array"
    assert result["units"].length.positive?, "units should not be empty"
  ensure
    file&.close
    file&.unlink
  end

  def test_sliding_window_basic
    result = IsccLib.sliding_window("Hello World", 3)
    assert result.is_a?(Array), "sliding_window should return an Array"
    assert result.all? { |s| s.is_a?(String) }, "all elements should be Strings"
    assert_equal 9, result.length, "should produce 9 trigrams from 11 chars"
    assert_equal "Hel", result.first
    assert_equal "rld", result.last
  end

  def test_sliding_window_error
    assert_raises(RuntimeError) { IsccLib.sliding_window("test", 1) }
  end

  def test_alg_simhash_basic
    digest = ("\xFF" * 4).b
    result = IsccLib.alg_simhash([digest, digest])
    assert result.is_a?(String), "alg_simhash should return a String"
    assert_equal Encoding::ASCII_8BIT, result.encoding, "should be binary"
    assert_equal 4, result.bytesize, "output length should match input digest length"
    assert_equal digest, result, "identical digests should produce same hash"
  end

  def test_alg_simhash_mismatched_error
    assert_raises(RuntimeError) do
      IsccLib.alg_simhash([("\x00" * 4).b, ("\x00" * 8).b])
    end
  end

  def test_alg_minhash_256_basic
    result = IsccLib.alg_minhash_256([1, 2, 3, 4, 5])
    assert result.is_a?(String), "alg_minhash_256 should return a String"
    assert_equal Encoding::ASCII_8BIT, result.encoding, "should be binary"
    assert_equal 32, result.bytesize, "should return 32 bytes (256 bits)"
  end

  def test_alg_cdc_chunks_basic
    data = ("Hello World " * 200).b
    result = IsccLib.alg_cdc_chunks(data, false, 1024)
    assert result.is_a?(Array), "alg_cdc_chunks should return an Array"
    assert result.length.positive?, "should produce at least one chunk"
    assert result.all? { |c| c.is_a?(String) }, "all chunks should be Strings"
    assert result.all? { |c| c.encoding == Encoding::ASCII_8BIT }, "chunks should be binary"
    reassembled = result.join
    assert_equal data, reassembled, "chunks should reassemble to original data"
  end

  def test_alg_cdc_chunks_empty
    result = IsccLib.alg_cdc_chunks("".b, false, 1024)
    assert result.is_a?(Array)
    assert_equal 1, result.length, "empty input should produce one empty chunk"
  end

  def test_soft_hash_video_v0_basic
    frame = (1..400).to_a
    frame_sigs = [frame, frame.map { |x| x + 1 }]
    result = IsccLib.soft_hash_video_v0(frame_sigs, 64)
    assert result.is_a?(String), "soft_hash_video_v0 should return a String"
    assert_equal Encoding::ASCII_8BIT, result.encoding, "should be binary"
    assert_equal 8, result.bytesize, "64 bits = 8 bytes"
  end

  def test_soft_hash_video_v0_256_bits
    frame = (1..400).to_a
    frame_sigs = [frame, frame.reverse]
    result = IsccLib.soft_hash_video_v0(frame_sigs, 256)
    assert result.is_a?(String)
    assert_equal 32, result.bytesize, "256 bits = 32 bytes"
  end

  def test_version
    assert IsccLib::VERSION.is_a?(String)
    assert_match(/\A\d+\.\d+\.\d+\z/, IsccLib::VERSION)
  end
end
