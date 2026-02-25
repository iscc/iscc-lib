# Next Work Package

## Step: JNI IllegalStateException for finalized hashers

## Goal

Fix the Java binding to throw `IllegalStateException` (not `IllegalArgumentException`) when calling
`update()` or `finalize()` on an already-finalized `DataHasher` or `InstanceHasher`. This makes the
Java API idiomatic — Java convention uses `IllegalStateException` for operations invalid in the
current object state, vs `IllegalArgumentException` for invalid input values.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-jni/src/lib.rs` (add `throw_state_error` helper, change 4 call sites,
    update 2 doc comments)
- **Reference**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (existing
    test patterns), `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (Java-side
    declarations)

## Not In Scope

- Changing exception types for non-state errors (all other `throw_and_default` call sites remain
    `IllegalArgumentException` — those ARE argument validation errors)
- Adding `IllegalStateException` to non-hasher functions (none of them have state)
- Refactoring the `throw_and_default` function signature to accept exception class as parameter
    (keep it simple with a separate helper)
- Updating `crates/iscc-wasm/CLAUDE.md` stale text (separate [low] issue)
- Release preparation or PR creation

## Implementation Notes

Add a `throw_state_error` helper alongside the existing `throw_and_default`:

```rust
fn throw_state_error<T: Default>(env: &mut JNIEnv, msg: &str) -> T {
    let _ = env.throw_new("java/lang/IllegalStateException", msg);
    T::default()
}
```

Change exactly 4 call sites from `throw_and_default` to `throw_state_error` — the "already
finalized" messages in:

1. `dataHasherUpdate` (line ~739) — `throw_and_default::<()>` → `throw_state_error::<()>`
2. `dataHasherFinalize` (line ~760) — `throw_and_default` → `throw_state_error`
3. `instanceHasherUpdate` (line ~821) — `throw_and_default::<()>` → `throw_state_error::<()>`
4. `instanceHasherFinalize` (line ~842) — `throw_and_default` → `throw_state_error`

Update the 2 doc comments on `dataHasherUpdate` and `instanceHasherUpdate` from "Throws
`IllegalArgumentException`" to "Throws `IllegalStateException`".

Add 2 Java test methods to `IsccLibTest.java`:

1. `testDataHasherThrowsIllegalStateAfterFinalize` — create hasher, finalize, then call update and
    assert `IllegalStateException`
2. `testInstanceHasherThrowsIllegalStateAfterFinalize` — same pattern for InstanceHasher

These follow the existing negative test pattern (e.g., `testTextTrimNegativeThrows` at line 343).

## Verification

- `cd crates/iscc-jni && cargo build` succeeds (Rust JNI crate compiles)
- `cd crates/iscc-jni/java && mvn test` passes (all existing 49 tests + 2 new = 51 tests)
- `grep -c 'throw_state_error' crates/iscc-jni/src/lib.rs` outputs `4` (exactly 4 call sites)
- `grep -c 'IllegalStateException' crates/iscc-jni/src/lib.rs` outputs at least `1` (the helper)
- `grep -c 'IllegalStateException' crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
    outputs at least `2` (the 2 new tests)
- `cargo clippy -p iscc-jni -- -D warnings` clean

## Done When

All verification criteria pass: the JNI crate compiles, Maven tests pass (51 total), exactly 4
state-error call sites use `IllegalStateException`, and clippy is clean.
