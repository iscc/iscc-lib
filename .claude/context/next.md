# Next Work Package

## Step: Ruby `i32` list elements raise `RangeError` for negative Bignums

## Goal

Fix the `normal` issue "Ruby `i32` list elements silently truncate negative Bignums": every `i32`
element conversion in `crates/iscc-rb/src/lib.rs` must raise `RangeError` for any value outside
`i32`, so `IsccLib.gen_audio_code_v0([-(2**63)])` stops returning the code of `[0]`.

## Alternatives Considered

- **Chosen:** Ruby `i32` narrowing. It is the handoff's pick, a one-file fix with a measured root
    cause, and it brings the binding in line with what `docs/ruby-api.md` → Error Handling already
    promises.
- **Rejected:** the two JNI issues (null-handle guard, pending exception). They share
    `crates/iscc-jni/src/lib.rs` and make a good next step, but they are a different crate with a
    different outcome. Bundling them would add two unrelated verdicts to one review.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-rb/src/lib.rs`; `crates/iscc-rb/test/test_array_args.rb` (tests)
- **Reference**: `~/.cargo/registry/src/*/magnus-0.8.2/src/r_bignum.rs` (`to_i32` at line 194,
    `to_i64` at line 243), `~/.cargo/registry/src/*/magnus-0.8.2/src/value.rs` (`Fixnum::to_i32` at
    line 2575), `crates/iscc-rb/CLAUDE.md` → Local Development

## Not In Scope

- `u32` / `usize` conversions (`alg_minhash_256` elements, `bits`, `nbytes`, `width`,
    `avg_chunk_size`, `bit_length`). I probed Magnus 0.8.2: `Fixnum` and `RBignum` `to_u32` /
    `to_usize` both reject negatives and check the upper bound, so they are correct. Leave them
    alone.
- `checked()` / `gen_iscc_id_v1`. It narrows through its own helper and raises `RuntimeError` by
    design (see the docs).
- A `data-edge.json` vector. An out-of-`i32` element is a native-type range limit: the Go, C, C++,
    .NET, Java, Swift and Kotlin harnesses would reject the value in their own JSON-to-`int32`
    conversion before it reached the binding. `edge-vectors.md` → Rules keeps inputs like that in
    per-binding tests.
- Upgrading Magnus or patching it upstream.
- Any edit to `docs/ruby-api.md`. Its Error Handling text already describes the fixed behaviour.

## Implementation Notes

- **Root cause (measured):** Magnus 0.8.2 `RBignum::to_i32` calls `rb_num2long` and checks only
    `res > i32::MAX` before `as i32`. On 64-bit, Fixnums cover ±2^62. So Bignums from `-(2**63)` to
    `-(2**62)-1` pass `rb_num2long` and get truncated. Values below `-(2**63)` already raise inside
    `rb_num2long`, and `Fixnum::to_i32` checks both bounds.
- **Fix shape:** add one helper that converts an `RArray` into `Vec<i32>` by going through
    `array_to_vec::<i64>` (Magnus' `i64` conversion raises `RangeError` for anything outside `i64`,
    and also for non-Integers, which keeps the current `TypeError` behaviour). Then narrow each
    value with `i32::try_from`, mapping a failure to `Error::new(ruby.exception_range_error(), …)`.
    Get `ruby` from `Ruby::get()`, as the file does elsewhere. Keep `array_to_vec`'s hidden-copy
    walk; never use Magnus' `Vec<T>` conversion (learnings: it segfaults on a shrinking array).
- **Finish the pattern:** there are two `i32` conversion sites, `gen_audio_code_v0` (line 115) and
    `frames_to_vec` (line 53, which serves `gen_video_code_v0` and `soft_hash_video_v0`). Route both
    through the helper, so that `Vec<i32>` is never produced by `array_to_vec::<i32>` directly. Then
    update `array_to_vec`'s doc comment, which today says "`RangeError` for most out-of-range
    integers".
- **Tests:** add them next to `test_out_of_range_integer_element_raises_range_error`. Assert each
    rejection on the raw call alone, one bad value per call (learnings). Cover:
    - `RangeError` for `-(2**63)` and `-(2**62) - 1` in `gen_audio_code_v0` (as a list element) and in
        `gen_video_code_v0` and `soft_hash_video_v0` (as a frame element, e.g. `[[-(2**63)]]`; the
        conversion runs before the core sees the frame length)
    - `RangeError` for `2**31` and `-(2**31) - 1` in audio (Fixnum bounds, so they stay pinned)
    - Boundaries accepted: `gen_audio_code_v0([-(2**31), 2**31 - 1])` returns a Hash with an `iscc`
        key
- **Build before testing:** the `.so` is gitignored and goes stale silently. Run from
    `crates/iscc-rb`: `bundle exec rake compile:dev`, then the tests, with `LANG=C.UTF-8` exported
    (Ruby conformance tests crash without it).
- **Gates:** this changes no core Rust, so CRAP and iai are untouched. `iscc-rb` is excluded from
    workspace clippy in CI, so run clippy on the crate explicitly.

## Verification

- `cd crates/iscc-rb && bundle exec rake compile:dev && LANG=C.UTF-8 bundle exec rake test` exits 0,
    including the new negative-Bignum and boundary tests in `test_array_args.rb`
- `ruby -Icrates/iscc-rb/lib -riscc_lib -e 'begin; IsccLib.gen_audio_code_v0([-(2**63)]); exit 1; rescue RangeError; exit 0; end'`
    exits 0 (run after the compile above)
- `grep -n 'array_to_vec::<i32>\|-> Result<Vec<i32>, Error>' crates/iscc-rb/src/lib.rs` shows only
    the new helper's signature, and `gen_audio_code_v0` and `frames_to_vec` both call that helper
- `cargo clippy -p iscc-rb -- -D warnings` and `cargo fmt -p iscc-rb --check` are clean
- `cd crates/iscc-rb && bundle exec standardrb test/test_array_args.rb` exits 0
- `mise run check` exits 0

## Done When

The advance agent is done when every `i32` element conversion in the Ruby binding raises
`RangeError` for out-of-range Bignums and all verification criteria pass.
