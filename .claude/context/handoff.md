## 2026-11-21 — Review of: Saturate UniFFI `text_trim`'s `nbytes` and correct its doc comment

**Verdict:** PASS

**Summary:** UniFFI `text_trim` now saturates `nbytes` with
`usize::try_from(nbytes).unwrap_or(usize::MAX)`, so a limit above `u32::MAX` no longer trims the
text on 32-bit Android. The doc comment describes what the core does, and both generated bindings
carry it. The diff is one Rust file plus the two regenerated files, exactly as scoped.

**Verification:**

- [x] `cargo test -p iscc-uniffi` passes, including the two new tests: `30 passed; 0 failed`
- [x] `cargo test -p iscc-uniffi --target i686-unknown-linux-musl text_trim`: `2 passed; 0 failed`
- [x] `grep -rn "word boundary" crates/iscc-uniffi/src packages/swift/Sources packages/kotlin/src`
    prints nothing, exit 1 (`git grep -i` over the whole tree outside `.claude` is empty too)
- [x] Generated-file diff touches only the `text_trim` doc-comment block and the checksum constant
    `46357` → `61011` in each file
- [x] `cargo clippy -p iscc-uniffi --all-targets -- -D warnings` clean; `mise run check` exit 0
- [x] Kotlin `packages/kotlin/gradlew -p packages/kotlin test --rerun-tasks`: `BUILD SUCCESSFUL`,
    JUnit 25 tests, 0 failures. Swift `swift test` (6.1.2 at `/tmp/swifttc`): 30 tests, 0 failures

Probes beyond next.md (3):

- Regeneration is a no-op: fresh bindgen output in `/tmp` equals the committed Kotlin file modulo
    trailing whitespace, and the Swift file except for the end-of-file newline. The new checksum is
    what bindgen produces
- Negative control re-run in a `git archive` copy: with the old `nbytes as usize` on i686 the
    `(1 << 32) | 5` test fails with `left: "Hello"`, `right: "Hello World"`; the `u64::MAX` test
    passes under the old cast too, as the advance handoff says
- Sibling casts: napi and wasm narrow an `f64` with `as usize`, which saturates in Rust; FFI, Python
    and Ruby take `usize` directly. No other surface has this defect

**Issues found:**

- (none filed). Note: the wrap test only bites on a 32-bit target, and no CI job runs Rust tests
    there (decision of 2026-11-21 on 32-bit coverage), so on CI it is green under either cast. That
    is settled, not a new issue

**Codex review:** No actionable findings. It confirmed the native, 32-bit and Swift runs and that
both checksum constants match the rebuilt library. Its Kotlin run was blocked by a build-directory
conflict with my own gradle run in the same tree.

**Next:** the decided cleanup "C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection
the FFI no longer has" (remove both, keep the C++ and .NET empty-input tests green). After that the
generated-artifacts gate: the UniFFI outputs are in a known-clean state as of this commit (a fresh
regeneration differs only in whitespace), which is the starting point that gate needs.

**Notes:**

- Resolved and deleted: "UniFFI `text_trim` truncates `nbytes` on 32-bit Android and documents a
    word boundary". The generated-artifacts entry lost its cross-reference to it
- No API change (`text_trim(text: String, nbytes: u64) -> String`), core untouched, no CRAP or iai
    impact. A `///` edit on a UniFFI export changes its checksum constant in both generated files
    (learnings.md)
- learnings.md is at its 200-line budget: every new bullet needs an equal cut
