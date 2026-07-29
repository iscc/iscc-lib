## 2026-07-29 — Review of: Redo JNI `gen_iscc_id_v1` with ordered semantic validation

**Verdict:** PASS

**Summary:** The iteration-183 NEEDS_WORK is resolved. The JNI `genIsccIdV1` wrapper now validates
the three semantic thresholds (`2^52`/`4096`/`2`) in the normative ts→hub→realm order *before*
narrowing to `(u64,u16,u8)`, mirroring the napi precedent. One multi-invalid ordering test was
added. Completes the 7th of 11 IDv1 fan-out surfaces (#43).

**Verification:**

- [x] `cargo fmt -p iscc-jni --check` clean; `cargo clippy -p iscc-jni --all-targets -- -D warnings`
    clean — re-run here, both clean.
- [x] `cargo build -p iscc-jni && mvn clean test` — 97 tests, 0 failures (84 IsccLibTest incl. new
    ordering test + golden/realm/round-trip; 13 UnicodeBoundaryTest).
- [x] New ordering test proves `(1L<<52, 65536, 0)`→timestamp and `(0L, 4096, 256)`→hub — asserted
    in `genIsccIdV1ValidationOrder`.
- [x] `mise run check` — all prek pre-commit hooks Passed.
- [x] (probe) Reference order confirmed at `iscc_id.py:127-133`: `timestamp>=2^52` → `hub_id>=2^12`
    → `realm_id not in (0,1)`, first-raise wins — exactly the wrapper's order. Golden
    `ISCC:MAIGHFECJMOPMIAB` still mints (covered by passing tests).

**Issues found:**

- (none) — scope clean: 1 core file (budget 3) + 1 excluded test file. No API break, no hot path.
    The wrapper additionally rejects negative `timestamp` (the ref, on unbounded Python ints, never
    sees one) — correct hardening, not a divergence.

**Codex review:** Clean — "enforces the specified timestamp→hub→realm validation order with correct
semantic bounds before narrowing. Rust checks and all 97 Java tests pass." No findings.

**Next:** Resume the #43 fan-out on **rb** (Magnus). Carry the wide-int lesson: any binding surface
taking wider-than-needed ints (rb/dotnet) must validate the three semantic thresholds
(`2^52`/`4096`/`2`) in ts→hub→realm order in the binding *before* narrowing — Go/ffi are exempt
(exact-width types). Remaining #43 surfaces after rb: uniffi (Swift/Kotlin), dotnet C# consumer,
cpp. Then the deferred Tier-1 32→33 doc/count sweep.

**Notes:** #43 remains open (v0.6.0 blocker) — 4 minting surfaces + doc sweep left. No decision
entry needed: this applies the established "validate before narrowing" pattern (decisions.md
2026-07-29), not a new trade-off.
