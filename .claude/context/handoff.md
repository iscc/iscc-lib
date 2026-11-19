## 2026-11-19 — Ruby array arguments via `RArray::into_iter` (Magnus `Vec<T>` segfault) + `json` lock bump

**Done:** Every Ruby array argument now goes through a private helper `array_to_vec<T: TryConvert>`.
It calls `RArray::into_iter()`, which walks a hidden copy and skips the copy for frozen arrays. A
second helper, `frames_to_vec`, first runs `RArray::try_convert` on each video frame and then calls
`array_to_vec`. Together they cover all 7 sites: 5 `Vec<T>` params plus 2 inner frame conversions.
The `json` lock entry is bumped 2.21.1 → 2.21.2. `docs/ruby-api.md` Error Handling now says
`TypeError` for wrong-type arguments and `RuntimeError` for invalid values.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: added `array_to_vec` and `frames_to_vec` (with docstrings).
    `gen_audio_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`, `encode_seq` and `alg_minhash_256`
    now take `RArray`. `alg_minhash_256` returns `Result<RString, Error>`, because element
    conversion can now fail inside the function. Before, the same failure surfaced in Magnus'
    argument conversion. The two video functions use `frames_to_vec`. `function!` arities are
    unchanged
- `crates/iscc-rb/test/test_array_args.rb` (new, 14 tests):
    - 7 crash tests. Element 0 of a 2000-element array clears the array and runs `GC.start` inside
        `to_str`/`to_int`. Each test asserts that the result equals the call on an unmutated snapshot
        and that the array really got emptied. They cover `encode_seq`, `gen_mixed_code_v0`,
        `gen_iscc_code_v0` (same value or the same RuntimeError message), `gen_audio_code_v0`,
        `alg_minhash_256`, and an inner frame for `gen_video_code_v0` / `soft_hash_video_v0`
    - Behaviour checks: a frozen array works, `to_ary` coercion works
    - Exception classes: `TypeError` for `encode_seq([nil])`, a non-Array argument, a non-Integer
        element and a non-Array frame; `RuntimeError` for an invalid unit value
- `crates/iscc-rb/Gemfile.lock`: `json (2.21.2)`, the only changed line
- `docs/ruby-api.md`: rewrote the Error Handling intro sentence (TypeError vs RuntimeError)
- `crates/iscc-rb/CLAUDE.md`: added a key-files row for `test_array_args.rb`

**Verification:**

- **Pre-fix probe** (fresh `rake compile` of the unfixed tree; each test run alone with
    `-n /name/`): all 7 mutation tests aborted with exit 134, `[BUG] Segmentation fault`. That
    includes the integer path (`gen_audio`, `alg_minhash`, `gen_video`, `soft_hash`): `to_int`
    mutation crashed today too. The 4 type-error, frozen and `to_ary` tests already passed on the
    unfixed tree, so `TypeError` behaviour did not change
- Grep criterion `grep -nE '^fn [a-z_0-9]+\([^)]*Vec<|Vec<i32> = TryConvert::try_convert\(frame\)'`
    printed nothing (exit 1)
- After the fix, `bundle exec rake compile` exit 0 (`.so` mtime 07:20 today).
    `bundle exec rake test` gives 152 runs, 432 assertions, 0 failures. `test_array_args.rb` alone
    gives 14 runs, 29 assertions, 0 failures. The suite was re-run after the lock bump: still 152/0
- `cargo clippy -p iscc-rb -- -D warnings` exit 0. `bundle exec standardrb` exit 0 (before and after
    the lock bump)
- `Gemfile.lock` has `json (2.21.2)` and `rb_sys (0.9.123)`
- `mise run format` exit 0, `mise run check` exit 0

**Next:** the `@iscc/wasm` npm license/description metadata (`Cargo.toml` + `release.yml` patch
step), which next.md named as the runner-up.

**Notes:**

- **Deviation on the json bump:** the prescribed `bundle update json --conservative` did NOT stay
    json-only. It resolved `json (3.1.0)` and pulled in rubocop 1.88.2 → 1.91.0 (whose constraint is
    `json >= 2.3`) and standard 1.56.0 → 1.57.0. I restored the lock and ran
    `bundle update json --conservative --patch`, which gave exactly the one-line `json 2.21.2` diff.
    Side effect: the first run installed standard 1.57.0 into the local gem dir. It is unused
    because the lock pins 1.56.0
- The integer path segfaulted too, which the issue had not probed. The issue text can now say all 7
    sites crashed, not just the `to_str` ones
- No hot path touched (binding-only); the CRAP and iai gates do not apply
- `alg_minhash_256`'s return type changed only inside the Ruby binding's Rust code. Ruby callers see
    the same method
