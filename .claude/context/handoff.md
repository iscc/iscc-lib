## 2026-11-20 — Review of: Fix the Java how-to `genSumCodeV0` example and state the precompiled Ruby gem range

**Verdict:** PASS_WITH_NOTES

**Summary:** The step did what next.md asked and every criterion passes. The Java example compiles
against the real classes, both Ruby docs state `Ruby 3.1 to 3.3`, and a parity test derives that
phrase from `release.yml`. The note is a defect the step exposed: the sentence next.md prescribed
for the Ruby docs ("on a newer Ruby `gem install iscc-lib` compiles the extension from source") is
false, because the published source gem cannot be built at all. It is filed as a `normal` issue.

**Verification:**

- [x] The fixed Java block, wrapped in a class, compiles — the block was extracted from
    `docs/howto/java.md` by script, `mvn -q … compile` exited 0, `javac -cp …/target/classes` exited
    0
- [x] `grep -nE "result\.(iscc|datahash|filesize|units)\(\)" docs/howto/java.md` prints nothing, and
    the only `genSumCodeV0(` call in the how-to (line 204) passes 4 arguments
- [x] `uv run pytest tests/test_ruby_gem_docs.py` — 9 passed. Fire cases checked on a throwaway copy
    of the tree: with `ruby-versions: 3.1, 3.2, 3.3, 3.4` both doc tests fail, and with the previous
    versions of the two docs both doc tests fail
- [x] Both Ruby docs say a newer Ruby builds from source and needs Rust and `libclang-dev`, and
    neither claims Ruby 3.4 / 4.0 support — true as a statement about the text. The text itself is
    wrong, see below
- [x] `uv run zensical build` exits 0 ("No issues found"). The rendered note in `howto/ruby` keeps
    its code block inside the admonition
- [x] `mise run check` exits 0. `ty` and ruff `S`/`C901` are clean on the new test file

**Issues found:**

- **The Ruby source gem cannot be built** (`normal`, filed). Measured with the published 0.7.0
    source gem in a clean `GEM_HOME`: without `rb_sys` installed, `extconf.rb` fails with
    `cannot load such file -- rb_sys/mkmf`; with `rb_sys` installed, `cargo` fails with
    `failed to find a workspace root`. The gem ships a `Cargo.toml` that inherits from the workspace
    and depends on `../iscc-lib`. No workflow installs the source gem. The claim was already in the
    docs before this step ("the gem builds from source"), so the step did not make them worse
- **Sum-Code example comments show `"ISCC:KAA..."`** (`low`, untriaged, filed). The call returns
    `ISCC:KUA…`. Pre-existing, in most how-tos
- Nits in `tests/test_ruby_gem_docs.py`, not filed: the match is a substring (`Ruby 3.1 to 3.3` also
    matches `Ruby 3.1 to 3.30`), only the first cross-gem step is read, the min–max phrase assumes a
    gapless list, and `ruby.md` names the range twice while one hit satisfies the test

**Issues closed:** "Java how-to `genSumCodeV0` example does not compile" (fixed and compiled).
"Precompiled Ruby gems stop at Ruby 3.3" (its wording branch is met; the source-gem issue replaces
it, because the source build that the wording relies on does not work).

**Codex review:** No actionable findings. It ran the 9 tests and compiled and ran the Java example.
It did not check the Ruby install claim.

**Next:** "The Ruby source gem cannot be built". It is user-facing, it makes the sentences this step
wrote untrue, and it reproduces locally in seconds:

```bash
(cd crates/iscc-rb && gem build iscc-lib.gemspec)
GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install crates/iscc-rb/iscc-lib-*.gem --local --no-document
```

The issue names two ways to resolve it. Which one to take is open: define-next should either pick
the cheaper one it can prove locally, or ask Titusz if both look expensive. Hypotheses, not
measured: a buildable source gem needs `rb_sys` as a gem dependency and a manifest that stands alone
(the core from crates.io or vendored into the gem). The `rb_sys` `0.9.123` pin in the `Gemfile`
exists for the cross-gem image and must not leak into what users install. After that, the remaining
`normal` work is the three parts of "Derived facts in guidance and docs".

**Notes:**

- **For Titusz:** this step took the wording branch of the Ruby issue, so `release.yml` still builds
    gems for Ruby 3.1, 3.2 and 3.3 only. The review then found that the fallback does not exist: on
    Ruby 3.4 and 4.0 `gem install iscc-lib` fails for 0.6.0 and 0.7.0, with or without a Rust
    toolchain. Only a checkout build works there. Whether to ship a fix release before v0.8.0 is
    your call; the loop will not cut one
- The docs on `develop` now state the false sentence more explicitly than before. They are not
    published until the `develop` → `main` merge, so the source-gem issue has to close before the
    v0.8.0 release, or the two Ruby docs have to be reworded
- No binding or core source changed, so the CRAP, iai and semver gates were not run
- `learnings.md` sits at its 200-line budget: one entry was rotated into the archive to make room
