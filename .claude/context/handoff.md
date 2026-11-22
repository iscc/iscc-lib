## 2026-11-22 — Rust core rejects ISCC-UNITs above 256 bits (encode and decode)

**Done:** `codec::encode_length` accepts only 32 to 256 bits (multiples of 32) for Meta through
Flake, so every gen function, all three hashers' `finalize` and both `encode_component` variants
reject `bits=288`. A `pub(crate)` checked helper `codec::unit_bit_length` (wraps `decode_length`,
errors on a Meta..Flake length above 256 bits; ISCC/ID unchanged) replaces all four internal
`decode_length` calls. No `codec` signature changed.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: bound + error text in `encode_length`
    (`invalid length {length} for {mtype:?} (must be a multiple of 32 from 32 to 256)`); doc notes
    on `encode_length`/`decode_length` (unchecked, points to the rule); `unit_bit_length` helper;
    `iscc_decompose` uses it; tests `test_unit_bit_length_above_256` (7 ok, 8 error, ISCC 320 and ID
    96 unchanged), `test_iscc_decompose_above_256`, 288/512 cases in `test_encode_length_invalid`
- `crates/iscc-lib/src/lib.rs`: `iscc_decode` and `soft_hash_codes_v0` (mixed) use the helper; the
    wide check in `gen_iscc_code_v0` calls a private `units_reach_128_bits` (stops at the first unit
    under 128 bits, like the reference's `all(...)`, and is reached only after the `wide`, count and
    Data+Instance short-circuits); `# Errors` text of `encode_component`/`iscc_decode`; 12
    `*_above_256` tests (meta, text, audio, mixed bits, mixed 288-bit input unit, data, instance,
    sum, Tier 1 `encode_component` 512 + 288, `iscc_decode`, wide `gen_iscc_code_v0`)
- `crates/iscc-lib/src/streaming.rs`: `# Errors` sections on the three `finalize`; 3 tests
- `docs/rust-api.md`: 7 `bits` rows + `gen_sum_code_v0` row say "multiple of 32, 32 to 256";
    `encode_component`, `iscc_decode` and streaming prose name the rule
- `.crap-baseline.json`: refreshed (see below)

**Verification:**

- `cargo test -p iscc-lib`: all pass (lib 341); `--no-default-features`: all pass (lib 284);
    `cargo test -p iscc-lib above_256`: 16 passed, 0 failed
- Mutation check: with both bounds disabled, 15 of the 16 tests fail. The exception is
    `test_gen_mixed_code_v0_bits_above_256`: no valid input unit exceeds 256 bits, so mixed
    `bits=288` was already rejected by the existing "Content-Code too short" check (the reference
    does the same). The input-unit side is covered by `test_gen_mixed_code_v0_unit_above_256`, which
    does fail without the fix
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- grep: `pub fn decode_length(...) -> u32` prints one line (codec.rs:414);
    `git grep -n "decode_length(" crates/iscc-lib/src/lib.rs` prints nothing
- CRAP: the first run of the CI lines exited 1 with 4 regressed, each +1 cc from the new `?`:
    `gen_iscc_code_v0` 21.08→22.10, `iscc_decompose` 19→20, `soft_hash_codes_v0` 7.12→8.15,
    `iscc_decode` 4→5. After `mise run crap:baseline` (re-run after rustfmt touched the file),
    `mise run coverage && cargo crap ... --fail-regression --fail-above` exits 0 (0 regressed, 190
    unchanged)
- `mise run bench:iai:check`: exit 0, 16 benchmarks within 10%. Against the previous local run:
    `bench_mixed_code` +0.44%, `bench_iscc_code` +0.28%, all others under +0.01% or unchanged
- `uv run pytest tests/test_rust_api_signatures.py`: 5 passed; `mise run check`: exit 0 (the first
    run's rustfmt fix was applied, second run clean)
- `data.json` vectors: the conformance tests still pass, so no vector uses a unit above 256 bits

**Next:** the Go port (`packages/go`: `encodeLength` bound plus a checked decode on the
`IsccDecode`/`IsccDecompose`/mixed/wide paths), then the `data-edge.json` rows (`diverges`), then
the binding API pages ("32 to 256").

**Notes:**

- The 288-bit `test_gen_iscc_code_v0_wide_above_256` also asserts that `wide=false` with the same
    288-bit Data unit still succeeds, pinning "non-wide unchanged, as in iscc-core"
- Rust error-order nuance in mixed: the core checks main type, then length, per code. The reference
    checks all main types first, then all lengths. Both reject; only which message wins can differ
    when two inputs are bad in different ways
- The `iai_regression.py` table still shows the stale `bench_instance_code` rows at about −50% (the
    existing `low` issue). I did not refresh `.iai-baseline.json`
