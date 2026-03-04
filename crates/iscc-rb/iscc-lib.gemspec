# frozen_string_literal: true

require_relative "lib/iscc_lib/version"

Gem::Specification.new do |spec|
  spec.name = "iscc-lib"
  spec.version = IsccLib::VERSION
  spec.authors = ["Titusz Pan"]
  spec.email = ["tp@py7.de"]

  spec.summary = "ISCC - International Standard Content Code (ISO 24138)"
  spec.description = "High-performance Ruby bindings for ISO 24138:2024 (ISCC). " \
                     "Native Rust extension via Magnus for content identification and matching."
  spec.homepage = "https://github.com/iscc/iscc-lib"
  spec.license = "Apache-2.0"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/iscc/iscc-lib"
  spec.metadata["documentation_uri"] = "https://lib.iscc.codes"
  spec.metadata["rubygems_mfa_required"] = "true"

  spec.files = Dir[
    "lib/**/*.rb",
    "src/**/*.rs",
    "extconf.rb",
    "Cargo.toml",
    "README.md",
    "LICENSE"
  ]

  spec.require_paths = ["lib"]
  spec.extensions = ["extconf.rb"]
end
