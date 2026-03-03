## 2026-03-03 — Review of: Scaffold iscc-rb crate with Magnus bridge and gem infrastructure

**Verdict:** PASS

**Summary:** The advance agent created a fully functional Ruby bindings scaffold with 10 of 32 Tier
1 symbols. The Magnus bridge compiles, `bundle exec rake compile` builds a native extension in
release profile, `gen_meta_code_v0` returns valid ISCC codes from Ruby, and all 10 Minitest smoke
tests (24 assertions) pass. Code quality is clean — no clippy warnings, well-structured docstrings,
idiomatic Ruby wrapper with `Result < Hash` pattern. All verification criteria met. No quality gate
circumvention. Scope discipline is excellent — no out-of-scope files touched.

**Verification:**

- [x] `cargo check -p iscc-rb` compiles successfully
- [x] `cargo clippy -p iscc-rb -- -D warnings` is clean (0 warnings)
- [x] `bundle exec rake compile` succeeds (builds native extension in release profile)
- [x] `bundle exec ruby -e "require 'iscc_lib'; puts IsccLib.gen_meta_code_v0('Hello')['iscc']"`
    prints `ISCC:AAAWKLHFXM75OAMK`
- [x] `bundle exec rake test` — 10 runs, 24 assertions, 0 failures, 0 errors, 0 skips
- [x] All files listed in Scope → Create exist at expected paths (11 of 11)
- [x] Root `Cargo.toml` includes `"crates/iscc-rb"` in workspace members (line 10)
- [x] `magnus` appears in `[workspace.dependencies]` with `version = "0.7"` +
    `features = ["rb-sys"]` (line 44)
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean across entire workspace

**Issues found:**

- CI `rust` job now uses `--exclude iscc-rb` for workspace clippy/test (fixed in the previous review
    commit). A dedicated Ruby CI job is needed as a follow-up step.

**Codex review:** One finding: [P2] Add a dedicated CI job for `iscc-rb` since it's now excluded
from the workspace clippy/test commands in CI. Valid — this is already planned as a near-term
follow-up (listed in the Ruby bindings issue scope).

**Next:** Implement the remaining ~22 Tier 1 symbols in the Magnus bridge. This is a large step that
should be broken into manageable chunks. Suggested ordering:

1. **Codec functions** (`iscc_encode`, `iscc_decode`, `iscc_decompose`, `iscc_normalize`) — these
    are the most widely used utility functions after gen_meta
2. **Remaining gen functions** (`gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
    `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
    `gen_iscc_code_v0`, `gen_sum_code_v0`) — each needs a Ruby Result class
3. **Algorithm function** (`alg_simhash_from_iscc`) — single function, straightforward
4. **Streaming types** (`DataHasher`, `InstanceHasher`) — Ruby class wrappers around the Rust
    streaming processors

After all symbols are implemented, add conformance tests against `data.json` and a `ruby` CI job.

**Notes:**

- **Magnus version**: 0.7.1 (not 0.8) because devcontainer Ruby is 3.1.2 and Magnus 0.8 requires
    Ruby 3.2+. Works fine with Rust edition 2024.
- **extconf.rb location**: At crate root (not `ext/iscc_lib/`), as rb_sys `ExtensionTask` expects it
    next to `Cargo.toml`. This deviates from the spec's suggested path but is functionally required.
- **Cargo lib name**: `iscc_rb` (not `iscc_lib`) to match package name for rb_sys binary name
    derivation. Ruby loads via `require_relative "iscc_lib/iscc_rb"`.
- **PATH for bundler**: `~/.local/share/gem/ruby/3.1.0/bin` must be on PATH to use `bundle` in the
    devcontainer. Not an issue for CI (uses `ruby/setup-ruby` action).
- **Root `.gitignore` `lib/` pattern**: The `!lib/` negation in `crates/iscc-rb/.gitignore` is
    essential — without it, Ruby source files under `lib/` would be gitignored.
- **`function!` macro**: Magnus 0.7's `function!` does NOT accept `&Ruby` as a parameter. Use
    `Ruby::get().expect("called from Ruby")` inside the function body to get the handle.
- **`_` prefix convention**: Gen functions are exposed to Ruby as `_gen_*_v0` (with underscore
    prefix) and the pure Ruby wrapper in `lib/iscc_lib.rb` provides the public API with keyword
    args. Text utility functions are exposed directly (no wrapper needed).
