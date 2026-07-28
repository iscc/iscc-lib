# Next Work Package

## Step: Rust core codec accepts ISCC-IDv1 (Version 1) for MainType `Id`

## Goal

Make the shipped generic `iscc_decode` / `iscc_decompose` accept MainType `Id` with Version 1
instead of raising `invalid Version: 1`, restoring drop-in parity with `iscc_core`. This is Part 1
of issue #43 (ISCC-IDv1) — the load-bearing, symbol-free half. Because every native binding (py,
napi, wasm, ffi, jni, rb, uniffi→swift/kotlin, dotnet) calls the core codec, this single-file core
change flips 10 of 11 surfaces at once; Go already did its own accept out-of-loop.

## Alternatives Considered

- **Chosen:** Part 1 codec change — the spec says it "can land as its own work package ahead of any
    new function," it is the prerequisite for `gen_iscc_id_v1`, and it is a small attributable diff
    on top of the unpushed/CI-unverified HEAD.
- **Rejected:** Part 2 `gen_iscc_id_v1` minting — depends on `encode_component`/`encode_header`
    accepting Version 1 (i.e. on this step), and fans out a new symbol across 11 surfaces; too large
    and out of order.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` — add `#[non_exhaustive]` and a `V1 = 1` variant to the
    `Version` enum in the same change; make `decode_header`, `encode_header`, `encode_component`
    accept Version 1 **only** when MainType is `Id`, and keep rejecting Version > 0 for every other
    MainType; add/update the in-file `#[cfg(test)]` cases (the existing `Version::try_from(1)`
    assertion at ~line 1045 will need to change to match the chosen design).
- **Reference**: `.claude/context/specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)" (Part
    1 + "Codec changes this requires"); `crates/iscc-lib/src/lib.rs:264` (`iscc_decode` returns
    `(u8,u8,u8,u8,Vec<u8>)`); issue #43 in `issues.md`.

## Not In Scope

- `gen_iscc_id_v1` minting or `IsccIdResult` — that is Part 2, a separate step.
- Any binding crate, `packages/go` rename, docs, or the Python differential test.
- Adding realm variants to `SubType` (spec forbids it — realm travels as a raw nibble).
- The separate codec input-cleaning (`iscc_clean`) `normal` issue.
- Acquiring any time/clock dependency.

## Implementation Notes

- The version gate is MainType-aware, so it cannot live purely in the context-free `TryFrom<u8>`:
    decode `mtype` first (already done at `decode_header:296`), then validate the version against
    it. Pick the simplest design (e.g. a small `decode_version(mtype, val)` helper, or accept `V1`
    in `TryFrom` and reject non-`Id` V1 in the three functions) — document whichever you choose.
- Realm 0 header = `0x6010`, realm 1 = `0x6110`; only the nibble value is written/read, so both
    realms round-trip and match the reference numerically. Do not special-case realm.
- Non-`Id` MainType with Version 1 must still error with the existing `invalid Version:`-style
    message.
- **API-BREAK:** adding `#[non_exhaustive]` to `Version` is itself a SemVer-major
    (`cargo-semver-checks` would report `enum_marked_non_exhaustive`), taken deliberately in the 0.x
    window. Record this as an `**API-BREAK:**` note in the advance handoff — `cargo-semver-checks`
    is not installed locally and the CI `semver` job is `continue-on-error`, so it is a written
    record, not a gate result.

## Verification

- `cargo test -p iscc-lib` passes, including a new case asserting
    `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` returns `(6, 0, 1, 0, <8-byte body>)` and the same via
    the no-prefix form.
- A new `#[cfg(test)]` case asserts `decode_header`/`iscc_decode` reject Version 1 for a non-`Id`
    MainType (e.g. a Meta header with version nibble 1 errors).
- `iscc_decompose("ISCC:MAIGHFECJMOPMIAB")` succeeds (no `invalid Version: 1`).
- `grep -n '#\[non_exhaustive\]' crates/iscc-lib/src/codec.rs` shows the attribute on
    `enum Version`, and the enum has a `V1 = 1` variant.
- `cargo clippy -p iscc-lib -- -D warnings` clean.
- `cargo fmt -p iscc-lib --check` clean.

## Done When

`cargo test -p iscc-lib` (with the new decode cases), clippy, and fmt all pass, and the core codec
accepts MainType `Id` Version 1 through `iscc_decode`/`iscc_decompose` while still rejecting Version
\> 0 for every other MainType.
