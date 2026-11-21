# Next Work Package

## Step: One section runner for `conformance_selftest` in Rust and Go

## Goal

Resolve the `[audit]` issue "`conformance_selftest` repeats one runner loop nine times, in Rust and
in Go". Each language gets one section runner (lookup, case loop, outcome reporting), and each of
the 11 sections only supplies how to parse its inputs and call its function. No behaviour changes.

## Alternatives Considered

- **Chosen:** single section runner (audit issue, review's "Next"). It is shipped Tier 1 code. The
    copies already differ (2 of 11 runners use `report_case`). It stays at 2 source files plus the
    regenerated CRAP baseline. It touches core code, so it goes alone, as the rules require.
- **Rejected:** the batch of small user-docs entries (pkg-config `0.2.0`, `KAA` prefix, `llms.txt`
    coverage + ISO scope, `docs/development.md` "Publish", Ruby Float docs). These are independent
    and docs-only, so they are the next step; they don't depend on this one in either direction.

## Scope

- **Audit refactor**, file budget 8 (uses 2 + the regenerated baseline)
- **Modify**: `crates/iscc-lib/src/conformance.rs`, `packages/go/conformance.go`,
    `.crap-baseline.json` (regenerated, not hand-edited)
- **Modify (tests)**: the `mod tests` in `conformance.rs`, `packages/go/conformance_test.go`
- **Reference**: `.github/workflows/ci.yml` → job "Coverage + CRAP" (the gate lines),
    `.cargo-crap.toml`

## Not In Scope

- Changing any `gen_*_v0`, `encode_seq` or `decode_seq` behaviour, the vector data, or the
    `conformance_selftest` / `ConformanceSelftest` signatures and return values
- Binding crates/packages: they call the core's selftest and need no edit
- The one-shot/streaming duplication audit issue (another `[audit]` entry, its own step)
- Merging `parseConformanceData` and `parseSeqConformanceData` in Go, unless the generic runner
    falls out naturally without it. Keep both if in doubt
- Widening `--epsilon` / `--fail-above` or editing `.cargo-crap.toml`

## Implementation Notes

**Rust** (`conformance.rs`, 693 lines; nine gen runners at :64–545, `report_case` :560, seq runners
:571/:610):

- One `run_section(data, func_name, case)`. It looks up `data[func_name].as_object()` and prints
    "section missing" when it fails. It loops over the cases, calls the closure, and folds every
    outcome through `report_case`. Every runner, the two seq runners included, goes through it
- One shared comparison helper for the "expected …, got …" and "error: …" arms. A gen closure can
    return `Option<IsccResult<String>>` (parse with `?`, then `Some(gen_x(..).map(|r| r.iscc))`),
    and the runner compares it against `outputs.iscc`. The seq outputs are bytes / `Vec<String>`.
    Either make the helper generic (`T: PartialEq + Debug`), or compare encode_seq as a lower-case
    hex `String`. advance picks; the mismatch text may change slightly (it is stderr diagnostics
    only)
- Keep the `#[cfg(feature = "meta-code")]` / `"text-processing"` gates on the meta/text parsers.
    `cargo test -p iscc-lib --no-default-features` must still compile and pass
- Keep the existing runner fn names (`run_encode_seq_tests` …) as thin wrappers, or update the tests
    that call them. Either is fine
- Watch closure cyclomatic complexity: cargo-crap does not count closure bodies, but it does count
    `?`, `||`, `if` and top-level `match` arms in named fns. Keep each named fn small

**Go** (`conformance.go`, 638 lines; `vectorEntry` :22, `seqVector` :508):

- A generic `runSection[V any](data map[string]map[string]V, funcName string, runCase …) bool` that
    serves both vector types (check that the `go` directive in `packages/go/go.mod` is ≥ 1.18). Also
    one helper each for "could not parse test inputs", "error: %v" and "expected %s, got %s", so
    each `Fprintf` text appears once. "could not decode input/output" may fold into the
    parse-failure message
- `runMetaCase`, `runEncodeSeqCase` and `runDecodeSeqCase` keep their per-case parsing and lose the
    repeated reporting

**Tests** (one bad case per runner call, see learnings → "One bad case per call"):

- Rust and Go: add a test that feeds a gen section through the shared runner, once with a
    mismatching `outputs.iscc`, once with an input that makes the function error (e.g. `bits` 7),
    and once with unparsable inputs. Each must return `false`, each in its own call. Also check that
    a missing gen section returns `false`. The existing seq failure-arm tests must stay green
    unchanged in intent

**CRAP baseline:** the gate is CI-only. Removing and moving functions re-keys `conformance.rs`
entries in `.crap-baseline.json`. Run `mise run crap:baseline` (it runs `mise run coverage` first)
after the code is final, and commit the result in the same step. Then run the CI gate line locally:
`cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`. No hot
path changes → no iai concern.

## Verification

- `cargo test -p iscc-lib` and `cargo test -p iscc-lib --no-default-features` pass, including the
    new runner failure-arm tests
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean
- `go test ./...` and `go vet ./...` pass in `packages/go`, including the new Go failure-arm tests
- Outside `mod tests`, the Rust file and the Go file each contain exactly one statement that prints
    "section missing from conformance data", and one that prints "could not parse test inputs"
    (`grep -n` shows a single line per string per file)
- After `mise run coverage`, the CI gate line
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0
- `mise run check` exits 0

## Done When

Both languages run all 11 conformance sections through one section runner, the selftest still
passes, and every criterion above holds on the working tree.
