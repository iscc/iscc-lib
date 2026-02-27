# Next Work Package

## Step: Implement pure Go DCT and WTA-Hash

## Goal

Port the remaining two algorithm primitives (DCT and WTA-Hash) from Rust to pure Go, completing the
algorithm layer (step 4 of ~5 in the Go rewrite). These are required by `gen_image_code_v0` and
`gen_video_code_v0` before the gen functions layer can begin.

## Scope

- **Create**: `packages/go/dct.go`, `packages/go/dct_test.go`, `packages/go/wtahash.go`,
    `packages/go/wtahash_test.go`
- **Modify**: none
- **Reference**: `crates/iscc-lib/src/dct.rs` (148 lines), `crates/iscc-lib/src/wtahash.rs` (389
    lines — mostly a 256-entry constant table)

## Not In Scope

- Adding `github.com/zeebo/blake3` or any new external dependencies (not needed for DCT/WTA-Hash)
- Implementing any `gen_*_v0` functions (those are step 5, after all algorithms are complete)
- Removing the WASM bridge (`iscc.go`, `iscc_ffi.wasm`) — only after all gen functions pass
    conformance
- Modifying existing Go files (`codec.go`, `utils.go`, `cdc.go`, `minhash.go`, `simhash.go`)
- Exporting DCT as a public function — `alg_dct` is `pub(crate)` in Rust, so `algDct` should be
    unexported in Go (lowercase). WTA-Hash is public in Rust (used by `soft_hash_video_v0`), so
    `AlgWtahash` should be exported

## Implementation Notes

### DCT (`dct.go`)

Port the Nayuki fast recursive DCT from `crates/iscc-lib/src/dct.rs`:

- **Function**: `algDct(v []float64) ([]float64, error)` — unexported, matches Rust `pub(crate)`
- **Validation**: input length must be a power of 2 and > 0; return error otherwise. Use
    `n > 0 && n&(n-1) == 0` for power-of-2 check (Go has no `is_power_of_two()`)
- **Algorithm**: recursive divide-and-conquer — exact translation of `dct_recursive` in Rust:
    1. Base case: length 1 → return copy of input
    2. Split into symmetric `alpha[i] = v[i] + v[n-1-i]` and antisymmetric
        `beta[i] = (v[i] - v[n-1-i]) / (math.Cos((float64(i)+0.5)*math.Pi/float64(n)) * 2.0)`
    3. Recurse on alpha and beta
    4. Interleave:
        `result = [alpha[0], beta[0]+beta[1], alpha[1], beta[1]+beta[2], ...,  alpha[half-1], beta[half-1]]`
- **Dependencies**: only `math` stdlib (`math.Pi`, `math.Cos`)
- **Helper**: `dctRecursive(v []float64) []float64` — unexported recursive implementation

### WTA-Hash (`wtahash.go`)

Port the WTA-Hash from `crates/iscc-lib/src/wtahash.rs`:

- **Function**: `AlgWtahash(vec []int64, bits uint32) ([]byte, error)` — exported (public in Rust)
- **Constant table**: `wtaVideoIdPermutations` — 256 pairs stored as
    `var wtaVideoIdPermutations =   [256][2]int{...}` (Go has no const arrays). Copy the exact 256
    `(i, j)` pairs from Rust source
- **Validation**: `len(vec) >= 380`, `bits > 0 && bits%8 == 0 && bits <= 256`; return error
    otherwise
- **Algorithm**: for each permutation pair `(i, j)` up to `bits` pairs, if `vec[i] < vec[j]`, set
    output bit to 1. Bits packed MSB-first: `result[bitIdx/8] |= 1 << (7 - (bitIdx % 8))`
- **Dependencies**: none beyond stdlib (`fmt` for errors)

### Testing pattern

Follow the established pattern from `cdc_test.go`, `minhash_test.go`, `simhash_test.go`:

- Package `iscc` (same package, access to unexported functions like `algDct`)
- Use `testing.T` with descriptive function names

**DCT tests** (port all 12 from Rust):

- `TestAlgDctEmptyError` — empty input returns error
- `TestAlgDctOddLengthError` — length 3 returns error
- `TestAlgDctAllZeros` — 64 zeros → all results ≈ 0
- `TestAlgDctAllOnes` — 64 ones → first = 64.0, rest ≈ 0
- `TestAlgDctUniformExactZeros` — 32 × 255.0 → first = 8160.0, rest = exact 0.0
- `TestAlgDctRange` — 0..63 → first ≈ 2016.0
- `TestAlgDctSingle` — `[42.0]` → `[42.0]`
- `TestAlgDctNonPowerOfTwoEvenError` — length 6, 10, 12 return error
- `TestAlgDctLength2Ok` — length 2 succeeds
- `TestAlgDctKnownValues` — `[1,2,3,4]` → `[10.0, -3.15432..., ~0, -0.22417...]`

**WTA-Hash tests** (port all 9 from Rust):

- `TestAlgWtahashAllZeros` — all-zero → all-zero output
- `TestAlgWtahashRange` — 0..379 → non-zero deterministic output
- `TestAlgWtahash256Bits` — 256-bit output → 32 bytes
- `TestAlgWtahashShortInputError` — 100 elements → error
- `TestAlgWtahashZeroBitsError` — bits=0 → error
- `TestAlgWtahashNonDivisibleBitsError` — bits=7 → error
- `TestAlgWtahashExceedsPermutationsError` — bits=512 → error
- `TestPermutationTableLength` — table has 256 entries
- `TestPermutationIndicesInRange` — all indices < 380

### Naming conventions

Follow Go conventions matching existing algorithm files:

- Public: `AlgWtahash`
- Unexported: `algDct`, `dctRecursive`, `wtaVideoIdPermutations`

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestAlgDct -count=1 -v` — all DCT tests pass
- `cd packages/go && go test -run TestAlgWtahash -count=1 -v` — all WTA-Hash tests pass
- `cd packages/go && go test -run TestPermutation -count=1 -v` — table validation tests pass
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (including existing pure Go and WASM bridge
    tests)
- `grep -c 'func algDct' packages/go/dct.go` returns 1
- `grep -c 'func AlgWtahash' packages/go/wtahash.go` returns 1

## Done When

All verification criteria pass: DCT and WTA-Hash modules compile, all new tests pass, all existing
Go tests still pass, and `go vet` is clean.
