## 2026-02-22 — Expose all 9 gen\_\*\_v0 functions in PyO3 bindings with type stub

**Done:** Added PyO3 wrappers for all 8 remaining `gen_*_v0` functions (meta, text, image, audio,
video, mixed, data, iscc-code), created `_lowlevel.pyi` type stubs for the native module, and
updated `__init__.py` to re-export all 9 functions.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added 8 `#[pyfunction]` wrappers matching the existing
    `gen_instance_code_v0` pattern. Type conversions: `Vec<i32>` for `&[i32]`, `Vec<Vec<i32>>` for
    `&[Vec<i32>]`, `Vec<String>` → `Vec<&str>` for `&[&str]` parameters. All 9 functions registered
    in the `iscc_lowlevel` module.
- `crates/iscc-py/python/iscc/_lowlevel.pyi`: Created type stub with all 9 function signatures using
    Python type annotations (`bytes`, `list[int]`, `str | None`, etc.).
- `crates/iscc-py/python/iscc/__init__.py`: Updated to import and re-export all 9 functions in
    `__all__` (alphabetically sorted).

**Verification:**

- `cargo build -p iscc-py` ✓
- `cargo clippy -p iscc-py -- -D warnings` ✓ (clean)
- `cargo fmt --check` ✓ (clean)
- `maturin develop` ✓ (built and installed)
- All 9 functions importable from Python ✓
- `uv run ty check` ✓ ("All checks passed!")
- `uv run pytest tests/` ✓ (3 passed)
- `cargo test -p iscc-lib` ✓ (143 passed, all conformance vectors match)

**Next:** The Python bindings are now complete. Next steps could include:

1. Adding Python-level conformance tests (pytest tests that exercise all 9 functions against
    data.json vectors)
2. Setting up CI/CD pipeline (GitHub Actions)
3. Adding benchmarks comparing Rust vs Python reference performance

**Notes:** The next.md verification section expected `gen_meta_code_v0('Hello')` to return
`ISCC:AAAWKLHFPV6OPKDG` but the actual output is `ISCC:AAAWKLHFXM75OAMK`. This is NOT a bug — all 16
gen_meta_code_v0 conformance test vectors pass correctly against the official data.json. The
expected value in next.md appears to have been incorrect (possibly from a different iscc-core
version or not verified against the reference).
