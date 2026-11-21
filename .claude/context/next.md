# Next Work Package

## Step: Saturate UniFFI `text_trim`'s `nbytes` and correct its doc comment

## Goal

Fix the `normal` [audit] issue "UniFFI `text_trim` truncates `nbytes` on 32-bit Android and
documents a word boundary". Right now an `nbytes` above `u32::MAX` silently trims the text on the
shipped `armeabi-v7a`/`x86` ABIs, and the published Swift/Kotlin API docs describe behaviour the
function does not have.

## Alternatives Considered

- **Chosen:** UniFFI `text_trim`. It is a silent wrong result on two shipped ABIs, the fix is small
    (1 non-test, non-generated file), and it can be checked on a real 32-bit target here.
- **Rejected:** removing the C++ `safe_data` and .NET `PinnedRows` padding (decided). It spans two
    surfaces and a pytest gate, and it removes dead code without fixing any wrong result. It is
    next.
- **Rejected:** the gate for undiffed generated artifacts. It is bigger (CI and hooks), and this
    step regenerates the UniFFI outputs by hand anyway, which gives that gate a known-clean starting
    point.

## Scope

- **Modify**: `crates/iscc-uniffi/src/lib.rs` (the `text_trim` body and doc comment, plus tests in
    its `#[cfg(test)]` module)
- **Modify (generated, not counted)**: `packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`
- **Reference**: `crates/iscc-lib/src/utils.rs` (`text_trim`, about lines 122-137),
    `packages/swift/CLAUDE.md` and `packages/kotlin/CLAUDE.md` → "Binding Generation" (exact bindgen
    commands)

## Not In Scope

- `sliding_window(width: u32)` and `alg_cdc_chunks(avg_chunk_size: u32)`. Converting `u32` to
    `usize` loses nothing on any target UniFFI ships, so leave them alone
- The `u32` constant getters (`META_TRIM_* as u32`). They widen from small constants, and no value
    is lost
- A `data-edge.json` vector. Out-of-range integer arguments are per-binding tests (decision of
    2026-11-21)
- Swift or Kotlin test additions. The host JVM and macOS are 64-bit, so a test there cannot see the
    bug; the 32-bit Rust run below covers it
- Changing `nbytes` to `u32` in the UniFFI signature (an API break on two published packages)
- Adding a CI job for i686 or for regenerating UniFFI outputs (the latter belongs to the
    generated-artifacts issue)

## Implementation Notes

- Body: `iscc_lib::text_trim(&text, usize::try_from(nbytes).unwrap_or(usize::MAX))`. A limit beyond
    the address space trims nothing, which matches the core's `text.len() <= nbytes` branch (it
    still strips Python whitespace)
- Doc comment: describe what the core does. It cuts to the longest valid UTF-8 prefix within
    `nbytes` bytes and drops any character that would be split, strips leading and trailing
    whitespace, and returns the text unchanged except for that strip when `nbytes` is at least its
    size. Keep it short; it is copied verbatim into the generated Swift and Kotlin
- Tests (next to the existing `text_trim` test at about line 643): `nbytes = u64::MAX` returns the
    whole text, and `nbytes = (1u64 << 32) | 5` returns the whole text (not `"Hello"`). The second
    one fails under the old cast on a 32-bit target only, so run it on i686 (Verification). The
    decision of 2026-11-21 on 32-bit coverage records all core tests passing on
    `i686-unknown-linux-musl` (installed via rustup); `iscc-uniffi` has no dev-dependencies, so the
    `alloca`/criterion blocker does not apply. rustc may warn that it drops the `cdylib` crate type
    on musl; that is harmless
- Regenerate both bindings with the commands in the two `CLAUDE.md` files (build
    `cargo build -p iscc-uniffi` first). Swift bindgen also writes `iscc_uniffiFFI.h` and a
    modulemap into `Sources/IsccLib/`. They are untracked there, so delete them; only
    `iscc_uniffi.swift` is tracked. Expected diff in each generated file: the doc-comment lines
    only. uniffi output is not hook-clean (learnings), so compare modulo trailing whitespace
    (`sed 's/[[:space:]]*$//'`). If an API checksum constant also changes, keep it (both sides are
    regenerated together) and say so in the handoff
- No CRAP/iai impact: `iscc-uniffi` sits outside both gates (the core is untouched)

## Verification

- `cargo test -p iscc-uniffi` passes, including the two new `text_trim` tests
- `cargo test -p iscc-uniffi --target i686-unknown-linux-musl text_trim` passes (both new tests run
    on a 32-bit `usize`)
- `grep -rn "word boundary" crates/iscc-uniffi/src packages/swift/Sources packages/kotlin/src`
    prints nothing (exit 1)
- `git diff` on both generated files, with trailing whitespace stripped, touches only the
    `text_trim` doc-comment lines (plus at most a checksum constant, reported in the handoff)
- `cargo clippy -p iscc-uniffi --all-targets -- -D warnings` is clean and `mise run check` exits 0
- If the toolchains are reachable (`swift`, gradle per memory), the Swift and Kotlin suites stay
    green. Record the commands and their results in the handoff

## Done When

The advance agent is done when an oversized `nbytes` returns the whole text on both a 64-bit and a
32-bit target, the doc comment matches the core in the Rust source and both generated bindings, and
every check above passes.
