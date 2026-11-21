# Next Work Package

## Step: CDC parameter maths matches the reference for every `avg_chunk_size` (Rust core + Go)

## Goal

Fix the `normal` [audit] issue "`alg_cdc_chunks` accepts an `avg_chunk_size` whose parameter maths
overflows": Rust `alg_cdc_params` and Go `algCdcParams` compute the reference's
`(min, max, center, mask_s, mask_l)` for every `u32` value, with no panic, no wrap, and the same
result on both. This is the core half; the `data-edge.json` vector follows as its own fan-out step.

## Alternatives Considered

- **Chosen:** reference-matching parameter maths in Rust and Go. It is a correctness defect in a
    public Tier 1 function (debug panic, release `mask_s = 0`), and the handoff names it. Matching
    the reference beats an upper-bound rejection: no `diverges` marker, no new error, no API change
- **Rejected:** Python video frame `OverflowError` (1 file, Python-only). Smaller and independent,
    but a binding-local exception class matters less than a core function that panics
- **Rejected:** adding the `alg_cdc_chunks` section to `data-edge.json` in this step. A new section
    makes all 12 harnesses and the C generator fail closed (learnings 226), and "core changes go
    first and alone" forbids combining a core fix with a fan-out

## Scope

- **Modify**: `crates/iscc-lib/src/cdc.rs` (`alg_cdc_params` + its unit tests + the `alg_cdc_chunks`
    doc comment if it mentions limits), `packages/go/cdc.go` (`algCdcParams`)
- **Tests**: `crates/iscc-lib/src/cdc.rs` `mod tests`, `packages/go/cdc_test.go`
- **Maybe**: `.crap-baseline.json` (only if `alg_cdc_params` changes score; via
    `mise run coverage && mise run crap:baseline`), `.iai-baseline.json` (only if `bench:iai:check`
    fails, which is not expected)
- **Reference**: `reference/iscc-core/iscc_core/cdc.py` (`alg_cdc_params`, `alg_cdc_offset`)

## Not In Scope

- The `data-edge.json` `alg_cdc_chunks` section and its 12 harnesses: the next step, as a fan-out
- An upper bound or any new rejection of large `avg_chunk_size`: the reference accepts every value
    from 2 up, and so must we
- The Rust `utf32` `cut_point == 0` fallback (`cut_point = remaining.len().min(4)`), and changes to
    `gear_scan` / `alg_cdc_offset` or their `unsafe` blocks
- Binding crates: they pass the `u32` through unchanged and need no edit

## Implementation Notes

Reference (Python ints): `min = avg // 4`, `max = avg * 8`, `offset = min + ceil(min / 2)`,
`center = avg - offset`, `bits = round(log2(avg))`, `mask_s = 2**(bits+1) - 1`,
`mask_l = 2**(bits-1) - 1`. `bits` reaches 31 from avg ≈ 1518500250 and 32 from ≈ 3037000500.

- **Masks:** `pattern` is `(pattern >> 1) + gear[b]` with every gear value < 2^31, so it stays below
    2^32 in the reference too. A mask with 32 or more bits therefore tests `pattern == 0`, the same
    as `u32::MAX`. Saturate: Rust `1u32.checked_shl(n).map_or(u32::MAX, |v| v - 1)` (`checked_shl`
    returns `None` for `n >= 32`). `mask_l` needs at most 31 bits, but use the same helper for both.
    **Go already saturates**: in `maskS = (1 << (bits+1)) - 1` the constant takes type `uint32`, and
    a Go shift of 32 or more gives 0, so 0 − 1 = `0xFFFFFFFF`. Leave Go's masks alone; pin them with
    a test
- **`max_size`:** `avg * 8` overflows `u32` from 2^29 (Rust: debug panic; Go: wrap). Compute it in
    `u64` and saturate to the platform int (Rust `usize::try_from(..).unwrap_or(usize::MAX)`; Go
    `int64` clamped to `math.MaxInt`). `ma` is only ever used as `min(ma, size)` with
    `size <= buffer.len()`, so saturation equals the reference on 32-bit targets (wasm32, Go 386)
- `min`, `offset`, `center` cannot overflow; keep their formulas
- Expected tuples `(min, max, center, mask_s, mask_l)`, computed from the reference formulas:
    - 2^29 = 536870912 → (134217728, 4294967296, 335544320, 1073741823, 268435455)
    - 2147483647 → (536870911, 17179869176, 1342177280, 4294967295, 1073741823)
    - 2^31 = 2147483648 → (536870912, 17179869184, 1342177280, 4294967295, 1073741823)
    - 4294967295 → (1073741823, 34359738360, 2684354560, 4294967295 saturated, 2147483647)
- Tests, the same four values on both surfaces: a parameter test asserting the tuples above (gate
    the `max` assertion on a 64-bit `usize`/`int`, e.g. `#[cfg(target_pointer_width = "64")]` /
    `strconv.IntSize == 64`), and an `alg_cdc_chunks` / `AlgCdcChunks` call on a small input (say 64
    bytes) returning exactly one chunk equal to the input. In a debug build that call panicked
    before the fix, so the Rust test is the regression proof
- Hot path: `alg_cdc_params` runs once per `alg_cdc_chunks` call. Run `mise run bench:iai:check`
    (CI-only gate) and the CI CRAP lines; if either moves, regenerate the baseline in this step,
    never widen epsilon or threshold

## Verification

- `cargo test -p iscc-lib` passes, including the new tests for the four values above
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean and
    `cargo check -p iscc-lib --target wasm32-unknown-unknown` exits 0
- `go test ./...` and `go vet ./...` in `packages/go` exit 0, including the new tests;
    `GOARCH=386 go vet ./...` exits 0
- `mise run bench:iai:check` exits 0
- `mise run coverage` then the CI `cargo crap` command (copied from `ci.yml`) exits 0 against the
    committed `.crap-baseline.json`
- `mise run check` exits 0

## Done When

Rust and Go return the reference's CDC parameters for 2^29, 2147483647, 2^31 and 4294967295 without
a panic or wrap, all six criteria pass, and the handoff names the pending `data-edge.json` fan-out.
