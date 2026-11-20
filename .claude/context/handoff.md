## 2026-11-20 — Review of: data-edge.json fan-out, group 3 — C and C++ harnesses via one generated header

**Verdict:** PASS_WITH_NOTES

**Summary:** The generator recovered from `ea69169` renders `data-edge.json` into a C99/C++17 header
(`crates/iscc-ffi/tests/data_edge_vectors.h`), and pytest fails when that header is stale. The C and
C++ tests now run every vector through one check function per section, with a total-row tally and
negative tests. It is clean, in scope (an honest fan-out, one commit per surface), and every next.md
check passes. The review commit carries three small test-robustness fixes from the Codex findings
(see below).

**Verification:**

- [x] `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py tests/test_vendored_fixtures.py`:
    100 passed (before and after my edits). `mise run check` exit 0
- [x] `uv run --script scripts/gen_data_edge_vectors.py` leaves
    `git status --porcelain crates/iscc-ffi/tests` empty
- [x] The no-vector-name `grep -nE …` over both harnesses prints nothing (exit 1)
- [x] C: `cargo build -p iscc-ffi` + the `ci.yml` gcc line, exit 0, 96 passed. 14
    `PASS: data_edge/…` lines (all fixture vectors except the two `c: skip` rows, which print
    `SKIP:`), tally PASS, 3 negative PASS lines
- [x] C++: fresh `/tmp/cpp-edge`, `ci.yml` flags, absolute `FFI_LIB_DIR`, ASAN on: exit 0, 0
    sanitizer lines, all 16 `PASS: data_edge/…` lines, tally PASS, 2 negative PASS lines (re-run
    after my fix: 95 passed)
- [x] Probes: (1) the negative's expected `ISCC:EAAWSPUDF7GJWQVD` is the code of `"Hello"`, so the C
    NUL negative fails only on the length check, as claimed; (2) a header rendered with `except.c`
    dropped from the `encode_seq` NUL row turns C red (the list path, which advance had not probed);
    (3) the C++ CMake tree is already repo-relative (`../../crates/iscc-ffi/include`), and
    vcpkg/release copy only headers, so the PRIVATE test include cannot break a consumer
- [x] Gate scan of all unpushed commits: no suppressions, skips or hook/threshold changes. No Rust
    source changed, so CRAP/iai/semver are untouched

**Issues found:**

- Filed `low` (Triage: untriaged): "C++ list wrappers pass NULL for an empty vector". The same
    pattern sits at five `iscc.hpp` sites (next.md named two); the input is still rejected
- Nit, not filed: the C `RUN_EDGE_SECTION` name buffer is 160 bytes, so a very long vector name is
    truncated in the PASS/FAIL line only (counting is unaffected)
- Nit, not filed: the generator keeps its own `SURFACE_IDS` copy next to `tests/test_data_edge.py`.
    Drift fails closed (an unknown `except` key → `SystemExit` → the staleness test goes red)
- Issue sweep: the Dependabot entry no longer mentions the `json` gem (`Gemfile.lock` pins 2.21.2).
    Three entries were compressed to get `issues.md` under budget

**Codex review:** (one run over `c0e123d..HEAD`, all four commits). Three findings were valid and I
fixed them in this commit:

1. C++ `run_edge_section` returned `count`, not the number of rows visited. It now counts the rows
    it visits, like the C macro. A loop shortened by one row now fails the tally ("got 11, expected
    16")
2. The pytest `"\\x" not in text` check went red on a valid vector holding a literal `\x` (rendered
    `\\x`). It now scans escape tokens, and an inline assert covers both cases
3. The pytest row-boundary split on `"};"` broke on a `};` payload. It now splits on `"\n};\n"`

Both pytest fixes were proven on a mutated fixture. The 4th Codex finding is the 160-byte name nit
above.

**Next:** The v0.8.0 "Shared edge-case vectors run everywhere" criterion should be met on all 12
surfaces once CI's `c-ffi` and `cpp` jobs are green on the pushed sha (update-state: verify on the
exact sha). Cheapest `normal` follow-ups: the Python `encode_seq` docstring issue (two docstrings),
then the `wasm-bindings.md` package.json spec-fact issue, or the .NET pinning-helper issue.

**Notes:** Only the ubuntu gcc/g++ in CI compile the header (both `c-ffi` and `cpp` jobs are
`ubuntu-latest`), so MSVC/clang never see it. The empty-list rows pass on C++ through the FFI NULL
guard, not the core's count check; see the filed issue.
