# Next Work Package

## Step: ISCC-SEQ codec in the Rust core (`encode_seq` / `decode_seq` / `read_unit`)

## Goal

Port the ISCC-SEQ codec from iscc-core 1.5.0 (IEP-0020, GitHub #45) into `iscc-lib`, with Tier 1
`encode_seq` / `decode_seq` and Tier 2 `codec::read_unit` / `codec::SEQ_MAINTYPES`, plus the
`test_seq.py` port and the seq conformance vectors. This is the first and foundational v0.8.0 slice;
every binding wraps it.

## Alternatives Considered

- **Chosen:** Rust core ISCC-SEQ — it is the v0.8.0 milestone and blocks all 11 binding slices;
    handoff and spec both order it first.
- **Rejected:** Go `isccClean` port (`normal` issue) — a prerequisite only for the Go seq slice,
    which comes later. The other small `normal` issues (wasm npm metadata, Ruby `json`/locale,
    c-ffi-api type names) are independent and can fill gaps between binding slices.

## Scope

- **Create**: `crates/iscc-lib/tests/test_seq.rs`
- **Modify**: `crates/iscc-lib/src/codec.rs`, `crates/iscc-lib/src/lib.rs` (crate-root re-export of
    `encode_seq`, `decode_seq`), `crates/iscc-lib/src/conformance.rs` (run the seq sections)
- **Generated (if the gates need it)**: `.crap-baseline.json`
- **Reference**: `.claude/context/specs/rust-core.md` → "encode_seq / decode_seq (ISCC-SEQ,
    IEP-0020)" (the normative spec, read it in full);
    `reference/iscc-core/iscc_core/codec.py:432-540`; `reference/iscc-core/tests/test_seq.py`;
    `reference/iscc-core/iscc_core/conformance.py:56-110`

## Not In Scope

- The Tier 1 33 → 35 count and docs sweep (crate `CLAUDE.md`, `notes/`, `docs/`). It waits until all
    surfaces bind seq, the same way the IDv1 sweep came after its fan-out (iter 190)
- Any binding crate, `iscc.h`, `NativeMethods.g.cs`, or the pure-Go port
- Widening `SubType` / `Version` or loosening `validate_version`; changing what `iscc_decode`,
    `iscc_decompose` or `encode_component` accept
- Raising `.iai-baseline.json`

## Implementation Notes

- **Raw header codec (spec "porting trap"):** factor out private raw-integer helpers. One decodes
    `(mt, st, vs, ln: u32, tail_start: usize)` from `&[u8]` with the 4-bit zero-padding strip, and
    one encodes four `u32` values to padded bytes. Keep values `u32` with no narrowing (SubType can
    be up to 4679). `decode_header` / `encode_header` become thin typed wrappers. The
    `u8::try_from`, `TryFrom` and `validate_version` gates stay in the wrappers only.
- **No added allocation on the hot path:** the raw decode returns the tail start, and only the typed
    `decode_header` builds the tail `Vec` (as it does today). `decode_header` sits under the iai
    `four_units` / decompose benches.
- **`read_unit(data, offset)`:** `offset > data.len()` must return `Err`, never panic (a Rust slice
    panics where Python is lenient, and release builds use `panic = "abort"`). Window =
    `data[offset..min(offset+8, len)]`. Canonicality = re-encode the raw fields and check that the
    window `starts_with` them. Then check MainType ∈ `SEQ_MAINTYPES`, then `ln <= 7`, then
    `end = offset + header_len + 4*(ln+1) <= data.len()`. Use the reference's check order.
- **`SEQ_MAINTYPES`:** a `pub const` slice/array of `MainType` (Meta, Semantic, Content, Data,
    Instance). Membership is tested against the raw `u32` mt, so an unknown MainType ≥ 8 is rejected
    as "not permitted", not with a panic.
- **`encode_seq(units: &[&str])`:** empty list → Err; per unit: `trim().is_empty()` → Err;
    `to_uppercase()` **before** `iscc_clean` (multibase trap, spec); `decode_base32`;
    `read_unit(&raw, 0)? == raw.len()` or Err (trailing data / several units). Concatenate in input
    order.
- **`decode_seq(data: &[u8])`:** empty → Err; loop `read_unit`; push
    `"ISCC:" + encode_base32(&data[off..end])`. Never go through `encode_component`.
- Rustdoc on all four items. Align the `iscc_decompose` doc comment with iscc-core 1.5.0 (spec "Also
    in scope"): its input is base32 of concatenated unit bytes, and it should point to `decode_seq`
    as the strict byte-level reader.
- **Conformance:** the reference `conformance_selftest` walks every section with `bytes:<hex>`
    decoding (`conformance.py:58-62`), so add `run_encode_seq_tests` / `run_decode_seq_tests` to
    `conformance_selftest`. Inputs: encode = `[[str…]]`, decode = `["bytes:<hex>"]`. Outputs:
    `"bytes:<hex>"` and a list of strings. Hex-decode with `hex::decode`, as `decode_stream` does
    (`hex` is already a dependency).
- **Test port (`tests/test_seq.rs`):** import `iscc_lib::{encode_seq, decode_seq}` and
    `iscc_lib::codec::{read_unit, SEQ_MAINTYPES}`. Port every `test_seq.py` case except the
    Python-only ones: `*_type_error`, `*bytes_like`, `*memoryview`, `tuple_and_generator`. For
    parametrized cases, loop over the same parameters. Also add explicit tests for each IEP-0020
    vector listed in the spec's "Verified when" (2 accepts with exact outputs, 7 Table-2 rejects),
    the round trip `encode_seq(decode_seq(x)) == x` on all 4 `decode_seq` vectors, and a behaviour
    guard: `iscc_decode` and `iscc_decompose` still **reject** `ISCC:GAIAAAAAAA` (Data, Version 1)
    and a SubType ≥ 8 unit from `test_seq.py`'s unknown-subtype cases, both of which `decode_seq`
    accepts.
- `test_iep_example`: port its base32 `TEXT` assertion. For its padded-base64 round trip, use the
    existing `data_encoding::BASE64` dependency, or drop that line. Do not add a dev-dependency.
- **Gates:** this edits the codec hot path. `mise run check` skips iai, so run
    `mise run bench:iai:check` explicitly against the **unmodified** `.iai-baseline.json`. Lines in
    `codec.rs` shift, and the CRAP gate fails only in CI, so run `mise run coverage` then
    `mise run crap:baseline` in this step and commit the regenerated `.crap-baseline.json`.

## Verification

- `cargo test -p iscc-lib` passes, including `tests/test_seq.rs` and
    `test_conformance_selftest_passes`; `conformance.rs` runs the `encode_seq` and `decode_seq`
    sections
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` and `cargo fmt -p iscc-lib --check` are
    clean
- `tests/test_seq.rs` asserts `decode_seq(hex "270000000000") == ["ISCC:E4AAAAAAAA"]`,
    `decode_seq(hex "301000000000300000000000") == ["ISCC:GAIAAAAAAA", "ISCC:GAAAAAAAAA"]`,
    `encode_seq(&["faaa-aaaaaaaa"]) == hex "28000000000000"`, and an `Err` for each of the 7
    IEP-0020 Table-2 inputs
- `tests/test_seq.rs` asserts that `iscc_lib::iscc_decode("ISCC:GAIAAAAAAA")` and
    `iscc_lib::iscc_decompose("ISCC:GAIAAAAAAA")` return `Err`
- `mise run bench:iai:check` exits 0, and `git log -1 --format=%h -- .iai-baseline.json` still
    prints `ecdee58` (the baseline is not raised)
- After `mise run coverage`, the CI CRAP gate exits 0:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`

## Done When

All verification criteria pass with ISCC-SEQ exported from the `iscc_lib` crate root and `read_unit`
/ `SEQ_MAINTYPES` reachable via `iscc_lib::codec`.
