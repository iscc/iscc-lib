"""Shared edge-case vectors (`data-edge.json`) checked against iscc_lib and iscc_core 1.5.0.

Every vector is schema-checked, run against iscc_lib (honouring `except.python`),
and run against the iscc_core reference: the reference must produce the stated
outcome exactly when the vector carries no `diverges` marker. A seed-presence
test pins the vectors the spec requires, since iterating the file cannot see a
missing one.
"""

import json
import re
from pathlib import Path

import iscc_core
import pytest

import iscc_lib

TESTS_DIR = Path(__file__).parent.parent / "crates" / "iscc-lib" / "tests"
DATA_EDGE_JSON = TESTS_DIR / "data-edge.json"
DATA_JSON = TESTS_DIR / "data.json"

SURFACE = "python"
SURFACE_IDS = {
    "rust",
    "python",
    "nodejs",
    "wasm",
    "c",
    "cpp",
    "java",
    "go",
    "ruby",
    "dotnet",
    "swift",
    "kotlin",
}
VECTOR_KEYS = {"inputs", "outputs", "error", "note", "diverges", "except"}
# Exception classes that count as a rejection. iscc_core raises ValueError (incl.
# binascii.Error), AssertionError, or IndexError (`iscc_decode("")`); any other class
# propagates and fails the test.
REJECTIONS = (ValueError, AssertionError, IndexError)
VECTOR_NAME = re.compile(r"^test_\d{4}_[a-z0-9_]+$")

UNIT_256_SPARE_BITS = "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBJ"

# (section, inputs) required by the spec's "Seed Set", plus the rows migrated
# from the former STRICT_BASE32_DIVERGENCES list in tests/test_seq.py.
SEED_SET = [
    ("iscc_decode", ["ISCC:GAAAAAAAAA======"]),
    ("iscc_decode", ["ISCC:GAAAAAAAAB"]),
    ("iscc_decode", ["ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ"]),
    ("iscc_decode", ["ISCC:GAAAAAAAAA"]),
    ("encode_seq", [["gaawdlqng6\N{LATIN SMALL LETTER LONG S}sdpf2"]]),
    ("encode_seq", [["gaawdlqng6\N{LATIN SMALL LETTER SHARP S}dpf2"]]),
    ("encode_seq", [["gaawdlqng6ssdpf2"]]),
    ("encode_seq", [[]]),
    ("encode_seq", [["ISCC:GAAAAAAAAA\x00junk"]]),
    ("encode_seq", [["ISCC:GAAAAAAAAB"]]),
    ("encode_seq", [[UNIT_256_SPARE_BITS]]),
    ("encode_seq", [["ISCC:GAAAAAAAAA======"]]),
    ("gen_text_code_v0", ["Hello\x00World", 64]),
    ("gen_mixed_code_v0", [[], 64]),
    ("gen_iscc_code_v0", [[]]),
    ("alg_simhash", [[]]),
]


def load_doc():
    """Load and parse data-edge.json."""
    return json.loads(DATA_EDGE_JSON.read_text(encoding="utf-8"))


def load_gen_output_keys():
    """Return each `data.json` gen function's output keys (the union over its vectors)."""
    data = json.loads(DATA_JSON.read_text(encoding="utf-8"))
    return {
        section: frozenset(
            key for vector in vectors.values() for key in vector["outputs"]
        )
        for section, vectors in data.items()
        if section.startswith("gen_")
    }


# Output subset rule (edge-vectors.md): a gen section's `outputs` may hold only the
# keys its function's `data.json` vectors carry, and must hold `iscc`.
GEN_OUTPUT_KEYS = load_gen_output_keys()


def convert_iscc_decode(result):
    """Encode an `iscc_decode` result as `[maintype, subtype, version, length, "bytes:<hex>"]`."""
    *header, digest = result
    return [int(field) for field in header] + [f"bytes:{digest.hex()}"]


def convert_encode_seq(result):
    """Encode `encode_seq` or `alg_simhash` bytes as a `bytes:<hex>` fixture value."""
    return f"bytes:{result.hex()}"


def decode_bytes(value):
    """Decode a `bytes:<hex>` fixture value into bytes."""
    prefix, sep, digits = value.partition(":")
    if prefix != "bytes" or not sep:
        pytest.fail(f"expected a bytes:<hex> input, got {value!r}")
    return bytes.fromhex(digits)


def prepare_alg_simhash(inputs):
    """Decode the `alg_simhash` digest list from `bytes:<hex>` strings into bytes."""
    digests, *rest = inputs
    return [[decode_bytes(d) for d in digests], *rest]


def prepare_alg_cdc_chunks(inputs):
    """Decode the `alg_cdc_chunks` data from a `bytes:<hex>` string into bytes."""
    data, *rest = inputs
    return [decode_bytes(data), *rest]


def convert_chunks(result):
    """Encode `alg_cdc_chunks` chunks as a list of `bytes:<hex>` fixture values.

    iscc_core returns a lazy generator, so the chunks are built here, outside the raw
    call; no vector expects an error, so none can surface here unnoticed.
    """
    return [f"bytes:{chunk.hex()}" for chunk in result]


# Section name -> preparer of the raw JSON inputs into call arguments. Sections not
# listed are called with the raw JSON inputs; `bytes:` strings are decoded only here.
PREPARERS = {
    "alg_simhash": prepare_alg_simhash,
    "alg_cdc_chunks": prepare_alg_cdc_chunks,
}


def prepare_inputs(section, inputs):
    """Return a vector's call arguments, identical for iscc_lib and iscc_core."""
    prepare = PREPARERS.get(section)
    return inputs if prepare is None else prepare(inputs)


# Section name -> converter of the raw result into its fixture form. The raw call is
# always `getattr(module, section)(*prepare_inputs(section, inputs))`, for iscc_lib and
# iscc_core alike, and is kept apart from the conversion so a conversion error never
# counts as a rejection.
CONVERTERS = {
    "iscc_decode": convert_iscc_decode,
    "encode_seq": convert_encode_seq,
    "gen_text_code_v0": dict,
    "gen_mixed_code_v0": dict,
    "gen_iscc_code_v0": dict,
    "alg_simhash": convert_encode_seq,
    "gen_meta_code_v0": dict,
    "gen_video_code_v0": dict,
    "alg_cdc_chunks": convert_chunks,
}


def validate_vector(section, name, vector):
    """Return the schema problems of one vector; an empty list means it is well-formed."""
    if not isinstance(vector, dict):
        return ["vector is not an object"]
    problems = []
    if not VECTOR_NAME.match(name):
        problems.append(f"name {name!r} is not test_NNNN_<slug>")
    unknown = sorted(str(key) for key in set(vector) - VECTOR_KEYS)
    if unknown:
        problems.append(f"unknown keys {unknown}")
    if not isinstance(vector.get("inputs"), list):
        problems.append("inputs is missing or not a list")
    if not isinstance(vector.get("note"), str) or not vector["note"]:
        problems.append("note is missing or empty")
    if ("outputs" in vector) == ("error" in vector):
        problems.append("needs exactly one of outputs / error")
    if vector.get("outputs") == {}:
        problems.append("outputs is an empty object, which every harness passes")
    if "error" in vector and (
        not isinstance(vector["error"], str) or not vector["error"]
    ):
        problems.append("error must be a non-empty string")
    diverges = vector.get("diverges")
    if "diverges" in vector and not (
        isinstance(diverges, str) and diverges.startswith("decisions.md ")
    ):
        problems.append("diverges must name a decisions.md entry")
    problems.extend(validate_except(vector.get("except", {})))
    if section.startswith("gen_"):
        problems.extend(validate_gen_outputs(section, vector))
    return problems


def validate_gen_outputs(section, vector):
    """Return the output-subset problems of one `gen_*` vector.

    The section needs a `data.json` counterpart (checked even on error vectors), and
    `outputs` must be an object holding `iscc` and only keys that counterpart carries.
    """
    allowed = GEN_OUTPUT_KEYS.get(section)
    if allowed is None:
        return [f"gen section {section!r} has no data.json counterpart"]
    if "outputs" not in vector:
        return []
    outputs = vector["outputs"]
    if not isinstance(outputs, dict):
        return ["gen outputs is not an object"]
    problems = [] if "iscc" in outputs else ["gen outputs lacks iscc"]
    unknown = sorted(str(key) for key in set(outputs) - allowed)
    if unknown:
        problems.append(f"gen output keys {unknown} not carried by data.json")
    return problems


def validate_except(exceptions):
    """Return the schema problems of an `except` map."""
    if not isinstance(exceptions, dict):
        return ["except is not an object"]
    problems = [f"unknown surface {k!r}" for k in exceptions if k not in SURFACE_IDS]
    problems += [
        f"except {k!r} must be error or skip"
        for k, v in exceptions.items()
        if v not in ("error", "skip")
    ]
    return problems


def outcome(module, section, inputs):
    """Run a vector's inputs and return `("error", None)` or `("output", value)`."""
    convert = CONVERTERS.get(section)
    if convert is None:
        pytest.fail(f"unknown section {section}")
    args = prepare_inputs(section, inputs)
    try:
        result = getattr(module, section)(*args)
    except REJECTIONS:  # Error text is never matched.
        return "error", None
    return "output", convert(result)


def same(expected, actual):
    """Return True if two fixture values are equal; a bool never equals a non-bool.

    Plain `==` lets `0 == False` and `1 == True`, so lists and dicts are compared
    element by element. Subclasses still match (an `int` subclass equals an `int`).
    """
    if isinstance(expected, bool) != isinstance(actual, bool):
        return False
    if isinstance(expected, list):
        return (
            isinstance(actual, list)
            and len(actual) == len(expected)
            and all(same(e, a) for e, a in zip(expected, actual))
        )
    if isinstance(expected, dict):
        return (
            isinstance(actual, dict)
            and actual.keys() == expected.keys()
            and all(same(v, actual[k]) for k, v in expected.items())
        )
    return expected == actual


def matches(vector, result):
    """Return True if an outcome equals the vector's stated outcome."""
    kind, value = result
    if "error" in vector:
        return kind == "error"
    if kind == "error":
        return False
    expected = vector["outputs"]
    if isinstance(expected, dict):
        return all(k in value and same(v, value[k]) for k, v in expected.items())
    return same(expected, value)


def all_vectors():
    """Return every vector of data-edge.json as a pytest param with a section/name id."""
    return [
        pytest.param(section, name, vector, id=f"{section}/{name}")
        for section, vectors in load_doc().items()
        for name, vector in vectors.items()
    ]


VECTORS = all_vectors()


def test_file_is_not_vacuous():
    """Every handled section holds vectors and the file holds no other section."""
    doc = load_doc()
    assert set(doc) == set(CONVERTERS)
    assert all(doc[section] for section in CONVERTERS)


@pytest.mark.parametrize(("section", "name", "vector"), VECTORS)
def test_vector_schema(section, name, vector):
    """Every vector is well-formed."""
    assert validate_vector(section, name, vector) == []


@pytest.mark.parametrize(("section", "name", "vector"), VECTORS)
def test_iscc_lib_outcome(section, name, vector):
    """iscc_lib produces each vector's stated outcome, honouring `except.python`."""
    exception = vector.get("except", {}).get(SURFACE)
    if exception == "skip":
        return
    if exception == "error" or "error" in vector:
        args = prepare_inputs(section, vector["inputs"])
        with pytest.raises(ValueError):
            getattr(iscc_lib, section)(*args)
        return
    result = outcome(iscc_lib, section, vector["inputs"])
    assert matches(vector, result), f"expected {vector['outputs']!r}, got {result!r}"


@pytest.mark.parametrize(("section", "name", "vector"), VECTORS)
def test_reference_outcome(section, name, vector):
    """iscc_core 1.5.0 produces the stated outcome iff the vector has no `diverges`."""
    assert iscc_core.__version__ == "1.5.0"
    agrees = matches(vector, outcome(iscc_core, section, vector["inputs"]))
    if "diverges" in vector:
        assert not agrees, f"iscc_core now agrees; drop diverges: {vector['diverges']}"
    else:
        assert agrees, (
            "iscc_core disagrees; fix the vector or add a sanctioned diverges"
        )


@pytest.mark.parametrize(
    ("section", "inputs"), [pytest.param(s, i, id=f"{s}/{i!r}") for s, i in SEED_SET]
)
def test_seed_vector_present(section, inputs):
    """Each spec seed vector is present in data-edge.json."""
    vectors = load_doc().get(section, {}).values()
    assert any(v["inputs"] == inputs for v in vectors), f"missing seed {inputs!r}"


GOOD = {"inputs": ["ISCC:GAAAAAAAAA"], "outputs": "x", "note": "n"}


def test_validate_vector_accepts_good_vector():
    """A minimal well-formed vector has no problems."""
    assert validate_vector("iscc_decode", "test_0001_good", GOOD) == []


@pytest.mark.parametrize(
    ("name", "vector", "problem"),
    [
        pytest.param("test_0001_x", ["x"], "not an object", id="not_object"),
        pytest.param("bad_name", GOOD, "test_NNNN", id="bad_name"),
        pytest.param(
            "test_0001_x",
            {k: v for k, v in GOOD.items() if k != "inputs"},
            "inputs",
            id="no_inputs",
        ),
        pytest.param("test_0001_x", {**GOOD, "inputs": "x"}, "inputs", id="str_inputs"),
        pytest.param(
            "test_0001_x",
            {k: v for k, v in GOOD.items() if k != "note"},
            "note",
            id="no_note",
        ),
        pytest.param(
            "test_0001_x", {**GOOD, "error": "e"}, "exactly one", id="both_outcomes"
        ),
        pytest.param(
            "test_0001_x",
            {k: v for k, v in GOOD.items() if k != "outputs"},
            "exactly one",
            id="no_outcome",
        ),
        pytest.param(
            "test_0001_x", {**GOOD, "outputs": {}}, "empty object", id="empty_outputs"
        ),
        pytest.param("test_0001_x", {**GOOD, "excpet": {}}, "unknown keys", id="typo"),
        pytest.param(
            "test_0001_x",
            {**GOOD, "diverges": "upstream bug"},
            "diverges",
            id="diverges",
        ),
        pytest.param(
            "test_0001_x", {**GOOD, "except": {"perl": "skip"}}, "surface", id="surface"
        ),
        pytest.param(
            "test_0001_x",
            {**GOOD, "except": {"go": "ignore"}},
            "error or skip",
            id="except_value",
        ),
    ],
)
def test_validate_vector_rejects_malformed(name, vector, problem):
    """Each malformed vector shape is reported."""
    problems = validate_vector("iscc_decode", name, vector)
    assert any(problem in p for p in problems), problems


def nul_text_vector():
    """Return a copy of the real `gen_text_code_v0/test_0001_embedded_nul` vector."""
    return json.loads(
        json.dumps(load_doc()["gen_text_code_v0"]["test_0001_embedded_nul"])
    )


def test_gen_output_keys_hold_iscc_for_every_gen_section():
    """Every data.json gen function carries `iscc`, so the derived table is not vacuous."""
    assert GEN_OUTPUT_KEYS
    assert all("iscc" in keys for keys in GEN_OUTPUT_KEYS.values())


def test_validate_gen_outputs_accepts_characters_for_text_code():
    """`characters` is an allowed gen_text_code_v0 output key."""
    vector = nul_text_vector()
    assert set(vector["outputs"]) == {"iscc", "characters"}
    assert validate_vector("gen_text_code_v0", "test_0001_embedded_nul", vector) == []


def test_validate_gen_outputs_accepts_error_vector():
    """A gen error vector carries no outputs, so the subset rule has nothing to check."""
    vector = {"inputs": [[], 64], "error": "e", "note": "n"}
    assert validate_vector("gen_mixed_code_v0", "test_0001_x", vector) == []


def test_validate_gen_outputs_rejects_unknown_key():
    """An output key the function's data.json vectors do not carry is the only problem."""
    vector = nul_text_vector()
    vector["outputs"]["foo"] = 1
    problems = validate_vector("gen_text_code_v0", "test_0001_embedded_nul", vector)
    assert len(problems) == 1 and "foo" in problems[0], problems


def test_validate_gen_outputs_rejects_key_of_other_function():
    """A key allowed for one gen function is unknown for another (`parts` on text)."""
    vector = nul_text_vector()
    vector["outputs"]["parts"] = []
    problems = validate_vector("gen_text_code_v0", "test_0001_embedded_nul", vector)
    assert len(problems) == 1 and "parts" in problems[0], problems


def test_validate_gen_outputs_rejects_missing_iscc():
    """Gen outputs without `iscc` are reported, and nothing else is."""
    vector = nul_text_vector()
    del vector["outputs"]["iscc"]
    problems = validate_vector("gen_text_code_v0", "test_0001_embedded_nul", vector)
    assert problems == ["gen outputs lacks iscc"], problems


def test_validate_gen_outputs_rejects_non_object():
    """Gen outputs that are not an object give exactly one problem."""
    vector = nul_text_vector()
    vector["outputs"] = vector["outputs"]["iscc"]
    problems = validate_vector("gen_text_code_v0", "test_0001_embedded_nul", vector)
    assert problems == ["gen outputs is not an object"], problems


def test_validate_gen_outputs_rejects_section_without_counterpart():
    """A gen section absent from data.json fails closed, even on an error vector."""
    vector = {"inputs": [[], 64], "error": "e", "note": "n"}
    problems = validate_vector("gen_foo_v0", "test_0001_x", vector)
    assert problems == ["gen section 'gen_foo_v0' has no data.json counterpart"]


@pytest.mark.parametrize(
    ("expected", "actual"),
    [
        pytest.param([3, 0, 0, 0, "x"], [3, False, 0, 0, "x"], id="false_for_zero"),
        pytest.param(True, 1, id="one_for_true"),
        pytest.param({"characters": False}, {"characters": 0}, id="zero_for_false"),
    ],
)
def test_matches_rejects_bool_number_lookalike(expected, actual):
    """A Boolean never matches a number, in either direction, at any depth."""
    assert not matches({"outputs": expected}, ("output", actual))


def test_matches_accepts_equal_nested_list():
    """A nested list with equal values of the same kinds matches."""
    expected = {"parts": ["a", "b"], "flag": True, "n": [1, [2, 3]]}
    actual = {"parts": ["a", "b"], "flag": True, "n": [1, [2, 3]], "extra": 0}
    assert matches({"outputs": expected}, ("output", actual))
