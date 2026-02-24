# Next Work Package

## Step: Replace unwrap() calls in JNI entrypoints with fallible error handling

## Goal

Eliminate all 21 `unwrap()` calls in `extern "system"` JNI functions that can panic across the FFI
boundary and abort the JVM process. Replace each with `throw_and_default` error handling that throws
a Java exception and returns a safe default value. This addresses the critical issue "iscc-jni:
`unwrap()` calls in JNI entrypoints can panic across FFI boundary" from issues.md.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-jni/src/lib.rs`
- **Reference**: issues.md (critical issue description),
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (existing conformance
    tests)

## Not In Scope

- Adding `std::panic::catch_unwind` as a safety net — that's a separate hardening step
- Adding `push_local_frame`/`pop_local_frame` for local reference management (separate issue)
- Validating `jint` negative values before casting (separate normal-priority issue)
- Changing exception types (e.g., `IllegalStateException` for finalized hashers) — separate low
    issue
- Adding new Java-side tests that deliberately trigger JNI allocation failures (difficult to test
    from Java; the Rust-side fix is mechanically verifiable)

## Implementation Notes

There are exactly 21 `unwrap()` calls to replace, falling into three patterns:

**Pattern A — `env.new_string(...).unwrap().into_raw()`** (16 occurrences): Lines 167, 187, 207,
228, 249, 271, 291, 311, 333, 356, 373, 392, 411, 430, 665, 744. Replace with:

```rust
match env.new_string(result.iscc) {
    Ok(s) => s.into_raw(),
    Err(e) => throw_and_default(&mut env, &e.to_string()),
}
```

**Pattern B — `env.byte_array_from_slice(...).unwrap().into_raw()`** (3 occurrences): Lines 516,
538, 589. Replace with:

```rust
match env.byte_array_from_slice(&result) {
    Ok(a) => a.into_raw(),
    Err(e) => throw_and_default(&mut env, &e.to_string()),
}
```

**Pattern C — `unwrap()` in loop body** (2 occurrences in `algCdcChunks`, lines 567-568):

```rust
// Before:
let barr = env.byte_array_from_slice(chunk).unwrap();
env.set_object_array_element(&arr, i as i32, &barr).unwrap();

// After:
let barr = match env.byte_array_from_slice(chunk) {
    Ok(a) => a,
    Err(e) => return throw_and_default(&mut env, &e.to_string()),
};
if let Err(e) = env.set_object_array_element(&arr, i as i32, &barr) {
    return throw_and_default(&mut env, &e.to_string());
}
```

All replacements use the existing `throw_and_default` helper — no new error-handling infrastructure
needed. The fix is mechanical and uniform.

## Verification

- `grep -c 'unwrap()' crates/iscc-jni/src/lib.rs` returns `0` (all unwrap calls eliminated)
- `cargo clippy -p iscc-jni -- -D warnings` passes
- `cargo build -p iscc-jni` succeeds
- `cd crates/iscc-jni/java && mvn test` passes all 46 conformance vectors (no behavioral change)

## Done When

All four verification commands pass — zero `unwrap()` calls remain in the JNI crate, clippy is
clean, and all conformance tests still pass.
