## 2026-03-02 — Expose `add_units`/`units` in Java/JNI binding (issue #21)

**Done:** Added `addUnits` boolean parameter to `genSumCodeV0` and `String[] units` field to
`SumCodeResult` across all three JNI layers: Java class (`SumCodeResult.java`), native declaration
(`IsccLib.java`), and Rust JNI bridge (`lib.rs`). The Rust side converts `Option<Vec<String>>` to a
nullable `jobjectArray` using the existing `build_string_array` helper, wrapped with
`unsafe { JObject::from_raw(arr) }`. Updated 4 existing test calls to pass `false` as the new 4th
argument and added 3 new tests verifying units enabled/disabled/content.

**Files changed:**

- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/SumCodeResult.java`: Added
    `public final String[] units` field (nullable), updated constructor to accept 4th
    `String[] units` param, updated Javadoc
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`: Changed native method
    signature from 3 params to 4 (`boolean addUnits`), updated Javadoc
- `crates/iscc-jni/src/lib.rs`: Added `add_units: jboolean` parameter, passes `add_units != 0` to
    core, converts `result.units` to nullable `jobjectArray` via `build_string_array`, updated
    constructor signature to `(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: Updated 4 existing tests
    to pass `false` as 4th arg, added 3 new tests (`genSumCodeV0UnitsEnabled`,
    `genSumCodeV0UnitsDisabled`, `genSumCodeV0UnitsContent`)

**Verification:**

- `cargo build -p iscc-jni` — success
- `cargo clippy -p iscc-jni -- -D warnings` — clean, no warnings
- `cargo test -p iscc-jni` — 0 Rust-side tests (no JVM in test env), passes
- `cd crates/iscc-jni/java && mvn test` — 65 tests run, 0 failures, 0 errors (62 existing + 3 new)
- `mise run check` — 14/14 pre-commit hooks passed

**Next:** Continue issue #21 binding propagation. Next target is Go (`packages/go`): add `addUnits`
parameter to `GenSumCodeV0` and `Units` field to `SumCodeResult`. This is a pure Go reimplementation
(not a Rust FFI wrapper), so the Go function needs to collect the Data-Code and Instance-Code ISCC
strings during `GenSumCodeV0` execution and return them in the result struct.

**Notes:** Issue #21 progress: Rust core ✅ → Python ✅ → Node.js ✅ → WASM ✅ → C FFI ✅ → JNI ✅ → Go
(next). One binding remains. The `unsafe { JObject::from_raw(arr) }` in the units conversion is the
same pattern used by `isccDecompose` — the raw pointer comes from `build_string_array` which returns
a valid `jobjectArray`.
