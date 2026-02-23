# Next Work Package

## Step: Add benchmark results documentation page

## Goal

Create a documentation page showing performance comparison between iscc-lib (Rust) and iscc-core
(Python reference), with actual measured speedup factors. This fulfills the target criterion
"Speedup factors published in documentation."

## Scope

- **Create**: `docs/benchmarks.md`
- **Modify**: `zensical.toml` (add `benchmarks.md` to nav array)
- **Reference**: `benchmarks/python/bench_iscc_core.py`, `benchmarks/python/bench_iscc_lib.py`,
    `crates/iscc-lib/benches/benchmarks.rs`, `docs/index.md` (style reference),
    `benchmarks/python/conftest.py`

## Implementation Notes

### 1. Run comparative benchmarks

First ensure iscc-lib Python bindings are built in release mode:

```bash
VIRTUAL_ENV=/workspace/iscc-lib/.venv maturin develop \
    --manifest-path crates/iscc-py/Cargo.toml --release
```

Then run the comparative pytest-benchmark suite:

```bash
pytest benchmarks/python/ --benchmark-only --benchmark-columns=mean,stddev,rounds
```

This will run both `bench_iscc_core.py` (Python reference) and `bench_iscc_lib.py` (Rust bindings)
with matching group names, enabling direct comparison. Capture the output for the docs page.

Also run `cargo bench -p iscc-lib` to get absolute Rust-native criterion numbers (these are faster
than the Python-binding numbers since they skip PyO3 overhead).

### 2. Create docs/benchmarks.md

Structure the page as:

- **Title**: "Benchmarks"
- **Intro**: Brief explanation — Rust core vs Python reference, what's measured
- **Methodology section**: Note the devcontainer environment, inputs used (summarize from bench
    files: text sizes, byte sizes, feature vector lengths, etc.), measurement tools (criterion for
    Rust, pytest-benchmark for Python comparison)
- **Results table**: One main table showing all 9 functions with columns:
    - Function name
    - Input description
    - iscc-core (Python) mean time
    - iscc-lib (Rust via Python) mean time
    - Speedup factor (X×)
- **Key findings**: Brief summary of speedup range (e.g., "10–200× across functions")
- **How to reproduce**: Commands to run benchmarks locally

Style guidelines (match existing docs):

- Use Material for MkDocs features (admonitions, tables)
- Keep it factual — report measured numbers, note that results vary by hardware
- Use `!!! info` or `!!! tip` for important callouts
- No JSON return value assumptions — functions return ISCC code strings

### 3. Update zensical.toml nav

Add `"benchmarks.md"` to the nav array after `"api.md"`:

```toml
nav = ["index.md", "architecture.md", "rust-api.md", "api.md", "benchmarks.md"]
```

### 4. Verify docs build

Run `uv run zensical build` to confirm the new page renders and the site builds successfully. Check
that `site/benchmarks/index.html` exists in the output.

## Verification

- `docs/benchmarks.md` exists with measured speedup factors for all 9 `gen_*_v0` functions
- `zensical.toml` nav includes `benchmarks.md`
- `uv run zensical build` succeeds without errors
- `site/benchmarks/index.html` exists in built output
- The benchmarks page contains a comparison table with iscc-core vs iscc-lib timings
- Speedup factors are derived from actual benchmark runs (not placeholder values)

## Done When

The advance agent is done when all verification criteria pass — the benchmarks page contains real
measured speedup factors for all 9 functions and the documentation site builds successfully.
