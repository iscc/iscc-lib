## 2026-11-19 — Review of: Bind `encode_seq` / `decode_seq` in the UniFFI crate (Swift + Kotlin)

**Verdict:** PASS

**Summary:** `crates/iscc-uniffi` exports `encode_seq`/`decode_seq` as thin pass-through wrappers.
Swift and Kotlin bindings were regenerated (purely additive), and both conformance harnesses run the
4 + 4 data.json seq vectors, a bytes round trip, one-bad-input-per-call negatives, the valid padding
control and an embedded-NUL probe. ISCC-SEQ is now bound on all 11 surfaces. The diff is in scope: 1
hand-edited source file, 3 generated files and 2 test files.

**Verification:**

- [x] `cargo test -p iscc-uniffi`: 28 passed (6 seq tests: vector encode/decode, bytes round trip,
    `[]` and empty-bytes errors, NUL). `cargo clippy -p iscc-uniffi --all-targets -- -D warnings`
    and workspace clippy: exit 0
- [x] `gradlew -p packages/kotlin clean test --offline`: BUILD SUCCESSFUL. JUnit XML
    `ConformanceTest` has tests=19, failures=0, errors=0, skipped=0, with all 9 seq methods listed
    and a fresh timestamp. No `Deprecat` lines
- [x] `swift test` (6.1.2 Debian 12 toolchain, fresh `--scratch-path /tmp/rv-swiftbuild`, absolute
    rpath): "Executed 19 tests, with 0 failures". All 9 seq methods passed, with no warnings from
    the package sources
- [x] Regenerated into a temp dir and normalized (trailing whitespace and EOF blank lines): `.kt`
    and `.h` identical. `.swift` differs only by raw bindgen's missing final newline. The
    `.modulemap` is hand-maintained and was not touched
- [x] `grep`: Swift `public func decodeSeq(data: Data)` / `public func encodeSeq(units: [String])`;
    Kotlin `` fun `decodeSeq` `` / `` fun `encodeSeq` ``. 0 trailing-whitespace lines in all three
    generated files
- [x] `mise run check`: exit 0, tree clean afterwards

**Issues found:**

- Nit: the Rust `test_encode_seq_embedded_nul_rejected` asserts only `is_err()` and has no message
    check or bare-unit control. The Swift/Kotlin NUL tests have both, so coverage is fine overall
- Nit: Swift `decodeStream` gained a `stream:` prefix precondition, but Kotlin `decodeStream` still
    uses a bare `removePrefix`. This asymmetry is harmless
- No gate circumvention in the unpushed range (define-next, advance, and the update-state and log
    commits). No `iscc-lib` API change and no hot-path change, so iai, CRAP and semver are
    unaffected

**Codex review:** ran on the main advance commit `246705d`. HEAD `1308c08` is only a handoff and
memory fixup, so it was not reviewed separately. No actionable findings. Codex independently ran the
28 Rust, 19 Swift and 19 Kotlin tests, and all passed.

**Next:** the 33→35 Tier-1 doc sweep, now that all 11 surfaces bind `encode_seq`/`decode_seq`. Known
stale spots:

- `crates/iscc-uniffi/src/lib.rs:3`: the module doc says "all 33 Tier 1 symbols"
- Swift/Kotlin `CLAUDE.md` + README: `ConformanceTests` is now 19 methods
- the Ruby files listed in the 209 handoff
- every "33" count across docs, READMEs, `notes/`, the per-crate `CLAUDE.md` files and `docs/llms*`

Grep for `33` / `Tier 1` across the tree rather than trusting this list. A sweep touching many files
needs the `**Fan-out:**` valve (one mechanically identical count change). Any per-surface API
example it adds must be verified against the real signature (190: a docs sweep fabricated a return
type). After the sweep: the Ruby Magnus `Vec<T>` segfault fix (`normal`, single file).

**Notes:**

- Embedded-NUL probes so far: JNI, Ruby, UniFFI (Swift + Kotlin) and Python keep the NUL and the
    core rejects it (napi/WASM not probed). Only the C-string surfaces (.NET, and C++ outside its
    `encode_seq` guard) truncate. That is the open `normal` issue, and its fix design is Titusz's
    call
- The WSL bind-mount `OSError: [Errno 22] Invalid argument` hit a prek/mdformat write again this
    review (on `learnings.md`). Re-running fixes it. Do not mistake it for a hook rejection
- The Swift Linux toolchain is still extracted at `/tmp/swifttc` (it does not survive a container
    rebuild)
