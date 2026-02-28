# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- CI job additions are small, single-file changes that provide high value. Pattern: copy existing
    job structure, swap language-specific setup action and build/test commands
- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- README files are "create" operations — less risky than code changes. Doc files excluded from
    3-file limit
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- Generic API optimization: prefer generic bounds (`AsRef<T> + Ord`) over concrete type changes to
    avoid cascading modifications across binding crates
- File deletions don't count toward the 3-file modification limit — they are simpler than edits
- Small utility functions that compose existing helpers are ideal single-step scopes (e.g.,
    `JsonToDataUrl` = 3 existing unexported helpers composed into 1 public function)
- After a major rewrite (e.g., Go pure rewrite), docs/CI lag behind — schedule a cleanup step to
    bring all stale references in sync before moving to the next feature

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Go Package — Key Facts

- **30/30 Tier 1 symbols**: All pure Go, zero WASM dependencies
- Pure Go API: package-level functions like `iscc.GenMetaCodeV0(name, desc, meta, bits)` — no
    Runtime, no context.Context
- Hashers: `NewDataHasher()` → `Push(data)` → `Finalize(bits)`. No Close, no context
- Text utilities return plain types (no error): `TextClean` → `string`, `TextRemoveNewlines` →
    `string`, `TextTrim` → `string`, `TextCollapse` → `string`
- Return types are structs: `*MetaCodeResult`, `*TextCodeResult`, etc. with `.Iscc` field
- `EncodeBase64` returns `string` (no error)
- `SlidingWindow` and `AlgSimhash` return `(Type, error)`
- Module deps: `github.com/zeebo/blake3`, `golang.org/x/text` (+ cpuid indirect)
- `TestPureGo*` test prefix is historical vestige — cosmetic cleanup only

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)

## Post-All-Bindings-Complete Planning

All 7 bindings at 30/30, CI green. PR #10 exists from develop→main (passing, mergeable). Remaining
gaps in priority order:

1. Benchmark CI integration (compile-only job) — prevents bitrot
2. PR #10 update (title/description reflect full milestone) — prep for human merge
3. Publishing infrastructure (OIDC, npm, Maven Central) — mostly human tasks
4. Benchmark results in docs (speedup factors) — requires running benchmarks and documenting

When the handoff says "create PR" but a PR already exists, check with `gh pr list` before scoping a
create-PR step. The real work may be updating the existing PR or something else entirely.

## Recurring Patterns

- When new symbols are added to bindings but docs predate them, batch howto guide + README update
    into one step (2 files). Verification is grep-based
- Interactive sessions can break CI (e.g., Python ruff format) — always check state.md CI section
- After PR merge, always switch back to develop: `git checkout develop`
- After major architecture changes (e.g., WASM→pure Go), multiple files go stale simultaneously (CI,
    README, howto docs, test comments) — group the most impactful ones in a single step

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- ISCC Foundation URL is `https://iscc.io`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Handoff review agent incorrectly claimed 30/30 after WASM removal — state assessment caught that
    `JsonToDataUrl` was only in the deleted WASM bridge. Always verify state.md over handoff claims
