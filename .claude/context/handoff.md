## 2026-02-27 — Implement pure Go CDC, MinHash, and SimHash/SlidingWindow

**Done:** Ported the three core algorithm primitives from Rust to pure Go: CDC (Content-Defined
Chunking with gear rolling hash), MinHash (64-dimensional with universal hash functions and
bit-interleaved compression), and SimHash + SlidingWindow (bit-vote aggregation and Unicode-aware
n-gram generation). All three modules follow the established Go conventions from `codec.go` and
`utils.go`.

**Files changed:**

- `packages/go/cdc.go`: New file — `cdcGear` table (256 entries), `algCdcParams`, `algCdcOffset`,
    `AlgCdcChunks` (public)
- `packages/go/minhash.go`: New file — `mpa`/`mpb` arrays, `minhashFn`, `minhashCompress`,
    `AlgMinhash256` (public)
- `packages/go/simhash.go`: New file — `AlgSimhash` (public with error return), `SlidingWindow`
    (public with error return)
- `packages/go/cdc_test.go`: New file — 15 tests covering params, offset, chunks, reassembly,
    determinism, UTF-32 alignment
- `packages/go/minhash_test.go`: New file — 8 tests covering empty features, single feature,
    determinism, compress, AlgMinhash256
- `packages/go/simhash_test.go`: New file — 14 tests covering single digest, empty, identical,
    opposite, mismatched lengths, 4-byte digests, SlidingWindow basic/unicode/edge cases

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestCdc -count=1 -v` — 15/15 CDC tests PASS
- [x] `cd packages/go && go test -run TestMinhash -count=1 -v` — 8/8 MinHash tests PASS
- [x] `cd packages/go && go test -run TestSimhash -count=1 -v` — 6/6 SimHash tests PASS
- [x] `cd packages/go && go test -run TestSlidingWindow -count=1 -v` — 10/10 SlidingWindow tests
    PASS (includes 2 existing WASM bridge tests)
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass (pure Go + existing WASM bridge, 9.6s)
- [x] `grep -c 'func Alg' packages/go/cdc.go` returns 1 (AlgCdcChunks)
- [x] `grep -c 'func Alg' packages/go/minhash.go` returns 1 (AlgMinhash256)
- [x] `grep -c 'func Alg' packages/go/simhash.go` returns 1 (AlgSimhash)
- [x] `grep -c 'func SlidingWindow' packages/go/simhash.go` returns 1
- [x] `mise run check` — all 14 hooks pass

**Next:** Implement DCT and WTA-Hash (`dct.go`, `wtahash.go`) — these are the remaining algorithm
primitives needed for Image-Code and Video-Code gen functions. Alternatively, proceed to implement
the gen functions that only need the current primitives (e.g., `gen_data_code_v0` needs CDC +
MinHash + BLAKE3, `gen_text_code_v0` needs MinHash + xxHash + SlidingWindow).

**Notes:** The MinHash internal function was named `minhashFn` instead of `minhash` to avoid
conflict with Go's convention where the package-level function name would shadow the type/package.
The `& maxi64` operation in minhash is a no-op for uint64 but kept for parity with the reference
implementation. Go's natural uint32/uint64 arithmetic wrapping matches Rust's
`wrapping_add`/`wrapping_mul` perfectly — no special handling needed. No new dependencies were added
(only `math` from stdlib).
