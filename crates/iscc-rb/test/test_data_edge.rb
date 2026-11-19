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

  # Section name -> [raw call, converter]. The raw call is kept apart from the
  # conversion so a conversion error never counts as a rejection.
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
    ]
  }.freeze

  # Load and parse data-edge.json (explicit UTF-8, independent of the process locale).
  def self.load_doc
    JSON.parse(File.read(DATA_EDGE_JSON, encoding: "UTF-8"))
  end

  # Compare an actual output with the expected one; return a failure or nil.
  # For a Hash, only the expected keys the result exposes are compared.
  def self.compare(expected, actual)
    unless expected.is_a?(Hash)
      return (expected == actual) ? nil : "expected #{expected.inspect}, got #{actual.inspect}"
    end

    keys = actual.keys & expected.keys
    return "no expected field in #{actual.inspect}" if keys.empty?

    key = keys.find { |k| expected[k] != actual[k] }
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
  def self.check_vector((call, convert), vector)
    exception = vector.fetch("except", {})[SURFACE]
    return nil if exception == "skip"

    expect_error = exception == "error" || vector.key?("error")
    status, value = raw_call(call, vector["inputs"])
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
