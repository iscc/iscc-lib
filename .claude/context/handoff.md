# Handoff

## 2026-07-28 — Review of: Rust core codec accepts ISCC-IDv1 (Version 1) for MainType `Id`

**Verdict:** NEEDS_WORK

**Summary:** The core change is correct for valid inputs — `iscc_decode`/`iscc_decompose` now accept
MainType `Id` Version 1, byte-matching the `iscc_core` oracle, with 7 well-scoped codec tests and a
clean `#[non_exhaustive] Version { V0, V1 }` + `validate_version` gate. But the change makes a
latent `decode_header` truncation bug newly reachable: malformed multi-nibble headers now silently
canonicalize to valid ISCCs. Not pushed — a codec.rs input-validation fix is owed first.

**Verification:**

- [x] `cargo test -p iscc-lib` — **298 passed**, incl. all 7 new codec cases + updated
    `test_version_try_from`.
- [x] `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` → `(6, 0, 1, 0, <8 bytes>)` — matches independent
    `iscc_core` 1.3.0 oracle `(6, 0, 1, 0, b'c\x94\x82K\x1c\xf6 \x01')`, both prefixed and bare.
- [x] Non-`Id` Version-1 rejection — `encode_header`/`decode_header`/`iscc_decode` all error
    (`invalid Version`) for a Meta header with version nibble 1.
- [x] `iscc_decompose("ISCC:MAIGHFECJMOPMIAB")` → `["MAIGHFECJMOPMIAB"]`, no `invalid Version: 1`.
- [x] `grep '#\[non_exhaustive\]'` present on `enum Version`; `V1 = 1` variant present.
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean
    (default/no-default/all-features).
- [x] `cargo fmt -p iscc-lib --check` clean.
- [x] `mise run check` — all 18 prek hooks Passed.
- [ ] **Robustness probe (beyond next.md):** `iscc_decode("MDFZAAAAAAAAAAAAAA")` returns
    `(6, 0, 1, 0, <8 bytes>)` and `iscc_decompose` rewrites it to `["MAIAAAAAAAAAAAAA"]` — a
    malformed header silently accepted. `iscc_core` rejects the string. **Fails.**

**Issues found:**

- **[critical, filed]** `decode_header` narrows `mtype_val`/`stype_val`/`version_val` with `as u8`
    before `TryFrom`, so a multi-nibble value wraps (version `257`→`1`, MainType `262`→`6`=`Id`) and
    a malformed input canonicalizes to a valid ISCC. Root cause pre-existing (`V0` wrap-to-0), but
    adding `Version::V1` made the v1-wrapping class newly reachable — on HEAD~1 these errored with
    `invalid Version: 1`. Fix: `u8::try_from(..)?` in `decode_header` + a rejection test. See
    issues.md. Tightening decode validation is blessed (decisions.md 2026-07-24), so it's a fix.
- **[fixed by review]** Pre-existing CI break in the unpushed out-of-loop commit `2c4e487`:
    `test_iscc_decode_rejects_uncomposable_sequence` (lib.rs) calls feature-gated
    `gen_meta_code_v0`/`gen_text_code_v0` but the commit added the
    `cargo test --no-default-features` (+ `--features text-processing`) CI steps, so both fail to
    **compile**. Gated the test `#[cfg(feature = "meta-code")]` (behaviour-neutral); full feature
    matrix now green.

**Codex review:** Independent Codex flagged exactly the truncation defect above (P2, codec.rs:323) —
confirmed live and filed as critical. No other findings.

**API-BREAK (recorded, accepted):** `#[non_exhaustive]` + `V1` on `codec::Version` is a SemVer-major
in the 0.x window, per next.md and specs/rust-core.md. `cargo-semver-checks` not installed locally;
CI `semver` job is `continue-on-error` — written record, not a gate result. Correct handling.

**Next:** Fix the `decode_header` truncation (issues.md, critical) — replace the three `as u8` casts
with range-checked `u8::try_from`, add a `#[cfg(test)]` case asserting
`iscc_decode`/`iscc_decompose` reject `"MDFZAAAAAAAAAAAAAA"`. Scope stays inside codec.rs. After
that lands green, the Part 2 `gen_iscc_id_v1` minting + `IsccIdResult` step (issue #43) has its
prerequisite.

**Notes:**

- The batch is NOT pushed (NEEDS_WORK). It still contains the large out-of-loop commit `2c4e487`
    (`iscc_decode` sequence normalization, +CI feature-matrix steps, specs/target), CI-unverified.
    My compile-break fix is committed but unpushed with it; the next cycle pushes the full batch
    once the truncation fix is green.
- `validate_version` design (helper, not folded into `TryFrom`) is sound — `TryFrom` is context-free
    and can't see MainType. Realm travels as the raw `SubType` nibble (realm 0→`None`, realm 1→
    `Image`), round-trips numerically, not special-cased — correct per spec.
