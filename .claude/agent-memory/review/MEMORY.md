# Review Agent Memory

Review patterns, quality gate knowledge, and common issues accumulated across CID iterations.

## Quality Gate Details

- `mise run check` runs 14 pre-commit hooks (file hygiene, formatting, linting)
- Pre-push hooks (clippy, cargo test, pytest, etc.) are not run by `mise run check` — verify clippy
    separately with `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests are NOT part of `mise run check` or pre-push hooks — must run `mvn test` explicitly
- Go tests are NOT part of `mise run check` or pre-push hooks — must run
    `cd packages/go && mise exec -- go test ./...` explicitly

## Common Issues

- Unused imports in Java code (e.g., `JsonNull` imported but only `isJsonNull()` method on
    `JsonElement` is used) — quick fix, remove the import
- Verification criteria in next.md that use generic `grep` patterns may false-positive on the
    replacement code — always verify grep criteria match the actual problematic pattern, not just a
    substring
- next.md test case specifications may have incorrect expected values (e.g., text_clean double-space
    collapsing) — when the advance agent adjusts test expectations, verify against the actual Rust
    implementation behavior rather than just accepting the spec
- Advance agent handoff test counts may be off by 1 (e.g., counting TestMain as a test) — always
    verify by running tests and counting top-level test functions
- `json.dumps` reformats JSON files (e.g., inline arrays become multi-line) — cosmetic but may
    appear as unintended changes in diffs. Check that formatting changes are idempotent

## Review Shortcuts

- Rust-only internal refactors (no public API changes, no binding crate changes):
    `cargo test -p   iscc-lib` + `cargo clippy -p iscc-lib -- -D warnings` + `mise run check` is
    sufficient — skip Maven/Go/Node/WASM tests

- For Java conformance test reviews: verify vector count matches expected (46 total:
    16+5+3+5+3+2+4+3+5), check `mvn test` output for 0 failures, compare structure against Node.js
    conformance tests in `crates/iscc-napi/__tests__/conformance.test.mjs`

- For Go conformance test reviews: same 46 vector count, check `go test -v` output shows all
    subtests pass, verify memory helpers handle empty/nil inputs correctly

- Clippy workspace check is fast (~2s) after initial build — always run it

- Documentation-only changes (READMEs, markdown): `mise run check` + clippy is sufficient — no need
    to run full test suites since no code was modified

- Python-only changes: `mise run check` + `pytest` is sufficient; skip `cargo test` and `mvn test`
    unless Rust/Java code was also modified

- Go-only changes: `mise run check` + `cd packages/go && mise exec -- go test ./...` is sufficient
    (must `cd` into the Go module directory — running from repo root with `./packages/go/` path
    fails with "cannot find main module")

- Full test suite (198 tests) runs in \<1s — always run it for Python changes

- Script-only changes (new Python scripts, mise task additions): `mise run check` + direct script
    invocation is sufficient — skip all test suites unless the script modifies test infrastructure

- Config-only changes (Cargo.toml metadata, wasm-pack profiles, CI workflow YAML): `mise run check`
    \+ `cargo check -p <crate>` is sufficient. If wasm-pack config changed, also run
    `wasm-pack build --target web --release crates/iscc-wasm` to verify end-to-end

## Codex Review Integration

- `codex exec review --ephemeral --commit HEAD` output ends with structured findings after a `codex`
    marker line. Use `sed -n '/^codex$/,$ p' /tmp/codex-review.txt | tail -n +2` to extract them
- Codex typically runs tests and grep searches to verify the commit — its findings are advisory and
    should be cross-referenced with your own analysis
- The `--commit HEAD~1` in the protocol template assumes advance is at HEAD~1, but when the review
    agent runs immediately after advance, the advance commit is at HEAD. Always use `--commit HEAD`
    for the advance commit (or verify with `git log` first). Codex reviewing the wrong commit
    (define-next instead of advance) produces mostly irrelevant findings about planning docs

## Python Binding State

- `__all__` count in `iscc_lib/__init__.py` is 45 as of iteration 7 (30 Tier 1 API symbols + 10
    result type classes + `__version__` + MT, ST, VS, core_opts)

## Verification Patterns

- `grep -c` counts ALL matching lines including function definitions — when next.md specifies "4
    call sites" but the function name also appears in a definition, expect count = call sites + 1.
    This is a valid pass if the arithmetic checks out

## Gotchas

- Git log shows iteration numbering resets when a new CID run starts (iteration 12 → iteration 1) —
    this is normal, each `mise run cid:run` starts a new run
- Go via mise requires `mise exec --` prefix — `go` is not on PATH in all environments
- The advance commit is at HEAD (not HEAD~1) when the review hasn't committed yet — use
    `git diff HEAD~1..HEAD` for the advance diff (define-next → advance)
- The `--stat` in handoff template says `git diff HEAD~2..HEAD~1` but this is wrong when the advance
    commit is the latest — always verify with `git log --oneline -5` first, then use the correct
    range (typically `git diff HEAD~1..HEAD` for define-next → advance)
