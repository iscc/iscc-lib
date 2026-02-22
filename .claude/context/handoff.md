## 2026-02-22 — Review of: Add Rust criterion benchmarks for all gen functions

**Verdict:** PASS

**Summary:** Criterion benchmarks for all 9 `gen_*_v0` functions were added cleanly. The benchmark
file is well-structured with proper benchmark groups, descriptive names, `black_box` usage, and
throughput metrics for streaming operations. All verification criteria from next.md pass: 143 tests
green, clippy clean (including bench code), fmt clean, no `unsafe`, and all benchmarks produce
timing output.

**Issues found:**

- (none)

**Next:** Python reference benchmarks via pytest-benchmark (Phase 0 second half per
`notes/09-performance-benchmarks.md`). This would benchmark the same 9 operations using `iscc-core`
to establish the Python baseline, enabling speedup factor computation. Alternatively, Node.js
bindings (`@iscc/lib` via napi-rs) could be started as the next major deliverable from the target.

**Notes:** Criterion 0.5.1 is used (not 0.8.x). The `--quick` flag only works with
`--bench benchmarks` (not bare `cargo bench`). Benchmark throughput numbers in containerized
environment: Data-Code ~1.3 GiB/s, Instance-Code ~3.6-4.1 GiB/s. These will differ significantly on
bare metal. Phase 0 Rust benchmarks are now complete; the benchmark infrastructure per `notes/09`
envisions additional phases (Python reference, bindings, Node.js, WASM, report generation, CI
tracking) that can be tackled incrementally.
