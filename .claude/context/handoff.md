## 2026-02-25 — Create Go package README

**Done:** Created `packages/go/README.md` for the Go module proxy audience, following the
established per-crate README structure. Includes title, badges, experimental notice, tagline
emphasizing pure Go / no cgo, the shared "What is ISCC" section, installation via `go get`, quick
start with `NewRuntime`/`GenMetaCodeV0`/`Close`, API table for all 9 gen functions, utilities
listing, architecture note about wazero, links, and license.

**Files changed:**

- `packages/go/README.md`: Created (new file, ~100 lines)

**Verification:** All 6 grep checks pass. `mise run check` passes clean (all 14 hooks including
mdformat). No formatting changes needed — file was clean on first write.

**Next:** The Go bindings package is now documented. Suggested next steps:

1. **Remaining 12 Go wrappers** — text utilities, algorithm primitives, streaming hashers
2. **Root README Go section** — add Go installation/quick-start alongside existing languages
3. **Documentation** — `docs/howto/go.md` how-to guide
4. **Go badge in root README** — once a Go module proxy badge URL exists

**Notes:** No surprises. The README follows the same 6-section pattern as all other per-crate
READMEs (What is ISCC, Installation, Quick Start, API Overview, Links, License) with an additional
Architecture section specific to the Go/WASM approach. The architecture section highlights the key
differentiator (pure Go, no cgo, cross-compilation friendly) that Go developers care about. Used em
dash (—) in the tagline matching the Rust README style; mdformat accepted it without changes.
