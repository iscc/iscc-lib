# frozen_string_literal: true

# Tests for the ISCC-SEQ codec (encode_seq / decode_seq, IEP-0020).
# Covers the byte round trip over data.json vectors, return types, and
# rejection of malformed inputs (one bad input per assertion).

require "test_helper"
require "json"

# Path to the shared conformance vectors.
SEQ_DATA_JSON = File.expand_path("../../iscc-lib/tests/data.json", __dir__)

# The decode_seq section of data.json (explicit UTF-8, independent of the process locale).
SEQ_DECODE_VECTORS = JSON.parse(File.read(SEQ_DATA_JSON, encoding: "UTF-8"))["decode_seq"]

class TestSeq < Minitest::Test
  # Decode a 'bytes:<hex>' fixture value to a binary String.
  def seq_bytes(value)
    [value.delete_prefix("bytes:")].pack("H*")
  end

  # The binary ISCC-SEQ of the four-unit decode_seq vector.
  def four_unit_seq
    seq_bytes(SEQ_DECODE_VECTORS["test_0001_four_units"]["inputs"][0])
  end

  def test_fixture_has_decode_seq_vectors
    assert_equal 4, SEQ_DECODE_VECTORS.size
    assert SEQ_DECODE_VECTORS.key?("test_0001_four_units")
  end

  def test_byte_round_trip
    SEQ_DECODE_VECTORS.each do |name, tc|
      data = seq_bytes(tc["inputs"][0])
      assert_equal data, IsccLib.encode_seq(IsccLib.decode_seq(data)), "round trip for #{name}"
    end
  end

  def test_encode_seq_returns_binary_string
    result = IsccLib.encode_seq(["ISCC:EEAYYNMFG5XOELK2"])
    assert_kind_of String, result
    assert_equal Encoding::BINARY, result.encoding
  end

  def test_decode_seq_returns_prefixed_strings
    result = IsccLib.decode_seq(four_unit_seq)
    assert_kind_of Array, result
    assert_equal 4, result.size
    result.each do |unit|
      assert_kind_of String, unit
      assert unit.start_with?("ISCC:"), "unit #{unit.inspect} lacks ISCC: prefix"
    end
  end

  def test_encode_seq_empty_list
    err = assert_raises(RuntimeError) { IsccLib.encode_seq([]) }
    assert_match(/at least one ISCC-UNIT/, err.message)
  end

  def test_encode_seq_embedded_nul
    assert_raises(RuntimeError) { IsccLib.encode_seq(["ISCC:EEAYYNMFG5XOELK2\0junk"]) }
  end

  def test_encode_seq_case_fold_alias
    assert_raises(RuntimeError) { IsccLib.encode_seq(["gaawdlqng6ſsdpf2"]) }
  end

  def test_encode_seq_case_fold_control
    assert_equal 10, IsccLib.encode_seq(["gaawdlqng6ssdpf2"]).bytesize
  end

  def test_encode_seq_padded_unit
    # Correctly padded: iscc-core 1.5.0 accepts it, iscc-lib's strict base32 rejects it.
    assert_raises(RuntimeError) { IsccLib.encode_seq(["ISCC:GAAAAAAAAA======"]) }
  end

  def test_encode_seq_padding_control
    assert_equal ["300000000000"].pack("H*"), IsccLib.encode_seq(["ISCC:GAAAAAAAAA"])
  end

  def test_encode_seq_unpadded_control
    # 2-byte header + 64-bit body, as in data.json encode_seq test_0004.
    assert_equal ["21018c3585376ee22d5a"].pack("H*"), IsccLib.encode_seq(["ISCC:EEAYYNMFG5XOELK2"])
  end

  def test_decode_seq_empty_bytes
    err = assert_raises(RuntimeError) { IsccLib.decode_seq("".b) }
    assert_match(/Empty ISCC-SEQ/, err.message)
  end

  def test_decode_seq_truncated
    data = four_unit_seq
    err = assert_raises(RuntimeError) { IsccLib.decode_seq(data.byteslice(0, data.bytesize - 1)) }
    assert_match(/Truncated ISCC-UNIT/, err.message)
  end
end
