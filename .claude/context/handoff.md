# Handoff

## 2026-07-27 — Propagation slice 5 — Unicode boundary vectors in the C FFI test

**Done:** Gated the `iscc-ffi` C surface on the 12 Unicode 16.0.0 boundary vectors — the 9th of 11
binding surfaces for criterion 3 of the Unicode issue. A PEP 723 generator renders the canonical
fixture into a pure-ASCII C header (non-ASCII UTF-8 bytes as 3-digit octal escapes), `test_iscc.c`
includes it and runs a 3-assertion metadata guard plus both vector sections, and a pytest drift gate
proves the tracked header is a byte-exact regeneration of the fixture.

**Files changed:**

- `scripts/gen_ffi_boundary_vectors.py` (new): PEP 723 generator, stdlib-only. Pure
    `render(fixture_path) -> str`; fail-closed `SystemExit` guards (version must be `"16.0.0"`, both
    sections non-empty, exactly 1 input per case, rendered text `.isascii()`); octal-only escaping
    (`\360…`, never `\x` — C hex escapes are greedy).
- `crates/iscc-ffi/tests/unicode_boundary_vectors.h` (new, generated, tracked): version macro,
    vector struct, 7 `text_clean` + 5 `text_collapse` entries. LF-only, one trailing newline, pure
    ASCII — byte-stable under the prek hygiene hooks (verified: `mise run check` did not touch it).
- `tests/test_gen_ffi_boundary_vectors.py` (new): 6 tests — render==tracked-header no-op anchor,
    ASCII/LF/no-`\x` invariants, octal-escape unit check (builds U+1FAE9 via `chr`, per the
    escape-decoding trap), mutated-fixture-renders-differently (gate provably fires), and
    wrong-version + empty-section fail-closed cases. All mutations in `tmp_path`.
- `crates/iscc-ffi/tests/test_iscc.c`: added quoted `#include "unicode_boundary_vectors.h"`, a
    `run_unicode_boundary_section` static helper (shared loop, `ASSERT_STR_EQ`, names
    `unicode_boundary/<section>/<case>`, frees every string), and block 29 with the metadata guard
    (version `"16.0.0"`, counts 7/5 hard-coded per convention) + both section loops.
- `docs/unicode.md`: the gated-suites paragraph now names the C FFI test program and the
    generated-header mechanism (`grep -c 'C FFI'` → 2).
- `crates/iscc-ffi/CLAUDE.md`: header listed in Module Layout; C-tests section points at the
    generator and the pytest gate.
- `.claude/agent-memory/advance/unicode-freeze.md`: slice-5 record; pending list now C++ + Swift
    only.

**Verification:** All next.md criteria pass, each backed by a command run this session:

- `cargo build -p iscc-ffi` exit 0; **unmodified** CI gcc line compiled with no new `-I`;
    `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` → **80 passed, 0 failed**, exit 0 (65 baseline + 3
    guard + 12 vectors); `grep -c '^PASS: unicode_boundary/'` → **12**
- `gcc -Wall -Wextra -fsyntax-only` — zero warnings (version guard goes through a `const char *`
    local so `-Waddress` never sees a literal-vs-NULL comparison)
- Regeneration no-op: reran the generator after staging — sha256 identical (`978076ab…`),
    `git status --porcelain -- <header>` shows `A ` with no unstaged `M`; header is tracked
    (`git ls-files --error-unmatch` exit 0); ASCII/no-`\x`/trailing-`\n` one-liner exit 0
- `uv run pytest -q tests/test_gen_ffi_boundary_vectors.py` → 6 passed; full `uv run pytest -q` →
    **399 passed** (393 + 6, none removed); `tests/test_vendored_fixtures.py` → 8 passed, file
    untouched
- `uv run ruff check` / `ruff format --check` (177 files) / `ruff check --select S,C901` /
    `uv run ty check` — all clean (one UP031 finding fixed at source: `%`-format → f-string, output
    byte-identical by sha256)
- `mise run check` exit 0, all hooks Passed, no file modified (only runner-owned `iterations.jsonl`
    dirty)
- Protected paths empty: `crates/iscc-ffi/src`, `crates/iscc-ffi/include`, `crates/iscc-lib`,
    `.crap-baseline.json`, `.iai-baseline.json`, `.claude/context/specs/`, `.github/workflows/`
- `uv run zensical build` → "No issues found"; `check_docs_nav.py` →
    `OK: 23 documentation pages consistent`

**Next:** Criterion 3 is now 9 of 11 surfaces. The two remaining (C++ needs `cmake`, Swift needs a
`swift` toolchain) cannot be executed in this container, so per the prior review the next
self-contained candidates are: pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` with an
`# exact tag:` comment (one line, RULED), or make the CI job table in `specs/ci-cd.md` exhaustive
(14 rows vs 21 jobs).

**Notes:**

- The generated header is deliberately NOT in `VENDORED_COPIES` (it is derived, not byte-identical);
    the regeneration-no-op pytest anchor is its drift gate, per next.md's explicit instruction.
    `tests/test_vendored_fixtures.py` was not touched.
- `.github/workflows/ci.yml` unchanged — the quoted include resolves from the header's sibling
    directory exactly as next.md's scoping predicted.
- One deviation from next.md's implementation sketch: the octal escape uses `f"\\{byte:03o}"`
    instead of the suggested `"\\%03o" % byte` because ruff UP031 rejects percent-format; the
    rendered header is byte-identical (same sha256 before/after).
- `mise run format` timed out once at the 2-minute default (mdformat leg on a cold run) after
    already applying its mdformat reflow of my CLAUDE.md bullet; the subsequent full
    `mise run check` completed clean within the raised timeout, so nothing was left half-applied.
- No Rust source, cbindgen header, fixture, baseline, spec, or workflow file moved; no benchmarked
    hot path touched — no CRAP/iai refresh needed.
