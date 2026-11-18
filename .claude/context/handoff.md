## 2026-11-18 — Python `encode_seq` / `decode_seq` binding with an `iscc_core` differential test

**Done:** I exposed ISCC-SEQ in the Python binding. In `lib.rs`, `encode_seq` returns `PyBytes` and
`decode_seq` returns `list[str]`. The `__init__.py` wrapper carries the reference's type contract.
`tests/test_seq.py` runs the fixture conformance vectors, round trips, the input forms and the error
arms. It also has an explicit strict-base32 differential against `iscc_core` 1.5.0: one bad input
per test case, with both directions asserted.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: two `#[pyfunction]`s, `encode_seq(Vec<String>) -> PyBytes` and
    `decode_seq(&[u8]) -> Vec<String>`. Both map errors to `PyValueError` and are registered in the
    module.
- `crates/iscc-py/python/iscc_lib/__init__.py`: two wrappers, both added to `__all__`.
    - `encode_seq(units: Iterable[str])` raises `TypeError` for a bare `str` or `bytes`, and otherwise
        calls `list(units)`.
    - `decode_seq` accepts `bytes`, `bytearray` and `memoryview`, and raises `TypeError` for anything
        else.
    - The docstrings are adapted from `codec.py` and state the strict base32 rule.
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: stubs for both functions.
- `tests/test_seq.py` (new): 47 tests.

**Verification:**

- `uv run maturin develop -m crates/iscc-py/Cargo.toml`: built and installed.
- `uv run pytest tests/test_seq.py`: 47 passed. Full `uv run pytest`: 477 passed.
- Both `python -c` one-liners from next.md exit 0 (crit1=0, crit2=0).
- `uv run ty check`: "All checks passed!"
- `cargo clippy -p iscc-py -- -D warnings`: clean.
- `mise run check`: exit 0, all hooks passed. Run with `LANG=C.UTF-8`.
- Divergence probe output, `iscc_core` vs `iscc_lib`:
    - `GAAAAAAAAB`: core gives `300000000000`; lib says "non-zero trailing bits at 9".
    - `GAAAAAAAAA======`: core gives `300000000000`; lib says "invalid symbol at 10".
    - 256-bit `…GBJ`: core gives the canonical bytes; lib says "non-zero trailing bits at 54".
- Benchmarks: none run. This step is binding-only and touches no core hot path.

**Next:**

- The Go `decodeBase32` spare-bit fix, plus Rust and Go strict-base32 tests (the `normal` issue).
    Together with the Go `isccClean` port, it is the prerequisite for Go seq.
- Otherwise, continue the seq fan-out on surfaces with no prerequisites: Node, WASM, Ruby, JNI.
- The C FFI step must regenerate `iscc.h` and `NativeMethods.g.cs` in the same step.

**Notes:**

- **Deviation from next.md:** the suggested ISCC-CODE case `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` has
    non-zero spare bits itself.
    - `iscc_lib` rejects it during base32 decoding ("non-zero trailing bits at 28") and never reaches
        the MainType check.
    - The test uses the canonical `ISCC:KUAALNA7GRZESMRFUURK6SRMTS7WU` instead, generated with
        `gen_iscc_code_v0` from the fixture's Data and Instance units.
    - Both libraries reject it with "MainType 5 is not permitted in an ISCC-SEQ (offset 0)".
    - The test matches on `MainType 5`.
- **Annotation:** I typed the wrapper as `Iterable[str]`, not `Sequence[str]`. Generators are
    accepted and tested, and `list(units)` handles any iterable.
- **Untyped bad inputs:** the deliberate wrong-type calls (`[1]`, `"abc"`) go through `parametrize`
    so `ty` does not flag them. No suppressions were added.
- **Element errors:** non-string elements raise `TypeError` through PyO3 `Vec<String>` extraction.
    The message differs from the reference's, but the exception type is the same.
- **No version pin:** I added no `iscc_core.__version__` pin, because a patch bump would turn it red
    with no behaviour change. If the reference ever becomes strict, the differential "reference
    accepts" tests fail loudly.
- **Out of scope, stale:** `crates/iscc-py/CLAUDE.md` still says "27 #[pyfunction]s". That belongs
    to the deferred Tier-1 count sweep.
