## 2026-11-18 — Go `EncodeSeq` / `DecodeSeq` (ISCC-SEQ on the pure-Go port)

**Done:** Ported ISCC-SEQ to `packages/go`: exported `EncodeSeq`/`DecodeSeq`, unexported `readUnit`,
`seqMainTypes`, and raw `uint32` header helpers (`encodeHeaderRaw`, `decodeHeaderRaw`). The typed
`encodeHeader`/`decodeHeader` are now thin wrappers with unchanged behaviour. `asciiUpper` gained
the no-allocation fast path. `ConformanceSelftest` now runs the `encode_seq`/`decode_seq` vectors
through a separate loader (`parseSeqConformanceData`) and two runners.

**Files changed:**

- `packages/go/codec.go`: raw header helpers, typed wrappers, `asciiUpper` fast path, ISCC-SEQ
    section (`seqMainTypes`, `headerMax`, `isSeqMainType`, `readUnit`, `EncodeSeq`, `DecodeSeq`),
    `bytes` import, file docstring
- `packages/go/conformance.go`: `seqVector`, `seqSections`, `parseSeqConformanceData`,
    `decodeBytesValue`, `runEncodeSeqTests`/`runEncodeSeqCase`,
    `runDecodeSeqTests`/`runDecodeSeqCase`, wired into `ConformanceSelftest`. Stale "9 gen
    functions"/"50" comments now say "every"
- `packages/go/conformance_test.go`: fixed the stale "46 vectors" comment, added runner tests (real
    data passes, missing sections fail, one mutated real vector per runner, 5 failure arms per
    runner with one case per call, plus a positive control)
- `packages/go/seq_test.go` (new): the `test_seq.rs` cases with one bad input per assertion, all 5
    `STRICT_BASE32_DIVERGENCES` rows (aliases rejected, twins accepted), raw-header unknown-SubType
    table, SubType 4679 and Version 1 round trips, non-zero padding nibble, out-of-range `readUnit`
    offsets (`len`, `len+1`, `100`, `-1`), `faaa-aaaaaaaa` → `28000000000000`, `AllocsPerRun == 0`

**Verification:**

- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` gives `ok`. `go vet` is clean and
    `gofmt -l packages/go` prints nothing
- The `EncodeSeq`/`DecodeSeq` signature greps each print one line (codec.go:879, :919).
    `grep -n 'func ReadUnit'` prints nothing (exit 1)
- `mise run check` exits 0
- Mutation proof: I edited a copy of the code, ran the tests, then restored it with `cp` (cmp
    clean):
    - no canonical prefix compare → 3 padding/Table-2 tests fail
    - `isccClean` before `asciiUpper` → 4 tests fail, including both `faaa-aaaaaaaa` tests
    - `uint8` SubType truncation in `readUnit` → 3 tests fail
    - `strings.ToUpper` instead of `asciiUpper` → the strict-divergence test fails
    - fast path disabled → the alloc test fails ("allocated 1 times")
    - mismatch check removed in either runner → its mutated-vector test and failure-arm test fail
- Alloc measurement with a throwaway test (deleted): `IsccDecode` of a 256-bit ISCC-CODE takes 80
    allocations with the fast path and 88 without it

**Next:** The text-utils U+001C–U+001F parity issue (Rust hot path, iai-gated), or the next ISCC-SEQ
surface (Node/WASM/FFI, …). The Tier-1 33→35 count and docs sweep stays deferred until every surface
binds seq. `packages/go/CLAUDE.md` still says "all 9 gen functions" and "46 conformance vectors",
and lists no seq functions. That belongs in the deferred docs sweep (out of scope here).

**Notes:**

- Gotcha: the first version of the alloc test (`_ = asciiUpper(...)`) still passed with the fast
    path removed, because inlining let the compiler drop the copy. It now assigns to a package-level
    sink (`asciiUpperSink`), and then the mutation is caught.
- `decodeHeaderRaw` returns `tailStart` (byte offset), not a copied tail. `decodeHeader` still
    copies the tail, so allocation behaviour on typed paths is unchanged.
- The `EncodeSeq` doc comment names the strict-base32 exception (padding, spare bits, non-ASCII
    case-fold aliases), as the confirmed decision requires.
- Go error strings use the Rust wording after the `iscc:` prefix (e.g. "Non-canonical ISCC-HEADER at
    offset N"), and the tests match on those substrings.
