## 2026-11-21 — The `go` CI job runs the Go suite a second time as a 32-bit (`GOARCH=386`) build

**Done:** Added one step to the `go` job in `ci.yml`, after "Run Go tests" and in `packages/go`:
`Run Go tests (32-bit, GOARCH=386)` runs `GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` and then
`GOARCH=386 go vet ./...` as a `run: |` block. There is no `-v` and no `-race`. Updated the `go` row
and the "Go job runs …" bullet in `ci-cd.md`, and added the 386 test and vet commands to the Build
Commands in `packages/go/CLAUDE.md`.

**Files changed:**

- `.github/workflows/ci.yml`: added the 386 step to the `go` job (no other job touched)
- `.claude/context/specs/ci-cd.md`: the `go` job-table row (same key) and the Go bullet
- `packages/go/CLAUDE.md`: 386 test and vet commands under Build Commands

**Verification:**

- `cd packages/go && GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` → `ok` (rc=0).
    `GOARCH=386 go vet ./...` → rc=0
- The next.md yaml assertion (run with `uv run python`, because system python3 has no PyYAML) → exit
    0
- `grep -c GOARCH=386` → 2 in `ci-cd.md`, 2 in `packages/go/CLAUDE.md`
- `mise run check` → rc=0, and it modified no file
- Throwaway proof that the step can see the defect, run in a detached worktree under `/tmp` (not in
    the checkout). I put back the signed subtraction from before 263
    (`offset := int(minSize) + int((minSize+1)/2)`, `centerSize := int(avgSize) - offset`) and
    returned `centerSize` without `clampInt`:
    - 386: `--- FAIL: TestCdcParamsLargeAvgMatchesReference` with
        `center_size: got -1610612736, want 2684354560` for avg 4294967295, package FAIL
    - amd64: `ok`. This confirms that today's amd64-only CI cannot see the defect
    - Cleanup: `git worktree remove --force` and `rm -rf`. `git worktree list` now shows only the
        checkout, and `git diff --stat packages/go/*.go` is empty

**Next:** `@iscc/wasm` non-number coercion (`null` → 0). define-next named it the natural next step;
it needs `JsValue` params across `crates/iscc-wasm/src/lib.rs`.

**Notes:**

- yamlfix wrapped a single-line `cmd && cmd` run in the middle of `./...`, so I used a block scalar.
    Actions runs `run:` with `bash -e`, so a failing `go test` stops the step before vet
- The `FAILED: encode_seq…` lines in the reverted 386 output come from the conformance runner's own
    negative self-tests. They also print on green runs, so grep `--- FAIL` to find the real failure
- The 386 step has not run in CI yet. The first push shows whether the runner behaves like the local
    amd64 host (static 386 binary on an x86-64 kernel)
