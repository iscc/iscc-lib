# Next Work Package

## Step: Both conformance runners reject a wrong-type or out-of-range vector value as unparsable

## Goal

Picks up the `normal` [audit] issue "The Rust and Go conformance runners disagree on malformed
vectors". It sets one rule, a wrong JSON type or an out-of-range value is a parse failure, records
it, and applies it to `conformance.rs` and `conformance.go`, so the two selftests give the same
verdict on every row of the issue's table.

## Alternatives Considered

- **Chosen:** runner wrong-type rule. It is the handoff's first item, needs no human decision, and
    the inputs in `data.json` are all well typed, so the strict rule breaks no real vector. Checked:
    every `bits` is an int, every list holds only ints or only strings, `meta` is null, a dict or a
    string, and every top-level key except `_metadata` is a section of cases.
- **Rejected:** `version:check` gaps (1 file). Smaller and also ready, but the handoff puts it
    second. It is next.
- Not loop-ready: `alg_minhash_256([])`, WASM `finalize()` (each needs Titusz's behaviour call), Go
    `EncodeComponent` (needs a rule recorded first, then a 12-surface vector fan-out).

## Scope

- **Modify**: `crates/iscc-lib/src/conformance.rs`, `packages/go/conformance.go`,
    `packages/go/conformance_test.go` (test), `.crap-baseline.json` (generated, only if the gate
    needs it), `.claude/context/decisions.md` (append one entry), `packages/go/CLAUDE.md` and
    `crates/iscc-lib/CLAUDE.md` (one line each on the shared rule)
- **Reference**: the issue's table in issues.md; learnings → "Fixture parsers coerce silently"

## Not In Scope

- The per-language data-edge harnesses (`tests/data_edge*.rs`, `data_edge_test.go`, the other 10
    bindings). This rule covers only the two `data.json` selftest runners
- Go's case-insensitive struct-tag match (`"Inputs"` matches `inputs`). Leave it unless it falls out
    of the change for free
- Library behaviour (`gen_*`, codec). Only fixture parsing changes; no `data.json` edit
- Moving the iai baseline. This code is off every benchmarked path

## Implementation Notes

**The rule (write it into decisions.md as the decision):** a vector value of the wrong JSON type, or
a number that is not an integer or is outside the argument's range, makes the case unparsable (Rust:
the parser returns `None`; Go: `errUnparsable`). It is never coerced, truncated, skipped or
defaulted. The only allowed value forms:

- `name`, `description`, `text`, stream, `bytes:` and code entries: JSON string (`null` rejected; an
    empty `description` still means no description)
- `meta`: `null`, string or object. A number, bool or array is rejected
- `bits`: integer in `u32` range. Pixel: integer 0..=255. `cv` and frame entries: integer in `i32`
    range. Code lists (mixed, iscc, encode_seq) and decode_seq outputs: every entry a string
- Top level: every key except `_metadata` must map to an object of case objects, or the selftest
    fails (Go already does this; Rust ignores unknown keys today. Add a small checked helper that
    `conformance_selftest` calls, so a test can feed it a synthetic value)

**Rust:** use `u32::try_from` / `u8::try_from` / `i32::try_from` on `as_u64` / `as_i64` instead of
`as` casts. `string_entries` must fail on a non-string entry (`collect::<Option<_>>()`, as the seq
runners already do). `meta_arg` must tell "absent" (`null`) apart from "rejected". Keep `?` chains
flat: cargo-crap counts each `?`, and CI fails a covered function whose score rises unless
`.crap-baseline.json` moves in the same commit.

**Go:** `json.Unmarshal` of `null` into a string, slice or number leaves the zero value with no
error. `unmarshalInputs` must reject a raw `null` input (the meta slot is a `json.RawMessage` and
goes through `metaArg`, which must also reject a value whose first byte is not `"` or `{`). Decode
`bits` into `uint32` and cv/frames into `int32` directly: `encoding/json` refuses `64.0`, `64.5` and
out-of-range numbers for integer targets. **Trap:** `[]uint8` is `[]byte`, which `encoding/json`
decodes from a base64 *string*. Decode pixels into a wider integer slice (e.g. `[]int`) and
range-check each one.

**Tests (one bad case per call, learnings → "One bad case per call"):** a Rust `#[test]` in the
`conformance.rs` test module and a Go `Test…` in `conformance_test.go`, each feeding the same table:
`bits` `64.0`, `64.5`, `-1`, `4294967296`; description `null`; name `null`; `meta` `5` and `[]`; a
non-string entry in a mixed and in an iscc code list; pixel `0.5`, `-256`, `256`; cv entry
`2147483648`; and a top-level key mapped to a number. Every case carries an output the correct input
would match, so only the parse arm can fail it. Keep one positive case per changed parser so the
table cannot pass because everything fails. Gate the meta row with `#[cfg(feature = "meta-code")]`
and the text row with `text-processing`, as the existing tests do.

## Verification

- `cargo test -p iscc-lib`, `cargo test -p iscc-lib --no-default-features` and
    `cargo test -p iscc-lib --no-default-features --features text-processing` pass, including the
    new wrong-type table test; `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean
- `CGO_ENABLED=0 go test -count=1 ./...`, `GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` and
    `go vet ./...` pass in `packages/go`, including the new wrong-type table test
- `grep -nE ' as (u8|u32|i32)\b' crates/iscc-lib/src/conformance.rs` prints nothing, and
    `grep -nE 'filter_map' crates/iscc-lib/src/conformance.rs` prints nothing
- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info && cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0
- `.claude/context/decisions.md` has a `## 2026-11-22 —` heading for the wrong-type rule, and
    `uv run pytest tests/test_vendored_fixtures.py` passes (no fixture changed)
- `mise run check` exits 0

## Done When

Both runners reject every row of the table as unparsable without a panic, each row is pinned by a
test on both sides, the rule is in decisions.md, and every check above passes.
