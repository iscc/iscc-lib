# Update-State Agent Memory — Archive

Archived iteration-by-iteration findings from completed project phases. Moved here to reduce
per-invocation context loading. Full history preserved in git.

See MEMORY.md for current active entries. See git log for the complete historical record of all
archived entries.

## Closed milestones

- **Core SumHasher (iteration 88, `3fc44d2`)**: `pub struct SumHasher` in `streaming.rs:157`
    (new/update/finalize(bits,wide,add_units)/Default). `gen_sum_code_v0` (lib.rs:997) drives it.
    Reachable as `iscc_lib::streaming::SumHasher` but NOT a crate-root re-export (only
    `DataHasher`/`InstanceHasher` are, lib.rs:24). SumHasher NOT promoted to Tier 1 — counts stay
    32\.
- **Module visibility (iteration 86, `3f6a61d`)**:
    `cdc/conformance/dct/minhash/simhash/utils/wtahash` = `pub(crate) mod`; only
    `codec/streaming/types` = `pub mod`. Issue swept.
- **PyO3 0.23→0.29 migration hop history (issue #1, iters 98-105)**: incremental one-minor-per-step,
    completed iter 105. Edits per hop: 0.23→0.24, 0.24→0.25 = ZERO source edits; 0.25→0.26 =
    `allow_threads`→`detach` (7 sites) + `PyObject`→`Py<PyAny>` (17 sites); 0.26→0.27 (`acf9277`) =
    `downcast`→`cast` / `downcast_into_unchecked`→`cast_into_unchecked` (2 sites in `to_pylist`);
    0.27→0.28 (iter 104) = compiled CLEAN but SILENT `#[pymodule] gil_used` default flip
    `true`→`false`, restored with explicit `gil_used = true` (lib.rs:697); 0.28→0.29 (iter 105
    `8df611f`) = ZERO source edits, gil_used default did NOT flip again. Recipe: bump pin →
    `cargo update -p pyo3` → build/clippy(-D warnings)/fmt → `uv run maturin develop` →
    `uv run pytest` (286). 0.29 ships both RustSec advisory fixes; issue #1 closed.
- **cargo-crap install flake mechanism (iter 100, FIXED iter 101 `628c5d9`)**: the install step
    lacked `--force`; `Swatinem/rust-cache@v2` (ci.yml:304) restored cargo's `.crates` metadata
    WITHOUT the binary, so binstall SKIPPED ("already installed") and `crap` died (exit 101),
    recurring every run after the first green one. Fix = `--force` on the binstall.
