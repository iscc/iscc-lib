# Handoff

## 2026-07-28 — Rust core codec accepts ISCC-IDv1 (Version 1) for MainType `Id`

**Done:** Added `#[non_exhaustive]` + a `V1 = 1` variant to `codec::Version`, and a MainType-aware
`validate_version` gate so `decode_header`/`encode_header` (and `encode_component` transitively)
accept Version 1 **only** for MainType `Id`, still rejecting Version > 0 for every other MainType.
`iscc_decode`/`iscc_decompose` now handle `ISCC:MAIGHFECJMOPMIAB` instead of raising
`invalid Version: 1`. Part 1 of issue #43; no new Tier 1 symbols.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: `Version` enum gains `#[non_exhaustive]` + `V1 = 1`; `TryFrom<u8>`
    maps `1 → V1`; new private `validate_version(mtype, version)` helper; called in `encode_header`
    (top) and `decode_header` (after enum decode). Updated `test_version_try_from`; added 7 tests
    (IDv1 realm-0 decode with/without prefix, decompose, header decode, realm-1 roundtrip, and three
    non-`Id` Version-1 rejection cases through `encode_header`/`decode_header`/`iscc_decode`).

**Verification:**

- `cargo test -p iscc-lib` → **298 passed, 0 failed** (incl. all 7 new codec cases). The pinned
    vector `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` returns the tuple
    `(6, 0, 1, 0, <8-byte body 0x6394824b1cf62001>)` — byte-identical to the reference — both
    prefixed and bare.
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` → clean (only the pre-existing
    proc-macro-error2 future-incompat dependency note).
- `cargo fmt -p iscc-lib --check` → clean (exit 0).
- `mise run check` → all 18 prek hooks Passed.
- `grep -n '#\[non_exhaustive\]' codec.rs` → present on `enum Version`; enum has `V1 = 1`.

**API-BREAK:** Adding `#[non_exhaustive]` to `codec::Version` is itself a SemVer-major
(`cargo-semver-checks` would report `enum_marked_non_exhaustive`), taken deliberately in the 0.x
window per next.md and specs/rust-core.md "Codec changes this requires". `cargo-semver-checks` is
not installed locally and the CI `semver` job is `continue-on-error`, so this is a written record,
not a gate result. Adding the `V1` variant is also additive-major but subsumed by the same window.

**Next:** Part 2 — `gen_iscc_id_v1(timestamp, hub_id, realm)` minting + `IsccIdResult`, which now
has its prerequisite (encode path accepts Version 1 for `Id`). That step fans a new symbol across 11
surfaces and is out of order/scope for a single package.

**Notes:**

- Design chosen: a single `validate_version` helper rather than folding the gate into `TryFrom<u8>`
    (which is context-free and cannot see the MainType). `TryFrom` maps the raw nibble; the helper
    rejects non-`Id` V1. Realm is NOT special-cased — it travels as the raw `SubType` nibble, so
    realm 0 decodes to `SubType::None` and realm 1 to `SubType::Image` (cosmetic), both round-trip
    numerically, per spec. `#[non_exhaustive]` on an in-crate enum does not force wildcard match
    arms inside the crate, so existing exhaustive matches on `Version` still compile.
- **OUT OF SCOPE — pre-existing CI break for review:**
    `cargo test -p iscc-lib --no-default-features` (a CI `rust`-job step, ci.yml:42) fails to
    **compile**: `lib.rs:2072` (`test_iscc_decode_rejects_uncomposable_sequence`) calls
    `gen_meta_code_v0`/`gen_text_code_v0` without a
    `#[cfg(feature = "meta-code")]`/`text-processing` gate. Verified this fails identically on
    pristine HEAD `codec.rs` (temporarily swapped it in, still failed, restored mine), so it is
    independent of this change — a latent break in the unpushed HEAD range. Not fixed (lib.rs is
    outside this package's codec.rs scope). Default-feature `cargo test`, clippy, fmt, and
    `--all-features` build all pass.
