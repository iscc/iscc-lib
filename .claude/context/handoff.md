## 2026-11-21 — Cut the pytest gate time: benchmarks disabled by default, Tier 1 doc tests copy only what they read

**Done:** `pyproject.toml` now sets `addopts = "--benchmark-disable"`, so `tests/test_benchmarks.py`
runs each benchmarked call once as a smoke test. The documented reproduce command gains
`--benchmark-enable`. The Tier 1 doc tests read the repo sources once per session (`functools.cache`
→ `{rel: bytes}`) and write them into each `tmp_path`; the 14 early-return cases (13
`test_fire_unrecognised_crate_root_item` + `test_floor_rejects_collapsed_parse`) get `lib.rs` only
via `_copy_lib`.

**Files changed:**

- `pyproject.toml`: `addopts = "--benchmark-disable"` in `[tool.pytest.ini_options]`
- `docs/benchmarks.md`: "How to reproduce" command adds `--benchmark-enable`, plus one sentence
    saying the repo config disables benchmarks by default
- `tests/test_check_tier1_docs.py`: `_tree_bytes` (cached), `_write`, `_copy_tree` (full tree from
    cache), `_copy_lib` (lib.rs only); `shutil` import dropped; module docstring updated. No
    assertion changed

**Verification:**

- `uv run pytest` — exit 0, 763 passed in 18.76 s (22 s wall). Before: 763 passed in 45.71 s. Top 10
    `--durations=10` after: tier1 `test_real_repo_passes` 0.86 s, spec_paths `test_real_repo_passes`
    0.40 s, tier1 `test_fire_pub_fn_added_to_lib_rs` 0.38 s,
    `test_seq.py::test_differential_rejects_space_like_chars_python_keeps` 0.38 s,
    `test_text_utils.py::test_differential_space_like_chars_kept_by_both` 0.36 s,
    `test_cid.py::test_run_metrics_snapshot_appends_and_commits` 0.33 s, tier1
    `test_fire_unregistered_doc_over_threshold` 0.30 s,
    `test_cid.py::test_enforce_meta_safety_flags_human_review_on_conflict` 0.28 s,
    `test_gil.py::test_gen_text_code_v0_concurrent_matches_single_thread` 0.26 s,
    `test_check_ci_job_table.py::test_real_repo_passes` 0.25 s. Nothing over 1 s remains
- `uv run pytest tests/test_benchmarks.py` — 18 passed in 0.67 s; `grep -c "Name (time in"` = 0
- `uv run pytest benchmarks/python/ --benchmark-enable --benchmark-only --benchmark-columns=mean,stddev,rounds`
    — exit 0, 18 passed, 9 stats tables (`Name (time in` ×9). Without `--benchmark-enable` it errors
    `Can't have both --benchmark-only and --benchmark-disable` (confirmed)
- `uv run pytest tests/test_check_tier1_docs.py` — 36 passed in 2.99 s
- `uv run zensical build` — exit 0, "No issues found"
- `mise run check` — exit 0

**Next:** The C FFI `(NULL, 0)` unification (`normal` `[audit]`) as its own step, as planned.

**Notes:**

- `git grep` finds no other `--benchmark-only` invocation (CI, mise, scripts) besides
    `docs/benchmarks.md`. The pre-push hook (`uv run pytest --timeout=120`) and CI's `uv run pytest`
    both inherit `addopts`
- The `uv run` warning about `VIRTUAL_ENV` not matching the project env is environment noise; I ran
    with `VIRTUAL_ENV=` exported
- No other slow module worth naming: the slowest test after the change is 0.86 s
