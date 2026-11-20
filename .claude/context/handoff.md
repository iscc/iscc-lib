## 2026-11-20 — Review of: Repair the Ruby source gem step — declare rb_sys only in the staged source gem

**Verdict:** PASS_WITH_NOTES

**Summary:** The repair is correct and narrow. The gemspec declares `rb_sys ~> 0.9` only when a
`core/` directory sits next to it, so the staged source gem has the dependency and the platform gems
have none. `Gemfile.lock` is byte-identical to its form before iteration 239, and the lookup that
`cross-gem@v1` really runs (checked against its `action.yml`) prints `0.9.123` again. The batch of
iterations 239 and 240 is pushed. One note from Codex about the 239 commit is filed as a `normal`
issue: the source gem is compiled without the workspace release profile.

**Verification:**

- [x] In `crates/iscc-rb`, the lookup pipeline prints `0.9.123`, and
    `ruby -e 'exit Gem::Specification.load("iscc-lib.gemspec").runtime_dependencies.empty?'` exits 0
    (system Ruby 3.1.2 and Ruby 3.4.13)
- [x] `uv run --script scripts/build_ruby_source_gem.py --out …` exits 0, and the built gem lists
    `rb_sys ~> 0.9` as a runtime dependency. From `/tmp` with `LANG=C.UTF-8` and an empty
    `GEM_HOME`/`GEM_PATH`, `gem install … --no-document` exits 0 (3 gems: `rb_sys` 0.9.130 came in
    through the dependency) and the selftest exits 0. Run on Ruby 3.4.13, the series of the CI job
- [x] `bundle install && bundle exec rake compile test`: 165 runs, 471 assertions, 0 failures
- [x] `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py`: 32 passed.
    With `      rb_sys (~> 0.9)` put back under `PATH` specs (in a `git archive` copy), only
    `test_cross_gem_lookup_reads_pinned_rb_sys` fails (1 failed, 22 passed)
- [x] `mise run check` exits 0

Probes beyond next.md:

- `BUNDLE_FROZEN=true bundle install` exits 0 and leaves `Gemfile.lock` untouched. This is the mode
    of `bundler-cache: true` in the `ruby` CI job and the cross-gem job
- Full pre-push gates on all files (clippy, cargo test, `ty`, ruff `S`/`C901`, pytest): green
- The new gemspec tests are the first in the pytest suite to call `ruby`. next.md's reason
    ("`gemspec_files()` already needs `ruby`") holds for the script, not for the suite. The
    `python-test` CI job has no `setup-ruby`; it relies on the Ruby 3.2.3 that the `ubuntu-24.04`
    runner image ships (checked in the image readme)

**Issues found:**

- **Filed `normal`: the source gem is built without the workspace release profile.** The staged
    `Cargo.toml` has no `[profile.release]`, so a source install is not stripped, has no LTO and
    unwinds on panic, while the platform gems get the workspace profile. Measured: 1,521,056 bytes
    unstripped against 1,061,576 stripped. The gem works and passes the selftest. It comes from the
    239 commit, not from the repair
- **Filed `low` (untriaged): the published source gems up to 0.7.0 stay broken** until a release
    ships the repaired one. A yank is a registry action for Titusz
- Closed: "The Ruby source gem cannot be built". Its check exists (`ruby-source-gem` CI job) and the
    same commands pass locally on Ruby 3.4
- Fixed in this review: `crates/iscc-rb/CLAUDE.md` named `configure-rubygems-credentials@main`,
    `release.yml` pins `@v2.1.0`. The line now points to the workflow instead of repeating the ref
- Note, not filed: a developer who runs `pytest` without `ruby` on the PATH gets two failing tests
    with the message "ruby is required to evaluate the gemspec". next.md asked for no skip markers

**Codex review:** Run over the whole unpushed batch (`--base c2811c5`). One finding (P2): the
standalone manifest drops `[profile.release]`. Confirmed with `file` and the byte sizes above, and
filed as the `normal` issue. Nothing about the repair itself.

**Next:** Two `normal` issues are open. The smaller one is "The Ruby source gem is built without the
workspace release profile": derive `[profile.release]` from the root `Cargo.toml` in
`render_manifest` (never retype the four keys), pin it with a test, and say in
`crates/iscc-rb/CLAUDE.md` → Publishing that a source install uses the same profile. Check the
install time on Ruby 3.4 before and after, because LTO with one codegen unit makes the user's
`gem install` slower; if that cost looks too high, the issue's second branch (a recorded decision)
is the way out. The other one is "Derived facts in guidance and docs are written by hand and drift".

**Notes:**

- The `ruby-source-gem` CI job runs for the first time on this push. update-state should read that
    job by name, not only the overall status. A red run there is an outside signal and reopens the
    source gem problem as a new `normal` issue with the job log
- decisions.md has a new entry on the source gem design (carried core, shipped `Cargo.lock`,
    conditional `rb_sys`). Do not re-open those three choices. The release profile is not part of
    that decision
- The staged manifest is its own cargo workspace: nothing from the root `Cargo.toml` (`[profile.*]`,
    `[patch]`, workspace lints) reaches a source install unless the script copies it
- No Rust source changed in this batch, so the CRAP, iai and semver gates are unaffected
