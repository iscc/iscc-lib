"""Tests for text utility functions and conformance_selftest exposed via Python bindings."""

from iscc_lib import (
    conformance_selftest,
    text_clean,
    text_collapse,
    text_remove_newlines,
    text_trim,
)


# ── conformance_selftest ────────────────────────────────────────────────────


def test_conformance_selftest_returns_true():
    """Verify conformance_selftest passes all vendored test vectors."""
    assert conformance_selftest() is True


# ── text_clean ──────────────────────────────────────────────────────────────


def test_text_clean_removes_control_chars():
    """Verify tab and other control chars are removed."""
    assert text_clean("hello\tworld") == "helloworld"


def test_text_clean_preserves_newlines():
    """Verify newline characters are kept."""
    assert text_clean("hello\nworld") == "hello\nworld"


def test_text_clean_collapses_empty_lines():
    """Verify consecutive empty lines collapse to one."""
    assert text_clean("a\n\n\nb") == "a\n\nb"


def test_text_clean_strips_whitespace():
    """Verify leading/trailing whitespace is stripped."""
    assert text_clean("  hello  ") == "hello"


def test_text_clean_normalizes_crlf():
    """Verify \\r\\n is normalized to \\n."""
    assert text_clean("a\r\nb") == "a\nb"


def test_text_clean_empty():
    """Verify empty input returns empty string."""
    assert text_clean("") == ""


def test_text_clean_nfkc():
    """Verify NFKC normalization (e.g., ℍ → H)."""
    result = text_clean("ℍ")
    assert "H" in result


# ── text_remove_newlines ────────────────────────────────────────────────────


def test_text_remove_newlines_basic():
    """Verify newlines are replaced with spaces."""
    assert text_remove_newlines("hello\nworld") == "hello world"


def test_text_remove_newlines_collapses_whitespace():
    """Verify multiple whitespace characters collapse to single space."""
    assert text_remove_newlines("a  b   c") == "a b c"


def test_text_remove_newlines_multiline():
    """Verify multiple newlines are handled."""
    assert text_remove_newlines("a\n\nb\nc") == "a b c"


def test_text_remove_newlines_empty():
    """Verify empty input returns empty string."""
    assert text_remove_newlines("") == ""


# ── text_trim ───────────────────────────────────────────────────────────────


def test_text_trim_no_truncation():
    """Verify text shorter than limit is returned as-is (stripped)."""
    assert text_trim("hello", 10) == "hello"


def test_text_trim_exact():
    """Verify text at exact limit is returned as-is."""
    assert text_trim("hello", 5) == "hello"


def test_text_trim_truncates():
    """Verify text is truncated at byte boundary."""
    assert text_trim("hello world", 5) == "hello"


def test_text_trim_unicode_boundary():
    """Verify multi-byte chars that would be split are dropped."""
    # "é" is 2 bytes in UTF-8 (C3 A9). Trimming at 1 byte drops it.
    assert text_trim("é", 1) == ""


def test_text_trim_strips_result():
    """Verify trailing whitespace is stripped after trimming."""
    assert text_trim("hello ", 6) == "hello"


# ── text_collapse ───────────────────────────────────────────────────────────


def test_text_collapse_basic():
    """Verify basic lowercasing and whitespace removal."""
    assert text_collapse("Hello World") == "helloworld"


def test_text_collapse_strips_accents():
    """Verify accented characters have their marks removed."""
    assert text_collapse("café") == "cafe"


def test_text_collapse_strips_punctuation():
    """Verify punctuation is removed."""
    assert text_collapse("hello, world!") == "helloworld"


def test_text_collapse_empty():
    """Verify empty input returns empty string."""
    assert text_collapse("") == ""


def test_text_collapse_unicode():
    """Verify Unicode text is handled correctly."""
    assert text_collapse("Ñoño") == "nono"
