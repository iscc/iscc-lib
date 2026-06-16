## 2026-06-16 — Add streaming `SumHasher` to the Python bindings

**Done:** Exposed a single-pass `SumHasher` class in the `iscc_lib` Python package — a PyO3 wrapper
(`#[pyclass] PySumHasher`) over the core `iscc_lib::streaming::SumHasher` plus a Pythonic wrapper
class mirroring `DataHasher`/`InstanceHasher`. Streaming consumers can now compute an ISCC-SUM
incrementally with one hasher instead of driving two and feeding every chunk twice. Closes the
Python half of issue #37.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: added `#[pyclass(name = "SumHasher")] PySumHasher` backed by
    `iscc_lib::streaming::SumHasher` (`Option<inner>` finalize-once pattern, copied from
    `PyDataHasher`); `finalize(bits=64, wide=false, add_units=false)` builds a `PyDict` with `iscc`,
    `datahash`, `filesize`, and `units` only when `Some` (matching the `gen_sum_code_v0` wrapper).
    Registered with `m.add_class::<PySumHasher>()`.
- `crates/iscc-py/python/iscc_lib/__init__.py`: imported `SumHasher as _SumHasher`; added public
    `SumHasher` wrapper (optional initial `bytes | bytearray | memoryview | BinaryIO`, `update()`
    reads file-likes in 64 KiB chunks, `finalize(bits, wide, add_units) -> SumCodeResult` reusing
    the existing `SumCodeResult`); added `"SumHasher"` to `__all__`.
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: added `class SumHasher` stub.
- `crates/iscc-py/README.md`: added `SumHasher` to the "Streaming:" line.
- `crates/iscc-py/CLAUDE.md`: updated `#[pyclass]` count (2→3), Module Layout streaming note, Python
    layer note, Type Mapping row, and the `__init__`-params pitfall to include `SumHasher`.
- `docs/howto/python.md`: added a `SumHasher` streaming example + updated the Streaming section
    intro.
- `tests/test_streaming.py`: added 11 `SumHasher` tests.

**Verification:**

- `cargo clippy -p iscc-py -- -D warnings` — clean.
- `maturin develop -m crates/iscc-py/Cargo.toml` — builds successfully.
- `pytest tests/test_streaming.py` — 58 passed (47 prior + 11 new).
- `pytest` (full suite) — 279 passed.
- `python -c "import iscc_lib; assert 'SumHasher' in iscc_lib.__all__; assert iscc_lib.SumHasher"` —
    exits 0.
- `ruff check` / `ruff format --check` / `ty check` — all clean.
- `mise run check` (15 pre-commit hooks) — all Passed (mdformat auto-reflowed the new doc prose on
    first run, re-ran clean).

New tests cover: parametrized `(bits, wide, add_units)` grid (64/128/256, wide on/off, units on/off)
matching `gen_sum_code_v0` on a `tmp_path` file; multi-chunk matching the manual two-hasher path
(`DataHasher` + `InstanceHasher` → `gen_iscc_code_v0`); `units` key omitted when disabled; `BytesIO`
== `bytes` input; constructor with initial data and `BinaryIO`; double-finalize and
update-after-finalize each raise `ValueError`; `SumHasher` in `__all__`.

**Next:** The WASM `SumHasher` wrapper — the remaining half of issue #37 (different test harness,
`crates/iscc-wasm`). After that, consider promoting `SumHasher` to a crate-root Tier 1 `pub use` and
bumping the documented "2 streaming types → 3" wording across all bindings (deliberately deferred
per this step's "Not In Scope"), or picking the highest-impact backlog item (npm
`optionalDependencies` #38, PyO3 0.23→0.29 migration, or GIL release #39).

**Notes:**

- Used `iscc_lib::streaming::SumHasher` (full module path) since `SumHasher` is intentionally NOT a
    crate-root re-export yet (per "Not In Scope"). `DataHasher`/`InstanceHasher` use the bare
    `iscc_lib::DataHasher` form because they ARE Tier 1 re-exports — this asymmetry is expected.
- The "Structured results" table in `docs/howto/python.md` (line ~235) lists `SumCodeResult` as
    `iscc, datahash, filesize` and omits the optional `units` field. This predates this step and was
    out of scope, so left untouched — flagging for the review agent in case it wants a follow-up.
- No core crate (`iscc-lib`) changes, no WASM changes, no GIL release, no Tier 1 count change — all
    correctly deferred. `_lowlevel` `PySumHasher` has no `__init__` data params (stream handling
    lives in the Python wrapper, per the crate pitfalls).
