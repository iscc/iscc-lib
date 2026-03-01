# Next Work Package

## Step: Export META_TRIM_META in Java and Go bindings

## Goal

Add the `META_TRIM_META = 128_000` constant to Java (`IsccLib.java`) and Go (`codec.go`) bindings,
completing issue #18 — the last two bindings that lack this Tier 1 constant.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — add
        `public static final int META_TRIM_META = 128_000;`
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — add `META_TRIM_META`
        assertion to `testConstants()` and update comment from "4" to "5"
    - `packages/go/codec.go` — add `MetaTrimMeta = 128_000` to the const block
- **Reference**:
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (existing constant pattern at
        lines 22-31)
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (existing test pattern at
        lines 398-407)
    - `packages/go/codec.go` (existing const block at lines 16-21)

## Not In Scope

- Adding a JNI native method for META_TRIM_META (it's a compile-time constant — no JNI call needed)
- Adding Go tests for the constant (Go has no constant-value tests for the existing 4 constants;
    adding one just for this would break the pattern — keep consistent)
- Updating documentation pages, READMEs, or the docs site (wait until gen_sum_code_v0 is done)
- Starting work on gen_sum_code_v0 or SumCodeResult (issue #15 — separate step)
- Updating `crates/iscc-jni/src/lib.rs` Rust JNI source (no JNI bridge needed for a Java constant)

## Implementation Notes

**Java** — follow the exact pattern of `META_TRIM_DESCRIPTION`:

```java
/** Maximum decoded payload byte length for the meta parameter in gen_meta_code_v0. */
public static final int META_TRIM_META = 128_000;
```

Place it immediately after `META_TRIM_DESCRIPTION` (line 25). In `IsccLibTest.java`, add
`assertEquals(128_000, IsccLib.META_TRIM_META);` to the `testConstants()` method (after line 404),
and update the Javadoc from "4 algorithm configuration constants" to "5 algorithm configuration
constants".

**Go** — follow the exact pattern of `MetaTrimDescription`:

```go
MetaTrimMeta        = 128_000
```

Place it after `MetaTrimDescription` in the existing `const` block (after line 18). The underscore
numeric separator is valid Go syntax (Go 1.13+).

Both are pure source-language constants (no FFI/JNI bridge involved).

## Verification

- `cargo test -p iscc-jni` passes (existing tests still compile and pass)
- `cd crates/iscc-jni && mvn test -q` passes (including the updated `testConstants` with 5
    assertions)
- `cd packages/go && go test ./... -count=1` passes (147+ tests, MetaTrimMeta is available)
- `cd packages/go && go vet ./...` clean
- `cargo clippy -p iscc-jni -- -D warnings` clean
- `grep -c 'META_TRIM_META' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
    outputs `1` (constant declared)
- `grep -c 'MetaTrimMeta' packages/go/codec.go` outputs `1` (constant declared)

## Done When

All verification criteria pass — META_TRIM_META is exported and tested in both Java and Go bindings,
completing the 6/6 binding propagation for issue #18.
