# Next Work Package

## Step: Python video frame values out of `i32` range raise `OverflowError` on every platform

## Goal

Fix the issue "Python video frame values raise a platform-dependent exception class" (`normal`,
`[audit]`): `gen_video_code_v0` and `soft_hash_video_v0` must raise `OverflowError` for a frame
value outside the `i32` range on Linux, macOS and Windows alike, the class the 2026-11-21 decision
keeps for Python integer conversion failures (and that `gen_audio_code_v0` already raises).

## Alternatives Considered

- **Chosen:** the Python video exception class. It is a correctness fix on a published surface,
    small (1 non-test file plus one docs sentence), and the review handoff suggested it
- **Rejected:** the Go `GOARCH=386` CI step. It is equally small, but it adds a test leg and changes
    no behaviour users see, so it comes after this fix. Go canonical `meta` was also rejected for
    now: the lone-surrogate vector first needs a check that every harness's JSON parser accepts
    `\ud800`

## Scope

- **Create**: none
- **Modify**: `crates/iscc-py/src/lib.rs` (`extract_frame_sigs` only), `docs/howto/python.md` (the
    `OverflowError` paragraph under error handling), `tests/test_int_range.py`
- **Reference**: `crates/iscc-py/src/lib.rs:28-100`, `tests/test_int_range.py`, decisions.md entry
    "2026-11-21 — Out-of-range integer arguments are per-binding tests, not `data-edge.json`
    vectors"

## Not In Scope

- Changing the exception class of anything else in `crates/iscc-py/src/lib.rs`: the empty-frame,
    ragged-frame and non-sequence errors stay `ValueError`, and so does `checked_id_arg` (IDv1 has
    its own documented `ValueError` contract)
- A `data-edge.json` vector: the decision cited above makes out-of-range integers per-binding tests
- Touching other bindings' video paths or the flat-buffer video entry point (it takes bytes and
    converts no Python `int`)
- Restructuring the `unsafe` extraction loop or its fast path

## Implementation Notes

- Root cause: `PyLong_AsLong` returns a C `long`, which is 32 bits on Windows and 64 bits on
    Linux/macOS. A value from 2^31 to 2^63-1 therefore gets CPython's `OverflowError` on Windows but
    reaches `i32::try_from` elsewhere, which maps the failure to `PyValueError` (`lib.rs:90-94`).
- Fix: read the value with `pyo3::ffi::PyLong_AsLongLong` (64 bits everywhere, so a value beyond
    `i64` gets CPython's `OverflowError` on every platform), and map the `i32::try_from` failure to
    `pyo3::exceptions::PyOverflowError` with the same message. Keep the `-1` + `PyErr_Occurred`
    check: it passes on CPython's own `OverflowError` and the `TypeError` for a non-int.
- Update the doc comment on `extract_frame_sigs` and the SAFETY comment, which both name
    `PyLong_AsLong`.
- Both `gen_video_code_v0` (line ~184) and `soft_hash_video_v0` (line ~601) go through
    `extract_frame_sigs`, so one change covers both. The tests must pin both entry points.
- Tests in `tests/test_int_range.py`, one bad value per call (learnings: one bad case per call),
    each on the raw API call with nothing else inside `pytest.raises`. Parametrize over at least
    `2**31`, `-(2**31) - 1` and `2**63` (beyond `i64`), for both functions, e.g.
    `iscc_lib.gen_video_code_v0([[v] + [0] * 379])` (frames have 380 values). Add one boundary
    positive case: `2**31 - 1` and `-(2**31)` are accepted (the call returns a dict / bytes).
- `docs/howto/python.md` ~line 552: extend "an `int` beyond 32 bits in a feature list" so it also
    covers frame signature values. Wording only, keep the code example.
- Binding-only change: neither the CRAP gate nor the iai baseline is affected.
- Rebuild the extension before running pytest (`uv run maturin develop -m crates/iscc-py/Cargo.toml`
    or the project's usual dev build); a stale `.so` silently runs the old code.

## Verification

- `uv run pytest tests/test_int_range.py` passes, including the new video cases for both functions
- `uv run pytest` passes (whole Python suite)
- `grep -n "PyLong_AsLong(" crates/iscc-py/src/lib.rs` prints nothing
- `grep -n "PyOverflowError" crates/iscc-py/src/lib.rs` shows the `i32::try_from` mapping in
    `extract_frame_sigs`
- `cargo clippy -p iscc-py --all-targets -- -D warnings` clean
- `mise run check` exits 0

## Done When

All verification criteria pass, and an out-of-range frame value raises `OverflowError` from both
video entry points, pinned by tests that do not depend on the platform's `long` width.
