# Handoff

## 2026-07-27 — Review of: Propagation slice 5 — Unicode boundary vectors in the C FFI test

**Verdict:** PASS

**Summary:** The C FFI surface is now gated on all 12 Unicode 16.0.0 boundary vectors — the 9th of
11 binding surfaces for criterion 3 of the Unicode issue. A PEP 723 generator renders the canonical
fixture into a tracked, pure-ASCII C header that `test_iscc.c` includes with a quoted `#include`
(the CI gcc line is untouched), and a 6-case pytest anchor proves the header is a byte-exact
regeneration of the fixture. Every next.md criterion passes, the scope is exactly one non-test/
non-doc file, and I independently decoded the header's octal escapes back to code points and
mutation-probed the gate five ways — all five red.

**Verification:**

- [x] `cargo build -p iscc-ffi` exit 0, then the **verbatim** CI gcc line compiles with no new `-I`;
    `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` → `80 passed, 0 failed`, exit 0
- [x] `grep -c '^PASS: unicode_boundary/'` → **12**, plus the 3 metadata guard lines (version
    16.0.0, counts 7 and 5)
- [x] `gcc -Wall -Wextra -fsyntax-only …` — zero warnings
- [x] `uv run --script scripts/gen_ffi_boundary_vectors.py` exit 0 and
    `git status --porcelain -- <header>` empty afterwards — regeneration is a no-op
- [x] `git ls-files --error-unmatch <header>` exit 0 — tracked, not gitignored
- [x] Header is pure ASCII, no `\x`, LF-only, exactly one trailing newline (checked on the raw
    bytes). *Note: next.md's one-liner form of this criterion is not runnable as written — in a bash
    double-quoted string `'\\x'` collapses to `'\x'` and Python raises `SyntaxError`. Ran it from a
    quoted heredoc instead; the substance passes and the new pytest case
    `test_tracked_header_is_ascii_lf_only_no_hex_escapes` pins it permanently.*
- [x] `uv run pytest -q tests/test_gen_ffi_boundary_vectors.py` → 6 passed, including the
    mutated-fixture case that proves the gate fires
- [x] `uv run pytest -q` → **399 passed** (393 + 6, none removed)
- [x] `tests/test_vendored_fixtures.py` → 8 passed, file untouched by the diff
- [x] `uv run ruff check`, `ruff format --check` (exit 0), `ruff check --select S,C901`,
    `uv run ty check` — all clean. `cargo clippy --workspace --all-targets -- -D warnings` clean
    (only the known dev-only `proc-macro-error2` future-incompat note)
- [x] `mise run check` — all 17 hooks Passed, no file modified (only runner-owned `iterations.jsonl`
    dirty)
- [x] Protected paths empty in both the working tree and the advance diff: `crates/iscc-ffi/src`,
    `crates/iscc-ffi/include`, `crates/iscc-lib`, `.crap-baseline.json`, `.iai-baseline.json`,
    `.claude/context/specs/`, `.github/workflows/`
- [x] `uv run zensical build` → "No issues found"; `check_docs_nav.py` →
    `OK: 23 documentation   pages consistent`; `grep -c 'C FFI' docs/unicode.md` → 2

**Independent probes beyond next.md** (a green generator is not a correct one):

- Decoded every octal escape in the tracked header back to code points with a standalone parser and
    diffed name/input/expected against the fixture — all 12 rows exact, both `_COUNT` macros and the
    version macro match the fixture. The `Final_Sigma` row is the sentinel value
    (`0391 03A3 0378 0392` → `03B1 03C2 03B2`), not the delete-filter `03C3`.
- Five mutations, each red: (1) delete-filter-shaped expected value → names the vector; (2) case
    dropped with its count macro → the 7/5 metadata guard reds; (3) version macro → `17.0.0` → guard
    reds; (4) one octal digit hand-edited in the tracked header → pytest drift anchor reds; (5) a
    fixture vector added without regenerating → same anchor reds. Work tree restored and verified
    clean after (4)/(5).
- `iscc-ffi` declares no `[features]` and no CI job builds it `--no-default-features`, so the two
    text symbols cannot vanish from a variant build. `iscc_free_string(NULL)` is a documented no-op,
    so the `ASSERT_STR_EQ` NULL branch is safe. CI's `python-test` job runs the drift gate on both
    3.10 and 3.14 (`testpaths = ["tests"]`).
- Gate-circumvention scan over all unpushed commits (`@{upstream}..HEAD`, 4 commits, no
    `cid(meta):`): no suppressions, no skips, no threshold or hook changes; no config/workflow file
    touched at all.

**Issues found:** (none blocking)

- Two blind spots worth *stating* rather than fixing, both class-wide rather than introduced here:
    the generator's `SECTIONS` dict silently ignores a hypothetical *new* fixture section, and
    deleting block 29 of `test_iscc.c` outright would red nothing. Every other boundary suite has
    the same two holes; gating only this one would be asymmetric machinery. No issue filed.
- The advance handoff justifies the `const char *unicode_version` local as avoiding a `-Waddress`
    warning. I compiled the un-localised form: gcc 12.2 in this container emits nothing either way.
    Harmless defensiveness, but the stated rationale is unverified — recorded in agent memory so it
    is not repeated as fact.
- One accepted deviation from next.md's sketch: octal escaping uses `f"\\{byte:03o}"` instead of the
    suggested `"\\%03o" % byte`, because ruff `UP031` rejects percent-format. Output is byte-
    identical; this is the correct call.

**Codex review:** ran to completion, no findings — "The generated header accurately represents the
canonical fixture, the drift tests cover regeneration and failure guards, and the C test compiles
and passes all 12 new vectors without affecting existing behavior."

**Next:** Criterion 3 is at **9 of 11**. The two remaining surfaces are blocked on absent toolchains
(C++ needs `cmake`, Swift needs `swift`), so pick a different self-contained step. Recommended, in
order:

1. **Pin `rubygems/configure-rubygems-credentials` to `@v2.1.0`** with an inline `# exact tag:`
    comment (`normal` `[human]`, already RULED — option (a), rationale in `decisions.md`
    2026-07-26). A one-line edit to `.github/workflows/release.yml` line 895. Verification is
    entirely static: `uv run scripts/check_release_workflow.py` and `… --check-action-inputs` (zero
    `warning: skipped` lines is part of the pass), plus confirming the tag still resolves
    (`gh api repos/rubygems/configure-rubygems-credentials/git/matching-refs/tags/v2.1.0`) and that
    the step passes no `with:` keys. Smallest remaining `normal` item by a wide margin.
2. **Make the CI job table in `specs/ci-cd.md` exhaustive** (`normal` `[human]`, 14 rows vs 21 real
    jobs). This one carries a `**Spec:**` field and human authorization, so the *review* agent may
    land the spec edit when resolving it — define-next should scope it as a spec-file step and say
    so explicitly. Note the job count moved to 21 when the `unicode-sweep` job landed in iter 157;
    re-derive it from `ci.yml` rather than trusting the issue text.

**Notes:**

- The generated-header mechanism is directly reusable for the C++ slice whenever `cmake` becomes
    available — the header is plain C and `packages/cpp` wraps the same FFI. Design rationale (why
    not a C JSON parser, why not `VENDORED_COPIES`) is recorded in `decisions.md` 2026-07-27 so that
    slice does not have to re-litigate it.
- Two rules now travel with any future generated test artifact: pure ASCII + LF + exactly one
    trailing newline (otherwise the prek hygiene hooks rewrite it and red the no-op gate), and
    3-digit octal escapes rather than `\x` (C hex escapes are greedy and unbounded). Both are in
    `learnings.md`.
- `learnings.md` was at its 200-line budget; the closed "v0.6.0 dep refresh + ruff 0.16 adoption"
    entry moved to `learnings-archive.md` (its live "never `ruff check --fix .`" rule is preserved
    in the `issues.md` dependency entry).
- Nothing was pushed to a benchmarked hot path and no Rust source moved, so no CRAP or iai baseline
    refresh was required or performed.
