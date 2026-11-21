## 2026-11-21 — CDC parameter maths matches the reference for every `avg_chunk_size` (Rust core + Go)

**Done:** Rust `alg_cdc_params` computes `max_size` in `u64` (saturating at `usize::MAX`) and builds
both masks through a private `low_bits_mask(n)` (`checked_shl`, so 32+ bits gives `u32::MAX`). Go
`algCdcParams` computes `maxSize` in `uint64` and does `centerSize` as `uint32`, clamping both to
`math.MaxInt` via a small `clampInt` helper. Go's masks were already saturating and are unchanged,
now pinned by tests. No API change and no new error.

**Files changed:**

- `crates/iscc-lib/src/cdc.rs`: `alg_cdc_params` u64 max + `low_bits_mask` helper (doc comments
    state the saturation); tests `test_alg_cdc_params_large_avg_matches_reference`,
    `test_alg_cdc_chunks_large_avg_single_chunk`, `test_low_bits_mask_saturates`
- `packages/go/cdc.go`: `algCdcParams` uint64 max, uint32 center, `clampInt`
- `packages/go/cdc_test.go`: `TestCdcParamsLargeAvgMatchesReference` (clamped compare, every arch),
    `TestCdcParamsLargeAvgExactOn64Bit` (skips unless `strconv.IntSize == 64`),
    `TestCdcChunksLargeAvgSingleChunk`, `TestClampIntSaturates`
- `.crap-baseline.json`: refreshed (5 cdc.rs line moves + new `low_bits_mask` entry, CRAP 1.0;
    `alg_cdc_params` stays CRAP 1.0)
- `.claude/agent-memory/advance/MEMORY.md`: CDC params entry

**Verification:**

- Reference cross-check: `uv run --no-project --with iscc-core==1.5.0` `alg_cdc_params` printed
    exactly the four next.md tuples. For 4294967295 the reference `mask_s` is 8589934591 (2^33−1);
    we return 4294967295 (saturated, which tests the same bits because pattern < 2^32)
- Regression proof: a standalone `rustc -C debug-assertions=on` program with the old
    `(avg * 8) as usize` on 536870912 → "attempt to multiply with overflow", exit 101. The new
    `test_alg_cdc_chunks_large_avg_single_chunk` passes in debug
- `cargo test -p iscc-lib`: all suites ok (lib 311 passed); `--release --lib cdc` 21 passed
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` exit 0 (after factoring a
    `type ExpectedParams` alias to satisfy `type_complexity`); `--tests --no-default-features` exit
    0; `cargo check -p iscc-lib --target wasm32-unknown-unknown` exit 0
- `packages/go`: `go vet ./...` and `GOARCH=386 go vet ./...` exit 0; `go test -count=1 ./...` ok;
    `GOARCH=386 go test -run 'Cdc|Clamp'` ok (the exact-64-bit test SKIPs there, as designed)
- `mise run bench:iai:check`: "OK: 16 benchmark(s) within 10% of baseline (Ir)", IAI_EXIT=0 (no iai
    baseline change)
- `mise run coverage` + CI line
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`:
    exit 0, "0 regressed, 0 new, 0 moved, 158 unchanged" against the refreshed baseline
- `mise run check`: exit 0 (the first run had cargo-fmt reflow the test table; rerun clean, and the
    coverage/baseline above were run after that reflow)

**Next:** the pending `data-edge.json` fan-out — add an `alg_cdc_chunks` section with the boundary
vectors (e.g. 2^29, 2147483647, 2^31, 4294967295 on a small input → one chunk) across all 12
harnesses and the C generator, as one unsplittable step (learnings 226).

**Notes:**

- Deviation from Implementation Notes ("`min`, `offset`, `center` cannot overflow; keep their
    formulas"): true for Rust, not for Go on 32-bit. Go's old `int(avgSize) - offset` gives
    `int(4294967295) = -1` on 386 (center 2684354560 > MaxInt32). Go now subtracts in `uint32` and
    clamps, which is the same as the reference because `cs` is only used as `min(cs, size)`. Rust
    keeps the formula: `usize` is unsigned and at least 32 bits, so 2684354560 fits on wasm32
- `max_size` saturation equals the reference on 32-bit targets for the same reason (`min(ma, size)`
    with `size <= buffer.len()`)
- Binding crates untouched: they pass the `u32` through
