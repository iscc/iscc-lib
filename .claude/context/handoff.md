## 2026-07-29 — Review of: Reject empty-cleaned codec input to close the `iscc_decompose` regression

**Verdict:** PASS

**Summary:** The advance agent added an empty-cleaned-input guard inside `codec::iscc_clean` that
returns `Err(InvalidInput("Empty ISCC string"))` when the cleaned result is `""`, protecting all
four call sites at once. Clean, minimal, matches the reference contract, and closes the HEAD
regression. The in-flight Rust `iscc_clean` routing (iters 191 + 192) is now a green PASS and pushes
under CI.

**Verification:**

- [x] `cargo test -p iscc-lib` — all pass, 0 failed (codec_clean 10/10 incl. 4 new empty-input
    tests)
- [x] New tests assert `Err(InvalidInput)` for `"   "`/`"-"`/`"iscc:"`/`"----"` across
    `iscc_decompose`, `iscc_decode` (→ private `iscc_normalize`), `gen_iscc_code_v0`,
    `gen_mixed_code_v0`
- [x] Valid-input differential cases still green (hyphenated→4 units, padded/lowercase-scheme→1;
    multibase `u`-prefixed dashes preserved)
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt --check` clean
- [x] `mise run check` (prek all-files) — all hooks Passed; `.crap-baseline.json` regenerated
    (`iscc_clean` cyclo 6→7, 100% covered); `.cargo-crap.toml` untouched (threshold 30.0)
- [x] **Probe (extra):** reference `iscc_decode` IndexErrors on all four forms + `""` — guard now
    matches parity exactly (ref `iscc_clean` errors on `"   "` directly, on the others downstream)

**Issues found:** (none) — diff touches only the 2 in-scope source/test files + the two generated
baselines; no gate weakening in the full `origin/develop..HEAD` unpushed range; no API break
(`iscc_clean` is `pub(crate)`; tightening decode validation is settled non-breaking, decision
2026-07-24).

**Codex review:** Clean — "correctly rejects inputs whose cleaned representation is empty without
changing valid-input behavior; full suite and clippy pass." No actionable findings.

**Next:** The Go `iscc_clean` port (issues.md, `normal`) is now fully unblocked — the Rust half is
complete. It MUST replicate this empty-cleaned guard (reject a cleaned `""` before decode) so
`isccDecompose` does not inherit the `Ok([])` gap. The Ruby `gen_iscc_id_v1` validation-order fix
(`normal`) and the ASCII-only iai benchmark gap (`normal`) remain the other CID-doable items.

**Notes:**

- Error message is a fixed `"Empty ISCC string"` (does not mirror Python's IndexError text — only
    that the input errors matters, per next.md).
- `iscc_decompose("")` (truly empty) was `Ok([])` at HEAD~1 too; the guard now rejects it,
    tightening toward the reference — consistent, not required by the step.
- learnings.md trimmed 202→199 (empty-input entry updated to note the 192 guard; two historical
    entries compressed). No new decision needed — routine fix under a settled decision.
