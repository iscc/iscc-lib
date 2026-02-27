# Next Work Package

## Step: Commit Go WASM binary for go get distribution

## Goal

Make the Go module distributable via `go get` by unignoring and committing the release WASM binary.
Currently `packages/go/iscc_ffi.wasm` is gitignored, which means Go consumers who `go get` the
module get no WASM binary — the `//go:embed iscc_ffi.wasm` directive fails at compile time. This
blocks two target verification criteria: "Package installs cleanly via `go get`" and "Embedded
`.wasm` binary is up to date with the Rust core."

## Scope

- **Create**: (none)
- **Modify**: `.gitignore` (remove `packages/go/*.wasm` line)
- **Reference**: `packages/go/iscc.go` (line 30: `//go:embed iscc_ffi.wasm`),
    `.github/workflows/ci.yml` (Go job builds WASM for testing)

## Not In Scope

- Changing the CI Go job — it currently builds a fresh debug WASM binary for testing, which is
    correct behavior (tests current code). Leave it as-is.
- Adding a `mise run go:build-wasm` task — useful but separate step
- Adding a CI step to verify the committed binary matches current FFI code — fragile across compiler
    versions and debug/release modes
- Updating `specs/ci-cd.md` release protocol to include WASM rebuild — can be done in a future step
    when the release process is formalized
- Re-triggering any release workflows
- Deleting resolved issues from `issues.md` (review agent's job)

## Implementation Notes

1. **Edit `.gitignore`**: Remove the line `packages/go/*.wasm` (line 232 in root `.gitignore`).

2. **Build the release WASM binary**: Run `cargo build -p iscc-ffi --target wasm32-wasip1 --release`
    (the `wasm32-wasip1` target is already installed in the devcontainer). The release profile
    (`lto = true`, `codegen-units = 1`, `strip = true`) produces a ~683KB binary vs ~11MB for
    debug.

3. **Copy to Go package**: `cp target/wasm32-wasip1/release/iscc_ffi.wasm packages/go/`

4. **Verify Go tests pass** with the release binary:
    `cd packages/go && CGO_ENABLED=0 go test -count=1 ./...`

5. **Stage both files**: `git add .gitignore packages/go/iscc_ffi.wasm`

The binary is already present locally (built during CI testing). The release binary is 683KB — well
within reason for a committed asset in a Go module. Many Go projects with embedded WASM (via wazero)
commit binaries of similar or larger size. This is the standard distribution pattern for Go modules
that embed binary assets.

## Verification

- `grep -c 'packages/go/\*.wasm' .gitignore` returns `0` (line removed)
- `git ls-files packages/go/iscc_ffi.wasm` returns `packages/go/iscc_ffi.wasm` (file is tracked)
- `file packages/go/iscc_ffi.wasm` contains `WebAssembly`
- `ls -la packages/go/iscc_ffi.wasm | awk '{print $5}'` returns a number less than 1000000 (release
    build, not debug)
- `cd packages/go && CGO_ENABLED=0 go test -count=1 ./...` passes (46 tests)

## Done When

All verification criteria pass: the WASM binary is tracked in git, is a release build under 1MB, and
Go tests pass with it.
