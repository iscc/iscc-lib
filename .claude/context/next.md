# Next Work Package

## Step: Make the Ruby source gem buildable and install-check it in CI

## Goal

Resolve the `normal` issue "The Ruby source gem cannot be built" through its first branch: a
standalone source gem that carries the core, built by one staging command that both workflows use.
CI installs it into a clean gem home outside the workspace on a Ruby with no precompiled gem.
`ruby-bindings.md` requires a source gem that compiles where no precompiled gem exists, so this
restores a spec criterion and needs no human decision.

## Alternatives Considered

- **Chosen:** buildable, CI-checked source gem. It is user-facing (Ruby 3.4/4.0 installs fail
    today), the Ruby docs on `develop` state a false fallback, and the spec mandates the fallback.
- **Rejected:** stop publishing the source gem and reword the docs. Cheaper, but it removes
    `ruby-bindings.md` lines 164 and 323 (a source gem compiles where no precompiled gem exists).
    Dropping an acceptance criterion is Titusz's call. The realignment decision of 2026-11-20 covers
    renaming a mechanism, not dropping a capability.
- **Runner-up (next step):** the drift issue's gate holes (fail-closed `lib.rs` parser, register the
    4 sub-threshold docs). Autonomous, but it fixes no user-facing defect.

## Scope

- **Create**: `scripts/build_ruby_source_gem.py` (PEP 723, `uv run --script`),
    `tests/test_build_ruby_source_gem.py`
- **Modify**: `crates/iscc-rb/iscc-lib.gemspec`, `.github/workflows/ci.yml`,
    `.github/workflows/release.yml`, `crates/iscc-rb/Gemfile.lock` (regenerated),
    `docs/howto/ruby.md`, `crates/iscc-rb/README.md`, `crates/iscc-rb/CLAUDE.md` (Publishing)
- **Reference**: `.claude/context/specs/ruby-bindings.md` (lines ~150-165, 310-330),
    `crates/iscc-rb/{Cargo.toml,extconf.rb,Rakefile,Gemfile}`, `crates/iscc-lib/Cargo.toml`, root
    `Cargo.toml`, issues.md entry, `release.yml` jobs `build-gem` / `publish-rubygems`

## Not In Scope

- Widening the cross-gem `ruby-versions` list or bumping the `rb_sys` 0.9.123 pin or image tag
- Yanking or re-publishing the broken 0.6.0/0.7.0 source gems: that is a registry action for Titusz
- Editing `ruby-bindings.md` or any Rust source
- Changing the `test-gem` smoke job or the precompiled-gem build
- The derived-fact drift issue

## Implementation Notes

Measured or read while scoping:

- HEAD's binding uses `encode_seq`, which crates.io `iscc-lib` 0.7.0 lacks. So the gem must **carry
    the core source**. A crates.io dependency cannot pass a CI check on `develop`.
- `conformance.rs` has a non-test `include_str!("../tests/data.json")`, so the carried core needs
    `tests/data.json`. `cargo package -p iscc-lib --list --allow-dirty` works and lists it.
- Both `crates/iscc-rb/Cargo.toml` and `crates/iscc-lib/Cargo.toml` inherit workspace keys, so the
    gem needs a standalone manifest for each.
- `cross-gem` reads the rb_sys version with `grep -E '^    rb_sys \(' Gemfile.lock | head -n 1`
    (exactly 4 spaces). A gemspec dependency adds `      rb_sys (~> …)` under PATH with 6 spaces,
    which that grep does not match. Re-check after regenerating the lock.
- Latest rb_sys (0.9.130) requires Ruby `>= 2.7`. The gemspec constraint must be a loose
    `~> 0.9`-style range, never `= 0.9.123`. The Gemfile pin must still resolve.

Hypotheses for advance to confirm or replace:

- The staging script copies the gem files into a fresh directory outside the repo (or under
    `target/`). It writes the iscc-rb manifest standalone: concrete version, edition and
    rust-version, the magnus requirement read from `cargo metadata`, the core as
    `path = "<in-gem dir>"`, and an empty `[workspace]` table so cargo never walks up into an
    enclosing workspace. It drops in the core from the extracted
    `cargo package -p iscc-lib   --no-verify` crate, whose manifest is already normalized. Then it
    runs `gem build` there and writes the `.gem` to `--out DIR`. Keep the rendering in small pure
    functions, unit-tested without Rust.
- Do **not** name the carried-core directory `vendor/` or `tmp/`. Both are gitignored Bundler and
    rake dirs in `crates/iscc-rb`, and a gemspec glob over them would sweep junk into the platform
    gems that `cross-gem` builds from the repo dir.
- Platform gems built by rake-compiler will likely inherit the `rb_sys` runtime dependency. Other
    Rust gems (wasmtime) accept that. Leave it unless it breaks the `test-gem` install.
- The workspace `[profile.release]` does not carry over. rb_sys picks its own profile, which is
    fine.
- The ci.yml job runs the script and then installs with network access (rb_sys and crates.io).
    Running `gem install --local` in a clean `GEM_HOME` fails once rb_sys is a dependency. Use a
    Ruby series outside the cross-gem list (`3.4`) so the job proves the fallback the docs promise.
    It needs Rust + `libclang-dev`, and runs from a cwd outside the checkout.
- In `release.yml`, `publish-rubygems` replaces the bare `gem build` with the script. It needs a
    Rust toolchain and uv on that runner. The push loop must push the staged gem.
    `scripts/check_release_workflow.py` must stay green.
- Docs: both Ruby docs list exactly what the CI install needed (Rust toolchain, `libclang-dev`, Ruby
    headers, network access for rb_sys and crates.io). Do not claim Ruby 4.0 unless CI tests it.

No core or binding source changes, so the CRAP, iai and semver gates are unaffected. The container
has Ruby 3.1.2, which is enough for the local install check. Export `LANG=C.UTF-8` for Ruby runs.

## Verification

- `uv run --script scripts/build_ruby_source_gem.py --out /tmp/rbgem` exits 0 and writes exactly one
    `iscc-lib-<workspace version>.gem`. After `gem unpack` of it,
    `grep -rnE 'workspace *= *true|path *= *"\.\./' --include=Cargo.toml <unpacked>` prints nothing
- From `/tmp` with `LANG=C.UTF-8` and a fresh empty `GEM_HOME`/`GEM_PATH`,
    `gem install /tmp/rbgem/iscc-lib-*.gem --no-document` exits 0. Then
    `ruby -e 'require "iscc_lib"; exit(IsccLib.conformance_selftest ? 0 : 1)'` exits 0 with the same
    env
- In `crates/iscc-rb`, `bundle install && bundle exec rake compile test` passes, and
    `grep -E '^    rb_sys \(' Gemfile.lock | head -n 1` names `0.9.123`
- `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py` passes. The tests
    cover manifest rendering, assert that `ci.yml` and `release.yml` both invoke the script, and
    assert that no workflow runs a bare `gem build iscc-lib.gemspec`
- `mise run check` exits 0 and `uv run zensical build` exits 0
- Both Ruby docs name every prerequisite the ci.yml source-gem job installs, and no longer
    contradict a working source build (manual check)

## Done When

The staged source gem builds, installs into a clean gem home outside the workspace and passes
`conformance_selftest`. Both workflows build it the same way, and all criteria above pass.
