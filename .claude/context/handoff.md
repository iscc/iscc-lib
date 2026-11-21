## 2026-11-21 — Saturate UniFFI `text_trim`'s `nbytes` and correct its doc comment

**Done:** UniFFI `text_trim` now converts `nbytes` with
`usize::try_from(nbytes).unwrap_or(usize::MAX)` instead of `as usize`, so a limit beyond the address
space trims nothing on 32-bit targets. The "word boundary" doc comment is replaced by a description
of the core's behaviour, and both UniFFI bindings are regenerated.

**Files changed:**

- `crates/iscc-uniffi/src/lib.rs`: `text_trim` body + doc comment; two tests
    (`test_text_trim_nbytes_max_returns_whole_text` with `u64::MAX` and leading/trailing spaces,
    `test_text_trim_nbytes_above_u32_does_not_wrap` with `(1u64 << 32) | 5`)
- `packages/swift/Sources/IsccLib/iscc_uniffi.swift` (generated): doc-comment lines + checksum
- `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (generated): same

**Verification:**

- `cargo test -p iscc-uniffi` → `30 passed; 0 failed` (both new tests listed `ok`)
- `cargo test -p iscc-uniffi --target i686-unknown-linux-musl text_trim` → `2 passed; 0 failed`
- Negative control (throwaway `git worktree`, new tests + old `nbytes as usize` body, i686): the
    `(1<<32)|5` test FAILED with `left: "Hello"`, `right: "Hello World"`; the `u64::MAX` test passes
    even under the old cast (it truncates to `u32::MAX`, still ≥ the text length). Worktree removed;
    `git worktree list` shows only the checkout
- `grep -rn "word boundary" crates/iscc-uniffi/src packages/swift/Sources packages/kotlin/src` → no
    output, exit 1
- Generated-file diff (`git diff -U0`, after the hooks normalized whitespace) in each file: the
    `text_trim` doc-comment block (1 line → 5 lines) and
    `uniffi_iscc_uniffi_checksum_func_text_trim() != 46357` → `!= 61011`. **Checksum constant
    changed** in both files: UniFFI hashes the doc comment into the function checksum; both sides
    were regenerated from the same build, so they agree. No other lines changed
- `cargo clippy -p iscc-uniffi --all-targets -- -D warnings` → clean
- `mise run check` → exit 0 on the third run (runs 1-2 normalized the raw bindgen output: trailing
    whitespace, then the Kotlin file's extra end-of-file blank lines)
- Kotlin: `packages/kotlin/gradlew -p packages/kotlin test` → `BUILD SUCCESSFUL`, exit 0; JUnit XML:
    25 tests, 0 failures, 0 errors, 0 skipped
- Swift (Linux toolchain at `/tmp/swifttc`, swift 6.1.2):
    `swift test --package-path packages/swift   --scratch-path /tmp/swiftbuild -Xlinker -L$PWD/target/debug -Xlinker -rpath -Xlinker   $PWD/target/debug`
    → `Executed 30 tests, with 0 failures`, exit 0

**Next:** the decided cleanup "C++ `safe_data` and .NET `PinnedRows` padding" (per the previous
review). The generated-artifacts gate can start from this known-clean regeneration.

**Notes:**

- Swift bindgen wrote untracked `iscc_uniffiFFI.h` and `iscc_uniffiFFI.modulemap` into
    `Sources/IsccLib/`; deleted. Bindgen also warned that `swift-format` is missing (harmless)
- No public API change: the signature stays `text_trim(text: String, nbytes: u64) -> String`
- No CRAP/iai impact: the core is untouched and `iscc-uniffi` sits outside both gates
