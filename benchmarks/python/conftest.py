"""Pytest configuration for benchmark file discovery.

Adds bench_*.py to the collection pattern so `pytest benchmarks/python/`
discovers benchmark modules alongside the default test_*.py pattern.
"""


def pytest_configure(config):
    """Add bench_*.py to python_files collection pattern."""
    current = config.getini("python_files")
    if "bench_*.py" not in current:
        current.append("bench_*.py")
