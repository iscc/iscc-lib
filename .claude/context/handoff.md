## 2026-02-25 — Add Go CI job to ci.yml

**Done:** Added a `go:` job to `.github/workflows/ci.yml` that builds the WASM module from Rust,
copies it to the Go package directory, and runs `go test` and `go vet`. The job follows the
established CI pattern (checkout → Rust toolchain → cache → language setup → build → test) and is
placed after the `java:` job at the end of the `jobs:` section.

**Files changed:**

- `.github/workflows/ci.yml`: Added `go:` job block with 8 steps (checkout, rust-toolchain with
    wasm32-wasip1 target, rust-cache, setup-go with go-version-file, cargo build WASM, cp WASM to
    packages/go, go test with CGO_ENABLED=0 and -count=1, go vet)

**Verification:** All four criteria from next.md pass:

- `git diff` shows exactly one new job block (`go:`) with no changes to existing jobs
- YAML validation (`yaml.safe_load`) exits 0
- All 8 required steps verified present in the Go job section
- `mise run check` passes — all 14 pre-commit hooks clean

**Next:** The Go bindings are now complete with CI coverage (14 tests, 46 conformance vectors).
Consider adding the remaining 12 Tier 1 Go function wrappers (text utils, algo primitives,
streaming), or creating `packages/go/README.md` and updating root README with Go installation
instructions.

**Notes:** No surprises. This was a straightforward YAML addition following the exact spec in
next.md. The `go-version-file` approach means the Go version in CI is controlled by `go.mod`
(currently 1.24.0) — no hardcoded version in the workflow.
