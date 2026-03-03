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

  # Result from gen_text_code_v0.
  class TextCodeResult < Result; end

  # Result from gen_image_code_v0.
  class ImageCodeResult < Result; end

  # Result from gen_audio_code_v0.
  class AudioCodeResult < Result; end

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

  # Generate a Text-Code from plain text content.
  #
  # @param text [String] plain text content
  # @param bits [Integer] bit length (default: 64)
  # @return [TextCodeResult] hash with iscc, characters
  def self.gen_text_code_v0(text, bits: 64)
    TextCodeResult[_gen_text_code_v0(text, bits)]
  end

  # Generate an Image-Code from pixel data.
  #
  # @param pixels [String] binary pixel data
  # @param bits [Integer] bit length (default: 64)
  # @return [ImageCodeResult] hash with iscc
  def self.gen_image_code_v0(pixels, bits: 64)
    ImageCodeResult[_gen_image_code_v0(pixels, bits)]
  end

  # Generate an Audio-Code from a Chromaprint feature vector.
  #
  # @param cv [Array<Integer>] Chromaprint fingerprint integers
  # @param bits [Integer] bit length (default: 64)
  # @return [AudioCodeResult] hash with iscc
  def self.gen_audio_code_v0(cv, bits: 64)
    AudioCodeResult[_gen_audio_code_v0(cv, bits)]
  end
end
