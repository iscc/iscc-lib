# Next Work Package

## Step: Polish docs landing page code examples and Key Features

## Goal

Fix three cosmetic inaccuracies in `docs/index.md` identified during the iteration 30 review: the
Key Features bullet omits Java and Go, the Rust Quick Start example treats the result as a JSON
string instead of a struct, and the Python Quick Start uses unnecessary `json.loads`. These are the
last documentation polish items before the project reaches full target compliance.

## Scope

- **Create**: (none)
- **Modify**: `docs/index.md`
- **Reference**: `crates/iscc-lib/src/lib.rs` (return type of `gen_text_code_v0`),
    `crates/iscc-py/python/iscc_lib/__init__.py` (Python API surface)

## Not In Scope

- Changing the README.md Quick Start examples (already correct — uses `result.iscc` for Rust and
    `result['iscc']` for Python)
- Adding or modifying the howto guides
- Updating any binding code or test files
- Evaluating the TypeScript port (separate [low] issue)
- Any CI/CD or publishing changes

## Implementation Notes

Three targeted edits in `docs/index.md`:

1. **Line 34 — Key Features bullet**: Change
    `Multi-language — use from Rust, Python, Node.js, WebAssembly, or C` to
    `Multi-language — use from Rust, Python, Java, Go, Node.js, WebAssembly, or C`. This matches
    the README.md line 25 which already lists all languages.

2. **Lines 62-63 — Rust Quick Start**: The current code is:

    ```rust
    let result = gen_text_code_v0("Hello World", 64)?;
    println!("{result}"); // JSON string
    ```

    `gen_text_code_v0` returns `IsccResult<TextCodeResult>` (a struct with an `.iscc` field), not a
    JSON string. Fix to:

    ```rust
    let result = gen_text_code_v0("Hello World", 64)?;
    println!("{}", result.iscc);
    ```

3. **Lines 72-77 — Python Quick Start**: The current code is:

    ```python
    import json
    from iscc_lib import gen_text_code_v0

    result = json.loads(gen_text_code_v0("Hello World"))
    print(result["iscc"])
    ```

    The Python binding returns a dict-like `IsccResult` object directly (with `__getattr__` →
    `__getitem__` delegation). No `json.loads` needed. Fix to:

    ```python
    from iscc_lib import gen_text_code_v0

    result = gen_text_code_v0("Hello World")
    print(result["iscc"])
    ```

    Note: Python `gen_text_code_v0` takes only `text` (with `bits=64` default) — no second argument
    needed.

## Verification

- `uv run zensical build` exits 0 — docs site builds without errors
- `grep 'Java, Go' docs/index.md` matches — Key Features mentions Java and Go
- `grep -c 'json.loads' docs/index.md` returns 0 — no unnecessary json.loads in Python example
- `grep -c 'import json' docs/index.md` returns 0 — no unused json import
- `grep 'result.iscc' docs/index.md` matches — Rust example accesses struct field
- `grep -c 'JSON string' docs/index.md` returns 0 — misleading comment removed
- `mise run check` passes — all pre-commit hooks clean

## Done When

All seven verification criteria pass, confirming the three cosmetic fixes are applied correctly and
the docs site builds cleanly.
