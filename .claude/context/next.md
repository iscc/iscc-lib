# Next Work Package

## Step: Cut the pytest gate time: benchmarks disabled by default, Tier 1 doc tests copy only what they read

## Goal

Pick up the `normal` `[audit]` issue "Pytest gate spends ~25 s of ~44 s on benchmark loops and
doc-tree copies". The pre-push pytest gate and CI's `uv run pytest` stop running timing loops and
stop re-reading about 90 docs off the slow 9p mount for every Tier 1 doc test case.

## Alternatives Considered

- **Chosen:** pytest gate time. It is 1 config file, 1 doc line and 1 test file, it touches no
    binding or core code, and every later step (FFI, integer range, docs) pays for the gate on each
    push. The handoff planned it.
- **Rejected:** C FFI `(NULL, 0)` unification (`normal` `[audit]`). It changes core FFI behaviour
    and flips tests on 3 surfaces, so it goes alone in the next step. The integer-range and
    published-docs issues come after it.

## Scope

- **Modify**: `pyproject.toml` (`[tool.pytest.ini_options]`), `docs/benchmarks.md` (the "How to
    reproduce" pytest command), `tests/test_check_tier1_docs.py`
- **Reference**: `scripts/check_tier1_docs.py` (`run_checks` returns early for unrecognised items
    and the floor, before any doc is read), `tests/test_benchmarks.py`,
    `benchmarks/python/conftest.py`

## Not In Scope

- Deleting or rewriting `tests/test_benchmarks.py`. It still runs each benchmarked call once as a
    smoke test when benchmarks are disabled. Whether it duplicates `benchmarks/python/` is a
    separate question.
- Changing `scripts/check_tier1_docs.py` or any assertion in the Tier 1 tests. Every test keeps its
    exact expected error list. Only where the fixture bytes come from changes.
- Speeding up other slow test modules found along the way. Name them in the handoff with their
    `--durations` numbers instead.
- Editing `notes/09-performance-benchmarks.md` (design record, not kept current).

## Implementation Notes

- **Benchmarks:** add `addopts = "--benchmark-disable"` to `[tool.pytest.ini_options]`. That ini
    also applies to `pytest benchmarks/python/`. Measured in the installed pytest-benchmark
    (`pytest_benchmark/session.py`): `--benchmark-disable` together with `--benchmark-only` raises
    `UsageError: Can't have both --benchmark-only and --benchmark-disable`, unless
    `--benchmark-enable` is also given (`disabled = disable and not enable`). So the reproduce
    command in `docs/benchmarks.md` must gain `--benchmark-enable`, or the documented command
    breaks. Add a short sentence there saying the repo config disables benchmarks by default.
- **Tier 1 doc tests:** two separate savings.
    1. The 13 `test_fire_unrecognised_crate_root_item` cases and `test_floor_rejects_collapsed_parse`
        return before any doc is read. Give them a helper that copies only `ct.LIB_RS` into
        `tmp_path`. `run_checks` still receives `TRACKED` and never opens the missing docs on these
        paths.
    2. The other `_copy_tree` callers need the full tree. Read the source bytes off the repo once per
        session (for example a `functools.cache` function returning `{rel: bytes}` for `ct.LIB_RS`
        and every non-skipped tracked doc), then write them into each `tmp_path`. Every test must
        still get its own writable copy, because the fire tests mutate files.
- `tmp_path` is under `/tmp` (local disk). The cost was reading off the `/workspace` 9p mount.
- A `tests/`-only and config-only step trips neither the CRAP nor the iai gate.

## Verification

- `uv run pytest` exits 0. The handoff reports its wall time and the top 10 of
    `uv run pytest --durations=10` (report-only, no threshold)
- `uv run pytest tests/test_benchmarks.py` exits 0 and its output does not contain the
    pytest-benchmark stats header `Name (time in`
- The pytest command in `docs/benchmarks.md` → "How to reproduce" contains `--benchmark-enable`, and
    running it with `uv run` exits 0 and prints the stats table
- `uv run pytest tests/test_check_tier1_docs.py` exits 0. Reading the file confirms that the 14
    early-return cases copy only `lib.rs` and that the repo docs are read once per session
- `uv run zensical build` exits 0
- `mise run check` exits 0

## Done When

The advance agent is done when all six verification criteria pass.
