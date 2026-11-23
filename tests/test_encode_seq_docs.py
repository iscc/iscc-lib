"""Python ``encode_seq`` docstrings name every strict-base32 divergence from iscc_core.

iscc-lib rejects ``=`` padding, non-zero spare bits and non-ASCII case-fold aliases
(``ſ``, ``ß``) in ``encode_seq`` unit strings, while ``iscc_core`` accepts all three
(pinned in ``data-edge.json``). Each of the three Python-visible docstrings — the
wrapper, the compiled extension and the ``.pyi`` stub — must name all three.
"""

import ast
from pathlib import Path

import iscc_lib
from iscc_lib import _lowlevel

_PYI = (
    Path(__file__).resolve().parents[1] / "crates/iscc-py/python/iscc_lib/_lowlevel.pyi"
)
_PHRASES = ("padding", "spare bits", "case-fold", "iscc_core")


def _pyi_docstring(name):
    """Return the docstring of the top-level function ``name`` in ``_lowlevel.pyi``."""
    tree = ast.parse(_PYI.read_text(encoding="utf-8"))
    for node in tree.body:
        if isinstance(node, ast.FunctionDef) and node.name == name:
            return ast.get_docstring(node) or ""
    raise AssertionError(f"{name} not found in {_PYI}")


def _missing(doc):
    """Return the required phrases absent from ``doc`` (whitespace-normalized)."""
    flat = " ".join((doc or "").split())
    return [p for p in _PHRASES if p not in flat]


def test_wrapper_docstring_names_divergences():
    """The public ``iscc_lib.encode_seq`` wrapper names all three divergences."""
    assert _missing(iscc_lib.encode_seq.__doc__) == []


def test_extension_docstring_names_divergences():
    """The compiled ``_lowlevel.encode_seq`` docstring names all three divergences."""
    assert _missing(_lowlevel.encode_seq.__doc__) == []


def test_pyi_docstring_names_divergences():
    """The ``_lowlevel.pyi`` stub docstring names all three divergences."""
    assert _missing(_pyi_docstring("encode_seq")) == []


def test_missing_flags_two_item_wording():
    """The pre-fix two-item wording is caught as missing the case-fold phrase."""
    old = "Base32 decoding is strict: `=` padding and non-zero spare bits are rejected."
    assert _missing(old) == ["case-fold", "iscc_core"]
