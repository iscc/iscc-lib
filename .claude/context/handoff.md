## 2026-11-21 — Review of: The `go` CI job runs the Go suite a second time as a 32-bit (`GOARCH=386`) build

**Verdict:** PASS

**Summary:** The `go` job in `ci.yml` has one more step that runs the Go suite and `go vet` as a
`GOARCH=386` build, and `ci-cd.md` and `packages/go/CLAUDE.md` say so. The diff touches exactly the
three files next.md named, no Go source and no other job. The step ran green on a GitHub runner, and
a reverted 32-bit fix fails it.

**Verification:**

- [x] `cd packages/go && GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` exits 0 — `ok`, rc=0;
    `GOARCH=386 go vet ./...` rc=0
- [x] The next.md YAML assertion on the `go` job exits 0 — run with `uv run python` (system
    `python3` has no PyYAML)
- [x] `.claude/context/specs/ci-cd.md` and `packages/go/CLAUDE.md` each contain `GOARCH=386` — 2
    lines each
- [x] `mise run check` exits 0 — all hooks passed, no file modified

Beyond next.md's list (3 probes):

- **The step works on CI.** I pushed the advance commits first. In run 44925892273 (`cea653f`) the
    `Go (go test, go vet)` job is green on `ubuntu-26.04` with Go 1.26.11, and the step
    `Run Go tests (32-bit, GOARCH=386)` printed `ok github.com/iscc/iscc-lib/packages/go 0.072s`
- **Both builds run the same tests.** In a `/tmp` copy, `go test -v` runs 560 tests on amd64 and on
    386 with 0 failures. The 386 build skips one, `TestCdcParamsLargeAvgExactOn64Bit`, on purpose.
    The test binary is `ELF 32-bit LSB executable, Intel 80386, statically linked`
- **The step sees the defect it guards.** In the same copy I put back the signed subtraction in
    `algCdcParams` and dropped `clampInt`. 386 fails with
    `--- FAIL: TestCdcParamsLargeAvgMatchesReference … center_size: got -1610612736, want 2684354560`;
    amd64 stays `ok`. This matches the advance agent's claim

**Issues found:**

- (none filed)
- Nit, not filed: `GOARCH=386 go vet` passes on the reverted code, so the vet line adds only
    type-checking for the 32-bit word size. It costs about 9 s together with the 386 test build

**Codex review:** "No actionable regressions found." It ran the Go tests and vet on both
architectures and the workflow and docs consistency checks.

**Next:** `@iscc/wasm` converts non-number values to numbers instead of rejecting them (`normal`).
It is the remaining correctness item that ordinary JavaScript callers can hit. The issue allows two
resolutions: reject a non-number on each numeric parameter of `crates/iscc-wasm/src/lib.rs`, pinned
by a Node test on a built package, or state the conversion in `docs/howto/wasm.md`. define-next
should pick one and say why. Smaller ready items if that step turns out too large for one iteration:
the Ruby 3.3 floor, the FFI padding removal (C++ `safe_data`, .NET `PinnedRows`), the Ruby Float
docs fix, the pkg-config version line.

**Notes:**

- Issue "No CI job runs the Go port as a 32-bit build" is resolved and deleted. `state.md` still
    lists it (lines 93, 161, 175); update-state will pick that up
- I shifted the `packages/go/CLAUDE.md` line references in the issue "Specs and crate CLAUDE.md
    files describe shipped work as pending" by 4, because this step added 4 lines above them
- The 386 step runs without `-v`, so the CI log shows only the `ok` line. To see which test fails
    after a red run, read the `--- FAIL` lines; the `FAILED: encode_seq…` lines come from the
    conformance runner's own negative self-tests and print on green runs too
- The Go tests read `../../crates/iscc-lib/tests/data.json`. A copy of `packages/go` alone fails ten
    conformance tests for that reason, not because of the code under test
- No decisions.md entry: the 32-bit coverage choice was already recorded on 2026-11-21
