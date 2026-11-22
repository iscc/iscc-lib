# Next Work Package

## Step: Rust core rejects ISCC-UNITs above 256 bits (encode and decode)

## Goal

First slice of the `normal` issue "Gen functions accept `bits` above 256 and return a unit that no
decoder reads" (decision 2026-11-22, "A unit is at most 256 bits"): the Rust core alone rejects a
unit length above 256 on every encode path and on every header decode path. The 10 surfaces built on
the core inherit it; the Go port, the `data-edge.json` rows and the per-surface API pages come
after.

## Alternatives Considered

- **Chosen:** the core bound — the longest remaining chain toward v0.8.0 (core → Go → vectors →
    docs) starts here, and core changes go first and alone.
- **Rejected:** Conan recipe checksum (`[audit]`, 3 files) — independent and short, so it can go in
    any later iteration; starting the 4-step bits chain first shortens the path to release. The
    conformance-table refactor and the specs/docs audit issue likewise wait.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs`, `crates/iscc-lib/src/lib.rs`,
    `crates/iscc-lib/src/streaming.rs` (doc comments and tests only), `docs/rust-api.md`;
    `.crap-baseline.json` only if the CRAP gate requires it (generated, not counted)
- **Reference**: `reference/iscc-core/iscc_core/codec.py` (`decode_length` → `LN(...)` raises for
    288), `reference/iscc-core/iscc_core/iscc_code.py:73-80`,
    `reference/iscc-core/iscc_core/code_content_mixed.py:83-89`

## Not In Scope

- **No signature change to any `codec` symbol.** `codec::decode_length` stays
    `fn(MainType, u32, SubType) -> u32` (Tier 2 is under the SemVer contract; a signature change
    would be an `**API-BREAK:**` stop). Add a checked crate-internal helper instead (see notes).
- The Go port (`packages/go`), `data-edge.json` rows, and binding docs / how-to pages: later steps.
- ID length validation (iscc-core's `LN` has no 88; Rust accepts it) — not part of the decision.
- Non-wide `gen_iscc_code_v0` reading a >256-bit unit: unchanged, it matches iscc-core (which calls
    `decode_length` only in the wide check).
- Do not refresh `.iai-baseline.json` (stale `bench_instance_code` rows are a separate `low` issue).

## Implementation Notes

**Encode side.** `codec::encode_length` for Meta/Semantic/Content/Data/Instance/Flake accepts only
`32..=256` in steps of 32; error text along the lines of
`invalid length {length} for {mtype:?} (must be a multiple of 32 from 32 to 256)`. Every gen
function that ends in `codec::encode_component` (meta, text, audio, mixed, data, instance, sum),
`DataHasher`/`InstanceHasher`/`SumHasher::finalize` and Tier 1 `encode_component` then reject
`bits=288`. Image and video already reject (`soft_hash_image_v0` checks `bits > 256`).

**Decode side.** Add one `pub(crate)` helper in `codec.rs` (name it e.g. `unit_bit_length`) that
returns `IsccResult<u32>`: it calls `decode_length` and rejects a Meta..Flake length index above 7
(over 256 bits), mirroring iscc-core's `LN(288)` ValueError. ISCC (up to 320) and ID keep their
current results. Switch **every** internal `decode_length` call to it (pattern rule — all four):
`codec.rs` `iscc_decompose` (~:662), `lib.rs` `iscc_decode` (~:301), `gen_mixed_code_v0` (~:834,
before the "too short" comparison, as in the reference), and the wide check in `gen_iscc_code_v0`
(~:1003; keep the short-circuit order so only a wide request with Data+Instance reaches it). Note on
`decode_length`'s doc comment that it does not validate the range and point to the rule.

**Docs.** In `docs/rust-api.md`, every `bits` row that says "multiple of 32" and the
`encode_component` `bit_length` row state "multiple of 32, 32 to 256". Update the `# Errors` text of
the touched core doc comments (`encode_component`, `iscc_decode`, the gen functions that have an
`# Errors` section, the hashers' `finalize`). `tests/test_rust_api_signatures.py` must stay green.

**Tests** (in-crate test modules; names containing `above_256` so one filter runs them): for each of
meta, text, audio, mixed, data, instance and sum `bits=288` is an error and `bits=256` still
succeeds; `DataHasher`, `InstanceHasher` and `SumHasher` `finalize(288)` error; Tier 1
`encode_component(0, 0, 0, 512, &[0; 64])` and `(…, 288, &[0; 36])` error; a hand-built 288-bit Meta
unit (`codec::encode_header(Meta, None, V0, 8)` + 36 body bytes → `encode_base32`) fails
`iscc_decode` and `iscc_decompose`; `gen_mixed_code_v0` with a hand-built 288-bit Content-Code
fails; `gen_iscc_code_v0(.., wide=true)` with a 288-bit Data unit fails. One bad case per call.
Update `test_encode_length_invalid` (288, 512 rejected) and the helper's own boundary test (7 ok, 8
error).

**Gates this step trips.** `encode_length` is on every gen hot path: run `mise run bench:iai:check`
(valgrind + runner are installed) and report the numbers. A new `?`/branch in a covered function can
fail CI's CRAP `--fail-regression` while green locally: run the CI lines below; only if they fail,
refresh with `mise run crap:baseline` and commit `.crap-baseline.json` in this step.

## Verification

- `cargo test -p iscc-lib` and `cargo test -p iscc-lib --no-default-features` pass, and
    `cargo test -p iscc-lib above_256` runs the new tests (non-zero count, 0 failed)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (no binding edit needed)
- `git grep -n "pub fn decode_length(mtype: MainType, length: u32, stype: SubType) -> u32" crates/iscc-lib/src/codec.rs`
    prints one line, and `git grep -n "decode_length(" crates/iscc-lib/src/lib.rs` shows no call
    outside test code
- `mise run coverage && cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0
- `mise run bench:iai:check` exits 0
- `uv run pytest tests/test_rust_api_signatures.py` passes and `mise run check` exits 0

## Done When

All six verification criteria pass with the core rejecting a >256-bit unit on encode and decode, and
no `codec` signature changed.
