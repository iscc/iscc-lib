> **HUMAN REVIEW RESOLVED (2026-11-21, Titusz → no large vectors, no bound):** `data-edge.json` gets
> no `alg_cdc_chunks` vector with `avg_chunk_size >= 2^29`, and iscc-lib keeps accepting every value
> from 2 up. ISO 24138 fixes the average chunk size at 1024, and the range has no memory-safety
> stake. The build mismatch is reported as iscc/iscc-core#141; nothing here waits on it. The
> decision is in decisions.md, and the issues.md entry is gone. Also decided: the 32-bit issue is
> now `normal` and asks for one Go `GOARCH=386` CI step (decisions.md, issues.md).

## 2026-11-21 — Review of: `alg_cdc_chunks` data-edge section on all 12 surfaces (reference-stable vectors)

**Verdict:** PASS_WITH_NOTES

**Summary:** The step added an `alg_cdc_chunks` section with 3 vectors to `data-edge.json`, its 4
vendored copies and the generated C header, and taught all 12 harnesses the section. Every surface
is green, no library code changed, and no surface made a design choice of its own. The fan-out is
honest: one behaviour the core already fixed, one commit per surface. The reference-build finding is
confirmed by measurement.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge` passes (7 passed, also with
    `--no-default-features` and `--all-features`); the CI CRAP lines exit 0 ("0 regressed, 0 new,
    162 unchanged")
- [x] `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`:
    156 passed, with the 3 `test_reference_outcome[alg_cdc_chunks/…]` rows
- [x] `uv run python scripts/gen_data_edge_vectors.py` leaves the header unchanged
- [x] nodejs `npm test` 238 pass; wasm `wasm-pack test --node … --features conformance` green
    (data_edge: 8 passed); java `mvn clean test` rc 0, `DataEdgeTest` XML 8 tests, 0 failures; go
    `CGO_ENABLED=0 go test ./...` ok and `go vet` clean; ruby 168 runs, 0 failures; dotnet 154
    passed (with the CI path, see Issues found)
- [x] c `test_iscc` 145 passed, 0 failed, 0 warnings under `-std=c99 -Wall -Wextra -pedantic`; cpp
    (cmake + ASAN) 115 passed; kotlin `DataEdgeTest` XML tests=6, failures=0; swift
    `swift test --filter DataEdge` 11 tests, 0 failures
- [x] The `< 2**29` assert exits 0; `mise run check` exits 0; clippy `--workspace --all-targets`
    clean

Probes beyond next.md (3):

- Reference builds: all 3 vectors match iscc-core 1.5.0 on Python 3.10, 3.14 (compiled) and 3.15
    (pure). 2^29, 2147483647 and 4294967295 raise `OverflowError` on 3.10 and 3.14 and return one
    chunk on 3.15
- Mutation on the six surfaces advance did not mutate (Java, .NET, Kotlin, WASM, Swift, C++):
    `bytes:1d1e` → `1d1f` turned all six red on `test_0001`. Restored
- `utf32: true` with average 2: both iscc-core builds yield empty chunks forever, iscc-lib returns
    4-byte chunks. next.md's reason for leaving such vectors out holds

**Issues found:**

- Fixed here: the `test_0001` note said the vector pins "the min/max chunk sizes". With average 2
    the maximum is 16 bytes and no chunk reaches it. The note now says so, in all 5 copies (pytest
    and the Rust harness re-run green, copies byte-identical)
- next.md's `dotnet test packages/dotnet` exits 1 (MSB1003, no project file in that directory). The
    CI line is `dotnet test packages/dotnet/Iscc.Lib.Tests/`
- Nit, not filed: `test_cdc_rows_render_data_flag_size_and_chunks` reads `vector["outputs"]` for
    every row of the real fixture, so the first error vector in this section needs a test edit
- Nit, not filed: the Java harness reads the average with Gson `getAsInt`, which wraps a value above
    2^31 - 1 instead of failing. The largest planned vector is 2147483647, so nothing is hidden
- Issue rewritten to its remainder: "No shared vector pins `alg_cdc_chunks` for `avg_chunk_size`
    from 2^29 up" (`normal`, with `**Upstream:** iscc/iscc-core`). It is blocked on the decision
    above
- decisions.md: the 2026-11-21 CDC entry said "iscc-core accepts every value". Corrected to "the
    pure-Python build", with the wheel's behaviour named. The decision itself is unchanged

**Codex review:** no actionable findings. It ran the targeted tests on all 12 surfaces and the
header consistency checks.

**Next:** Python video frame values raise a platform-dependent exception class (issues.md, 1
non-test file). Do not scope the 2^29 / 2147483647 vectors until Titusz has answered the question at
the top. Other ready `normal` work: the Ruby floor move to 3.3, the stale spec and `CLAUDE.md`
statements (audit, up to 8 files), the one-shot gen duplication.

**Notes:**

- A vector for `alg_cdc_chunks` must be probed on both iscc-core builds before it goes into
    `data-edge.json` (learnings, Reference Implementation). CI's pytest legs cover both: 3.10 loads
    the wheel, 3.15 the pure build
- The `utf32` endless-chunk behaviour of the reference is not filed: iscc-lib's `cut_point == 0`
    guard is deliberate and the reference gives no result to compare against
- Commit order as in the `alg_simhash` fan-out: the per-surface harness commits are not
    self-consistent on their own, the last commit (fixture, copies, generator, header) closes them
- learnings.md is at 200 lines again: the DATA TABLES bullet moved to `learnings-archive.md`
