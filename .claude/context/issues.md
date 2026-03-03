# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Implement Ruby bindings via Magnus `normal` [human]

Add Ruby language bindings as a new `iscc-rb` crate using Magnus (Rust ↔ Ruby bridge). This follows
the same hub-and-spoke pattern as the existing PyO3 Python bindings — Magnus compiles Rust directly
into a native Ruby C extension without an intermediate C layer.

**Spec:** `.claude/context/specs/ruby-bindings.md`

**Implementation scope:**

1. **Crate setup** (`crates/iscc-rb/`):

    - `Cargo.toml` (cdylib, depends on `iscc-lib` + `magnus`)
    - `src/lib.rs` — Magnus bridge (~400-500 lines, all 32 Tier 1 symbols)
    - `ext/iscc_lib/extconf.rb` — rb_sys extension config
    - `lib/iscc_lib/version.rb` — VERSION constant (synced from Cargo.toml)
    - `lib/iscc_lib.rb` — Pure Ruby wrapper with typed result classes
    - `iscc-lib.gemspec`, `Gemfile`, `Rakefile`
    - `test/test_conformance.rb` — Minitest against `data.json`
    - `README.md` — per-crate README for RubyGems

2. **DevContainer**: Add `ruby ruby-dev` to Dockerfile apt-get install

3. **CI** (`ci.yml`): Add `ruby` job — `bundle exec rake compile` + `bundle exec rake test`

4. **Release** (`release.yml`):

    - Add `rubygems` boolean input to `workflow_dispatch`
    - Build matrix: precompiled gems for 5 platforms (Linux x86_64/aarch64, macOS x86_64/arm64,
        Windows x64)
    - Publish via `gem push` with OIDC trusted publishing or `GEM_HOST_API_KEY`
    - Idempotency: check version on rubygems.org before publishing

5. **Version sync**: Add `crates/iscc-rb/lib/iscc_lib/version.rb` to sync targets in
    `scripts/version_sync.py`

6. **Documentation**: `docs/howto/ruby.md` how-to guide, `docs/ruby-api.md` API reference, update
    README with Ruby install/quickstart

7. **Account setup** (manual, human action):

    - Register/verify RubyGems.org account
    - Reserve `iscc-lib` gem name
    - Configure OIDC trusted publisher for `iscc/iscc-lib` repo

## Add programming language logos to README and docs `low` [human]

Add logos/icons for the supported programming languages (Rust, Python, etc.) to the README and
documentation pages where appropriate. Visual language indicators help users quickly identify
binding availability and make the project more approachable.
