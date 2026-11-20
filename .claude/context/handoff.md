## 2026-11-20 — Review of: Make the Ruby source gem buildable and install-check it in CI

**Verdict:** NEEDS_WORK

**Summary:** The source gem itself is good: the staging script builds a standalone gem that installs
into a clean gem home and passes `conformance_selftest` on Ruby 3.1.2 and on Ruby 3.4.13. But the
new `rb_sys` gemspec dependency unpins the release's platform-gem build. The `cross-gem` action no
longer finds `0.9.123` in `Gemfile.lock` and falls back to the latest remote rb_sys against the
pinned 0.9.123 image. Nothing is pushed. Commit `d299106` stays, and the repair goes on top of it.

**Verification:**

- [x] `uv run --script scripts/build_ruby_source_gem.py --out /tmp/rbgem` exits 0 and writes exactly
    `iscc-lib-0.7.0.gem`. After `gem unpack`, the `workspace = true` / `path = "../` grep prints
    nothing
- [x] From `/tmp`, `LANG=C.UTF-8`, empty `GEM_HOME`/`GEM_PATH`: `gem install … --no-document` exits
    0, and the selftest exits 0 with `iscc_rb.so` loaded from the clean gem home
- [x] `bundle install && bundle exec rake compile test` passes (165 runs, 0 failures), and
    `grep -E '^    rb_sys \(' Gemfile.lock | head -n 1` names `0.9.123`. **Passes as written, but
    the criterion does not measure what it was meant to.** See the blocker below
- [x] `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py`: 29 passed
- [x] `mise run check` exits 0 and `uv run zensical build` exits 0 ("No issues found")
- [x] Both Ruby docs name what the CI job installs (Rust toolchain, `libclang-dev`, Ruby headers,
    network access) and no longer contradict a working source build

Probes beyond next.md:

- Ruby 3.4.13 (precompiled, via mise): the built gem installs into an empty gem home from `/tmp` and
    passes the selftest. The docs' Ruby 3.4 claim holds locally, before the CI job has run
- The shipped `Cargo.lock` is honoured: all 127 packages in the installed gem's lock are entries of
    the workspace lock

**Issues found:**

- **Blocker: the `rb_sys` pin lookup of `cross-gem` is broken.** The action (`action.yml` at `v1`)
    runs `grep rb_sys Gemfile.lock | head -n 1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+'`. It is not
    anchored to four spaces, as next.md stated. At `d299106` the first match is the new PATH line
    `rb_sys (~> 0.9)`, the pipeline exits 1, and the action falls back to `gem info rb_sys --remote`
    (0.9.130 today). At `HEAD~1` the same pipeline prints `0.9.123`. `release.yml` and
    `crates/iscc-rb/CLAUDE.md` ("rb_sys Version Pinning") both describe this mismatch as breaking
    the cross build. `release.yml` only runs on dispatch, so no CI run would show it before a
    release
- **`test_gemfile_lock_pins_cross_gem_rb_sys` models the wrong lookup.** It uses next.md's anchored
    regex, so it is green while the real pipeline fails
- **The step changed the precompiled-gem build, which next.md excluded.** Besides the lookup, every
    platform gem would declare `rb_sys` as a runtime dependency (it pulls `rake-compiler-dock`). The
    published 0.7.0 platform gems have no runtime dependency. `test-gem` would see this only in a
    release run
- **`run()` hides the reason for a failure.** `subprocess.run(check=True, capture_output=True)`
    raises `CalledProcessError`, whose message holds only the command and exit status. A failing
    `cargo package` or `gem build` in the release job would log no stderr
- Nit: `gemspec_files()` splits the file list on any whitespace. No listed file has a space today
- Accepted: the one-row edit to `specs/ci-cd.md` outside Scope. `check_ci_job_table.py` forces it,
    and it is a checkable fact. The `# noqa: S603` has a reason comment and follows the other
    scripts

**Codex review:** One finding (P1), and it is the blocker above. Codex read the action's real
lookup. I confirmed it against `action.yml` at `v1` and by running the pipeline on `HEAD` and
`HEAD~1`.

**Next:** Repair this step only. Required:

1. The action's real pipeline, run on `crates/iscc-rb/Gemfile.lock`, prints the Gemfile pin:
    `grep rb_sys Gemfile.lock | head -n 1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+'` gives `0.9.123`
2. Replace `test_gemfile_lock_pins_cross_gem_rb_sys` with a test of that lookup: take the first line
    that contains `rb_sys`, extract `X.Y.Z`, and compare it with the Gemfile pin and with the
    `tag:` input of `cross-gem` in `release.yml`. It must fail on the `Gemfile.lock` of `d299106`
3. `run()` lets stderr through (capture stdout only)
4. `crates/iscc-rb/CLAUDE.md` → Publishing describes the dependency as it then is
5. Re-run every criterion of this step, including the clean install. A clean gem home proves the
    source gem still declares `rb_sys`, because `extconf.rb` needs `rb_sys/mkmf`

Two ways to meet 1. Neither is tested yet:

- **Recommended:** declare `rb_sys` only for the staged source gem, for example
    `spec.add_dependency "rb_sys", "~> 0.9" if File.directory?("core")`. The gemspec is already
    directory-dependent through its `core/**/*` glob. `bundle install` then returns `Gemfile.lock`
    to its `HEAD~1` form, and the platform gems keep having no runtime dependency
- Keep the dependency unconditional and change the "Link Gemfile.lock" step in `release.yml` so the
    file the action greps starts with the pinned line. The platform gems then keep the new runtime
    dependency, which nothing tests before a release

**Notes:**

- The `normal` issue "The Ruby source gem cannot be built" stays open until the repaired step is
    pushed and the `ruby-source-gem` CI job is green
- At the PASS review: delete that issue and file a `low` remainder. The 0.6.0 and 0.7.0 source gems
    on rubygems.org stay broken until the next release, and a yank is a registry action for Titusz
- Ruby 3.4 for a local check takes 24 seconds: from a directory outside the repo,
    `MISE_RUBY_COMPILE=false mise x ruby@3.4 -- gem install …`. It does not touch `mise.toml`
- Shipping the workspace `Cargo.lock` in the gem is sound and was not in next.md. It needs a
    decisions.md entry at the PASS review (carried core, lock, staging script)
- No Rust source changed, so the CRAP, iai and semver gates are unaffected
