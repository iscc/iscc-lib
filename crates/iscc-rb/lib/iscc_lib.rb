# frozen_string_literal: true

# Ruby bindings for iscc-lib — ISO 24138:2024 (ISCC).
#
# This module provides the public API for generating ISCC codes from Ruby.
# Result classes wrap the native Hash returns with attribute-style access.

require_relative "iscc_lib/version"
require_relative "iscc_lib/iscc_rb"

module IsccLib
  # Base result class providing both Hash-style and attribute-style access.
  class Result < Hash
    # Allow attribute-style access for hash keys (e.g., result.iscc).
    def method_missing(name, *args)
      key = name.to_s
      return self[key] if key?(key)

      super
    end

    # Support respond_to? for dynamic attribute access.
    def respond_to_missing?(name, include_private = false)
      key?(name.to_s) || super
    end
  end

  # Result from gen_meta_code_v0.
  class MetaCodeResult < Result; end

  # Generate a Meta-Code from name and optional metadata.
  #
  # @param name [String] content name (required, non-empty)
  # @param description [String, nil] optional content description
  # @param meta [String, nil] optional JSON metadata
  # @param bits [Integer] bit length (default: 64)
  # @return [MetaCodeResult] hash with iscc, name, metahash, and optionally description, meta
  def self.gen_meta_code_v0(name, description: nil, meta: nil, bits: 64)
    MetaCodeResult[_gen_meta_code_v0(name, description, meta, bits)]
  end
end
