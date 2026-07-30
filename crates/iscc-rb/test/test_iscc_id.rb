# frozen_string_literal: true

# Tests for the experimental ISCC-IDv1 minting function (gen_iscc_id_v1).
# Covers the golden vector, decode round-trip, and validation ordering.

require "test_helper"

class TestIsccId < Minitest::Test
  def test_gen_iscc_id_v1_golden
    result = IsccLib.gen_iscc_id_v1(1_751_831_876_325_218, 1, 0)
    assert_kind_of IsccLib::IdCodeResult, result
    assert_equal "ISCC:MAIGHFECJMOPMIAB", result.iscc
  end

  def test_gen_iscc_id_v1_round_trip
    iscc = IsccLib.gen_iscc_id_v1(1_751_831_876_325_218, 1, 0).iscc
    mt, st, vs, li, digest = IsccLib.iscc_decode(iscc)
    assert_equal 6, mt, "MainType Id"
    assert_equal 0, st, "SubType is realm"
    assert_equal 1, vs, "Version V1"
    assert_equal 0, li, "64-bit length index"
    assert_equal 8, digest.bytesize, "8-byte body"

    n = digest.unpack1("Q>")
    assert_equal 1_751_831_876_325_218, n >> 12, "timestamp in high 52 bits"
    assert_equal 1, n & 0xFFF, "hub_id in low 12 bits"
    assert_equal 0, st, "realm from SubType"
  end

  def test_gen_iscc_id_v1_round_trip_corners
    max_ts = (1 << 52) - 1
    [0, 1].each do |realm|
      [0, 4095].each do |hub_id|
        [0, max_ts].each do |timestamp|
          iscc = IsccLib.gen_iscc_id_v1(timestamp, hub_id, realm).iscc
          _mt, st, vs, _li, digest = IsccLib.iscc_decode(iscc)
          n = digest.unpack1("Q>")
          assert_equal timestamp, n >> 12
          assert_equal hub_id, n & 0xFFF
          assert_equal realm, st
          assert_equal 1, vs
        end
      end
    end
  end

  def test_gen_iscc_id_v1_validation_order
    # First failing check wins: timestamp is checked before hub_id and realm.
    err = assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(1 << 52, 4096, 2) }
    assert_match(/timestamp/, err.message)

    # HUB-ID is checked before realm.
    err = assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(0, 4096, 2) }
    assert_match(/hub_id/, err.message)

    # Realm is the last check.
    err = assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(0, 0, 2) }
    assert_match(/realm/, err.message)
  end

  def test_gen_iscc_id_v1_rejects_negative
    assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(-1, 0, 0) }
    assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(0, -1, 0) }
    assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(0, 0, -1) }
  end

  def test_gen_iscc_id_v1_validation_order_bignum
    # Values beyond i64 must not raise RangeError during argument marshalling;
    # the first failing check still wins (timestamp before hub_id and realm).
    err = assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(1 << 52, 1 << 100, 2) }
    assert_match(/timestamp/, err.message)

    # A timestamp beyond u64 raises RuntimeError, not RangeError.
    err = assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(1 << 70, 0, 0) }
    assert_match(/timestamp/, err.message)

    # A hub_id beyond u64 reports hub_id once the timestamp is valid.
    err = assert_raises(RuntimeError) { IsccLib.gen_iscc_id_v1(0, 1 << 100, 2) }
    assert_match(/hub_id/, err.message)
  end
end
