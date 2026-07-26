# Handoff

## 2026-07-26 — Review of: Propagate the Unicode boundary fixture to the Python and Go test suites (propagation slice 1)

**Verdict:** PASS_WITH_NOTES

**Summary:** The first two binding surfaces are now gated by the canonical Unicode 16.0.0 boundary
fixture. Python runs all 12 vectors plus a metadata guard (13 passed); the pure-Go package embeds a
byte-identical vendored copy and runs 9 of 12, skipping exactly the 3 ruled table-dependent cases —
verified independently, including all four sequence vectors passing in Go. Scope discipline is
exemplary: zero non-test, non-doc source files touched, no oracle duplication, canonical fixture and
both baselines untouched.

**Verification:**

- [x] `uv run pytest tests/test_unicode_boundary.py -q` — **13 passed** (≥ 13 collected, 0 failures)
- [x] `uv run pytest -q` — **371 passed**, no regressions
- [x] `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go` — `ok`, exit 0
- [x] `go vet ./...` in `packages/go` — exit 0
- [x] `… go test -v -run UnicodeBoundary … | grep -c -- '--- SKIP'` — exactly **3**; the three lines
    are precisely the ruled cases (`text_clean` U+1FAE9 + U+113C5, `text_collapse` U+1FAE9), and 9
    subtests PASS incl. all four sequence vectors
- [x] `cmp` canonical vs vendored fixture — exit 0; both 2344 bytes, both `isascii()` true
- [x] `gofmt -l .` in `packages/go` — empty
- [x] `cargo test -p iscc-lib` — **336 passed, 0 failed** (281+28+22+4+1, unchanged)
- [x] `git status --porcelain crates/iscc-lib/ .crap-baseline.json .iai-baseline.json` — empty
- [x] `mise run check` — every prek hook Passed, exit 0, zero reformats of the advance diff
- [x] `uv run zensical build` — "No issues found"; `check_docs_nav.py` — OK, 23 pages
- [x] `grep -F -c 'unicode_boundary.json' docs/unicode.md` = **2**;
    `grep -F -c 'testdata/unicode_boundary.json' packages/go/CLAUDE.md` = **2**
- [x] *(review-added)* `cargo clippy --workspace --all-targets -- -D warnings`, `ty check`,
    `ruff check --select S`, `ruff check --select C901` — all clean (pre-push gates)

**Mutation probes (review-added — the guards are not vacuous):**

- Corrupt a **non-skipped** expected output in the Go copy → `FAIL … got "ab", want "aXb"`
- Rename a **skipped** case → the stale-skip guard fires
    (`skip key "…" names a case missing from the fixture`) *and* the renamed case stops being
    skipped and fails loudly
- Drop a case + downgrade `unicode_data_version` → both metadata assertions fire
- Python: substitute the iteration-149 mislabeled oracle `e U+015A` → `AssertionError`, 1 failed
- Confirmed the suites are **design-discriminating without an oracle column**: the sequence vectors'
    expected values are the decomposed sentinel outputs (`0065 0301`, `1100 1161`,
    `03B1 03C2 03B2`), all unequal to the delete-filter results, so plain equality against
    `outputs.result` reds on a delete-filter regression. next.md's Not-In-Scope reasoning holds.

**Issues found:**

- (none blocking) The `t.Skipf` calls are the only skip-shaped construct in the batch and are
    authorized verbatim by `decisions.md` 2026-07-26 ("Go skips the Unicode-16 boundary vectors
    until go1.27"), each with an inline reason naming the ruling. Not gate weakening — the Go suite
    *gains* 9 live assertions.
- Filed `[review]` `normal`: **"Gate byte-identity of the vendored test-vector copies"**. Six
    vendored copies (5 × `data.json`, 1 × `unicode_boundary.json`) match by convention only — no
    gate. All agree at HEAD (`md5sum`), so the check would be green today and the step is small.
    next.md deliberately deferred it.
- Recorded in `issues.md` + `decisions.md`: a **go1.27 bump checklist**. See Codex review below.
- Nit, not fixed: `want, _ := vec.Outputs["result"].(string)` swallows a missing/renamed `result`
    key. Harmless here — no expected output is the empty string, so every case would still fail
    loudly — but a `t.Fatalf` on the failed assertion would be strictly better.

**Codex review:** One P1 finding, **substantiated and accepted as a note, not a defect**. Codex
observed that under `GOTOOLCHAIN=go1.27rc1` the new Go suite fails while `HEAD^` passes, because
`x/text` selects Unicode 17 tables and the unconditional skip map covers only the three Go 1.26
failures. I confirmed the mechanism independently (`x/text@v0.40.0/unicode/norm/tables17.0.0.go`
carries `//go:build go1.27`): Go passes the `U+20C1` / `U+A7F1` vectors today *for the wrong reason*
— its 15.0 tables call them `Cn` and the category-`C` filter drops them, coinciding with the freeze
rule's output by accident. At go1.27, 5 currently-green cases flip red and the 3 skips become
unnecessary. **Codex's proposed remedy (version-gate the suite) is rejected**: that converts a
designed failure signal into pre-emptive test skipping and would let a toolchain bump land with Go
quietly non-conformant. The red is the trigger to land the 731-range freeze table in
`packages/go/utils.go`, which `decisions.md` already says Go needs regardless. Risk of surprise is
low — `ci.yml` pins Go via `go-version-file: packages/go/go.mod` (`go 1.26.1`). Rationale recorded
in `decisions.md`; a bump checklist is in the `issues.md` Unicode entry. No other findings.

**Next:** Continue the Unicode fixture propagation with slice 2. Nine binding surfaces remain (napi,
WASM, Ruby, C FFI, JNI/Java, UniFFI, Kotlin, Swift, C#, C++) plus the four sibling `data.json`
copies. Suggested grouping by build cost: the **WASM + Ruby** pair or **C FFI + JNI** are runnable
in-container; **napi must rebuild its stale checked-in artifact first**
(`crates/iscc-napi/iscc-lib.linux-x64-gnu.node` still returns `aSb` for
`text_clean("a" U+A7F1 "b")`) so it is the most expensive slice. Alternatively, land the small
`[review]` drift gate first — it is green at HEAD, guards every remaining slice, and is roughly a
one-file pytest anchor test.

**Notes:**

- The propagation pattern is now established and worth reusing verbatim: read the canonical fixture
    by relative path where the language can (Python), vendor with `cp` + verify with `cmp` where it
    cannot (Go `//go:embed`), assert `unicode_data_version` + per-section counts (7 / 5), and never
    copy the `delete_filter_output` oracle. Any skip list needs a stale-key guard.
- Never write the fixture through the Write/Edit tools — the `\uXXXX` escapes decode to literal
    UTF-8. This bit both agents in iteration 149; the advance agent correctly used `cp` this time.
- `docs/unicode.md`'s pure-Go admonition says the skips last "until go1.27 ships newer Unicode
    tables". Accurate for the skips, but incomplete about what go1.27 *also* breaks. Left alone
    deliberately (the preceding sentence already states Go "does not yet implement the freeze rule",
    so no user is misled); worth widening whenever the Go freeze table lands.
- After `uv run zensical build` I re-ran `scripts/gen_llms_full.py` — the build wipes `site/`.
- `learnings.md` was pruned to 199 lines: the three settled codec rules (bitwise selftest masking,
    exact-length decode guard, `decode_length` multiples) moved to `learnings-archive.md`; all three
    are pinned by tests, so the notes were redundant.
