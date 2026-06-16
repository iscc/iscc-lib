# Next Work Package

## Step: Add streaming `SumHasher` to the Python bindings

## Goal

Expose a single-pass `SumHasher` class in the `iscc_lib` Python package (PyO3 wrapper over the core
`streaming::SumHasher` added in iteration 88), so streaming consumers can compute an ISCC-SUM
incrementally without driving two hashers and feeding every chunk twice. This closes the Python half
of issue #37 ("Add streaming `SumHasher` to Python and WASM bindings").

## Scope

- **Create**: (none — new tests go in the existing `tests/test_streaming.py`)
- **Modify** (code, 3 files):
    - `crates/iscc-py/src/lib.rs` — add a `#[pyclass(name = "SumHasher")]` `PySumHasher` backed by
        `iscc_lib::streaming::SumHasher`; register it with `m.add_class::<PySumHasher>()` in the
        `#[pymodule]`.
    - `crates/iscc-py/python/iscc_lib/__init__.py` — import `SumHasher as _SumHasher` from
        `._lowlevel`; add a public `SumHasher` wrapper class mirroring `DataHasher`/`InstanceHasher`;
        add `"SumHasher"` to `__all__`.
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add a `class SumHasher` type stub.
- **Modify** (docs):
    - `docs/howto/python.md` — add a `SumHasher` streaming example alongside the existing
        `DataHasher`/`InstanceHasher` sections.
    - `crates/iscc-py/README.md` — add `SumHasher` to the "Streaming:" line (~line 57).
    - `crates/iscc-py/CLAUDE.md` — update the streaming-type mentions (Module Layout note + Type
        Mapping row + pitfalls) to include `SumHasher`.
- **Modify** (tests): `tests/test_streaming.py` — add `SumHasher` tests.
- **Reference**:
    - `crates/iscc-lib/src/streaming.rs` (lines ~155-210) — core `SumHasher` API (`new` /
        `update(&[u8])` / `finalize(bits, wide, add_units) -> SumCodeResult`).
    - `crates/iscc-py/src/lib.rs` — `PyDataHasher`/`PyInstanceHasher` (lines ~520-609) and the
        `gen_sum_code_v0` PyO3 wrapper (lines ~334-351) for the `units`-optional dict construction.
    - `crates/iscc-py/python/iscc_lib/__init__.py` — `DataHasher`/`InstanceHasher` wrapper classes
        (lines ~288-345) and `SumCodeResult` (line ~185).
    - `.claude/context/specs/python-bindings.md` → "Streaming SumHasher" (lines ~237-267).

## Not In Scope

- **WASM `SumHasher` wrapper** (the other half of issue #37) — separate follow-up step; it uses a
    different test harness and would push past the 3-code-file limit. Do NOT touch
    `crates/iscc-wasm`.
- **Promoting `SumHasher` to a crate-root Tier 1 `pub use`** or bumping the documented "32 Tier 1
    symbols / 2 streaming types" count. `streaming` is already a `pub mod`, so
    `iscc_lib::streaming::SumHasher` is reachable without a core change. SumHasher is a Python/WASM
    streaming convenience (issue #37 is binding-specific), not a symbol bound in all 12 languages,
    so it must NOT be folded into the "bound in all languages" Tier 1 count. Do NOT modify
    `crates/iscc-lib/`.
- **GIL release (`py.allow_threads`)** for the new `SumHasher.update()` — that is issue #39, a
    separate step.
- Changing `gen_sum_code_v0` or any existing hasher behavior.

## Implementation Notes

- **Rust layer (`PySumHasher`)**: copy the `PyDataHasher` shape exactly — `inner: Option<...>`,
    `#[new]` constructs `Some(iscc_lib::streaming::SumHasher::new())`,
    `update(&mut self, data:   &[u8])` uses
    `as_mut().ok_or_else(|| PyValueError::new_err("SumHasher already finalized"))?`, and `finalize`
    uses `take().ok_or_else(...)?`. Use
    `#[pyo3(signature = (bits=64, wide=false, add_units=false))]` and build the return `PyDict`
    exactly like the `gen_sum_code_v0` wrapper: always set `iscc`, `datahash`, `filesize`, and set
    `units` only when `r.units` is `Some` (omit the key otherwise — do not set `None`).
- **Do NOT** add `__init__` params to the `_lowlevel` `PySumHasher` — stream/initial-data handling
    lives in the Python wrapper (per `crates/iscc-py/CLAUDE.md` pitfalls). `_lowlevel.update` takes
    `&[u8]` only.
- **Python wrapper (`SumHasher`)**: mirror `DataHasher` — constructor takes optional
    `bytes | bytearray | memoryview | BinaryIO`; `update()` accepts the same union and reads
    file-likes in `_CHUNK_SIZE` (64 KiB) loops;
    `finalize(self, bits: int = 64, wide: bool = False,   add_units: bool = False) -> SumCodeResult`
    returns `SumCodeResult(self._inner.finalize(bits,   wide, add_units))`. `SumCodeResult` already
    exists (line ~185) — reuse it.
- **`.pyi` stub**: mirror the existing `DataHasher` stub but add `wide`/`add_units` params to
    `finalize` returning `dict[str, Any]`.
- **Tests** (`tests/test_streaming.py`): follow the existing `test_data_hasher_*` patterns:
    - Single-update and multi-chunk results match `gen_sum_code_v0` on a `tempfile` written with the
        same bytes, across `(bits, wide, add_units)` combinations (64/128/256, wide on/off, units
        on/off).
    - Result also matches the manual two-hasher path (`DataHasher` + `InstanceHasher` →
        `gen_iscc_code_v0`).
    - `BinaryIO` (`io.BytesIO`) input produces identical output to `bytes` input.
    - Constructor with initial data and with `BinaryIO`.
    - Finalize-once: a second `finalize()` raises `ValueError`; an `update()` after `finalize()`
        raises `ValueError`.
    - `from iscc_lib import SumHasher` works and `"SumHasher" in iscc_lib.__all__`.

## Verification

- `maturin develop -m crates/iscc-py/Cargo.toml` builds the extension successfully.
- `cargo clippy -p iscc-py -- -D warnings` clean.
- `pytest tests/test_streaming.py` passes (existing tests + new `SumHasher` tests).
- `pytest` (full Python suite) passes.
- `python -c "import iscc_lib; assert 'SumHasher' in iscc_lib.__all__; assert iscc_lib.SumHasher"`
    exits 0.
- A test asserts `SumHasher(...).finalize(bits, wide, add_units)` equals
    `gen_sum_code_v0(tempfile, bits, wide, add_units)` for the same bytes across the parameter grid.
- A test asserts a second `finalize()` and a post-`finalize()` `update()` each raise `ValueError`.
- `ruff check` and `ruff format --check` clean; `ty check` clean (stub present).

## Done When

The Python `SumHasher` class is importable from `iscc_lib`, streams data incrementally, produces
output identical to `gen_sum_code_v0` and the two-hasher pattern across all
`(bits, wide, add_units)` combinations, enforces finalize-once semantics, is listed in `__all__`,
and all verification checks pass.
