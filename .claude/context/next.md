# Next Work Package

## Step: Expose `add_units`/`units` in Java/JNI binding (issue #21)

## Goal

Add `addUnits` parameter and `units` field to the Java/JNI binding for `genSumCodeV0`, so Java
callers can request the individual Data-Code and Instance-Code ISCC strings — matching the Rust
core, Python, Node.js, WASM, and C FFI bindings that already have this support. This continues issue
#21 binding propagation (Rust ✅ → Python ✅ → Node.js ✅ → WASM ✅ → C FFI ✅ → **JNI** → Go).

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-jni/src/lib.rs` — add `add_units: jboolean` param, pass to Rust core, convert
        `Option<Vec<String>>` → nullable `jobjectArray` via `build_string_array`, update constructor
        signature
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/SumCodeResult.java` — add
        `public final String[] units` field (nullable), update constructor to accept 4th param
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — add `boolean addUnits`
        param to native method declaration, update Javadoc
- **Reference**:
    - `crates/iscc-jni/src/lib.rs` lines 119–135 (`build_string_array` helper)
    - `crates/iscc-jni/src/lib.rs` lines 401–446 (current `genSumCodeV0` JNI function)
    - `crates/iscc-jni/src/lib.rs` lines 685–702 (`isccDecompose` — example of returning
        `jobjectArray`)
    - `crates/iscc-ffi/src/lib.rs` — C FFI units pattern for reference
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — existing tests

## Not In Scope

- Go binding `add_units`/`units` support (that's the next step after this)
- Documentation updates to `docs/rust-api.md` or `docs/architecture.md` (deferred until all bindings
    complete)
- Refactoring existing Java tests or renaming test methods
- Adding overloaded Java methods for backward compat — just extend the native signature

## Implementation Notes

### Java side

**`SumCodeResult.java`**: Add `public final String[] units` as 4th field. Constructor signature
becomes `(String iscc, String datahash, long filesize, String[] units)`. The `units` field is
nullable — `null` when `addUnits=false`, a 2-element `String[]` containing
`[Data-Code ISCC, Instance-Code ISCC]` when true.

**`IsccLib.java`**: Change native declaration to:

```java
public static native SumCodeResult genSumCodeV0(String path, int bits, boolean wide, boolean addUnits);
```

Update the Javadoc to mention `addUnits` and `units`.

### Rust JNI side

**`lib.rs`**: The `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` function needs:

1. Add `add_units: jboolean` parameter after `wide`
2. Pass `add_units != 0` as 4th arg to `iscc_lib::gen_sum_code_v0`
3. Convert `result.units`:
    - `Some(units)` → call `build_string_array(&mut env, &units)` to get `jobjectArray`, wrap in
        `JValue::Object` (needs unsafe `JObject::from_raw`)
    - `None` → pass `JValue::Object(&JObject::null())`
4. Update constructor signature from `"(Ljava/lang/String;Ljava/lang/String;J)V"` to
    `"(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V"`
5. Add the units `JValue` as 4th constructor argument

The `build_string_array` helper already exists at line 120 and is used by `isccDecompose` and
`slidingWindow`. Reuse it directly. Note that `build_string_array` returns a `jobjectArray` (raw
pointer) — wrap it with `unsafe { JObject::from_raw(arr) }` to pass as `JValue::Object`.

### Existing test updates

4 existing Maven tests call `genSumCodeV0` with 3 args — update all to pass `false` as 4th arg:

- `genSumCodeV0Equivalence`
- `genSumCodeV0ResultFields`
- `genSumCodeV0NonExistentFile`
- `genSumCodeV0WideMode`

### New Maven tests (3)

1. **`genSumCodeV0UnitsEnabled`** — call with `addUnits=true`, assert `result.units != null`,
    `result.units.length == 2`, each element starts with `"ISCC:"`
2. **`genSumCodeV0UnitsDisabled`** — call with `addUnits=false`, assert `result.units == null`
3. **`genSumCodeV0UnitsContent`** — call with `addUnits=true`, verify `units[0]` starts with
    `"ISCC:GA"` (Data-Code) and `units[1]` starts with `"ISCC:IA"` (Instance-Code)

## Verification

- `cargo test -p iscc-jni` passes (all Rust-side JNI tests)
- `cargo clippy -p iscc-jni -- -D warnings` clean
- `cd crates/iscc-jni/java && mvn test` passes (62 existing + 3 new = 65 tests)
- `cargo build -p iscc-jni` succeeds

## Done When

All verification criteria pass: JNI bridge compiles, Rust clippy clean, and Maven tests (including 3
new units tests) all pass with `addUnits` parameter and `units` field properly exposed.
