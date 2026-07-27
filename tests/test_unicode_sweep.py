"""Tests for the Unicode 16.0.0 differential sweep gate in ``scripts/unicode_sweep.py``.

The full 17.8M-comparison sweep is CI-only (its own job on CPython 3.14); this suite
pins the gate's fail-closed machinery fast and portably: the scalar denominator, the
comparisons-per-scalar arithmetic, the oracle-version and stale-extension guards, and
— by monkeypatching the oracle to a wrong answer — that the comparison itself is
load-bearing rather than always-equal. Zero-divergence checks are guarded by the
interpreter's Unicode data version, since the oracle is only uniform 16.0.0 on
CPython 3.14.
"""

import importlib.util
import os
import unicodedata
from pathlib import Path

import pytest

# Load scripts/unicode_sweep.py by path — it is a repo gate script, not a package.
_SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "unicode_sweep.py"
_spec = importlib.util.spec_from_file_location("unicode_sweep", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
us = importlib.util.module_from_spec(_spec)
_loader.exec_module(us)

# Boundary-heavy sample: ASCII, the Final_Sigma pivot U+0295, unassigned U+0378,
# Sigma itself, a 16.0 emoji, two post-16.0 assignments, the sentinel, and the
# largest scalar.
SMALL_SCALARS = [
    0x0041,
    0x0295,
    0x0378,
    0x03A3,
    0x1FAE9,
    0x20C1,
    0x113C5,
    0xFFFF,
    0x10FFFF,
]


def test_scalar_values_count_and_no_surrogates():
    """scalar_values() yields exactly the scalar space and skips every surrogate."""
    scalars = list(us.scalar_values())
    assert len(scalars) == us.EXPECTED_SCALAR_COUNT
    assert set(range(0xD800, 0xE000)).isdisjoint(scalars)


def test_expected_comparisons_arithmetic():
    """Dropping a context cannot silently shrink the sweep past the pinned total."""
    assert len(us.CONTEXTS) == 8
    assert us.EXPECTED_COMPARISONS == us.EXPECTED_SCALAR_COUNT * len(us.CONTEXTS) * 2


def test_sweep_comparison_count():
    """sweep() performs contexts x functions comparisons per scalar."""
    count, _divergences = us.sweep(SMALL_SCALARS)
    assert count == len(SMALL_SCALARS) * len(us.CONTEXTS) * 2


@pytest.mark.skipif(
    unicodedata.unidata_version != "16.0.0",
    reason="oracle needs Unicode 16.0.0 tables (CPython 3.14)",
)
def test_sweep_zero_divergences_on_boundary_scalars():
    """The boundary-heavy sample diverges nowhere when the oracle is uniform 16.0.0."""
    _count, divergences = us.sweep(SMALL_SCALARS)
    assert divergences == []


def test_sweep_reports_divergence_with_wrong_oracle(monkeypatch):
    """A deliberately wrong oracle is reported — the comparison is load-bearing."""
    patched = tuple(
        (name, lambda _text: "wrong-oracle", subject)
        for name, _oracle, subject in us.FUNCTION_PAIRS
    )
    monkeypatch.setattr(us, "FUNCTION_PAIRS", patched)
    count, divergences = us.sweep([0x0041])
    assert count == len(us.CONTEXTS) * 2
    assert len(divergences) == count
    sample = divergences[0]
    assert sample.code_point == 0x0041
    assert sample.expected == "wrong-oracle"
    assert sample.actual != sample.expected


def test_check_oracle_rejects_pre_16_tables():
    """A pre-16.0 interpreter must fail closed, naming the required version."""
    with pytest.raises(SystemExit) as exc:
        us.check_oracle("15.1.0")
    assert "16.0.0" in str(exc.value)


def test_check_oracle_accepts_16():
    """The pinned oracle version passes the guard."""
    assert us.check_oracle("16.0.0") is None


def test_check_extension_fresh_stale_fires(tmp_path):
    """A Rust source newer than the extension fails closed, naming the rebuild."""
    extension, source_dir, rust_source = _freshness_fixture(tmp_path)
    os.utime(extension, (1_000_000, 1_000_000))
    os.utime(rust_source, (2_000_000, 2_000_000))
    with pytest.raises(SystemExit) as exc:
        us.check_extension_fresh(extension, [source_dir])
    assert "unicode:sweep" in str(exc.value)


def test_check_extension_fresh_current_passes(tmp_path):
    """An extension newer than every Rust source passes the guard."""
    extension, source_dir, rust_source = _freshness_fixture(tmp_path)
    os.utime(extension, (2_000_000, 2_000_000))
    os.utime(rust_source, (1_000_000, 1_000_000))
    us.check_extension_fresh(extension, [source_dir])


def _freshness_fixture(tmp_path):
    """Create a stand-in extension and one Rust source file under tmp_path."""
    extension = tmp_path / "_lowlevel.abi3.so"
    extension.write_bytes(b"")
    source_dir = tmp_path / "src"
    source_dir.mkdir()
    rust_source = source_dir / "lib.rs"
    rust_source.write_text("// stand-in\n", encoding="utf-8")
    return extension, source_dir, rust_source
