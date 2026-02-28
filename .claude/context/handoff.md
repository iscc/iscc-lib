## 2026-02-28 — Remove WASM vestiges from Go CI, README, and docs

**Done:** Removed all WASM/wazero references from three Go-related files. The CI Go job now has 4
steps (checkout, setup-go, test, vet) with zero Rust dependencies. The README describes the pure Go
architecture with direct function call examples. The howto guide uses `iscc.Function(...)` pattern
throughout with correct return types (typed result structs, not strings).

**Files changed:**

- `.github/workflows/ci.yml`: Removed `dtolnay/rust-toolchain` (with `wasm32-wasip1` target),
    `Swatinem/rust-cache`, "Build WASM module", and "Copy WASM to Go package" steps from the Go job.
    Go job reduced from 6 steps to 4.
- `packages/go/README.md`: Rewrote description (pure Go, no binary artifacts), Quick Start (direct
    `iscc.GenMetaCodeV0` call, no Runtime/context), API tables (package-level functions returning
    typed structs), Streaming section (`Push` → `Finalize`, no `Close`), Architecture section
    (native compiled code, no runtime overhead).
- `docs/howto/go.md`: Removed "Runtime setup" section. Updated all code examples from
    `rt.Method(ctx, ...)` to `iscc.Function(...)`. Updated return types from `(string, error)` to
    typed result structs (`*MetaCodeResult`, etc.) with `.Iscc` field access. Updated streaming from
    `Update`/`Close` to `Push`/`Finalize`. Text utilities now show direct return values (no error).
    Error handling section no longer mentions WASM runtime failures.

**Verification:** All 6 verification criteria pass:

- No `wasm32-wasip1` in CI ✓
- No `wazero`/`wasm` in README ✓
- No `wazero`/`wasm`/`Runtime`/`NewRuntime` in howto ✓
- `iscc.GenMetaCodeV0` pattern in howto ✓
- `iscc.GenMetaCodeV0` pattern in README ✓
- `mise run check` — all 14 hooks pass ✓

**Next:** The Go pure rewrite is fully complete and documented. Suggested next priorities:

1. PR from develop → main — the Go pure rewrite is a major milestone worth merging
2. Benchmark CI integration
3. Publishing infrastructure (OIDC, npm, Maven Central)

**Notes:** No surprises. The `GenIsccCodeV0` signature takes a `wide bool` parameter that was
missing from the old howto examples — added it in the ISCC-CODE section. The mdformat hook adjusted
some table column widths and line wrapping — cosmetic only.
