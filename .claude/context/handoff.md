## 2026-02-22 — Fix CI — add maturin dependency and rename Python module to iscc_lib

**Done:** Added `maturin` to root `pyproject.toml` dev dependencies so CI can build Python bindings,
and renamed the Python module from `iscc` to `iscc_lib` to match the PyPI package name. Updated all
imports across `__init__.py`, type stubs, and test files.

**Files changed:**

- `pyproject.toml`: Added `"maturin"` to `[dependency-groups] dev` list
- `crates/iscc-py/pyproject.toml`: Changed `module-name` from `iscc._lowlevel` to
    `iscc_lib._lowlevel`
- `crates/iscc-py/python/iscc/` → `crates/iscc-py/python/iscc_lib/`: Renamed module directory via
    `git mv`
- `crates/iscc-py/python/iscc_lib/__init__.py`: Updated import from `iscc._lowlevel` to
    `iscc_lib._lowlevel`
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Updated docstring module reference
- `tests/test_conformance.py`: Changed `from iscc import` to `from iscc_lib import`
- `tests/test_smoke.py`: Changed `from iscc import` to `from iscc_lib import`
- `uv.lock`: Auto-updated by `uv sync` after adding maturin dependency

**Verification:**

- `uv run maturin --version` → maturin 1.12.4 ✓
- `maturin develop` builds wheel named `iscc_lib-0.1.0` ✓
- `pytest` → 49 passed (46 conformance + 3 smoke) ✓
- `cargo test -p iscc-lib` → 143 passed ✓
- `cargo clippy -- -D warnings` → clean ✓
- `cargo fmt --check` → clean ✓
- `python -c "import iscc_lib"` → succeeds ✓
- `python -c "import iscc"` → ModuleNotFoundError ✓
- No stale `.so` or `__pycache__` files tracked in git ✓

**Next:** CI should now pass with both Rust and Python jobs succeeding. Next priorities from
target.md would be Node.js bindings (`@iscc/lib` via napi-rs) or WASM bindings, but first verify the
CI workflow actually passes on GitHub by pushing this change.

**Notes:** The `uv.lock` file was automatically updated when `uv sync` resolved the new maturin
dependency — this is expected and must be committed alongside the other changes. Stale build
artifacts (`.so`, `__pycache__`) in the renamed directory were cleaned up locally but were never
tracked by git, so no git cleanup was needed.
