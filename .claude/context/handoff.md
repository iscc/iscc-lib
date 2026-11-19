## 2026-11-19 — Review of: Ruby array arguments via `RArray::into_iter` (Magnus `Vec<T>` segfault) + `json` lock bump

**Verdict:** PASS_WITH_NOTES

**Summary:** All 7 Ruby array-conversion sites now go through `array_to_vec` (`RArray::into_iter`,
which walks a hidden copy), so an element whose `to_str`/`to_int` empties the caller's array no
longer crashes. The new `test_array_args.rb` (16 tests) compares each mutating call against an
unmutated snapshot. `json` is bumped to 2.21.2 and is the only line changed in the lock. Review
added three small fixes: a `TryConvertOwned` bound, a full-result assert in the mixed-code test, and
a narrower `RangeError` sentence in the docs.

**Verification:**

- [x] Grep criterion (no `Vec<` param, no `Vec<i32> = TryConvert::try_convert(frame)`): prints
    nothing (exit 1)
- [x] `bundle exec rake compile && bundle exec rake test`: 154 runs, 434 assertions, 0 failures,
    against a freshly compiled `.so`. Covers `to_str` (`encode_seq`, `gen_mixed_code_v0`,
    `gen_iscc_code_v0`) and `to_int` (audio, minhash, 2 video frame cases)
- [x] `cargo clippy -p iscc-rb -- -D warnings` exit 0 (workspace `--all-targets` too).
    `bundle exec standardrb` exit 0
- [x] `Gemfile.lock` has `json (2.21.2)` and `rb_sys (0.9.123)`
- [x] `test_non_string_element_raises_type_error` asserts `TypeError` for `encode_seq([nil])`. The
    docs Error Handling section names `TypeError` for wrong-type arguments
- [x] `mise run check` exit 0, and no files were rewritten
- [x] Probe: the advance claim that the `to_int` path crashed before the fix holds. I compiled the
    `1d318eb` `lib.rs`, and the audio and minhash mutation tests each aborted with exit 134
    (`[BUG] Segmentation fault`)

**Issues found:**

- Codex: `gen_audio_code_v0([-(2**63)])` returns the code of `[0]`. Magnus `RBignum::to_i32` skips
    the lower bound. This is pre-existing (same conversion as the old `Vec<i32>`). Probed and filed
    as `normal`
- Codex: `gen_iscc_id_v1(1 << 70, 0, 0)` raises `RuntimeError` by design (pinned in
    `test_iscc_id.rb`), which contradicted the docs' blanket `RangeError` sentence. Fixed in review:
    the docs now scope `RangeError` to list elements and name the `gen_iscc_id_v1` exception
- Codex: the mixed-code test compared only `.iscc`, which matches for the first 2 of 2000 units
    (probed: `.iscc` equal, `parts` differ). Fixed in review: it now asserts the whole result Hash
- Codex: the `array_to_vec` bound was `TryConvert`. Review changed it to `TryConvertOwned` (the
    bound Magnus' own `to_vec` uses), so `Vec<RString>` cannot compile. No behaviour change
- Accepted, not filed: `bits` is now converted before the array copy is taken. If `bits.to_int`
    empties the array, audio/mixed hash the emptied array instead of the original. Memory-safe, the
    video functions already behaved this way, and the input is absurd
- Nit: `test_gen_iscc_code_v0_survives_mutating_to_str` compares an error message ("Data-Code and
    Instance-Code are mandatory"), which a truncated array would also produce. next.md allowed this
- Out of scope but kept: the `crates/iscc-rb/CLAUDE.md` key-files row (docs only)

**Codex review:** Two runs, one on `adf4160` (the main commit) and one on `91085fc`. Three findings
were fixed in review (docs sentence, mixed-test assert, `TryConvertOwned` bound), one was filed, and
one was accepted (`bits` ordering). The `adf4160` run's docs finding (`RangeError` vs
`RuntimeError`, `nil`) was already fixed by `91085fc`.

**Next:** Three `normal` candidates are ready. (1) The NUL-truncation issue, which Titusz decided
today: .NET and C++ reject an embedded NUL at the wrapper boundary (decisions.md 2026-11-19). (2)
`@iscc/wasm` license/description metadata. (3) The newly filed Ruby negative-Bignum truncation, a
small fix in `iscc-rb`.

**Notes:**

- **Titusz's uncommitted interactive edits were in the working tree** during this review: a
    decisions.md entry, "Embedded NUL is rejected at the .NET and C++ wrapper boundary", and the
    matching rewrite of the NUL issue. I left them verbatim and committed them with this review, so
    prek's stash/restore during the push cannot take them away while his session is live
- **Dependabot alert #1 is still `open`** (checked via `gh api`). The default branch is `main`, so
    the alert closes only after develop merges into main. The `json` issue entry is deleted because
    nothing is left for CID to fix. update-state must not report the alert as fixed before that
    merge
- The Ruby segfault issue is deleted: both of its "Resolved when" clauses are met
- decisions.md sits at its 400-line budget, so I added no entry this review
