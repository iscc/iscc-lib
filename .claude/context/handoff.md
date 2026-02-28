## 2026-02-28 — Fix Go representation in architecture diagram

**Done:** Updated the Mermaid diagram in `docs/architecture.md` to show Go as a standalone
disconnected node with a green style, rather than connected to CORE. Updated the explanatory text to
distinguish the five binding crates from the standalone Go reimplementation. Removed all 5 vestigial
"do NOT require the WASM binary" comments from Go test files.

**Files changed:**

- `docs/architecture.md`: Mermaid diagram — Go node disconnected from CORE with green style; text
    below diagram rewritten to distinguish binding crates from standalone Go module
- `packages/go/minhash_test.go`: Removed WASM binary comment (line 2)
- `packages/go/utils_test.go`: Removed WASM binary comment (line 2)
- `packages/go/codec_test.go`: Removed WASM binary comment (line 2)
- `packages/go/cdc_test.go`: Removed WASM binary comment (line 2)
- `packages/go/simhash_test.go`: Removed WASM binary comment (line 2)

**Verification:**

- `GO.*-->.*CORE` in architecture.md: 0 matches ✓
- `standalone reimplementation` in architecture.md: found at line 31 ✓
- `All binding crates are thin wrappers` in architecture.md: 0 matches ✓
- `WASM binary` in packages/go/: 0 matches ✓
- `uv run zensical build`: succeeded (0.57s) ✓
- `go test ./... && go vet ./...`: passed (0.064s) ✓
- `mise run check`: all 14 hooks pass ✓

**Next:** The project is in maintenance mode with all functional requirements met. Remaining work is
human-dependent: merge PR #10 (develop → main), trigger 0.0.2 releases, configure Maven Central
publishing. The only remaining automated low-priority task is ci-cd.md standard action set gap
(missing Go/Java CI actions).

**Notes:** Straightforward documentation and comment cleanup. No surprises. The Mermaid `style`
directive with green fill/stroke visually distinguishes Go from the Rust-dependent binding crates.
