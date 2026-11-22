## 2026-11-22 — Review of: One-shot gen functions stop re-implementing their streaming counterparts

**Verdict:** PASS

**Summary:** The three duplicated computations of the `[audit]` issue each exist once now:
`utils::blake3_multihash_hex` writes the multihash prefix, WASM `gen_sum_code_v0` calls the core
`SumHasher`, and `canonicalize_json` parses JSON meta once. `gen_instance_code_v0` hashes its input
once, which halves its instruction count. Four non-test files, no public signature, error text or
result changed. The issue is closed and deleted.

**Verification:**

- [x] `cargo test -p iscc-lib` and both feature legs pass — 450 / 384 (`--no-default-features`) /
    417 (`text-processing`) passed, 0 failed; both clippy lines clean (also with `--tests` on the
    no-default leg)
- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` — 11 + 8 + 127 passed, the new
    `test_gen_sum_code_v0_matches_sum_hasher_wide_with_units` among them
- [x] `mise run bench:iai:check` exits 0 — `bench_instance_code.bytes_1m` 5,236,470 → 2,621,027 Ir
    (−49.95%), `bytes_64k` −49.22%; the other 14 rows between +0.00% and −7.65% (known drift)
- [x] `cargo llvm-cov … && cargo crap … --fail-regression --fail-above` exits 0 — 0 regressed, 2
    improved, 2 new, 31 moved, 1 removed (`parse_meta_json`)
- [x] The three greps — `multi_hash_blake3` in `gen_instance_code_v0` = 0, `serde_json::from_str` in
    `gen_meta_code_v0` = 0, `instance_hasher\|1e20{` in `crates/iscc-wasm/src/lib.rs` = 0. In
    `streaming.rs` the same pattern prints 11, not 0: it matches `SumHasher`'s own `instance_hasher`
    field (the one implementation that is meant to stay) and seven test names. The count was 12
    before the step, and `1e20{` alone is 0. The criterion was written too broadly; its intent is
    met
- [x] `mise run check` exits 0

**Probes beyond next.md (1 of 3 used):** one program run against the core at `HEAD~1` and at `HEAD`
with 27 `meta` inputs × 3 `bits` values, `json_to_data_url` on each, and instance inputs through
both the one-shot function and `InstanceHasher`. The 134 output lines are identical (57 `Ok`, and
every error arm: both parse prefixes, both size limits, base64, recursion limit, invalid `bits`).

**Issues found:**

- Filed `low`, untriaged: "iai baseline for `bench_instance_code` is twice the measured cost". The
    perf check fails only above +10% of the baseline, so with the old rows still in
    `.iai-baseline.json` a second BLAKE3 pass could come back unnoticed. The step left the baseline
    alone as next.md asked. Every refresh so far was a `cid(human)` commit, so this one is for
    Titusz
- Not filed (nit): `gen_meta_code_v0` builds the data URL before the post-decode size check, so an
    oversized JSON payload pays one base64 encode before it is rejected. The pre-decode limit bounds
    that at about 171 kB
- `multi_hash_blake3` is gated `#[cfg(any(feature = "meta-code", test))]`. That is correct scoping
    (Meta-Code is its only non-test caller), not a lint suppression
- Two open issues cited line numbers that this step shifted. They name the functions now

**Codex review:** no actionable findings. Codex ran the core tests under the three feature
configurations and the WASM suite and reported that outputs and error handling are preserved.

**Next:** the Rust half of "Rust and Go per-function tests re-parse `data.json` beside the
conformance runners" (`normal`, `[audit]`, up to 8 files). It includes settling one malformed-vector
policy that the Rust and Go runners share (bare hex without `stream:`, missing `iscc` output). Of
the three `normal` issues left, it is the only one the loop can do without Titusz: both
`alg_minhash_256([])` and the WASM `finalize()` rule wait for his decision.

**Notes:**

- Do not write a `grep -c` criterion whose pattern also matches the code that is meant to stay (here
    the struct field of the one remaining implementation). Test the pattern on the current tree
    first and count what must go
- The CI `perf` job uploads its run as the `iai-baseline` artifact. A baseline refresh from it would
    also remove the 3–8% drift on nine other rows
- No decision recorded: the step chose between no real alternatives
