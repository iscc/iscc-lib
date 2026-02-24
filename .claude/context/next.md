# Next Work Package

## Step: Fix napi packaging, version skew, and unnecessary clone

## Goal

Resolve all three open normal-priority issues in the napi crate (`crates/iscc-napi`): add a
`"files"` allowlist to `package.json` so `npm publish` includes entrypoints, fix the
`alg_cdc_chunks` wrapper to avoid per-chunk Vec allocation, and regenerate `index.js`/`index.d.ts`
with the correct version (`0.0.1`) to eliminate the version skew. These fixes make the napi package
publish-ready.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-napi/package.json`, `crates/iscc-napi/src/lib.rs`
- **Reference**: `.claude/context/issues.md` (three napi issues: version skew, npm packaging,
    unnecessary clone)

## Not In Scope

- Returning structured result objects from `gen_*_v0` functions (state.md notes napi returns only
    `.iscc` string) — that's a larger feature change tracked separately
- Changing any other binding crate (WASM silent-null, FFI video copy, etc.)
- Adding `npm/` platform-specific package directories or `napi prepublish` setup — that's for the
    release workflow, which already handles it
- Modifying the napi `.gitignore` — generated files (`index.js`, `index.d.ts`, `*.node`) should
    remain gitignored per established convention
- Adding new tests — existing 66 tests (9 conformance + 57 function) already cover `alg_cdc_chunks`
    behavior; the clone fix changes allocation strategy, not output

## Implementation Notes

### 1. npm packaging fix (`package.json`)

Add a `"files"` allowlist to `package.json` that explicitly includes the files needed in the npm
tarball. Without this, `npm publish` falls back to `.gitignore` which excludes `index.js` and
`index.d.ts` — the package's main entry and type declarations.

```json
"files": [
  "index.js",
  "index.d.ts",
  "*.node",
  "README.md"
]
```

Place the `"files"` field after `"types"` and before `"napi"` for conventional ordering.

### 2. alg_cdc_chunks clone removal (`src/lib.rs`)

Change line ~230 from:

```rust
iscc_lib::alg_cdc_chunks(data.as_ref(), utf32, avg)
    .iter()
    .map(|c| Buffer::from(c.to_vec()))
    .collect()
```

To:

```rust
iscc_lib::alg_cdc_chunks(data.as_ref(), utf32, avg)
    .into_iter()
    .map(|c| Buffer::from(c))
    .collect()
```

`alg_cdc_chunks` returns `Vec<&[u8]>` (borrowed slices). Using `.into_iter()` consumes the Vec and
yields `&[u8]` directly. `Buffer::from(&[u8])` copies the slice data into a napi Buffer in one step,
eliminating the intermediate `Vec<u8>` allocation from `.to_vec()`.

If `Buffer::from(&[u8])` doesn't compile (napi v3 may only impl `From<Vec<u8>>`), use
`.map(|c: &[u8]| Buffer::from(c.to_vec()))` with `into_iter()` — this still avoids the double-deref
from `.iter()` and is cleaner, though the allocation saving is minimal. In that case, leave a
`// TODO: napi-rs v3 lacks From<&[u8]> for Buffer` comment.

### 3. Version skew fix (regenerate `index.js`)

After the code changes, run `npx napi build --platform` in `crates/iscc-napi/` to regenerate
`index.js` and `index.d.ts` with the correct version from `package.json` (`0.0.1`). This replaces
the 26 occurrences of `expected 0.1.0` with `expected 0.0.1`.

The regenerated files remain gitignored — this is intentional (CI regenerates them too). The fix
ensures local development uses correct version checks. Verify after regenerating:
`grep -c 'expected 0.0.1' crates/iscc-napi/index.js` should return 26 (replacing the stale 0.1.0).

## Verification

- `cargo clippy -p iscc-napi -- -D warnings` clean
- `npm test` passes in `crates/iscc-napi/` (all 66 existing tests)
- `grep '"files"' crates/iscc-napi/package.json` returns a match (files field present)
- `grep 'c\.to_vec()' crates/iscc-napi/src/lib.rs` returns 0 matches (clone removed)
- `grep -c 'expected 0.0.1' crates/iscc-napi/index.js` returns 26 (version skew fixed)

## Done When

All verification criteria pass: clippy clean, all 66 napi tests pass, `package.json` has `"files"`
allowlist, `alg_cdc_chunks` no longer uses `.to_vec()`, and regenerated `index.js` references the
correct `0.0.1` version.
