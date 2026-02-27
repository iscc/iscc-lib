# Next Work Package

## Step: Implement pure Go CDC, MinHash, and SimHash/SlidingWindow

## Goal

Port the three core algorithm primitives (CDC, MinHash, SimHash + SlidingWindow) from Rust to pure
Go, completing the algorithm layer in the dependency chain (codec → text utils → **algorithms** →
gen functions). These are prerequisites for all 9 `gen_*_v0` functions.

## Scope

- **Create**: `packages/go/cdc.go`, `packages/go/minhash.go`, `packages/go/simhash.go`,
    `packages/go/cdc_test.go`, `packages/go/minhash_test.go`, `packages/go/simhash_test.go`
- **Modify**: (none)
- **Reference**: `crates/iscc-lib/src/cdc.rs`, `crates/iscc-lib/src/minhash.rs`,
    `crates/iscc-lib/src/simhash.rs`, `packages/go/codec.go` (for Go conventions/patterns),
    `packages/go/utils.go` (for Go conventions/patterns)

## Not In Scope

- DCT (`dct.rs`) and WTA-Hash (`wtahash.rs`) — these are only needed for Image-Code and Video-Code
    and belong in a separate step
- Any `gen_*_v0` functions — those compose algorithms and are the next layer
- Streaming hashers (`DataHasher`, `InstanceHasher`) — depend on gen functions and CDC
- External dependencies (`github.com/zeebo/blake3`, xxHash) — the three algorithm primitives are
    pure computation with no external deps (they take pre-computed features/data as input)
- Modifying `go.mod` — no new dependencies needed for these algorithms
- Removing the WASM bridge code (`iscc.go`) — it coexists during the rewrite
- `sliding_window_strs` or `sliding_window_bytes` internal variants — only the public
    `SlidingWindow` (string-based) is Tier 1

## Implementation Notes

### CDC (`cdc.go`)

Port from `crates/iscc-lib/src/cdc.rs`. Four components:

1. **`cdcGear` table** — 256-entry `[256]uint32` constant (unexported). Copy values exactly from
    Rust `CDC_GEAR`.
2. **`algCdcParams(avgSize uint32) (mi, ma, cs int, maskS, maskL uint32)`** — unexported helper.
    Calculates min/max/center sizes and masks. Note: `min_size.div_ceil(2)` in Rust → use
    `(minSize + 1) / 2` in Go (integer ceiling division). `(avg_size as f64).log2().round()` →
    `math.Round(math.Log2(float64(avgSize)))`. Import `math`.
3. **`algCdcOffset(buffer []byte, mi, ma, cs int, maskS, maskL uint32) int`** — unexported helper.
    Gear rolling hash scanning in two phases (strict mask then relaxed mask). Use
    `(pattern >> 1) + cdcGear[buffer[i]]` with Go's natural `uint32` wrapping.
4. **`AlgCdcChunks(data []byte, utf32 bool, avgChunkSize uint32) [][]byte`** — public function.
    Returns `[][]byte` slices into the original data (use Go slicing `data[pos:pos+cutPoint]`).
    Empty input → single empty slice `[][]byte{data[0:0]}`. UTF-32 alignment:
    `cutPoint -= cutPoint % 4`, if zero then `cutPoint = min(len(remaining), 4)`.

**Go-specific**: No `wrapping_add` needed — Go's `uint32` arithmetic wraps naturally. The `min()`
builtin is available in Go 1.21+.

### MinHash (`minhash.go`)

Port from `crates/iscc-lib/src/minhash.rs`. Four components:

1. **Constants** — `maxi64 uint64 = math.MaxUint64`, `mprime uint64 = (1 << 61) - 1`,
    `maxH uint64 = (1 << 32) - 1`, `mpa [64]uint64`, `mpb [64]uint64` (all unexported). Copy
    MPA/MPB arrays exactly from Rust.
2. **`minhash(features []uint32) []uint64`** — unexported. For each of 64 dimensions, compute
    `min(((a*uint64(f) + b) & maxi64) % mprime) & maxH)` across all features. Return `maxH` per
    dimension when features is empty. **Critical**: use `uint64` arithmetic throughout. Go's `*` on
    `uint64` wraps at 2^64 naturally (matching Rust's `wrapping_mul`). The `& maxi64` is a no-op
    for uint64 but keep it for clarity/parity with reference.
3. **`minhashCompress(mhash []uint64, lsb int) []byte`** — unexported. Bit-interleaved compression.
    Extract `lsb` LSBs from each hash, iterate bit positions 0..lsb then hash values, pack
    MSB-first into bytes. Use `(totalBits + 7) / 8` for ceiling division.
4. **`AlgMinhash256(features []uint32) []byte`** — public. Calls `minhash` → `minhashCompress` with
    `lsb=4`. Returns 32 bytes.

### SimHash + SlidingWindow (`simhash.go`)

Port from `crates/iscc-lib/src/simhash.rs`. Two public functions:

1. **`AlgSimhash(hashDigests [][]byte) ([]byte, error)`** — public. Validates all digests have equal
    length (return `fmt.Errorf(...)` if not). Bit-vote aggregation: count bits across all digests,
    set bit if `count*2 >= n`. Empty input → 32 zero bytes. Uses MSB-first bit ordering
    (`7 - (i % 8)`). No need for a separate unchecked variant (Go has no binding crate separation).
2. **`SlidingWindow(seq string, width int) ([]string, error)`** — public. Returns overlapping
    substrings of `width` runes. Return error if `width < 2`. If input shorter than width, return
    single element with full input. Use `[]rune` conversion for proper Unicode character counting,
    then `string(runes[i:end])` for each window.

### Testing pattern

Follow the established pattern from `codec_test.go` and `utils_test.go`:

- Package `iscc` (same package, access to internal functions)
- Use `testing.T` with subtests where appropriate
- Test edge cases: empty input, single element, deterministic output, reassembly (CDC)
- Port the unit tests from Rust directly (see test sections at bottom of each `.rs` file)
- CDC: test params calculation, small buffer offset, max-size cap, empty/small chunks, reassembly,
    determinism, utf32 alignment (including the 3-byte regression test)
- MinHash: empty features (all 0xFF), single feature, deterministic, compress basic/all-ones
- SimHash: single digest (passthrough), empty (32 zero bytes), identical digests, opposite digests,
    mismatched lengths (error), SlidingWindow basic/unicode/edge cases

### Naming conventions

Follow Go conventions matching existing `codec.go` / `utils.go`:

- Public: `AlgCdcChunks`, `AlgMinhash256`, `AlgSimhash`, `SlidingWindow`
- Unexported: `algCdcParams`, `algCdcOffset`, `minhash`, `minhashCompress`, `cdcGear`, `mpa`, `mpb`,
    `maxi64`, `mprime`, `maxH`

## Verification

- `cd packages/go && go build ./...` exits 0 — compiles alongside existing WASM code
- `cd packages/go && go test -run TestCdc -count=1 -v` — CDC tests pass
- `cd packages/go && go test -run TestMinhash -count=1 -v` — MinHash tests pass
- `cd packages/go && go test -run TestSimhash -count=1 -v` — SimHash tests pass
- `cd packages/go && go test -run TestSlidingWindow -count=1 -v` — SlidingWindow tests pass
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + existing WASM bridge tests)
- `grep -c 'func Alg' packages/go/cdc.go` returns 1 (AlgCdcChunks)
- `grep -c 'func Alg' packages/go/minhash.go` returns 1 (AlgMinhash256)
- `grep -c 'func Alg' packages/go/simhash.go` returns 1 (AlgSimhash)
- `grep -c 'func SlidingWindow' packages/go/simhash.go` returns 1

## Done When

All verification criteria pass — three algorithm files compile, all unit tests pass, and existing
WASM bridge tests continue to pass alongside the new pure Go code.
