# Next Work Package

## Step: Add bench_sum_code criterion benchmark for gen_sum_code_v0

## Goal

Add the missing criterion benchmark for `gen_sum_code_v0` to complete the target requirement that
"criterion benchmarks exist for all Rust `gen_*_v0` functions". This is the last gap before full
target completion — all other sections (core, bindings, docs, README, CI) are already met.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/benches/benchmarks.rs` (add `bench_sum_code` function, update
    docstring from "9" to "10", add import, register in `criterion_group!`),
    `crates/iscc-lib/Cargo.toml` (add `tempfile` dev-dependency if not already present)
- **Reference**: `crates/iscc-lib/src/lib.rs` (for `gen_sum_code_v0` signature:
    `path: &Path,   bits: u32, wide: bool`), `Cargo.toml` (root workspace dependencies)

## Not In Scope

- Adding benchmarks for `gen_sum_code_v0` to the Python benchmark suite (`benchmarks/python/`)
- Updating `docs/benchmarks.md` with sum-code speedup numbers (no Python baseline exists yet)
- Updating the "Bench (compile check)" CI job config — it already compiles all bench targets
- Running actual benchmarks and collecting timing data
- Working on issue #16 (feature flags for minimal builds)

## Implementation Notes

`gen_sum_code_v0` takes `path: &std::path::Path` (not `&[u8]`), so the benchmark must write temp
files. Follow the same throughput pattern as `bench_data_code` and `bench_instance_code`:

1. **Import**: Add `gen_sum_code_v0` to the `use iscc_lib::{...}` block. Add `use std::io::Write`
    and `use tempfile::NamedTempFile` (or `std::env::temp_dir` + manual file creation).

2. **Temp file approach**: Use `tempfile::NamedTempFile` if the `tempfile` crate is already a
    dev-dependency. If not, add it. Check `Cargo.toml` dev-dependencies first. If adding `tempfile`
    feels heavyweight, an alternative is `std::env::temp_dir()` with a unique filename — but
    `tempfile` is the idiomatic Rust choice and auto-cleans up.

3. **Benchmark function** `bench_sum_code`:

    - Create a benchmark group `"gen_sum_code_v0"`
    - Two sizes: 64KB and 1MB (matching `bench_data_code` pattern)
    - For each size: write `deterministic_bytes(size)` to a temp file, set
        `group.throughput(Throughput::Bytes(...))`, bench `gen_sum_code_v0(&path, 64, false)`
    - The temp file must be created OUTSIDE the benchmark closure (file I/O setup is not part of what
        we're measuring). Use `NamedTempFile::new()` + `write_all()` before the `bench_with_input`
        call, then pass the path into the closure.

4. **Docstring**: Update line 1 from "9 `gen_*_v0`" to "10 `gen_*_v0`"

5. **Registration**: Add `bench_sum_code` to the `criterion_group!` macro, after `bench_iscc_code`
    and before `bench_data_hasher_streaming`.

6. **Dev dependency**: If `tempfile` isn't already in dev-dependencies, add it to the workspace
    `Cargo.toml` under `[workspace.dependencies]` and to `crates/iscc-lib/Cargo.toml` under
    `[dev-dependencies]`. Use a recent stable version (e.g., `tempfile = "3"`).

## Verification

- `cargo bench -p iscc-lib --bench benchmarks --no-run` exits 0 (benchmark compiles)
- `grep -c 'gen_sum_code_v0' crates/iscc-lib/benches/benchmarks.rs` returns at least 2 (import +
    function usage)
- `grep 'all 10' crates/iscc-lib/benches/benchmarks.rs` matches the updated docstring
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo test -p iscc-lib` still passes (310 tests)

## Done When

All verification criteria pass — the benchmark file compiles with 10 `gen_*_v0` benchmarks, clippy
is clean, and existing tests still pass.
