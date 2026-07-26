# frozen_string_literal: true

# Unicode 16.0.0 boundary conformance tests for the Ruby binding.
# Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json)
# against text_clean and text_collapse, gating the declared-Unicode-version freeze
# rule: code points unassigned in Unicode 16.0.0 are mapped to the noncharacter
# sentinel U+FFFF before normalization and removed by the unchanged category-C
# filter, while assigned code points flow through the regular pipeline. The
# sequence vectors additionally pin the sentinel map against the superseded
# delete-filter design.

require "test_helper"
require "json"

# Path to the canonical Unicode boundary vectors (read in place, no vendored copy).
BOUNDARY_JSON = File.expand_path("../../iscc-lib/tests/unicode_boundary.json", __dir__)

# Load and parse the boundary fixture once.
BOUNDARY_DATA = JSON.parse(File.read(BOUNDARY_JSON))

class TestUnicodeBoundary < Minitest::Test
  # Guard against a truncated or renamed fixture silently defining zero vector tests.
  def test_boundary_fixture_metadata
    assert_equal "16.0.0", BOUNDARY_DATA["_metadata"]["unicode_data_version"],
      "fixture must declare Unicode data version 16.0.0"
    assert_equal 7, BOUNDARY_DATA["text_clean"].length,
      "expected 7 text_clean boundary cases"
    assert_equal 5, BOUNDARY_DATA["text_collapse"].length,
      "expected 5 text_collapse boundary cases"
  end

  # ── text_clean ───────────────────────────────────────────────────────────

  BOUNDARY_DATA["text_clean"].each do |vector_name, tc|
    define_method("test_text_clean_#{vector_name}") do
      assert_equal tc["outputs"]["result"], IsccLib.text_clean(*tc["inputs"]),
        "text_clean mismatch for #{vector_name}"
    end
  end

  # ── text_collapse ────────────────────────────────────────────────────────

  BOUNDARY_DATA["text_collapse"].each do |vector_name, tc|
    define_method("test_text_collapse_#{vector_name}") do
      assert_equal tc["outputs"]["result"], IsccLib.text_collapse(*tc["inputs"]),
        "text_collapse mismatch for #{vector_name}"
    end
  end
end
