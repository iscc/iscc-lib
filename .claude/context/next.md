# Next Work Package

## Step: Mint `gen_iscc_id_v1` on the Ruby (Magnus) surface

## Goal

Add the experimental `gen_iscc_id_v1` minting function to the Ruby binding (issue #43, a v0.6.0
release blocker), completing the 8th of 11 IDv1 fan-out surfaces. Ruby is a wide-int surface, so it
must validate the three semantic thresholds in normative ts→hub→realm order *before* narrowing.

## Alternatives Considered

- **Chosen:** Ruby `gen_iscc_id_v1` — the handoff's designated next surface; applies the fresh
    "validate before narrowing" pattern (napi/wasm/jni) while it is well-understood.
- **Rejected:** uniffi (Swift/Kotlin) minting — higher-risk single step (one core edit plus three
    regenerated checked-in artifacts); do it after the simpler rb surface. The 32→33 doc sweep is
    lower value while minting surfaces remain unshipped.

## Scope

- **Modify**: `crates/iscc-rb/src/lib.rs` (native fn + `init` registration + header docstring),
    `crates/iscc-rb/lib/iscc_lib.rb` (idiomatic wrapper + result class), `docs/howto/ruby.md` (IDv1
    section with the decode bit-math recipe)
- **Create**: `crates/iscc-rb/test/test_iscc_id.rb` (golden + round-trip + validation-order tests)
- **Reference**: `crates/iscc-napi/src/lib.rs:301-335` (`checked`/`gen_iscc_id_v1` validation
    precedent), `crates/iscc-lib/src/lib.rs:1064-1140` (core fn + golden/round-trip/ordering tests),
    spec `rust-core.md` "ISCC-IDv1 Operations" (validation order, test placement)

## Not In Scope

- The repo-wide Tier-1 32→33 doc/count sweep (separate step) — only fix `src/lib.rs`'s **own**
    header docstring (`Symbols (32 of 32)` → 33, add the new name) since you edit that file.
- No `decode_iscc_id_v1` and no Ruby version enum widening — `iscc_decode` returns `version` as a
    plain Integer, so the decode round-trip already works with no wrapper change.
- Do not touch other surfaces (uniffi/dotnet/cpp) or the codec.

## Implementation Notes

- Ruby Integers are arbitrary precision. Take all three params as `i64` in the native fn and add a
    `checked(value: i64, max_exclusive: i64, name) -> Result<i64, Error>` helper that rejects
    `value < 0 || value >= max_exclusive` with a `RuntimeError`. Call it in order: `timestamp`
    (`< 4_503_599_627_370_496` = 2^52) → `hub_id` (`< 4096`) → `realm` (`< 2`), then narrow
    (`as u64/u16/u8`) and call `iscc_lib::gen_iscc_id_v1`. First failing check wins.
- Native fn returns `RHash` with key `"iscc"` (mirror `gen_text_code_v0`); register as
    `_gen_iscc_id_v1` with arity 3. Ruby wrapper `self.gen_iscc_id_v1(timestamp, hub_id, realm)`
    (positional, reference order) wraps it in a new `IdCodeResult < Result` class.
- Round-trip recipe for the docs + test: `mt, st, vs, li, digest = IsccLib.iscc_decode(iscc)`, then
    `n = digest.unpack1("Q>")`, `timestamp = n >> 12`, `hub_id = n & 0xFFF`, `realm = st`;
    `vs == 1`.

## Verification

- Rebuild the native ext first (`cd crates/iscc-rb && bundle exec rake compile:dev`), then
    `bundle exec rake test` passes — new golden, round-trip, and validation-order tests included.
- Golden: `IsccLib.gen_iscc_id_v1(1751831876325218, 1, 0).iscc == "ISCC:MAIGHFECJMOPMIAB"`.
- Validation order asserted: `(1<<52, 4096, 2)`→timestamp msg, `(0, 4096, 2)`→hub msg,
    `(0, 0, 2)`→realm msg (first-failing-check-wins).
- Round-trip: `IsccLib.iscc_decode(IsccLib.gen_iscc_id_v1(1751831876325218,1,0).iscc)[2] == 1` and
    the bit-math recovers `(1751831876325218, 1, 0)`.
- `cargo clippy -p iscc-rb --all-targets -- -D warnings` and `cargo fmt -p iscc-rb --check` clean;
    `bundle exec standardrb` clean on the modified/created Ruby files.

## Done When

Ruby exposes `gen_iscc_id_v1` with ordered ts→hub→realm validation, the golden/round-trip/ordering
tests pass under `rake test`, and all lint/format checks are clean.
