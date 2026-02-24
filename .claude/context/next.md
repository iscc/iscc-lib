# Next Work Package

## Step: Create per-crate READMEs for iscc-lib, iscc-py, and iscc-napi

## Goal

Create registry-facing README.md files for the three primary publishable crates so that developers
on crates.io, PyPI, and npm see complete, standalone documentation without needing to visit the
GitHub repository.

## Scope

- **Create**: `crates/iscc-lib/README.md`, `crates/iscc-py/README.md`, `crates/iscc-napi/README.md`
- **Modify**: `crates/iscc-lib/Cargo.toml` (change `readme` from `"../../README.md"` to
    `"README.md"`), `crates/iscc-py/pyproject.toml` (add `readme = "README.md"` under `[project]`)
- **Reference**: `README.md` (root — reuse "What is ISCC" text, quick start examples, MainTypes
    table), `.claude/context/target.md` § Per-Crate READMEs (required sections),
    `crates/iscc-napi/package.json` (npm auto-detects README.md, no change needed)

## Not In Scope

- READMEs for iscc-wasm, iscc-ffi, iscc-jni, or packages/go — those are batch 2
- Root README updates (Java/Go sections) — separate step
- Documentation site changes (howto pages) — separate step
- Publishing configuration or CI changes
- Performance optimizations from issues.md

## Implementation Notes

Each README follows the template from target.md § Per-Crate READMEs. Required sections:

1. **Package name + tagline** — one-line description. Use the package's registry name (e.g.,
    `iscc-lib` for crates.io, `iscc-lib` for PyPI, `@iscc/lib` for npm).

2. **Badges** — registry version badge, CI status badge, license badge. Badge URLs:

    - CI: `https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg`
    - Crate: `https://img.shields.io/crates/v/iscc-lib.svg`
    - PyPI: `https://img.shields.io/pypi/v/iscc-lib.svg`
    - npm: `https://img.shields.io/npm/v/@iscc/lib.svg`
    - License: `https://img.shields.io/badge/License-Apache_2.0-blue.svg`

3. **Experimental notice** — same text as root README (v0.0.x, APIs may change)

4. **What is ISCC** — reuse the 2-sentence version from root README (the ISCC is a
    similarity-preserving fingerprint…)

5. **Installation** — registry-specific command only:

    - Rust: `cargo add iscc-lib`
    - Python: `pip install iscc-lib`
    - Node.js: `npm install @iscc/lib`

6. **Quick start** — minimal code example. Use the examples from root README as a base but verify
    correctness:

    - **Rust**: `iscc_lib::gen_meta_code_v0(...)` returns `MetaCodeResult` struct with `.iscc` field
    - **Python**: `iscc_lib.gen_meta_code_v0(...)` returns dict with `['iscc']` key
    - **Node.js**: `ic.gen_meta_code_v0(...)` returns a **string** (not an object) — the napi binding
        returns `String` directly. The example must reflect this:
        `const iscc = gen_meta_code_v0("title");`

7. **API overview** — list the 9 `gen_*_v0` functions plus key utilities available in this binding
    (text utils, algorithm primitives, streaming hashers, etc.)

8. **Links** — documentation site (`https://lib.iscc.codes`), repository
    (`https://github.com/iscc/iscc-lib`), ISCC specification
    (`https://www.iso.org/standard/77899.html`), ISCC Foundation (`https://iscc.io`)

9. **License** — Apache-2.0

**Manifest updates:**

- `crates/iscc-lib/Cargo.toml`: change line `readme = "../../README.md"` → `readme = "README.md"`
- `crates/iscc-py/pyproject.toml`: add `readme = "README.md"` under `[project]` section (after
    `dynamic = ["version"]`)
- `crates/iscc-napi/package.json`: no change needed — npm auto-detects `README.md` in the package
    directory

**Style notes:**

- Keep READMEs concise (80-120 lines each). These are registry pages, not full documentation.
- Use consistent formatting across all three READMEs — same section order, similar lengths.
- No emojis. Use standard markdown headings and code blocks.
- Use ISCC Foundation URL `https://iscc.io` (not other variants).

## Verification

- `test -f crates/iscc-lib/README.md && echo OK` exits 0
- `test -f crates/iscc-py/README.md && echo OK` exits 0
- `test -f crates/iscc-napi/README.md && echo OK` exits 0
- `grep -q 'readme = "README.md"' crates/iscc-lib/Cargo.toml && echo OK` exits 0
- `grep -q 'readme = "README.md"' crates/iscc-py/pyproject.toml && echo OK` exits 0
- Each README contains all required sections: `grep -c '^## ' crates/iscc-lib/README.md` returns at
    least 6 (Installation, Quick Start, API, Links, License, plus intro sections)
- `cargo clippy --workspace --all-targets -- -D warnings` remains clean (no code changes, but verify
    manifest change doesn't break anything)
- `mise run check` passes (formatting hooks validate markdown)

## Done When

All three per-crate README files exist with registry-specific content, manifest files point to local
READMEs, and all verification commands pass.
