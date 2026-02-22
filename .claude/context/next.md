# Next Work Package

## Step: Add Rust criterion benchmarks for all gen functions

## Goal

Establish Rust-side performance benchmarks using criterion for all 9 `gen_*_v0` functions. This is
Phase 0 of the benchmark plan and provides the first quantitative performance data for the Rust
core, which is a key deliverable of the project.

## Scope

- **Create**: `crates/iscc-lib/benches/benchmarks.rs` — criterion benchmark file covering all 9 gen
    functions
- **Modify**: `Cargo.toml` (workspace root) — add `criterion` to `[workspace.dependencies]`
- **Modify**: `crates/iscc-lib/Cargo.toml` — add `criterion` dev-dependency and `[[bench]]` section
- **Reference**: `notes/09-performance-benchmarks.md`, `crates/iscc-lib/src/lib.rs`,
    `crates/iscc-lib/tests/data.json`

## Implementation Notes

1. **Add criterion dependency**: Add `criterion = { version = "0.5", features = ["html_reports"] }`
    to `[workspace.dependencies]` in root `Cargo.toml`. In `crates/iscc-lib/Cargo.toml`, add
    `criterion = { workspace = true }` to `[dev-dependencies]` and add:

    ```toml
    [[bench]]
    name = "benchmarks"
    harness = false
    ```

2. **Benchmark file structure**: Create a single `benchmarks.rs` file using `criterion_group!` and
    `criterion_main!`. Group benchmarks by operation type. Use representative inputs extracted from
    the conformance vectors (`data.json`) or simple synthetic data.

3. **Benchmark inputs** (use inline/const data — no external files needed):

    - `gen_meta_code_v0`: name="Die Unendliche Geschichte", description="Von Michael Ende", bits=64
    - `gen_text_code_v0`: a ~1000-char synthetic text string, bits=64
    - `gen_image_code_v0`: 1024-byte pixel array (gradient pattern), bits=64
    - `gen_audio_code_v0`: 300-element i32 vector (sequential values), bits=64
    - `gen_video_code_v0`: 10 frames of 380-element i32 vectors, bits=64
    - `gen_mixed_code_v0`: 2 Content-Code strings from conformance tests, bits=64
    - `gen_data_code_v0`: 64KB deterministic byte buffer, bits=64
    - `gen_instance_code_v0`: 64KB deterministic byte buffer, bits=64
    - `gen_iscc_code_v0`: array of 4 ISCC unit strings from conformance tests, wide=false

4. **For streaming operations** (data/instance): also add a 1MB variant to show throughput scaling.
    Generate the buffer deterministically (e.g., repeating byte pattern).

5. **Use `criterion::black_box`** on inputs to prevent compiler optimization of benchmark inputs.

6. **Import paths**: `use iscc_lib::{gen_meta_code_v0, gen_text_code_v0, ...}` — all gen functions
    are pub exports from the crate root.

7. **Do NOT add benchmark data files** — use inline data for this step. The `benchmarks/data/`
    directory structure from notes/09 can come later.

## Verification

- `cargo bench -p iscc-lib` runs successfully and produces timing output for all 9 gen functions
- `cargo test -p iscc-lib` still passes (143 tests — benchmarks don't interfere with tests)
- `cargo clippy -p iscc-lib -- -D warnings` still clean (including bench code)
- No `unsafe` code in the benchmark file
- Benchmark names are descriptive (e.g., `gen_meta_code_v0/name+desc`, `gen_data_code_v0/64KB`)

## Done When

`cargo bench -p iscc-lib` completes without errors, reporting timing results for all 9 `gen_*_v0`
functions, and all existing tests and quality gates continue to pass.
