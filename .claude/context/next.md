# Next Work Package

## Step: Add Node.js conformance tests for the napi crate

## Goal

Verify that all 9 `gen_*_v0` napi-rs bindings produce correct ISCC codes by running the vendored
`data.json` conformance vectors from JavaScript. This bridges the gap between "napi crate compiles"
and "napi bindings are proven correct."

## Scope

- **Create**: `crates/iscc-napi/__tests__/conformance.test.mjs`
- **Modify**: `crates/iscc-napi/package.json` (add `test` script, no extra test framework)
- **Reference**: `tests/test_conformance.py` (Python test structure to mirror),
    `crates/iscc-lib/tests/data.json` (conformance vectors), `crates/iscc-napi/src/lib.rs` (function
    signatures)

## Implementation Notes

### Test Runner

Use Node.js built-in `node:test` + `node:assert/strict` (available since v20, zero extra
dependencies). No vitest/jest needed.

### Build the Native Addon First

Before tests can run, the native addon must be built:

```bash
cd crates/iscc-napi && npm install && npx napi build --platform
```

This generates `iscc-lib.*.node`, `index.js`, and `index.d.ts` in the crate root.

### package.json Changes

Add a `test` script:

```json
"scripts": {
  "build": "napi build --platform --release",
  "build:debug": "napi build --platform",
  "test": "node --test __tests__/conformance.test.mjs"
}
```

### Test File Structure

Mirror the Python conformance tests (`tests/test_conformance.py`). Load `data.json`, iterate each
function's test vectors, call the binding, assert output matches expected ISCC string.

```javascript
import {
    readFileSync
} from 'node:fs';
import {
    join,
    dirname
} from 'node:path';
import {
    fileURLToPath
} from 'node:url';
import {
    describe,
    it
} from 'node:test';
import {
    strictEqual
} from 'node:assert';
```

Import all 9 functions from the built package:

```javascript
import {
    gen_meta_code_v0,
    gen_text_code_v0,
    ...
} from '../index.js';
```

### Input Parsing (mirror Python test patterns)

- **`stream:` prefix** (for `gen_data_code_v0`, `gen_instance_code_v0`): strip `"stream:"` prefix,
    hex-decode remainder to `Buffer`. Empty after prefix → empty `Buffer.alloc(0)`.
- **`gen_meta_code_v0`**: inputs are `[name, description, meta, bits]`. If `meta` is an object
    (dict), `JSON.stringify` it with sorted keys. If `null`, pass `undefined`. Description: pass
    `null`/`undefined` if empty/null.
- **`gen_image_code_v0`**: inputs `[pixels_array, bits]` → `Buffer.from(pixels_array)` for the pixel
    data.
- **`gen_audio_code_v0`**: inputs `[cv_array, bits]` → pass the plain JS array of signed integers
    directly.
- **`gen_video_code_v0`**: inputs `[frame_sigs_array, bits]` → array of arrays of signed integers.
- **`gen_mixed_code_v0`**: inputs `[codes_array, bits]` → array of ISCC code strings.
- **`gen_iscc_code_v0`**: inputs `[codes_array]` → array of ISCC code strings. Note: no `wide`
    parameter in test vectors (default false).

### JSON Sorted Keys

For `meta` dict → string conversion, use a sort-keys approach:

```javascript
function jsonSortedStringify(obj) {
    return JSON.stringify(obj, Object.keys(obj).sort());
}
```

### Data Path

`data.json` is at `../../iscc-lib/tests/data.json` relative to the `__tests__/` directory. Use
`join(dirname(fileURLToPath(import.meta.url)), '..', '..', 'iscc-lib', 'tests', 'data.json')`.

### Test Organization

One `describe` block per gen function (9 total). Inside each, one `it` per test vector. Use the
vector key name as the test name for traceability:

```javascript
describe('gen_meta_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_meta_code_v0)) {
        it(name, () => {
            ...
        });
    }
});
```

## Verification

- `cd crates/iscc-napi && npm install && npx napi build --platform` succeeds without errors
- `cd crates/iscc-napi && npm test` runs all conformance vectors and all tests pass
- All 9 gen functions are tested (same vector count as Python: ~143 total across all functions)
- `cargo test -p iscc-lib` still passes (143 tests — no Rust regression)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean

## Done When

`npm test` in `crates/iscc-napi/` passes all conformance vectors for all 9 gen functions, matching
the Python test count, and existing Rust tests and clippy remain green.
