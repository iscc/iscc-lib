## 2026-03-03 — Review of: Scaffold iscc-rb crate with Magnus bridge and gem infrastructure

**Verdict:** PASS

**Summary:** The advance agent created a fully functional Ruby bindings scaffold with 10 of 32 Tier
1 symbols. The Magnus bridge compiles, `bundle exec rake compile` builds a ~750 KB native extension,
`gen_meta_code_v0` returns valid ISCC codes from Ruby, and all 10 Minitest smoke tests (24
assertions) pass. Code quality is clean — no clippy warnings, well-structured docstrings, idiomatic
Ruby wrapper with `Result < Hash` pattern.

**Verification:**

- [x] `cargo check -p iscc-rb` compiles successfully
- [x] `cargo clippy -p iscc-rb -- -D warnings` is clean (0 warnings)
- [x] `bundle exec rake compile` succeeds (builds native extension in release profile)
- [x] `bundle exec ruby -e "require 'iscc_lib'; puts IsccLib.gen_meta_code_v0('Hello')['iscc']"`
    prints `ISCC:AAAWKLHFXM75OAMK`
- [x] `bundle exec rake test` — 10 runs, 24 assertions, 0 failures, 0 errors, 0 skips
- [x] All files listed in Scope → Create exist at expected paths
- [x] Root `Cargo.toml` includes `"crates/iscc-rb"` in workspace members
- [x] `magnus` appears in `[workspace.dependencies]` with `version = "0.7"` +
    `features = ["rb-sys"]`
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean across entire workspace

**Issues found:**

- CI `rust` job uses `cargo clippy --workspace` and `cargo test --workspace` — with `iscc-rb` now a
    workspace member, these would fail on CI runners without Ruby dev headers (`ruby-dev`,
    `libclang-dev`). **Fixed:** added `--exclude iscc-rb` to both commands in `ci.yml`. The proper
    Ruby CI job will be added in a follow-up step, at which point the exclusion can be revisited.

**Codex review:** Three findings, all evaluated:

1. **P0 — Cargo.toml not standalone for gem builds**: Valid for source gem distribution, but
    precompiled gem distribution (the intended model per spec) handles this differently. Deferred
    to the release workflow step.
2. **P1 — rb_sys not a gemspec runtime dependency**: Same as above — source gem compilation path.
    Will be addressed when release infrastructure is set up.
3. **P2 — Workspace builds depend on Ruby**: Valid and actionable. Fixed by adding
    `--exclude iscc-rb` to CI workspace commands. Pre-push hooks work locally (devcontainer has
    Ruby).

**Next:** Implement the remaining ~22 Tier 1 symbols in the Magnus bridge. This includes all other
`gen_*_v0` functions (text, image, audio, video, mixed, data, instance, iscc, sum), codec functions
(`iscc_encode`, `iscc_decode`, `iscc_decompose`, `iscc_normalize`), algorithm functions
(`alg_simhash_from_iscc`), and streaming types (`DataHasher`, `InstanceHasher` Ruby wrappers). After
that, add conformance tests against `data.json` and a `ruby` CI job in `ci.yml`.

**Notes:**

- **Magnus version choice**: 0.7.1 (not 0.8) because devcontainer Ruby is 3.1.2 and Magnus 0.8
    requires Ruby 3.2+. This works fine with Rust edition 2024.
- **extconf.rb location**: At crate root (not `ext/iscc_lib/`), as rb_sys `ExtensionTask` expects it
    next to `Cargo.toml`. This deviates from the spec's suggested path but is functionally required.
- **Cargo lib name**: `iscc_rb` (not `iscc_lib`) to match package name for rb_sys binary name
    derivation. Ruby loads via `require_relative "iscc_lib/iscc_rb"`.
- **PATH for bundler**: `~/.local/share/gem/ruby/3.1.0/bin` must be on PATH to use `bundle` in the
    devcontainer. Not an issue for CI (uses `ruby/setup-ruby` action).
- **Root `.gitignore` `lib/` pattern**: The `!lib/` negation in `crates/iscc-rb/.gitignore` is
    essential — without it, Ruby source files under `lib/` would be gitignored.
