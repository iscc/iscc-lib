## 2026-02-23 — Review of: Python streaming support for gen_data_code_v0 and gen_instance_code_v0

**Verdict:** NEEDS_WORK

**Summary:** Implementation is functionally correct — all tests pass, pre-commit hooks clean.
However the pre-push `ty` type checker rejects the `hasattr(data, "read")` duck-typing pattern
because `ty` doesn't narrow `bytes | BinaryIO` after `hasattr()` — it still sees `data.read()` as
calling on `object` (the union's common base). The fix is straightforward: invert the check to
`if isinstance(data, bytes): ... else: data = data.read()`, or use an explicit `cast()`.

**Issues found:**

- `ty` pre-push hook fails with `call-non-callable: Object of type 'object' is not callable` on
    lines 136 and 143 of `__init__.py` — `data.read()` after `hasattr(data, "read")` guard

**Push failure:**

```
error[call-non-callable]: Object of type `object` is not callable
   --> crates/iscc-py/python/iscc_lib/__init__.py:136:16
    |
134 |     """Generate an ISCC Data-Code from raw byte data or a file-like stream."""
135 |     if hasattr(data, "read"):
136 |         data = data.read()
    |                ^^^^^^^^^^^

error[call-non-callable]: Object of type `object` is not callable
   --> crates/iscc-py/python/iscc_lib/__init__.py:143:16
    |
141 |     """Generate an ISCC Instance-Code from raw byte data or a file-like stream."""
142 |     if hasattr(data, "read"):
143 |         data = data.read()
    |                ^^^^^^^^^^^
```

**Next:** Fix the `ty` type checker error in both functions. Recommended approach: replace
`if hasattr(data, "read"): data = data.read()` with
`if not isinstance(data, bytes): data = data.read()` — this gives `ty` proper type narrowing (in the
`else` branch, `data` is `BinaryIO` which has `.read()`). Alternatively, use
`isinstance(data, (io.RawIOBase, io.BufferedIOBase, io.IOBase))` or `typing.cast`. The
`isinstance(data, bytes)` inversion is simplest and still duck-type compatible since anything that
isn't `bytes` will be treated as a stream.

**Notes:** The `ty` type checker is stricter than mypy/pyright about `hasattr()` type narrowing.
This is a known limitation — `ty` doesn't support `hasattr()`-based narrowing (it's a relatively new
type checker). The `isinstance` approach is equally correct at runtime and satisfies the type
checker. All other quality gates pass — this is the only blocking issue.
