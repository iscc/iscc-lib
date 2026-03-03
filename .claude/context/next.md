# Next Work Package

## Step: Scaffold iscc-rb crate with Magnus bridge and gem infrastructure

## Goal

Create the Ruby bindings crate (`crates/iscc-rb/`) with a working Magnus bridge, Ruby gem
infrastructure, and a subset of Tier 1 symbols. This proves the full compile → load → call pipeline
end-to-end before investing in all 32 symbols. Addresses the "Implement Ruby bindings via Magnus"
issue.

## Scope

- **Create**:
    - `crates/iscc-rb/Cargo.toml` — cdylib crate depending on `iscc-lib` + `magnus`
    - `crates/iscc-rb/src/lib.rs` — Magnus bridge with initial symbols (~10 of 32)
    - `crates/iscc-rb/ext/iscc_lib/extconf.rb` — rb_sys extension config
    - `crates/iscc-rb/lib/iscc_lib.rb` — Pure Ruby wrapper (result classes + public API for
        implemented symbols)
    - `crates/iscc-rb/lib/iscc_lib/version.rb` — VERSION constant
    - `crates/iscc-rb/iscc-lib.gemspec` — Gem specification
    - `crates/iscc-rb/Gemfile` — Development dependencies
    - `crates/iscc-rb/Rakefile` — rb_sys build tasks
    - `crates/iscc-rb/test/test_helper.rb` — Test setup (Minitest)
    - `crates/iscc-rb/test/test_smoke.rb` — Smoke test for implemented symbols
    - `crates/iscc-rb/README.md` — Per-crate README for RubyGems
- **Modify**:
    - `Cargo.toml` (root) — add `"crates/iscc-rb"` to workspace members, add `magnus` to
        `[workspace.dependencies]`
    - `.devcontainer/Dockerfile` — add `libclang-dev` to apt-get line (needed by rb-sys/bindgen)
- **Reference**:
    - `.claude/context/specs/ruby-bindings.md` — full spec with code patterns
    - `crates/iscc-py/src/lib.rs` — PyO3 bridge (pattern to follow)
    - `crates/iscc-py/Cargo.toml` — binding crate config pattern
    - `crates/iscc-jni/Cargo.toml` — another binding crate config pattern

## Not In Scope

- Implementing all 32 Tier 1 symbols (only ~10 in this step; remaining symbols in a follow-up)
- CI job in `ci.yml` (separate step after the crate compiles and tests pass locally)
- Release workflow changes in `release.yml`
- Version sync in `scripts/version_sync.py`
- Documentation (`docs/howto/ruby.md`, `docs/ruby-api.md`)
- README update with Ruby install/quickstart section
- Precompiled gem cross-compilation setup (rake-compiler-dock)
- Streaming types (`DataHasher`, `InstanceHasher` Ruby wrappers) — defer to a later step

## Implementation Notes

### Initial Symbols (~10 of 32)

Implement these to prove the binding pattern:

1. **`gen_meta_code_v0(name, description, meta, bits)`** → Ruby Hash (the flagship function, proves
    Hash return + optional params)
2. **`text_clean(text)`** → String
3. **`text_remove_newlines(text)`** → String
4. **`text_trim(text, max_bytes)`** → String
5. **`text_collapse(text)`** → String
6. **Constants**: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
    `TEXT_NGRAM_SIZE`

### Magnus Bridge Pattern

Follow the spec in `ruby-bindings.md`. Key pattern:

```rust
use magnus::{function, prelude::*, Error, Ruby, RHash};

fn gen_meta_code_v0(name: String, description: Option<String>, meta: Option<String>, bits: u32) -> Result<RHash, Error> {
    let result = iscc_lib::gen_meta_code_v0(&name, description.as_deref(), meta.as_deref(), bits);
    let hash = RHash::new();
    hash.aset("iscc", result.iscc)?;
    // ... set other fields
    Ok(hash)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("IsccLib")?;
    module.define_module_function("_gen_meta_code_v0", function!(gen_meta_code_v0, 4))?;
    // ...
    Ok(())
}
```

### Cargo.toml Pattern

```toml
[package]
name = "iscc-rb"
version.workspace = true
edition.workspace = true
publish = false

[lib]
name = "iscc_lib"
crate-type = ["cdylib"]

[dependencies]
iscc-lib = { path = "../iscc-lib" }
magnus = { workspace = true }
```

### Ruby Gem Infrastructure

- `extconf.rb`: Use `require "mkmf"` + `require "rb_sys/mkmf"` to configure the Rust extension
    build. Call `create_rust_makefile("iscc_lib/iscc_lib")`.
- `Gemfile`: `gem "rake-compiler"`, `gem "rb_sys"`, `gem "minitest"`
- `Rakefile`: `require "rb_sys/extensiontask"` to define the compile task
- `iscc-lib.gemspec`: See spec for metadata. `spec.extensions = ["ext/iscc_lib/extconf.rb"]`
- `lib/iscc_lib/version.rb`: `module IsccLib; VERSION = "0.1.0"; end`
- `lib/iscc_lib.rb`: Require the native extension, define Result classes (see spec), define public
    API wrappers that call `_gen_meta_code_v0` etc.

### Dev Environment Setup

1. `libclang-dev` is needed by rb-sys/bindgen to generate Ruby C extension bindings. Add to
    Dockerfile apt-get line. For immediate use:
    `sudo apt-get update && sudo apt-get install -y  libclang-dev`
2. `bundler` is available at `~/.local/share/gem/ruby/3.1.0/bin/bundle` (user-install). Run
    `gem install bundler --user-install` if not present, and add
    `$HOME/.local/share/gem/ruby/3.1.0/bin` to PATH.
3. Ruby headers are at `/usr/include/ruby-3.1.0` (ruby-dev package already installed).
4. `rb_sys` gem must be installed: `gem install rb_sys --user-install`

### Important: Magnus Version

Use `magnus = "0.7"` (latest 0.7.x). The 0.7 line is the stable release supporting Ruby 3.1+. Check
crates.io for the latest patch version. The 0.8.x line exists but verify Ruby 3.1 compatibility. If
0.7.x has issues with Rust edition 2024, try 0.8.x instead.

### rb-sys Build Flow

The compile flow is: `bundle exec rake compile` → runs extconf.rb → invokes `cargo build` on the
Rust crate → produces a `.so` (Linux) / `.bundle` (macOS) / `.dll` (Windows) → copies to
`lib/iscc_lib/`. The Rakefile's `RbSys::ExtensionTask` handles this automatically.

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` is clean
- `bundle exec rake compile` succeeds in `crates/iscc-rb/` (compiles the native extension)
- `bundle exec ruby -e "require 'iscc_lib'; puts IsccLib.gen_meta_code_v0('Hello')['iscc']"` prints
    a valid ISCC string starting with `ISCC:`
- `bundle exec rake test` passes the smoke test
- All files listed in Scope → Create exist at their expected paths
- Root `Cargo.toml` includes `"crates/iscc-rb"` in workspace members
- `magnus` appears in `[workspace.dependencies]`

## Done When

The advance agent is done when `cargo check -p iscc-rb` compiles, `bundle exec rake compile` builds
the native extension, and `bundle exec rake test` passes the smoke test confirming
`gen_meta_code_v0` returns a valid ISCC Hash from Ruby.
