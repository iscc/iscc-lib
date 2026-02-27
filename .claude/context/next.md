# Next Work Package

## Step: Implement `json_to_data_url` — 30th Tier 1 symbol

## Goal

Add `json_to_data_url` as the last of 30 Tier 1 public symbols in the Rust core. This utility
function converts a JSON string into a `data:` URL with JCS canonicalization, enabling all bindings
to support dict/object meta parameters by delegating encoding to Rust (issue #5 layer 1).

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/lib.rs` — add `pub fn json_to_data_url`, tests, and doc comment
- **Reference**:
    - `reference/iscc-core/iscc_core/code_meta.py` — lines 70-77 (dict meta → data URL flow)
    - `.claude/context/specs/rust-core.md` — Encoding Utilities section (signature, behavior)
    - Existing private helpers `parse_meta_json` (line 143) and `build_meta_data_url` (line 156) in
        `crates/iscc-lib/src/lib.rs`

## Not In Scope

- Propagating `json_to_data_url` (or any of the 7 new symbols) to bindings — that's the next phase
- Refactoring `gen_meta_code_v0` to call `json_to_data_url` internally — the existing code path
    works and the refactor risks changing behavior or performance for no user-facing benefit
- Adding `core_opts` namespace or enum types to bindings — separate steps
- Non-JSON media type variants (XML, etc.) — iscc-core only supports JSON

## Implementation Notes

The function combines two existing private helpers into a single public API:

1. **`parse_meta_json(meta_str)`** (line 143) — parses JSON string, JCS-canonicalizes via
    `serde_json_canonicalizer`
2. **`build_meta_data_url(json_bytes, json_value)`** (line 156) — determines media type from
    `@context` key, base64-encodes, formats `data:` URL

**Signature**: `pub fn json_to_data_url(json: &str) -> IsccResult<String>`

Returns `IsccResult` (not bare `String`) because JSON parsing can fail. This follows the
`encode_component` / `iscc_decode` error-returning pattern for Tier 1 functions.

**Algorithm** (matches iscc-core `gen_meta_code_v0` lines 71-76):

1. Parse `json` to `serde_json::Value` — return `IsccError::InvalidInput` on invalid JSON
2. JCS-canonicalize via `serde_json_canonicalizer::to_writer` → canonical bytes
3. Check `json_value.get("@context").is_some()` → `application/ld+json` else `application/json`
4. Base64-encode canonical bytes with `data_encoding::BASE64` (standard, with padding)
5. Return `format!("data:{media_type};base64,{b64}")`

**Placement**: Define near the other encoding utilities and the existing private helpers (around
line 120-165 area, near `decode_data_url` and `build_meta_data_url`). Add a doc comment with
`# Errors` and `# Examples` sections.

**Dependencies**: All already present — `serde_json`, `serde_json_canonicalizer`, `data_encoding`.

**Tests** (add in the `#[cfg(test)]` module at the bottom of `lib.rs`):

1. Basic JSON object → data URL with `data:application/json;base64,...` prefix
2. JSON with `@context` key → `data:application/ld+json;base64,...` prefix
3. JCS canonicalization: `{"b":1,"a":2}` → verify key ordering in canonical output (decode base64
    payload and check it equals `{"a":2,"b":1}`)
4. Round-trip: feed `json_to_data_url` output into `decode_data_url` (private helper) and verify
    decoded bytes match JCS-canonical form of original input
5. Error case: invalid JSON string (e.g., `"not json"`) returns `Err(IsccError::InvalidInput(...))`
6. Compatibility: `json_to_data_url("{\"some\": \"object\"}")` produces the same data URL as the
    `test_0016_meta_data_url` conformance vector's meta field — note the conformance vector uses a
    charset-qualified data URL (`data:application/json;charset=utf-8;base64,...`) while our
    function omits charset (matching Python's `DataURL.from_byte_data`), so this may NOT match
    exactly. If it differs, document the reason in a test comment and verify the _payload_ (base64
    content) matches.

## Verification

- `cargo test -p iscc-lib` passes (292 existing + new tests, 0 failures)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `grep -c 'pub fn json_to_data_url' crates/iscc-lib/src/lib.rs` returns 1
- Test confirms `json_to_data_url("{\"key\":\"value\"}")` returns a string starting with
    `data:application/json;base64,`
- Test confirms `json_to_data_url("{\"@context\":\"x\"}")` returns a string starting with
    `data:application/ld+json;base64,`

## Done When

All verification criteria pass, confirming `json_to_data_url` is the 30th and final Tier 1 public
symbol in the Rust core with proper JCS canonicalization, media type detection, and error handling.
