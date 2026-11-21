# Next Work Package

## Step: Raise the Ruby floor to 3.3 and precompile gems for Ruby 3.3 and 3.4

## Goal

Closes the `normal` `[human]` issue "The Ruby floor is past end of life and current Rubies get no
precompiled gem" (decided by Titusz 2026-11-21): the gem declares `>= 3.3`, CI tests that floor, and
`release.yml` precompiles for every series the pinned cross-gem image can build (3.3, 3.4).

## Alternatives Considered

- **Chosen:** Ruby floor — a human decision waiting to be carried out; Ruby 3.4 users get a
    precompiled gem, which they lack today. The handoff recommends it first.
- **Rejected:** remove C++ `safe_data` / .NET `PinnedRows` padding — also decided and small, but it
    is internal cleanup with no user-visible effect. Next in line.

## Scope

- **Modify**: `crates/iscc-rb/iscc-lib.gemspec` (`required_ruby_version = ">= 3.3.0"`)
- **Modify**: `.github/workflows/ci.yml` (`ruby` job `ruby-version: '3.3'`; reword the
    `ruby-source-gem` comment, which says 3.4 has no precompiled gem)
- **Modify**: `.github/workflows/release.yml` (`build-gem` cross-gem `ruby-versions: 3.3, 3.4`;
    update the comment next to `tag: 0.9.123`)
- **Modify**: `crates/iscc-rb/Gemfile` (lift the minitest hold, see notes) + `Gemfile.lock`
    (generated)
- **Modify (docs)**: `crates/iscc-rb/README.md`, `docs/howto/ruby.md` (floor and the "Ruby 3.3 to
    3.4" range), `crates/iscc-rb/CLAUDE.md` (lines 35-36 version list, line 137 dev Ruby note),
    `.claude/context/specs/ruby-bindings.md` (gemspec snippet line 181, lines 293-294)
- **Modify (tests)**: `tests/test_ruby_gem_docs.py` (add the floor-parity test below)
- **Reference**: `crates/iscc-rb/lib/iscc_lib.rb` (version-aware loader; must find `3.4/`)

## Not In Scope

- Ruby 4.0 precompiled gems. They need rb_sys >= 0.9.124 / a newer image, which the Gemfile hold
    says breaks `RbSys::ExtensionTask`. That is a separate step that has to be verified on its own.
    Keep `rb_sys "0.9.123"` and `tag: 0.9.123` exactly as they are. Moving the `ruby-source-gem` job
    to Ruby 4.0 is out for the same reason: it stays on 3.4
- The devcontainer's Debian system Ruby 3.1 (`.devcontainer/Dockerfile`). Do not change it. Name the
    gap in the handoff
- The `test-gem` / `publish-rubygems` jobs. They already run 3.3
- Yanking published source gems (`low` issue, human only)

## Implementation Notes

- **The pinned image builds 3.4: verified.** rb-sys-dock 0.9.123 is
    `FROM rake-compiler-dock-image:1.10.0-mri-*`. That image sets
    `RUBY_CC_VERSION=3.4.5:3.3.9:3.2.9:3.1.7:…`, and its x64-mingw-ucrt plan also builds 3.4.5.
    rake-compiler-dock 1.10.0's `cross_rubies` maps `"3.4" => "3.4.5"`. Sources:
    https://raw.githubusercontent.com/oxidize-rb/rb-sys/v0.9.123/docker/Dockerfile.x86_64-linux and
    https://raw.githubusercontent.com/rake-compiler/rake-compiler-dock/v1.10.0/Dockerfile.mri.erb.
    The current release.yml comment ("rbconfigs for 3.1.7/3.2.9/3.3.9") is incomplete. Correct it
- `release.yml` never runs in CI. After editing, run `uv run scripts/check_release_workflow.py`
    (also a prek hook).
- **Floor parity gate:** add a test to `tests/test_ruby_gem_docs.py` that reads three values and
    asserts they are equal: the gemspec `required_ruby_version` floor (major.minor), the
    `ruby-version` of the `ruby` job in `ci.yml`, and the lowest cross-gem series in `release.yml`.
    Parse with `yaml` and a regex, the same way the file already does. Add a negative unit test that
    a mismatched trio fails (pure function, one bad case per call).
- **Docs gate:** after the change, the existing parity test expects both Ruby docs to contain
    `Ruby 3.3 to 3.4`. Also state the floor (`>= 3.3`) where the docs give the requirement
- **minitest:** the Gemfile hold exists only because of the 3.1 floor. Lift it to `~> 6.0` and
    regenerate the lock with `bundle update minitest --conservative`, run under Ruby 3.4
    (`mise exec ruby@3.4.13 -- …`; system Ruby 3.1 is below the floor and Bundler will refuse it).
    The lock must still pin `rb_sys (0.9.123)`. Record the Ruby suite's runs/assertions count under
    minitest 5 (before the bump) and 6 in the handoff. They must match. If minitest 6 needs more
    than trivial test edits, keep `~> 5.0`, rewrite the hold comment with the real reason, and say
    why in the handoff
- standardrb on Ruby 3.3+ may enable new cops. Fix any finding in the source. Never add a disable
- `crates/iscc-rb/CLAUDE.md` line 137: replace the `ruby/3.1.0` PATH tip with how to run the suite
    on a supported Ruby here (`mise exec ruby@3.4.13 -- bundle exec rake compile test`)

## Verification

- `uv run pytest tests/test_ruby_gem_docs.py` passes, including the new floor-parity test and its
    negative case
- `grep -n 'ruby-versions: 3.3, 3.4' .github/workflows/release.yml` and
    `grep -n 'required_ruby_version = ">= 3.3.0"' crates/iscc-rb/iscc-lib.gemspec` each print one
    line. `grep -n "ruby-version: '3.3'" .github/workflows/ci.yml` prints the `ruby` job's line
- In `crates/iscc-rb`, under `mise exec ruby@3.4.13 --`: `bundle exec standardrb` exits 0 and
    `bundle exec rake compile test` reports 0 failures and 0 errors
- `git grep -n -E '3\.1 to 3\.3|>= 3\.1|3\.1, 3\.2' -- crates/iscc-rb docs .github .claude/context/specs`
    prints nothing
- `uv run scripts/check_release_workflow.py` exits 0.
    `grep -n 'rb_sys (0.9.123)'   crates/iscc-rb/Gemfile.lock` prints a line
- `mise run check` exits 0

## Done When

All six verification criteria pass, and the handoff records the minitest run counts and the
devcontainer system-Ruby gap.
