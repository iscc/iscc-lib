## 2026-03-03 — Review of: Add DataHasher and InstanceHasher streaming types to Ruby bridge

**Verdict:** PASS

**Summary:** Added `DataHasher` and `InstanceHasher` streaming classes to the Ruby Magnus bridge,
completing all 32/32 Tier 1 symbols. Implementation is clean, follows established patterns, adapts
the PyO3 `Option<inner>` pattern correctly for Magnus using `RefCell<Option<inner>>`. All 61 tests
pass (152 assertions, 0 failures).

**Verification:**

- [x] `cargo check -p iscc-rb` compiles successfully
- [x] `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- [x] `bundle exec rake compile` — builds native extension in release profile
- [x] `bundle exec rake test` — 61 runs, 152 assertions, 0 failures, 0 errors, 0 skips
- [x] `IsccLib::DataHasher.new` returns a DataHasher instance (proven by tests)
- [x] `IsccLib::InstanceHasher.new` returns an InstanceHasher instance (proven by tests)
- [x] Streaming result matches one-shot: DataHasher and InstanceHasher both match `gen_*_v0`
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Codex review:** Clean — no correctness issues found. Codex confirmed consistent wiring end-to-end
(Rust types, class registration, Ruby reopen/wrapper, tests).

**Next:** Ruby binding is now feature-complete (32/32 Tier 1 symbols). The next logical step is
adding conformance tests against `data.json` vectors to verify correctness across all gen functions.
After that: Ruby CI job in `ci.yml`, `version_sync.py` for gemspec, `docs/howto/ruby.md`, and
RubyGems publishing setup.

**Notes:**

- Design choice B (direct class name, prefixed methods) was the right call. `_DataHasher` is not a
    valid Ruby constant name. The `#[magnus::wrap(class = "IsccLib::DataHasher")]` + Ruby
    `class   DataHasher` reopen pattern is clean and avoids inner delegation.
- The `RefCell<Option<inner>>` adaptation for Magnus (vs PyO3's direct `Option<inner>`) is the
    correct approach: Magnus instance methods receive `&self`, so interior mutability is needed.
- 32/32 symbols means the Ruby `issues.md` entry scope items 1 (crate setup) and the symbol surface
    portion are done. Remaining from that issue: conformance tests, CI job, release workflow,
    version_sync, documentation, account setup.
