## 2026-11-20 — Review of: Ruby `i32` list elements raise `RangeError` for negative Bignums

**Verdict:** PASS

**Summary:** The advance agent added `array_to_i32_vec` (convert through `i64`, narrow with
`i32::try_from`) to `crates/iscc-rb/src/lib.rs` and routed both `i32` conversion sites through it,
with 5 tests in `test_array_args.rb`. The diff is one code file plus one test file, matches next.md,
and closes the `normal` issue: `IsccLib.gen_audio_code_v0([-(2**63)])` now raises `RangeError`. The
root cause and the "unsigned conversions are already correct" claim both hold in the Magnus 0.8.2
source (`r_bignum.rs`, `value.rs`).

**Verification:**

- [x] `cd crates/iscc-rb && bundle exec rake compile:dev && LANG=C.UTF-8 bundle exec rake test`
    exits 0 — fresh `.so`, 165 runs, 471 assertions, 0 failures, 0 errors, 0 skips
- [x] `ruby -Icrates/iscc-rb/lib -riscc_lib -e '…gen_audio_code_v0([-(2**63)])…'` exits 0
- [x] `grep -n 'array_to_vec::<i32>\|-> Result<Vec<i32>, Error>' crates/iscc-rb/src/lib.rs` shows
    only the helper's signature (line 55); `frames_to_vec` (line 80) and `gen_audio_code_v0` (line
    142\) call `array_to_i32_vec`
- [x] `cargo clippy -p iscc-rb -- -D warnings` and `cargo fmt -p iscc-rb --check` are clean
    (workspace clippy with `--all-targets` is clean too)
- [x] `cd crates/iscc-rb && bundle exec standardrb test/test_array_args.rb` exits 0
- [x] `mise run check` exits 0

Extra probes (3): a boundary sweep over 14 values from `-(2**64)` to `2**64` gave `RangeError`
outside `i32` and a code inside it for audio, video and soft-hash-video, and the correct `u32` /
`usize` bounds for `alg_minhash_256` and `sliding_window`. An in-range audio result equals the
Python binding's. A Float element is accepted (see below).

**Issues found:**

- Filed `low`, untriaged: "Ruby integer list elements accept a Float and truncate it".
    `gen_audio_code_v0([1.9])` returns the code of `[1]` where Python raises `TypeError`. It is old
    behaviour and follows Ruby's implicit `to_int` convention, so Titusz decides whether it is a
    defect. The new helper's doc comment ("`TypeError` for a non-Integer") is loose on this point
- Not filed (nits): the helper builds a `Vec<i64>` and then a `Vec<i32>`, where one pass would do
    (next.md prescribed this shape; the lists are small). Because of the two passes, `[2**31, "x"]`
    now raises `TypeError` where it raised `RangeError` before
- I did not repeat the advance agent's mutation run. Its result is consistent with the issue's
    measured pre-fix behaviour, and the tests ran against a rebuilt extension

**Codex review:** No actionable findings. Codex rebuilt the extension, ran the 165 tests and 75
boundary and coercion checks of its own, plus clippy, fmt and standardrb.

**Next:** The two JNI issues, as one step in `crates/iscc-jni/src/lib.rs`: the null-handle guard on
the four streaming-hasher functions (`normal`, `[human]`) and the pending-exception handling in
`throw_and_default` (`normal`, `[review]`). If define-next finds they need separate verdicts, take
the null-handle guard first: it is the smaller one and has a clear test per function.

**Notes:**

- Remaining `normal` issues after this step: the two JNI ones, the `iscc-ffi` `lib.rs` split, the
    derived-facts parity check and the `data-edge.json` harness strictness (Swift and Python)
- JNI review needs `mvn clean test` (plain `mvn test` reuses stale classes) and a run under
    `JAVA_TOOL_OPTIONS=-Xcheck:jni`. The pom's `<argLine>` overrides `-DargLine`, so a small `-Xmx`
    for the `OutOfMemoryError` test must go through the pom or `JAVA_TOOL_OPTIONS`
- The 2026-07-28 decision (JNI exports keep hand-thrown exceptions) stays binding for that step
- No core Rust, public API or benchmarked path changed here, so CRAP and iai were not run
