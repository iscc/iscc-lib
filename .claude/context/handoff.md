# Handoff

## 2026-07-28 — Bump uniffi 0.31 → 0.32 and regenerate the Swift + Kotlin bindings

**Done:** Pinned `uniffi = "0.32"` in the workspace (rewrote the stale 3-line pin comment), rebuilt
`iscc-uniffi`, and regenerated both checked-in bindings via the two documented bindgen commands
verbatim — pure regeneration, zero hand edits to generated code, no `crates/iscc-uniffi/src` change
needed (as next.md predicted: proc-macro-only crate, both constructors sync, no `uniffi.toml`).

**Files changed:**

- `Cargo.toml`: `uniffi = "0.31"` → `"0.32"`; pin comment now evergreen (no "pending bump" / "not
    verifiable locally" wording)
- `Cargo.lock`: `uniffi` → 0.32.0 (+ new transitives: askama 0.16, cargo_metadata 0.23.1,
    uniffi_pipeline, uniffi_internal_macros, fs-err 3.3.1)
- `packages/swift/Sources/IsccLib/iscc_uniffi.swift`: regenerated (406-line diff vs HEAD)
- `packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h`: regenerated — after hook normalization
    it is byte-identical to HEAD, so it drops out of the final diff
- `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`: regenerated (~980-line diff)

**Verification:**

- [x] `grep -n 'uniffi = "0.32"' Cargo.toml` → L55; Cargo.lock resolves `uniffi 0.32.0`;
    `rust-version = "1.85"` → L17 unchanged; stale-wording grep → no matches
- [x] Regeneration determinism: two raw bindgen runs produced sha256-identical output for all three
    files. **Caveat next.md anticipated:** raw output carries trailing whitespace and a
    missing/extra EOF newline that the prek hygiene hooks strip, so a raw regen over the committed
    tree leaves whitespace-only diffs (not `git status` empty). Proven content-identical:
    `diff <(sed 's/[[:space:]]*$//' raw) <(sed … committed)` differs only in EOF blank lines / final
    newline for all three files. No hand edits made.
- [x] Kotlin (final committed bytes): `cargo build -p iscc-uniffi` → `./gradlew clean test` BUILD
    SUCCESSFUL; `ConformanceTest.xml` `tests="9" skipped="0" failures="0" errors="0"`,
    `UnicodeBoundaryTest.xml` `tests="13" skipped="0" failures="0" errors="0"`
- [x] Swift (swift.org 6.1.2 Debian 12 tarball recipe, final bytes):
    `swift test --scratch-path   /tmp/swiftbuild -Xlinker -L../../target/debug -Xlinker -rpath -Xlinker ../../target/debug`
    → `Executed 12 tests, with 0 failures (0 unexpected)`, 0 skips
- [x] `mise run lint` exit 0 (clippy clean, 187 files formatted); `mise run test` exit 0 (17 cargo
    `test result: ok` lines incl. iscc-uniffi 21 passed; 441 pytest passed)
- [x] `mise run audit` exit 0 (`advisories ok, bans ok, licenses ok, sources ok`), no `deny.toml`
    edit — only non-fatal `warning[duplicate]` entries (libloading, shlex, syn, winnow), from 0.32's
    bindgen-side transitives
- [x] `mise run check` exit 0, 18 hooks Passed (after the EOF-fixer second-pass, see Notes)

**Next:** v0.6.0's CID-doable dependency work is complete per the prior review. Remaining release
items are human-gated (semver flip / MSRV job are v1.0.0 prerequisites). The stale Go `Final_Sigma`
claim in `specs/rust-core.md` L149-157 remains for review to fix inline if it chooses.

**Notes:**

- **Hygiene hooks rewrite the raw generated files** (recorded per next.md instruction, no hand
    edits): trailing whitespace on blank lines (all three) + EOF newline (swift file ends without
    one; kt/h end with extra blank lines). The `.h` becomes byte-identical to the 0.31 checked-in
    version after normalization — 0.32's header changes are whitespace-only. Hook ordering quirk:
    `mise run format` needed a follow-up `mise run check` pass before the kt file reached a fixed
    point (trailing-ws strip exposed a fresh EOF-blank-line for the next end-of-file-fixer run).
- Kotlin compile emits one warning in generated code (`iscc_uniffi.kt:1082 Expression is unused` —
    uniffi's intentional `IntegrityCheckingUniffiLib` object reference in
    `uniffiEnsureInitialized`). Warning only; Gradle build is not -Werror. Do not hand-fix.
- bindgen still prints `Warning: Unable to auto-format … using swift-format` (no swift-format in
    container) — same as 0.31; Kotlin uses `--no-format` as documented.
- Both CLAUDE.md generation commands parse unchanged under 0.32 (ran verbatim); the `--library` flag
    is deprecated-but-accepted per the 0.32 CLI change, so the recipes stay accurate.
- The `proc-macro-error2 v2.0.1` future-incompat warning remains pre-existing (dev-only via
    iai-callgrind-macros).
- macOS `swift` CI job on the pushed commit is the final confirmation per Done-When.
