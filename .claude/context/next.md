# Next Work Package

## Step: Accept dict meta in Python gen_meta_code_v0

## Goal

Enable `gen_meta_code_v0` in the Python binding to accept `dict` for the `meta` parameter (in
addition to `str | None`), matching iscc-core behavior. This is one of 4 remaining iscc-core drop-in
compatibility extensions and unblocks `iscc-sdk` migration (issue #5).

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py` — widen `gen_meta_code_v0` wrapper to accept
        `meta: str | dict | None`, serialize dict to JSON then call `json_to_data_url()`
    - `tests/test_new_symbols.py` — add tests for dict meta parameter
- **Reference**:
    - `reference/iscc-core/iscc_core/code_meta.py` — lines 62-77 show the dict→data URL conversion
        logic
    - `.claude/context/specs/python-bindings.md` — "Dict for gen_meta_code_v0 meta Parameter" section
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — type stubs (lowlevel stays `str | None`)

## Not In Scope

- Updating `_lowlevel.pyi` — the lowlevel Rust function still only accepts `str | None`; the dict
    handling is entirely in the Python wrapper layer
- Adding `MT`, `ST`, `VS` IntEnum classes (issue #6) — separate step
- Adding `core_opts` SimpleNamespace (issue #8) — separate step
- Widening `gen_image_code_v0` for PIL pixel data (issue #4) — separate step
- Propagating symbols to Node.js/WASM/C/Java/Go bindings — separate step
- Any Rust core changes — this is Python-wrapper-only

## Implementation Notes

In `crates/iscc-py/python/iscc_lib/__init__.py`:

1. Add `import json` at the top of the file.
2. In `gen_meta_code_v0`, before calling `_gen_meta_code_v0`, check if `meta` is a `dict`:
    ```python
    def gen_meta_code_v0(
        name: str,
        description: str | None = None,
        meta: str | dict | None = None,
        bits: int = 64,
    ) -> MetaCodeResult:
        """Generate an ISCC Meta-Code from content metadata."""
        if isinstance(meta, dict):
            meta = json_to_data_url(
                json.dumps(meta, separators=(",", ":"), ensure_ascii=False)
            )
        return MetaCodeResult(_gen_meta_code_v0(name, description, meta, bits))
    ```

Key details from the iscc-core reference (`code_meta.py` lines 70-76):

- The dict is serialized via `jcs.canonicalize(meta)` (JCS = RFC 8785), then base64-encoded into a
    data URL.
- `json_to_data_url()` in the Rust core already handles JCS canonicalization internally (it parses
    the JSON, re-serializes to JCS, and base64-encodes). So the Python wrapper only needs to do
    `json.dumps()` to get valid JSON, then `json_to_data_url()` handles the rest.
- The `separators=(",", ":")` in `json.dumps` produces compact JSON (no extra whitespace). This
    doesn't need to be JCS-canonical because `json_to_data_url()` re-canonicalizes.
- `ensure_ascii=False` preserves non-ASCII characters (matching iscc-core behavior).
- The `@context` key detection for media type selection (`application/json` vs
    `application/ld+json`) is handled inside `json_to_data_url()` — no Python-side logic needed.

Tests to add in `tests/test_new_symbols.py`:

- `test_gen_meta_dict_meta_basic` — pass `meta={"key": "value"}`, verify result has `meta` key
    starting with `data:application/json;base64,`
- `test_gen_meta_dict_meta_ld_json` — pass
    `meta={"@context": "https://schema.org", "name": "Test"}`, verify result `meta` starts with
    `data:application/ld+json;base64,`
- `test_gen_meta_dict_meta_matches_string` — pass the same data as both dict and pre-computed data
    URL string, verify ISCC codes match (confirming dict→string→Rust path produces identical output)
- `test_gen_meta_str_meta_still_works` — regression: pass `meta="data:application/json;base64,..."`
    as string, verify it still works
- `test_gen_meta_dict_meta_none_still_works` — regression: `meta=None` still works

## Verification

- `uv run pytest tests/test_new_symbols.py -x` passes (13 existing + ~5 new tests)
- `uv run pytest tests/ -x` passes (all ~172+ tests, 0 failures)
- `uv run ruff check crates/iscc-py/python/` clean
- `uv run ruff format --check crates/iscc-py/python/` clean
- `python -c "from iscc_lib import gen_meta_code_v0; r = gen_meta_code_v0('Test', meta={'key': 'val'}); assert r['meta'].startswith('data:application/json;base64,')"`
    exits 0
- `cargo clippy -p iscc-py -- -D warnings` clean (no Rust changes expected, but verify)

## Done When

All verification criteria pass, confirming `gen_meta_code_v0` accepts `dict` for the `meta`
parameter and produces output matching the iscc-core reference behavior.
