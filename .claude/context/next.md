# Next Work Package

## Step: Core `iscc_clean` trims exactly what Python `str.strip()` trims (U+001C–U+001F)

## Goal

Picks up the `normal` issue "`iscc_clean` trims fewer whitespace characters than iscc-core". The
core cleaner must strip the same character set as the reference, so that every cleaned entry point
(decode, decompose, normalize, `gen_iscc_code_v0`, `gen_mixed_code_v0`, `encode_seq`) and every
binding agrees with iscc-core. This also fixes the set the later Go `isccClean` port must copy.

## Alternatives Considered

- **Chosen:** the core trim-parity fix. It is the hub: Go's `isccClean` port and the 10 unbound seq
    surfaces all inherit the final strip set. It also unblocks the spec's Python "both sides reject
    the same inputs" criterion. One source file plus tests.
- **Rejected (it was the handoff's top pick):** Go `decodeBase32` spare-bit fix + Rust/Go
    strict-base32 tests. It does not depend on this step and loses nothing by waiting. It is the
    natural next step.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs`, `.crap-baseline.json` (only if the CRAP gate needs it,
    see notes)
- **Tests**: `crates/iscc-lib/tests/codec_clean.rs`, `tests/test_seq.py`
- **Reference**: `reference/iscc-core/iscc_core/codec.py:762` (`iscc_clean`),
    `.github/workflows/ci.yml` (CRAP gate step, ~L408)

## Not In Scope

- Go (`packages/go`): the `isccClean` port and the `decodeBase32` fix are separate issues/steps.
- Rust strict-base32 tests (`GAAAAAAAAB`, `GAAAAAAAAA======`). They belong with the Go
    `decodeBase32` step.
- **Text utilities.** `utils.rs` `text_trim` (`.trim()`, L125/L132) and `text_collapse`/
    `text_remove_newlines` (`split_whitespace`, L94/L107/L115) mirror reference `.strip()` /
    `.split()` (`code_meta.py:184,199`), so they probably have the same U+001C–U+001F divergence on
    Tier 1. Do NOT touch them: they feed Meta/Text-Code output and conformance. Advance: mention the
    probable divergence in your handoff so review can file an issue.
- Any binding crate, `lib.rs:431`, docs, or the Tier-1 count sweep.
- Regenerating `.iai-baseline.json`. **Not authorised in this step.**

## Implementation Notes

- **The divergent set (probed under CPython 3.13):** `{c : chr(c).strip() == ""}` is exactly Rust's
    `char::is_whitespace` set plus U+001C, U+001D, U+001E and U+001F. Nothing is trimmed by Rust but
    kept by Python. Add one private predicate `is_py_strip_char` that returns true when
    `c.is_whitespace()` holds or `c` is in `'\u{1c}'..='\u{1f}'`. Give it a doc comment naming
    `str.strip()`, then trim via `trim_matches(is_py_strip_char)`.
- **Replace all four trim sites in `codec.rs`:** L585 (`iscc.trim()`), L601 (`scheme.trim()`), L602
    (`rest.trim()`) and the `encode_seq` empty-unit guard at L816 (`unit.trim().is_empty()`).
    Missing the guard changes no pass/fail result (`["\x1e"]` errors either way), so the test must
    assert the guard's "must not be empty" message.
- Keep the `Cow` borrow paths. Do not add allocations; this is the iai-gated hot path (learnings:
    193).
- **Rust tests (`codec_clean.rs`):** for each of U+001C..=U+001F, `iscc_decode` (or
    `iscc_decompose`) of the separator placed (a) around the whole string, (b) before the colon, (c)
    after the colon must equal the clean `ISCC:GAAAAAAAAA` result. Also:
    - `encode_seq(&["\u{1e}"])` errors with "must not be empty";
    - a predicate test that compares it against real `char::is_whitespace` (no hand-typed table) and
        asserts that U+200B is NOT stripped.
    - Do not put character counts in assertions.
- **Python differential (`tests/test_seq.py`):** derive the strip set live
    (`[c for c in map(chr, range(0x110000)) if c.strip() == ""]`), assert the 4 separators are in
    it, and check that for every char in it both `iscc_core.encode_seq([ch + "ISCC:GAAAAAAAAA"])`
    and `iscc_lib.encode_seq(...)` return identical bytes. Add one shared-rejection case with U+200B
    (probed: Python does not strip it).
- Run `maturin develop -m crates/iscc-py/Cargo.toml` before pytest. A stale extension false-reds.
- **CRAP (CI-only gate):** after `mise run coverage`, run CI's exact gate:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`. If
    it reports a regression in `codec.rs` functions you touched or shifted, run
    `mise run crap:baseline` and commit it. Note the local rustc in your handoff (learnings: a rustc
    change re-scores untouched fns). Do not widen thresholds.

## Verification

- `cargo test -p iscc-lib` passes, including the new `codec_clean.rs` separator, guard and predicate
    tests
- `grep -nE '\.trim\(\)' crates/iscc-lib/src/codec.rs` prints nothing (the four sites above are its
    only `.trim()` calls)
- `uv run pytest tests/test_seq.py` passes after `maturin develop -m crates/iscc-py/Cargo.toml`,
    including the live-derived strip-set differential and the U+200B shared rejection
- `mise run bench:iai:check` exits 0 against the committed, unmodified `.iai-baseline.json`
- `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` exits
    0 (after `mise run coverage`)
- `mise run check` exits 0 and `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean

## Done When

All six verification criteria pass, with the text utilities, Go and the iai baseline untouched.
