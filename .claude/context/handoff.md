## 2026-06-17 — Migrate PyO3 0.27 → 0.28 (issue #1, incremental toward 0.29)

**Done:** Advanced the workspace `pyo3` pin one minor version (`0.27` → `0.28`, resolves to 0.28.3)
toward the 0.29 target where the two RustSec advisories clear. This hop required **zero source
changes** — only the `Cargo.toml` pin and the regenerated `Cargo.lock`. The raw `pyo3::ffi::*` C-API
sites, `#[pyo3(signature = ...)]` macros, `detach`, and `Py<PyAny>` returns all compiled clean under
`-D warnings` with no new deprecations.

**Files changed:**

- `Cargo.toml`: line 35 workspace pin `pyo3 = { version = "0.27", ... }` → `version = "0.28"`
    (`abi3-py310` preserved)
- `Cargo.lock`: regenerated via `cargo update -p pyo3` — pyo3 + 4 sibling crates (pyo3-build-config,
    pyo3-ffi, pyo3-macros, pyo3-macros-backend) `0.27.2` → `0.28.3`; transitive `indoc`,
    `memoffset`, `unindent` removed (no longer pulled by pyo3 0.28). Generated file, not counted
    toward file scope.

**Verification:** All criteria green.

- `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.28"` with `abi3-py310`
- `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.28.3"`
- `cargo build -p iscc-py` → exit 0
- `cargo clippy -p iscc-py -- -D warnings` → exit 0, clean (no source edits needed)
- `cargo fmt --all --check` → exit 0
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` → exit 0, `cp310-abi3` wheel installed
- `uv run pytest` → 286 passed, 1 warning (the known pre-existing iscc_core Pydantic-V1/Py3.14
    `UserWarning` from `iscc_core/options.py:18`, unrelated to this change)
- `cargo tree -p iscc-py -i pyo3` → single `pyo3 v0.28.3`, no duplicates
- `allow_threads` count in `crates/iscc-py/src/lib.rs` = 0 (no deprecated APIs remain)
- `mise run check` → all 15 pre-commit hooks Passed
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (pre-push defense)

**Next:** Continue the incremental migration **PyO3 0.28 → 0.29** (issue #1) — this is the FINAL
hop, where the two RustSec advisories shipped in the published wheel finally clear (the stated
endpoint of issue #1). Note: `cargo update` already reports `0.29.0` is available. The proven recipe
holds: bump `Cargo.toml` line 35 → `cargo update -p pyo3` → build / clippy (`-D warnings`) / fmt →
`uv run maturin develop` → `uv run pytest` (286). 0.27→0.28 was a clean lockfile-only bump (no
source edits), but 0.28→0.29 is a full major-cycle boundary — treat `-D warnings` as the gate and
port exactly what the compiler flags. After 0.29 lands, verify the RustSec advisories actually clear
(`cargo audit` if available) before closing issue #1.

**Notes:**

- No backward-compat or perf concern: iscc-py is a binding crate (not the stability-committed core),
    pyo3 is an internal dep, and no source/compute path changed at all. The pytest video/text
    benches still show iscc-lib ~3×/1.3× faster than iscc-core.
- Still open and unstarted: CRAP `--fail-above 30` hardening (issue #2 — HUMAN REVIEW REQUESTED on
    the spec; `--fail-regression` lets new uncovered high-CRAP funcs through) and the
    `iai-callgrind` perf-regression CI gate (issue #3 — valgrind unavailable in the devcontainer).
    Both untouched per Not-In-Scope.
- The transitive removals (`indoc`, `memoffset`, `unindent`) in Cargo.lock are pyo3 0.28 dropping
    those build/dev deps — benign, expected.
