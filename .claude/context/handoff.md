## 2026-11-19 — Bind `encode_seq` / `decode_seq` in the UniFFI crate (Swift + Kotlin)

**Done:** `crates/iscc-uniffi` exports `encode_seq(Vec<String>) -> Vec<u8>` and
`decode_seq(Vec<u8>) -> Vec<String>`, which pass straight to the core with no binding guards. The
Swift and Kotlin bindings were regenerated, and both conformance harnesses run the 4 + 4 seq vectors
plus a bytes round trip, negatives, the padding control and an embedded-NUL probe. ISCC-SEQ is now
bound on all 11 surfaces.

**Files changed:**

- `crates/iscc-uniffi/src/lib.rs`: 2 exported wrappers + 6 unit tests (vector encode/decode, bytes
    round trip, `[]` and empty-bytes errors, embedded NUL)
- `packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`: regenerated (after format
    the diff only adds lines: +33 / +22 / +49)
- `packages/swift/Tests/IsccLibTests/ConformanceTests.swift`: `decodeBytes` (asserts the `bytes:`
    prefix), a shared `hexToData` (`decodeStream` now also asserts its `stream:` prefix),
    `assertIsccError`, and 9 seq test methods
- `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt`: `decodeBytes` (`require`s
    the prefix), `assertIsccError`, and the same 9 methods (ByteArrays compared with
    `assertArrayEquals`)

**Verification:**

- `cargo test -p iscc-uniffi`: 28 passed, including the 6 seq tests.
    `cargo clippy -p   iscc-uniffi --all-targets -- -D warnings`: exit 0
- `./gradlew clean test` (packages/kotlin): BUILD SUCCESSFUL. The JUnit XML reports tests=19,
    failures=0, errors=0 and lists `testEncodeSeq`, `testDecodeSeq`, `testSeqBytesRoundTrip`,
    `testEncodeSeqRejectsEmptyList`, `testDecodeSeqRejectsEmptyBytes`,
    `testDecodeSeqRejectsTruncatedBytes`, `testEncodeSeqRejectsPadding`,
    `testEncodeSeqPaddingControl`, `testEncodeSeqRejectsEmbeddedNul`
- `swift test` (swift 6.1.2 Debian 12 toolchain in /tmp/swifttc, `--scratch-path /tmp/swiftbuild`):
    "Executed 19 tests, with 0 failures". The same 9 seq methods passed, and the suite was re-run
    after formatting
- Regen no-op: bindgen run into a temp dir, compared after `sed 's/[[:space:]]*$//'` + stripping
    trailing EOF blank lines: `.h` and `.kt` identical. The `.swift`, re-diffed against the
    committed HEAD bytes, differs only by the missing final newline in the raw bindgen output
- `grep`: Swift `public func decodeSeq(data: Data)throws -> [String]` /
    `public func encodeSeq(units: [String])throws -> Data`; Kotlin `` fun `decodeSeq` `` /
    `` fun `encodeSeq` `` found
- `mise run check`: exit 0
- Core messages pinned in the negatives (checked first through the Python binding): `[]` → "at least
    one ISCC-UNIT", empty → "Empty ISCC-SEQ", last byte dropped → "Truncated ISCC-UNIT",
    `ISCC:GAAAAAAAAA======` → "base32 decode error", `unit+"\0junk"` → "base32 decode error".
    Control: `ISCC:GAAAAAAAAA` → `300000000000`

**Next:** the 33→35 doc sweep, now that all 11 surfaces bind seq. It includes Swift/Kotlin
`CLAUDE.md` + README (`ConformanceTests` is now 19 methods, not "9 methods / 50 vectors"), plus the
Ruby files listed in the previous handoff. After that, the Magnus `Vec<T>` segfault fix.

**Notes:**

- **Embedded NUL on UniFFI: the NUL is kept and the core rejects it**, in both Swift and Kotlin
    (strings cross as length-delimited RustBuffers). Each test has a control that the bare unit
    encodes, so the rejection comes from the NUL suffix. UniFFI was the last surface left to probe.
    Per learnings 206/207, only the C-string surfaces (.NET, and C++ without its guard) truncate
- `mise run format` failed once with
    `failed to write ... iscc_uniffi.swift: Invalid argument (os   error 22)` (looks like a WSL
    bind-mount flake). It left 86 trailing-whitespace lines in the Swift file. Re-running the hook
    on that file, then a further format pass, cleaned it up. Check `grep -c '[[:space:]]$'` on
    generated files and do not trust a single format pass
- Generated `.kt` still emits the known benign "Expression is unused" compile warning
- No core or hot-path change, so iai and CRAP are not affected, and there is no `iscc-lib` API
    change
