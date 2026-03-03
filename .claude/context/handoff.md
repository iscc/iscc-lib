## 2026-03-03 — Review of: Add Ruby conformance tests against data.json

**Verdict:** PASS

**Summary:** Created `crates/iscc-rb/test/test_conformance.rb` with 50 dynamically generated test
methods covering all 9 gen\_\*\_v0 functions against official data.json vectors. Implementation
faithfully mirrors Python `tests/test_conformance.py` structure using Ruby/Minitest idioms. One
latent bug found (Ruby `JSON.generate` ignoring `sort_keys: true`) was fixed during review.

**Verification:**

- [x] `cd crates/iscc-rb && bundle exec rake test` passes — 111 runs, 295 assertions, 0 failures
- [x] Test output shows `test_gen_meta_code_v0_test_0001_title_only` style names (dynamically
    generated)
- [x] Total test count = 111 (61 existing + 50 conformance vectors) ≥ 100
- [x] `grep -c "define_method"` returns 9 (one per function section)
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention in diff
- [x] Scope discipline — only `test_conformance.rb` added (plus handoff.md), no out-of-scope changes
- [x] `cargo clippy -p iscc-rb -- -D warnings` — clean

**Issues found:**

- Fixed: `JSON.generate(meta_val, sort_keys: true)` is a no-op in Ruby's json gem — `sort_keys`
    option is silently ignored. Replaced with `JSON.generate(meta_val.sort.to_h)` for correct key
    sorting. Current vectors pass either way (keys happen to be pre-sorted), but this prevents
    future breakage.

**Codex review:** One actionable finding — the `sort_keys: true` no-op in `prepare_meta_arg`. Fixed
during review (step 9). No other issues.

**Next:** Ruby binding is now conformance-verified (50/50 vectors). The next logical step is adding
the Ruby CI job to `.github/workflows/ci.yml` (compile + test), or `version_sync.py` gemspec
integration. The CI job is higher priority since it enables automated verification on every push.

**Notes:**

- The Ruby issues.md entry item 1 (crate setup) is fully done now: 32/32 symbols + conformance
    tests. Remaining from that issue: CI job, release workflow, version_sync, documentation, account
    setup.
- Ruby `JSON.generate` does NOT support `sort_keys` — this is a Ruby footgun. Added to learnings.
