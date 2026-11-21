## 2026-11-21 — Review of: Remove hand-written counts and line numbers from the specs and per-crate CLAUDE.md files

**Verdict:** PASS

**Summary:** The advance commit removed every in-scope count, tally, file size and `path:NN`
reference from 14 specs and 10 per-crate/package `CLAUDE.md` files, without replacing any of them by
a corrected number. All Tier 1 symbol names are kept and the three parity gates stay green. A
wrap-insensitive sweep over all 27 files, including the three the commit did not touch, found no
leftover count of the in-scope classes.

**Verification:**

- [x] `path:NN` grep over specs and `CLAUDE.md` files prints nothing
- [x] Count-class grep (with `grep -v 'JUnit [0-9]'`) prints nothing
- [x] `~N lines` / `line N in` grep (with `grep -v exceeds`) prints nothing
- [x] `check_tier1_docs.py && check_spec_paths.py && check_ci_job_table.py` exits 0 (20 full + 2
    exact docs, 15 specs, 21 CI jobs)
- [x] The advance commit touches only specs and per-crate `CLAUDE.md` files, plus `handoff.md` and
    the advance `MEMORY.md` that the protocol requires in the same commit
- [x] `mise run check` exits 0
- [x] Done When: a joined-line sweep for digit and spelled-out counts before countable nouns finds
    only keep-class hits (items listed next to the number, algorithm parameters, test values)

Reworded sentences checked against the code: `Rake::ExtensionTask#define` holds the guard (vendored
rake-compiler 1.3.1), the `gen_iscc_id_v1` doc comment marks it experimental, and `rust-core.md`
names the one IDv1 symbol that exists. Unpushed range: `.md` files only, no gate change.

**Issues found:**

- Nit, not filed: `python-bindings.md` and `crates/iscc-py/CLAUDE.md` now say "one typed result
    class per `gen_*_v0` function". That is true, but `__init__.py` also has `IsccIdResult` for
    `gen_iscc_id_v1`, which the sentence leaves out. The old "9" and "10" were both wrong.
- Nit, not filed: `ci-cd.md` says a push shows "one more check name than there are job keys". That
    is still derived from the matrix size, but the matrix is written out in the same sentence.
- Nit, not filed (advance reported it): `crates/iscc-rb/CLAUDE.md` quotes the rake-compiler guard as
    `@gem_spec && @gem_spec.platform == 'ruby'`; 1.3.1 reads `!@no_native && (@gem_spec && …)`. The
    meaning for the gemspec rule is the same.
- The advance handoff said 15 specs and 11 `CLAUDE.md` files changed. The diff has 14 and 10
    (`documentation.md`, jni and ffi had no hit).

**Codex review:** No findings ("documentation-only changes introduce no actionable regressions").

**Next:** The file-list part of "Derived facts in guidance and docs are written by hand and drift"
(`normal`, the last v0.8.0 blocker; the entry is rewritten to that remainder). All 12 per-crate and
per-package `CLAUDE.md` files carry a fenced directory tree or a module table (`crates/iscc-lib`,
`crates/iscc-rb`, `packages/go` use tables), and nothing checks them. The specs' trees are checked
for existence only. The step has to choose between removing a list and gating it. Suggested first
move: run the tree parser of `scripts/check_spec_paths.py` over the 12 `CLAUDE.md` files and count
the stale paths, so the choice rests on a measurement. Extending the script to those files is a code
change with tests; removing the trees is docs-only.

**Notes:**

- `learnings.md` is at its 200-line budget. This review dropped three hand-written counts from it
    ("35 symbols", "21 targets", "23 pages") and moved the Conan `GENERATED` fact to the archive.
- Docs-only step: no core change, so the CRAP, iai and semver gates and the conformance vectors are
    unaffected, and there is no API break.
- Versions in the touched files are still out of scope and still inconsistent in places (JUnit 5 in
    `java-bindings.md`, JUnit 6 in the crate file).
