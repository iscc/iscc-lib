# frozen_string_literal: true

# Tests for Ruby Array argument conversion in the native extension.
# An element whose implicit conversion (`to_str` / `to_int`) clears the
# caller's array and runs GC must not crash the process: arguments are
# converted from a private copy of the array. Also pins the exception
# classes for wrong-type arguments (TypeError), out-of-range integers
# (RangeError) and invalid values (RuntimeError).

require "test_helper"

# Number of elements, large enough that the array owns a heap buffer.
ARRAY_ARGS_SIZE = 2000

# An element that empties its containing array and runs GC on conversion.
class MutatingElement
  # Build an element that converts to `value` and clears `target` first.
  def initialize(value)
    @value = value
    @target = nil
  end

  attr_writer :target

  # Implicit String conversion: shrink the target array, collect, return.
  def to_str
    mutate
    @value
  end

  # Implicit Integer conversion: shrink the target array, collect, return.
  def to_int
    mutate
    @value
  end

  private

  # Clear the target array and force a full GC run.
  def mutate
    @target&.clear
    GC.start
  end
end

class TestArrayArgs < Minitest::Test
  TEXT_UNIT = IsccLib.gen_text_code_v0("Hello World").iscc
  IMAGE_UNIT = "ISCC:EEAYYNMFG5XOELK2"

  # Return [mutating_array, snapshot] where element 0 clears the array on conversion.
  def mutating_array(first, rest)
    element = MutatingElement.new(first)
    arr = [element] + Array.new(ARRAY_ARGS_SIZE - 1) { |i| rest.call(i) }
    element.target = arr
    snapshot = [first] + arr.drop(1)
    [arr, snapshot]
  end

  # Return [mutating_units, snapshot] alternating two valid ISCC units.
  def mutating_units
    mutating_array(TEXT_UNIT, ->(i) { i.even? ? IMAGE_UNIT : TEXT_UNIT })
  end

  # Return [mutating_ints, snapshot] of non-negative integers (valid as i32 and u32).
  def mutating_ints
    mutating_array(7, ->(i) { (i * 7919) % 100_000 })
  end

  # Assert `call` on the mutating array gives the snapshot's result (value or RuntimeError).
  def assert_same_outcome(arr, snapshot, &call)
    expected = begin
      call.call(snapshot)
    rescue RuntimeError => e
      e
    end
    if expected.is_a?(RuntimeError)
      err = assert_raises(RuntimeError) { call.call(arr) }
      assert_equal expected.message, err.message
    else
      assert_equal expected, call.call(arr)
    end
  end

  def test_encode_seq_survives_mutating_to_str
    arr, snapshot = mutating_units
    expected = IsccLib.encode_seq(snapshot)
    assert_equal expected, IsccLib.encode_seq(arr)
    assert_empty arr
  end

  def test_gen_mixed_code_v0_survives_mutating_to_str
    arr, snapshot = mutating_units
    expected = IsccLib.gen_mixed_code_v0(snapshot)
    # Compare the whole result: `.iscc` alone matches for the first two units.
    assert_equal expected, IsccLib.gen_mixed_code_v0(arr)
    assert_empty arr
  end

  def test_gen_iscc_code_v0_survives_mutating_to_str
    arr, snapshot = mutating_units
    assert_same_outcome(arr, snapshot) { |codes| IsccLib.gen_iscc_code_v0(codes).iscc }
    assert_empty arr
  end

  def test_gen_audio_code_v0_survives_mutating_to_int
    arr, snapshot = mutating_ints
    expected = IsccLib.gen_audio_code_v0(snapshot)
    assert_equal expected.iscc, IsccLib.gen_audio_code_v0(arr).iscc
    assert_empty arr
  end

  def test_alg_minhash_256_survives_mutating_to_int
    arr, snapshot = mutating_ints
    expected = IsccLib.alg_minhash_256(snapshot)
    assert_equal expected, IsccLib.alg_minhash_256(arr)
    assert_empty arr
  end

  def test_gen_video_code_v0_survives_mutating_frame_to_int
    frame, snapshot = mutating_ints
    expected = IsccLib.gen_video_code_v0([snapshot])
    assert_equal expected.iscc, IsccLib.gen_video_code_v0([frame]).iscc
    assert_empty frame
  end

  def test_soft_hash_video_v0_survives_mutating_frame_to_int
    frame, snapshot = mutating_ints
    expected = IsccLib.soft_hash_video_v0([snapshot], 64)
    assert_equal expected, IsccLib.soft_hash_video_v0([frame], 64)
    assert_empty frame
  end

  def test_frozen_array_argument_is_accepted
    units = [TEXT_UNIT, IMAGE_UNIT].freeze
    assert_equal IsccLib.encode_seq(units.dup), IsccLib.encode_seq(units)
  end

  def test_to_ary_argument_is_coerced
    units = Struct.new(:units) { def to_ary = units }.new([TEXT_UNIT, IMAGE_UNIT])
    assert_equal IsccLib.encode_seq([TEXT_UNIT, IMAGE_UNIT]), IsccLib.encode_seq(units)
  end

  def test_non_string_element_raises_type_error
    assert_raises(TypeError) { IsccLib.encode_seq([nil]) }
  end

  def test_non_array_argument_raises_type_error
    assert_raises(TypeError) { IsccLib.encode_seq(TEXT_UNIT) }
  end

  def test_non_integer_element_raises_type_error
    assert_raises(TypeError) { IsccLib.gen_audio_code_v0(["1"]) }
  end

  def test_non_array_frame_raises_type_error
    assert_raises(TypeError) { IsccLib.gen_video_code_v0([1]) }
  end

  def test_out_of_range_integer_element_raises_range_error
    assert_raises(RangeError) { IsccLib.gen_audio_code_v0([2**40]) }
  end

  # Negative Bignums that fit `long` but not `i32` (Magnus truncated these).
  NEGATIVE_BIGNUMS = [-(2**63), -(2**62) - 1].freeze

  def test_negative_bignum_audio_element_raises_range_error
    NEGATIVE_BIGNUMS.each do |value|
      assert_raises(RangeError) { IsccLib.gen_audio_code_v0([value]) }
    end
  end

  def test_negative_bignum_video_frame_element_raises_range_error
    NEGATIVE_BIGNUMS.each do |value|
      assert_raises(RangeError) { IsccLib.gen_video_code_v0([[value]]) }
    end
  end

  def test_negative_bignum_soft_hash_video_frame_element_raises_range_error
    NEGATIVE_BIGNUMS.each do |value|
      assert_raises(RangeError) { IsccLib.soft_hash_video_v0([[value]], 64) }
    end
  end

  def test_fixnum_just_outside_i32_raises_range_error
    [2**31, -(2**31) - 1].each do |value|
      assert_raises(RangeError) { IsccLib.gen_audio_code_v0([value]) }
    end
  end

  def test_i32_boundaries_are_accepted
    result = IsccLib.gen_audio_code_v0([-(2**31), 2**31 - 1])
    assert_kind_of Hash, result
    assert result.key?("iscc")
  end

  def test_negative_minhash_feature_raises_range_error
    assert_raises(RangeError) { IsccLib.alg_minhash_256([-1]) }
  end

  def test_invalid_unit_value_raises_runtime_error
    assert_raises(RuntimeError) { IsccLib.encode_seq(["ISCC:NOTAVALIDCODE"]) }
  end
end
