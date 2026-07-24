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
- **Backend-activation claims — verify the build.rs gating (iters 117-118, #42, RESOLVED)**: don't
    trust "flag set → goal met". blake3 WASM SIMD is activated by the `blake3/wasm32_simd` **Cargo
    feature** (`CARGO_FEATURE_WASM32_SIMD` → `blake3_wasm32_simd` cfg → `Platform::WASM32_SIMD`),
    NOT `-C target-feature=+simd128` alone. Honest wiring proof:
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` shows
    `wasm32_simd`. Counting `v128` opcodes ALONE is a FALSE-POSITIVE (LLVM auto-vectorizes the
    portable path). NUANCE (iter 118): the global RUSTFLAGS is NOT required to *compile* the backend
    — blake3's SIMD fns carry `#[target_feature(enable = "simd128")]`, so a no-flag
    `cargo build --target wasm32-unknown-unknown` succeeds; the flag broadens simd128 to the whole
    crate. Don't let docs claim "flag needed so intrinsics compile" — test a no-flag wasm build to
    check such claims
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples
- **Docs site URL**: `https://lib.iscc.codes/` NOT `https://iscc-lib.iscc.io/`. Advance agents
    consistently get this wrong — always verify
- **next.md relative path errors**: verify relative paths with `realpath` during review
- **CI find cross-arch bug**: when multiple targets extract to same CWD with same lib name, `find`
    patterns must include target name (generic wildcards match all extracted dirs)
- **Advance agent idle claims**: always verify remaining issue priorities independently — they may
    claim "only low-priority remain" when `normal` issues still exist
- **Decode body-length must be EXACT, not `>= nbytes` (iter 119 found; Go iter 120, Rust core iter
    121 — RESOLVED)**: a `len(tail) < nbytes` guard silently drops trailing base32 bytes, so `...AB`
    aliases `...ABAA` — codec-wide, NOT ID-specific. BOTH Go `IsccDecode` and Rust core
    `iscc_decode` now reject too-long via a 2-branch "too short"/"too long" form (keeps the "too
    short" message/test intact); all 11 bindings inherit the Rust fix. Classified non-breaking for
    the Tier 1 symbol — rationale in `decisions.md` 2026-07-24. Composite `iscc_decompose`
    legitimately consumes trailing units — do NOT harden it. When reviewing new decode/parse
    surface, probe exact-length rejection with a throwaway test
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
- **Rust dep-refresh (`cargo update`, no `-p`) — iter 124, slice 1 of v0.6.0 dep issue**: verify
    `git diff --name-only -- Cargo.toml` is EMPTY (pins untouched: uniffi 0.31/pyo3 0.29/criterion
    0.5/iai-callgrind 0.16/magnus 0.7/jni 0.21/napi 3), then run the full 4-gate set
    `mise run test`+`lint`+`audit`+`bench:iai:check` (not the lockfile-only shortcut — a
    `cargo   update` can shift Ir + pull new transitive licenses). `cargo-deny` is the main risk
    (passed clean). Perf-gate tools NOT in fresh container — install per learnings.md Tooling note
    first. GOTCHA: next.md's pin grep `'uniffi = { version = "0.31"'` (inline-table) FAILS —
    manifest uses plain-string `uniffi = "0.31"`; that's a next.md defect, not a miss (verify the
    actual pin form)
- **Version sync addition**: `mise run check` + `uv run scripts/version_sync.py --check` + clippy
- **Script-only (shell)**: `bash -n <script>` + `mise run check` + clippy (when no Rust changes)
- **release.yml-only (iter 123, #49)**: NOT exercised by CID pushes (only
    `workflow_dispatch`+registry) → static-verify only: YAML parse + matrix-entry presence +
    artifact-name consistency across the build→test→publish chain. Adding a wheel target =
    build+test matrix only; `publish-pypi` collects via `pattern: wheels-*`+`merge-multiple`.
    `test-*` jobs still lack the `!cancelled()&&!failure()` guard (tracked single-registry
    re-trigger bug) — don't flag as new
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
- **Trust Codex on dependency-internals findings (iters 117-118)**: it correctly caught (117) that
    blake3's `wasm32_simd` backend needs the Cargo feature, then (118, P3) that blake3's SIMD fns
    carry `#[target_feature(enable = "simd128")]` so the global RUSTFLAGS is NOT needed to compile
    the backend — both verified against blake3 source + a no-flag wasm build. When Codex cites a
    dep's build.rs / `#[target_feature]` / feature gating, check the dep source (and build it)
    before dismissing
- **Codex is strong on input-validation edge cases (iter 119)**: on new decode/parse code it caught
    a real trailing-byte acceptance gap in `DecodeIsccID`/`IsccDecode` the local tests missed. When
    Codex flags "accepts malformed input / trailing data", probe it empirically before dismissing —
    but characterize regression-vs-pre-existing (here it was a pre-existing codec-wide gap)
