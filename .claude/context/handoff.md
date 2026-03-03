## 2026-03-03 — Add gen_text/image/audio_code_v0 to Ruby bridge

**Done:** Added `gen_text_code_v0`, `gen_image_code_v0`, and `gen_audio_code_v0` to the Ruby Magnus
bridge, advancing from 16/32 to 19/32 Tier 1 symbols. Each function follows the established pattern:
Rust bridge function with `RHash` return, `_`-prefixed registration in `init()`, Ruby Result
subclass, and keyword-arg wrapper method.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: Added 3 bridge functions (`gen_text_code_v0`, `gen_image_code_v0`,
    `gen_audio_code_v0`) + registered in `init()` with `_` prefix. Updated symbol count 16→19.
- `crates/iscc-rb/lib/iscc_lib.rb`: Added `TextCodeResult`, `ImageCodeResult`, `AudioCodeResult`
    classes and 3 wrapper methods with keyword `bits:` parameter.
- `crates/iscc-rb/test/test_smoke.rb`: Added 6 test methods covering basic functionality and
    attribute access for all 3 new functions.

**Verification:** All 5 verification criteria from next.md pass:

- `cargo check -p iscc-rb` — compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- `bundle exec rake compile` — builds native extension in release profile
- `bundle exec rake test` — 25 runs, 61 assertions, 0 failures, 0 errors, 0 skips
- `bundle exec ruby -e "..."` — prints `ISCC:EAASKDNZNYGUUF5A`
- `mise run check` — all 14 pre-commit hooks pass
- `mise run lint` — all checks pass

**Next:** Continue adding gen functions to Ruby bridge. Next batch: `gen_video_code_v0` (nested
`Vec<Vec<i32>>` frame signatures — needs special handling), `gen_mixed_code_v0` (`Vec<String>`
input), `gen_data_code_v0` (`&[u8]` binary data). These three have slightly more complex parameter
types than the current batch.

**Notes:**

- `gen_image_code_v0` requires exactly 1024 pixels (32×32 image). Initial test used 100 bytes which
    caused a RuntimeError. Fixed to use 1024-byte buffers.
- `gen_image_code_v0` uses the `RString` + `unsafe { pixels.as_slice() }` pattern (same as
    `encode_base64`) for binary data — no copy needed since the slice is passed directly to a pure
    Rust function without intervening Ruby API calls.
- `gen_audio_code_v0` takes `Vec<i32>` — Magnus handles the Ruby Array → Vec conversion
    automatically.
