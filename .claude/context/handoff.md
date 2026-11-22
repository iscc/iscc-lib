## 2026-11-22 — Review of: Go port rejects ISCC-UNITs above 256 bits (encode and decode)

**Verdict:** PASS

**Summary:** The Go port now applies the 256-bit unit limit on encode (`encodeLength`) and on decode
(a checked `unitBitLength` at all four former `decodeLength` call sites). The diff stays in scope (3
Go source files, one how-to comment, tests). A differential of 291 cases against the Rust core
(through `iscc_lib`, built from HEAD) shows no difference in outcome or output, on amd64 and 386.

**Verification:**

- [x] `CGO_ENABLED=0 go test -count=1 ./...` and the `GOARCH=386` run pass;
    `go test -run Above256 -v ./...` lists passing tests for encode (7 gen functions, both hashers,
    `EncodeComponent`, `encodeLength`), decode (`IsccDecode`, `IsccDecompose`, `unitBitLength`),
    mixed and the wide cases (19 tests with the two added in review)
- [x] `go vet ./...` and `GOARCH=386 go vet ./...` clean; `gofmt -l .` empty
- [x] `git grep -n "decodeLength(" -- packages/go ':!*_test.go'` prints `codec.go:313` (definition)
    and `codec.go:335` (inside `unitBitLength`) only
- [x] `git grep -n ">= 32)" -- packages/go` prints nothing; `git grep -n "32 to 256"` shows
    `encodeLength`, the `EncodeComponent` comment and the how-to line (plus the older
    `IsccNormalizeSeq` comment and the test helper)
- [x] `mise run check` exits 0
- [x] Beyond next.md: differential Go vs Rust core, 291 cases (over-long units of every MainType at
    length indexes 7 to 72, short bodies, sequences, mixed, `GenIsccCodeV0` wide and standard with 2
    and 3 units, every gen function, both hashers, `GenSumCodeV0` and `EncodeComponent` at 13 `bits`
    values up to 2^32-1): 0 differences, no panic, amd64 output equals 386 output

**Issues found:**

- `GenSumCodeV0` had no `Above256` test (Rust has one), and no Go test read the second unit in the
    wide check (128-bit Data + 288-bit Instance, `wide=true`). Both behave correctly; I added
    `TestGenSumCodeV0Above256` and `TestGenIsccCodeV0WideAbove256InstanceAfterWideData` (tests only)
- Nit, not filed: the `unitBitLength` error branch inside `IsccDecode` cannot be reached, because
    `isccNormalize` runs `IsccDecompose` first and that rejects the unit. `TestIsccDecodeAbove256`
    therefore passes through the decompose check. The call is harmless (same shape as in Rust, see
    the iteration-301 nit)
- The bits issue is rewritten to the remaining problem (shared vector, API pages), still `normal`

**Codex review:** no actionable findings ("The 256-bit limits preserve composite and ISCC-ID
behavior").

**Next:** the `data-edge.json` rows for the 256-bit limit across all 12 harnesses (issue "No vector
pins the 256-bit unit limit, and most API pages lack the range"). Facts measured in this review that
the rows can rely on, identical on Go and the Rust core:

- Encode: `bits=288` fails for meta, text, audio, data, instance, both hashers, sum and
    `encode_component`. Image and video also fail, with their own earlier errors. Use a gen function
    without input units for the encode row (iteration-301 note)
- Decode: a 288-bit unit of MainType 0 to 4 and 7 fails `iscc_decode` and `iscc_decompose`, alone or
    as any member of a sequence
- `gen_iscc_code_v0`: an over-long Data or Instance unit fails only with `wide=true` and only when
    the wide check reaches it (288-bit Data + any Instance; 128-bit Data + 288-bit Instance). With
    `wide=false`, with a 64-bit Data unit first, or with three units, the call succeeds and returns
    the standard code. Do not write a row that expects a rejection there
- Check each row on both iscc-core builds (CPython 3.10 and 3.15) before marking it `diverges`: the
    decode rejection matches iscc-core, the encode rejection is the divergence

After that, the API-page docs step, then the three `[audit]` `normal` issues.

**Notes:**

- No new decision and no new learning: the step ports a decided behaviour, and learnings.md is at
    its 200-line budget
- Go-only step: no CRAP, iai or semver impact; the core crate is untouched
