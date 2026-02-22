# Next Work Package

## Step: Add Python conformance tests for all 9 gen\_\*\_v0 functions

## Goal

Validate the full Python → PyO3 → Rust → result pipeline by running all 46 conformance vectors from
`data.json` through the Python `iscc` package via pytest. This is the main gap preventing the Python
bindings target criteria from being met ("pytest passes the same conformance vectors from Python").

## Scope

- **Create**: `tests/test_conformance.py`
- **Modify**: (none — the existing `test_smoke.py` stays as-is)
- **Reference**:
    - `crates/iscc-lib/tests/data.json` — the 46 conformance vectors (same file used by Rust tests)
    - `crates/iscc-py/src/lib.rs` — Python function signatures and type expectations
    - `crates/iscc-lib/src/lib.rs` lines ~790-1260 — Rust conformance test patterns (how each
        function's inputs are parsed from `data.json`)

## Implementation Notes

Create a single `tests/test_conformance.py` file that:

1. **Loads `data.json`** from `crates/iscc-lib/tests/data.json` using a path relative to the test
    file's location (e.g., `Path(__file__).parent.parent / "crates/iscc-lib/tests/data.json"`).

2. **Parametrizes tests** using `@pytest.mark.parametrize` over each function's test vectors from
    `data.json`. Use `pytest.param(..., id=tc_name)` for clear test IDs.

3. **Handles input type conversions** per function:

    - `gen_meta_code_v0(name, description, meta, bits)`: `description` may be `""` (pass as empty
        string or `None` — the Rust side accepts `Option<&str>`, but empty string `""` is valid and
        distinct from `None`). `meta` may be `None`, a JSON string, or a dict/object — if it's a
        dict, serialize to JSON string via `json.dumps(meta, sort_keys=True)`. The last element is
        `bits` (int).
    - `gen_text_code_v0(text, bits)`: straight string + int.
    - `gen_image_code_v0(pixels, bits)`: `pixels` is a list of ints in JSON → convert to
        `bytes(pixels)`.
    - `gen_audio_code_v0(cv, bits)`: `cv` is a list of signed ints → pass as Python list.
    - `gen_video_code_v0(frame_sigs, bits)`: `frame_sigs` is a list of lists of ints → pass as Python
        list of lists.
    - `gen_mixed_code_v0(codes, bits)`: `codes` is a list of strings → pass as Python list.
    - `gen_data_code_v0(data, bits)`: `data` has `"stream:"` prefix — strip prefix, hex-decode
        remainder to `bytes`.
    - `gen_instance_code_v0(data, bits)`: same `"stream:"` prefix pattern as data code.
    - `gen_iscc_code_v0(codes)`: input is `[list_of_code_strings]` (just one element in inputs
        array). No `wide` parameter in test vectors — use default `wide=False`.

4. **Asserts only the `iscc` output field** — compare `result == expected_iscc` for each vector. The
    `iscc` output field is always present. Other output fields (`name`, `metahash`, `datahash`,
    `filesize`, `characters`, `parts`) exist but are NOT returned by the Python API (which returns
    a single string), so skip them.

5. **Meta code special cases**:

    - `description=""` in JSON → pass as `""` to Python (empty string, not `None`)
    - `meta` can be `None` → pass as `None`
    - `meta` can be a dict (`{'some': 'object'}`) → pass as `json.dumps(meta, sort_keys=True)`
    - `meta` can be a data URL string → pass as-is

6. **Counts**: The test should produce 46 parametrized test cases total (16 + 5 + 3 + 5 + 3 + 2 + 4
    \+ 3 + 5 = 46).

7. **Style**: Use functional test style (no classes). Import all 9 functions from `iscc`. Use
    `pathlib.Path` for file paths. Keep the file concise — use helper functions to reduce
    repetition across the 9 test functions.

## Verification

- `pytest tests/test_conformance.py -v` passes with 46 test cases (all green)
- `pytest tests/` passes (both smoke and conformance tests — 49 total: 3 + 46)
- `ruff check tests/test_conformance.py` clean
- `ruff format --check tests/test_conformance.py` clean
- All pre-commit hooks pass (`uv run prek run --all-files`)

## Done When

All 46 conformance vectors pass through pytest, validating that the Python bindings produce
identical output to the Rust core for every test vector in `data.json`.
