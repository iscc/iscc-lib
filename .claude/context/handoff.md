## 2026-11-19 — Review of: Repair the data-edge harnesses — CRAP-clean Rust helpers, tight Python rejection checks

**Verdict:** PASS

**Summary:** The three iter-215 findings are fixed within the three scoped files. The Rust harness
is now a `SECTIONS` table of small callers, and its highest CRAP score is 20 (`check_vector`, cc 4),
down from 210. The Python `pytest.raises` and `outcome()`'s `try` now wrap only the raw
`getattr(module, section)(*inputs)` call. `REJECTIONS` gains `IndexError`, and
`iscc_decode/test_0005_empty_string` passed without any harness edit. The batch also carries the
iter-215 data-edge commit, which is now green, so the whole range is pushed.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge`: 7 passed under default,
    `--no-default-features`, `--no-default-features --features text-processing` and `--all-features`
- [x] CI CRAP lines (rustc 1.100.0): `cargo llvm-cov` exit 0, then
    `cargo crap … --fail-regression --fail-above` exit 0 (0 regressed, 26 new, 119 unchanged). No
    `test_data_edge.rs` function scores above 20
- [x] `uv run pytest tests/test_data_edge.py tests/test_seq.py tests/test_vendored_fixtures.py`: 154
    passed. `iscc_decode/test_0005_empty_string` PASSED in `test_vector_schema`,
    `test_iscc_lib_outcome` and `test_reference_outcome`
- [x] By reading: `with pytest.raises(ValueError):` holds only the raw
    `getattr(iscc_lib, section)(…)` call, conversion sits outside the `try` in `outcome()`, and
    `REJECTIONS` includes `IndexError`
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean (also
    `--tests --no-default-features`). `ruff check tests/test_data_edge.py` passes, with 0 `noqa`
- [x] `mise run check` exit 0, and the tree was unchanged afterwards
- Extra checks (3): `iscc_core.iscc_decode("")` raises `IndexError` and `iscc_lib` raises
    `ValueError: Empty ISCC string`, as the vector's note says. No other `pytest.raises` in `tests/`
    wraps a conversion. The full pre-push stage (`prek run --hook-stage pre-push --all-files`) exits
    0 over the 13-commit push range, which includes the human `tools/cid.py` / `tests/test_cid.py`
    changes

**Issues found:**

- (none filed) Nit: the new vector's `note` calls the Rust error "ValueError-class", which is Python
    terminology. Reword it to "rejects" when the file is vendored
- Gate scan of every unpushed commit (`@{upstream}..HEAD`, 13 commits): no suppression, skip,
    threshold or hook change. The only manifest change is `exclude` for the data-edge files in
    `crates/iscc-lib/Cargo.toml` (from iter 215, per spec)

**Codex review:** Ran on HEAD and found nothing to act on. It confirmed that dispatch, feature
gating and failure reporting are preserved, and re-ran the 154 Python tests and the 7 Rust tests.

**Next:** The binding fan-out of `data-edge.json`: the 11 surfaces plus the vendored copies
(register them in `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`). Declare a `**Fan-out:**`
line that names the decided behaviour (`edge-vectors.md` → Rules: every vector runs, only `except`
deviates, an unknown section fails) and make one commit per surface. Each harness copies the Python
shape: the raw call is separate from conversion, and the rejection is asserted on the raw call only.
C and C++ may read a generated table guarded by a staleness check (generator in commit `ea69169`).
Fallback if the fan-out is judged too big: the `normal` Python `encode_seq` docstring issue, a
one-file fix.

**Notes:**

- `iscc_decode("")` is proven on Rust and Python only. Go has its own decoder, so the fan-out should
    expect this vector to fail there first. A real Go defect is fixed in Go, never by an `except`
    entry
- The CRAP gate scores only `-p iscc-lib`, so binding harnesses are not CRAP-scored. The 0% coverage
    trap applies only to `crates/iscc-lib/tests/*.rs`
- `learnings.md` is at 199/200 lines and `issues.md` at 292/300. Any addition must be net-zero
