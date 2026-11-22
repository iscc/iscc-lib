## 2026-11-22 — Both conformance runners reject a wrong-type or out-of-range vector value as unparsable

**Done:** One rule for both `data.json` selftest runners: a wrong JSON type (incl. `null` outside
`meta`), a fraction or an out-of-range number makes the case unparsable — Rust parsers return
`None`, Go returns `errUnparsable`; nothing is coerced, truncated or skipped. Top-level keys other
than `_metadata` must map to an object of case objects on both sides (Rust: new `check_sections`,
called by `conformance_selftest`). The rule is in decisions.md and pinned by the same table on both
sides.

**Files changed:**

- `crates/iscc-lib/src/conformance.rs`: `bits_at`/`i32_entries`/pixel parse use
    `u32/i32/u8::try_from`; `string_entries(&Value) -> Option<Vec<&str>>` (strict, also reused by
    `encode_seq`); `meta_arg` returns `Option<Option<String>>` (null → `Some(None)`,
    number/bool/array → `None`); `check_sections`; tests
    `test_wrong_type_vector_values_are_unparsable`, `…_meta_…` (meta-code), `…_text_…`
    (text-processing), `test_seq_wrong_type_values_are_unparsable`, `test_check_sections`
- `packages/go/conformance.go`: `parseSection` + `isJSONObject` (null section/case rejected);
    `containsNull` (token scan) — `unmarshalInputs` rejects a null at any depth, except a
    `*json.RawMessage` target (meta slot) which is handed through; `metaArg` rejects anything but
    null/string/object; `bits` → `uint32`, cv/frames → `int32`/`[][]int32` directly, pixels →
    `[]int` + 0..=255 check; `decode_seq` outputs reject null entries
- `packages/go/conformance_test.go`: `TestWrongTypeVectorValuesAreUnparsable` (17 bad + 8 good
    rows), `TestSeqWrongTypeValuesAreUnparsable`, `TestParseConformanceDataRejectsNonCaseSections`
- `.crap-baseline.json`: regenerated (`mise run crap:baseline`) after `mise run check`
- `.claude/context/decisions.md`: `## 2026-11-22 — A wrong-type or out-of-range vector value …`
- `crates/iscc-lib/CLAUDE.md`, `packages/go/CLAUDE.md`: one bullet each naming the shared rule

**Verification:**

- `cargo test -p iscc-lib`: 456 passed; `--no-default-features`: 388 passed;
    `--no-default-features --features text-processing`: all result lines `ok` (291 lib tests)
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; also
    `--tests --no-default-features`
- `packages/go`: `CGO_ENABLED=0 go test -count=1 ./...` ok, `GOARCH=386 … go test` ok, `go vet`
    (amd64 + 386) clean, `gofmt -l .` empty
- Both grep criteria (` as (u8|u32|i32)\b`, `filter_map`) print nothing
- CRAP gate against the OLD baseline: `↑ 2 regressed` — `conformance_selftest` 3.0→4.1 (the
    `check_sections` branch) and `string_entries` 1.0→2.0 (the `?`), all ≤ 9.0. After regenerating:
    `cargo crap … --fail-regression --fail-above` exit 0, `0 regressed · 181 unchanged`. The
    baseline diff is large (+240/−112) because the regen also records the 17 functions that
    iteration 291 left as "new" and the moved line numbers
- `uv run pytest tests/test_vendored_fixtures.py`: 10 passed (no fixture changed)
- `mise run check`: exit 0 (first run, mdformat reflowed decisions.md; second run clean)
- Teeth proof (throwaway `git worktree` at HEAD with the new tests copied in, removed afterwards):
    old Go fails every bad row of the table plus both seq rows and the null section/case rows; old
    Rust fails the meta table (`meta 5` read as no meta → `(true, true)`) and the core table
    (`(true, false)`, mixed entry skipped). `git worktree list` shows only the checkout

**Next:** `version:check` passes a stale second doc snippet and skips a vanished target (the
handoff's second item; one script plus tests).

**Notes:**

- Beyond next.md's table I also reject `null` *inside* lists in Go (code-list entry, pixel, cv,
    frame, decode_seq unit) via `containsNull`: Go decodes such a null as `""`/`0`, while Rust's
    `as_str`/`as_i64` already refuse it, so without it the runners would still disagree. A `null`
    inside a `meta` object stays allowed on both sides (the meta slot is exempt)
- Go's case-insensitive struct-tag match (`"Inputs"`) is untouched, as scoped
- The Go `null`-section/`null`-case gap was found while implementing: `json.Unmarshal(null, &map)`
    left a nil map, so `{"gen_data_code_v0": null}` used to parse as an empty, passing section. Now
    an error, pinned by the test
- The issue "The Rust and Go conformance runners disagree on malformed vectors" should be resolved
    by the review (every row pinned on both sides); I did not edit issues.md
