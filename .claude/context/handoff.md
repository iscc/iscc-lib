# Handoff

## 2026-07-28 — Review of: Migrate the JNI binding from jni 0.21 to jni 0.22

**Verdict:** PASS_WITH_NOTES

**Summary:** All 33 `extern "system"` exports moved cleanly onto `EnvUnowned`/`Env` with the Java
contract intact — 82/82 Maven tests, all gates green, no Java, Kotlin, fixture or core source
touched, 3 non-doc files in the diff. One avoidable regression rides along: next.md's API-mapping
table wrongly listed `Env::byte_array_from_slice` as deprecated, so the diff hand-rolls a
`build_byte_array` that adds an allocation and a full copy per returned `byte[]`. Filed, not
blocking.

**Verification:**

- [x] `jni = "0.22"` in Cargo.toml (line 49); `held:` lines are only criterion (44) and uniffi (51)
- [x] Negative grep for the eight 0.21 API names in `crates/iscc-jni/src/lib.rs` — no match (exit 1)
- [x] `cargo clippy -p iscc-jni --all-targets -- -D warnings` → 0 after touching the source;
    workspace clippy also 0 (only the known dev-only proc-macro-error2 future-incompat note)
- [x] `cargo build -p iscc-jni` + `mvn clean test -f crates/iscc-jni/java/pom.xml` → 82 tests (69 +
    13), 0 failures, matching the pre-bump total in state.md; the tracked `java/` tree stayed clean.
    `strings target/debug/libiscc_jni.so` → `jni-0.22.4`, `jni-sys-0.4.1`
- [x] `mise run audit` → advisories/bans/licenses/sources ok
- [x] `mise run check` → exit 0, 18 hooks passed, tracked tree unchanged

**Beyond next.md (2 probes):** a standalone `java -Xcheck:jni` program exercised the 7 natives the
Maven suite never calls (all correct, including `conformanceSelftest` under the new
`jboolean = bool` alias) and asserted the runtime array classes; a second probe pushed the five
rewritten `with_local_frame` sites past the ~512 local-ref limit (1025 CDC chunks, 4988 n-grams,
1000-element `String[]` and `int[][]` inputs) — zero `-Xcheck:jni` warnings from either.

**Issues found:**

- `build_byte_array` re-implements the non-deprecated `Env::byte_array_from_slice` (jni-0.22.4
    `src/env.rs:3349` carries no attribute; the note next.md read belongs to
    `set_object_array_element` two functions above). Extra `Vec<i8>` alloc + copy per call, worst on
    `algCdcChunks`. **Filed.** Every other row of next.md's mapping table checked out — those APIs
    really are deprecated or removed, so the rest of the migration was forced.
- 7 of the 33 natives have no Java test (`conformanceSelftest`, `encodeBase64`,
    `textRemoveNewlines`, `isccDecompose`, `algSimhash`, `algMinhash256`, `softHashVideoV0`) — a
    whole-crate signature rewrite went green with 26 of 33 proven. **Filed.**
- Doc drift filed as one entry: `crates/iscc-rb/CLAUDE.md:108` (magnus, observed unfiled since 167)
    and `specs/java-bindings.md` ("jni crate (v0.21)", "~1060 lines"). The spec half carries a HUMAN
    REVIEW REQUESTED marker inside the issue — human-owned file, so I did not edit it.
- Minor, not filed: `dataHasherUpdate`/`dataHasherFinalize` still dereference the `jlong` handle
    with no zero check while `*Free` has one — pre-existing, unchanged here, would segfault the JVM
    if a caller passed 0. `extract_int_array` takes `&Env` where its siblings take `&mut Env`. The
    null-`JString` error message wording changed (now names `get_string_utf_chars`); no test or doc
    asserts message text.

**Codex review:** one finding, P2 — the same `build_byte_array` copy, called out as an
"O(input-size) copy and thousands of allocations" on `algCdcChunks`. Confirmed at the source and
filed; Codex's mechanism claim is right (0.22 still ships the helper, undeprecated) even though
next.md said otherwise. No other findings.

**Next:** The dependency refresh has no Rust majors left. Two small, well-scoped candidates from
this review, in order of value: (1) the `build_byte_array` revert to `env.byte_array_from_slice`
plus its two `crates/iscc-jni/CLAUDE.md` rows — one source file, mechanically verifiable, closes a
fresh regression; (2) the seven untested JNI natives, which is the only way that crate's next API
bump gets a real gate. The doc-drift entry is a reasonable third. Beyond those, the `release.yml`
action-freshness pass is the last CID-doable dependency item (static evidence only, per the
2026-07-25 decision); everything else in issues.md is `low` or human-gated.

**Notes:**

- `jni-sys` 0.4 aliases `jboolean` to Rust `bool`, so the `!= 0` conversions are gone from three
    parameters (`wide`, `addUnits`, `utf32`) and one return. Java-invisible, but it makes the JVM's
    boolean normalization load-bearing for soundness; the alias is upstream's decision and the probe
    confirms `conformanceSelftest` still returns true.
- The exception contract is verified at the source, not just behaviourally:
    `ThrowRuntimeExAndDefault::on_error` calls `Env::exception_check` first (jni-0.22.4
    `src/errors/policy.rs:188`) and returns the default without throwing when an exception is
    pending. `on_panic` does **not** check — irrelevant here (release builds are `panic = "abort"`).
    Recorded in decisions.md.
- `.claude/context/specs/java-bindings.md` is now the only place still claiming jni v0.21; no crate
    README, docs page or test teaches a 0.21 API.
- Still observed and unfiled (update-state has carried it twice): `packages/dotnet` is the only
    ecosystem without a lockfile, its two test packages floating on `3.*` and `18.*`. Left for the
    audit pass at iteration 170 rather than filed here.
