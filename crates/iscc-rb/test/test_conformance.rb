# frozen_string_literal: true

# Conformance tests for all 9 gen_*_v0 functions against data.json vectors.
# Mirrors tests/test_conformance.py using Ruby/Minitest idioms.

require "test_helper"
require "json"

# Path to the shared conformance vectors.
DATA_JSON = File.expand_path("../../iscc-lib/tests/data.json", __dir__)

# Load and parse the full data.json once.
CONFORMANCE_DATA = JSON.parse(File.read(DATA_JSON))

class TestConformance < Minitest::Test
  # Convert meta input from JSON value to Ruby argument for gen_meta_code_v0.
  def self.prepare_meta_arg(meta_val)
    return nil if meta_val.nil?
    return meta_val if meta_val.is_a?(String)

    if meta_val.is_a?(Hash)
      # Sort keys for JCS-compatible JSON string (Ruby's json gem ignores sort_keys option)
      return JSON.generate(meta_val.sort.to_h)
    end

    raise "unexpected meta type: #{meta_val.class}"
  end

  # Decode 'stream:<hex>' input to binary string.
  def self.decode_stream(stream_str)
    hex_data = stream_str.delete_prefix("stream:")
    return "".b if hex_data.empty?

    [hex_data].pack("H*")
  end

  # ── gen_meta_code_v0 ─────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_meta_code_v0"].each do |vector_name, tc|
    define_method("test_gen_meta_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      name = inputs[0]
      description = inputs[1]
      meta = self.class.prepare_meta_arg(inputs[2])
      bits = inputs[3]

      result = IsccLib.gen_meta_code_v0(
        name,
        description: description.empty? ? nil : description,
        meta: meta,
        bits: bits
      )
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
      assert_equal outputs["name"], result["name"], "name mismatch for #{vector_name}"
      assert_equal outputs["metahash"], result["metahash"], "metahash mismatch for #{vector_name}"

      if outputs.key?("description")
        assert_equal outputs["description"], result["description"],
          "description mismatch for #{vector_name}"
      else
        refute result.key?("description"),
          "description should be absent for #{vector_name}"
      end

      if outputs.key?("meta")
        assert_equal outputs["meta"], result["meta"], "meta mismatch for #{vector_name}"
      else
        refute result.key?("meta"), "meta should be absent for #{vector_name}"
      end
    end
  end

  # ── gen_text_code_v0 ─────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_text_code_v0"].each do |vector_name, tc|
    define_method("test_gen_text_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      result = IsccLib.gen_text_code_v0(inputs[0], bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
      assert_equal outputs["characters"], result["characters"],
        "characters mismatch for #{vector_name}"
    end
  end

  # ── gen_image_code_v0 ────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_image_code_v0"].each do |vector_name, tc|
    define_method("test_gen_image_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      pixels = inputs[0].pack("C*")
      result = IsccLib.gen_image_code_v0(pixels, bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
    end
  end

  # ── gen_audio_code_v0 ────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_audio_code_v0"].each do |vector_name, tc|
    define_method("test_gen_audio_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      result = IsccLib.gen_audio_code_v0(inputs[0], bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
    end
  end

  # ── gen_video_code_v0 ────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_video_code_v0"].each do |vector_name, tc|
    define_method("test_gen_video_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      result = IsccLib.gen_video_code_v0(inputs[0], bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
    end
  end

  # ── gen_mixed_code_v0 ────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_mixed_code_v0"].each do |vector_name, tc|
    define_method("test_gen_mixed_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      result = IsccLib.gen_mixed_code_v0(inputs[0], bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
      assert_equal outputs["parts"], result["parts"], "parts mismatch for #{vector_name}"
    end
  end

  # ── gen_data_code_v0 ─────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_data_code_v0"].each do |vector_name, tc|
    define_method("test_gen_data_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      data = self.class.decode_stream(inputs[0])
      result = IsccLib.gen_data_code_v0(data, bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
    end
  end

  # ── gen_instance_code_v0 ─────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_instance_code_v0"].each do |vector_name, tc|
    define_method("test_gen_instance_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      data = self.class.decode_stream(inputs[0])
      result = IsccLib.gen_instance_code_v0(data, bits: inputs[1])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
      assert_equal outputs["datahash"], result["datahash"], "datahash mismatch for #{vector_name}"
      assert_equal outputs["filesize"], result["filesize"], "filesize mismatch for #{vector_name}"
    end
  end

  # ── gen_iscc_code_v0 ─────────────────────────────────────────────────────

  CONFORMANCE_DATA["gen_iscc_code_v0"].each do |vector_name, tc|
    define_method("test_gen_iscc_code_v0_#{vector_name}") do
      inputs = tc["inputs"]
      outputs = tc["outputs"]
      result = IsccLib.gen_iscc_code_v0(inputs[0])
      assert_equal outputs["iscc"], result["iscc"], "iscc mismatch for #{vector_name}"
    end
  end
end
