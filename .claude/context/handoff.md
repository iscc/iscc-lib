## 2026-11-22 — One-shot gen functions stop re-implementing their streaming counterparts

**Done:** Each of the three computations the `[audit]` issue cites now exists once. A
`pub(crate) utils::blake3_multihash_hex(&blake3::Hash)` writes the `1e`/`20` prefix in one place,
and `multi_hash_blake3`, `InstanceHasher::finalize` and `gen_instance_code_v0` all call it.
`gen_instance_code_v0` now hashes once. WASM `gen_sum_code_v0` calls the core
`streaming::SumHasher`. A private `canonicalize_json` parses and canonicalizes JSON meta once, for
both `gen_meta_code_v0` and `json_to_data_url`.

**Files changed:**

- `crates/iscc-lib/src/utils.rs`: added `blake3_multihash_hex`; `multi_hash_blake3` delegates to it
    and is gated `#[cfg(any(feature = "meta-code", test))]`. Its only non-test caller left is
    Meta-Code, so `--no-default-features` clippy flagged it as dead code
- `crates/iscc-lib/src/streaming.rs`: `InstanceHasher::finalize` uses the helper (no `format!`)
- `crates/iscc-lib/src/lib.rs`: `parse_meta_json` replaced by
    `canonicalize_json(json,   parse_error_prefix) -> CanonicalJson { bytes, is_json_ld }`.
    `build_meta_data_url` takes `&CanonicalJson`. `gen_meta_code_v0` resolves `(payload, data_url)`
    in one match with no second parse, and both size checks run in the same order as before.
    `json_to_data_url` is a one-liner. `gen_instance_code_v0` formats from its own digest. 3 tests
    added; one existing assertion *message* (not its condition) renamed `parse_meta_json` →
    `canonicalize_json`
- `crates/iscc-wasm/src/lib.rs`: `gen_sum_code_v0` = `SumHasher` new/update/finalize, with the arg
    checks in the original order and the doc comment updated. Added private
    `wasm_sum_result(SumCodeResult)`, shared with `SumHasher::finalize`, which had the same field
    mapping
- `crates/iscc-wasm/tests/unit.rs`: `test_gen_sum_code_v0_matches_sum_hasher_wide_with_units` (bits
    128, wide, add_units, 20 kB input; all 4 fields equal, 2 units)

**Verification:**

- `cargo test -p iscc-lib`: 450 passed, 0 failed. `--no-default-features`: 384 passed.
    `--no-default-features --features text-processing`: 417 passed
- clippy `-p iscc-lib --no-default-features -- -D warnings`, the same with `--tests` for both
    feature legs, and `--workspace --exclude iscc-rb --all-targets -- -D warnings`: no errors
- `wasm-pack test --node crates/iscc-wasm --features conformance`: exit 0 (11 + 8 + 127 passed), the
    new sum test included
- `mise run bench:iai:check`: exit 0. `bench_instance_code.bytes_1m` 5,236,470 → 2,621,027 Ir
    (−49.95%) and `bytes_64k` −49.22%. All other benches fall between +0.00% and −7.65%, which is
    the known baseline drift (the meta benches pass no JSON `meta`). Baseline not refreshed
- CRAP: `cargo llvm-cov … && cargo crap … --fail-regression --fail-above`: exit 0, "0 regressed, 2
    improved, 2 new, 31 moved, 1 removed (parse_meta_json)". No baseline refresh
- Greps: `multi_hash_blake3` in `gen_instance_code_v0` = 0, `serde_json::from_str` in
    `gen_meta_code_v0` = 0, `blake3::hash` in `gen_instance_code_v0` = 1. In
    `crates/iscc-wasm/src/lib.rs`, `instance_hasher\|1e20{` = 0. **In `streaming.rs` it prints 11,
    not 0:** every match is `SumHasher`'s own `instance_hasher` field (the one remaining
    implementation) or a `test_instance_hasher_*` test name. `grep -c '1e20{'` on that file = 0. I
    did not rename the field or the tests just to make a grep pass. The criterion is over-broad
- `mise run check`: exit 0

**Next:** the Rust half of "Rust and Go per-function tests re-parse `data.json`", which next.md
named as the follow-up.

**Notes:**

- No public signature, error text or result field changed. Error texts are pinned by the new
    `test_json_parse_error_texts`: `invalid JSON in meta: …` from `gen_meta_code_v0` and
    `invalid JSON: …` from `json_to_data_url`. `JSON canonicalization failed: …` is shared. The old
    second parse in `gen_meta_code_v0` had an unreachable `invalid JSON:` arm, now gone
- `gen_meta_code_v0` now builds the data URL (one base64 encode) before the post-decode size check.
    Only an oversized payload that is about to be rejected pays for it, and hashing still happens
    after the check
- Out of scope, not touched: `gen_data_code_v0` vs `DataHasher` (as next.md says)
