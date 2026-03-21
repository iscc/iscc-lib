## 2026-03-21 — Review of: Add pytest-benchmark comparing iscc-lib vs iscc-core

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation adding 18 pytest-benchmark functions (9 gen\_\*\_v0
functions x 2 implementations) comparing iscc-lib vs iscc-core. All verification criteria pass, all
quality gates clean, no suppressions or scope violations. Code is simple, correct, and uses real
conformance data with output assertions.

**Verification:**

- [x] `uv run pytest tests/test_benchmarks.py --benchmark-only -q` exits 0 — 18 passed in 12.38s
- [x] benchmark output grep shows 18 `bench_` entries
- [x] `uv run pytest tests/ --benchmark-disable -q` — 229 passed in 0.92s
- [x] `uv run ruff check tests/test_benchmarks.py` — clean
- [x] `uv run ruff format --check tests/test_benchmarks.py` — clean
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** One P2 finding — benchmarks are collected by default `uv run pytest` (CI and
pre-push hook), adding ~11s calibration overhead per run. Valid observation. Mitigations for future:
add `--benchmark-disable` to CI pytest step or `addopts` in pyproject.toml, and use
`--benchmark-only` for dedicated benchmark runs. Not blocking — the overhead is acceptable and
doesn't cause timeouts.

**Next:** The "Benchmarks" target section has one remaining unmet criterion: "Speedup factors
published in documentation." This could be a natural next step — add a performance comparison table
to `docs/` showing the speedup factors observed (meta ~20x, text ~33x, image ~15x, audio ~50x,
etc.). Additionally, consider adding `--benchmark-disable` to the CI pytest command and pre-push
hook to avoid the ~11s overhead on every run. Both remaining issues in issues.md are `low` priority.

**Notes:**

- The advance agent incorrectly claimed `mise run check` fails due to trailing whitespace in
    `swift-bindings.md` — it passes cleanly. The pre-existing issue may have been auto-fixed by
    hooks in a prior commit
- Representative speedups: meta ~20x, text ~33x, image ~15x, audio ~50x, video ~13x, mixed ~30x,
    data ~11x, instance ~62x, iscc ~20x
- `load_vectors()` re-parses data.json 9 times at module load — minor inefficiency, happens once,
    doesn't affect benchmark results
