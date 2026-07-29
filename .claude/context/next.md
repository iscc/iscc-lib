# Next Work Package

## Step: Redo JNI `gen_iscc_id_v1` with ordered semantic validation

## Goal

**Reframe** of iteration 183's JNI slice (first NEEDS_WORK — same goal, corrected validation design,
not a backtrack). Fix the `genIsccIdV1` wrapper so its input validation obeys the normative
cross-surface order (ts→hub→realm, first failing check wins) for multi-invalid inputs, completing
the 7th of 11 IDv1 fan-out surfaces. Live v0.6.0 blocker (issue #43, "ISCC-IDv1 unsupported outside
Go"). The Java native decl, golden/realm/round-trip tests already exist locally (commit `8201277`,
unpushed); only the Rust validation and one ordering test are owed.

## Alternatives Considered

- **Chosen:** JNI redo — an unpushed NEEDS_WORK slice on a release blocker; it must be resolved
    before any new surface, else broken code strands the fan-out.
- **Rejected:** advance to the Ruby (Magnus) surface — the handoff's *subsequent* step, but starting
    it now leaves the JNI ordering bug unfixed and unpushed. Finish the in-flight slice first.

## Scope

- **Modify**: `crates/iscc-jni/src/lib.rs` — replace the wide-narrowing guard block in
    `Java_io_iscc_iscc_1lib_IsccLib_genIsccIdV1` with three semantic-threshold checks in
    ts→hub→realm order; update the fn doc comment to describe the ordered checks.
- **Modify (test, excluded from budget)**:
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — add one multi-invalid
    ordering test.
- **Reference**: `crates/iscc-napi/src/lib.rs:328-335` (the `checked(v, thresh, name)` precedent),
    `.claude/context/specs/rust-core.md` §Validation (~L542, normative order), learnings.md "IDv1
    validation ORDER is normative".

## Not In Scope

- The other 4 surfaces (rb, uniffi→Swift/Kotlin, dotnet C# consumer, cpp) and the Tier-1 32→33 doc
    sweep — each its own #43 slice.
- Any enum-widening or decode change: JNI `isccDecode` returns `version` as a plain `int`, so the
    round-trip already works — do NOT touch `IsccDecodeResult`.
- Changing the golden/realm/round-trip tests already present — keep them; only ADD the ordering
    test.

## Implementation Notes

- **The bug (183):** the wrapper guards `hubId` in `0..=65535` then `realm` in `0..=255` (wide
    narrowing bounds) and skips the timestamp check, deferring semantic ranges to core. For
    multi-invalid inputs this reports the wrong field: `(1L<<52, 65536, 0)` reports hub (ref:
    timestamp); `(0, 4096, 256)` reports realm (ref: hub).
- **Fix:** validate the three SEMANTIC thresholds IN the wrapper, in order, BEFORE narrowing —
    mirroring napi. In ts→hub→realm order, `throw_and_default` when:
    `timestamp < 0 || timestamp >= 4_503_599_627_370_496` (2^52), then `hubId < 0 || hubId >= 4096`,
    then `realm < 0 || realm >= 2`. Only after all three pass, cast to `(u64, u16, u8)` and call
    `iscc_lib::gen_iscc_id_v1`. This subsumes core's checks so single-invalid cases (realm=2) stay
    rejected and existing tests stay green.
- Message text must name the field so the ordering test can assert it — keep a distinct substring
    per field (e.g. contains `"timestamp"`, `"hub"`, `"realm"` respectively).
- **New ordering test** (JUnit `@Test`): assert `genIsccIdV1(1L<<52, 65536, 0)` throws with a
    message naming **timestamp** (not hub), and `genIsccIdV1(0L, 4096, 256)` throws with a message
    naming **hub** (not realm). Use `assertThrows(...).getMessage()` + `assertTrue(contains(...))`.

## Verification

- `cargo clippy -p iscc-jni --all-targets -- -D warnings` clean; `cargo fmt -p iscc-jni --check`
    clean.
- `cargo build -p iscc-jni && cd crates/iscc-jni/java && mvn clean test` passes — prior
    golden/realm/ round-trip tests plus the new ordering test green (surefire sets
    `java.library.path` to `target/debug`).
- New ordering test proves `(1L<<52, 65536, 0)`→timestamp and `(0L, 4096, 256)`→hub.
- `mise run check` — all prek pre-commit hooks pass.

## Done When

`genIsccIdV1` validates ts→hub→realm before narrowing, the multi-invalid ordering test asserts the
correct winning field, the golden still mints `ISCC:MAIGHFECJMOPMIAB` and round-trips at Version 1,
and all cargo + mvn + prek gates pass.
