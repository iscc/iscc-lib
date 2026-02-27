# Issues

Tracked issues and feature requests for the CID workflow.

## Purpose

This file is a lightweight, append-only channel for humans and agents to flag problems that
define-next can prioritize. It bridges the gap between ad-hoc observations and the formal
state→target planning loop.

## Format

Each issue is an H2 heading with a priority tag, description, and source attribution:

```markdown
## [priority] Issue title

Description of the problem, including context and any relevant file paths or error messages.

**Source:** [human|review|advance]
**Spec:** .claude/context/target.md#section-name (optional — only if rooted in a spec gap)
**Upstream:** iscc/iscc-core (optional — only if the fix belongs in the upstream reference)
```

### Spec field

The optional `**Spec:**` field links an issue to a specific section in target.md or a sub-spec file
that needs updating. Source determines what happens:

- **`[human]` + `Spec:`** — the review agent updates the referenced spec directly when resolving the
    issue. The human created the issue, so the spec change is implicitly authorized.
- **`[review]`/`[advance]` + `Spec:`** — the review agent flags with `HUMAN REVIEW REQUESTED` and
    describes the proposed spec change in the issue description. It does NOT modify target.md. The
    human must approve and make the change.

### Upstream field

The optional `**Upstream:**` field marks an issue as belonging to an external repository (e.g.,
`iscc/iscc-core`). Upstream issues are always human-gated — filing on GitHub is a visible public
action that requires review regardless of source:

- Any issue with `**Upstream:**` triggers `HUMAN REVIEW REQUESTED` in the CID loop
- The issue description should include concrete evidence: failing conformance vectors, expected vs
    actual output, specific code references in the upstream repo
- The issue stays in issues.md as a draft until the human reviews it and either asks an interactive
    session to file it via `gh issue create -R <repo>` or files it manually
- After filing, update the issue description with the GitHub issue URL and delete it from issues.md

## Priority Levels

- **critical** — must be addressed in the next iteration; overrides normal gap analysis
- **normal** — considered alongside state→target gaps; define-next weighs it against other work
- **low** — pick up when no higher-priority work remains

## Management Rules

- **Append only** — add new issues at the end of this file
- **Resolution** — the review agent deletes resolved issues after verifying the fix (history lives
    in git)
- **Source tags** — agents that add issues must include a source tag: `[human]`, `[review]`, or
    `[advance]`
- **Scope** — track any problems that affect correctness, architecture, maintainability, or
    performance; not style preferences or minor nits

<!-- Add issues below this line -->

## [critical] Rewrite Go bindings as pure Go (replace WASM/wazero bridge)

The current Go bindings use a WASM/wazero bridge: Rust core compiled to `wasm32-wasip1`, embedded
via `//go:embed`, executed through the wazero runtime. This requires committing a ~700KB binary
artifact to git (`packages/go/iscc_ffi.wasm`), polluting repository history and weakening the
`check-added-large-files` guard (raised from 500KB to 1024KB globally to accommodate it).

**Decision:** Replace with a pure Go implementation of all ISCC algorithms. The Rust core is ~6,300
lines — all pure algorithms (CDC, MinHash, SimHash, DCT, WTA-Hash, codec) that translate directly to
Go. Well-maintained Go libraries exist for all external dependencies (BLAKE3, xxHash, Unicode
normalization).

**Migration steps:**

1. **Remove WASM artifacts and infrastructure:**

    - Delete `packages/go/iscc_ffi.wasm` from git tracking
    - Re-add `packages/go/*.wasm` to `.gitignore`
    - Restore `check-added-large-files` threshold to 256KB (`args: [--maxkb=256]`)
    - Remove `wazero` dependency from `packages/go/go.mod`
    - Remove WASM build/copy steps from CI (`ci.yml` Go job)

2. **Implement pure Go modules** (port from `crates/iscc-lib/src/`):

    - `codec.go` — header encode/decode, base32, component encoding, ISCC decomposition (~1,500 lines
        Rust)
    - `utils.go` — text normalization (NFKC/NFD), cleaning, trimming, collapsing (~280 lines Rust)
    - `cdc.go` — Content-Defined Chunking with gear rolling hash (~300 lines Rust)
    - `minhash.go` — 64-dimensional MinHash with universal hash functions (~260 lines Rust)
    - `simhash.go` — SimHash bit-vote aggregation, sliding window n-grams (~330 lines Rust)
    - `dct.go` — Nayuki fast recursive DCT for image hashing (~150 lines Rust)
    - `wtahash.go` — Winner-Take-All hash for video fingerprinting (~390 lines Rust)
    - `iscc.go` — all 9 `gen_*_v0` functions + streaming hashers + public API

3. **Preserve the existing public API surface** — all 30 Tier 1 symbols, same function signatures,
    same Go naming conventions (`GenMetaCodeV0`, `DataHasher`, etc.). This is a backend
    replacement, not an API change.

4. **Validate against conformance vectors** — the existing `iscc_test.go` (1,353 lines, 46 test
    functions) should pass with minimal changes (remove `Runtime` struct ceremony, call
    package-level functions directly).

5. **Simplify CI** — Go job no longer needs `wasm32-wasip1` target or WASM build step. Just
    `go test ./...` and `go vet ./...`.

**Benefits:**

- No binary artifacts in git
- Native performance (no WASM interpreter overhead)
- First-class Go developer experience (debuggable, readable source)
- Simpler CI (no cross-compilation target for WASM)
- `check-added-large-files` tightened to 256KB (sensible guard for a source-only repo)

**Spec:** .claude/context/target.md#go-bindings--go-module

**Source:** [human]
