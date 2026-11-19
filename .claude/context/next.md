# Next Work Package

## Step: Bind `encode_seq` / `decode_seq` in the UniFFI crate (Swift + Kotlin)

## Goal

Export `encode_seq` / `decode_seq` from `crates/iscc-uniffi`, regenerate the Swift and Kotlin
bindings, and run both data.json seq sections in each conformance harness. This binds ISCC-SEQ on
the last two of the 11 surfaces (v0.8.0, GitHub #45).

## Alternatives Considered

- **Chosen:** UniFFI seq. It is the last missing surface of the v0.8.0 headline feature, and one
    source file feeds both Swift and Kotlin.
- **Rejected:** the Ruby Magnus `Vec<T>` segfault fix (`normal`, single file). It is self-contained
    and can go next. Finishing the feature first unblocks the 33→35 doc sweep.

## Scope

- **Modify**: `crates/iscc-uniffi/src/lib.rs` (the only hand-edited source file)
- **Regenerate** (generated, outside the 3-file budget, as in `3c31f89` for IDv1):
    `packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`
- **Tests**: `packages/swift/Tests/IsccLibTests/ConformanceTests.swift`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt`, plus Rust unit tests in
    `lib.rs`
- **Reference**: `crates/iscc-lib/src/codec.rs` (`encode_seq` L818, `decode_seq` L857);
    `crates/iscc-uniffi/src/lib.rs` `gen_iscc_code_v0` (string-list input) and `iscc_decompose`
    (string-list output); `packages/swift/CLAUDE.md` and `packages/kotlin/CLAUDE.md` ("Binding
    Generation", plus the Swift Linux toolchain recipe under "Common Pitfalls")

## Not In Scope

- The 33→35 doc sweep, including the Swift/Kotlin `CLAUDE.md` and README files. Steps 204–209 all
    deferred docs to one sweep after the fan-out, and that sweep is the next step
- Editing either vendored `data.json`: both are byte-identical to `crates/iscc-lib/tests/data.json`
    and already carry the seq sections
- Fixing embedded-NUL handling (see the notes) or any other `normal` issue
- Hand-editing generated files, or touching the hand-maintained `module.modulemap`

## Implementation Notes

- Wrappers: `encode_seq(units: Vec<String>) -> Result<Vec<u8>, IsccUniError>` (build `Vec<&str>` as
    in `gen_iscc_code_v0`) and `decode_seq(data: Vec<u8>) -> Result<Vec<String>, IsccUniError>`.
    Pass straight to core, with no guards in the binding. Core owns the error messages, e.g. "at
    least one ISCC-UNIT" for `[]` and "Empty ISCC-SEQ" for empty bytes. Expected generated shapes:
    Swift `encodeSeq(units:)` returns `Data`; Kotlin `encodeSeq(units)` returns `ByteArray`.
- Regeneration (recipes are in the package `CLAUDE.md` files): run `cargo build -p iscc-uniffi`
    first. Swift bindgen writes the `.h` and `.modulemap` into `Sources/IsccLib/`; move the `.h` to
    `Sources/iscc_uniffiFFI/` and delete the generated `.modulemap`. Kotlin uses `--no-format`. Raw
    bindgen output has trailing whitespace and EOF-newline issues, so run `mise run format`, which
    may need two passes to settle.
- **Harness traps:**
    - The seq vectors use `bytes:<hex>`, not `stream:`. Swift `decodeStream` drops 7 characters
        (`stream:`), so on `bytes:` (6 characters) it would silently eat a hex digit. Kotlin
        `removePrefix("stream:")` leaves the prefix in place for `parseHex`. Add a `bytes:`-aware hex
        helper in each harness.
    - `outputs` is a string for `encode_seq` and an array for `decode_seq`, so the existing
        `getAsJsonObject("outputs")` and `as! [String: Any]` patterns crash. `encode_seq` inputs are
        nested: the unit list is `inputs[0]`. `decode_seq` input is `inputs[0]` (a `bytes:` string).
    - Kotlin `assertEquals` on `ByteArray` compares references, so use `assertArrayEquals`.
    - Hard-code the per-section counts (4 and 4) like the other test methods.
- Add tests in both languages:
    - a bytes round trip `encodeSeq(decodeSeq(x)) == x` over the `decode_seq` vectors
    - negatives, one bad input per call: `[]`, empty bytes, truncated bytes (drop the last byte of a
        vector), and the strict-padding unit `ISCC:GAAAAAAAAA======`
    - the padding control `ISCC:GAAAAAAAAA` → hex `300000000000`. Do NOT use the `EEAY…====` literal:
        iscc-core rejects it too, so it cannot catch a padding regression
- **Embedded-NUL probe:** UniFFI passes strings length-delimited, so `unit + "\0junk"` should reach
    core intact and be rejected (as on JNI/Ruby). Pin that with a test in both languages. If either
    language returns a result instead, keep a test that records the observed behaviour, report it in
    the handoff, and do not fix it: the NUL fix design is the human's call (open `normal` issue).
- Binding-only step: no core or hot-path change, so the iai and CRAP gates are unaffected.

## Verification

- `cargo test -p iscc-uniffi` and `cargo clippy -p iscc-uniffi -- -D warnings` both exit 0, with
    Rust unit tests covering an `encode_seq` / `decode_seq` round trip and the empty-input errors
- `./gradlew clean test` in `packages/kotlin` exits 0 (after `cargo build -p iscc-uniffi`), and the
    JUnit XML report lists the seq test methods
- `swift test` passes locally using the Linux toolchain recipe in `packages/swift/CLAUDE.md`
    (`--scratch-path` outside the repo); do not defer this to macOS CI. The seq test methods appear
    in the output
- Regenerating Swift and Kotlin bindings into a temp dir and diffing against the committed files
    after `sed 's/[[:space:]]*$//'` shows no difference (EOF blank lines aside), so the committed
    bindings match `lib.rs`
- `grep -n "func encodeSeq\|func decodeSeq" packages/swift/Sources/IsccLib/iscc_uniffi.swift` and
    the Kotlin equivalent (`` fun `encodeSeq` ``/`` fun `decodeSeq` ``) each find a public export
- `mise run check` exits 0

## Done When

All six verification criteria pass, with the seq vectors running in both the Swift and the Kotlin
conformance harnesses.
