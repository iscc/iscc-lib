## 2026-11-21 — Python video frame values out of `i32` range raise `OverflowError` on every platform

**Done:** `extract_frame_sigs` reads each frame value with `PyLong_AsLongLong` (64 bits on every
platform) instead of `PyLong_AsLong` (C `long`, 32 bits on Windows), and maps the `i32::try_from`
failure to `PyOverflowError` (same message). Both `gen_video_code_v0` and `soft_hash_video_v0` go
through it; tests pin both.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: import `PyOverflowError`; `extract_frame_sigs` doc comment, SAFETY
    comment, `PyLong_AsLongLong` call, `PyOverflowError` mapping. Empty/ragged/non-sequence errors
    untouched (`ValueError`)
- `tests/test_int_range.py`: module docstring sentence on video; 4 parametrized tests — both entry
    points × `2**31`, `-(2**31) - 1`, `2**63` raise `OverflowError` (raw call alone inside
    `pytest.raises`, one bad value per call); both entry points × `2**31 - 1`, `-(2**31)` accepted
    (dict with `ISCC:` iscc / 8-byte `bytes`)
- `docs/howto/python.md`: "in a feature list" → "in a feature list or a frame signature" (mdformat
    reflowed the paragraph; code example unchanged)

**Verification:**

- `uv run maturin develop -m crates/iscc-py/Cargo.toml` then
    `uv run pytest tests/test_int_range.py`: 16 passed
- Mutation: reverting the mapping to `PyValueError` (rebuilt) → 4 failed (`2147483648` and
    `-2147483649` on both functions), 12 passed. Restored and rebuilt. `2**63` stays green on Linux
    under the mutation because CPython raises `OverflowError` itself there; it pins the beyond-`i64`
    path
- `uv run pytest`: 794 passed
- `grep -n "PyLong_AsLong(" crates/iscc-py/src/lib.rs`: no output
- `grep -n PyOverflowError`: import line + the `i32::try_from` mapping in `extract_frame_sigs`
- `cargo clippy -p iscc-py --all-targets -- -D warnings`: clean
- `mise run check`: rc=0 (first run reflowed docs/howto/python.md via mdformat, second run clean)

**Next:** the Go `GOARCH=386` CI step (decided `normal`, issues.md), or the Ruby floor move to 3.3 /
stale spec + `CLAUDE.md` statements.

**Notes:**

- Not verified on Windows locally; the fix removes the platform dependence by construction (64-bit
    `long long` on all three OSes, and every out-of-`i32` value now ends in `OverflowError` either
    from CPython or from the mapping)
- Binding-only change: no CRAP or iai impact. No hot-path bench run (Python marshalling only; the
    per-value call cost of `PyLong_AsLongLong` matches `PyLong_AsLong`)
- The docs sentence "Python raises it while converting the argument, before the call reaches the
    library" stays accurate for frame values: the binding's conversion raises it before the core
    runs
