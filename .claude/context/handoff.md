## 2026-03-03 — Add Ruby conformance tests against data.json

**Done:** Created `crates/iscc-rb/test/test_conformance.rb` with 50 dynamically generated test
methods (one per data.json vector) covering all 9 gen\_\*\_v0 functions. Uses `define_method` for
individual test names, mirrors the Python `tests/test_conformance.py` structure with Minitest
idioms.

**Files changed:**

- `crates/iscc-rb/test/test_conformance.rb`: New file — conformance tests for gen_meta_code_v0 (20),
    gen_text_code_v0 (5), gen_image_code_v0 (3), gen_audio_code_v0 (5), gen_video_code_v0 (3),
    gen_mixed_code_v0 (2), gen_data_code_v0 (4), gen_instance_code_v0 (3), gen_iscc_code_v0 (5)

**Verification:** `bundle exec rake test` — 111 runs, 295 assertions, 0 failures, 0 errors, 0 skips.
All 50 conformance vectors pass alongside 61 existing smoke/streaming tests. `mise run check` — all
14 pre-commit hooks pass. `grep -c "define_method"` returns 9 (one per function section).

**Next:** Ruby CI job in `.github/workflows/ci.yml` (compile + test), `version_sync.py` gemspec
integration, `docs/howto/ruby.md` how-to guide, and RubyGems publishing setup in `release.yml`.

**Notes:** No surprises. The implementation was straightforward — the Ruby API surface maps cleanly
to the data.json vectors. Helper methods `prepare_meta_arg` and `decode_stream` are class methods on
`TestConformance` (called via `self.class.method_name` from instance methods). JSON `sort_keys`
option in `JSON.generate` handles JCS-compatible output for dict meta inputs.
