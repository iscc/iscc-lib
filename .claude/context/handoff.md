## 2026-02-23 — Add benchmark results documentation page

**Done:** Created `docs/benchmarks.md` with measured speedup factors for all 9 `gen_*_v0` functions,
derived from actual pytest-benchmark and Criterion runs. Updated `zensical.toml` nav to include the
benchmarks page. Documentation site builds successfully.

**Files changed:**

- `docs/benchmarks.md`: Created benchmark documentation page with methodology, inputs table, Python
    comparison results (iscc-core vs iscc-lib), native Rust Criterion results with throughput, key
    findings, and reproduction commands
- `zensical.toml`: Added `"benchmarks.md"` to nav array after `"api.md"`

**Verification:**

- `docs/benchmarks.md` exists with measured speedup factors for all 9 functions ✓
- `zensical.toml` nav includes `benchmarks.md` ✓
- `uv run zensical build` succeeds without errors ✓
- `site/benchmarks/index.html` exists in built output (29,834 bytes) ✓
- Comparison table contains iscc-core vs iscc-lib timings from actual benchmark runs ✓
- Speedup factors range from 1.3× (Video-Code) to 158× (Data-Code) ✓
- `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all pass ✓

**Next:** The target criterion "Speedup factors published in documentation" is now fulfilled. The
project state should be updated to reflect that all target criteria are met. Consider reviewing
whether any remaining target items need attention, or if the project is ready for an initial
release.

**Notes:** The Python venv is at `/home/dev/.venvs/iscc-lib` (not `.venv` in the project root) —
used `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib` for maturin. Video-Code shows only 1.3× speedup via
Python bindings because PyO3 overhead dominates the 2 µs native Rust execution time (converting
nested Python lists to Rust vectors is expensive relative to the fast computation). Instance-Code
shows a moderate 4× because iscc-core already uses a C-backed BLAKE3 hasher.
