## 2026-07-29 — Redo JNI `gen_iscc_id_v1` with ordered semantic validation

**Done:** Replaced the wide-narrowing guard block in the JNI `genIsccIdV1` wrapper with three
semantic-threshold checks in the normative ts→hub→realm order (first failing check wins), mirroring
the napi `checked(v, thresh, name)` precedent. Added one multi-invalid ordering test. Completes the
7th of 11 IDv1 fan-out surfaces (issue #43, v0.6.0 blocker).

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: `genIsccIdV1` now validates `timestamp` (`0..2^52`), then `hubId`
    (`0..4096`), then `realm` (`0..2`) BEFORE narrowing to `(u64,u16,u8)`. Messages name the field
    (`"timestamp"`, `"hubId (hub)"`, `"realm"`). Doc comment rewritten to describe the ordered
    checks.
- `crates/iscc-jni/java/.../IsccLibTest.java`: added `genIsccIdV1ValidationOrder` — asserts
    `(1L<<52, 65536, 0)` throws naming **timestamp** (not hub) and `(0L, 4096, 256)` throws naming
    **hub** (not realm).

**Verification:**

- `cargo fmt -p iscc-jni --check` clean; `cargo clippy -p iscc-jni --all-targets -- -D warnings`
    clean.
- `cargo build -p iscc-jni && mvn clean test` — 97 tests, 0 failures (84 IsccLibTest incl. the new
    ordering test + golden/realm/round-trip; 13 UnicodeBoundaryTest). Golden still mints
    `ISCC:MAIGHFECJMOPMIAB` and round-trips at Version 1.
- `mise run check` — all prek pre-commit hooks Passed (exit 0).

**Next:** Resume the #43 fan-out on **rb** (Magnus). Carry the same lesson: any wide-int binding
surface (rb/dotnet if they take wider-than-needed ints) must validate the three semantic thresholds
(`2^52`/`4096`/`2`) in ts→hub→realm order in the binding before narrowing. Go/ffi are exempt
(exact-width types). Remaining surfaces after rb: uniffi (Swift/Kotlin), dotnet C# consumer, cpp.
Then the deferred Tier-1 32→33 doc/count sweep.

**Notes:** No API break, no hot path touched. Scope clean: 1 core file (budget 3) + 1 test file
(excluded). The `hubId (hub)` message wording keeps a distinct substring per field while satisfying
the test's `contains("hub")` assertion. No `IsccDecodeResult` change — JNI `isccDecode` already
returns `version` as a plain int, so the round-trip works without enum widening.
