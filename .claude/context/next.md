# Next Work Package

## Step: Add DataHasher and InstanceHasher streaming types to Ruby bridge

## Goal

Expose the final 2 of 32 Tier 1 symbols — `DataHasher` and `InstanceHasher` streaming types — in the
Ruby Magnus bridge, completing the Ruby binding's full symbol surface. These are **class-based**
(not module functions), requiring a different Magnus pattern from all 30 prior symbols.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-rb/src/lib.rs` — add `RbDataHasher` and `RbInstanceHasher` struct definitions with
        `new`, `update`, `finalize` methods; register as classes under `IsccLib` in `init()`; update
        docstring symbol count from 30→32
    - `crates/iscc-rb/lib/iscc_lib.rb` — add Ruby wrapper classes for `DataHasher` and
        `InstanceHasher` that delegate to Rust and provide `bits: 64` default, wrapping finalize
        return in `DataCodeResult` / `InstanceCodeResult`
    - `crates/iscc-rb/test/test_iscc_lib.rb` — add streaming type tests (test file, excluded from
        3-file limit)
- **Reference**:
    - `crates/iscc-lib/src/streaming.rs` — Rust core `DataHasher` / `InstanceHasher` API
    - `crates/iscc-py/src/lib.rs` — Python `PyDataHasher` / `PyInstanceHasher` pattern
        (`Option<inner>`, one-shot finalize, dict return)
    - `crates/iscc-napi/src/lib.rs` — Node.js pattern (for comparison)

## Not In Scope

- Conformance tests against `data.json` (separate future step)
- Ruby CI job in `ci.yml` (separate future step)
- RubyGems publishing in `release.yml` (separate future step)
- `version_sync.py` update for gemspec (separate future step)
- `docs/howto/ruby.md` guide (separate future step)
- Full `iscc-rb/README.md` content (separate future step)
- Ruby linting with `standard` gem (separate future step)
- Any changes to other binding crates
- Renaming `update` to `push` or any other Ruby-specific method aliasing

## Implementation Notes

### Magnus class pattern (NEW — first time in this bridge)

All 30 prior symbols used `define_module_function` (stateless functions on the `IsccLib` module).
Streaming types need `define_class` + `define_method` (stateful class instances).

**Struct definition** — use `#[magnus::wrap(class = "IsccLib::DataHasher")]`:

```rust
use std::cell::RefCell;

#[magnus::wrap(class = "IsccLib::DataHasher")]
struct RbDataHasher {
    inner: RefCell<Option<iscc_lib::DataHasher>>,
}
```

**Interior mutability** — Magnus instance methods take `&self` (not `&mut self`), so use
`RefCell<Option<inner>>` for one-shot finalize semantics. `Option` allows `take()` on finalize so
the hasher can only be consumed once.

**Method registration in `init()`:**

```rust
let data_hasher = module.define_class("DataHasher", ruby.class_object())?;
data_hasher.define_singleton_method("new", function!(RbDataHasher::rb_new, 0))?;
data_hasher.define_method("update", method!(RbDataHasher::update, 1))?;
data_hasher.define_method("finalize", method!(RbDataHasher::finalize, 1))?;
```

Note: use `method!` (not `function!`) for instance methods. `function!` is for module/class-level
functions (no `self`). `method!` passes `&self` as first argument.

### Return types from `finalize`

- `DataHasher#finalize(bits)` → returns `RHash` with key `"iscc"` (String)
- `InstanceHasher#finalize(bits)` → returns `RHash` with keys `"iscc"` (String), `"datahash"`
    (String), `"filesize"` (Integer)

This matches the Python pattern (returns dict). The existing `DataCodeResult` and
`InstanceCodeResult` classes in `iscc_lib.rb` wrap these hashes.

### Ruby wrapper layer

Add Ruby wrapper classes in `iscc_lib.rb` that:

1. Delegate to the Rust class (registered with `_` prefix as `_DataHasher` / `_InstanceHasher`)
2. Provide `bits: 64` default for `finalize`
3. Wrap the `finalize` return hash in `DataCodeResult` / `InstanceCodeResult`
4. Return `self` from `update` to enable method chaining

This follows the same two-layer pattern as gen functions: Rust does the work (prefixed), Ruby wraps
with defaults and result classes.

```ruby
class DataHasher
  def initialize
    @inner = _DataHasher.new
  end

  def update(data)
    @inner.update(data)
    self
  end

  def finalize(bits: 64)
    DataCodeResult[@inner.finalize(bits)]
  end
end
```

Alternatively, if using `#[magnus::wrap]` the Rust struct IS the Ruby object and wrapping becomes
trickier. The advance agent should choose the simpler approach — either:

- (A) Register Rust class as `_DataHasher` (prefixed), Ruby `DataHasher` wraps it
- (B) Register Rust class as `DataHasher` directly, have `finalize` take the bits arg, handle
    defaults and result wrapping in Rust

Option (A) is consistent with the gen function pattern. Option (B) is simpler code. The advance
agent should pick whichever results in cleaner code.

### Error handling

- `update` after `finalize` → `RuntimeError: DataHasher already finalized` (match Python message)
- `finalize` after `finalize` → `RuntimeError: DataHasher already finalized`
- Invalid `bits` → propagate Rust core error via `to_magnus_err`

### Tests to add

1. Basic usage: `new → update → finalize` produces valid ISCC string starting with `"ISCC:"`
2. Streaming equivalence: streaming result matches `gen_data_code_v0` / `gen_instance_code_v0` for
    same input data
3. Multi-update: split data across multiple `update` calls, verify same result as single update
4. Double-finalize error: verify `RuntimeError` on second `finalize` call
5. Update-after-finalize error: verify `RuntimeError` on `update` after `finalize`
6. InstanceHasher `finalize` returns hash with `datahash` and `filesize` fields
7. Method chaining: `hasher.update(data).update(more_data).finalize` works

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- `bundle exec rake compile` — builds native extension in release profile (run from
    `crates/iscc-rb/`)
- `bundle exec rake test` — all tests pass (46 existing + ~7-10 new streaming tests, 0 failures)
- `IsccLib::DataHasher.new` returns a DataHasher instance (not error)
- `IsccLib::InstanceHasher.new` returns an InstanceHasher instance (not error)
- Streaming result matches one-shot: DataHasher result matches `gen_data_code_v0` for same data
- `mise run check` — all pre-commit/pre-push hooks pass

## Done When

All verification criteria pass and both `DataHasher` and `InstanceHasher` are usable from Ruby with
`new → update → finalize` interface, completing 32/32 Tier 1 symbols.
