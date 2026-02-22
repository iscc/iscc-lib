## 2026-02-22 — Add Rust criterion benchmarks for all gen functions

**Done:** Added criterion benchmarks covering all 9 `gen_*_v0` functions. Each function has at least
one benchmark with representative inputs (conformance-derived strings, synthetic data, gradient
pixels). Streaming functions (data_code, instance_code) include 64KB and 1MB variants to show
throughput scaling.

**Files changed:**

- `Cargo.toml`: Added `criterion = { version = "0.5", features = ["html_reports"] }` to
    `[workspace.dependencies]`
- `crates/iscc-lib/Cargo.toml`: Added criterion dev-dependency and `[[bench]]` section
- `crates/iscc-lib/benches/benchmarks.rs`: New file — criterion benchmarks for all 9 gen functions

**Verification:** All checks pass:

- `cargo bench -p iscc-lib --bench benchmarks` completes with timing output for all 9 functions
- `cargo test -p iscc-lib` — 143 tests pass (benchmarks don't interfere)
- `cargo clippy -p iscc-lib --benches -- -D warnings` — clean
- `cargo fmt -p iscc-lib -- --check` — clean
- No `unsafe` code in the benchmark file

Sample benchmark results (containerized environment, not representative of bare-metal):

- gen_meta_code_v0: 22-41 µs
- gen_text_code_v0: 137 µs (1000 chars)
- gen_image_code_v0: 317 µs
- gen_audio_code_v0: 47 µs
- gen_video_code_v0: 1.9 µs
- gen_mixed_code_v0: 1.6 µs
- gen_data_code_v0: 1.2-1.5 GiB/s throughput
- gen_instance_code_v0: 3.7-4.2 GiB/s throughput
- gen_iscc_code_v0: 2.7 µs

**Next:** Phase 0 benchmarks are complete for Rust. Next logical steps: Python reference benchmarks
via pytest-benchmark (to compute speedup factors), or Node.js bindings (`@iscc/lib` via napi-rs) as
defined in the target.

**Notes:** criterion 0.5.1 was used (not 0.8.x which would require different API). The `--quick`
flag only works with `--bench benchmarks` (not bare `cargo bench`) since the built-in test harness
doesn't recognize criterion flags. Benchmark names follow the pattern `gen_*_code_v0/description`
for easy identification.
