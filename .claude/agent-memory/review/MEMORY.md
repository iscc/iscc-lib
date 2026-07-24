# Review Agent Memory

Concise index. Detail in topic files: `review-patterns.md` (docs/verification/issues/gotchas),
`gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP gate recipes), `binding-reviews.md`
(per-binding shortcuts + UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags). Stale detail in
`MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` runs 15 pre-commit hooks (file hygiene, formatting, linting incl. Ruby)
- Pre-push hooks (clippy, cargo test, pytest, etc.) NOT in `mise run check` — verify clippy with
    `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests: `mvn test`. Go tests: `cd packages/go && mise exec -- go test ./...`
- `check-added-large-files` threshold is `--maxkb=256`
- **`ty check` external Python files**: files importing packages not in the venv (e.g.
    `conanfile.py` → `conan`) fail `ty check`. Fix: `[tool.ty.src] exclude` in `pyproject.toml` —
    proper scope exclusion, not circumvention
- **Pre-push needs `iscc_lib` built** for `ty check`/`pytest` hooks:
    `cd crates/iscc-py && uv run   maturin develop --release` first (see `binding-reviews.md`
    Environment)

## Common Issues

- Go `go get` adds deps as `// indirect` — run `go mod tidy` after
- Verification grep patterns may false-positive — verify match specificity. `grep -qv 'pattern'` is
    always true for multi-line files — use `! grep -q 'pattern'` to verify absence
- next.md test specs / expected values / test counts may be wrong — always run tests, verify against
    Rust implementation
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples
- **Docs site URL**: `https://lib.iscc.codes/` NOT `https://iscc-lib.iscc.io/`. Advance agents
    consistently get this wrong — always verify
- **next.md relative path errors**: verify relative paths with `realpath` during review
- **CI find cross-arch bug**: when multiple targets extract to same CWD with same lib name, `find`
    patterns must include target name (generic wildcards match all extracted dirs)
- **Advance agent idle claims**: always verify remaining issue priorities independently — they may
    claim "only low-priority remain" when `normal` issues still exist
- **prek stash conflict**: untracked files with formatting issues break prek stash/restore during
    commit. Fix: move untracked files to /tmp before committing, restore after
- **`mise run check` mdformat on context files** (recurring): define-next writes `next.md` +
    `define-next/MEMORY.md` non-conforming, so `prek --all-files` reformats every cycle (NOT an
    advance regression). Staged-only `git commit` is unaffected, but the pre-push mdformat hook runs
    on the whole push range and WILL reject the batch. Fix: STAGE the reformatted `next.md` +
    `define-next/MEMORY.md` into the review commit (mechanical rewrap, zero semantic change). Never
    stage `iterations.jsonl` (runner-owned)
- **No-op / human-handoff iteration (iter 111)**: verify scope is empty
    (`git diff HEAD~1..HEAD --stat -- crates/ packages/ scripts/ docs/ notes/ .claude/context/specs/`)
    and still scan `@{upstream}..HEAD` for gate circumvention. SIGNAL: only `normal` [review]
    HUMAN-REVIEW spec amendments + `low` [human] remaining (strict IDLE cond #2 NOT met) → flag
    **HUMAN REVIEW REQUESTED** (runner "pause"), NOT `**IDLE**` (reserved for all-`low`, runs
    meta-improve). Verdict still PASS; push clean batch
- **Concurrent CID loops (iter 97, resolved iter 98 — in `MEMORY-archive.md`)**: spurious
    `mise run check` "files modified" on an untouched file + mid-review working-tree change = a
    SECOND loop racing. Confirm `ps aux | grep -E 'cid:run|claude -p CID iteration'`; flag HUMAN
    REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Docs-only**: `mise run check` + clippy
- **Python-only**: `mise run check` + `pytest`
- **Go-only**: `mise run check` + `cd packages/go && mise exec -- go test ./...`
- **Ruby-only**: `mise run check` + `cargo clippy -p iscc-rb -- -D warnings` +
    `pushd crates/iscc-rb && bundle exec rake test; popd` + `bundle exec standardrb` (needs
    `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`)
- **Kotlin-only**: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy
    workspace + `mise run check`
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see gate-reviews.md Audit)
- **Version sync addition**: `mise run check` + `uv run scripts/version_sync.py --check` + clippy
- **Script-only (shell)**: `bash -n <script>` + `mise run check` + clippy (when no Rust changes)
- **CI/Audit/Perf/Semver/CRAP gate reviews**: see `gate-reviews.md`
- **Binding propagation (napi/wasm/ffi/jni/ruby/dotnet/kotlin/uniffi)**: see `binding-reviews.md`
- Cross-platform CI: bash syntax needs `shell: bash` if matrix includes Windows

## Codex Review Integration

- Codex findings are advisory — cross-reference with your own analysis. Use `--commit HEAD` (verify
    with `git log` first)
- Go codec findings: dismiss — Go mirrors Rust reference faithfully
- `.NET version` findings: `dotnet-version: '8.0'` is valid for `actions/setup-dotnet@v4` (resolves
    to latest 8.0.x). Dismiss "use `8.0.x`" suggestions
- Confused by large generated Kotlin/Swift diffs — those findings are advisory
