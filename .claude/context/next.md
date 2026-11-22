# Next Work Package

## Step: One-shot gen functions stop re-implementing their streaming counterparts

## Goal

Picks up the `normal` `[audit]` issue "One-shot gen functions re-implement their streaming
counterparts": each of the three cited computations (BLAKE3 multihash, the WASM sum composition, the
meta JSON parse + canonicalize) exists once, and `gen_instance_code_v0` hashes its input once.

## Alternatives Considered

- **Chosen:** one-shot dedup — the handoff's pick, 4 non-test files, and it removes a real 2× BLAKE3
    cost on the instance hot path. Of the four `normal` issues, only this one and the parser one can
    be done without a decision from Titusz
- **Rejected:** the Rust half of "Rust and Go per-function tests re-parse `data.json`" — also needs
    a malformed-vector policy that both runners share, so it is the bigger step. It comes next
- `alg_minhash_256([])` and WASM `finalize()` wait for Titusz's behaviour decision (handoff)

## Scope

- **Modify**: `crates/iscc-lib/src/utils.rs`, `crates/iscc-lib/src/streaming.rs`,
    `crates/iscc-lib/src/lib.rs`, `crates/iscc-wasm/src/lib.rs`
- **Reference**: the issue body in issues.md; `crates/iscc-wasm/src/lib.rs` → `SumHasher` wrapper
    (around line 782) and `gen_sum_code_v0` (around line 348); `.iai-baseline.json`

## Not In Scope

- `gen_data_code_v0` vs `DataHasher`: not cited by the issue. The one-shot path chunks the whole
    slice without the tail buffer copy, so delegating would cost Ir. Leave it alone
- No `.iai-baseline.json` refresh (the baseline is 3–8% high from toolchain drift and a refresh is a
    human call, state.md). An Ir *drop* on the instance benches passes the check as is
- No change to any public error text, signature or result field; no binding other than WASM
- The other one-shot functions in `iscc-napi`/`iscc-py`/… already call the core; do not touch them

## Implementation Notes

1. **One multihash formatter.** In `utils.rs`, add a `pub(crate)` helper that formats an existing
    `blake3::Hash` as the multihash hex (`1e` + `20` + digest). `multi_hash_blake3(data)` becomes
    `helper(&blake3::hash(data))`. `InstanceHasher::finalize` (its `format!("1e20{}", …)`) and
    `gen_instance_code_v0` both call the helper on the digest they already have. After this, the
    `1e`/`20` prefix bytes are written in one place only, and `gen_instance_code_v0` calls
    `blake3::hash` exactly once and `multi_hash_blake3` not at all
2. **WASM sum.** `iscc_wasm::gen_sum_code_v0` builds one `iscc_lib::streaming::SumHasher`, calls
    `update(data)` and `finalize(bits, wide, add_units)`, and maps `SumCodeResult` into
    `WasmSumCodeResult` (`filesize as f64`). Delete the hand copy and its comment. Keep the
    `opt_u32`/`opt_bool` argument checks and their order. Update the doc comment (it names the two
    inner hashers)
3. **Meta parse once.** Today `gen_meta_code_v0` parses a JSON `meta` in `parse_meta_json` and
    parses it again to build the data URL, and `json_to_data_url` repeats the parse+canonicalize
    body. Make one private helper that parses and canonicalizes once and returns both the canonical
    bytes and what `build_meta_data_url` needs (the parsed `Value`, or just the `@context` flag).
    `gen_meta_code_v0` keeps that result and builds `meta` from it with no second
    `serde_json::from_str`; `json_to_data_url` becomes helper + `build_meta_data_url`. **Keep both
    error texts byte-for-byte**: `gen_meta_code_v0` says `invalid JSON in meta: …` (Go mirrors it
    in `packages/go/code_meta.go`), `json_to_data_url` says `invalid JSON: …`, and both say
    `JSON canonicalization failed: …`. The helper may return the raw serde error and let each
    caller map it, or take the prefix; your choice. Mind the `meta-code` feature gate on every new
    item, and that the pre-/post-decode size checks keep their order (size check on the canonical
    bytes before hashing)
4. **Tests** (outside the budget): one Rust unit test that `gen_instance_code_v0(d).datahash` equals
    `utils::multi_hash_blake3(d)` and the `InstanceHasher` result for a few inputs incl. empty; one
    that `json_to_data_url(j)` equals the `meta` field `gen_meta_code_v0` returns for the same JSON
    (with and without `@context`); one `#[wasm_bindgen_test]` in `crates/iscc-wasm/tests/unit.rs`
    that `gen_sum_code_v0` with `add_units = true` and `wide = true` equals the WASM `SumHasher`
    result on the same bytes. Existing tests must pass unchanged — editing an existing assertion is
    a red flag
5. **Gates that fail in CI only.** `gen_instance_code_v0` and `gen_meta_code_v0` are benchmarked
    (iai) and covered (CRAP). `valgrind`, `iai-callgrind-runner` and `wasm-pack` are installed
    here. Removing branches only lowers CRAP; if the CRAP gate reports a regression anyway (e.g.
    the helper split), refresh with `mise run coverage && mise run crap:baseline` in this step

## Verification

- `cargo test -p iscc-lib`, plus the two CI feature legs (`--no-default-features`, and
    `--no-default-features --features text-processing`) pass;
    `cargo clippy -p iscc-lib --no-default-features -- -D warnings` and
    `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean
- `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including the new sum test
- `mise run bench:iai:check` exits 0
- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info && cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0
- `sed -n '/^pub fn gen_instance_code_v0/,/^}/p' crates/iscc-lib/src/lib.rs | grep -c multi_hash_blake3`,
    the same for `serde_json::from_str` in `gen_meta_code_v0`, and
    `grep -c 'instance_hasher\|1e20{' crates/iscc-wasm/src/lib.rs crates/iscc-lib/src/streaming.rs`
    all print 0 (per file)
- `mise run check` exits 0

## Done When

All six verification criteria pass with the issue's three duplications each reduced to one
implementation and no public error text or result changed.
