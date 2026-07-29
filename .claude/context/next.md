# Next Work Package

## Step: Route a shared `iscc_clean` helper through the four Rust codec-input sites

## Goal

Close the Rust half of the "Codec input cleaning diverges from iscc-core `iscc_clean`" `normal`
`[review]` issue: hyphen-separated (the canonical display form), whitespace-padded, and
case-insensitive-`iscc:`-scheme inputs must parse instead of raising, matching the reference. This
is the most substantive open v0.6.0 release blocker and affects real user input.

## Alternatives Considered

- **Chosen:** codec `iscc_clean` (Rust half) — real drop-in-compat bug; the hyphen-grouped form
    (`ISCC:KACY-PXW4-…`) is how ISCCs are displayed, so users hit it constantly.
- **Rejected:** Ruby `gen_iscc_id_v1` validation-order fix — the issue itself states practical
    impact is nil (only `> i64::MAX` inputs, ~292K-year timestamps); lower user value than codec.
- **Rejected here:** the Go half of this same issue — different language, non-mechanical port; it is
    a separate step (see Not In Scope), so this step does not yet close the issue.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` (add private `iscc_clean` helper; route
    `iscc_decompose` at ~L520), `crates/iscc-lib/src/lib.rs` (route `iscc_normalize` ~L222,
    `gen_mixed_code_v0` ~L810, `gen_iscc_code_v0` ~L894)
- **Create**: `crates/iscc-lib/tests/codec_clean.rs` (differential tests) — outside the 3-file
    budget
- **Reference**: `reference/iscc-core/iscc_core/codec.py:644` (`iscc_clean`) and `:363`
    (`normalize_multiformat` calls it); issue "Codec input cleaning diverges…" in issues.md

## Not In Scope

- The Go port (`packages/go` codec.go / code_content_mixed.go / code_iscc.go / isccNormalize) — a
    separate follow-up step; the issue stays open until both surfaces are fixed.
- Multiformat (multibase-prefixed `f`/`b`/`v`/`z`/`u`) decode support — a documented non-goal
    (`lib.rs:216-219`); `iscc_clean` must NOT corrupt such inputs but decode may still reject them.
- Changing `iscc_normalize`'s compose logic or any `gen_*_v0` output for already-valid inputs.

## Implementation Notes

Port `iscc_clean(iscc) -> IsccResult<String>` faithfully (returns cleaned code, no `ISCC:` prefix,
no dashes):

1. `let s = iscc.trim();` then split on `':'`, trimming each part (mirror `part.strip()`).
2. **1 part:** strip dashes **only if** the first char is NOT a multibase prefix (`f`,`b`,`v`,`z`,
    `u`) — an unconditional `.replace('-', "")` would corrupt multibase input. Return as-is
    otherwise.
3. **2 parts:** scheme must equal `"iscc"` **case-insensitively**, else
    `InvalidInput("Invalid  scheme: …")`; return the code with dashes stripped.
4. **>2 parts:** `InvalidInput("Malformed ISCC string: …")`.

Replace the four ad-hoc `strip_prefix("ISCC:")…replace('-', "")` snippets with a call to the helper.
`gen_iscc_code_v0` holds `cleaned: Vec<&str>` (borrows) — switch to `Vec<String>` and adjust the
length/borrow checks. In `iscc_normalize`, the wide-mode header read must use the cleaned string.
`decode_base32` already uppercases, so lowercase-body parity needs no extra work.

CRAP gate is CI-only and enforcing: adding branches to covered fns can red it even when green
locally — if `mise run coverage` shows regressions, regenerate `.crap-baseline.json` via
`mise run crap:baseline` in this same step (never widen the epsilon).

## Verification

- `cargo test -p iscc-lib` passes (existing suite + new `codec_clean.rs`).
- New differential tests: for a valid multi-unit ISCC `s` from the existing tests,
    `iscc_decompose(hyphenate(s)) == iscc_decompose(s)`,
    `iscc_decode("  iscc:{s}  ")? ==   iscc_decode(s)?`, and `gen_iscc_code_v0`/`gen_mixed_code_v0`
    accept the hyphenated + padded + lowercase-scheme forms with output identical to the plain form.
- A `#[test]` asserts `iscc_clean` leaves dashes intact for a multibase-prefixed input (starts with
    `u`).
- `cargo clippy -p iscc-lib -- -D warnings` clean; `cargo fmt -p iscc-lib --check` clean.

## Done When

The four Rust codec-input sites route through one `iscc_clean` helper, the dash/whitespace/
lowercase-scheme forms parse with output identical to the plain form, and all checks above pass.
