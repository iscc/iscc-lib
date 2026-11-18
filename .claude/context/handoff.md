## 2026-11-18 — Review of: Core `iscc_clean` trims exactly what Python `str.strip()` trims (U+001C–U+001F)

**Verdict:** PASS_WITH_NOTES

**Summary:** A private `is_py_strip_char` predicate (`char::is_whitespace` + U+001C..=U+001F) now
drives all four codec trim sites, including the `encode_seq` empty-unit guard. The `Cow` paths are
kept and nothing is allocated. My exhaustive differential matches `iscc_core` 1.5.0 in both
directions: every code point, 3 positions (around, before colon, after colon), one unit per call, 0
differences. The issue is resolved and deleted. I added the reverse-direction check, filed the
text-utils divergence, and moved the Go strip-set requirement into the Go issue.

**Verification:**

- [x] `cargo test -p iscc-lib` passes: 297 lib + 17 `codec_clean` + the rest. Feature matrix also
    green: `--no-default-features` 246 lib, `--features text-processing` 267 lib, NDF clippy clean
- [x] `grep -nE '\.trim\(\)' crates/iscc-lib/src/codec.rs` prints nothing (exit 1)
- [x] `uv run pytest tests/test_seq.py` passes after `maturin develop`: 77 at advance, 78 with my
    added test
- [x] `mise run bench:iai:check` exits 0 against the unmodified `.iai-baseline.json` (16/16 within
    10%; `four_units` 12,784 vs 13,369, −4.38%)
- [x] `mise run coverage` + CI's exact `cargo crap … --fail-regression --fail-above` exits 0 (118
    unchanged). CI on 72054e8 ran rustc 1.100.0, the same as local, and also scores
    `run_encode_seq_tests` at 100%, so dropping its stale 80.8% baseline entry matches CI
- [x] `mise run check` exits 0; `cargo clippy -p iscc-lib --all-targets -- -D warnings` and the
    workspace clippy are clean

**Issues found:**

- Committed tests proved only one direction: every char Python strips is stripped by Rust. Nothing
    pinned "Rust strips nothing Python keeps", which is the other half of "exactly". **Fixed in
    review:** `test_differential_rejects_space_like_chars_python_keeps` in `tests/test_seq.py`. It
    derives candidates live (categories `Zs/Zl/Zp/Cc/Cf` minus the strip set; this covers all of
    `White_Space`), and both libraries must reject `ch + UNIT`. Runs in 0.64s. Mutation check:
    dropping U+001E from the oracle makes it fail
- Filed `normal`: `text_trim` and `text_remove_newlines` keep U+001C–U+001F, which iscc-core strips.
    Probed over every code point: exactly these 4 differ. Meta-Code (code + metahash) and Text-Code
    did not diverge on the probed inputs, because `text_clean` drops all `Cc` first
- Nit (not filed): the predicate unit test in `codec.rs` and the `is_python_whitespace` helper in
    `codec_clean.rs` restate the predicate's own formula, so they are mirrors, not oracles. The real
    oracle is the live-derived Python strip set in `tests/test_seq.py`
- `lib.rs:431` (`payload.trim()`, the metahash payload) cannot reach the 4 separators, because its
    inputs are already `text_clean`ed. No action needed

**Codex review:** Two runs. The main commit `e8ca0ac` was clean (17 Rust + 77 Python tests re-run).
HEAD (handoff-only) raised a P3: advance's "~−5% toolchain drift" explanation for the iai numbers is
unsupported. The untouched benches range from −7.65% to +0.05%, so the drop is not uniform. The
edit's measured cost stands (`four_units` +0.19%, `two_codes` +0.11% on a same-rustc rerun). The
finding is valid and is reflected below.

**Next:** Candidates, all `normal`:

- **Go `decodeBase32` spare-bit fix + Rust/Go strict-base32 tests** (`[human]`, independent,
    advance's top pick).
- **Go `isccClean` port.** Its entry now requires the Python strip set. Per the Go docs (unprobed),
    `unicode.IsSpace`/`strings.TrimSpace` follow `White_Space` and miss U+001C–U+001F.
- **Text-utils trim parity.** Reuse `codec::is_py_strip_char`, and run `bench:iai:check`, since the
    utilities sit on the Meta-Code benches.

Gate maintenance (not an issue): 10 of 16 iai benches sit 3.5–7.7% below the committed baseline. For
those benches the effective tolerance is looser than 10%, so +10% regressions there would pass. A
deliberate `.iai-baseline.json` refresh needs explicit authorisation in its own step.

**Notes:**

- Spec criterion (`rust-core.md` seq checklist, "both sides reject the same inputs"):
    `tests/test_seq.py` now covers both directions of the whitespace set plus the strict-base32
    divergence. The spec checkbox is update-state's to judge.
- Python tests derive the strip set from the running interpreter, so they follow the CI Python
    version automatically. No hand-typed table exists anywhere.
- `learnings.md` was at the 199/200 budget. The settled Kotlin consumer-floor pointer moved to the
    archive, which made room for the strip-set update.
