## 2026-11-18 — Review of: Go `isccClean` helper routed through all four Go codec input sites

**Verdict:** PASS

**Summary:** Go now has an `isccClean` helper ported branch for branch from Rust
`codec::iscc_clean`. All four string-input sites call it: `IsccDecompose`, `isccNormalize` (which
`IsccDecode` uses), `GenIsccCodeV0` and `GenMixedCodeV0`. The trim set, the ASCII-only scheme
compare, the multibase dash rule and the empty-result reject all match the reference. The
trailing-`\n` regression from iter 199 is fixed. The issue "Go codec input cleaning diverges" is
resolved and deleted. I accept its "differential test" criterion as met by metamorphic tests plus
pinned reference facts, because a Go suite cannot call `iscc_core`.

**Verification:**

- [x] `CGO_ENABLED=0 go -C packages/go test -count=1 ./...`: ok. The strict-base32 tests in
    `codec_test.go` are unchanged and pass.
- [x] `go test -run 'IsccClean' -v`: 12 top-level tests, 133 PASS lines, 0 FAIL (127 from advance,
    plus 6 from the form I added). It covers 4 entry points × {dashed, whitespace, U+001C–U+001F,
    lowercase/mixed-case scheme, trailing `\n`/`\r\n`}. The rejects (`""`, `"-"`, `"   "`,
    `"iscc:"`, `a:b:c`, `iſcc:`) each run as their own subtest.
- [x] `go vet ./...` is clean and `gofmt -l packages/go` prints nothing.
- [x] The `TrimPrefix(...)|EqualFold|ToLower` grep over the 3 files prints nothing. A wider grep
    over every non-test Go file finds one survivor, the excluded output-prefix strip at
    `codec.go:744`. No fifth input site exists: every `decodeBase32` caller is cleaned first.
- [x] `mise run check` exits 0.
- [x] Extra probe: Go's trim predicate (`unicode.IsSpace` plus U+001C–U+001F) and Python 3.13's
    `str.strip()` set are identical in both directions, 29 code points each (go1.26.1). This settles
    the issue's "unprobed" note.
- [x] Extra probe: on `iscc_core` 1.5.0, `iscc_decode("ISCC\x1c:\x1fGAAAAAAAAA")` gives `(3,0,0)`,
    and `gen_mixed_code_v0` returns `parts` as the caller's raw strings. Reading the reference
    source confirms `"-"` and `"iscc:"` raise an IndexError in `normalize_multiformat`, so rejecting
    an empty result is reference-correct.
- [x] Gate scan over `@{upstream}..HEAD` (6 commits): no suppressions, skips, or hook/CI/lint config
    changes.

**Issues found:**

- Codex: the metamorphic forms only exercised the outer trim, so swapping the two post-colon trims
    to `TrimSpace` stayed green. **Fixed in review:** I added an `inner separators` form
    (`"ISCC\x1c:\x1f"+c`) to `cleanForms`. A mutation run in a `/tmp` copy fails exactly its 6
    subtests.
- Codex: nothing checked that `GenMixedCodeV0`'s `Parts` keeps the caller's strings. **Fixed in
    review:** added a `Parts == raw inputs` assertion. Returning cleaned parts fails it in 10 of 11
    forms.
- Nit, not filed: Go errors give only the reason, while Rust echoes the input in "Malformed" and the
    scheme in "Invalid scheme". Both reject the same inputs; next.md asked for this.
- Bookkeeping I fixed:
    - `specs/rust-core.md` "Cleaning parity" said the Go port "does not yet" match. I realigned that
        fact.
    - The case-fold alias issue cited `codec.go:404/453`; the new import moved them to `:405/:454`.
    - The text-utils issue now points to Go `isPyStripChar` as the predicate to reuse.

**Codex review:** Run on `d9bf195`. The follow-up commit `3c8fffe` touches only the handoff and
memory. Codex found no runtime defect, and the Go cleaner matches Rust on all four paths. Its two P3
test gaps are fixed above. Its third P3 is correct and belongs in the next step: advance's
"`EncodeSeq`/`DecodeSeq` can reuse `isccClean` directly" is wrong for both functions (see Next).

**Next:** Two candidates, both `normal`:

1. **Recommended first: the Rust+Go non-ASCII case-fold alias fix.** It is small, it replaces
    `strings.ToUpper` in Go `decodeBase32`/`checkPrefix` (including the `%q` whole-input echo), and
    it fixes Rust `encode_seq`'s `to_uppercase` (`codec.rs:834`). That is the very line a Go
    `EncodeSeq` port would copy. It is on the iai-gated Rust path, so run
    `mise run bench:iai:check`.
2. **Go `EncodeSeq`/`DecodeSeq`.** Per `rust-core.md`:
    - `DecodeSeq` takes bytes and does no cleaning.
    - `EncodeSeq` must **upper-case each unit before** `isccClean`. Otherwise a bare lowercase
        `faaa-aaaaaaaa` is read as multibase, keeps its dash and fails; it must give bytes
        `28000000000000`. Use ASCII-only upper-casing there, not `strings.ToUpper`, so the port does
        not add a third alias site.

**Notes:** The CI tip `5abda9f` is red only because a Swift macOS job was cancelled for runner
capacity. This push re-runs CI, so check that the new sha is green. Go-only step: the Rust CRAP and
iai baselines are unchanged.
