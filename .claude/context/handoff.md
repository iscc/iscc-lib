## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the Ruby (Magnus) surface

**Verdict:** PASS_WITH_NOTES

**Summary:** The Ruby binding now exposes `gen_iscc_id_v1` (8th of 11 IDv1 fan-out surfaces, #43).
The native fn validates the three semantic thresholds (`2^52`/`4096`/`2`) in ts→hub→realm order via
a `checked()` helper before narrowing i64→(u64,u16,u8), matching the napi precedent and the
reference order (`iscc_id.py:127-133`). All required tests pass and gates are clean. One real but
pathological edge case (out-of-i64 args) is filed as a `[review]` issue — see Codex review.

**Verification:**

- [x] `bundle exec rake compile:dev` then `bundle exec rake test` — re-run here: 129 runs, 368
    assertions, 0 failures.
- [x] Golden — `gen_iscc_id_v1(1_751_831_876_325_218, 1, 0).iscc == "ISCC:MAIGHFECJMOPMIAB"` (test
    passes; matches core golden).
- [x] Validation order first-fail-wins — `(1<<52, 4096, 2)`→timestamp, `(0, 4096, 2)`→hub,
    `(0, 0, 2)`→realm; negatives rejected on each param. Order confirmed against reference.
- [x] Round-trip — `iscc_decode` + `digest.unpack1("Q>")` bit-math recovers `(ts, hub, realm)`,
    `vs == 1`. `iscc_decode` returns `version` as a plain Integer, so no enum widening needed.
- [x] `cargo clippy -p iscc-rb --all-targets -- -D warnings` + `cargo fmt -p iscc-rb --check` clean;
    `bundle exec standardrb` clean on both Ruby files; `mise run check` — all prek hooks Passed.
- [x] (probe) Scope clean — 2 code files (`src/lib.rs`, `iscc_lib.rb`) + 1 test + 1 doc, all in
    next.md's Modify/Create list; header docstring 32→33 own-file only per Not-In-Scope. No API
    break, no hot path, no gate circumvention across `@{upstream}..HEAD`.

**Issues found:**

- (filed `[review]`, normal) Ruby validation order breaks for args `> i64::MAX`: Magnus narrows
    Integer→i64 during marshalling, *before* the fn body, so `gen_iscc_id_v1(1<<52, 1<<100, 2)`
    raises `RangeError` instead of reporting the earlier-invalid timestamp, and `(1<<70,0,0)` raises
    `RangeError` where the docstring promises `RuntimeError`. Only triggers for values beyond
    `i64::MAX` (≈9.2e18) — no realistic timestamp/hub/realm reaches it, so practical impact is nil.

**Codex review:** One P2 finding, valid and confirmed live: arbitrary-precision args exceeding i64
bypass the ordered validation during Magnus marshalling. Filed as the `[review]` issue above rather
than blocking — the delivered work is correct for all realistic inputs, the advance agent faithfully
implemented next.md's prescribed `i64` design, and the divergence needs a `2^100`-scale bignum to
manifest. Fix belongs with a cross-surface pass (validate magnitude in Ruby, or accept
`magnus::Integer`).

**Next:** Continue #43 fan-out. Remaining minting surfaces: **uniffi** (Swift/Kotlin — higher-risk:
one core edit + 3 regenerated checked-in artifacts), **dotnet** C# consumer, **cpp**. uniffi/dotnet
use fixed-width FFI types so they are exempt from the Ruby marshalling gap, but confirm during their
step. Then the deferred repo-wide Tier-1 32→33 doc/count sweep.

**Notes:** #43 stays open (v0.6.0 blocker). Lesson recorded: any arbitrary-precision-input surface
must validate magnitude *before* the native narrowing layer, not just before the codec narrowing —
the "validate before narrowing" principle applies to the marshalling boundary too.
