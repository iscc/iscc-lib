## 2026-11-22 — Review of: Both conformance runners reject a wrong-type or out-of-range vector value as unparsable

**Verdict:** PASS

**Summary:** The Rust and Go `data.json` selftest runners now share one rule: a wrong JSON type, a
`null` outside `meta`, a fraction or an out-of-range number makes a case unparsable, and a top-level
key that is not an object of case objects fails the selftest. Every row of the audit issue's table
is pinned by the same test table on both sides, so the issue is closed. The change touches fixture
parsing only: no library behaviour, no `data.json` edit, no benchmarked path.

**Verification:**

- [x] `cargo test -p iscc-lib`, `--no-default-features` and
    `--no-default-features --features text-processing` pass (all exit 0); the five new tests
    (`test_wrong_type_*`, `test_seq_wrong_type_*`, `test_check_sections`) ran by name;
    `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean (workspace clippy too)
- [x] `packages/go`: `CGO_ENABLED=0 go test -count=1 ./...`, the same under `GOARCH=386`, and
    `go vet ./...` (amd64 and 386) pass; `gofmt -l .` prints nothing; the three new tests ran by
    name
- [x] `grep -nE ' as (u8|u32|i32)\b'` and `grep -nE 'filter_map'` on `conformance.rs` print nothing
- [x] `cargo llvm-cov … && cargo crap … --fail-regression --fail-above` exits 0 (`0 regressed`,
    `181 unchanged`)
- [x] `decisions.md` has the `## 2026-11-22 —` entry for the rule;
    `uv run pytest tests/test_vendored_fixtures.py`: 10 passed
- [x] `mise run check` exits 0

Three checks beyond next.md's list:

- **Differential table**: 89 further malformed inputs (bools, strings for numbers, exponents, nested
    lists, objects, bad stream prefixes, boundary values) went through every parser on both sides
    from one shared file. 86 give the same verdict. The 3 that differ are all `-0` as a pixel, `cv`
    or frame entry (see Issues found)
- **Baseline regen**: old and new `.crap-baseline.json` compared per function. Only the two scores
    the advance handoff names rose (`conformance_selftest` 3.0 → 4.1, `string_entries` 1.0 → 2.0).
    Every other difference is a function that iteration 291 left as "new", or an improvement
- **Mutations** in a `/tmp` copy: `containsNull` that never reports a null turns six rows red;
    `parseSection` without its object checks turns the two `null` rows red

**Issues found:**

- `-0` as a pixel, `cv` or frame entry: the Rust runner rejects it (serde_json reads `-0` as a
    float), the Go runner reads 0. Pre-existing and not in the issue's table. Not filed: no vendored
    vector can hold it (`data.json` is gated byte-identical to iscc-core's). It is named in the
    decision entry under "Not covered", together with Go's case-insensitive key match
- (nothing else)

**Codex review:** no actionable findings. Codex re-ran the Rust tests on all three feature sets, the
Go tests on amd64 and 386, clippy and `go vet`.

**Next:** `version:check` passes a stale second doc snippet and skips a vanished target silently
(`normal` [audit], one script plus tests). It is the only remaining `normal` issue that needs no
decision from Titusz. The other three wait on a recorded rule: `alg_minhash_256([])`, the WASM
`finalize()` consumption rule, and Go `EncodeComponent` on a short digest (for that one the issue
itself suggests rejection; define-next may record the rule in the step that applies it, but it is a
12-surface vector fan-out).

**Notes:**

- Closed and deleted from issues.md: "The Rust and Go conformance runners disagree on malformed
    vectors". issues.md now holds 4 `normal` and 15 `low` entries
- decisions.md was over budget (413 lines). I tightened the new entry to 12 lines and rotated the
    two oldest entries into `decisions-archive.md` with the runner's own function; the file is at
    387 lines. The rotated titles stay binding (`alg_simhash` rejects an empty list; review runs
    without the advisor)
- Go rejects a `null` at any depth of an input (except inside the `meta` slot), which goes one step
    past next.md's table. It is the right reading of the rule: Rust's `as_str`/`as_i64` already
    refuse a `null` list entry, so without it the runners would still disagree
- Rust's top-level check lives in `check_sections`; the call from `conformance_selftest` cannot be
    reached with synthetic data (the function reads the embedded file), so that one branch stays
    uncovered. That is what moved its CRAP score, and it is accepted
- No API change, no perf-relevant change: `conformance_selftest` keeps its signature and result on
    the real vectors
