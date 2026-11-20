## 2026-11-20 — Fix the Java how-to `genSumCodeV0` example and state the precompiled Ruby gem range

**Done:** The Java how-to "single pass" Sum-Code example now calls the 4-argument
`genSumCodeV0(path, bits, wide, addUnits)` and reads the `public final` fields. Below it is a
one-line parameter list. Both Ruby docs say precompiled platform gems are built for
`Ruby 3.1 to 3.3`. They also say that a newer Ruby, or a platform with no precompiled gem, compiles
from source, which needs Rust and `libclang-dev`. A parity test derives that phrase from
`release.yml`'s cross-gem step.

**Files changed:**

- `docs/howto/java.md`: fixed the Sum-Code block (4 args, `result.iscc` / `.datahash` / `.filesize`)
    and added a parameters line naming `addUnits` → `result.units`
- `docs/howto/ruby.md`: the intro sentence names Ruby 3.1 to 3.3. The "Build from source" note gives
    the range, the from-source fallback for a newer Ruby or an unsupported platform, and Rust +
    `libclang-dev` + headers. The checkout-build snippet is now introduced as an alternative
- `crates/iscc-rb/README.md`: the install paragraph says the same as the how-to
- `tests/test_ruby_gem_docs.py` (new): `cross_gem_ruby_series` (yaml, `str()`-normalised, raises
    `ValueError` if there is no cross-gem step or the list is empty), `ruby_range_phrase` (sorts by
    version tuple) and `doc_names_range` (whitespace-collapsed match). There are 9 tests: the real
    workflow, both real docs, version sort (3.10 > 3.9), a single float value, a wrapped phrase
    matching, a fire case where `Ruby 3.1 to 3.4` does not match, and fire cases for a missing step
    and for empty `ruby-versions`

**Verification:**

- `mvn -q -f crates/iscc-jni/java/pom.xml compile` exited 0. The block, copied verbatim into
    `/tmp/javaex/Example.java` (checked with `diff` against lines 199–207 of the how-to →
    identical), compiled with `javac -cp crates/iscc-jni/java/target/classes` and exited 0. After
    `cargo build -p iscc-jni` it also ran and printed `ISCC:KUABGAWXH2VYUQWIVYLQGD2NZH6TM`, a
    `1e20…` datahash and `11000`
- `grep -nE "result\.(iscc|datahash|filesize|units)\(\)" docs/howto/java.md` printed nothing (exit
    1). The only `genSumCodeV0(` call in the how-to (line 204) passes 4 arguments
- `uv run pytest tests/test_ruby_gem_docs.py`: 9 passed
- Real-doc fire probe: with the parsed workflow's `ruby-versions` (raw `'3.1, 3.2, 3.3'`) changed in
    memory to `3.1, 3.2, 3.3, 3.4`, the phrase becomes `Ruby 3.1 to 3.4` and both real docs fail to
    match (`[False, False]`)
- `uv run zensical build` exited 0
- `mise run check` exited 0. The first run reformatted the files (mdformat reflow, ruff format of
    the test) and the second run was clean
- `prek run --hook-stage pre-push --files <the 4 files>` exited 0: clippy, cargo test, ty, ruff
    S/C901 and pytest all passed

**Next:** The three parts of "Derived facts in guidance and docs": the fail-closed `lib.rs` parser,
registering the 4 sub-threshold docs, and the hand-written counts such as `crates/iscc-rb/CLAUDE.md`
"5 platforms".

**Notes:**

- **For Titusz:** this step took the Ruby issue's wording branch. `release.yml` still builds gems
    for 3.1, 3.2 and 3.3 only, so users on Ruby 3.4 or 4.0 compile from source. Nothing has measured
    that source build on 3.4 / 4.0, and the docs make no claim that it works there
- The docs and tests changed, but no binding or core source did, so the CRAP, iai and semver gates
    are unaffected
- The `ruby.md` "Build from source" note previously read as if the clone-and-compile snippet were
    the automatic fallback. I said instead that `gem install` compiles the extension itself, and
    introduced the snippet as "to build from a checkout of the repository instead"
