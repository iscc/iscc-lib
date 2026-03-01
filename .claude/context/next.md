# Next Work Package

## Step: Export META_TRIM_META in Python binding

## Goal

Expose `META_TRIM_META = 128_000` in the Python binding (`iscc_lib` package) so Python consumers can
access it as a module-level constant, via `core_opts.meta_trim_meta`, and with proper type stubs.
This is the first binding propagation step for issue #18, starting with the highest-priority
binding.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add `m.add("META_TRIM_META", iscc_lib::META_TRIM_META)?;`
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add import, `__all__` entry, and `core_opts`
        attribute
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stub with docstring
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` — verify `META_TRIM_META` is `pub const` (already confirmed)
    - Existing constant patterns in all 3 files above (META_TRIM_NAME, META_TRIM_DESCRIPTION)

## Not In Scope

- Exporting META_TRIM_META in the other 5 bindings (Node.js, WASM, C FFI, Java, Go) — separate step
- Adding `gen_sum_code_v0` or `SumCodeResult` to any binding
- Updating documentation pages or READMEs for the new constant
- Adding Python tests for `gen_meta_code_v0` payload validation (Rust core already tests this)

## Implementation Notes

Follow the exact pattern used by `META_TRIM_NAME` and `META_TRIM_DESCRIPTION` in each file:

**1. `crates/iscc-py/src/lib.rs`** (Rust PyO3 module init, line ~608):

Add after the existing `m.add("META_TRIM_DESCRIPTION", ...)` line:

```rust
m.add("META_TRIM_META", iscc_lib::META_TRIM_META)?;
```

**2. `crates/iscc-py/python/iscc_lib/__init__.py`** (3 changes):

- Import: add `META_TRIM_META as META_TRIM_META,` in the `_lowlevel` import block (alphabetical
    order — after `META_TRIM_DESCRIPTION`, before `TEXT_NGRAM_SIZE`)
- `core_opts`: add `meta_trim_meta=META_TRIM_META,` after the `meta_trim_description` line
- `__all__`: add `"META_TRIM_META",` after `"META_TRIM_DESCRIPTION",` (alphabetical order)

**3. `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`** (type stub):

Add after the `META_TRIM_DESCRIPTION` stub (after line 10):

```python
META_TRIM_META: int
"""Max byte length for decoded meta parameter payload (128,000)."""
```

## Verification

- `cargo test -p iscc-py` passes (existing Rust-side tests still work)
- `uv run pytest` passes (all existing Python tests still pass)
- `python -c "from iscc_lib import META_TRIM_META; assert META_TRIM_META == 128_000"` exits 0
- `python -c "from iscc_lib import core_opts; assert core_opts.meta_trim_meta == 128_000"` exits 0
- `python -c "import iscc_lib; assert 'META_TRIM_META' in iscc_lib.__all__"` exits 0
- `cargo clippy -p iscc-py -- -D warnings` clean

## Done When

All six verification commands pass — `META_TRIM_META` is accessible as a Python module constant, in
`core_opts`, and listed in `__all__`, with existing tests unaffected.
