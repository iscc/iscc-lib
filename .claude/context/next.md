# Next Work Package

## Step: Promote text utilities to Tier 1 public API

## Goal

Make the 4 text utility functions (`text_clean`, `text_remove_newlines`, `text_trim`,
`text_collapse`) publicly accessible from the `iscc_lib` crate, with re-exports at the crate root
for a flat API. These are the first batch of Tier 1 symbols beyond the 9 `gen_*_v0` functions.

## Scope

- **Create**: `crates/iscc-lib/tests/test_text_utils.rs` — dedicated unit tests
- **Modify**:
    - `crates/iscc-lib/src/utils.rs` — change 4 functions from `pub(crate)` to `pub`
    - `crates/iscc-lib/src/lib.rs` — change `pub(crate) mod utils` to `pub mod utils`, add re-exports
- **Reference**:
    - `reference/iscc-core/iscc_core/code_meta.py` lines 173–228 (`text_trim`, `text_remove_newlines`,
        `text_clean`)
    - `reference/iscc-core/iscc_core/code_content_text.py` lines 108–135 (`text_collapse`)
    - `crates/iscc-lib/src/utils.rs` — current implementation
    - `crates/iscc-lib/src/lib.rs` — current module structure and re-exports

## Implementation Notes

### Visibility changes

In `utils.rs`, change these 4 function signatures:

- `pub(crate) fn text_clean(text: &str) -> String` → `pub fn text_clean(text: &str) -> String`
- `pub(crate) fn text_remove_newlines(text: &str) -> String` →
    `pub fn text_remove_newlines(text: &str) -> String`
- `pub(crate) fn text_trim(text: &str, nbytes: usize) -> String` →
    `pub fn text_trim(text: &str, nbytes: usize) -> String`
- `pub(crate) fn text_collapse(text: &str) -> String` → `pub fn text_collapse(text: &str) -> String`

Keep all other functions in `utils.rs` (like `multi_hash_blake3`, `is_c_category`,
`is_cmp_category`) at their current visibility — they are internal helpers not part of Tier 1.

### Module and re-exports in lib.rs

Change `pub(crate) mod utils;` to `pub mod utils;` so the module is accessible from outside the
crate. Add flat re-exports at the crate root so users can do `iscc_lib::text_clean(...)` without
navigating into the module:

```rust
pub use utils::{text_clean, text_collapse, text_remove_newlines, text_trim};
```

Place this near the existing `pub use types::*;` line.

### Docstrings

Each function already has doc comments. Enhance them slightly to match the Python reference
docstrings in tone and content:

- `text_clean`: "Clean and normalize text for display" — NFKC normalize, remove control chars except
    newlines, collapse consecutive empty lines, strip whitespace
- `text_remove_newlines`: "Remove newlines and collapse whitespace to single spaces" — converts
    multi-line text to a single normalized line
- `text_trim`: "Trim text so its UTF-8 encoded size does not exceed `nbytes`" — truncates at valid
    UTF-8 boundary, strips whitespace
- `text_collapse`: "Normalize and simplify text for similarity hashing" — NFD → lowercase → filter
    whitespace/control/mark/punctuation → NFKC

### Tests

Create `crates/iscc-lib/tests/test_text_utils.rs` with tests covering:

1. **text_clean**:

    - Basic NFKC normalization (e.g., fullwidth chars → ASCII)
    - Control character removal (keeps newlines)
    - Consecutive empty line collapse (max 1 empty line between content)
    - `\r\n` → `\n` normalization
    - Leading/trailing whitespace stripping
    - Empty input → empty output

2. **text_remove_newlines**:

    - Multi-line text → single line
    - Multiple consecutive spaces → single space
    - Leading/trailing whitespace removed
    - Empty input → empty output

3. **text_trim**:

    - Text shorter than nbytes → unchanged (but trimmed)
    - Text exactly at nbytes → unchanged
    - Truncation at valid UTF-8 boundary (e.g., multi-byte chars)
    - Result is whitespace-stripped

4. **text_collapse**:

    - Lowercasing
    - Whitespace removal
    - Punctuation removal
    - Diacritics removal (NFD decompose + filter marks)
    - Empty input → empty output

5. **Public API access**: Verify all 4 functions are callable via `iscc_lib::text_clean(...)` etc.
    (flat crate root import)

## Verification

- `cargo test -p iscc-lib` passes (all existing 143 tests + new text utility tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo doc -p iscc-lib --no-deps` succeeds (doc comments render)
- All 4 functions are importable from crate root: `iscc_lib::text_clean`,
    `iscc_lib::text_remove_newlines`, `iscc_lib::text_trim`, `iscc_lib::text_collapse`
- No changes to existing gen function behavior (internal usage unchanged)

## Done When

All verification criteria pass — the 4 text utilities are public Tier 1 API with dedicated tests,
accessible via flat imports from the crate root.
