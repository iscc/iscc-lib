## 2026-07-28 — Add `gen_iscc_id_v1` minting to the Rust core

**Done:** Implemented Tier 1
`gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8) -> IsccResult<IsccIdResult>` (issue #43 Part
2\) plus the `IsccIdResult { iscc }` result struct, matching the `iscc-core` reference. Clock-free —
caller supplies the timestamp. Re-baselined the CI CRAP gate to absorb the new function.

**Files changed:**

- `crates/iscc-lib/src/types.rs`: added `IsccIdResult { pub iscc: String }`, `#[non_exhaustive]`.
- `crates/iscc-lib/src/lib.rs`: added `pub fn gen_iscc_id_v1` (before tests mod) + 3 `#[test]`s
    (golden, round-trip, validation-ordering).
- `.crap-baseline.json`: regenerated (generated artifact; +1 entry for `gen_iscc_id_v1`).

**Verification:**

- `cargo test -p iscc-lib` — **302 (+3) passed, 0 failed**; new golden/round-trip/validation tests
    green. Golden: `gen_iscc_id_v1(1751831876325218, 1, 0).iscc == "ISCC:MAIGHFECJMOPMIAB"`.
- `cargo clippy -p iscc-lib -- -D warnings` — clean. `cargo fmt -p iscc-lib --check` — clean.
- `mise run coverage` +
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression   --fail-above` —
    **exit 0** after re-baseline: `0 regressed, 0 new, 0 moved, 108 unchanged`.
- `mise run check` — all prek hooks Passed (no failures/errors).
- `cargo build -p iscc-lib --no-default-features` — compiles (new tests use no feature-gated deps).

**Next:** Issue #43 fan-out remains. Suggested order: (1) 11-surface binding fan-out wrapping
`gen_iscc_id_v1`; (2) Go `EncodeIsccID` rename + `DecodeIsccID`/`IsccIDv1Result` deletion; (3)
Python differential test against `iscc_core` (needs rebuilt wheel); (4) Tier-1 32→33 doc/count sweep
across `docs/`, `notes/`, CLAUDE.md/README.

**Notes:**

- Validation order matches reference exactly, first failure wins: `timestamp >= 2^52` → "Timestamp
    overflow"; `hub_id >= 2^12` → "HUB-ID overflow"; `realm ∉ {0,1}` → "Realm-ID must be 0 (test) or
    1 (operational)". Test asserts `to_string()` with the `IsccError` Display prefix
    `invalid input: `.
- Tier 1 is now **33 symbols** (was 32). The CLAUDE.md/notes/docs count text still says 32 — that is
    the deliberately-separate doc-sweep step (Not In Scope here), so it is currently stale by
    design.
- Additive symbol, no API break — `semver` stays informational. No new bench, no iai baseline
    change.
- The gate flagged only `1 new` (no moved) even before re-baseline — adding the fn just before the
    tests mod shifted no scored function bodies. Re-baselined anyway per next.md to bake it in.
- Pre-existing, out-of-scope: `cargo test -p iscc-lib --no-default-features` fails to COMPILE (a
    test calls `gen_meta_code_v0` ungated) — unchanged by this step; the library build is clean.
