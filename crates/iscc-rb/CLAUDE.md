# Ruby Binding Crate (iscc-rb)

Guidance for agents working on the Ruby binding crate.

## Architecture

Magnus 0.7.1 bridge between Rust `iscc-lib` core and Ruby. Compiled as a native extension via
`rb_sys`. The compiled shared library is named `iscc_rb.so` (matching the Cargo package name
`iscc-rb` → `iscc_rb`), loaded by Ruby as `require "iscc_lib/iscc_rb"`.

## Key Files

| File                       | Purpose                                                        |
| -------------------------- | -------------------------------------------------------------- |
| `src/lib.rs`               | Magnus bindings — `#[magnus::init]` entry point                |
| `lib/iscc_lib.rb`          | Ruby API wrapper — Result classes, keyword args, native loader |
| `lib/iscc_lib/version.rb`  | `IsccLib::VERSION` constant                                    |
| `iscc-lib.gemspec`         | Gem specification (name: `iscc-lib`)                           |
| `Rakefile`                 | Build tasks — `RbSys::ExtensionTask` with gemspec              |
| `extconf.rb`               | Extension config — must be at crate root (next to Cargo.toml)  |
| `Gemfile` / `Gemfile.lock` | Dependencies pinned to specific rb_sys version                 |
| `.standard.yml`            | Ruby linting config (standard gem + rubocop-minitest)          |
| `test/test_smoke.rb`       | Smoke tests — all functions, constants, attribute access       |
| `test/test_iscc_lib.rb`    | Streaming hasher tests — DataHasher, InstanceHasher            |
| `test/test_conformance.rb` | Conformance tests against data.json vectors                    |
| `test/test_helper.rb`      | Test setup — load path and minitest require                    |

## Cross-Compilation (Critical Knowledge)

The gem ships precompiled native extensions for 5 platforms: `x86_64-linux`, `aarch64-linux`,
`x86_64-darwin`, `arm64-darwin`, `x64-mingw-ucrt`. Cross-compilation uses
`oxidize-rb/actions/cross-gem@v1` with `rb-sys-dock` Docker containers.

### Rakefile Must Pass Gemspec

```ruby
GEMSPEC = Gem::Specification.load("iscc-lib.gemspec")
RbSys::ExtensionTask.new("iscc-rb", GEMSPEC) do |ext|
  ext.lib_dir = "lib/iscc_lib"
end
```

Without the gemspec, `rake-compiler`'s `Rake::ExtensionTask` skips `native:platform` task
registration entirely (line 58 in extensiontask.rb:
`define_native_tasks if @gem_spec && @gem_spec.platform == 'ruby'`). This causes
`Don't know how to build task 'native:x86_64-linux'` inside the Docker container.

### Version-Aware Native Extension Loader

Cross-compilation for multiple Ruby versions (3.1, 3.2, 3.3) places `.so` files in version-specific
subdirectories: `lib/iscc_lib/3.1/iscc_rb.so`, `lib/iscc_lib/3.2/iscc_rb.so`, etc. The loader in
`lib/iscc_lib.rb` must try the versioned path first:

```ruby
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require_relative "iscc_lib/#{$1}/iscc_rb"
rescue LoadError
  require_relative "iscc_lib/iscc_rb"
end
```

The fallback handles source-compiled installations where the `.so` is at the flat path.

### rb_sys Version Pinning

`Gemfile.lock` pins `rb_sys` to a specific version (e.g., 0.9.123). This is critical because:

1. Each rb_sys version bundles a specific `rake-compiler-dock` version
2. `rake-compiler-dock` contains a `cross_rubies` hash mapping minor versions to patch versions
    (e.g., "3.3" → "3.3.9" in 1.10.0, vs "3.3" → "3.3.10" in 1.11.0)
3. The Docker image must match — image 0.9.123 has rbconfig for 3.3.9, not 3.3.10
4. A version mismatch causes `no configuration section for specified version of Ruby`

### Gemfile.lock Symlink in CI

The `oxidize-rb/actions/cross-gem@v1` action's configure step runs `grep rb_sys Gemfile.lock` in the
**repository root** (ignoring `working-directory`). When the gem crate is in a subdirectory, the
grep fails and falls back to `gem info rb_sys --remote`, installing the latest version from
RubyGems.

**Fix:** The release workflow creates a symlink: `ln -sf crates/iscc-rb/Gemfile.lock Gemfile.lock`

### Docker Image Tag

The `tag` input to `cross-gem` controls which Docker image is used. Pin it to match the rb_sys
version in `Gemfile.lock`. Mismatched images may have Ruby versions (e.g., Ruby 4.0 in 0.9.124) that
break `RbSys::ExtensionTask`.

## Publishing

RubyGems publishing uses **OIDC trusted publishing** (no API keys). Configured via
`rubygems/configure-rubygems-credentials@main` with `id-token: write` permission. The trusted
publisher is registered on rubygems.org for `iscc/iscc-lib` + `release.yml`.

## Magnus Patterns

- `#[magnus::wrap(class = "IsccLib::ClassName")]` for streaming classes
- `RefCell<Option<inner>>` pattern (Magnus gives `&self`, not `&mut self`)
- All native functions use `_` prefix (`_gen_meta_code_v0`, `_update`, `_finalize`) to avoid
    collision with Ruby wrapper methods; class prefix `_DataHasher` does NOT work (Ruby constants
    must start with uppercase)
- `unsafe { data.as_slice() }` for zero-copy binary data from `RString` — safe only when the slice
    is consumed immediately without intervening Ruby API calls that could trigger GC
- `RString::from_slice` to copy borrowed Rust slices into Ruby strings

## Local Development

```bash
cd crates/iscc-rb
bundle install                  # Install Ruby dependencies
bundle exec rake compile:dev    # Build debug extension
bundle exec rake test           # Run tests
bundle exec standardrb --fix    # Lint/format
```

Requires `libclang-dev` for rb-sys/bindgen compilation.

## Common Pitfalls

- **Cargo lib name must match package name** — `iscc_rb`, not `iscc_lib`. rb_sys derives the binary
    name from the package name
- **Root `.gitignore` has `lib/` pattern** — need `!lib/` negation in `crates/iscc-rb/.gitignore`
- **`JSON.generate` ignores `sort_keys: true`** — use `meta_val.sort.to_h` before `JSON.generate`
- **`bundler` not on PATH** in devcontainer — need `$HOME/.local/share/gem/ruby/3.1.0/bin` on PATH
