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

  # Result from gen_video_code_v0.
  class VideoCodeResult < Result; end

  # Result from gen_mixed_code_v0.
  class MixedCodeResult < Result; end

  # Result from gen_data_code_v0.
  class DataCodeResult < Result; end

  # Result from gen_instance_code_v0.
  class InstanceCodeResult < Result; end

  # Result from gen_iscc_code_v0.
  class IsccCodeResult < Result; end

  # Result from gen_sum_code_v0.
  class SumCodeResult < Result; end

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

  # Generate a Video-Code from frame signature vectors.
  #
  # @param frame_sigs [Array<Array<Integer>>] nested array of frame signatures
  # @param bits [Integer] bit length (default: 64)
  # @return [VideoCodeResult] hash with iscc
  def self.gen_video_code_v0(frame_sigs, bits: 64)
    VideoCodeResult[_gen_video_code_v0(frame_sigs, bits)]
  end

  # Generate a Mixed-Code from multiple ISCC content code strings.
  #
  # @param codes [Array<String>] ISCC unit strings
  # @param bits [Integer] bit length (default: 64)
  # @return [MixedCodeResult] hash with iscc, parts
  def self.gen_mixed_code_v0(codes, bits: 64)
    MixedCodeResult[_gen_mixed_code_v0(codes, bits)]
  end

  # Generate a Data-Code from binary data.
  #
  # @param data [String] binary data
  # @param bits [Integer] bit length (default: 64)
  # @return [DataCodeResult] hash with iscc
  def self.gen_data_code_v0(data, bits: 64)
    DataCodeResult[_gen_data_code_v0(data, bits)]
  end

  # Generate an Instance-Code from binary data.
  #
  # @param data [String] binary data
  # @param bits [Integer] bit length (default: 64, accepted for API consistency)
  # @return [InstanceCodeResult] hash with iscc, datahash, filesize
  def self.gen_instance_code_v0(data, bits: 64)
    InstanceCodeResult[_gen_instance_code_v0(data, bits)]
  end

  # Generate a composite ISCC-CODE from individual unit codes.
  #
  # @param codes [Array<String>] ISCC unit strings (Data-Code + Instance-Code, optional Content-Code)
  # @param wide [Boolean] use 256-bit combination (default: false for 128-bit)
  # @return [IsccCodeResult] hash with iscc
  def self.gen_iscc_code_v0(codes, wide: false)
    IsccCodeResult[_gen_iscc_code_v0(codes, wide)]
  end

  # Generate a composite ISCC-CODE from a file in a single pass.
  #
  # @param path [String] file path
  # @param bits [Integer] bit length (default: 64)
  # @param wide [Boolean] use 256-bit combination (default: false)
  # @param add_units [Boolean] include individual unit codes (default: false)
  # @return [SumCodeResult] hash with iscc, datahash, filesize, and optionally units
  def self.gen_sum_code_v0(path, bits: 64, wide: false, add_units: false)
    SumCodeResult[_gen_sum_code_v0(path, bits, wide, add_units)]
  end
end
