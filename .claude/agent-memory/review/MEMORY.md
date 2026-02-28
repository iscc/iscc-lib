# Review Agent Memory

Review patterns, quality gate knowledge, and common issues accumulated across CID iterations.

## Quality Gate Details

- `mise run check` runs 14 pre-commit hooks (file hygiene, formatting, linting)
- Pre-push hooks (clippy, cargo test, pytest, etc.) are not run by `mise run check` — verify clippy
    separately with `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests are NOT part of `mise run check` or pre-push hooks — must run `mvn test` explicitly
- Go tests are NOT part of `mise run check` or pre-push hooks — must run
    `cd packages/go && mise exec -- go test ./...` explicitly
- `check-added-large-files` threshold is `--maxkb=1024` (temporarily raised for Go WASM binary).
    Must be restored to `--maxkb=256` after Go pure rewrite removes the WASM binary from git

## Common Issues

- Go `go get` adds dependencies as `// indirect` even when directly imported — advance agents should
    run `go mod tidy` after adding deps. Check for this in review
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
- Doc examples claiming "ISCC:" prefix on decompose results — `iscc_decompose` returns units WITHOUT
    the prefix. Always cross-check doc comments against actual function behavior (Rust docstring +
    test assertions)

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

- `codex exec review --ephemeral --commit HEAD` output ends with structured findings after
    `Full review comments:` marker. Use
    `sed -n '/^Full review comments:/,$ p' /tmp/codex-review.txt` to extract them
- Codex typically runs tests and grep searches to verify the commit — its findings are advisory and
    should be cross-referenced with your own analysis
- The `--commit HEAD~1` in the protocol template assumes advance is at HEAD~1, but when the review
    agent runs immediately after advance, the advance commit is at HEAD. Always use `--commit HEAD`
    for the advance commit (or verify with `git log` first). Codex reviewing the wrong commit
    (define-next instead of advance) produces mostly irrelevant findings about planning docs
- Codex findings about Go codec design (silent truncation, dash stripping, trailing bytes) were all
    dismissed because Go faithfully mirrors Rust reference. When reviewing Go ports, always validate
    Codex findings against the Rust implementation before acting on them

## Binding State

- Python `__all__` count is 45 as of iteration 7 (30 Tier 1 API + 10 result types + `__version__` +
    MT, ST, VS, core_opts). All 30/30 Tier 1 symbols propagated
- Node.js has 30/30 Tier 1 symbols as of iteration 8 (124 tests total: 103 existing + 21 new)
- WASM has 30/30 Tier 1 symbols as of iteration 9 (59 unit tests + 9 conformance tests)
- C FFI has 30/30 Tier 1 symbols as of iteration 10 (77 Rust unit tests, 49 C test assertions)
- Java JNI has 30/30 Tier 1 symbols as of iteration 11 (58 Maven tests: 51 existing + 7 new)
- Go/wazero has 30/30 Tier 1 symbols as of iteration 12 (48 total Runtime methods: 27 public + 21
    private helpers, 7 new tests)
- Go pure rewrite progress: codec complete (570 lines, 48 tests), text utils complete (130 lines, 21
    tests), algorithms complete — CDC (129 lines, 15 tests), MinHash (205 lines, 8 tests),
    SimHash+SlidingWindow (86 lines, 14 tests), DCT (52 lines, 10 tests), WTA-Hash (92 lines, 9
    tests). All 9 gen functions DONE: GenMetaCodeV0 (281 lines) + GenTextCodeV0 (41 lines) + xxh32
    (81 lines) + GenDataCodeV0 (90 lines) + GenInstanceCodeV0 (67 lines) + GenImageCodeV0 (134
    lines) + GenAudioCodeV0 (112 lines) + GenVideoCodeV0 (61 lines) + GenMixedCodeV0 (92 lines) +
    GenIsccCodeV0 (148 lines) + ConformanceSelftest (471 lines, all 46 vectors embedded via
    `//go:embed`). WASM bridge types renamed: `WasmDataHasher`, `WasmInstanceHasher`. 30/30 Tier 1
    symbols in pure Go. Next: WASM bridge cleanup (remove iscc.go, iscc_ffi.wasm, wazero dep,
    restore 256KB threshold)

## Binding Propagation Review Shortcuts

- For napi-rs binding propagation: `cd crates/iscc-napi && npm test` +
    `cargo clippy -p iscc-napi   --all-targets -- -D warnings` + `mise run check` covers all gates.
    Verify 7 new symbols individually via `node -e` one-liners (constants values, function types)

- Binding propagation diffs are typically 2 files: native wrapper source + test file. Quick to
    review — check error mapping pattern, type conversions, test coverage categories

- WASM binding propagation: `wasm-pack test --node crates/iscc-wasm` +
    `wasm-pack test --node crates/iscc-wasm --features conformance` +
    `cargo clippy -p iscc-wasm --all-targets -- -D warnings` + `mise run check` covers all gates.
    Remember to update `crates/iscc-wasm/CLAUDE.md` API surface list when symbols are added

- C FFI binding propagation: `cargo test -p iscc-ffi` +
    `cargo clippy -p iscc-ffi --all-targets -- -D warnings` + `mise run check` + C test compilation
    (requires cbindgen header generation:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/tests/iscc.h`
    then gcc build and run). Clean up generated iscc.h after testing

- Java JNI binding propagation: `cargo build -p iscc-jni` +
    `cargo clippy -p iscc-jni --all-targets -- -D warnings` + `cd crates/iscc-jni/java && mvn test`
    \+ `mise run check` covers all gates. Use `grep -c 'pub extern "system" fn'` (not
    `extern "system"`) to count actual functions — the module doc comment also matches the looser
    pattern

- For JNI `isccDecode` returning structured data: verify `env.find_class` path uses `/` separators
    (not `.`), constructor signature matches class constructor, and `JValue::Object` wraps byte
    array reference correctly

## Verification Patterns

- `grep -c` counts ALL matching lines including function definitions — when next.md specifies "4
    call sites" but the function name also appears in a definition, expect count = call sites + 1.
    This is a valid pass if the arithmetic checks out

## Documentation Review Patterns

- Always verify documented API names against actual binding source code attributes (`js_name`,
    `#[pyfunction]`, `#[napi(js_name)]`) — next.md specs may have incorrect naming that the advance
    agent faithfully reproduces
- WASM constants have `js_name = "META_TRIM_NAME"` (uppercase) despite Rust function being
    `meta_trim_name()` — this is a known divergence point

## Issues Cleanup

- The review agent only cleans up issues resolved in the *current* iteration's advance step. It does
    NOT sweep the full issues.md backlog for stale entries resolved in prior CID loops. This led to
    issues #5-#8 persisting for 4+ iterations after their fixes landed. **Mitigation:** after
    verifying the advance work, also scan issues.md for any other entries that are now resolved
    (check state.md "met" sections against issue descriptions)

## Environment

- Python `iscc_lib` module must be compiled before pre-push hooks can pass. If `ty` or `pytest`
    fails with `No module named 'iscc_lib'`, run
    `cd crates/iscc-py && uv run maturin develop --release` to build the Python extension. This is a
    one-time setup per container/checkout

## Gotchas

- Git log shows iteration numbering resets when a new CID run starts (iteration 12 → iteration 1) —
    this is normal, each `mise run cid:run` starts a new run
- Go via mise requires `mise exec --` prefix — `go` is not on PATH in all environments
- The advance commit is at HEAD (not HEAD~1) when the review hasn't committed yet — use
    `git diff HEAD~1..HEAD` for the advance diff (define-next → advance)
- The `--stat` in handoff template says `git diff HEAD~2..HEAD~1` but this is wrong when the advance
    commit is the latest — always verify with `git log --oneline -5` first, then use the correct
    range (typically `git diff HEAD~1..HEAD` for define-next → advance)
