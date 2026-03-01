## 2026-03-01 — Add gen_sum_code_v0 to Python bindings

**Done:** Exposed `gen_sum_code_v0` and `SumCodeResult` in the Python package (`iscc_lib`),
completing 32/32 Tier 1 symbols in the primary consumer binding. The PyO3 wrapper accepts a `str`
path, the Python public wrapper additionally accepts `os.PathLike`, and a type stub was added.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added `gen_sum_code_v0` PyO3 function (accepts `&str` path,
    `bits: u32`, `wide: bool`, returns `PyDict` with `iscc`/`datahash`/`filesize`) and registered it
    in the pymodule
- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `os` import, `_gen_sum_code_v0` lowlevel
    import, `SumCodeResult(IsccResult)` class, public `gen_sum_code_v0` wrapper with
    `str | os.PathLike` support, and both symbols to `__all__`
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added type stub for `gen_sum_code_v0`
- `tests/test_smoke.py`: Added 6 tests covering equivalence with `gen_instance_code_v0`, `PathLike`
    acceptance, error on nonexistent path, result type checks, attribute access, and wide mode

**Verification:**

- `cargo test -p iscc-py` passes (0 Rust-side tests — iscc-py has no Rust tests)
- `cargo clippy -p iscc-py -- -D warnings` clean
- `uv run pytest tests/ -x` passes — 204 tests (25 smoke + 179 conformance/others)
- `uv run ruff check` clean (via `mise run check`)
- Import verification: `from iscc_lib import gen_sum_code_v0, SumCodeResult` succeeds
- `__all__` verification: both `gen_sum_code_v0` and `SumCodeResult` present

**Next:** Propagate `gen_sum_code_v0` to Node.js bindings (`crates/iscc-napi/`) following the same
pattern, or to C FFI / Java / WASM bindings. Issue #15 binding tasks 5-6 track the full propagation
across all bindings.

**Notes:** The wide mode test required `bits=128` because `gen_iscc_code_v0` only produces a
different wide-mode output when both input codes are 128-bit+. With default `bits=64`, the Data-Code
body is only 64 bits, and wide mode falls back to the same output. This is expected behavior from
the Rust core, not a bug. Also fixed a pre-existing duplicate `MixedCodeResult` entry in `__all__`
that was introduced by the edit.
