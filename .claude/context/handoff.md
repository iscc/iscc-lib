## 2026-11-19 — npm packages ship a license and a plain-text description

**Done:** `crates/iscc-wasm/Cargo.toml` now sets `license.workspace = true` (Apache-2.0) and a
browser-specific `description` literal, so wasm-pack writes both into `pkg/package.json`.
`crates/iscc-napi/package.json` gained the napi `description` right after `license`.

**Files changed:**

- `crates/iscc-wasm/Cargo.toml`: added `license.workspace = true` + `description`; moved the
    `publish = false` trailing comment onto its own line (see Notes)
- `crates/iscc-napi/package.json`: added `"description"`
- `.claude/agent-memory/advance/MEMORY.md`: finished the truncated NUL line; npm metadata note

**Verification:**

- `wasm-pack build --target web --dev crates/iscc-wasm`: exit 0 (fresh build, version 0.7.0). The
    generated `pkg/package.json` has `"license": "Apache-2.0"` and the exact WASM description, so
    wasm-pack 0.13.1 does resolve `license.workspace`, and no literal fallback was needed. wasm-pack
    logged an INFO line "License key is set ... no LICENSE file(s) were found", which next.md says
    is informational
- `node -e` with `assert.strictEqual`: WASM `license` + `description`, napi `description` +
    `license` all match. Output: `node asserts OK`
- `cargo metadata --no-deps --format-version 1`: parses; `iscc-wasm` shows license `Apache-2.0` and
    the description
- Ran release.yml's "Set npm package name and version" logic on a copy of the generated JSON: result
    `@iscc/wasm 0.7.0 Apache-2.0 | Browser-compatible ...`, so both fields survive the patch
- `mise run check`: the first run failed (taplo rewrote the TOML, see Notes). After the fix it exits
    0 with no hook failures

**Next:** Ruby negative-Bignum truncation (`RBignum::to_i32`), the alternative next.md rejected for
this step. The Dependabot `soupsieve`/`setuptools` lock refresh and the Python `encode_seq`
docstrings are also small.

**Notes:**

- taplo lines up a trailing comment with the longest line in its block. With the long `description`
    line it pushed `publish = false  # Published via npm...` out to column 119. I put the comment on
    its own line above `publish = false`; the value did not change (cargo metadata `publish: []`,
    same as before)
- I did not rebuild wasm after moving the comment, because that edit only touches a comment.
    `cargo metadata` was re-checked afterwards
- Manifest-only change: the CRAP, iai and semver gates are not affected. No tests were added,
    because next.md rules out a pytest that just restates the manifest
- `crates/iscc-wasm/pkg/` is gitignored; the fresh build replaced the stale 0.5.0 copy on disk
