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

  # Cargo.lock and core/ exist only in the standalone tree that
  # scripts/build_ruby_source_gem.py stages for the source gem; in this directory
  # (where cross-gem builds the platform gems) both globs match nothing.
  spec.files = Dir[
    "lib/**/*.rb",
    "src/**/*.rs",
    "extconf.rb",
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "LICENSE"
  ] + Dir["core/**/*"].select { |path| File.file?(path) }

  spec.require_paths = ["lib"]
  spec.extensions = ["extconf.rb"]

  # Only the source gem needs rb_sys: its extconf.rb requires rb_sys/mkmf to build the
  # extension on install. The platform gems cross-gem builds in this directory are
  # precompiled and need no runtime dependency, and a dependency here would put a
  # version-less rb_sys line first in Gemfile.lock, which breaks the cross-gem pin lookup.
  # core/ exists only in the staged tree, so the dependency is fixed into the source gem
  # when `gem build` runs there. __dir__ keeps the answer independent of the cwd.
  spec.add_dependency "rb_sys", "~> 0.9" if File.directory?(File.join(__dir__, "core"))
end
