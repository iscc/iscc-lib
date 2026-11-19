## 2026-11-19 — Repair the data-edge harnesses — CRAP-clean Rust helpers, tight Python rejection checks

**Done:** Split the Rust harness into small helpers so every function in `test_data_edge.rs` scores
≤ 20 in CRAP (cc ≤ 4), down from 210 and 156. A `SECTIONS: &[(&str, Caller)]` table replaces the
`match` and `KNOWN_SECTIONS`. In the Python harness, `pytest.raises` and `outcome()`'s `try` now
wrap only the raw call, and `REJECTIONS` includes `IndexError`. Added
`iscc_decode/test_0005_empty_string`: the Rust harness passed it unchanged, and Python passes it now
that `REJECTIONS` includes `IndexError`.

**Files changed:**

- `crates/iscc-lib/tests/test_data_edge.rs`: one caller per section (`call_decode`, `call_seq`,
    `call_text` ×2 cfg bodies, `call_mixed`, `call_iscc_code`) plus a `to_outcome` helper; the
    `SECTIONS` table + `find_caller`; `check_vector` split into `exception` / `expects_error` /
    `run_vector` / `judge` (`judge_ok`, `judge_err`, `judge_output`); `check_doc` → per-section
    `check_section`. The 7 tests and their assertions are unchanged; only
    `covers_every_known_section` now iterates `SECTIONS` instead of `KNOWN_SECTIONS`
- `tests/test_data_edge.py`: `CALLERS` → `CONVERTERS` (`convert_iscc_decode`, `convert_encode_seq`,
    `dict`). The raw call is `getattr(module, section)(*inputs)` everywhere; conversion happens
    after the `try`. `REJECTIONS = (ValueError, AssertionError, IndexError)`, comment updated
- `crates/iscc-lib/tests/data-edge.json`: `iscc_decode/test_0005_empty_string` (`inputs: [""]`,
    `error`, `note`, no `diverges`; not in `SEED_SET`)

**Verification:**

- `cargo test -p iscc-lib --test test_data_edge`: 7 passed under the default features,
    `--no-default-features`, `--no-default-features --features text-processing` and `--all-features`
- CI CRAP lines (lcov written to `/tmp/lcov.info`): `cargo llvm-cov -p iscc-lib --lcov …` exit 0,
    then `cargo crap --lcov … --baseline .crap-baseline.json --fail-regression --fail-above` **exit
    0** (`0 regressed, 26 new, 119 unchanged`). Highest `test_data_edge.rs` scores: `check_vector`
    20.0 (cc 4); `call_text`, `call_mixed`, `judge_output` and `judge` 12.0 (cc 3); the rest ≤ 6. A
    first cut had `judge` and `check_vector` at 30.0 (cc 5), so I flattened them
- Observed cargo-crap 0.2.2 counting: each `?` and each `match` arm adds 1; `let … else`, closures
    and `.then()` add nothing (for example `check_section` = 1, `call_text` with 2 `?` = 3)
- `uv run pytest tests/test_data_edge.py tests/test_seq.py tests/test_vendored_fixtures.py`: 154
    passed (151 before: 3 new rows, all for `iscc_decode/test_0005_empty_string`: schema,
    `test_iscc_lib_outcome` and `test_reference_outcome`, each PASSED in `-v`)
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` exit 0 (re-run after the final edit);
    `--tests --no-default-features` clean
- `uv run ruff check tests/test_data_edge.py`: all checks passed; `grep -c noqa` = 0; ruff `S,C901`
    clean; `uv run ty check` clean
- `mise run format` + `mise run check`: exit 0, and the tree was unchanged afterwards apart from my
    3 files

**Next:** The binding fan-out of `data-edge.json` to the 11 surfaces plus the 4 vendored copies
(fill `VENDORED_COPIES`). Copy this Python shape: keep the raw call apart from the converter, and
assert the rejection on the raw call only. The Rust shape (a table of small callers) keeps any
compiled harness CRAP-clean.

**Notes:**

- No `src/` change, so iai Perf is unaffected. No change to `.cargo-crap.toml` or
    `.crap-baseline.json`; the new functions show as NEW and pass `--fail-above`
- I used `IndexError`, not `LookupError` as review had suggested, because next.md names `IndexError`
    and the narrower class keeps `KeyError` out
- The `error` text of the new vector ("empty ISCC string") is descriptive only; no harness matches
    error text
