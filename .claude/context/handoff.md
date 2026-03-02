## 2026-03-02 — Add bench_sum_code criterion benchmark for gen_sum_code_v0

**Done:** Added the `bench_sum_code` criterion benchmark function for `gen_sum_code_v0`, completing
all 10 `gen_*_v0` benchmarks. The benchmark writes deterministic data to temp files (64KB and 1MB)
and measures throughput, matching the pattern used by `bench_data_code` and `bench_instance_code`.
Added `tempfile` as a workspace and dev-dependency.

**Files changed:**

- `crates/iscc-lib/benches/benchmarks.rs`: Added `gen_sum_code_v0` import, `std::io::Write` and
    `tempfile::NamedTempFile` imports, `bench_sum_code` function with 64KB/1MB throughput
    benchmarks, registered in `criterion_group!`, updated docstring from "9" to "10"
- `crates/iscc-lib/Cargo.toml`: Added `tempfile = { workspace = true }` to dev-dependencies
- `Cargo.toml`: Added `tempfile = "3"` to workspace dependencies

**Verification:**

- `cargo bench -p iscc-lib --bench benchmarks --no-run` — exits 0 (compiles)
- `grep -c 'gen_sum_code_v0' crates/iscc-lib/benches/benchmarks.rs` — returns 5 (≥2 required)
- `grep 'all 10' crates/iscc-lib/benches/benchmarks.rs` — matches updated docstring
- `cargo clippy -p iscc-lib -- -D warnings` — clean
- `cargo test -p iscc-lib` — 310 tests pass (256 + 31 + 22 + 1 doctest)
- `mise run check` — 14/14 hooks pass

**Next:** All 10 `gen_*_v0` benchmarks now exist. The target requirement for criterion benchmarks is
fully met. The next update-state should evaluate whether all target.md criteria are satisfied — this
may be the final gap before target completion.

**Notes:** The `tempfile` crate (v3) was the only new dependency added. Temp files are created
outside the benchmark closure so file I/O setup isn't measured. The `NamedTempFile` handles cleanup
automatically. No surprises during implementation.
