## 2026-03-04 — Configure Standard Ruby linting

**Done:** Set up Standard Ruby (`standard` gem v1.54.0) with `rubocop-minitest` plugin as the
linter/formatter for the Ruby binding crate. Added CI enforcement step, pre-commit auto-fix hook
(pre-commit stage), and pre-push check hook. Fixed 8 mechanical lint issues across test files
(argument alignment, minitest assertion style).

**Files changed:**

- `crates/iscc-rb/.standard.yml`: Created — Standard Ruby config with rubocop-minitest plugin and
    vendor directory exclusion
- `crates/iscc-rb/Gemfile`: Added `standard ~> 1.0` and `rubocop-minitest ~> 0.36` gems
- `crates/iscc-rb/Gemfile.lock`: Updated by bundle install (new gems: standard, rubocop,
    rubocop-minitest, rubocop-performance, and transitive deps)
- `.github/workflows/ci.yml`: Added `bundle exec standardrb` step in Ruby job before clippy
- `.pre-commit-config.yaml`: Added `standardrb-fix` hook (pre-commit stage, auto-fix) and
    `standardrb` hook (pre-push stage, check-only). Both use portable `ruby -e "puts Gem.user_dir"`
    for PATH resolution
- `crates/iscc-rb/test/test_smoke.rb`: Fixed 5 lint issues — `refute_includes` instead of
    `refute result.include?`, `refute_empty` instead of `refute result.empty?`, `assert` instead of
    `assert_equal true`
- `crates/iscc-rb/test/test_conformance.rb`: Fixed 3 argument alignment issues (multi-line method
    calls use one level of indentation per Standard Ruby style)

**Verification:**

- `bundle exec standardrb` exits 0 — all Ruby files pass
- `bundle exec rake test` — 111 runs, 299 assertions, 0 failures, 0 errors, 0 skips
- `mise run check` — all 15 pre-commit hooks pass (including new Ruby linting hook)
- `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` — clean

**Next:** The Ruby bindings linting setup is complete. Remaining Ruby items are: (1)
`docs/ruby-api.md` API reference page, (2) RubyGems account setup for publishing. The root README Go
quickstart example is stale (uses old WASM-bridge API) — worth fixing in a housekeeping step.

**Notes:**

- The `standardrb --fix` auto-fixed 7 of 8 issues. The remaining one (`Minitest/AssertTruthy`:
    `assert_equal true, x` → `assert x`) was flagged as "unsafe" auto-fix because it changes
    assertion semantics (truthy vs strict equality). Fixed manually since `conformance_selftest`
    returns a boolean — `assert` is the correct idiom.
- Pre-commit hook uses `bash -c '...'` wrapper with `pass_filenames: false` and `files:` pattern to
    scope to Ruby files. The PATH issue (`bundle` not on system PATH in devcontainer) is solved
    portably via `ruby -e "puts Gem.user_dir"` to locate the gem bin directory at runtime.
- yamlfix reformatted the long `entry:` strings into multi-line YAML scalars — this is expected
    behavior and the hooks work correctly with the wrapped format.
- The `.standard.yml` includes `ignore: vendor/**/*` to skip vendored gem files in
    `crates/iscc-rb/vendor/bundle/`. Standard Ruby ignores `vendor/` by default, but explicit config
    ensures clarity.
