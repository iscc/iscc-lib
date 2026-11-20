# Next Work Package

## Step: JNI hasher null-handle guard and OOM/pending-exception mapping

## Goal

Close the two `normal` JNI issues in `crates/iscc-jni/src/lib.rs` as one step: "JNI streaming-hasher
functions dereference Java-supplied handle without null guard" and "JNI exports report a pending JVM
exception as `IllegalArgumentException`". Both live in the crate's throw helpers and four hasher
functions, so one verdict suffices.

## Alternatives Considered

- **Chosen:** both JNI issues together — same file, same helpers, independent tests; the handoff
    recommends it and the diff is one code file plus javadoc and docs.
- **Rejected:** `iscc-ffi` `lib.rs` split (move `mod tests` out) — also `normal`, but it changes no
    behaviour; the JNI pair fixes UB and a JNI-spec violation, so it goes first.

## Scope

- **Modify**: `crates/iscc-jni/src/lib.rs`;
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (javadoc only);
    `docs/java-api.md`; `docs/howto/java.md`; `crates/iscc-jni/CLAUDE.md` (Error Handling + mapping
    table row)
- **Tests**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`, plus one small
    test-source `main` class for the child JVM if needed
- **Reference**: jni 0.22.4 in `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/jni-0.22.4/`
    — `src/objects/jstring.rs` ~158 (`JString::new` catch arm), `src/env.rs` ~1139
    (`exception_check`), `src/macros.rs` ~89 (`jni_call_post_check_ex`)

## Not In Scope

- Detecting a *stale* (freed, non-zero) handle — needs a handle registry; only zero is guarded
- Changing the `*Free` functions (zero is already a no-op) or the 2026-07-28 decision (hand-thrown
    exceptions, `#[repr(transparent)]` returns stay)
- Adding `-Xcheck:jni` to CI or changing the pom's `<argLine>`; the `iscc-ffi` split; Ruby Float
    truncation (`low`)

## Implementation Notes

**The issue's suggested fix is insufficient — measured in the jni 0.22.4 source.** `JString::new`
(behind `env.new_string`) uses `jni_call_with_catch_and_null_check!`: it *catches and clears* the
JVM's `OutOfMemoryError` and returns `Error::JniCall(JniError::NoMemory)`, whose `Display` is "JNI
call failed" — exactly Codex's message. So at `throw_and_default` nothing is pending and an
`exception_check()` guard alone changes nothing. Other calls (`jni_call_post_check_ex!`, e.g.
`set_region`) return `Error::JavaException` and leave the exception pending — there the guard is
what prevents `ThrowNew` over a pending exception.

**Error mapping, by type.** 90 call sites pass `&e.to_string()` to `throw_and_default`; some `e` are
`jni::errors::Error`, some `iscc_lib::IsccError`. Make the mapping type-driven so no site can be
misclassified: e.g. a small trait (or `impl From`) for `jni::errors::Error`, `IsccError` and `&str`,
and have `throw_and_default` take it; every site then passes `e` (or `&e`), not a string. Mapping:

- any input: if `env.exception_check()` → return `Ok(T::default())` without throwing
- `jni::errors::Error::JniCall(JniError::NoMemory)` → throw `java/lang/OutOfMemoryError` (a fresh
    one — jni already discarded the original; the class is what Java callers catch)
- every other error / message → `IllegalArgumentException` with the `Display` text, as today

Add the same `exception_check` guard to `throw_state_error`. Update both doc comments, the module
`//!` "Error handling" paragraph and `crates/iscc-jni/CLAUDE.md`.

**Null guard.** In `dataHasherUpdate`, `dataHasherFinalize`, `instanceHasherUpdate`,
`instanceHasherFinalize`, first statement inside `with_env`: `if ptr == 0` → `throw_and_default`
with "DataHasher handle is null" / "InstanceHasher handle is null" (`IllegalArgumentException`;
`IllegalStateException` stays reserved for the finalized state). Reword the `// SAFETY:` comments.

**Doc drift, same step.** `docs/java-api.md` (~611, 646, 654, 699, 707), `docs/howto/java.md` (~267)
and the hasher `@throws` javadoc in `IsccLib.java` say a finalized hasher throws
`IllegalArgumentException`; code, tests and `specs/java-bindings.md` (line ~208) say
`IllegalStateException`. Fix the docs; document the zero-handle `IllegalArgumentException`; add one
line to `java-api.md` → Error Handling that allocation failure surfaces as `OutOfMemoryError`.

**Tests.** One `@Test` per hasher function: `assertThrows(IllegalArgumentException.class, …)` with
handle `0L`. OOM: never starve the surefire JVM — spawn a child JVM via `ProcessBuilder` (java from
`ProcessHandle.current().info().command()`, `-cp` from `java.class.path`, the same
`-Djava.library.path`, `-Xmx16m`, `-Xcheck:jni`) running a test-source `main` that triggers an
allocation failure inside a JNI call and prints the caught exception's class name. Codex's trigger:
`encodeSeq` over 500,000 valid units (one shared `String` instance); `decodeSeq` of a large sequence
(many `new_string`) is the fallback. Assert stdout `java.lang.OutOfMemoryError` and no
`WARNING in native method` on stderr. Confirm the test fails on the unfixed helper and say so in the
commit. If no trigger is deterministic, drop that test and argue why in the commit and CLAUDE.md
(the issue allows it).

Binding-only change: CRAP and iai are not affected.

## Verification

- `cargo build -p iscc-jni` then `mvn -f crates/iscc-jni/java/pom.xml clean test` exits 0, with the
    four null-handle tests and the child-JVM OOM test (or its documented rationale)
- With `JAVA_TOOL_OPTIONS=-Xcheck:jni` set, the same `mvn … clean test` exits 0 and its log has no
    `WARNING in native method` line (and shows `Picked up JAVA_TOOL_OPTIONS`)
- `grep -n 'e.to_string())' crates/iscc-jni/src/lib.rs` prints nothing; `grep -n 'ptr == 0'` on the
    same file shows the four guards; `exception_check` appears in both throw helpers
- `grep -l IllegalStateException` over `docs/java-api.md`, `docs/howto/java.md` and `IsccLib.java`
    lists all three, and no sentence there says a finalized hasher throws `IllegalArgumentException`
- `cargo clippy -p iscc-jni -- -D warnings` and `cargo fmt -p iscc-jni --check` clean
- `mise run check` exits 0

## Done When

All six verification criteria pass on the working tree.
