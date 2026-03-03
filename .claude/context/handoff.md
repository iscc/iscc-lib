## 2026-03-03 — Scaffold iscc-rb crate with Magnus bridge and gem infrastructure

**Done:** Created the `crates/iscc-rb/` Ruby bindings crate with a working Magnus 0.7.1 bridge, Ruby
gem infrastructure, and 10 of 32 Tier 1 symbols. The full compile → load → call pipeline works
end-to-end: `gen_meta_code_v0` returns a valid ISCC Hash from Ruby, and all 5 constants are
accessible. 10 Minitest assertions pass covering gen function, text utilities, constants, and
attribute-style access on Result objects.

**Files changed:**

- `crates/iscc-rb/Cargo.toml`: cdylib crate depending on `iscc-lib` + `magnus` (workspace dep)
- `crates/iscc-rb/src/lib.rs`: Magnus bridge with 6 functions (`_gen_meta_code_v0`, `text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`) and 5 constants
- `crates/iscc-rb/extconf.rb`: rb_sys extension config (at crate root, not `ext/` — required by
    rb_sys ExtensionTask)
- `crates/iscc-rb/lib/iscc_lib.rb`: Pure Ruby wrapper with `Result` base class, `MetaCodeResult`,
    and `gen_meta_code_v0` keyword-arg public API
- `crates/iscc-rb/lib/iscc_lib/version.rb`: `VERSION = "0.1.0"` constant
- `crates/iscc-rb/iscc-lib.gemspec`: Gem specification with proper metadata
- `crates/iscc-rb/Gemfile`: Development dependencies (rake-compiler, rb_sys, minitest)
- `crates/iscc-rb/Gemfile.lock`: Locked dependency versions
- `crates/iscc-rb/Rakefile`: rb_sys ExtensionTask + Minitest task
- `crates/iscc-rb/test/test_helper.rb`: Minitest setup
- `crates/iscc-rb/test/test_smoke.rb`: 10 smoke tests (24 assertions)
- `crates/iscc-rb/README.md`: Per-crate README for RubyGems
- `crates/iscc-rb/.gitignore`: Ignores vendor/, tmp/, \*.so, .bundle/; re-includes lib/
- `Cargo.toml` (root): Added `"crates/iscc-rb"` to workspace members,
    `magnus = { version = "0.7",   features = ["rb-sys"] }` to workspace deps
- `.devcontainer/Dockerfile`: Added `libclang-dev` to apt-get line (needed by rb-sys/bindgen)

**Verification:**

- `cargo check -p iscc-rb` — compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean, zero warnings
- `bundle exec rake compile` — builds native extension (release profile, ~750 KB .so)
- `bundle exec ruby -e "require 'iscc_lib'; puts IsccLib.gen_meta_code_v0('Hello')['iscc']"` —
    prints `ISCC:AAAWKLHFXM75OAMK`
- `bundle exec rake test` — 10 runs, 24 assertions, 0 failures, 0 errors, 0 skips
- `mise run check` — all 14 pre-commit hooks pass

**Next:** Implement the remaining ~22 Tier 1 symbols in the Magnus bridge (all other `gen_*_v0`
functions, codec functions, algorithm functions, streaming types). Then add CI job in `ci.yml` and
conformance tests against `data.json`.

**Notes:**

- **Magnus version:** Used 0.7.1 (not 0.8) because Magnus 0.8 requires Ruby 3.2+ and the
    devcontainer has Ruby 3.1.2. Magnus 0.7.1 works fine with Rust edition 2024 and Ruby 3.1.
- **extconf.rb location:** rb_sys `ExtensionTask` expects `extconf.rb` at the Cargo manifest
    directory (crate root), not in `ext/iscc_lib/`. Moved from the spec's suggested location.
- **Cargo lib name:** Changed from `iscc_lib` to `iscc_rb` to match the Cargo package name
    normalization. rb_sys ExtensionTask derives the binary name from the package name (`iscc-rb` →
    `iscc_rb`), and it must match the compiled library name. Ruby loads via
    `require_relative "iscc_lib/iscc_rb"`.
- **Root `.gitignore` has `lib/` pattern:** This blocks all `lib/` directories at any depth. Added
    `!lib/` negation in `crates/iscc-rb/.gitignore` to re-include the Ruby source directory.
- **Bundler vendor path:** Gems installed to `vendor/bundle` (local path, not system gems) since the
    devcontainer's system gem path (`/var/lib/gems/3.1.0`) is not writable by the dev user. The
    `.bundle/config` and `vendor/` are gitignored.
- **Magnus `function!` macro:** Does not accept `&Ruby` as a function parameter (unlike PyO3's
    `Python<'_>`). Use `Ruby::get().expect("called from Ruby")` inside the function body to access
    the Ruby runtime.
- **Scope note:** Only `MetaCodeResult` is defined in the Ruby wrapper — the other 9 result classes
    will be added when their corresponding gen functions are implemented.
