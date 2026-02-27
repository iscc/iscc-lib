## 2026-02-27 — Review of: Implement pure Go CDC, MinHash, and SimHash/SlidingWindow

**Verdict:** PASS

**Summary:** Three core algorithm modules (CDC, MinHash, SimHash+SlidingWindow) ported from Rust to
pure Go. Implementation is a faithful translation of the Rust reference with correct arithmetic
wrapping semantics, identical constant tables, and matching algorithm logic. All 39 Go tests pass,
`go vet` clean, 14 pre-commit hooks pass. Two minor fixes applied: `var` → `const` for scalar
constants, docstring correction for `minhashFn`.

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestCdc -count=1 -v` — 15/15 CDC tests PASS
- [x] `cd packages/go && go test -run TestMinhash -count=1 -v` — 8/8 MinHash tests PASS
- [x] `cd packages/go && go test -run TestSimhash -count=1 -v` — 6/6 SimHash tests PASS
- [x] `cd packages/go && go test -run TestSlidingWindow -count=1 -v` — 10/10 SlidingWindow tests
    PASS (8 new pure Go + 2 existing WASM bridge)
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass
- [x] `grep -c 'func Alg' packages/go/cdc.go` returns 1 (AlgCdcChunks)
- [x] `grep -c 'func Alg' packages/go/minhash.go` returns 1 (AlgMinhash256)
- [x] `grep -c 'func Alg' packages/go/simhash.go` returns 1 (AlgSimhash)
- [x] `grep -c 'func SlidingWindow' packages/go/simhash.go` returns 1
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex raised two advisory concerns: (1) small `avgChunkSize` could produce
negative `centerSize` — dismissed, matches Rust behavior exactly (no validation there either, the
algorithm expects reasonable chunk sizes like 1024); (2) function names might conflict with WASM
bridge — dismissed, WASM bridge uses `(rt *Runtime)` method receivers while new code uses
package-level functions, no collision. No actionable findings.

**Next:** Implement DCT and WTA-Hash (`dct.go`, `wtahash.go`) — these are the remaining algorithm
primitives needed before the gen functions layer. Alternatively, implement `gen_data_code_v0` and
`gen_instance_code_v0` which only need CDC + MinHash + BLAKE3 (requires adding
`github.com/zeebo/blake3` dependency). The gen functions are the higher-value target since they
directly enable conformance testing.

**Notes:** Go pure rewrite is at step 3/5 (codec ✓, text utils ✓, algorithms ✓ partial). Three of
five algorithm modules done, two remaining (DCT, WTA-Hash). The WASM bridge code (`iscc.go`) still
coexists — it will be removed once all gen functions are ported and conformance-tested. No new
external dependencies were added in this step (`math` is stdlib). The `check-added-large-files`
threshold (1024KB) can be lowered to 256KB once the WASM binary is removed from git.
