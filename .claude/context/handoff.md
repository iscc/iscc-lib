## 2026-03-03 — Add DataHasher and InstanceHasher streaming types to Ruby bridge

**Done:** Added `DataHasher` and `InstanceHasher` streaming classes to the Ruby Magnus bridge,
completing all 32/32 Tier 1 symbols. Both follow the `new → update → finalize` pattern with one-shot
finalize semantics via `RefCell<Option<inner>>`. Ruby wrapper reopens the native classes to add
`bits: 64` default, result wrapping (`DataCodeResult`/`InstanceCodeResult`), and method chaining on
`update`.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: Added `RbDataHasher` and `RbInstanceHasher` structs with
    `#[magnus::wrap]`, `rb_new`, `update`, and `finalize` methods. Registered as classes under
    `IsccLib` module with `_update`/`_finalize` method names (Ruby wrapper provides public API).
    Updated docstring symbol count from 30→32.
- `crates/iscc-rb/lib/iscc_lib.rb`: Reopened native `DataHasher` and `InstanceHasher` classes to add
    `update` (returns `self` for chaining), `finalize(bits: 64)` (default bits, wraps return in
    result class).
- `crates/iscc-rb/test/test_iscc_lib.rb`: Added 15 new tests covering basic usage, streaming
    equivalence with one-shot, multi-update, double-finalize error, update-after-finalize error,
    method chaining, attribute access, datahash/filesize fields.

**Verification:** 61 tests, 152 assertions, 0 failures, 0 errors, 0 skips. `cargo check -p iscc-rb`
clean, `cargo clippy -p iscc-rb -- -D warnings` clean (0 warnings), `bundle exec rake compile`
builds, `bundle exec rake test` all pass, `mise run check` all 14 pre-commit hooks pass,
`cargo test -p iscc-lib` 314 tests all pass.

**Next:** Ruby binding is now feature-complete (32/32 Tier 1 symbols). Next steps could be:
conformance tests against `data.json`, Ruby CI job in `ci.yml`, Ruby how-to guide
(`docs/howto/ruby.md`), `version_sync.py` update for gemspec, or RubyGems publishing setup.

**Notes:**

- Chose option B variant from next.md: Rust classes registered directly as `DataHasher`/
    `InstanceHasher` (no `_` prefix on class names — Ruby constants must start with uppercase).
    Methods registered as `_update`/`_finalize` (valid method names with `_` prefix). Ruby reopens
    the native class to wrap methods with defaults and result types.
- The `_` prefix pattern that works for gen functions (module methods) doesn't work for class names
    because Ruby constants must start with an uppercase letter. `_DataHasher` would be treated as a
    method/variable name, causing `NameError`.
- `#[magnus::wrap(class = "IsccLib::DataHasher")]` creates the Ruby class during native extension
    loading. Ruby's `class DataHasher` inside `module IsccLib` reopens (not shadows) the existing
    class, allowing method additions/overrides. This is a clean pattern for adding Ruby-level
    convenience to native classes.
