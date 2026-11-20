# Next Work Package

## Step: Fix the Java how-to `genSumCodeV0` example and state the precompiled Ruby gem range

## Goal

Closes two `normal` v0.8.0 blockers in one docs step: "Java how-to `genSumCodeV0` example does not
compile" and "Precompiled Ruby gems stop at Ruby 3.3". For Ruby this takes the **wording branch**:
the docs say which Ruby series get a precompiled gem. It does not widen the `release.yml` matrix, so
Ruby 3.4 / 4.0 users still build from source; the review should state this in the handoff for
Titusz.

## Alternatives Considered

- **Chosen:** Java example + Ruby gem range wording. Both are user-facing defects in published docs,
    small, and locally verifiable. The handoff points here.
- **Rejected:** the drift issue's gate holes (fail-closed `lib.rs` parser, register the 4
    sub-threshold docs). The work is larger, it is tooling, and it blocks nothing a reader sees. It
    is next.
- **Rejected:** widening the cross-gem `ruby-versions` list. The change would need the rb_sys pin
    and the Docker image tag to move together, and `release.yml` never runs in CI, so the only
    evidence would be static.

## Scope

- **Create**: `tests/test_ruby_gem_docs.py` (a parity test, outside the file budget)
- **Modify**: `docs/howto/java.md`, `docs/howto/ruby.md`, `crates/iscc-rb/README.md`
- **Reference**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (line 161),
    `.../SumCodeResult.java` (`public final` fields `iscc`, `datahash`, `filesize`, `units`),
    `docs/java-api.md` (around line 337, the correct form), `.github/workflows/release.yml` (the
    cross-gem step, `ruby-versions: 3.1, 3.2, 3.3`, around line 850), `tests/test_check_docs_nav.py`
    (style for a repo-reading pytest)

## Not In Scope

- Any edit to `release.yml`, `Gemfile.lock`, the rb_sys pin or the gemspec `required_ruby_version`.
    That is the matrix branch of the Ruby issue
- `crates/iscc-rb/CLAUDE.md`'s "5 platforms" count, and other hand-written counts. They belong to
    part 3 of the drift issue
- Other Java how-to sections, `docs/java-api.md`, Kotlin and Swift how-tos. A grep found the
    accessor-call error only at `docs/howto/java.md` lines 205–207
- Deleting either issue from issues.md. The review does that

## Implementation Notes

**Java** (`docs/howto/java.md`, the "single pass" block, around line 204): call
`IsccLib.genSumCodeV0("example.bin", 64, false, false)`, which takes 4 arguments: path, bits, wide,
addUnits. Read the fields directly: `result.iscc`, `result.datahash`, `result.filesize`. You may add
a second call with `addUnits = true` that prints `result.units`, as `docs/java-api.md` does, but it
is optional.

To prove the example compiles:

1. Run `mvn -f crates/iscc-jni/java/pom.xml compile`.
2. Wrap the block verbatim in a `main` method of a class in a scratch dir under `/tmp`, with
    `import io.iscc.iscc_lib.*;` and `throws Exception`.
3. Compile it with `javac -cp crates/iscc-jni/java/target/classes`.

Running it is optional, because it needs the native library.

**Ruby wording.** State only measured facts:

- precompiled platform gems are built for the Ruby series in the cross-gem list (3.1 to 3.3)
- on a newer Ruby, or on a platform with no precompiled gem, `gem install iscc-lib` compiles the
    extension from source, which needs a Rust toolchain and `libclang-dev`

Do **not** write that the gem "works on" or "supports" Ruby 3.4 / 4.0: nothing has measured the
source build there. Use the literal form `Ruby 3.1 to 3.3` in both files, so the test can match it.
Put it in `docs/howto/ruby.md`: in the intro sentence at line 9 and/or the "Build from source" note
at line 30. Put it in `crates/iscc-rb/README.md` at lines 31–32.

**Parity test** (`tests/test_ruby_gem_docs.py`; functions only, no test classes). It turns the range
into a checked derived fact:

- Load `release.yml` with `yaml.safe_load` (pyyaml is a dev dep). Find the step whose `uses` starts
    with `oxidize-rb/actions/cross-gem` and read `with["ruby-versions"]`. It may be a string or a
    float, so normalise it with `str()`, then split on commas. Fail if fewer than 1 series parses,
    or if no such step is found.
- Derive `Ruby {min} to {max}` from the series, sorted as version tuples. Check
    whitespace-normalised text (mdformat reflows at 100 columns, so collapse `\s+` to one space) and
    assert both docs contain the phrase.
- Keep the derivation and matching in small helpers. Add a fire case: a doc text that names a
    different range (e.g. `Ruby 3.1 to 3.4`) must fail the matching helper, and a helper call on a
    workflow dict with no cross-gem step must raise.

This is a docs + tests step: no binding or core source changes, so the CRAP, iai and semver gates
are unaffected.

## Verification

- The fixed Java block, wrapped in a class, compiles:
    `javac -cp crates/iscc-jni/java/target/classes` exits 0 after
    `mvn -f crates/iscc-jni/java/pom.xml compile`
- `grep -nE "result\.(iscc|datahash|filesize|units)\(\)" docs/howto/java.md` prints nothing, and the
    howto's `genSumCodeV0(` calls each pass 4 arguments
- `uv run pytest tests/test_ruby_gem_docs.py` passes, including the fire cases
- `docs/howto/ruby.md` and `crates/iscc-rb/README.md` both say a newer Ruby builds from source and
    needs Rust and `libclang-dev`. Neither claims Ruby 3.4 / 4.0 support
- `uv run zensical build` exits 0
- `mise run check` exits 0

## Done When

The advance agent is done when the Java example compiles against the real classes, both Ruby docs
state the range that `release.yml` builds and a test pins it, and every check above passes.
