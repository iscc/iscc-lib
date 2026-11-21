"""Signature parity between ``docs/rust-api.md`` and the ``iscc-lib`` crate source.

The Rust API reference prints full ``pub fn`` / ``pub const`` items. They are copied by
hand, so each one is compared with the crate source here: an item must occur in a
``crates/iscc-lib/src/*.rs`` file, ignoring whitespace, a trailing comma before ``)`` and
the ``mut`` binding of a by-value ``self`` (rustdoc omits it). The match is textual and
crate-wide: it proves the printed item exists, not which ``impl`` block it belongs to.
"""

import re
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[1]
_DOC = _ROOT / "docs" / "rust-api.md"
_SRC_DIR = _ROOT / "crates" / "iscc-lib" / "src"
_BLOCK_RE = re.compile(r"```rust\n(.*?)```", re.DOTALL)
_ITEM_RE = re.compile(
    r"^ *pub (?:fn|const) .*?(?=^ *pub (?:fn|const) |^\}|\Z)", re.DOTALL | re.MULTILINE
)
_GEN_RE = re.compile(r"^pub fn (gen_\w+)", re.MULTILINE)


def squash(text: str) -> str:
    """Return ``text`` without whitespace and without a trailing comma before ``)``."""
    return re.sub(r"\s+", "", text).replace(",)", ")")


def doc_items(markdown: str) -> list[str]:
    """Return every ``pub fn`` / ``pub const`` item printed in the Rust code blocks."""
    return [
        " ".join(match.group(0).split())
        for block in _BLOCK_RE.findall(markdown)
        for match in _ITEM_RE.finditer(block)
    ]


def stale_items(markdown: str, source: str) -> list[str]:
    """Return the documented items that do not occur in the Rust ``source`` text."""
    haystack = squash(source.replace("(mut self", "(self"))
    stale = []
    for item in doc_items(markdown):
        needle = squash(item) if item.endswith(";") else squash(item) + "{"
        if needle not in haystack:
            stale.append(item)
    return stale


def _source() -> str:
    """Return the concatenated text of the committed crate source files."""
    return "\n".join(
        path.read_text(encoding="utf-8") for path in sorted(_SRC_DIR.glob("*.rs"))
    )


def test_documented_signatures_match_source():
    """Every item printed in the Rust API reference occurs in the crate source."""
    assert stale_items(_DOC.read_text(encoding="utf-8"), _source()) == []


def test_every_gen_function_signature_is_documented():
    """Each crate-root ``gen_*`` function has a printed signature, so the check is not vacuous."""
    lib_rs = (_SRC_DIR / "lib.rs").read_text(encoding="utf-8")
    documented = set(
        _GEN_RE.findall("\n".join(doc_items(_DOC.read_text(encoding="utf-8"))))
    )
    in_source = set(_GEN_RE.findall(lib_rs))
    assert in_source
    assert in_source - documented == set()


def test_wrong_return_type_is_reported():
    """A signature whose return type differs from the source is stale."""
    item = "pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<String>"
    assert stale_items(f"```rust\n{item}\n```\n", _source()) == [item]


def test_wrong_const_value_is_reported():
    """A constant printed with a value the source does not have is stale."""
    item = "pub const IO_READ_SIZE: usize = 65536;"
    assert stale_items(f"```rust\n{item}\n```\n", _source()) == [item]


def test_multi_line_item_matches_single_line_source():
    """Line breaks and a trailing comma in the printed form do not make an item stale."""
    printed = "pub fn gen_text_code_v0(\n    text: &str,\n    bits: u32,\n) -> IsccResult<TextCodeResult>"
    assert stale_items(f"```rust\n{printed}\n```\n", _source()) == []
