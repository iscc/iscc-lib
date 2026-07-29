## 2026-07-29 — Review of: Route a shared `iscc_clean` helper through the four Rust codec-input sites

**Verdict:** NEEDS_WORK

**Summary:** The `iscc_clean` port is faithful to the reference and correctly fixes the three
documented divergences (hyphen-grouped, whitespace-padded, lowercase-scheme forms all parse
identically to the plain form; multibase dashes preserved). But it introduced a correctness
regression in the Tier-1 public API `iscc_decompose`: inputs that clean to `""` now silently return
`Ok([])` instead of erroring — Codex caught it, and the advance's verification claim about it was
false. Fix is small; landing it makes this a clean PASS.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 309 + 6 (codec_clean) + 28 + 22 + 4 + 1 doctest, 0 failed.
- [x] Differential tests present: `codec_clean.rs` covers decompose/decode/gen accept hyphenated +
    padded + lowercase-scheme, output identical to plain. Confirmed the 3 exact issue examples
    resolve (hyphenated composite → 4 units; padded + lowercase single → 1 unit each).
- [x] Multibase dash-preservation `#[test]` present in codec.rs (`u`-prefixed keeps dashes).
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt --check` clean.
- [x] `mise run check` (prek all-files) → all hooks Passed.
- [x] CRAP baseline regen legitimate — `.cargo-crap.toml` untouched (threshold 30.0), baseline only
    records the 3 regressed + 2 new function crap values (all ≤ 21.1); no epsilon/threshold
    widening.
- [ ] **Empty-input handling** — REGRESSION, see below.

**Issues found:**

- **[regression, must fix]** `iscc_decompose` now returns `Ok([])` for `"   "`, `"-"`, `"iscc:"`,
    `"----"` — all of which errored at HEAD~1 and error in the reference `iscc_decompose`.
    `iscc_clean` returns `""` for these, `decode_base32("")` returns `Ok(empty)` (NOT an error, as
    the advance handoff wrongly claimed), the while-loop is skipped → `Ok([])`. Silent acceptance of
    malformed input in a stability-committed Tier-1 API, and a fresh divergence from the very
    reference this step exists to match. Fix: guard for an empty cleaned code (raise `InvalidInput`,
    mirroring the reference's `code[0]` IndexError) — either inside `iscc_clean` or before the loop
    in `iscc_decompose` — and add a test asserting the four forms above error. (`iscc_normalize` /
    `gen_*_v0` look protected: empty flows into `decode_header([])` / the `< 2` count check, which
    already error — confirm with a test.)

**Codex review:** [P2] "Reject inputs that clean to an empty code" (codec.rs:560-561) — verified
genuine and reproduced against HEAD~1. This is the NEEDS_WORK item above. No other findings.

**Next:** Add the empty-cleaned-code guard + tests to close the regression; then this Rust half is
clean. After that, the Go half of the codec `iscc_clean` divergence (issues.md — non-mechanical port
of the four `packages/go` sites, must NOT copy the empty-input gap) closes the issue. The Ruby
`gen_iscc_id_v1` validation-order fix remains the last `normal` codec-adjacent item.

**Notes:**

- Error messages mirror Python exactly: `"Invalid scheme: {scheme}"` (single space; next.md's double
    space was a typo — advance used the correct single space) and `"Malformed ISCC string: {iscc}"`.
- `iscc_decompose("")` already returned `Ok([])` at HEAD~1 (pre-existing), so that one input is not
    a regression — but the whitespace/dash/lowercase-scheme forms are new.
- Do not re-derive the codec-cleaning design: it matches the reference and is settled; only the
    empty-input edge needs closing.
