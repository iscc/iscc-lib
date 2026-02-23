## 2026-02-23 — Add conformance_selftest and text utilities to Python bindings

**Done:** Exposed `conformance_selftest` and 4 text utility functions (`text_clean`,
`text_remove_newlines`, `text_trim`, `text_collapse`) through the Python bindings. Added 5
`#[pyfunction]` wrappers in the PyO3 crate, registered them in the module, added type stubs, and
re-exported them from `iscc_lib.__init__`.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added 5 `#[pyfunction]` wrappers and registered them in the
    `iscc_lowlevel` module
- `crates/iscc-py/python/iscc_lib/__init__.py`: Added direct imports from `_lowlevel` for all 5
    functions (no wrapper needed — they return simple types) and added them to `__all__`
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added type stubs with docstrings for all 5
    functions
- `tests/test_text_utils.py`: New test file with 22 tests covering all 5 functions

**Verification:** All 85 Python tests pass (63 existing + 22 new). All pre-commit hooks pass.
Workspace-wide clippy clean. `maturin develop` builds successfully. All 5 functions are importable
from `iscc_lib` and produce correct results:

- `conformance_selftest()` returns `True`
- `text_clean("hello\tworld")` returns `"helloworld"`
- `text_collapse("Hello World")` returns `"helloworld"`
- `text_trim("hello world", 5)` returns `"hello"`
- `text_remove_newlines("hello\nworld")` returns `"hello world"`

**Next:** Continue expanding the Python binding API surface — add remaining Tier 1 symbols:
algorithm primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `sliding_window`), codec
functions (`encode_base64`, `iscc_decompose`), and streaming types (`DataHasher`, `InstanceHasher`).
Also consider adding the same 5 functions to Node.js, WASM, and C FFI bindings.

**Notes:** The text utility functions and `conformance_selftest` all have trivial signatures (str →
str, void → bool) so they don't need the `IsccResult` wrapper pattern used by `gen_*_v0` functions.
They're imported directly from `_lowlevel` using the `as` re-export pattern for `ty` type checker
compatibility.
