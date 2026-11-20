## 2026-11-20 — Make the Ruby source gem buildable and install-check it in CI

**Done:** `scripts/build_ruby_source_gem.py` stages a standalone gem tree in a temp dir and runs
`gem build` there. The tree holds the gemspec files, a generated `Cargo.toml` (concrete keys, magnus
from `cargo metadata`, core as `path = "core"`, empty `[workspace]`), the core from
`cargo package -p iscc-lib` under `core/`, and the workspace `Cargo.lock`. The gemspec now depends
on `rb_sys ~> 0.9` and globs `core/**/*` + `Cargo.lock`. A new ci.yml job `ruby-source-gem` builds
the gem and installs it on Ruby 3.4 into an empty gem home. `release.yml` `publish-rubygems` builds
and pushes the same gem.

**Files changed:**

- `scripts/build_ruby_source_gem.py` (new): PEP 723, stdlib only. Pure render/version/extract
    helpers. Fails closed on a version.rb/workspace mismatch, an unhandled iscc-rb dependency, or an
    unsafe crate path
- `tests/test_build_ruby_source_gem.py` (new, 25 tests): manifest rendering, version checks, crate
    extraction, gemspec ↔ `CORE_DIR`, Gemfile.lock 4-space `rb_sys` pin, both workflows call the
    script, no workflow runs `gem build` directly (+ a detector fire case), CI Ruby series outside
    the cross-gem list, docs name the prerequisites
- `crates/iscc-rb/iscc-lib.gemspec`: `rb_sys ~> 0.9` runtime dependency; `Cargo.lock` + `core/**/*`
    file globs (they match nothing in the repo dir, so the repo-dir file list is unchanged: checked
    with `Gem::Specification.load(...).files`)
- `crates/iscc-rb/Gemfile.lock`: `bundle install` added only `      rb_sys (~> 0.9)` under PATH
- `.github/workflows/ci.yml`: job `ruby-source-gem` (Rust stable, libclang-dev, Ruby 3.4, setup-uv
    v9.0.0; install from `$RUNNER_TEMP` with `GEM_HOME=GEM_PATH=$RUNNER_TEMP/gem-home`, then
    `conformance_selftest`)
- `.github/workflows/release.yml`: `publish-rubygems` sets up Rust + uv, runs the script with
    `--out source-gem`, and the push loop pushes `source-gem/*.gem`
- `.claude/context/specs/ci-cd.md`: one job-table row for `ruby-source-gem`. **Forced edit outside
    Scope**: `check_ci_job_table.py` reds without it. mdformat then re-aligned the whole table (the
    diff is 45 lines, but only the one row's content changed)
- `docs/howto/ruby.md`, `crates/iscc-rb/README.md`: the source build needs a Rust toolchain,
    `libclang-dev`, Ruby headers, and network access (rb_sys gem + crates.io). CI tests Ruby 3.4.
    Ruby 4.0 is not claimed
- `crates/iscc-rb/CLAUDE.md` (Publishing): source-gem recipe, the loose `rb_sys` range vs the
    Gemfile pin, and the `vendor/`/`tmp/` warning

**Verification:**

- `uv run --script scripts/build_ruby_source_gem.py --out /tmp/rbgem` → exit 0, writes exactly
    `iscc-lib-0.7.0.gem`. After `gem unpack`, the `workspace = true` / `path = "../` grep exits 1
    (no output)
- From `/tmp`, `LANG=C.UTF-8`, empty `GEM_HOME=GEM_PATH`: `gem install … --no-document` exit 0. It
    pulled rb_sys 0.9.130 + rake-compiler-dock 1.12.0. `conformance_selftest` exit 0, and
    `$LOADED_FEATURES` shows `iscc_rb.so` loaded from the clean gem home. Re-run on the final
    (post-format) script output: same result. **Ruby 3.1.2 locally. The Ruby 3.4 CI job has not run
    yet.**
- `crates/iscc-rb`: `bundle install && bundle exec rake compile test` → 165 runs, 0 failures;
    `grep -E '^    rb_sys \(' Gemfile.lock | head -n 1` → `rb_sys (0.9.123)`
- `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py` → 29 passed; also
    passing: the ci-job-table, release-workflow and spec-paths tests (83 together)
- Fire check against HEAD's workflows: `bare_gem_builds(HEAD release.yml)` →
    `['gem build iscc-lib.gemspec']`; `script_jobs` is empty for HEAD ci.yml and release.yml
- `mise run check` exit 0; pre-push hooks on the two new files exit 0;
    `scripts/check_release_workflow.py` OK; actionlint 1.7.7 on both workflows exit 0;
    `uv run zensical build` exit 0

**Next:** Watch the first `ruby-source-gem` CI run on Ruby 3.4. Then the drift issue's gate holes
(the runner-up in next.md).

**Notes:**

- **Lockfile:** I ship the workspace `Cargo.lock` in the gem, which next.md did not specify, so
    installs resolve the crate versions CI tested. This matters for Unicode-table crates. Cargo
    prunes the extra workspace entries on its own. The rb_sys gem version itself still floats within
    `~> 0.9`
- Platform gems now also declare the `rb_sys` runtime dependency (rake-compiler keeps runtime deps).
    Untested here: `test-gem` and cross-gem only run in `release.yml`
- The 0.6.0/0.7.0 source gems on rubygems.org stay broken (a registry action for Titusz). This fix
    ships with the next release
- No Rust source changed, so the CRAP, iai and semver gates are unaffected
