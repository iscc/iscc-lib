"""Type-name parity between the generated C header and the C FFI docs.

cbindgen prefixes every exported type with ``iscc_`` (``[export] prefix`` in
``crates/iscc-ffi/cbindgen.toml``), so ``IsccByteBuffer`` in Rust is
``iscc_IsccByteBuffer`` in ``iscc.h``. The C-facing docs must use the prefixed names, or
copied snippets do not compile. The type set is derived from ``iscc.h`` itself, so a
type added to the header is checked without editing this file. Pure Python, no compiler.
"""

import re
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[1]
_HEADER = _ROOT / "crates" / "iscc-ffi" / "include" / "iscc.h"
_DOC_FILES = [
    _ROOT / "docs" / "c-ffi-api.md",
    _ROOT / "crates" / "iscc-ffi" / "README.md",
    _ROOT / "docs" / "howto" / "c-cpp.md",
]
_TYPEDEF_RE = re.compile(r"typedef struct iscc_(\w+)")


def header_type_names(header_text: str) -> set[str]:
    """Return the bare (unprefixed) names of every ``typedef struct iscc_X`` in the header."""
    return set(_TYPEDEF_RE.findall(header_text))


def find_bare_names(text: str, names: set[str]) -> list[tuple[int, str]]:
    """Return ``(line_number, name)`` for each bare type name at an identifier boundary."""
    pattern = re.compile(
        r"(?<![A-Za-z0-9_])("
        + "|".join(sorted(names, key=len, reverse=True))
        + r")(?![A-Za-z0-9_])"
    )
    hits = []
    for lineno, line in enumerate(text.splitlines(), start=1):
        hits.extend((lineno, m.group(1)) for m in pattern.finditer(line))
    return hits


def _names() -> set[str]:
    """Return the type names derived from the committed ``iscc.h``."""
    return header_type_names(_HEADER.read_text(encoding="utf-8"))


def test_header_parse_finds_known_types():
    """The header parse yields the known types, so an empty set cannot pass the docs check."""
    names = _names()
    assert "IsccByteBuffer" in names
    assert "FfiDataHasher" in names


def test_c_ffi_api_doc_uses_prefixed_names():
    """``docs/c-ffi-api.md`` names no exported type without its ``iscc_`` prefix."""
    path = _DOC_FILES[0]
    hits = find_bare_names(path.read_text(encoding="utf-8"), _names())
    assert hits == [], [
        f"{path.relative_to(_ROOT)}:{line}: {name}" for line, name in hits
    ]


def test_ffi_readme_uses_prefixed_names():
    """``crates/iscc-ffi/README.md`` names no exported type without its ``iscc_`` prefix."""
    path = _DOC_FILES[1]
    hits = find_bare_names(path.read_text(encoding="utf-8"), _names())
    assert hits == [], [
        f"{path.relative_to(_ROOT)}:{line}: {name}" for line, name in hits
    ]


def test_c_cpp_howto_uses_prefixed_names():
    """``docs/howto/c-cpp.md`` names no exported type without its ``iscc_`` prefix."""
    path = _DOC_FILES[2]
    hits = find_bare_names(path.read_text(encoding="utf-8"), _names())
    assert hits == [], [
        f"{path.relative_to(_ROOT)}:{line}: {name}" for line, name in hits
    ]


def test_bare_name_is_flagged():
    """A bare ``IsccByteBuffer`` is reported with its line; the prefixed form is not."""
    text = "iscc_IsccByteBuffer ok;\nIsccByteBuffer buf = iscc_alg_simhash(d, l, n);\n"
    assert find_bare_names(text, {"IsccByteBuffer", "IsccByteBufferArray"}) == [
        (2, "IsccByteBuffer")
    ]


def test_bare_longer_name_is_flagged_whole():
    """``IsccByteBufferArray`` is reported as itself, not as its ``IsccByteBuffer`` prefix."""
    hits = find_bare_names(
        "IsccByteBufferArray arr;", {"IsccByteBuffer", "IsccByteBufferArray"}
    )
    assert hits == [(1, "IsccByteBufferArray")]
