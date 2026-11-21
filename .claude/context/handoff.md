## 2026-11-21 — Review of: One section runner for `conformance_selftest` in Rust and Go

**Verdict:** PASS

**Summary:** Both languages now run all 11 conformance sections through one generic section runner
(`run_section` + `report_case` in Rust, `runSection` + `reportCase` in Go), and each section
supplies only its input parsing and the call. The diff stays inside next.md's scope (2 source files,
the Go test file, the regenerated CRAP baseline), changes no public signature, and keeps the
selftest result identical. The `[audit]` issue is resolved and deleted.

**Verification:**

- [x] `cargo test -p iscc-lib` passes (lib 315 passed) and `--no-default-features` passes (lib 264),
    including the 4 new runner tests; the CI leg `--no-default-features --features text-processing`
    passes too
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean (workspace clippy clean as well,
    `NativeMethods.g.cs` untouched)
- [x] `go test ./...` and `go vet ./...` pass in `packages/go` (also as `GOARCH=386`); `gofmt -l`
    prints nothing
- [x] Outside `mod tests`, each file prints "section missing from conformance data" once (rs :80, go
    :90) and "could not parse test inputs" once (rs :94, go :107)
- [x] After `mise run coverage`,
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 (`0 regressed, 0 new, 165 unchanged`); a regeneration into `/tmp` equals the committed
    baseline
- [x] `mise run check` exits 0

Beyond next.md's list (2 probes): in a throwaway copy, each of the four failure arms (mismatch,
error, parse failure, missing section) was changed to report success, once in Rust and once in Go.
All 8 mutations were caught by the new and the existing tests, so each arm is proven alone.

**Issues found:**

- (none filed)
- Nits, pre-existing and unreachable with the vendored `data.json`: the Rust gen parsers still index
    `inputs[N]` (a too-short array would panic, although the doc comment says "does not panic");
    `string_entries` drops a non-string code silently where the `encode_seq` parser reports a parse
    failure; Go reports a missing `outputs.iscc` as a mismatch, Rust as a parse failure

**Codex review:** No actionable findings. Codex re-ran the Rust tests with and without default
features and the Go tests natively and as a 32-bit build; all passed.

**Next:** the batch of small user-docs entries, all `normal` and docs-only: the pkg-config
`Version: 0.2.0` example, the Sum-Code `KAA` prefix comments, `llms.txt` coverage plus the ISO 24138
scope sentences, `docs/development.md` → "Publish" (with the Swift spec heading), and the Ruby Float
conversion docs. Two of them carry a comment-only source edit (`array_to_i32_vec` in
`crates/iscc-rb/src/lib.rs`, the `iscc` field comment in `SumCodeResult.java`); list those as
comment-only in next.md so they stay outside the file cap.

**Notes:**

- Diagnostic text on stderr changed slightly, as next.md allowed: Rust mismatch lines print both
    sides with `{:?}` (quoted), `encode_seq` compares lower-case hex strings, Go renders
    `decode_seq` unit lists with `%q`, and Go's "could not decode …" messages are folded into "could
    not parse test inputs"
- Go: a too-short `inputs` array is now a parse failure instead of an index panic
- No benchmarked hot path was touched, so no iai run was needed
- The remaining `[audit]` code entries are the one-shot/streaming duplication (mind the iai
    baseline) and the `alg_minhash_256([])` decision; both need their own step
