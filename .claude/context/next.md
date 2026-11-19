# Next Work Package

## Step: Repair the data-edge harnesses — CRAP-clean Rust helpers, tight Python rejection checks

## Goal

Fix the three findings from the iter-215 NEEDS_WORK review so the unpushed data-edge commit at HEAD
passes the enforcing CI CRAP gate and `develop` can be pushed green. The Python harness shape gets
fixed now because the 11-surface fan-out will copy it. This is a repair of the same step with the
fix the review prescribed, not a third attempt at a failed approach.

## Alternatives Considered

- **Chosen:** harness repair. HEAD is CI-red (CRAP `--fail-above`) and unpushed, so it comes before
    everything else, and the fan-out must not copy a flawed harness shape.
- **Rejected:** the binding fan-out of `data-edge.json`. It stacks 10 surfaces on a red, unpushed
    HEAD and would copy the `pytest.raises` / rejection-tuple flaws. The Python `encode_seq`
    docstring issue (`normal`) is unrelated to this repair; it stays out to keep the diff small and
    a CI result easy to attribute.

## Scope

- **Modify**: `crates/iscc-lib/tests/test_data_edge.rs`, `tests/test_data_edge.py`,
    `crates/iscc-lib/tests/data-edge.json` (add one vector, see note 3)
- **Reference**: `.claude/context/specs/edge-vectors.md` (Rules), `.cargo-crap.toml`,
    `.github/workflows/ci.yml` lines 386-410 (the CRAP job)

## Not In Scope

- Changing `.cargo-crap.toml` (no `exclude` for `crates/iscc-lib/tests/**`, no threshold change) or
    `--epsilon`. Only Titusz can narrow the gate
- Editing `.crap-baseline.json`. New functions show as "NEW" and do not trip `--fail-regression`;
    only `--fail-above` failed
- Vendoring `data-edge.json` into other binding trees, filling `VENDORED_COPIES`, or any binding
    harness (that is the next step)
- The Python `encode_seq` docstring issue, and the pre-existing
    `clippy --all-targets --no-default-features` E0432 in `benches/*.rs`
- Adding a local CRAP hook to `mise run check` (separate gate-maintenance decision)

## Implementation Notes

1. **Rust (CRAP).** Integration-test functions get 0% coverage in LCOV and `missing = "pessimistic"`
    applies, so CRAP = cc² + cc. Every function in the file must have cyclomatic complexity ≤ 5
    (score ≤ 30). Aim for ≤ 4 everywhere, including `check_doc`, which sits at exactly 30.0 today.
    Today `call` is cc 14 (210) and `check_vector` cc 12 (156); which constructs cargo-crap counts
    is not obvious, so measure with the CI lines instead of estimating. Suggested shape:
    - One small function per section (`call_decode`, `call_seq`, `call_text`, `call_mixed`,
        `call_iscc_code`) with a shared signature such as
        `fn(&[Value]) -> Result<Option<Outcome>, String>`. A `const SECTIONS: &[(&str, Caller)]`
        table replaces both the `match` and `KNOWN_SECTIONS` (one list, not two). Lookup is a `find`
        on the table, and the unknown-section error comes from the `None` case.
    - Keep the feature gating: `gen_text_code_v0` returns `Ok(None)` under
        `not(feature = "text-processing")`. Put the `#[cfg]` on the two `call_text` bodies, not on
        the table entry, so the section stays known in every feature set.
    - Split `check_vector` into small parts, for example `exception(vector) -> Option<&str>`, the
        call, and a `judge(expect_error, outcome, vector)` step. Split `check_doc`'s inner loop into
        a per-section helper.
    - Keep all 7 tests and their assertions unchanged; failure messages must keep the substrings the
        negative tests match. Check the result with CI's own lines (Verification), not by estimating.
2. **Python `pytest.raises` scope.** In `test_iscc_lib_outcome`, only the raw call
    `getattr(iscc_lib, section)(*vector["inputs"])` goes inside `pytest.raises(ValueError)`.
    Conversions (`dict()`, `int()`, `.hex()`) must also leave the `try` in `outcome()`, or a
    conversion `ValueError` on the reference side counts as a rejection. Split each caller into a
    raw call and a converter, so `pytest.raises` and the `try` both wrap only the raw call. Keep
    `ValueError` as the iscc_lib-side contract.
3. **Reference-side rejections.** Probed: `iscc_core.iscc_decode("")` raises `IndexError` (iscc_lib
    raises `ValueError`). Add `IndexError` to `REJECTIONS` and update its comment. No bare/broad
    `except Exception` (ruff BLE001), no `# noqa`. To prove that the "one edit" rule now holds, add
    `test_0005_empty_string` to the `iscc_decode` section of `data-edge.json`: `"inputs": [""]`, an
    `error` text, a `note`, and no `diverges` (both sides reject). Do not add it to `SEED_SET` (it
    is not a spec seed). The Rust harness must pass it unchanged, because the core rejects empty
    input (empty guard in `iscc_clean`).

Neither change touches `src/`, so iai Perf is unaffected; CRAP is affected only through the test
file, and the Verification checks that.

## Verification

- `cargo test -p iscc-lib --test test_data_edge` passes (7 tests), also with `--no-default-features`
    and with `--no-default-features --features text-processing`
- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` followed by
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0, and the report shows no `test_data_edge.rs` function scoring above 30
- `uv run pytest tests/test_data_edge.py tests/test_seq.py tests/test_vendored_fixtures.py` passes,
    including `iscc_decode/test_0005_empty_string` in both `test_iscc_lib_outcome` and
    `test_reference_outcome`
- In `tests/test_data_edge.py`, the `with pytest.raises(ValueError):` block contains only the raw
    `getattr(iscc_lib, section)(…)` call, and `REJECTIONS` includes `IndexError` (checked by
    reading)
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean, and
    `uv run ruff check tests/test_data_edge.py` exits 0 with no `noqa` in that file
- `mise run check` exits 0

## Done When

All six checks pass, with the CI CRAP gate line exiting 0 on the working tree.
