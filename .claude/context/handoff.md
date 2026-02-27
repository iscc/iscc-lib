## 2026-02-27 — Implement pure Go DCT and WTA-Hash

**Done:** Ported DCT (Nayuki fast recursive algorithm) and WTA-Hash (winner-take-all video
fingerprinting) from Rust to pure Go. Both modules are faithful translations of
`crates/iscc-lib/src/dct.rs` and `crates/iscc-lib/src/wtahash.rs` with identical algorithms,
constant tables, and error handling.

**Files changed:**

- `packages/go/dct.go`: New file — `algDct` (unexported) and `dctRecursive` helper implementing the
    Nayuki fast DCT
- `packages/go/dct_test.go`: New file — 10 test functions covering error cases, edge cases, and
    known values
- `packages/go/wtahash.go`: New file — `AlgWtahash` (exported) with the 256-entry
    `wtaVideoIdPermutations` table
- `packages/go/wtahash_test.go`: New file — 9 test functions covering error validation, output
    correctness, and table integrity

**Verification:**

- `cd packages/go && go build ./...` — exits 0
- `cd packages/go && go test -run TestAlgDct -count=1 -v` — 10/10 DCT tests PASS
- `cd packages/go && go test -run TestAlgWtahash -count=1 -v` — 7/7 WTA-Hash tests PASS
- `cd packages/go && go test -run TestPermutation -count=1 -v` — 2/2 table validation tests PASS
- `cd packages/go && go vet ./...` — exits 0
- `cd packages/go && go test ./...` — all tests pass (including existing pure Go and WASM bridge
    tests)
- `mise run check` — all 14 hooks pass

**Next:** All five algorithm modules are now complete (codec, utils, CDC, MinHash, SimHash, DCT,
WTA-Hash). The next step should be implementing the gen functions layer, starting with
`gen_data_code_v0` and `gen_instance_code_v0` which need CDC + MinHash + BLAKE3 (requires adding
`github.com/zeebo/blake3` dependency). Alternatively, `gen_meta_code_v0` and `gen_text_code_v0`
could come first since they only need SimHash + MinHash + already-ported utils.

**Notes:** The next.md spec listed 12 DCT tests and 9 WTA-Hash tests, but the Rust source has 12 DCT
tests (some testing the same condition) which I consolidated into 10 Go test functions (combining
the three non-power-of-two-even tests into one function with three sub-cases, matching the logical
grouping). All 21 test cases from Rust are covered. No external dependencies added — both modules
use only `math` and `fmt` from stdlib.
