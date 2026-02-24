# Next Work Package

## Step: JNI jint validation and local reference frame safety

## Goal

Harden the JNI bridge against two classes of robustness bugs: (1) negative `jint` values silently
producing wrong results when cast to unsigned types, and (2) JNI local reference table overflow in
loops that create references per iteration without cleanup. Both are in `crates/iscc-jni/src/lib.rs`
and together bring the JNI bindings to production-grade safety.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-jni/src/lib.rs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- **Reference**: `.claude/context/issues.md` (issue descriptions for `jint` validation and local
    reference overflow)

## Not In Scope

- Changing `throw_and_default` to support `IllegalStateException` for state errors — that's a
    separate low-priority issue
- Adding validation for `bits` parameters in `gen_*_v0` JNI functions — Rust-side validation already
    catches invalid values via `IsccResult::Err`, so the JNI cast produces a Rust error rather than
    silent wrong behavior
- Adding `softHashVideoV0` `bits` validation — same reasoning as above, Rust validates internally
- Refactoring the `throw_and_default` helper or JNI error handling pattern
- Any changes to the Java wrapper class `IsccLib.java` (native declarations don't need changing)

## Implementation Notes

### jint negative value validation (3 call sites)

Add a `>= 0` guard before the cast in each function. Throw `IllegalArgumentException` with a
descriptive message via `throw_and_default`:

1. **`textTrim`** (line 424): Before `nbytes as usize`, check `nbytes < 0` → throw
    `"nbytes must be non-negative"`
2. **`slidingWindow`** (line 518): Before `width as usize`, check `width < 0` → throw
    `"width must be non-negative"` (note: Rust `sliding_window` separately validates `width >= 2`)
3. **`algCdcChunks`** (line 605): Before `avg_chunk_size as u32`, check `avg_chunk_size < 0` → throw
    `"avg_chunk_size must be non-negative"`

Pattern for each:

```rust
if nbytes < 0 {
    return throw_and_default(&mut env, "nbytes must be non-negative");
}
```

### JNI local reference frame safety (5 loops)

Wrap each loop body in `env.push_local_frame(N)` / `env.pop_local_frame(&JObject::null())` to
release per-iteration local references. In all 5 loops, the data is copied to Rust-owned types
(Vec<i32>, String, Vec<u8>) or set into a JNI array before the iteration ends, so the local refs are
safely released.

Use a frame capacity of 16 (generous for the 1-3 refs created per iteration):

1. **`extract_int_array_2d`** (line 68-76): `get_object_array_element` creates 1 ref per iteration
2. **`extract_string_array`** (line 85-93): `get_object_array_element` + `get_string` = 2 refs
3. **`build_string_array`** (line 104-108): `new_string` = 1 ref per iteration
4. **`algSimhash`** (line 545-554): `get_object_array_element` = 1 ref per iteration
5. **`algCdcChunks`** (line 614-621): `byte_array_from_slice` = 1 ref per iteration

Pattern for helper functions (return `Result`):

```rust
for i in 0..num {
    env.push_local_frame(16).map_err(|e| e.to_string())?;
    // ... existing loop body ...
    env.pop_local_frame(&JObject::null()).map_err(|e| e.to_string())?;
}
```

For `algSimhash` and `algCdcChunks` (which use `throw_and_default` not `?`), call `push_local_frame`
at the start of each iteration and `pop_local_frame` before continuing to the next iteration. On
early-return error paths, the frame is cleaned up by the JVM when the native method returns, so no
explicit pop is needed on error.

### Java tests

Add new test methods in `IsccLibTest.java` that verify negative jint values throw
`IllegalArgumentException`:

- `textTrim("hello", -1)` → throws `IllegalArgumentException`
- `slidingWindow("hello", -1)` → throws `IllegalArgumentException`
- `algCdcChunks(data, false, -1)` → throws `IllegalArgumentException`

Use JUnit 5 `assertThrows(IllegalArgumentException.class, ...)` for each.

## Verification

- `cargo test -p iscc-jni` passes (all existing Rust unit tests)
- `cargo clippy -p iscc-jni -- -D warnings` clean
- `cd crates/iscc-jni/java && mvn test` passes (existing 46 conformance vectors + 3 new negative
    value tests)
- `grep -c 'push_local_frame' crates/iscc-jni/src/lib.rs` returns 5 (one per loop)
- `grep -c 'pop_local_frame' crates/iscc-jni/src/lib.rs` returns 5 (one per loop)
- `grep -cP 'nbytes < 0|width < 0|avg_chunk_size < 0' crates/iscc-jni/src/lib.rs` returns 3

## Done When

All verification criteria pass: Rust tests, clippy, Java conformance + negative-value exception
tests all green, and all 5 loops use push/pop local frame.
