"""Full-code-space differential sweep of `text_clean` / `text_collapse` on Unicode 16.0.0.

iscc-lib freezes its Unicode data version at 16.0.0 with vendored tables (the
unassigned-code-point sentinel map and the `Cased` / `Case_Ignorable` freeze), but
every *unconditional* lowercase mapping and all normalization tables still come from
the Rust toolchain's dependencies. This gate measures that unguarded residual: it
compares `iscc_lib.text_clean` / `iscc_lib.text_collapse` against the `iscc-core`
reference implementation running on CPython 3.14, whose `unicodedata` ships uniform
Unicode 16.0.0 tables. `iscc_core` calls `unicodedata` directly with no freeze rule of
its own, so post-16.0 code points are `Cn` to the oracle exactly as the sentinel makes
them for iscc-lib — on 3.14 it *is* the uniform-16.0.0-tables reference.

The denominator is fixed: 1,112,064 Unicode scalar values (`range(0x110000)` minus the
2,048 surrogates, which `&str` cannot carry) x 8 sequence contexts x 2 functions =
17,793,024 comparisons, and the divergence set must be empty. Both counts are asserted
before success is reported, so a run that generated zero cases cannot read green.

CI-only (its own `unicode-sweep` job) plus the `mise run unicode:sweep` escape hatch:
the sweep needs a --release extension build (~60 s even then) and CPython 3.14
specifically — an older interpreter carries pre-16.0 Unicode tables and could only
skip, which is exactly the fail-open shape this gate exists to avoid.

Usage:
    uv run scripts/unicode_sweep.py
"""

from __future__ import annotations

import platform
import sys
import unicodedata
from collections.abc import Callable, Iterable, Iterator, Sequence
from pathlib import Path
from typing import NamedTuple

import iscc_core

import iscc_lib
from iscc_lib import _lowlevel

# Fail-closed expectations: a mismatch means the wrong oracle or a shrunken sweep.
EXPECTED_UNIDATA_VERSION = "16.0.0"
EXPECTED_SCALAR_COUNT = 1_112_064
EXPECTED_COMPARISONS = 17_793_024
MAX_REPORTED_DIVERGENCES = 20

ROOT = Path(__file__).resolve().parent.parent
# Rust sources whose edits invalidate the built extension module.
RUST_SOURCE_DIRS = (
    ROOT / "crates" / "iscc-lib" / "src",
    ROOT / "crates" / "iscc-py" / "src",
)

# (name, prefix, suffix): each scalar `c` is swept as `prefix + c + suffix` through
# both functions. Rows 3-6 cover the four sequence classes the spec mandates
# (base+Cn+mark, jamo+Cn+jamo, Sigma+Cn+cased, Cn-between-marks) for *every* scalar,
# not only the unassigned ones. Non-ASCII pieces are built with chr() so this file
# stays pure ASCII.
CONTEXTS: tuple[tuple[str, str, str], ...] = (
    ("bare", "", ""),  # the single-code-point sweep proper
    ("ascii", "a", "b"),  # the boundary-fixture shape
    ("base_mark", "e", chr(0x0301)),  # base + c + combining acute
    ("jamo", chr(0x1100), chr(0x1161)),  # jamo + c + jamo
    ("sigma", chr(0x0391) + chr(0x03A3), chr(0x0392)),  # Alpha Sigma + c + Beta
    ("marks", chr(0x0301), chr(0x0301)),  # c between combining marks
    ("space", " ", " "),  # whitespace filter + .strip()
    ("upper", "A", "Z"),  # cased neighbours for lowercasing
)

# (name, oracle, subject) per function under sweep. Read by sweep() at call time so
# the pytest suite can monkeypatch the oracle to prove the comparison is load-bearing.
FUNCTION_PAIRS: tuple[tuple[str, Callable[[str], str], Callable[[str], str]], ...] = (
    ("text_clean", iscc_core.text_clean, iscc_lib.text_clean),
    ("text_collapse", iscc_core.text_collapse, iscc_lib.text_collapse),
)


class Divergence(NamedTuple):
    """One case where iscc-lib output differs from the iscc-core oracle."""

    function: str
    context: str
    code_point: int
    expected: str
    actual: str


def scalar_values() -> Iterator[int]:
    """Yield every Unicode scalar value: all code points minus the surrogate block."""
    for code_point in range(0x110000):
        if 0xD800 <= code_point <= 0xDFFF:
            continue
        yield code_point


def sweep(scalars: Iterable[int]) -> tuple[int, list[Divergence]]:
    """Compare oracle and subject output for every scalar in every context.

    Returns ``(comparison_count, divergences)``. Performs no printing; callers own
    reporting and the fail-closed count assertions.
    """
    comparisons = 0
    divergences: list[Divergence] = []
    for code_point in scalars:
        char = chr(code_point)
        for context, prefix, suffix in CONTEXTS:
            case = prefix + char + suffix
            for function, oracle, subject in FUNCTION_PAIRS:
                comparisons += 1
                expected = oracle(case)
                actual = subject(case)
                if actual != expected:
                    divergences.append(
                        Divergence(function, context, code_point, expected, actual)
                    )
    return comparisons, divergences


def check_oracle(unidata_version: str) -> None:
    """Fail closed unless the interpreter carries Unicode 16.0.0 oracle tables."""
    if unidata_version != EXPECTED_UNIDATA_VERSION:
        raise SystemExit(
            f"need Unicode {EXPECTED_UNIDATA_VERSION} oracle data, got "
            f"{unidata_version} — run under CPython 3.14, whose unicodedata ships "
            "16.0.0 tables"
        )


def check_extension_fresh(extension: Path, source_dirs: Sequence[Path]) -> None:
    """Fail closed when any Rust source is newer than the built extension module.

    A stale gitignored extension silently measures the previous commit, so the sweep
    refuses to run against one.
    """
    extension_mtime = extension.stat().st_mtime
    newest_source = max(
        (path.stat().st_mtime for src in source_dirs for path in src.rglob("*.rs")),
        default=0.0,
    )
    if newest_source > extension_mtime:
        raise SystemExit(
            f"{extension} is older than the Rust sources; rebuild and rerun via "
            "`mise run unicode:sweep`"
        )


def main() -> int:
    """Run the fail-closed differential sweep over the full scalar space."""
    check_oracle(unicodedata.unidata_version)
    extension_file = _lowlevel.__file__
    if extension_file is None:
        raise SystemExit("iscc_lib._lowlevel has no __file__; cannot check freshness")
    check_extension_fresh(Path(extension_file), RUST_SOURCE_DIRS)
    scalars = list(scalar_values())
    if len(scalars) != EXPECTED_SCALAR_COUNT:
        raise SystemExit(
            f"expected {EXPECTED_SCALAR_COUNT} scalar values, got {len(scalars)}"
        )
    comparisons, divergences = sweep(scalars)
    if comparisons != EXPECTED_COMPARISONS:
        raise SystemExit(
            f"expected {EXPECTED_COMPARISONS} comparisons, got {comparisons}"
        )
    print(
        f"oracle: iscc-core {iscc_core.__version__}, "
        f"unidata {unicodedata.unidata_version}, "
        f"python {platform.python_version()}"
    )
    for div in divergences[:MAX_REPORTED_DIVERGENCES]:
        print(
            f"DIVERGENCE {div.function} [{div.context}] U+{div.code_point:04X}: "
            f"expected {div.expected!r}, got {div.actual!r}"
        )
    print(f"TOTAL {comparisons} comparisons, {len(divergences)} divergences")
    return 1 if divergences else 0


if __name__ == "__main__":
    sys.exit(main())
