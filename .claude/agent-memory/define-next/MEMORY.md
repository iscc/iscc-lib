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
- File deletions don't count toward the 3-file modification limit — they are simpler than edits
- After a major rewrite (e.g., Go pure rewrite), docs/CI lag behind — schedule a cleanup step to
    bring all stale references in sync before moving to the next feature
- State assessments can go stale — always verify claimed gaps by reading the actual files. The state
    may say "met" for something that still has stale content (e.g., wazero references in docs after
    Go rewrite)
- When state says "all automatable work complete," grep for known stale terms (e.g., old tech names)
    across docs — state assessment may miss content-level inaccuracies

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
- Return types are structs: `*MetaCodeResult`, `*TextCodeResult`, etc. with `.Iscc` field
- Go module structure: 36 .go files (18 source + 18 test), go.mod, go.sum, README.md
- Go tests load conformance vectors from `../../crates/iscc-lib/tests/data.json` (relative path)

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- `scripts/version_sync.py` uses only stdlib (json, re, sys, pathlib) — can run as
    `python scripts/version_sync.py --check` without uv. Spec says "(run in CI)" but wasn't added

## Project Near-Completion State (Iteration 16)

All 7 bindings at 30/30, CI green with 9 jobs. PR #10 exists from develop→main.

**Remaining automated gaps (in priority order):**

1. ~~Benchmark CI integration~~ — DONE (iteration 10)
2. ~~Benchmark results in docs~~ — DONE
3. ~~LLM docs coverage~~ — DONE (iteration 11)
4. ~~Tabbed multi-language code examples~~ — DONE (iteration 12)
5. ~~Fix stale Go example on landing page~~ — DONE (iteration 13)
6. ~~Add version:check to CI~~ — DONE (iteration 14)
7. ~~Stale spec cleanup (ci-cd.md)~~ — DONE (iteration 15)
8. Fix stale wazero/WASM refs in docs — SCOPED (iteration 16)
9. ci-cd.md standard action set gap (missing Go/Java actions) — cosmetic, future step
10. Tab order standardization — LOW priority, needs human review
11. Publishing infrastructure (OIDC, npm, Maven Central) — human tasks
12. PR #10 merge — human task

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- ISCC Foundation URL is `https://iscc.io`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them
