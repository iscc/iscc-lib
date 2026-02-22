# Next Work Package

## Step: Fix CI — add maturin dependency and rename Python module to iscc_lib

## Goal

Fix the failing Python CI job by adding `maturin` to root dev dependencies and renaming the Python
module from `iscc` to `iscc_lib` to match the PyPI package name. After this step, both CI jobs (Rust
and Python) should pass.

## Scope

- **Create**: (none)
- **Modify**:
    - `pyproject.toml` — add `maturin` to `[dependency-groups] dev`
    - `crates/iscc-py/pyproject.toml` — change `module-name` from `iscc._lowlevel` to
        `iscc_lib._lowlevel`
    - `crates/iscc-py/python/iscc/` → rename directory to `crates/iscc-py/python/iscc_lib/` (includes
        `__init__.py`, `_lowlevel.pyi`, `py.typed`; delete the old `iscc/` directory including
        `__pycache__` and stale `.so`)
    - `crates/iscc-py/python/iscc_lib/__init__.py` — update import: `from iscc._lowlevel` →
        `from iscc_lib._lowlevel`
    - `tests/test_conformance.py` — update import: `from iscc import` → `from iscc_lib import`
    - `tests/test_smoke.py` — update import: `from iscc import` → `from iscc_lib import`
- **Reference**: `crates/iscc-py/Cargo.toml` (no changes needed — crate name `_lowlevel` stays)

## Implementation Notes

1. **Add maturin to root pyproject.toml**: Add `"maturin"` to the `dev` dependency group list. This
    is the root cause of the CI failure — `uv sync --group dev` doesn't install maturin because
    it's only in `crates/iscc-py/pyproject.toml` build-requires, which CI doesn't process.

2. **Rename Python module directory**: Use
    `git mv crates/iscc-py/python/iscc  crates/iscc-py/python/iscc_lib` to rename the directory.
    Then clean up any stale artifacts (`__pycache__`, `.so` files) that shouldn't be tracked. The
    `_lowlevel.pyi` type stub and `py.typed` marker move with the directory.

3. **Update module-name in maturin config**: In `crates/iscc-py/pyproject.toml`, change
    `module-name = "iscc._lowlevel"` to `module-name = "iscc_lib._lowlevel"`. This tells maturin
    where to place the compiled `.so`/`.pyd` file.

4. **Update Python imports**: In `__init__.py`, change `from iscc._lowlevel import ...` to
    `from iscc_lib._lowlevel import ...`. In both test files, change `from iscc import ...` to
    `from iscc_lib import ...`.

5. **Verify locally**: Run `maturin develop --manifest-path crates/iscc-py/Cargo.toml` then `pytest`
    to confirm all 49 tests still pass with the new module name.

6. **Clean stale files**: Remove `crates/iscc-py/python/iscc/_lowlevel.abi3.so` and `__pycache__`
    from git tracking if they were accidentally committed. These are build artifacts.

## Verification

- `uv sync --group dev` installs maturin (verify with `uv run maturin --version`)
- `uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml` builds successfully
- `uv run pytest` passes all 49 tests (46 conformance + 3 smoke)
- `python -c "import iscc_lib"` succeeds (new module name)
- `python -c "import iscc"` fails with ImportError (old module name no longer exists)
- `cargo test -p iscc-lib` still passes (143 tests — Rust unaffected)
- No stale `.so` or `__pycache__` files tracked in git

## Done When

Both `maturin develop` and `pytest` succeed using the `iscc_lib` module name, all 49 Python and 143
Rust tests pass, and no stale build artifacts remain in the repository.
