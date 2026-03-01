## 2026-03-01 — Export META_TRIM_META in Python binding

**Done:** Added `META_TRIM_META = 128_000` to the Python binding in all three required locations:
PyO3 module init (Rust), Python `__init__.py` (import, `core_opts`, `__all__`), and type stub
(`.pyi`). Followed the exact pattern used by `META_TRIM_NAME` and `META_TRIM_DESCRIPTION`.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added `m.add("META_TRIM_META", iscc_lib::META_TRIM_META)?;` after
    `META_TRIM_DESCRIPTION` line in module init
- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `META_TRIM_META` import (alphabetical order),
    `core_opts.meta_trim_meta` attribute, and `__all__` entry
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added `META_TRIM_META: int` type stub with
    docstring

**Verification:**

- `cargo test -p iscc-py` passes (0 Rust-side tests, compiles clean)
- `cargo clippy -p iscc-py -- -D warnings` clean
- `uv run pytest` passes — 198 tests, 0 failures
- `python -c "from iscc_lib import META_TRIM_META; assert META_TRIM_META == 128_000"` — OK
- `python -c "from iscc_lib import core_opts; assert core_opts.meta_trim_meta == 128_000"` — OK
- `python -c "import iscc_lib; assert 'META_TRIM_META' in iscc_lib.__all__"` — OK
- All 14 pre-commit hooks pass

**Next:** Expose `META_TRIM_META` in the remaining 5 binding crates (Node.js, WASM, C FFI, Java,
Go). Go already has `MetaTrimMeta` patterns in `codec.go`; the other 4 are Rust-based and follow the
same `m.add()` / export pattern. Each needs constant export + test. This can likely be done as a
single step since it's mechanical repetition.

**Notes:** Required `uv run maturin develop -m crates/iscc-py/Cargo.toml` to rebuild the `.so` file
after adding the Rust constant — the stale `.abi3.so` caused import errors until rebuilt. No new
tests were added since next.md explicitly excluded Python tests for the constant (Rust core already
tests it), and the verification criteria use inline assertions. All existing tests remain green.
