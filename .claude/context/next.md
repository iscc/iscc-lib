# Next Work Package

## Step: Configure Standard Ruby linting

## Goal

Set up Standard Ruby (`standard` gem) as the linter/formatter for the Ruby binding crate, with CI
enforcement and pre-commit hook integration. This brings Ruby code quality tooling to parity with
the Rust (clippy), Python (ruff), and TOML (taplo) linting already in place.

## Scope

- **Create**: `crates/iscc-rb/.standard.yml` (Standard Ruby config with rubocop-minitest plugin)
- **Modify**: `crates/iscc-rb/Gemfile` (add `standard` and `rubocop-minitest` gems)
- **Modify**: `.github/workflows/ci.yml` (add `bundle exec standardrb` step in the `ruby` job)
- **Modify**: `.pre-commit-config.yaml` (add standardrb hooks for pre-commit auto-fix and pre-push
    check)
- **Fix**: Any Ruby source files (`lib/iscc_lib.rb`, `Rakefile`, test files) that fail `standardrb`
    — these are mechanical lint fixes, not feature work
- **Reference**:
    - `.pre-commit-config.yaml` — existing hook patterns (ruff-check --fix, cargo-clippy)
    - `.github/workflows/ci.yml` lines 165-188 — existing Ruby CI job
    - `crates/iscc-rb/Gemfile` — current gem dependencies
    - `crates/iscc-rb/lib/iscc_lib.rb` — main Ruby source (209 lines)
    - `crates/iscc-rb/test/test_smoke.rb` — largest test file (361 lines)

## Not In Scope

- Creating `docs/ruby-api.md` API reference page — that's a separate documentation step
- Modifying Ruby bridge Rust code (`src/lib.rs`) — the linter only applies to `.rb` files
- Adding custom RuboCop rules beyond what Standard Ruby provides by default
- Upgrading Magnus or other Rust/Ruby dependencies
- Any changes to the Ruby API surface or test coverage

## Implementation Notes

1. **Gemfile additions** — add to `crates/iscc-rb/Gemfile`:

    ```ruby
    gem "standard", "~> 1.0"
    gem "rubocop-minitest", "~> 0.36"
    ```

2. **`.standard.yml`** — create at `crates/iscc-rb/.standard.yml`:

    ```yaml
    plugins:
      - rubocop-minitest
    ```

    Standard Ruby uses its own config format (not `.rubocop.yml`). The `plugins` key loads the
    minitest extension for test-specific lint rules.

3. **CI integration** — add a step in the `ruby` job in `.github/workflows/ci.yml`, BEFORE
    `rake compile` (linting doesn't need compiled extensions):

    ```yaml
      - name: Run standardrb
        run: bundle exec standardrb
        working-directory: crates/iscc-rb
    ```

    This follows the pattern of clippy running before compile in the same job.

4. **Pre-commit hooks** — add two local hooks in `.pre-commit-config.yaml`:

    - Pre-commit stage (auto-fix): `standardrb --fix` on Ruby files, matching the `ruff-check --fix`
        pattern
    - Pre-push stage (check): `standardrb` without --fix, matching the `cargo-clippy` pattern
    - Use `language: system`, `types: [ruby]`
    - The hooks need `bundle exec` and must run from `crates/iscc-rb/`. Use a
        `bash -c 'cd crates/iscc-rb && bundle exec standardrb ...'` wrapper or the prek `entry` with
        `pass_filenames: false` + `files: ^crates/iscc-rb/` pattern. Note: prek is a pre-commit
        drop-in — it supports the same config keys

5. **Fix lint issues** — run `cd crates/iscc-rb && bundle exec standardrb --fix` to auto-fix issues.
    Review any remaining non-auto-fixable issues. Standard Ruby defaults:

    - Double quotes for strings (Ruby convention)
    - Frozen string literal comments (`# frozen_string_literal: true`)
    - Consistent indentation (2 spaces)
    - No trailing whitespace (already handled by pre-commit hygiene hooks)

6. **Verify bundle install works** — after adding gems, run `cd crates/iscc-rb && bundle install` to
    ensure the Gemfile.lock updates cleanly. Commit the updated `Gemfile.lock`.

## Verification

- `cd crates/iscc-rb && bundle exec standardrb` exits 0 (all Ruby files pass linting)
- `cd crates/iscc-rb && bundle exec rake compile && bundle exec rake test` still passes (111 tests,
    295 assertions)
- `grep 'standard' crates/iscc-rb/Gemfile` finds the gem entry
- `test -f crates/iscc-rb/.standard.yml` — config file exists
- `grep 'standardrb' .pre-commit-config.yaml` finds the hook entries
- `grep 'standardrb' .github/workflows/ci.yml` finds the CI step
- `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` remains clean

## Done When

All verification criteria pass — `standardrb` runs clean on all Ruby files, CI has the lint step,
pre-commit hooks are configured, and existing tests still pass.
