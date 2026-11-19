# Next Work Package

## Step: Ruby array arguments via `RArray::into_iter` (Magnus `Vec<T>` segfault) + `json` lock bump

## Goal

Fix the `normal` issue "Ruby `Vec<T>` parameters segfault when an element's `to_str` mutates the
array". Every Ruby array argument gets converted through `RArray::into_iter()` (dup + hide) instead
of Magnus' borrowed-slice `to_vec`. The step also bundles the `normal` issue "Dependabot alert: Ruby
`json` gem in `crates/iscc-rb/Gemfile.lock`", which touches the same crate.

## Alternatives Considered

- **Chosen:** Ruby array-conversion fix + `json` bump. It is a memory-safety crash reachable from
    plain Ruby and the handoff's "Next". Both fixes live in one crate, so one Ruby CI run covers
    both.
- **Rejected:** `@iscc/wasm` npm license/description (`Cargo.toml` + `release.yml` patch step). It
    is small, but it is metadata that only takes effect at release, and a crash outranks it. Good
    next pick.
- **Rejected:** `docs/c-ffi-api.md` type names. Rename vs cbindgen alias is a design call.

## Scope

- **Modify**: `crates/iscc-rb/src/lib.rs`, `crates/iscc-rb/Gemfile.lock` (lock file, outside the
    budget), `docs/ruby-api.md` (Error Handling section only)
- **Create / modify tests**: a new `crates/iscc-rb/test/test_array_args.rb`, or extend
    `test/test_seq.rb` / `test/test_iscc_lib.rb`
- **Reference**: `~/.cargo/registry/src/*/magnus-0.8.2/src/r_array.rs` (`to_vec` L1148, `into_iter`
    L1625: dups unless frozen), `src/try_convert.rs:239` (`Vec<T>` =
    `RArray::try_convert(val)?.to_vec()`), and `alg_simhash` in `lib.rs` (the existing safe pattern)

## Not In Scope

- A page-wide audit of every "Raises `RuntimeError`" line in `docs/ruby-api.md`. Change only the
    Error Handling section (around L852-866).
- Return-type `Vec<String>` (`iscc_decompose`, `decode_seq`, `sliding_window`). Outputs are safe.
- Refreshing other gems, or touching `rb_sys` (held at 0.9.123, see the `Gemfile` comment).
- The `lib/iscc_lib.rb` wrapper, unless a test proves it needs a change.
- Upstream Magnus reports, and the other bindings' array marshalling.

## Implementation Notes

**There are 7 conversion sites, not 5.** The 5 parameters are `gen_audio_code_v0(cv: Vec<i32>)`,
`gen_mixed_code_v0(codes: Vec<String>)`, `gen_iscc_code_v0(codes: Vec<String>)`,
`encode_seq(units: Vec<String>)` and `alg_minhash_256(features: Vec<u32>)`. The other 2 are inside
`gen_video_code_v0` and `soft_hash_video_v0`: they iterate the outer `RArray` safely, but each frame
goes through `TryConvert::try_convert(frame)` into `Vec<i32>`, which is the same `to_vec` walk over
the inner array.

Write one small private generic helper, `RArray` → `into_iter()` → `TryConvert` per element →
`Vec<T>`, and use it at all 7 sites. Parameters become `RArray`. Each inner frame is converted with
`RArray::try_convert(frame)?` first, then the helper. Keep the `function!` arities unchanged.
`RArray::try_convert` still coerces through `to_ary` and raises `TypeError` for a non-Array, the
same as today's `Vec<T>` path. A frozen array is not dup'd, but it also cannot be mutated.

The segfault repro from the issue (iter 209) is a ~2000-element array whose first element defines
`to_str`, which runs `arr.clear` then `GC.start`. Tests must:

- run that object through `encode_seq`, `gen_mixed_code_v0` and `gen_iscc_code_v0` without crashing.
    Assert the result equals the call on an unmutated array snapshot (or the expected error), not
    just "no crash"
- cover the integer path: an element whose `to_int` mutates the array, against `gen_audio_code_v0`
    or `alg_minhash_256`, plus one inner video frame. Whether this crashes today has not been
    probed. Do not claim it did
- pin the exception class for a non-Array argument and for a non-String element (e.g.
    `encode_seq([nil])`). Today that is `TypeError`. Update the `docs/ruby-api.md` Error Handling
    sentence to match: `TypeError` for wrong-type arguments, `RuntimeError` for invalid values

A test that runs a mutating callback should run in-process. A crash fails the whole `rake test` run,
and that failure is the signal.

**`json` bump (probed read-only):** `bundle outdated json` resolves. The rubocop constraint is
`json (~> 2.3)`, and 2.21.2 is the newest 2.x, so run `bundle update json --conservative` in
`crates/iscc-rb`. It should land 2.21.2 with no other line changes. If it moves anything besides
`json`, revert the lock and record why in the handoff.

**Rebuild the native extension before testing** (`bundle exec rake compile`). The gitignored `.so`
goes stale silently. This step is binding-only, so it trips neither the CRAP gate nor the iai gate.

## Verification

- `grep -nE '^fn [a-z_0-9]+\([^)]*Vec<|Vec<i32> = TryConvert::try_convert\(frame\)' crates/iscc-rb/src/lib.rs`
    prints nothing (checked against today's tree: it hits exactly the 7 sites and no return types or
    `collect` targets)
- In `crates/iscc-rb`: `bundle exec rake compile && bundle exec rake test` exits 0, and the tests
    include the mutating-`to_str` case for `encode_seq` and `gen_mixed_code_v0` plus the `to_int`
    case
- `cargo clippy -p iscc-rb -- -D warnings` and `bundle exec standardrb` (in `crates/iscc-rb`) exit 0
- `crates/iscc-rb/Gemfile.lock` contains `json (2.21.2)` (or a later 2.x) and `rb_sys (0.9.123)`
- A test asserts the `TypeError` class for `encode_seq([nil])`, and `docs/ruby-api.md` Error
    Handling names `TypeError` for wrong-type arguments
- `mise run check` exits 0

## Done When

The advance agent is done when all six verification criteria pass, with the Ruby suite run against a
freshly compiled extension.
