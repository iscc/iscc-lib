## 2026-03-03 — Add Ruby CI job to ci.yml

**Done:** Added a dedicated `ruby` CI job to `.github/workflows/ci.yml` that installs libclang-dev,
sets up Ruby 3.1 with bundler-cache, runs clippy on iscc-rb, compiles the native extension, and runs
all Ruby tests. Placed after the `go` job and before the `bench` job.

**Files changed:**

- `.github/workflows/ci.yml`: Added `ruby` job (key `ruby`, display name
    `Ruby (magnus build, test)`) with 8 steps: checkout, rust-toolchain (with clippy), rust-cache,
    libclang-dev install, ruby/setup-ruby (3.1, bundler-cache), clippy, compile, test

**Verification:** All 6 verification criteria pass:

- `grep -c 'name: Ruby'` returns 1
- `grep 'cargo clippy -p iscc-rb'` finds the clippy step
- `grep 'bundle exec rake test'` finds the test step
- `grep 'libclang-dev'` finds the dependency install step
- `grep -c 'exclude iscc-rb'` returns 2 (Rust job keeps its excludes)
- YAML validates successfully
- `mise run check` passes all 14 pre-commit hooks

**Next:** The Ruby binding infrastructure is now complete (32/32 symbols, 111 tests, CI job). Next
logical steps: version_sync.py gemspec integration, release.yml RubyGems publish job, or
docs/howto/ruby.md documentation.

**Notes:** The job follows the exact pattern from next.md. The `ruby/setup-ruby@v1` action's
`working-directory` parameter is set as an action `with:` input (not step-level), which tells
bundler where to find the Gemfile. Clippy runs before `rake compile` so it catches lint errors
faster and populates the cargo cache for the subsequent compile step.
