# Review Agent Memory

Review patterns, quality gate knowledge, and common issues accumulated across CID iterations.

## Quality Gate Details

- `mise run check` runs 14 pre-commit hooks (file hygiene, formatting, linting)
- Pre-push hooks (clippy, cargo test, pytest, etc.) are not run by `mise run check` — verify clippy
    separately with `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests are NOT part of `mise run check` or pre-push hooks — must run `mvn test` explicitly
- The advance commit is at HEAD (not HEAD~1) when iterating — use `git diff HEAD~1..HEAD` for the
    advance diff, not `HEAD~2..HEAD~1` (that's the define-next diff)

## Common Issues

- Unused imports in Java code (e.g., `JsonNull` imported but only `isJsonNull()` method on
    `JsonElement` is used) — quick fix, remove the import

## Review Shortcuts

- For Java conformance test reviews: verify vector count matches expected (46 total:
    16+5+3+5+3+2+4+3+5), check `mvn test` output for 0 failures, compare structure against Node.js
    conformance tests in `crates/iscc-napi/__tests__/conformance.test.mjs`
- Clippy workspace check is fast (~2s) after initial build — always run it
- Documentation-only changes (READMEs, markdown): `mise run check` + clippy is sufficient — no need
    to run full test suites since no code was modified

## Gotchas

- Git log shows iteration numbering resets when a new CID run starts (iteration 12 → iteration 1) —
    this is normal, each `mise run cid:run` starts a new run
