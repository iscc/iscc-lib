## 2026-03-03 — Add gen_video/mixed/data_code_v0 to Ruby bridge

**Done:** Added `gen_video_code_v0`, `gen_mixed_code_v0`, and `gen_data_code_v0` to the Ruby Magnus
bridge, advancing from 19/32 to 22/32 Tier 1 symbols. Each function follows the established pattern:
Rust bridge function returning `RHash`, `_`-prefixed module registration, Ruby `Result` subclass,
and keyword-arg wrapper with `bits: 64` default.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: Added 3 bridge functions (`gen_video_code_v0` with `RArray` →
    `Vec<Vec<i32>>` nested conversion via `into_iter()` + `TryConvert`, `gen_mixed_code_v0` with
    `Vec<String>` → `Vec<&str>` conversion, `gen_data_code_v0` with `RString` binary slice pattern).
    Registered all 3 in `init()`. Updated symbol count in docstring from 19 to 22.
- `crates/iscc-rb/lib/iscc_lib.rb`: Added `VideoCodeResult`, `MixedCodeResult`, `DataCodeResult`
    classes and corresponding `self.gen_video_code_v0`, `self.gen_mixed_code_v0`,
    `self.gen_data_code_v0` wrapper methods.
- `crates/iscc-rb/test/test_smoke.rb`: Added 6 smoke tests (basic + attribute access for each new
    function). Video test uses 400-element frame arrays (WTA-Hash minimum is 380). Mixed test uses
    Text-Code + Image-Code (Content-Codes required, not Meta-Codes).

**Verification:**

- `cargo check -p iscc-rb` — compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- `bundle exec rake compile` — builds native extension in release profile
- `bundle exec rake test` — 31 runs, 75 assertions, 0 failures, 0 errors, 0 skips
- `gen_video_code_v0` one-liner — prints `ISCC:EMAVFD4RIMPXYSWS`
- `gen_data_code_v0` one-liner — prints `ISCC:GAAY5EN4BF2LH3FW`
- `mise run check` — all pre-commit hooks pass

**Next:** Continue adding gen functions to Ruby bridge. Next batch should be `gen_instance_code_v0`
(binary data → hash with `iscc`, `datahash`, `filesize`), `gen_iscc_code_v0` (string arrays + wide
flag), and `gen_sum_code_v0` (file path + add_units flag). These have the most complex return types
of the remaining functions.

**Notes:**

- Magnus 0.7.1 `RArray::each()` is deprecated — must use `into_iter()` instead. The `next.md`
    example code used the deprecated API.
- `TryConvert::try_convert(val)` is an associated function in Magnus 0.7.1, not a method — cannot
    call `val.try_convert()`. Required explicit import of `TryConvert` trait.
- `gen_video_code_v0` requires frame signatures with ≥380 elements per frame (WTA-Hash minimum).
    Small test arrays like `[[1,2,3]]` will error.
- `gen_mixed_code_v0` only accepts Content-Codes (Text/Image/Audio/Video), not Meta-Codes. Passing
    Meta-Code ISCCs results in "all codes must be Content-Codes" error.
