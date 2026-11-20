## 2026-11-20 — Ruby `i32` list elements raise `RangeError` for negative Bignums

**Done:** Added `array_to_i32_vec` (convert via `array_to_vec::<i64>`, then narrow with
`i32::try_from` → `RangeError`) plus a small `narrow_to_i32` helper in `crates/iscc-rb/src/lib.rs`.
Both `i32` conversion sites now go through it: `gen_audio_code_v0` and `frames_to_vec` (which serves
`gen_video_code_v0` and `soft_hash_video_v0`). `array_to_vec`'s doc comment no longer says "most"
and warns against `T = i32`.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: `array_to_i32_vec` + `narrow_to_i32`, two call sites rerouted,
    `array_to_vec`/`frames_to_vec` doc comments updated
- `crates/iscc-rb/test/test_array_args.rb`: 5 tests: negative Bignums `-(2**63)` and `-(2**62) - 1`
    in audio, video frame and soft-hash-video frame; Fixnums `2**31` and `-(2**31) - 1` in audio;
    boundaries `[-(2**31), 2**31 - 1]` accepted (Hash with `iscc`). Each `assert_raises` wraps one
    raw call with one bad value

**Verification:**

- `bundle exec rake compile:dev && LANG=C.UTF-8 bundle exec rake test`: 165 runs, 471 assertions, 0
    failures, 0 errors, 0 skips
- Mutation check: with the helper body swapped back to `array_to_vec::<i32>(ary)` and the gem
    rebuilt, exactly the 3 negative-Bignum tests failed (audio: "nothing was raised"; video and
    soft-hash: a different exception class than `RangeError`). Restored the source (`diff` clean),
    rebuilt, and the suite is green again
- `ruby -Icrates/iscc-rb/lib -riscc_lib -e '…gen_audio_code_v0([-(2**63)])…'` exits 0, message
    "integer -9223372036854775808 out of range for `i32`"
- `grep -n 'array_to_vec::<i32>\|-> Result<Vec<i32>, Error>' …` matches only line 55 (the helper's
    signature). `array_to_i32_vec(` is called at line 80 (`frames_to_vec`) and line 142
    (`gen_audio_code_v0`)
- `cargo clippy -p iscc-rb -- -D warnings` clean; `cargo fmt -p iscc-rb --check` exit 0;
    `bundle exec standardrb test/test_array_args.rb` exit 0
- `mise run check` exit 0

**Next:** The two JNI issues (null-handle guard, pending exception) both live in
`crates/iscc-jni/src/lib.rs` and fit together as the next step.

**Notes:**

- Magnus `TryConvert for i32` and `for i64` both start with `Integer::try_convert(val)?`
    (`try_convert.rs:73-87`), so non-Integer inputs (Strings, Floats) behave exactly as before. Only
    the integer range check changed. `test_non_integer_element_raises_type_error` still passes
- No core Rust, public API or benchmarked path changed, so CRAP and iai were not run
- The out-of-scope `u32`/`usize` conversions were left untouched, as next.md directs
