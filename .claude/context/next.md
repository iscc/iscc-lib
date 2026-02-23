# Next Work Package

## Step: Add Node.js CI job to workflow

## Goal

Add a Node.js job to `.github/workflows/ci.yml` that builds the napi native addon and runs the 46
JavaScript conformance tests, bringing Node.js to the same CI verification level as Rust and Python.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add a `nodejs` job
- **Reference**: `crates/iscc-napi/package.json` (build/test scripts), existing Rust and Python jobs
    in `ci.yml` (patterns to follow), learnings.md (CI action conventions)

## Implementation Notes

Add a new job named `nodejs` (display name: `Node.js (napi build, test)`) to the existing CI
workflow. Follow the established patterns from the Rust and Python jobs:

1. **Standard actions**: `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`,
    `Swatinem/rust-cache@v2`, `actions/setup-node@v4` (Node.js 20.x LTS)
2. **Build step**: `cd crates/iscc-napi && npm install` — this installs `@napi-rs/cli` and the napi
    build runs during install via napi-rs conventions. If npm install alone doesn't trigger the
    build, add an explicit `npm run build` step after install.
3. **Test step**: `cd crates/iscc-napi && npm test` — runs
    `node --test __tests__/conformance.test.mjs`
4. Do NOT use `mise` — call tools directly (per learnings)
5. The Rust toolchain + cache are needed because napi builds the native Rust addon
6. The job should be independent (no `needs:` dependency on the rust job) — all three jobs run in
    parallel

**Important**: The `napi build --platform` command (used in `npm run build`) compiles the Rust crate
into a `.node` native addon. The `package.json` scripts already have the correct build commands.
Check whether `npm install` alone triggers the napi build — if not, add `npm run build` explicitly
before `npm test`.

Typical napi-rs projects do NOT auto-build on `npm install` — the build must be triggered
explicitly. So the CI steps should be:

```yaml
  - name: Install npm dependencies
    run: npm install
    working-directory: crates/iscc-napi
  - name: Build native addon
    run: npx napi build --platform
    working-directory: crates/iscc-napi
  - name: Run tests
    run: npm test
    working-directory: crates/iscc-napi
```

Use `working-directory` instead of `cd` for cleaner YAML (matches GitHub Actions best practices).

## Verification

- `ci.yml` contains three jobs: `rust`, `python`, `nodejs`
- The `nodejs` job uses `actions/setup-node@v4` with Node.js 20
- The `nodejs` job builds the native addon and runs `npm test`
- The workflow YAML is valid (no syntax errors)
- `cargo test -p iscc-lib` still passes (143 tests)
- `cargo clippy -p iscc-lib -- -D warnings` is clean
- `cargo fmt --all --check` is clean

## Done When

The `ci.yml` file contains a working Node.js job that builds the napi crate and runs the conformance
tests, and all existing verification criteria (Rust tests, clippy, fmt) continue to pass.
