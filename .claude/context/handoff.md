## 2026-07-29 — Mint `gen_iscc_id_v1` on the Ruby (Magnus) surface

**Done:** Added the experimental `gen_iscc_id_v1` minting function to the Ruby binding (#43, 8th of
11 IDv1 fan-out surfaces). The native fn takes three `i64` params and validates the semantic
thresholds (`2^52`/`4096`/`2`) in ts→hub→realm order via a `checked()` helper *before* narrowing to
`(u64,u16,u8)`, mirroring the napi precedent.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: `checked(i64, i64, name)` helper + `gen_iscc_id_v1` native fn
    (returns `RHash` with key `"iscc"`); registered `_gen_iscc_id_v1` arity 3; header docstring
    32→33 (own file only, per Not-In-Scope).
- `crates/iscc-rb/lib/iscc_lib.rb`: `IdCodeResult < Result` class + positional
    `self.gen_iscc_id_v1(timestamp, hub_id, realm)` wrapper.
- `crates/iscc-rb/test/test_iscc_id.rb` (new): golden, round-trip (+corners), validation-order,
    negative-rejection tests.
- `docs/howto/ruby.md`: experimental IDv1 section with decode bit-math recipe + `IdCodeResult` row.

**Verification:**

- `bundle exec rake compile:dev` then `bundle exec rake test` — 129 runs, 368 assertions, 0
    failures.
- Golden asserted: `gen_iscc_id_v1(1_751_831_876_325_218, 1, 0).iscc == "ISCC:MAIGHFECJMOPMIAB"`.
- Validation order asserted first-fail-wins: `(1<<52, 4096, 2)`→timestamp, `(0, 4096, 2)`→hub,
    `(0, 0, 2)`→realm; negatives rejected on each param.
- Round-trip: `iscc_decode` + `digest.unpack1("Q>")` recovers `(ts, hub, realm)`, `vs == 1`.
- `cargo fmt -p iscc-rb --check` + `cargo clippy -p iscc-rb --all-targets -- -D warnings` clean;
    `bundle exec standardrb` clean on both Ruby files; `mise run check` — all prek hooks Passed.

**Next:** Continue #43 fan-out. Remaining surfaces: uniffi (Swift/Kotlin — higher-risk, one core
edit + 3 regenerated checked-in artifacts), dotnet C# consumer, cpp. Then the deferred repo-wide
Tier-1 32→33 doc/count sweep.

**Notes:** Ruby Integers are arbitrary precision, so the binding must validate before narrowing —
same lesson as napi/jni. The native fn rejects negatives (`value < 0`); the pure-Python ref never
sees them (unbounded ints), so this is hardening, not divergence. No API break, no hot path. #43
stays open (v0.6.0 blocker).
