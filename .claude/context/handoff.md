## 2026-11-21 — Review of: Raise the Ruby floor to 3.3 and precompile gems for Ruby 3.3 and 3.4

**Verdict:** PASS_WITH_NOTES

**Summary:** The gemspec, the CI `ruby` job and the `release.yml` cross-gem list all say Ruby 3.3 as
the floor, precompiled gems cover 3.3 and 3.4, minitest moved to 6.1.0 with identical counts, and a
parity test ties the three floors together. The work is correct and the `[human]` issue is closed.
Two notes: the advance commit failed the pre-push `ty` check on one test line (fixed in this
review), and a rebuilt devcontainer can no longer run the Ruby gates (filed `normal`).

**Verification:**

- [x] `uv run pytest tests/test_ruby_gem_docs.py` passes with the floor-parity test and its negative
    cases — 47 passed together with `tests/test_build_ruby_source_gem.py`
- [x] The three greps print one line each: `release.yml:850`, gemspec `:16`, `ci.yml:253` (the
    `ruby` job)
- [x] Under `mise exec ruby@3.4.13 --`: `bundle exec standardrb` exits 0;
    `bundle exec rake compile test` → `168 runs, 481 assertions, 0 failures, 0 errors, 0 skips`
- [x] The `3\.1 to 3\.3|>= 3\.1|3\.1, 3\.2` sweep prints nothing (exit 1). A wider sweep of my own
    for `Ruby 3.1` / `3.2` found only the two intended mentions of the system Ruby
- [x] `uv run scripts/check_release_workflow.py` exits 0; `Gemfile.lock:28` is `rb_sys (0.9.123)`
- [x] `mise run check` exits 0, tree clean afterwards
- [x] Handoff records the minitest counts (5.27.0 and 6.1.0: 168 runs, 481 assertions) and the
    devcontainer gap

Extra checks (3 of 3):

- **The floor itself, Ruby 3.3.14** (`mise install ruby@3.3`, a 30 s prebuilt download):
    `bundle install`, `standardrb` exit 0, `rake compile test` → 168 runs, 481 assertions, 0
    failures. The handoff listed this as not verified; it is now
- **The image claim**, re-derived from source: the rb-sys 0.9.123 image is `FROM`
    rake-compiler-dock-image 1.10.0, whose `RUBY_CC_VERSION` is `3.4.5:3.3.9:3.2.9:3.1.7:…` (the
    x64-mingw-ucrt plan too). rb-sys-dock resolves `--ruby-versions` through the locked
    rake-compiler-dock gem 1.10.0, whose `cross_rubies` maps 3.4 → 3.4.5 and 3.3 → 3.3.9. They match
- **Floor parity on the real files with one value changed per run** (gemspec 3.2, CI 3.4, release
    list `3.2, 3.3, 3.4`, release list `3.4`): each fails with `Ruby floor mismatch`

**Issues found:**

- **Fixed here (step 9):** `tests/test_build_ruby_source_gem.py:477` called `.group(1)` on an
    optional match, and `uv run ty check` (a pre-push gate) failed. Now an explicit
    `assert out_match is not None`. `ty`, ruff and both test files are green
- **Filed `normal`:** "A rebuilt devcontainer cannot run the Ruby gates". `post-create.sh` still
    runs `bundle install` under system Ruby 3.1, which Bundler refuses, and nothing installs the
    bundle for the mise Ruby the standardrb hooks now use. Today's container works
- **Filed `low`, untriaged:** "`release.yml` loads only one precompiled Ruby binary before
    publishing" (`test-gem` runs 3.3 on x86_64 Linux only; the 3.4 binary is new and never loaded)
- **Filed `low`, untriaged:** "Ruby 4.0 gets no precompiled gem" (the remainder of the closed issue;
    needs rb_sys past the 0.9.123 hold)
- The two unscoped changes are accepted. The hooks had to leave system Ruby or `mise run check`
    could not pass. The replaced test assertion (CI Ruby outside the cross-gem list) became false by
    design; the new one pins that the job installs the built `.gem` file, which is what makes it a
    source install on any Ruby. No gate was weakened. 5 code files changed, within the cap
- Nit, not filed: the hooks say `ruby@3.4`, the docs `ruby@3.4.13` (covered by the `normal` issue)

**Codex review:** one finding (P2), the same devcontainer bootstrap gap: the hooks select mise Ruby
3.4 but the bootstrap installs no bundle for it, so `mise run check` fails on a fresh container with
`bundler: command not found: standardrb`. Filed as the `normal` issue above.

**Next:** "A rebuilt devcontainer cannot run the Ruby gates" (`normal`). It is small and it protects
the loop's own gate: after a container rebuild every `mise run check` would fail. Suggested shape:
pin `ruby` in `mise.toml`, let `post-create.sh` run `bundle install` under the mise Ruby, and make
the hooks and the two docs use that one pin. Do not change the gemspec, CI or release.yml. After
that, the decided cleanup "C++ `safe_data` and .NET `PinnedRows` padding".

**Notes:**

- Resolved and deleted: "The Ruby floor is past end of life and current Rubies get no precompiled
    gem". The spec (`specs/ruby-bindings.md`) was updated by the advance step
- **Never run `rake clobber` in `crates/iscc-rb`.** rb_sys lists the workspace cargo `target/` in
    it. I ran it during this review and it emptied the build cache (gitignored, rebuilt by the next
    cargo run). The extension in `lib/` is rebuilt for Ruby 3.4.13, as the advance step left it
- The first CI run of the `ruby` job on 3.3 follows this push. The local 3.3.14 run predicts green
- `release.yml` was not run (it is `workflow_dispatch`-only). The 3.4 cross-build rests on the
    source reading above
