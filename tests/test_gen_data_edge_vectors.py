"""Tests for the C/C++ edge-vector header generator in ``scripts/gen_data_edge_vectors.py``.

The anchor test asserts that regenerating the tracked header from the canonical
``data-edge.json`` is a byte-exact no-op; this carries the staleness gate into CI,
which runs pytest but never the generator. A mutation test proves the gate fires.
Fail-closed guards (unknown section, bad input arity, bad ``except`` key, a gen
output key outside the ``data.json`` set) run on fixture copies in ``tmp_path``; the
tracked fixture is never mutated. Every string literal of the header fits the ISO C99
minimum (``C_LITERAL_MAX``); longer strings are buffers filled by ``iscc_edge_init()``,
which both harnesses call. Counts are derived from the fixture, never written as
literals. No network, no mocks.
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
HARNESSES = {
    _REPO_ROOT / "crates" / "iscc-ffi" / "tests" / "test_iscc.c": 'RUN_EDGE_SECTION("',
    _REPO_ROOT / "packages" / "cpp" / "tests" / "test_iscc.cpp": 'run_edge_section("',
}
# A C block comment or a string literal (group 1 holds the literal body).
C_TOKEN = re.compile(r'/\*.*?\*/|"((?:[^"\\\n]|\\.)*)"', flags=re.DOTALL)
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


def unescape_c(body):
    """Decode the bytes of a C string literal body (octal and single-character escapes)."""
    raw = body.encode("ascii")
    out = bytearray()
    for match in re.finditer(rb"\\([0-7]{1,3}|.)|([^\\])", raw, flags=re.DOTALL):
        escape, plain = match.groups()
        if plain is not None:
            out += plain
        elif escape[:1].isdigit():
            out.append(int(escape, 8))
        else:
            out += escape
    return bytes(out)


def c_string_literals(text):
    """Return the decoded bytes of every string literal, adjacent pieces joined."""
    literals = []
    previous_end = None
    for match in C_TOKEN.finditer(text):
        if match.group(1) is None:
            previous_end = None
            continue
        piece = unescape_c(match.group(1))
        gap = text[previous_end : match.start()] if previous_end is not None else None
        if gap is not None and not gap.strip():
            literals[-1] += piece
        else:
            literals.append(piece)
        previous_end = match.end()
    return literals


def memcpy_pieces(text, name):
    """Return the (offset, bytes, count) of every `memcpy` into buffer `name`."""
    pattern = rf'memcpy\({name} \+ (\d+), "((?:[^"\\]|\\.)*)", (\d+)\);'
    return [
        (int(offset), unescape_c(body), int(count))
        for offset, body, count in re.findall(pattern, text)
    ]


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


def test_escape_c_bytes_emits_three_digit_octal():
    """Non-ASCII and NUL bytes become 3-digit octal; quote, backslash, ? are escaped."""
    emoji = chr(0x1FAE9)
    assert gen.escape_c_bytes(f"a{emoji}b".encode()) == "a\\360\\237\\253\\251b"
    assert gen.escape_c_bytes(b'say "hi" \\ bye') == 'say \\"hi\\" \\\\ bye'
    assert gen.escape_c_bytes(b"a\x001") == "a\\0001"
    assert gen.escape_c_bytes(b"??=") == "\\?\\?="


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
    lines, init = gen.list_arrays("ident", [], [])
    assert lines == [
        'static const char *const ident_items[] = {""};',
        "static const size_t ident_lens[] = {0};",
    ]
    assert init == "{ident_items, ident_lens, 0}"


def test_byte_list_renders_binary_items_with_explicit_lengths():
    """A digest list renders each item as octal-escaped bytes with its own length."""
    lines, init = gen.byte_list_arrays("ident", [b"\xf0\x0f", b"<<"], [])
    assert lines == [
        'static const char *const ident_items[] = {"\\360\\017", "<<"};',
        "static const size_t ident_lens[] = {2, 2};",
    ]
    assert init == "{ident_items, ident_lens, 2}"
    assert gen.byte_list_arrays("ident", [], []) == gen.list_arrays("ident", [], [])


def test_alg_simhash_rows_render_digests_and_empty_sentinel():
    """The alg_simhash rows carry decoded digests and the non-NULL empty-list sentinel."""
    fixture = load_fixture()
    text = HEADER.read_text(encoding="utf-8")
    for name, vector in fixture["alg_simhash"].items():
        digests = [bytes.fromhex(d.removeprefix("bytes:")) for d in vector["inputs"][0]]
        index = list(fixture["alg_simhash"]).index(name)
        lines, init = gen.byte_list_arrays(
            f"iscc_edge_alg_simhash_{index}", digests, []
        )
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


def test_frame_arrays_render_lengths_and_sentinels():
    """Each frame renders as an int32 array with its own length; empties get sentinels."""
    lines, init = gen.frame_arrays("ident", [[1, -2], []])
    assert lines == [
        "static const int32_t ident_f0[] = {",
        "    1, -2,",
        "};",
        "static const int32_t ident_f1[] = {",
        "    0,",
        "};",
        "static const int32_t *const ident_frames[] = {ident_f0, ident_f1};",
        "static const uintptr_t ident_lens[] = {2, 0};",
    ]
    assert init == "{ident_frames, ident_lens, 2}"
    empty_lines, empty_init = gen.frame_arrays("ident", [])
    assert "static const uintptr_t ident_lens[] = {0};" in empty_lines
    assert empty_init == "{ident_frames, ident_lens, 0}"


def test_int32_array_wraps_values():
    """A long frame wraps at INT32_PER_LINE values per line and keeps every value."""
    values = list(range(gen.INT32_PER_LINE + 1))
    lines = gen.int32_array("ident", values)
    assert len(lines) == 4
    body = " ".join(lines[1:-1]).replace(",", " ").split()
    assert [int(v) for v in body] == values


def test_video_rows_render_frames_with_their_lengths():
    """The gen_video_code_v0 rows carry every frame with its length (ragged included)."""
    fixture = load_fixture()
    text = HEADER.read_text(encoding="utf-8")
    section = fixture["gen_video_code_v0"]
    for index, (name, vector) in enumerate(section.items()):
        ident = f"iscc_edge_gen_video_code_v0_{index}"
        lines, init = gen.frame_arrays(ident, vector["inputs"][0])
        assert all(line in text for line in lines), name
        assert f'{{"{name}", ' in text and init in text, name
    lens = {tuple(len(f) for f in v["inputs"][0]) for v in section.values()}
    assert any(len(set(row)) > 1 for row in lens), "no ragged video row"


def test_meta_row_renders_meta_text():
    """A meta input renders as C text: an object as its JSON text, a string as is."""
    fixture = load_fixture()
    text = HEADER.read_text(encoding="utf-8")
    object_rows = 0
    for index, (name, vector) in enumerate(fixture["gen_meta_code_v0"].items()):
        meta = vector["inputs"][2]
        if isinstance(meta, dict):
            object_rows += 1
            raw = json.dumps(meta, ensure_ascii=False)
        else:
            assert isinstance(meta, str), name
            raw = meta
        ident = f"iscc_edge_gen_meta_code_v0_{index}_meta"
        assert gen.c_str(raw.encode("utf-8"), ident, []) in text, name
    assert object_rows >= 1


def test_non_int32_frame_value_fails_closed(tmp_path):
    """A frame value outside the int32 range refuses to render."""

    def mutate(fixture):
        vector = next(iter(fixture["gen_video_code_v0"].values()))
        vector["inputs"][0][0][0] = 2**31

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="int32"):
        gen.render(mutated)


def test_non_object_meta_fails_closed(tmp_path):
    """A meta input that is neither an object nor a string refuses to render."""

    def mutate(fixture):
        vector = next(iter(fixture["gen_meta_code_v0"].values()))
        vector["inputs"][2] = 7

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="expected a string"):
        gen.render(mutated)


def test_cdc_rows_render_data_flag_size_and_chunks():
    """The alg_cdc_chunks rows carry the data, utf32 flag, average size and chunk list."""
    fixture = load_fixture()
    text = HEADER.read_text(encoding="utf-8")
    for index, (name, vector) in enumerate(fixture["alg_cdc_chunks"].items()):
        data, utf32, avg = vector["inputs"]
        chunks = [bytes.fromhex(c.removeprefix("bytes:")) for c in vector["outputs"]]
        ident = f"iscc_edge_alg_cdc_chunks_{index}"
        lines, init = gen.byte_list_arrays(ident, chunks, [])
        raw = bytes.fromhex(data.removeprefix("bytes:"))
        head = f"{gen.c_str(raw, f'{ident}_data', [])}, {int(utf32)}, {avg}u"
        assert all(line in text for line in lines), name
        assert f"{head}, {init}}}," in text, name
    assert '{"", 0}, 0, 1024u, ' in text, (
        "empty data must render as a non-NULL empty string"
    )


def test_cdc_error_row_renders_zeroed_chunk_list(tmp_path):
    """An alg_cdc_chunks error row renders a zeroed chunk list and no chunk arrays."""

    def mutate(fixture):
        vector = fixture["alg_cdc_chunks"]["test_0003_empty_input"]
        del vector["outputs"]
        vector["error"] = "e"

    text = gen.render(write_mutated_fixture(tmp_path, mutate))
    assert '{"", 0}, 0, 1024u, {0, 0, 0}},' in text
    assert "iscc_edge_alg_cdc_chunks_2_items" not in text


@pytest.mark.parametrize(
    ("index", "value", "message"),
    [
        (0, "0001", "bytes:<hex>"),
        (1, 0, "boolean"),
        (2, 2**32, "uint32"),
        (2, True, "non-negative integer"),
    ],
)
def test_cdc_bad_input_fails_closed(tmp_path, index, value, message):
    """An alg_cdc_chunks input of the wrong type or range refuses to render."""

    def mutate(fixture):
        vector = next(iter(fixture["alg_cdc_chunks"].values()))
        vector["inputs"][index] = value

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match=message):
        gen.render(mutated)


def test_encode_component_rows_render_header_digest_and_unit():
    """The encode_component rows carry the header fields, bit length, digest and unit."""
    text = HEADER.read_text(encoding="utf-8")
    assert '0, 0, 0, 64u, {"\\001\\002\\003\\004", 4}, {0, 0}},' in text, (
        "the short-digest error row must carry its digest and a zeroed unit"
    )
    assert (
        '0, 0, 0, 64u, {"\\000\\001\\002\\003\\004\\005\\006\\007", 8}, '
        '{"AAAQAAICAMCAKBQH", 16}},'
    ) in text


@pytest.mark.parametrize(
    ("index", "value", "message"),
    [
        (0, 256, "uint8"),
        (2, True, "non-negative integer"),
        (3, 2**32, "uint32"),
        (4, "0001", "bytes:<hex>"),
    ],
)
def test_encode_component_bad_input_fails_closed(tmp_path, index, value, message):
    """An encode_component input of the wrong type or range refuses to render."""

    def mutate(fixture):
        vector = next(iter(fixture["encode_component"].values()))
        vector["inputs"][index] = value

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match=message):
        gen.render(mutated)


def test_encode_component_non_string_unit_fails_closed(tmp_path):
    """An encode_component output that is not a string refuses to render."""

    def mutate(fixture):
        fixture["encode_component"]["test_0002_exact_length_digest"]["outputs"] = 7

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="expected a string"):
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


def test_unknown_gen_output_key_fails_closed(tmp_path):
    """A gen_text_code_v0 output key that data.json does not carry refuses to render."""

    def mutate(fixture):
        _, vector = find_vector(fixture, "gen_text_code_v0", [NUL_TEXT, 64])
        vector["outputs"]["foo"] = 1

    mutated = write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="foo"):
        gen.render(mutated)


def test_gen_iscc_and_characters_only_still_renders(tmp_path):
    """A gen_text_code_v0 vector holding exactly iscc and characters renders."""

    def mutate(fixture):
        _, vector = find_vector(fixture, "gen_text_code_v0", [NUL_TEXT, 64])
        vector["outputs"] = {"iscc": vector["outputs"]["iscc"], "characters": 10}

    mutated = write_mutated_fixture(tmp_path, mutate)
    assert gen.render(mutated) == gen.render(FIXTURE)


def test_gen_output_keys_derive_from_data_json():
    """The allowed gen output keys are read from data.json, each holding iscc."""
    data = json.loads(gen.DATA_JSON_PATH.read_text(encoding="utf-8"))
    sections = {s for s in data if s.startswith("gen_")}
    assert set(gen.gen_output_keys()) == sections
    assert all("iscc" in keys for keys in gen.gen_output_keys().values())
    assert "characters" in gen.allowed_output_keys("gen_text_code_v0")


def test_gen_section_without_data_json_counterpart_fails_closed():
    """A gen section absent from data.json refuses to render, even on an error vector."""
    with pytest.raises(SystemExit, match="gen_foo_v0"):
        gen.gen_iscc(
            {"inputs": [], "error": "e"},
            "gen_foo_v0/test_0001_x",
            "gen_foo_v0",
            "ident",
            [],
        )


def test_c_string_literals_joins_pieces_and_decodes_escapes():
    """The literal scanner joins adjacent pieces, decodes escapes and skips comments."""
    text = '/* "no" */ x = "a\\101" "\\"b\\?";\ny = "\\0001";'
    assert c_string_literals(text) == [b'aA"b?', b"\x001"]
    oversized = '"' + "x" * (gen.C_LITERAL_MAX + 1) + '"'
    assert [len(s) for s in c_string_literals(oversized)] == [gen.C_LITERAL_MAX + 1]


def test_tracked_header_string_literals_fit_c99_minimum():
    """Every string literal of the header, adjacent pieces joined, fits C_LITERAL_MAX bytes."""
    literals = c_string_literals(HEADER.read_text(encoding="utf-8"))
    assert literals, "no string literal checked"
    too_long = [len(s) for s in literals if len(s) > gen.C_LITERAL_MAX]
    assert too_long == []


def test_tracked_header_always_defines_edge_init():
    """The header defines iscc_edge_init, with an empty body when no buffer exists."""
    text = HEADER.read_text(encoding="utf-8")
    assert "#include <string.h>\n" in text
    assert "static void iscc_edge_init(void) {\n" in text
    assert gen.init_function([]).endswith("static void iscc_edge_init(void) {\n}\n")


def test_harnesses_call_edge_init_before_the_first_edge_section():
    """Both harnesses call iscc_edge_init() before they run the first edge section."""
    for path, first_run in HARNESSES.items():
        source = path.read_text(encoding="utf-8")
        call = source.find("iscc_edge_init();")
        assert call != -1, path.name
        assert call < source.index(first_run), path.name


def test_oversized_meta_renders_buffer_filled_by_init_pieces(tmp_path):
    """An oversized non-ASCII meta is a len + 1 buffer whose init pieces rebuild its UTF-8.

    The one-byte ASCII prefix puts a piece boundary inside a two-byte UTF-8 sequence.
    """
    meta = {"a": "x" + "\u00fc" * 3000}

    def mutate(fixture):
        vector = next(iter(fixture["gen_meta_code_v0"].values()))
        vector["inputs"][2] = meta

    text = gen.render(write_mutated_fixture(tmp_path, mutate))
    raw = json.dumps(meta, ensure_ascii=False).encode("utf-8")
    assert len(raw) > gen.C_LITERAL_MAX
    name = "iscc_edge_gen_meta_code_v0_0_meta_buf"
    assert f"static char {name}[{len(raw) + 1}];\n" in text
    assert f"{{{name}, {len(raw)}}}" in text
    pieces = memcpy_pieces(text, name)
    assert len(pieces) == -(-len(raw) // gen.C_LITERAL_MAX)
    offsets = [offset for offset, _, _ in pieces]
    assert offsets == list(range(0, len(raw), gen.C_LITERAL_MAX))
    assert all(len(piece) == count <= gen.C_LITERAL_MAX for _, piece, count in pieces)
    assert b"".join(piece for _, piece, _ in pieces) == raw
    assert all(len(s) <= gen.C_LITERAL_MAX for s in c_string_literals(text))


def test_oversized_nul_bytes_survive_init_pieces():
    """An oversized string of NUL bytes fills its buffer through explicit byte counts."""
    data = b"\x00" * (gen.C_LITERAL_MAX + 10)
    bufs = []
    assert gen.c_str(data, "id", bufs) == f"{{id_buf, {len(data)}}}"
    assert bufs == [("id_buf", data)]
    pieces = memcpy_pieces(gen.init_function(bufs), "id_buf")
    assert [count for _, _, count in pieces] == [gen.C_LITERAL_MAX, 10]
    assert b"".join(piece for _, piece, _ in pieces) == data


def test_short_string_stays_a_literal():
    """A string of exactly C_LITERAL_MAX bytes stays a literal and adds no buffer."""
    bufs = []
    data = b"x" * gen.C_LITERAL_MAX
    assert gen.c_str(data, "id", bufs) == f'{{"{data.decode()}", {len(data)}}}'
    assert bufs == []


def test_oversized_list_item_and_row_name_render_buffers(tmp_path):
    """An oversized list item and an oversized row name each become a buffer of their own."""
    bufs = []
    big = b"x" * (gen.C_LITERAL_MAX + 1)
    lines, init = gen.byte_list_arrays("id", [b"a", big], bufs)
    assert lines == [
        'static const char *const id_items[] = {"a", id_i1_buf};',
        f"static const size_t id_lens[] = {{1, {len(big)}}};",
    ]
    assert init == "{id_items, id_lens, 2}"
    assert bufs == [("id_i1_buf", big)]
    long_name = "n" * (gen.C_LITERAL_MAX + 1)

    def mutate(fixture):
        section = fixture["iscc_decode"]
        first = next(iter(section))
        fixture["iscc_decode"] = {
            (long_name if name == first else name): vector
            for name, vector in section.items()
        }

    text = gen.render(write_mutated_fixture(tmp_path, mutate))
    name = "iscc_edge_iscc_decode_0_name_buf"
    assert f"static char {name}[{len(long_name) + 1}];\n" in text
    assert f"    {{{name}, ISCC_EDGE_" in text
    pieces = memcpy_pieces(text, name)
    assert b"".join(piece for _, piece, _ in pieces) == long_name.encode()
