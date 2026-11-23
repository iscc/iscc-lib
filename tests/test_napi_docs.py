"""Docs-claim test: the ``@iscc/lib`` pages state the result type of ``gen_sum_code_v0``.

The napi addon returns an ISCC string from nine code generators, but ``gen_sum_code_v0`` returns a
``NapiSumCodeResult`` object. ``crates/iscc-napi/README.md`` (the npm page) must name that object,
and neither it nor ``docs/howto/nodejs.md`` may claim that every generator returns a string. Every
field the README names on its result sentence must be a field of the struct in
``crates/iscc-napi/src/lib.rs``, so a renamed field breaks this test. No network, no mocks.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README = ROOT / "crates" / "iscc-napi" / "README.md"
HOWTO = ROOT / "docs" / "howto" / "nodejs.md"
LIB_RS = ROOT / "crates" / "iscc-napi" / "src" / "lib.rs"
# Split so `git grep` for the false claim finds only docs that make it, not this test.
BANNED_PHRASE = "return ISCC " + "strings directly"
RESULT_SENTENCE_RE = re.compile(
    r"`gen_sum_code_v0` returns a `NapiSumCodeResult` object[^.]*\."
)
STRUCT_RE = re.compile(r"pub struct NapiSumCodeResult \{(.*?)\n\}", re.DOTALL)
FIELD_RE = re.compile(r"^\s*pub (\w+):", re.MULTILINE)
SENTENCE_FIELDS = ["iscc", "datahash", "filesize", "units"]


def normalise_whitespace(text):
    """Collapse every whitespace run to one space (mdformat reflows prose at 100 columns)."""
    return re.sub(r"\s+", " ", text)


def readme_result_sentence():
    """Return the README sentence that describes the ``gen_sum_code_v0`` result object."""
    text = normalise_whitespace(README.read_text(encoding="utf-8"))
    match = RESULT_SENTENCE_RE.search(text)
    assert match, "crates/iscc-napi/README.md has no gen_sum_code_v0 result sentence"
    return match.group(0)


def struct_fields():
    """Return the field names of ``pub struct NapiSumCodeResult`` parsed out of ``lib.rs``."""
    match = STRUCT_RE.search(LIB_RS.read_text(encoding="utf-8"))
    assert match, (
        "pub struct NapiSumCodeResult not found in crates/iscc-napi/src/lib.rs"
    )
    return set(FIELD_RE.findall(match.group(1)))


def test_readme_names_napi_sum_code_result():
    """The npm README names the object type ``gen_sum_code_v0`` returns."""
    assert "NapiSumCodeResult" in README.read_text(encoding="utf-8")


def test_docs_drop_all_generators_return_strings_claim():
    """Neither Node.js page claims that every code generator returns a string."""
    for path in [README, HOWTO]:
        text = normalise_whitespace(path.read_text(encoding="utf-8"))
        assert BANNED_PHRASE not in text, (
            f"{path.relative_to(ROOT)} contains {BANNED_PHRASE!r}"
        )


def test_readme_result_sentence_names_struct_fields():
    """Every field on the README's result sentence is a field of ``NapiSumCodeResult``."""
    sentence = readme_result_sentence()
    fields = struct_fields()
    for name in SENTENCE_FIELDS:
        assert f"`{name}`" in sentence, f"result sentence does not name {name!r}"
        assert name in fields, f"NapiSumCodeResult has no field {name!r}"


def test_struct_fields_match_readme_fields():
    """The README names every field of ``NapiSumCodeResult``, so an added field is documented."""
    assert struct_fields() == set(SENTENCE_FIELDS)
