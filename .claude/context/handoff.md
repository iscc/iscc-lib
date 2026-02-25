## 2026-02-25 — Add `__version__` attribute and fix iscc-py module docstring

**Done:** Added `__version__` attribute to `iscc_lib` Python package via
`importlib.metadata.version("iscc-lib")`, fixed three incorrect module name references in
`crates/iscc-py/src/lib.rs` (changed `iscc._lowlevel` → `iscc_lib._lowlevel` and
`python/iscc/__init__.py` → `python/iscc_lib/__init__.py`), and added two tests verifying the
version attribute.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `from importlib.metadata import version` and
    `__version__ = version("iscc-lib")` near the top; added `"__version__"` to `__all__`
- `crates/iscc-py/src/lib.rs`: Fixed three references — line 1 docstring (`iscc._lowlevel` →
    `iscc_lib._lowlevel`), line 3 (`iscc` → `iscc_lib`), line 4 path (`python/iscc/__init__.py` →
    `python/iscc_lib/__init__.py`), and line 360 module doc (`iscc._lowlevel` →
    `iscc_lib._lowlevel`)
- `tests/test_smoke.py`: Added `test_version_exists_and_correct` (verifies `__version__` is a string
    equal to `"0.0.1"`) and `test_version_in_all` (verifies `"__version__"` is in `__all__`)

**Verification:** All 159 Python tests pass (19 smoke + 140 conformance).
`python -c "import iscc_lib; print(iscc_lib.__version__)"` prints `0.0.1`.
`grep -q 'iscc_lib._lowlevel' crates/iscc-py/src/lib.rs` exits 0. No old incorrect `iscc._lowlevel`
references remain. `cargo clippy -p iscc-py -- -D warnings` clean.

**Next:** Remaining `[low]` issues: iscc-wasm conformance feature gate, iscc-wasm stale CLAUDE.md,
iscc-jni exception mapping, and TypeScript port evaluation.

**Notes:** next.md specified the version as `"0.0.1"` which is correct for the workspace Cargo.toml
version. The installed package had a stale `0.1.0` version from a previous build — rebuilding with
`maturin develop` resolved the mismatch. Also found a third incorrect `iscc._lowlevel` reference on
line 360 (the `#[pymodule]` docstring) that wasn't mentioned in next.md but was caught by the
verification grep and fixed.
