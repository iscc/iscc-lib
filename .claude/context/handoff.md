## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the JNI (Java) surface

**Verdict:** NEEDS_WORK

**Summary:** `genIsccIdV1` mints the golden ISCC correctly, throws on invalid realm, round-trips
through `isccDecode` at Version 1, and all cargo + mvn + prek gates are green. But the wrapper's
input validation violates the spec's **normative cross-surface validation order** (rust-core.md
§Validation, criterion ~L542): it uses wide narrowing guards (hub 0-65535, realm 0-255) and skips
the timestamp check, so for multi-invalid inputs it reports the wrong field. This breaks the #43
drop-in-compatibility contract on a v0.6.0 release blocker.

**Verification:**

- [x] `cargo build -p iscc-jni` clean; `cargo clippy -p iscc-jni --all-targets -- -D warnings`
    clean; `cargo fmt -p iscc-jni --check` clean.
- [x] `mvn clean test` — 96 tests, 0 failures (83 IsccLibTest incl. 3 new + 13 UnicodeBoundaryTest).
- [x] `mise run check` — all prek pre-commit hooks Passed.
- [x] (probe) Golden `ISCC:MAIGHFECJMOPMIAB` for `(1751831876325218, 1, 0)` — matches the live
    `iscc-core` oracle (`uv run --with iscc-core`) and the passing `genIsccIdV1Golden` test.
- [ ] **Validation order normative on every surface (spec ~L542)** — FAILS for multi-invalid inputs
    (see Issues). Single-invalid-field cases fire correctly; the prescribed tests don't exercise
    ordering.

**Issues found:**

- **Validation order violates the normative spec.** rust-core.md §Validation: checks fire
    ts→hub→realm, "first failing check wins", asserted cross-surface. The wrapper guards `hubId`
    0-65535 then `realm` 0-255 (wide narrowing bounds) and leaves the timestamp check to core, so:
    `genIsccIdV1(1<<52, 65536, 0)` reports hub, reference reports **timestamp**;
    `genIsccIdV1(0, 4096, 256)` reports realm, reference reports **hub**. Fix: mirror napi
    (`crates/iscc-napi/src/lib.rs:329-331`) — validate the three **semantic** thresholds
    (`2^52`/`4096`/`2`) in ts→hub→realm order in the wrapper *before* narrowing, then cast. Add a
    multi-invalid-input test asserting the winning field. Existing tests stay green (realm=2 is
    still rejected). This is the sole NEEDS_WORK cause — no issues.md entry; the next cycle fixes
    it.

**Codex review:** [P2] flagged exactly this ordering divergence with the same two counterexamples —
confirmed against core (`lib.rs:1064-1075`, ts→hub→realm) and the normative spec. Not advisory-only
here: it corroborates a real spec-conformance gap.

**Next:** Redo the JNI IDv1 slice with napi-style ordered semantic validation (ts→hub→realm, bounds
`2^52`/`4096`/`2`) before narrowing, plus a multi-invalid-input ordering test. Then resume the #43
fan-out on **rb** (Magnus) — carry the same lesson: any wide-int binding surface (rb/dotnet if they
take wider-than-needed ints) must validate the three semantic thresholds in order in the binding;
Go/ffi are exempt (exact-width types). After the fan-out, the deferred Tier-1 32→33 doc/count sweep.

**Notes:** No API break, no hot path touched, scope was clean (3 files, within budget). The output
contract for valid inputs is correct; only the error-ordering contract is broken. napi/wasm are the
correct precedent (decisions.md 2026-07-29 "validate before narrowing"). Not pushed — NEEDS_WORK.
