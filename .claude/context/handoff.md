## 2026-11-21 — Raise the Ruby floor to 3.3 and precompile gems for Ruby 3.3 and 3.4

**Done:** The gemspec declares `>= 3.3.0`, the CI `ruby` job tests 3.3, and `release.yml` cross-gem
builds `3.3, 3.4` on the unchanged `tag: 0.9.123` image (comment corrected to list
3.4.5/3.3.9/3.2.9/3.1.7). minitest moved to `~> 6.0` (6.1.0) with no test edits. A floor-parity test
ties the gemspec, CI and release floors. Two fixes outside the listed scope were needed to keep the
gates green (see Notes).

**Files changed:**

- `crates/iscc-rb/iscc-lib.gemspec`: `required_ruby_version = ">= 3.3.0"`
- `crates/iscc-rb/Gemfile`: hold comment dropped, `minitest "~> 6.0"`. `Gemfile.lock`: minitest
    6.1.0 (+ `drb`), `rb_sys (0.9.123)` unchanged (`bundle update minitest --conservative`, Ruby
    3.4.13)
- `.github/workflows/ci.yml`: `ruby` job `ruby-version: '3.3'`. `ruby-source-gem` comment reworded:
    it installs the built source-gem file, so it covers the source path on any Ruby
- `.github/workflows/release.yml`: `ruby-versions: 3.3, 3.4`, image comment corrected
- `.pre-commit-config.yaml` (**unscoped**): both standardrb hooks run
    `mise exec ruby@3.4 -- bundle exec standardrb[ --fix]` instead of system `ruby`
- `tests/test_ruby_gem_docs.py`: `gemspec_floor`, `ci_ruby_job_version`, `check_floor_parity` +
    `test_ruby_floor_parity` (real files) + 8 unit tests (equal trio passes; one mismatch per
    position, one per call; missing gemspec floor; missing ruby job; missing setup-ruby version)
- `tests/test_build_ruby_source_gem.py` (**unscoped**): see Notes
- Docs: `crates/iscc-rb/README.md`, `docs/howto/ruby.md` (`Ruby 3.3 to 3.4`, `>= 3.3`),
    `crates/iscc-rb/CLAUDE.md` (3.3/3.4 loader paths; pitfall now gives the mise command),
    `.claude/context/specs/ruby-bindings.md` (gemspec snippet, dev Ruby paragraph)

**Verification:**

- Ruby suite, `mise exec ruby@3.4.13 -- bundle exec rake compile test`: minitest 5.27.0 →
    `168 runs, 481 assertions, 0 failures, 0 errors, 0 skips`. minitest 6.1.0 → the same counts
- `mise exec ruby@3.4.13 -- bundle exec standardrb` exits 0 (no new cop findings)
- `uv run pytest tests/test_ruby_gem_docs.py tests/test_build_ruby_source_gem.py`: 47 passed. Full
    `uv run pytest`: 809 passed
- Floor parsers checked against the HEAD (pre-change) files: they read `3.1 3.1 3.1`, and
    `check_floor_parity('3.1','3.1','3.3')` raises `Ruby floor mismatch`
- The four greps print one line each (release.yml:850, gemspec:16, ci.yml:253, Gemfile.lock:28). The
    `3\.1 to 3\.3|>= 3\.1|3\.1, 3\.2` sweep prints nothing (exit 1)
- `uv run scripts/check_release_workflow.py` exits 0. `mise run check` exits 0 (the first run
    reformatted the hook YAML via yamlfix, the second was clean). Pre-push
    `prek run standardrb --hook-stage pre-push --all-files` passes
- Not verified: the suite on Ruby 3.3 (no mise 3.3 here; CI's `ruby` job is the first run) and the
    cross-gem build for 3.4 (release.yml never runs in CI; image support is from next.md's sources)

**Next:** the decided `normal` cleanup "C++ `safe_data` and .NET `PinnedRows` padding". Ruby 4.0
precompiled gems (rb_sys ≥ 0.9.124) stay a separate, verify-on-its-own step.

**Notes:**

- **Unscoped hook change (6th non-doc file, within budget):** the floor alone breaks the standardrb
    hooks on the devcontainer's system Ruby 3.1. Proved in a `/tmp` copy with minitest 5 kept:
    Bundler fails with
    `every version of iscc-lib depends on Ruby >= 3.3.0 ... current Ruby version   is = 3.1.2`. So
    `mise run check` could not pass without it. The hooks now need mise's Ruby 3.4
    (`mise exec ruby@3.4` uses an installed 3.4.x, else installs one). Ruby is NOT pinned in
    `mise.toml`, so a fresh machine downloads Ruby on the first `.rb` commit. Titusz may prefer a
    `mise.toml` pin or a devcontainer Ruby bump instead
- **Unscoped test change:** `test_ci_source_gem_install_uses_ruby_without_precompiled_gem` asserted
    the `ruby-source-gem` job's Ruby (3.4) has no precompiled gem. That is false by design now
    (next.md keeps the job on 3.4). Replaced with
    `test_ci_source_gem_install_takes_built_source_gem_file`: the install runs
    `gem install <--out dir>/iscc-lib-*.gem` (the file, not a registry lookup), plus the unchanged
    empty-gem-home/selftest assertions. The module docstring follows
- **Devcontainer gap:** `.devcontainer/Dockerfile` still installs Debian's Ruby 3.1, below the
    floor. It can no longer run the Ruby suite or `bundle exec` in `crates/iscc-rb`. Use
    `mise exec ruby@3.4.13 -- …` with `LANG=C.UTF-8` (documented in `crates/iscc-rb/CLAUDE.md`)
- Loader: `lib/iscc_lib.rb` derives `3.4` from `RUBY_VERSION` and tries `iscc_lib/3.4/iscc_rb`
    first, so it needed no change
