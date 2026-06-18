# Next Work Package

## Step: Add the iai-callgrind bench harness (compiles locally; CI gate is a follow-up slice)

## Goal

Land the `iai-callgrind` instruction-count benchmark harness for `iscc-lib`'s hot paths so the
v1.0.0 performance-regression gate has benches to run. This is the unblocked first slice of the
`iai-callgrind` perf-gate issue (#3): only *running* the benches needs valgrind (absent in the
devcontainer), while the harness itself compiles and is fully verifiable locally. The gate is
already mandated by `target.md`, `rust-core.md`, and `ci-cd.md`, so no spec amendment is needed.

## Scope

- **Create**: `crates/iscc-lib/benches/iai_benches.rs` — iai-callgrind library benchmarks for the
    hot CPU paths.
- **Modify**: `Cargo.toml` (root) — add `iai-callgrind` to `[workspace.dependencies]`.
- **Modify**: `crates/iscc-lib/Cargo.toml` — add the `iai-callgrind` dev-dependency and a
    `[[bench]]` entry (`name = "iai_benches"`, `harness = false`).
- **Reference**: `crates/iscc-lib/benches/benchmarks.rs` (working input builders to mirror);
    `.claude/context/specs/ci-cd.md` -> "Performance - `iai-callgrind`";
    `.claude/context/specs/rust-core.md` -> "Performance parity or improvement";
    `crates/iscc-lib/src/cdc.rs` and `crates/iscc-lib/src/minhash.rs` (primitive signatures).

## Not In Scope

- Do NOT add the `Perf` CI job, commit a baseline file, or run the benches - running needs valgrind
    (absent locally) and is the next slice; the review agent verifies it against CI.
- Do NOT add `mise` tasks (bench run / baseline-refresh) yet - they cannot be verified locally;
    follow-up slice.
- Do NOT modify or delete the existing criterion `benchmarks.rs` - it stays for local wall-clock
    profiling and human-facing speedup numbers.
- Do NOT touch `.cargo-crap.toml` - its `crates/iscc-lib/benches/**` exclusion already covers the
    new bench file.
- Do NOT flip the `Semver` gate to enforcing or begin v1.0.0 release prep (both human-driven).

## Implementation Notes

- Pin the latest stable `iai-callgrind` in `[workspace.dependencies]` (confirm the current version
    on crates.io, e.g. `iai-callgrind = "0.14"`), then reference it from `iscc-lib` as a
    dev-dependency (`iai-callgrind = { workspace = true }`), mirroring how `criterion` is wired.
- Use the `#[library_benchmark]` macro API: each hot path is a `#[library_benchmark]` fn that
    returns `black_box(...)` of the call result; collect them with `library_benchmark_group!` and
    wire the group into `main!(library_benchmark_groups = ...)`. Confirm the exact macro names
    against the pinned version's README - the API shifted across 0.x minors.
- Use `std::hint::black_box` (NOT `criterion::black_box`) in the iai harness.
- Cover the in-memory CPU hot paths, mirroring representative inputs from `benchmarks.rs`
    (`deterministic_bytes`, `synthetic_text`): `gen_meta_code_v0`, `gen_text_code_v0`,
    `gen_image_code_v0`, `gen_audio_code_v0`, `gen_video_code_v0`, `gen_mixed_code_v0`,
    `gen_data_code_v0`, `gen_instance_code_v0`, `gen_iscc_code_v0`, plus the primitives
    `alg_cdc_chunks(data, false, 1024)` and `alg_minhash_256(&[u32])` (infallible, returns
    `Vec<u8>`).
- DEFER `gen_sum_code_v0` (file I/O - its instruction count is syscall-dominated and would need an
    iai setup closure to create the temp file outside the measured region); note the deferral in the
    file's module docstring.
- The harness needs the default features (`meta-code` -> `text-processing`) for the gen functions,
    exactly like the criterion bench. `cargo build -p iscc-lib --bench iai_benches` (default
    features on) compiles fine; only `--no-default-features --all-targets` would fail - a known
    pre-existing bench limitation that CI never exercises.
- Compilation does NOT require valgrind or `iai-callgrind-runner`; those are runtime-only and the
    follow-up CI slice installs the runner via `cargo binstall` (heed the rust-cache `--force`
    poisoning gotcha) on a valgrind-enabled Linux runner.

## Verification

- `cargo build -p iscc-lib --bench iai_benches` exits 0 (the harness compiles without valgrind or
    the runner installed).
- `cargo clippy -p iscc-lib --benches -- -D warnings` clean.
- `cargo fmt -p iscc-lib --check` clean.
- `cargo test -p iscc-lib` still passes (no regression in the existing suite).
- `grep -q '^iai-callgrind' Cargo.toml` and `grep -q 'iai-callgrind' crates/iscc-lib/Cargo.toml`
    both succeed; `crates/iscc-lib/Cargo.toml` contains a `[[bench]]` with `name = "iai_benches"`
    and `harness = false`.

## Done When

`cargo build -p iscc-lib --bench iai_benches` compiles the new iai-callgrind harness clean and all
existing `iscc-lib` quality gates (clippy, fmt, test) stay green, leaving only the valgrind-gated CI
job plus committed baseline for the clearly-scoped follow-up slice.
