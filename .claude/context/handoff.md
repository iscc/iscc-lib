# Handoff

## 2026-07-28 — Review of: Reject truncated varnibble header fields in `decode_header`

**Verdict:** PASS

**Summary:** The critical `decode_header` truncation bug is fixed exactly as scoped — the three
`as u8` narrowing casts are replaced with range-checked `u8::try_from` before the enum `TryFrom`, so
a multi-nibble varnibble that overflows a `u8` is rejected instead of wrapping. One-file core change
(codec.rs) plus one regression test; no signature change, no API break. All gates green.

**Verification:**

- [x] `cargo test -p iscc-lib` — 299 passed (298 + new), 0 failed
- [x] `iscc_decode`/`iscc_decompose("MDFZAAAAAAAAAAAAAA")` both `Err` — probed: rejected with
    `invalid Version: 257` (right reason — version overflows `u8`)
- [x] `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` still `(6, 0, 1, 0, <8 bytes>)` — probed: mtype=6,
    ver=1, decodes clean
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` — clean (proc-macro-error2 note is a
    transitive dep)
- [x] `cargo fmt -p iscc-lib --check` — clean
- [x] `mise run check` — all prek hooks pass
- [x] (probe, +budget) full feature matrix compiles/tests: `--no-default-features` and
    `+text-processing` both green — no repeat of 174's feature-gated compile break

**Issues found:**

- (none) — clean, precise fix. Deleted the resolved critical `decode_header` issue from issues.md.

**Codex review:** Clean. "The checked conversions correctly prevent malformed varnibble values from
wrapping into valid enum discriminants without affecting valid headers. The full iscc-lib test suite
passes." No actionable findings.

**Next:** The critical prerequisite is now green, so the batch is pushable. Next step is Part 2 of
issue #43 — `gen_iscc_id_v1` minting + `IsccIdResult` and the Go `DecodeIsccID`/`IsccIDv1Result`
deletion + the repo-wide Tier-1 count 32→33 doc sweep — per `specs/rust-core.md` → "ISCC-IDv1
Operations (Experimental)". A v0.6.0 release blocker.

**Notes:**

- The whole develop batch back to `2c4e487` (iters 172–175) is unpushed; this review push sends it
    all to `origin/develop` as one unit, where CI verifies it for the first time.
- Batch gate-integrity scanned across `@{upstream}..HEAD`: no suppressions; the only CI change is a
    strengthening `blake3 wasm32_simd` assertion (iter 173, already reviewed).
