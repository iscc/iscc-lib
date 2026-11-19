## 2026-11-19 — Review of: `data-edge.json` seed set with the Rust and Python harnesses

**Verdict:** NEEDS_WORK

**Summary:** The file, both harnesses and the `STRICT_BASE32_DIVERGENCES` migration are correct and
match the spec. All 15 vectors are right, and all six next.md checks pass. But the new Rust harness
fails the enforcing CI CRAP gate, so pushing would turn develop red. next.md had said "Neither iai
nor CRAP is affected". Two Python harness weaknesses (from Codex, both confirmed) should be fixed in
the same repair step, because the 11-surface fan-out will copy this harness shape.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge`: 7 passed, incl. the 4 negatives. Also 7/7
    under `--no-default-features`, `--all-features` and
    `--no-default-features --features   text-processing`
- [x] `uv run pytest tests/test_data_edge.py tests/test_seq.py tests/test_vendored_fixtures.py`: 151
    passed, incl. the 15 seed-presence rows
- [x] `grep -n STRICT_BASE32_DIVERGENCES tests/test_seq.py`: no match (exit 1)
- [x] `cargo package --list -p iscc-lib --allow-dirty`: only `tests/data.json`, no `data-edge` or
    `data_edge` line
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings`: clean
- [x] `mise run check`: exit 0, tree clean afterwards
- [ ] **CI CRAP gate** (not in next.md): CI's two lines
    (`cargo llvm-cov -p iscc-lib --lcov   --output-path lcov.info`, then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json   --fail-regression --fail-above`)
    **exit 1**. `call` (test_data_edge.rs:84) scores 210 and `check_vector` (:127) scores 156,
    against a threshold of 30. `check_doc` scores exactly 30.0 (warned, not failed)
- Extra checks: pre-push `uv run ty check` and ruff `S`/`C901` on the 3 Python files are clean. The
    unpushed `cid(human)` commits touch no gate config and add no suppressions

**Issues found:**

- **Blocking: the CRAP gate is red.** Files under `crates/iscc-lib/tests/*.rs` get no coverage in
    LCOV, and `.cargo-crap.toml` sets `missing = "pessimistic"`, so they score as 0% covered. At 0%,
    CRAP = cc² + cc, so every function in a Rust test file needs cyclomatic complexity ≤ 5. The 8
    test helpers already in `.crap-baseline.json` all score 12 or less
- **Repair item: `pytest.raises` covers more than the call.** `test_iscc_lib_outcome` wraps
    `CALLERS[section]`, which also converts the result with `dict()` / `int()`. Those raise
    `ValueError` themselves, so a binding that *returned* a string instead of raising would still
    pass the error vectors of `iscc_decode`, `gen_mixed_code_v0` and `gen_iscc_code_v0`
- **Repair item: the reference-side rejection tuple is too narrow.**
    `REJECTIONS = (ValueError,   AssertionError)`. `iscc_core.iscc_decode("")` raises `IndexError`
    (probed). So adding the known empty-input vector would need a harness edit, which breaks the
    spec rule "A vector costs one edit". ruff BLE001 flags both `except Exception:` and
    `except Exception as exc:` (probed). So widen the tuple, for example with `LookupError`; do not
    add `# noqa`
- Fixed on sight: stale `STRICT_BASE32_DIVERGENCES` pointers in the `packages/go/seq_test.go`
    comment and in the `encode_seq` docstring issue (issues.md)
- Not filed (minor, pre-existing): `cargo clippy -p iscc-lib --all-targets --no-default-features`
    fails with E0432 in `benches/*.rs`. CI does not run that combination
- Not verified: "absent from every published artifact" is checked only via `cargo package`. Add a
    Python sdist and Ruby gem check when the fan-out vendors the file

**Codex review:** Three findings, all confirmed. (P1) The CRAP gate fails. I reproduced it with CI's
exact commands: exit 1, scores 210 and 156. (P2) `IndexError` from the reference escapes
`outcome()`. Confirmed with `iscc_core.iscc_decode("")`. (P2) `pytest.raises(ValueError)` also wraps
the output conversion. Confirmed by reading the code: `dict("…")` and `int("I")` raise `ValueError`.

**Next:** A repair step on the two harness files only (`crates/iscc-lib/tests/test_data_edge.rs`,
`tests/test_data_edge.py`):

1. Split `call` and `check_vector` so that every function in `test_data_edge.rs` has cyclomatic
    complexity ≤ 5, ideally ≤ 4 so that `check_doc` is not sitting on 30.0. One small function per
    section, chosen from a table of function pointers, removes the big `match`. Keep the 4 negative
    tests and the feature-gated `gen_text_code_v0` arm. Put CI's two CRAP lines into Verification;
    the second must exit 0
2. In `test_iscc_lib_outcome`, put only the raw `getattr(iscc_lib, section)(*inputs)` inside
    `pytest.raises(ValueError)`. Convert results only when comparing outputs
3. Widen the reference-side rejections so that `IndexError` counts, with no `# noqa`. Keep
    `pytest.raises(ValueError)` on the iscc_lib side, because that is the documented contract

After that, the binding fan-out described in the advance handoff: copy `data-edge.json` next to the
4 vendored `data.json` copies, fill the empty `VENDORED_COPIES` entry, and add a harness on each of
the other 11 surfaces. The `iscc_decode` output encoding is `[mt, st, vs, li, "bytes:<hex>"]`.

**Notes:**

- Do **not** fix the CRAP failure by adding `crates/iscc-lib/tests/**` to `.cargo-crap.toml`'s
    `exclude` or by raising the threshold. That narrows the gate's scope, and only Titusz can change
    it
- New functions that are absent from the baseline do not trip `--fail-regression` (CI shows them as
    "NEW"); only `--fail-above` failed here. A baseline refresh is therefore optional, but only a
    rerun that exits 0 proves the gate passes
- Gate maintenance: this is another CRAP failure that only CI would catch (see learnings, "CI-ONLY
    gap"). Consider a cheap local equivalent; `mise run check` never runs CRAP
- Accepted, not a spec breach: under `--no-default-features` the harness returns `Ok(None)` for
    `gen_text_code_v0`, because that function is not compiled into that build
