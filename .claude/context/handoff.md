# Handoff

## 2026-07-28 — Review of: Re-baseline the CRAP gate for the IDv1 batch complexity increase

**Verdict:** PASS

**Summary:** `.crap-baseline.json` regenerated to absorb the covered-complexity growth from the IDv1
batch (iters 172–175). The enforcing CI CRAP gate now exits 0, codec.rs untouched, all tests green.
A clean one-file generated-artifact refresh — exactly the gate's documented remedy.

**Verification:**

- [x] `mise run coverage` +
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json   --fail-regression --fail-above` —
    **EXIT 0**: `0 regressed, 0 new, 0 moved, 107 unchanged`.
- [x] `git diff --name-only HEAD~1..HEAD` = `.crap-baseline.json` + `handoff.md` + advance memory —
    **codec.rs untouched** (no refactor; `git diff … -- codec.rs` empty).
- [x] `decode_header` baseline entry cyclomatic 16.0 / coverage 100 / crap 16.0; matches current
    source (gate reports 0 moved, so source == baseline). 16 < absolute `--fail-above` 30.
- [x] No function > 30: max is `gen_meta_code_v0` at 22.27 (107 entries checked).
- [x] `cargo test -p iscc-lib` — 299 + 28 + 22 + 4 + 1 passed, 0 failed.
- [x] `mise run check` — all hooks Passed.

**On the flagged deviation:** next.md's Implementation-Note-2 premise ("only `decode_header` moved;
stop if any other function moved") was wrong, and the advance agent was right to proceed. Verified
independently: the diff is against `a7e84c8` (the *actual* last baseline refresh — Final_Sigma,
~iter 170), NOT iter 121 as the advance handoff stated. That historical detail is inaccurate but
immaterial — every moved function traces to the reviewed IDv1 batch (172–175): `decode_header`
12→16, `Version::try_from` 3→4, `encode_header` 6→7, `validate_version`/`iscc_normalize` new,
`iscc_decode`/`iscc_decompose` coverage improved. Determinism confirmed two ways: gate `0 moved`,
and Codex reports the baseline byte-identical to a fresh coverage-derived report. Baking them in is
the correct remedy.

**Issues found:** (none material). Minor: advance handoff/memory misattribute the last baseline
refresh to iter 121 — it was a7e84c8. No action needed; deltas are correct regardless.

**Codex review:** No actionable regressions. "Regenerated baseline is byte-identical to a fresh
coverage-derived report; the CRAP regression and absolute-threshold gate passes with all 107
functions unchanged."

**Next:** develop CI should go green on this push (the informational `semver` red from the
intentional `#[non_exhaustive]` on `Version` aside — expected pre-1.0). Then resume issue #43 Part
2: `gen_iscc_id_v1` minting + Go `DecodeIsccID`/`IsccIDv1Result` deletion + the Tier-1 32→33
symbol-count doc sweep — the remaining v0.6.0 blocker (spec: `specs/rust-core.md` → "ISCC-IDv1
Operations").

**Notes:** `lcov.info` is gitignored (not staged). The cargo-crap warning about
`tests/test_unicode_boundary.rs` having no LCOV entry is pre-existing and benign (test-only file).
