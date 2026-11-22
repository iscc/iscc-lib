# frozen_string_literal: true

# Shared edge-case vectors (crates/iscc-lib/tests/data-edge.json) run against the Ruby bindings.
#
# Every vector runs, `except.ruby` is the only deviation, and a function section this
# harness does not handle fails. A rejection is asserted on the raw call alone (it must
# raise RuntimeError); the result is converted to its fixture form only after a
# successful call.

require "test_helper"
require "json"

# Path to the shared edge-case vectors.
DATA_EDGE_JSON = File.expand_path("../../iscc-lib/tests/data-edge.json", __dir__)

module DataEdge
  # Surface id of this harness in the `except` map.
  SURFACE = "ruby"

  # Encode a binary string as a `bytes:<hex>` fixture value.
  def self.bytes_value(data)
    "bytes:#{data.unpack1("H*")}"
  end

  # Convert an `iscc_decode` result to `[mt, st, vs, length, "bytes:<hex>"]`.
  def self.convert_decode(result)
    mt, st, vs, li, digest = result
    [mt, st, vs, li, bytes_value(digest)]
  end

  # Convert a gen result (a Hash subclass) to a plain Hash.
  def self.convert_hash(result)
    result.to_h
  end

  # Decode a `bytes:<hex>` fixture value into a binary string; raise on any other value.
  def self.decode_bytes(value)
    digits = value.delete_prefix("bytes:")
    unless value.start_with?("bytes:") && digits.match?(/\A(?:[0-9a-f]{2})*\z/)
      raise ArgumentError, "expected a bytes:<hex> input, got #{value.inspect}"
    end

    [digits].pack("H*")
  end

  # Decode the `alg_simhash` digest list from `bytes:<hex>` strings into binary strings.
  def self.prepare_simhash(inputs)
    [inputs[0].map { |d| decode_bytes(d) }, *inputs.drop(1)]
  end

  # Decode the `alg_cdc_chunks` data from a `bytes:<hex>` string into a binary string.
  def self.prepare_cdc(inputs)
    [decode_bytes(inputs[0]), *inputs.drop(1)]
  end

  # Convert `alg_cdc_chunks` chunks to a list of `bytes:<hex>` values.
  def self.convert_chunks(chunks)
    chunks.map { |c| bytes_value(c) }
  end

  # Decode the `encode_component` digest (the last input) from `bytes:<hex>` into a binary string.
  def self.prepare_component(inputs)
    [*inputs.take(4), decode_bytes(inputs[4])]
  end

  # Return a bare string result (`encode_component`) unchanged.
  def self.convert_string(result)
    result
  end

  # Prepare `gen_meta_code_v0` inputs: an empty description becomes nil and a meta
  # Hash is serialized to the JSON string the binding takes.
  def self.prepare_meta(inputs)
    name, description, meta, *rest = inputs
    meta_arg = meta.is_a?(Hash) ? JSON.generate(meta) : meta
    [name, (description.empty? ? nil : description), meta_arg, *rest]
  end

  # Section name -> [raw call, converter, optional input preparer]. Preparing and
  # converting stay outside the raw call so neither error ever counts as a rejection;
  # `bytes:` inputs are decoded only by a section's preparer.
  SECTIONS = {
    "iscc_decode" => [->(inputs) { IsccLib.iscc_decode(inputs[0]) }, method(:convert_decode)],
    "encode_seq" => [->(inputs) { IsccLib.encode_seq(inputs[0]) }, method(:bytes_value)],
    "gen_text_code_v0" => [
      ->(inputs) { IsccLib.gen_text_code_v0(inputs[0], bits: inputs[1]) }, method(:convert_hash)
    ],
    "gen_mixed_code_v0" => [
      ->(inputs) { IsccLib.gen_mixed_code_v0(inputs[0], bits: inputs[1]) }, method(:convert_hash)
    ],
    "gen_iscc_code_v0" => [
      ->(inputs) { IsccLib.gen_iscc_code_v0(inputs[0], wide: inputs.fetch(1, false)) },
      method(:convert_hash)
    ],
    "alg_simhash" => [
      ->(inputs) { IsccLib.alg_simhash(inputs[0]) }, method(:bytes_value), method(:prepare_simhash)
    ],
    "gen_meta_code_v0" => [
      lambda do |inputs|
        IsccLib.gen_meta_code_v0(inputs[0], description: inputs[1], meta: inputs[2], bits: inputs[3])
      end,
      method(:convert_hash), method(:prepare_meta)
    ],
    "gen_video_code_v0" => [
      ->(inputs) { IsccLib.gen_video_code_v0(inputs[0], bits: inputs[1]) }, method(:convert_hash)
    ],
    "alg_cdc_chunks" => [
      ->(inputs) { IsccLib.alg_cdc_chunks(*inputs) }, method(:convert_chunks), method(:prepare_cdc)
    ],
    "encode_component" => [
      ->(inputs) { IsccLib.encode_component(*inputs) }, method(:convert_string),
      method(:prepare_component)
    ]
  }.freeze

  # Load and parse data-edge.json (explicit UTF-8, independent of the process locale).
  def self.load_doc
    JSON.parse(File.read(DATA_EDGE_JSON, encoding: "UTF-8"))
  end

  # Compare an actual output with the expected one; return a failure or nil.
  # For a Hash, every expected key must be present in the result and equal.
  def self.compare(expected, actual)
    unless expected.is_a?(Hash)
      return (expected == actual) ? nil : "expected #{expected.inspect}, got #{actual.inspect}"
    end

    missing = expected.keys.find { |k| !actual.key?(k) }
    return "missing field #{missing} in #{actual.inspect}" if missing

    key = expected.keys.find { |k| expected[k] != actual[k] }
    key && "field #{key}: expected #{expected[key].inspect}, got #{actual[key].inspect}"
  end

  # Run the raw call; return [:error, exception] or [:ok, result].
  def self.raw_call(call, inputs)
    [:ok, call.call(inputs)]
  rescue RuntimeError => e
    [:error, e]
  end

  # Judge a successful raw call against the vector's expectation.
  def self.judge_ok(expect_error, convert, result, vector)
    got = convert.call(result)
    return "expected an error, got #{got.inspect}" if expect_error
    return "vector has neither outputs nor error" unless vector.key?("outputs")

    compare(vector["outputs"], got)
  end

  # Check one vector and return a failure description, or nil.
  def self.check_vector((call, convert, prepare), vector)
    exception = vector.fetch("except", {})[SURFACE]
    return nil if exception == "skip"

    expect_error = exception == "error" || vector.key?("error")
    inputs = prepare ? prepare.call(vector["inputs"]) : vector["inputs"]
    status, value = raw_call(call, inputs)
    if status == :error
      return expect_error ? nil : "expected an output, got error: #{value.message}"
    end

    judge_ok(expect_error, convert, value, vector)
  rescue => e
    "raw call raised #{e.class}: #{e.message}"
  end

  # Run every vector of a parsed document; return one `section/vector: reason` per failure.
  def self.check_doc(doc)
    doc.flat_map do |section, vectors|
      entry = SECTIONS[section]
      next ["#{section}: unknown section"] unless entry

      vectors.filter_map do |name, vector|
        reason = check_vector(entry, vector)
        reason && "#{section}/#{name}: #{reason}"
      end
    end
  end
end

class TestDataEdge < Minitest::Test
  def test_data_edge_vectors_pass
    doc = DataEdge.load_doc
    DataEdge::SECTIONS.each_key do |section|
      refute_empty doc.fetch(section, {}), "section #{section} has no vectors"
    end
    failures = DataEdge.check_doc(doc)

    assert_empty failures, "data-edge.json failures:\n#{failures.join("\n")}"
  end

  def test_data_edge_fails_unknown_section
    doc = {"gen_unknown_v0" => {"test_0001_x" => {"inputs" => ["x"], "outputs" => "x", "note" => "n"}}}
    failures = DataEdge.check_doc(doc)

    assert_equal 1, failures.size, failures.inspect
    assert_includes failures[0], "unknown section"
  end

  def test_data_edge_fails_expected_error_that_succeeds
    # Valid inputs marked `error` prove each rejection-only caller reaches the binding.
    doc = {
      "gen_mixed_code_v0" => {"test_0001_x" => {
        "inputs" => [["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"], 64], "error" => "e", "note" => "n"
      }},
      "gen_iscc_code_v0" => {"test_0001_x" => {
        "inputs" => [["GABVVC5DMJJGYKZ4ZBYVNYABFFYXG", "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]],
        "error" => "e", "note" => "n"
      }}
    }
    failures = DataEdge.check_doc(doc)

    assert_equal 2, failures.size, failures.inspect
    assert(failures.all? { |f| f.include?("expected an error") }, failures.inspect)
  end

  def test_data_edge_fails_output_mismatch
    doc = {"encode_seq" => {"test_0001_x" => {
      "inputs" => [["ISCC:GAAAAAAAAA"]], "outputs" => "bytes:300000000001", "note" => "n"
    }}}
    failures = DataEdge.check_doc(doc)

    assert_equal 1, failures.size, failures.inspect
    assert_includes failures[0], 'expected "bytes:300000000001"'
  end

  def test_data_edge_fails_missing_output_key
    # `iscc` is correct; the only bad case is a key the result does not have.
    doc = {"gen_text_code_v0" => {"test_0001_x" => {
      "inputs" => ["Hello\u0000World", 64],
      "outputs" => {"iscc" => "ISCC:EAASKDNZNYGUUF5A", "datahash" => "x"}, "note" => "n"
    }}}
    failures = DataEdge.check_doc(doc)

    assert_equal 1, failures.size, failures.inspect
    assert_includes failures[0], "missing field datahash"
  end

  def test_data_edge_honours_except
    outputs = [3, 0, 0, 0, "bytes:00000000"]
    doc = {"iscc_decode" => {
      "test_0001_error" => {
        "inputs" => ["ISCC:GAAAAAAAAA"], "outputs" => outputs, "note" => "n", "except" => {"ruby" => "error"}
      },
      "test_0002_skip" => {
        "inputs" => ["ISCC:GAAAAAAAAB"], "outputs" => outputs, "note" => "n", "except" => {"ruby" => "skip"}
      }
    }}
    failures = DataEdge.check_doc(doc)

    assert_equal 1, failures.size, failures.inspect
    assert failures[0].start_with?("iscc_decode/test_0001_error: expected an error"), failures.inspect
  end
end
