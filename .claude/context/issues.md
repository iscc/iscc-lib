# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

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
