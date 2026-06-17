# Next Work Package

## Step: Release the GIL during Python hashing (`py.allow_threads`)

## Goal

Release the Python GIL around the pure-Rust CPU-bound hashing work so threaded consumers (e.g.
`iscc-sdk`'s `ThreadPoolExecutor`) can overlap hashing instead of serializing on the GIL. Resolves
issue #39 ("Release the GIL during Python hashing"). Output bytes are identical, so conformance is
unaffected — this is purely a concurrency improvement.

## Scope

- **Modify**: `crates/iscc-py/src/lib.rs` — wrap the pure-Rust compute in `py.allow_threads(...)` at
    exactly these 7 call sites:
    - one-shot functions (already take `py: Python<'_>`): `gen_data_code_v0` (~:292),
        `gen_instance_code_v0` (~:305), `gen_image_code_v0` (~:150), `gen_sum_code_v0` (~:335)
    - streaming `update()` methods (must ADD a `py: Python<'_>` parameter — see notes):
        `PyDataHasher::update` (~:541), `PyInstanceHasher::update` (~:585), `PySumHasher::update`
        (~:631)
- **Create**: `tests/test_gil.py` — deterministic concurrency-correctness test (multiple threads
    producing byte-identical output to the single-threaded path). Tests are excluded from the file
    limit.
- **Reference**: `.claude/context/specs/python-bindings.md` → "GIL Release During Hashing"
    (acceptance criteria); `crates/iscc-py/CLAUDE.md` (binding rules); `crates/iscc-py/src/lib.rs`
    (existing finalize methods already take `py: Python<'_>` and show the pattern).

## Not In Scope

- **Do NOT touch `release.yml` / npm `optionalDependencies` (#38)** — that is a separate
    release-workflow step requiring a real publish to verify.
- **Do NOT bump PyO3** (still `0.23` in root `Cargo.toml`). `allow_threads` is the correct API name
    in 0.23 (do not rename to `detach`).
- **Do NOT wrap `finalize()` methods** — issue #39 scopes only `update()` + the 4 one-shot
    functions. The bulk compute (CDC/xxh32 for Data, BLAKE3 for Instance) happens in `update`.
- **Do NOT change any Python-facing signature.** Adding `py: Python<'_>` to `update()` is invisible
    to Python; `_lowlevel.pyi` stubs must stay unchanged.
- **Do NOT add a 2-thread multi-GB perf microbenchmark to CI** — non-deterministic on shared
    runners. The ~2× throughput target is an aspiration, not a CI gate.
- **No size-threshold optimization is required** (premature; the Python wrapper already feeds 64 KiB
    chunks to `update()`). Prefer the simplest unconditional release.

## Implementation Notes

- Pattern for one-shot functions (compute inside the closure, build the dict AFTER):
    ```rust
    let r = py
        .allow_threads(|| iscc_lib::gen_data_code_v0(data, bits))
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    ```
    `&[u8]` and `&str` are `Ungil + Send`, and the result structs are plain data (`Ungil`), so the
    closure type-checks. Keep `PyDict` construction (which needs the GIL) outside the closure.
- Pattern for `update()` methods — add the injected `py` parameter, take the `&mut inner` borrow
    BEFORE releasing, then release around the pure call:
    ```rust
    fn update(&mut self, py: Python<'_>, data: &[u8]) -> PyResult<()> {
        let inner = self.inner.as_mut()
            .ok_or_else(|| PyValueError::new_err("DataHasher already finalized"))?;
        py.allow_threads(|| inner.update(data));
        Ok(())
    }
    ```
    `&mut iscc_lib::DataHasher` is `Ungil` (the core hashers hold no Python types). The
    finalized-error check stays GIL-held (it touches `self`).
- **Soundness of the `&[u8]` borrow**: the public Python wrapper in `__init__.py` already coerces
    inputs to immutable `bytes` before calling `_lowlevel`, so the borrowed buffer cannot be mutated
    by another thread during the release. Prefer the borrowed `&[u8]` if it compiles; only copy to
    an owned `Vec<u8>` before the closure if the borrow checker objects.
- `tests/test_gil.py`: spin up N threads (e.g., via `concurrent.futures.ThreadPoolExecutor` or
    `threading.Thread`), each hashing the same and/or distinct byte payloads through
    `gen_data_code_v0`, `gen_instance_code_v0`, a streaming
    `DataHasher`/`InstanceHasher`/`SumHasher`, and assert outputs equal the single-threaded results.
    This verifies correctness-under-concurrency deterministically (it does not assert a speedup).
    Use `from iscc_lib import ...` (public API), per the crate's test convention.

## Verification

- `cargo build -p iscc-py` compiles (proves the `Ungil`/`Send` bounds are satisfied at every call
    site).
- `cargo clippy -p iscc-py -- -D warnings` clean.
- `cargo fmt -p iscc-py --check` clean.
- `grep -c "allow_threads" crates/iscc-py/src/lib.rs` returns at least 7.
- `maturin develop -m crates/iscc-py/Cargo.toml` succeeds, then `pytest tests/` passes (existing
    conformance + smoke + streaming suites unchanged, plus the new `tests/test_gil.py`).
- `ruff check tests/test_gil.py` and `ruff format --check tests/test_gil.py` clean.

## Done When

The 3 streaming `update()` methods and the 4 one-shot byte-data functions release the GIL around
their pure-Rust compute, no Python-facing signature changes, `tests/test_gil.py` confirms identical
output under multithreaded use, and all verification commands pass.
