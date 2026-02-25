# Next Work Package

## Step: Add Go CI job to ci.yml

## Goal

Add a `Go (go test, go vet)` job to `.github/workflows/ci.yml` so the 14 existing Go tests
(including 9 conformance tests covering all 46 vectors) run automatically on push/PR — protecting
against regressions as Go bindings grow.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add one new job block
- **Reference**: `packages/go/iscc_test.go` (build instructions in line 4-5 comments),
    `packages/go/go.mod` (Go version and dependencies)

## Not In Scope

- Adding the remaining 12 Tier 1 Go function wrappers (text utils, algo primitives, streaming)
- Creating `packages/go/README.md`
- Updating the root README with Go installation/quick-start sections
- Adding Go to the docs site navigation or howto guides
- Using `mise` in CI (learnings say: call tools directly)
- Running `go fmt` or `staticcheck` — keep it simple like the existing CI jobs

## Implementation Notes

The job follows the established CI pattern: checkout → Rust toolchain → Rust cache → language setup
→ build → test. The Go-specific twist is that the WASM binary must be built first.

**Job structure:**

```yaml
go:
  name: Go (go test, go vet)
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-wasip1
    - uses: Swatinem/rust-cache@v2
    - uses: actions/setup-go@v5
      with:
        go-version-file: packages/go/go.mod
    - name: Build WASM module
      run: cargo build -p iscc-ffi --target wasm32-wasip1
    - name: Copy WASM to Go package
      run: cp target/wasm32-wasip1/debug/iscc_ffi.wasm packages/go/
    - name: Run Go tests
      run: CGO_ENABLED=0 go test -v -count=1 ./...
      working-directory: packages/go
    - name: Run Go vet
      run: go vet ./...
      working-directory: packages/go
```

**Key details:**

- Use `actions/setup-go@v5` (latest) with `go-version-file` pointing to `packages/go/go.mod` so the
    Go version is controlled by the module file (currently 1.24.0), not hardcoded in CI.
- The `wasm32-wasip1` target is added to the Rust toolchain in the setup step via `targets:` (not a
    separate `rustup target add` step).
- The WASM binary is built in debug mode (matching the local dev workflow) — release mode would be
    slower and unnecessary for CI testing.
- `CGO_ENABLED=0` confirms pure-Go operation (no cgo dependency on the CI runner).
- `-count=1` prevents test caching so conformance tests always re-run.
- Place the new job at the end of the `jobs:` section, after the `java:` job.

## Verification

- `git diff .github/workflows/ci.yml` shows exactly one new job block (`go:`) with no changes to
    existing jobs
- The YAML is valid: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`
    exits 0
- The new job includes steps for: checkout, rust-toolchain with wasm32-wasip1, rust-cache, setup-go,
    cargo build, cp wasm, go test, go vet
- `mise run check` passes (pre-commit hooks clean)

## Done When

All four verification criteria pass — the Go CI job is defined in `ci.yml` with the correct build
chain (Rust WASM → copy → Go test) and no existing jobs are modified.
