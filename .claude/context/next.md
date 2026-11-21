# Next Work Package

## Step: The `go` CI job runs the Go suite a second time as a 32-bit (`GOARCH=386`) build

## Goal

Close the issue "No CI job runs the Go port as a 32-bit build" (decided 2026-11-21, decisions.md
"32-bit coverage: a Go `GOARCH=386` CI step, no native 32-bit Rust leg"). Today the 32-bit handling
in `algCdcParams` (iteration 263) and the canonical `meta` writer (266) are untested in CI: CI runs
amd64 only, where reverting the 32-bit fix fails no test.

## Alternatives Considered

- **Chosen:** Go `GOARCH=386` CI step (handoff "Next") — decided by the human, correctness coverage
    for shipped Go code, and the suite already passes on 386 locally, so the step is small and lands
    green. It was rejected once as "one line"; it is now the oldest ready correctness item.
- **Rejected:** `@iscc/wasm` non-number coercion (`null` → 0) — a real user-facing defect, but a
    code fix means taking every numeric parameter as `JsValue` across `crates/iscc-wasm/src/lib.rs`
    (~15 call sites plus the `Vec<f64>` element path), a far bigger step; it is the natural next
    one.
- Also open and ready: Ruby 3.3 floor (touches `ci.yml` + `release.yml` too — keep it separate so a
    red run is attributable), FFI padding removal, JNI loader fallback, docs/spec issues.

## Scope

- **Modify**: `.github/workflows/ci.yml` (the `go` job only)
- **Modify (docs)**: `.claude/context/specs/ci-cd.md` (the `go` row of the job table and the "Go job
    runs …" bullet), `packages/go/CLAUDE.md` (Build Commands: add the 386 command)
- **Reference**: `packages/go/cdc.go` (`algCdcParams`, `clampInt`), the decisions.md entry named in
    Goal

## Not In Scope

- A native 32-bit Rust leg (`i686` cargo test) — explicitly rejected by the decision
- A job matrix over `GOARCH` or a second job key — one extra step in the existing `go` job suffices
    and keeps the `ci-cd.md` job table at its current keys
- Any change to Go source or tests; if the 386 run fails in CI, report it, do not patch around it
- Windows CI (`low`, untriaged) and the Ruby floor change in the same `ci.yml`

## Implementation Notes

- Add after "Run Go tests", in the same job and `working-directory: packages/go`:
    `name: Run Go tests (32-bit, GOARCH=386)`,
    `run: GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...`. Leave `-v` off so the second run does
    not double the log. Optionally also `GOARCH=386 go vet ./...` in the same `run:` (vet
    type-checks for the target word size).
- Why it runs on an amd64 runner: with `CGO_ENABLED=0` Go builds a static 386 test binary and the
    x86-64 Linux kernel executes i386 binaries natively (no multilib needed). Do not add `-race`
    (unsupported on 386).
- Prove the step can see the defect it guards (throwaway check, not committed): temporarily restore
    the signed form of the `algCdcParams` subtraction (e.g. `int(avgSize) - …` without the unsigned
    subtract/`clampInt`), run `GOARCH=386 CGO_ENABLED=0 go test ./...` and confirm a failure, then
    restore the file and confirm `git diff --stat packages/go` is empty. If no test fails on the
    reverted form, say so in the handoff instead of adding a test — that would be a separate issue.
- YAML formatting: `yamlfix` runs in prek; run `mise run format` before committing.

## Verification

- `cd packages/go && GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` exits 0
- `python3 -c "import yaml;s=yaml.safe_load(open('.github/workflows/ci.yml'))['jobs']['go']['steps'];assert any('GOARCH=386' in (x.get('run') or '') and x.get('working-directory')=='packages/go' for x in s)"`
    exits 0
- `.claude/context/specs/ci-cd.md` and `packages/go/CLAUDE.md` each contain `GOARCH=386`
- `mise run check` exits 0

## Done When

The `go` job in `ci.yml` runs the Go suite once more with `GOARCH=386`, the docs say so, and all
verification criteria pass.
