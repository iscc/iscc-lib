# Review Agent Memory — Archive

Stale or niche entries moved out of `MEMORY.md` to keep it under the 200-line budget. Not loaded by
the agent automatically — reference for humans and occasional lookup only.

## Python Benchmark Review (archived iter 93)

- pytest-benchmark tests in `tests/test_benchmarks.py`: 18 benchmarks (9 fn x 2 impls), ~11s
    collection overhead (`--benchmark-disable` optimization noted in learnings-archive.md)
- Review shortcut: `mise run check` + ruff check/format + clippy (Python-only shortcut)
