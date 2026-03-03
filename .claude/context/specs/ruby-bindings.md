# Spec: Ruby Bindings — Native Extension via Magnus

A Ruby gem `iscc-lib` providing native Rust-powered ISCC functions via Magnus (Rust ↔ Ruby bridge).
Follows the same hub-and-spoke model as all other bindings: thin bridge crate wrapping `iscc-lib`
core, with a pure Ruby wrapper layer for idiomatic API.

## Architecture

**Two-layer design** (matching the Python binding pattern):

1. **Rust bridge** (`crates/iscc-rb/src/lib.rs`): Magnus-annotated functions that call `iscc-lib`
    core and return Ruby `Hash` objects. Single file, ~400–500 lines.
2. **Pure Ruby wrapper** (`crates/iscc-rb/lib/iscc_lib.rb`): Typed result classes with attribute
    access, keyword arguments, streaming convenience methods. Provides idiomatic Ruby DX.

**Why Magnus (not C FFI gem):**

- Magnus is the Ruby equivalent of PyO3 — generates native Ruby C extensions directly from Rust
- No manual memory management (Ruby GC handles everything)
- Rich return types (Ruby Hash, custom classes) without C intermediary
- Same proven pattern as the existing PyO3 Python bindings
- `rb_sys` integration for cross-platform precompiled gems

## Crate Structure

```
crates/iscc-rb/
├── Cargo.toml              # cdylib, depends on iscc-lib + magnus
├── src/
│   └── lib.rs              # Magnus bridge (~400-500 lines)
├── ext/
│   └── iscc_lib/
│       └── extconf.rb      # rb_sys extension config
├── lib/
│   └── iscc_lib/
│       ├── version.rb      # VERSION constant (synced from Cargo.toml)
│       └── iscc_lib.rb     # Pure Ruby wrapper with result classes
├── iscc-lib.gemspec        # Gem specification
├── Gemfile                 # Development dependencies
├── Rakefile                # rb_sys build tasks
├── README.md               # Per-crate README for RubyGems
└── test/
    ├── test_helper.rb
    └── test_conformance.rb # Minitest against data.json
```

## Rust Bridge Layer

The Rust bridge follows the same patterns as `iscc-py`:

- All `gen_*_v0` functions return Ruby `Hash` (via `magnus::RHash`)
- Optional dict keys are omitted entirely (not set to `nil`)
- Streaming types (`DataHasher`, `InstanceHasher`) use `Option<inner>` for one-shot finalize
- Errors mapped to `Ruby RuntimeError` via `magnus::Error`
- Byte data accepted as Ruby `String` (which can hold binary data in Ruby)

```rust
use magnus::{function, prelude::*, Error, Ruby, RHash};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("IsccLib")?;
    module.define_module_function("_gen_meta_code_v0", function!(gen_meta_code_v0, 4))?;
    // ... all other functions
    Ok(())
}
```

## Ruby Wrapper Layer

### Result Classes

```ruby
module IsccLib
  class Result < Hash
    def method_missing(name, *args)
      return self[name.to_s] if key?(name.to_s)
      super
    end

    def respond_to_missing?(name, include_private = false)
      key?(name.to_s) || super
    end
  end

  class MetaCodeResult < Result; end
  class TextCodeResult < Result; end
  class ImageCodeResult < Result; end
  class AudioCodeResult < Result; end
  class VideoCodeResult < Result; end
  class MixedCodeResult < Result; end
  class DataCodeResult < Result; end
  class InstanceCodeResult < Result; end
  class IsccCodeResult < Result; end
  class SumCodeResult < Result; end
end
```

### Public API

```ruby
module IsccLib
  def self.gen_meta_code_v0(name, description: nil, meta: nil, bits: 64)
    MetaCodeResult[_gen_meta_code_v0(name, description, meta, bits)]
  end

  def self.gen_data_code_v0(data, bits: 64)
    if data.respond_to?(:read)
      hasher = DataHasher.new(bits)
      while chunk = data.read(4_194_304)
        hasher.update(chunk)
      end
      hasher.finalize
    else
      DataCodeResult[_gen_data_code_v0(data, bits)]
    end
  end

  def self.gen_sum_code_v0(path, bits: 64, wide: false)
    SumCodeResult[_gen_sum_code_v0(path.to_s, bits, wide)]
  end
end
```

### Target DX

```ruby
require "iscc_lib"

# Rich result objects
result = IsccLib.gen_meta_code_v0("Title", description: "A description")
result["iscc"]      # => "ISCC:..."   (Hash access)
result.iscc         # => "ISCC:..."   (attribute access)
result.name         # => "Title"
result.metahash     # => "1e20..."

# Streaming for large files
File.open("large_file.bin", "rb") do |f|
  result = IsccLib.gen_data_code_v0(f)
end

# Or path-based
result = IsccLib.gen_sum_code_v0("path/to/file.bin")
result.iscc         # => "ISCC:..."
result.datahash     # => "1e20..."
result.filesize     # => 123456
```

## Gem Distribution

### Precompiled Native Gems

Use `rb_sys` + `rake-compiler` + `rake-compiler-dock` for cross-compilation. Users get
`gem install iscc-lib` with no Rust toolchain required.

**Build platforms:**

| OS             | Target                    | Gem platform   |
| -------------- | ------------------------- | -------------- |
| ubuntu-latest  | x86_64-unknown-linux-gnu  | x86_64-linux   |
| ubuntu-latest  | aarch64-unknown-linux-gnu | aarch64-linux  |
| macos-14       | aarch64-apple-darwin      | arm64-darwin   |
| macos-14       | x86_64-apple-darwin       | x86_64-darwin  |
| windows-latest | x86_64-pc-windows-msvc    | x64-mingw-ucrt |

Plus a source gem (`.gem`) for platforms without precompiled binaries — requires Rust toolchain to
install.

### Gem Metadata

```ruby
# iscc-lib.gemspec
Gem::Specification.new do |spec|
  spec.name          = "iscc-lib"
  spec.version       = IsccLib::VERSION
  spec.authors       = ["Titusz Pan"]
  spec.email         = ["tp@py7.de"]
  spec.summary       = "ISCC - International Standard Content Code (ISO 24138)"
  spec.homepage      = "https://github.com/iscc/iscc-lib"
  spec.license       = "Apache-2.0"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata["source_code_uri"] = "https://github.com/iscc/iscc-lib"
  spec.metadata["documentation_uri"] = "https://lib.iscc.codes"

  spec.extensions = ["ext/iscc_lib/extconf.rb"]
end
```

### RubyGems Publishing

**Authentication:** RubyGems supports OIDC trusted publishing (added 2024). Configure the GitHub
Actions workflow as a trusted publisher on rubygems.org — no long-lived API key needed.

**Fallback:** If OIDC is not available or practical, use `GEM_HOST_API_KEY` repository secret.

## Account Setup Required

1. **RubyGems.org account**: Register or verify existing account at rubygems.org under the ISCC
    organization or `titusz` account
2. **Gem name reservation**: Verify `iscc-lib` gem name is available on rubygems.org and reserve it
    with an initial placeholder publish if needed
3. **OIDC trusted publisher**: Configure the `iscc/iscc-lib` repository as a trusted publisher for
    the `iscc-lib` gem on rubygems.org (under Settings → Trusted Publishers)

## CI Integration

### CI Job (ci.yml)

```yaml
ruby:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: ruby/setup-ruby@v1
      with:
        ruby-version: '3.3'
        bundler-cache: true
        working-directory: crates/iscc-rb
    - run: bundle exec rake compile
      working-directory: crates/iscc-rb
    - run: bundle exec rake test
      working-directory: crates/iscc-rb
```

### Release Jobs (release.yml)

Add `rubygems` boolean input to `workflow_dispatch` and a build matrix producing precompiled gems
for all 5 platforms. Publish via `gem push` with OIDC or API key authentication.

## DevContainer Setup

Add Ruby to the devcontainer Dockerfile:

```dockerfile
# Install Ruby (for Ruby bindings development)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ruby ruby-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
```

System Ruby from Debian Bookworm (Ruby 3.1) is sufficient for development. The CI matrix tests
against Ruby 3.1–3.3+. `bundler` ships with Ruby 3.1+.

## Version Sync

Add to `scripts/version_sync.py` sync targets:

| Target                                   | What is synced      |
| ---------------------------------------- | ------------------- |
| `crates/iscc-rb/lib/iscc_lib/version.rb` | `VERSION = "X.Y.Z"` |

The `Cargo.toml` version is authoritative (as with all other bindings). The gemspec reads from the
Ruby `VERSION` constant, which is synced by the version script.

## Documentation

- **How-to guide**: `docs/howto/ruby.md` — installation, quick start, streaming, result objects
- **API reference**: `docs/ruby-api.md` — all public methods with signatures and examples
- **README update**: Add Ruby to the per-language install and quick start sections
- **Per-crate README**: `crates/iscc-rb/README.md` for RubyGems page

## Verification Criteria

- [ ] `gem install iscc-lib` succeeds (precompiled gem, no Rust toolchain needed)
- [ ] All 10 `gen_*_v0` functions return `Hash` with the same keys/values as iscc-core
- [ ] `result["iscc"]` and `result.iscc` both work (Hash + attribute access)
- [ ] Streaming functions accept IO objects (anything with `.read`)
- [ ] `gen_sum_code_v0` accepts path strings and `Pathname` objects
- [ ] Conformance tests pass against vendored `data.json` vectors
- [ ] All 32 Tier 1 symbols are accessible from Ruby
- [ ] `DataHasher` and `InstanceHasher` streaming types work correctly
- [ ] Precompiled gems available for Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (x64)
- [ ] Source gem compiles on platforms without precompiled binaries (requires Rust)
- [ ] `bundle exec rake test` passes in CI
- [ ] RubyGems OIDC trusted publishing configured (or API key fallback)
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
- [ ] Per-crate README renders correctly on rubygems.org
- [ ] Documentation site includes Ruby how-to guide and API reference
