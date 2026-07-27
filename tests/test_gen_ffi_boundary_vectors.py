"""Tests for the C boundary-vector header generator in
``scripts/gen_ffi_boundary_vectors.py``.

The anchor test asserts that regenerating the tracked header from the canonical
fixture is a byte-exact no-op — this is what carries the drift gate into CI, since
CI runs pytest but never the generator. A mutation test renders a fixture copy with
one altered expected value and asserts the output differs, proving the gate fires
rather than merely passing. Fail-closed guards (wrong Unicode version, empty
section) are exercised on fixture copies in ``tmp_path``; the tracked fixture is
never mutated. No network, no mocks.
"""

import importlib.util
import json
from pathlib import Path

import pytest

# Load scripts/gen_ffi_boundary_vectors.py by path — a repo script, not a package.
_REPO_ROOT = Path(__file__).resolve().parents[1]
_SCRIPT_PATH = _REPO_ROOT / "scripts" / "gen_ffi_boundary_vectors.py"
_spec = importlib.util.spec_from_file_location("gen_ffi_boundary_vectors", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
gen = importlib.util.module_from_spec(_spec)
_loader.exec_module(gen)

FIXTURE = _REPO_ROOT / "crates" / "iscc-lib" / "tests" / "unicode_boundary.json"
HEADER = _REPO_ROOT / "crates" / "iscc-ffi" / "tests" / "unicode_boundary_vectors.h"


def _write_mutated_fixture(tmp_path, mutate):
    """Copy the canonical fixture, apply ``mutate`` to the dict, write to tmp_path."""
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    mutate(fixture)
    path = tmp_path / "unicode_boundary.json"
    path.write_text(json.dumps(fixture), encoding="utf-8")
    return path


def test_tracked_header_matches_fresh_render():
    # Regenerating from the canonical fixture must be a byte-exact no-op, so the
    # tracked header cannot drift from the fixture.
    assert gen.render(FIXTURE) == HEADER.read_text(encoding="utf-8")


def test_tracked_header_is_ascii_lf_only_no_hex_escapes():
    # Pure ASCII, LF-only, one trailing newline, and no greedy C hex escapes.
    raw = HEADER.read_bytes()
    text = raw.decode("utf-8")
    assert text.isascii()
    assert "\\x" not in text
    assert b"\r" not in raw
    assert text.endswith("\n") and not text.endswith("\n\n")


def test_escape_c_string_emits_three_digit_octal():
    # U+1FAE9 (UTF-8 f0 9f ab a9) must render as four 3-digit octal escapes;
    # printable ASCII passes through, quote and backslash are escaped.
    emoji = chr(0x1FAE9)
    assert gen.escape_c_string(f"a{emoji}b") == "a\\360\\237\\253\\251b"
    assert gen.escape_c_string('say "hi" \\ bye') == 'say \\"hi\\" \\\\ bye'


def test_mutated_expected_value_renders_different_header(tmp_path):
    # A changed expected value must change the render — the gate provably fires.
    def mutate(fixture):
        case = fixture["text_collapse"]["test_0004_seq_u0378_preserves_final_sigma"]
        case["outputs"]["result"] = "mutated"

    mutated = _write_mutated_fixture(tmp_path, mutate)
    assert gen.render(mutated) != gen.render(FIXTURE)


def test_wrong_unicode_version_fails_closed(tmp_path):
    # A fixture declaring any other Unicode version must refuse to render.
    def mutate(fixture):
        fixture["_metadata"]["unicode_data_version"] = "17.0.0"

    mutated = _write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="17.0.0"):
        gen.render(mutated)


def test_empty_section_fails_closed(tmp_path):
    # An empty vector section must refuse to render rather than emit a
    # zero-length array that would pass vacuously in C.
    def mutate(fixture):
        fixture["text_clean"] = {}

    mutated = _write_mutated_fixture(tmp_path, mutate)
    with pytest.raises(SystemExit, match="text_clean"):
        gen.render(mutated)
