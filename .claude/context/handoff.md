## 2026-02-22 — Review of: Scaffold iscc-py crate with PyO3/maturin and expose gen_instance_code_v0

**Verdict:** NEEDS_WORK

**Summary:** The iscc-py crate is correctly scaffolded following the `_lowlevel` pattern from
notes/02-language-bindings.md. `gen_instance_code_v0` is exposed through PyO3 with proper default
arguments, the full pipeline works end-to-end (Rust → native module → Python import → correct
conformance output), and the `IsccError::NotImplemented` variant was cleanly removed. All local
verification passes (cargo build, test, clippy, fmt, maturin develop, pytest). However, the pre-push
hook fails on `ty` type checking because `iscc._lowlevel` is a native module with no type stub.

**Issues found:**

- **Pre-push `ty` type checker failure**:
    `error[unresolved-import]: Cannot resolve imported module   iscc._lowlevel` in
    `crates/iscc-py/python/iscc/__init__.py:3`. The `_lowlevel` module is a native extension built
    by maturin — it doesn't exist as Python source, so `ty` can't resolve the import. Fix: add a
    `_lowlevel.pyi` type stub in `crates/iscc-py/python/iscc/` declaring the function signatures.
    This is the correct approach since the package already has `py.typed`.

**Push failure:**

```
Python type checking.....................................................Failed
- hook id: ty
- exit code: 1

  error[unresolved-import]: Cannot resolve imported module `iscc._lowlevel`
   --> crates/iscc-py/python/iscc/__init__.py:3:6
    |
  3 | from iscc._lowlevel import gen_instance_code_v0
    |      ^^^^^^^^^^^^^^
```

**Next:** Add a `_lowlevel.pyi` type stub file at `crates/iscc-py/python/iscc/_lowlevel.pyi` that
declares the function signatures for the native module. This will satisfy the `ty` type checker. The
stub should declare:

```python
def gen_instance_code_v0(data: bytes, bits: int = 64) -> str: ...
```

This is a small, focused fix. After this, proceed to expose the remaining 8 `gen_*_v0` functions
(adding their stubs to the `.pyi` file at the same time).

**Notes:** The `ty` type checker runs in the pre-push hook and checks all Python files in the
project. Native extension modules (built by maturin/PyO3) need `.pyi` type stubs for `ty` to resolve
imports. This will be needed for every function added to `_lowlevel`. The advance agent should
create the stub file as part of the next work package.
