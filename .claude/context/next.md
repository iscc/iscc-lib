# Next Work Package

## Step: Restore the zero-copy JNI byte-array helper and test the seven untested natives

## Goal

Close two `normal` issues left by iteration 168 in one pass: replace the hand-rolled
`build_byte_array` (extra `Vec<i8>` alloc + copy per returned `byte[]`) with jni 0.22's
non-deprecated `Env::byte_array_from_slice`, and give the seven never-tested JNI natives — three of
which are exactly the `byte[]`-returning sites this step edits — their first JUnit coverage so the
change is guarded rather than hand-verified.

## Alternatives Considered

- **Chosen:** the two JNI issues together — the byte-array revert touches five return sites, three
    of them (`algSimhash`, `algMinhash256`, `softHashVideoV0`) with no Java test at all; landing the
    fix without the tests would repeat 168's "green suite, unproven natives" pattern.
- **Rejected:** the `release.yml` action-freshness pass (last CID-doable dependency item) — static
    evidence only, no CI exercises it, and it would be the fourth workflow/tooling step in a row
    while a fresh performance regression in a published binding sits open.

## Scope

- **Modify**: `crates/iscc-jni/src/lib.rs` (only non-doc, non-test file)
- **Modify**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (tests)
- **Modify**: `crates/iscc-jni/CLAUDE.md` (the type-mapping row and the pitfall bullet that teach
    `build_byte_array`)
- **Reference**: `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` (covers all seven functions with
    concrete inputs and expected values — the sibling template),
    `crates/iscc-lib/tests/test_algorithm_primitives.rs`, `tests/test_algo.py`,
    `~/.cargo/registry/src/*/jni-0.22.4/src/env.rs` around line 3349

## Not In Scope

- `.claude/context/specs/java-bindings.md` — human-owned, carries a HUMAN REVIEW REQUESTED marker;
    `crates/iscc-rb/CLAUDE.md:108` (the other half of the doc-drift issue) also stays for a later
    step.
- The zero-handle deref in `dataHasherUpdate`/`dataHasherFinalize` and the `&Env` vs `&mut Env`
    asymmetry in `extract_int_array` — pre-existing, unfiled, leave untouched.
- No Kotlin/UniFFI tests, no `iscc-lib` core changes, no dependency bumps, no reshaping of
    `build_string_array` / `extract_*` helpers.
- Do not edit `issues.md` — the review agent resolves issues.

## Implementation Notes

**Byte-array fix.** `Env::byte_array_from_slice(&mut self, buf: &[u8]) -> Result<JByteArray>` is
present and un-deprecated in jni-0.22.4 (`src/env.rs:3349`); it does `JByteArray::new` +
`set_region` over a transmuted `&[i8]`, so no allocation. Delete `build_byte_array` and call
`env.byte_array_from_slice(...)` at all five sites: `isccDecode` (~739), `algSimhash` (~865),
`algMinhash256` (~894), `algCdcChunks` (~944, inside the per-chunk `with_local_frame` — the
closure's `env` carries the same `'local` lifetime, so the substitution is direct), and
`softHashVideoV0` (~974). Keep each site's existing error shape
(`match … => throw_and_default(env, &e.to_string())`).

**Tests.** Add `@Test` methods to `IsccLibTest.java` for `conformanceSelftest`, `encodeBase64`,
`textRemoveNewlines`, `isccDecompose`, `algSimhash`, `algMinhash256`, `softHashVideoV0`. Derive
every expected value from the referenced sibling suites or the Rust core tests — do not invent
semantics from memory. Two assertions are structural and must be present:

- element-class checks, because HotSpot type-checks nothing a native returns:
    `assertEquals("[Ljava.lang.String;", IsccLib.isccDecompose(code).getClass().getName())` and the
    `"[[B"` equivalent on `algCdcChunks` (extend the existing chunk test rather than duplicate it).
- `conformanceSelftest()` is the crate's only `jboolean` return and jni-sys 0.4 aliases that to Rust
    `bool` — assert it returns `true`.

Follow the file's existing style: plain `@Test` methods, JUnit 6 static-import assertions, no
mocking. Maven's surefire config points `-Djava.library.path` at `target/debug`, so run
`cargo build -p iscc-jni` before `mvn clean test` (plain `mvn test` can reuse stale test classes).

## Verification

- `grep -rn "build_byte_array\|Vec<i8>" crates/iscc-jni/` finds nothing (exit 1) — helper gone from
    both the source and `CLAUDE.md`.
- Every one of the seven names is called from the Java test sources — this loop prints nothing:
    `for m in conformanceSelftest encodeBase64 textRemoveNewlines isccDecompose algSimhash algMinhash256 softHashVideoV0; do grep -q "IsccLib\.$m(" crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/*.java || echo "MISSING $m"; done`
- `cargo build -p iscc-jni && mvn clean test -f crates/iscc-jni/java/pom.xml` → 0 failures, 0
    errors, and a total of at least 89 tests (82 before this step, at least 7 new).
- `IsccLibTest.java` asserts both `"[Ljava.lang.String;"` and `"[[B"` via `getClass().getName()`.
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` exits 0.
- `mise run check` exits 0 and leaves the tracked tree unchanged.

## Done When

All six verification checks pass with the byte-array helper removed, the five return sites on
`env.byte_array_from_slice`, and all 33 natives called by at least one JUnit test.
