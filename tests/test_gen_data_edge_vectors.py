"""Tests for the C/C++ edge-vector header generator in ``scripts/gen_data_edge_vectors.py``.

The anchor test asserts that regenerating the tracked header from the canonical
``data-edge.json`` is a byte-exact no-op; this carries the staleness gate into CI,
which runs pytest but never the generator. A mutation test proves the gate fires.
Fail-closed guards (unknown section, bad input arity, bad ``except`` key) run on
fixture copies in ``tmp_path``; the tracked fixture is never mutated. Counts are
derived from the fixture, never written as literals. No network, no mocks.
"""

import importlib.util
import json
import re
from pathlib import Path

import pytest

# Load scripts/gen_data_edge_vectors.py by path: a repo script, not a package.
_REPO_ROOT = Path(__file__).resolve().parents[1]
_SCRIPT_PATH = _REPO_ROOT / "scripts" / "gen_data_edge_vectors.py"
_spec = importlib.util.spec_from_file_location("gen_data_edge_vectors", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
gen = importlib.util.module_from_spec(_spec)
_loader.exec_module(gen)

FIXTURE = _REPO_ROOT / "crates" / "iscc-lib" / "tests" / "data-edge.json"
HEADER = _REPO_ROOT / "crates" / "iscc-ffi" / "tests" / "data_edge_vectors.h"
NUL_TEXT = "Hello\x00World"


def load_fixture():
    """Load the canonical data-edge.json fixture."""
    return json.loads(FIXTURE.read_text(encoding="utf-8"))


def write_mutated_fixture(tmp_path, mutate):
    """Copy the canonical fixture, apply ``mutate`` to the dict, write to tmp_path."""
    fixture = load_fixture()
    mutate(fixture)
    path = tmp_path / "data-edge.json"
    path.write_text(json.dumps(fixture), encoding="utf-8")
    return path


def c_escape_chars(text):
    """Return the character after each backslash escape, scanning left to right."""
    return re.findall(r"\\(.)", text, flags=re.DOTALL)


def find_vector(fixture, section, inputs):
    """Return the name and vector of ``section`` whose inputs equal ``inputs``."""
    for name, vector in fixture[section].items():
        if vector["inputs"] == inputs:
            return name, vector
    raise AssertionError(f"no {section} vector with inputs {inputs!r}")


def test_tracked_header_matches_fresh_render():
    """Regenerating from the canonical fixture is a byte-exact no-op."""
    assert gen.render(FIXTURE) == HEADER.read_text(encoding="utf-8")


def test_tracked_header_is_ascii_lf_only_no_hex_escapes():
    """The header is pure ASCII, LF-only, one trailing newline, no greedy hex escapes."""
    raw = HEADER.read_bytes()
    text = raw.decode("utf-8")
    assert text.isascii()
    assert "x" not in c_escape_chars(text)
    assert c_escape_chars("a\\x61 \\\\x61") == ["x", "\\"]
    assert b"\r" not in raw
    assert text.endswith("\n") and not text.endswith("\n\n")


def test_header_counts_match_fixture():
    """Each section count macro and the total macro equal the fixture's vector counts."""
    fixture = load_fixture()
    text = HEADER.read_text(encoding="utf-8")
    assert set(fixture) == set(gen.RENDERERS)
    for section, vectors in fixture.items():
        assert f"#define {gen.count_macro(section)} {len(vectors)}\n" in text
    total = sum(len(vectors) for vectors in fixture.values())
    assert f"#define ISCC_EDGE_TOTAL_COUNT {total}\n" in text


def test_every_vector_name_is_rendered_once():
    """Every fixture vector appears as exactly one row of its section."""
    text = HEADER.read_text(encoding="utf-8")
    for section, vectors in load_fixture().items():
        body = text.split(f"iscc_edge_{section}[{gen.count_macro(section)}] = {{")[1]
        body = body.split("\n};\n")[0]
        for name in vectors:
            assert body.count(f'{{"{name}", ') == 1, f"{section}/{name}"


def test_escape_c_string_emits_three_digit_octal():
    """Non-ASCII and NUL bytes become 3-digit octal; quote, backslash, ? are escaped."""
    emoji = chr(0x1FAE9)
    assert gen.escape_c_string(f"a{emoji}b") == "a\\360\\237\\253\\251b"
    assert gen.escape_c_string('say "hi" \\ bye') == 'say \\"hi\\" \\\\ bye'
    assert gen.escape_c_string("a\x001") == "a\\0001"
    assert gen.escape_c_string("??=") == "\\?\\?="


def test_nul_text_row_resolves_c_skip_cpp_error():
    """The embedded-NUL gen_text_code_v0 row renders as c=skip, cpp=error."""
    name, vector = find_vector(load_fixture(), "gen_text_code_v0", [NUL_TEXT, 64])
    assert gen.expectations(vector, name) == "ISCC_EDGE_SKIP, ISCC_EDGE_ERROR"
    row = (
        f'{{"{name}", ISCC_EDGE_SKIP, ISCC_EDGE_ERROR,\n     {{"Hello\\000World", 11}}'
    )
    assert row in HEADER.read_text(encoding="utf-8")


def test_expectations_follow_outputs_error_and_except():
    """Without `except` the expectation is ok for outputs and error for error rows."""
    assert gen.expectations({"outputs": "x"}, "w") == "ISCC_EDGE_OK, ISCC_EDGE_OK"
    assert gen.expectations({"error": "e"}, "w") == "ISCC_EDGE_ERROR, ISCC_EDGE_ERROR"
    vector = {"outputs": "x", "except": {"cpp": "error", "go": "skip"}}
    assert gen.expectations(vector, "w") == "ISCC_EDGE_OK, ISCC_EDGE_ERROR"


def test_empty_list_renders_non_null_sentinel_with_count_zero():
    """An empty input list points at a one-element sentinel array with count 0."""
    lines, init = gen.list_arrays("ident", [])
    assert lines == [
        'static const char *const ident_items[] = {""};',
        "static const size_t ident_lens[] = {0};",
    ]
    assert init == "{ident_items, ident_lens, 0}"


def test_byte_list_renders_binary_items_with_explicit_lengths():
    """A digest list renders each item as octal-escaped bytes with its own length."""
    lines, init = gen.byte_list_arrays("ident", [b"\xf0\x0f", b"<<"])
    assert lines == [
        'static const char *const ident_items[] = {"\\360\\017", "<<"};',
        "static const size_t ident_lens[] = {2, 2};",
    ]
    assert init == "{ident_items, ident_lens, 2}"
    assert gen.byte_list_arrays("ident", []) == gen.list_arrays("ident", [])


def test_alg_simhash_rows_render_digests_and_empty_sentinel():
    """The alg_simhash rows carry decoded digests and the non-NULL empty-list sentinel."""
    fixture = load_fixture()
    text = HEADER.read_text(encoding="utf-8")
    for name, vector in fixture["alg_simhash"].items():
        digests = [bytes.fromhex(d.removeprefix("bytes:")) for d in vector["inputs"][0]]
        index = list(fixture["alg_simhash"]).index(name)
        lines, init = gen.byte_list_arrays(f"iscc_edge_alg_simhash_{index}", digests)
        assert all(line in text for line in lines), name
        assert f'{{"{name}", ' in text and init in text, name
    empty, _ = find_vector(fixture, "alg_simhash", [[]])
    assert f'{{"{empty}", ISCC_EDGE_ERROR, ISCC_EDGE_ERROR,' in text


def test_non_bytes_digest_fails_closed(tmp_path):
    """An alg_simhash digest that is not a bytes:<hex> value refuses to render."""

    def mutate(fixture):
        _, vector = find_vector(fixture, "alg_simhash", [[]])
        vector["inputs"] = [["f00f"]]

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="bytes:<hex>"):
        gen.render(mutated)


def test_mutated_expected_value_renders_different_header(tmp_path):
    """A changed expected value changes the render, so the staleness gate fires."""

    def mutate(fixture):
        name, _ = find_vector(fixture, "gen_text_code_v0", [NUL_TEXT, 64])
        fixture["gen_text_code_v0"][name]["outputs"]["iscc"] = "ISCC:EAAWSPUDF7GJWQVD"

    mutated = write_mutated_fixture(tmp_path, mutate)
    assert gen.render(mutated) != gen.render(FIXTURE)


def test_unknown_section_fails_closed(tmp_path):
    """A section the generator has no renderer for refuses to render."""

    def mutate(fixture):
        fixture["gen_data_code_v0"] = {
            "test_0001_x": {"inputs": ["x", 64], "error": "e", "note": "n"}
        }

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="gen_data_code_v0"):
        gen.render(mutated)


def test_bad_input_arity_fails_closed(tmp_path):
    """A vector with an unexpected input arity refuses to render."""

    def mutate(fixture):
        _, vector = find_vector(fixture, "gen_text_code_v0", [NUL_TEXT, 64])
        vector["inputs"] = [NUL_TEXT]

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="arity"):
        gen.render(mutated)


def test_bad_except_key_fails_closed(tmp_path):
    """An `except` key outside the surface ids refuses to render."""

    def mutate(fixture):
        _, vector = find_vector(fixture, "gen_text_code_v0", [NUL_TEXT, 64])
        vector["except"]["perl"] = "skip"

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="perl"):
        gen.render(mutated)


def test_empty_section_fails_closed(tmp_path):
    """An empty section refuses to render rather than emit a zero-length array."""

    def mutate(fixture):
        fixture["gen_iscc_code_v0"] = {}

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="gen_iscc_code_v0"):
        gen.render(mutated)
