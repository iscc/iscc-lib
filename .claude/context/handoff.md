> **HUMAN REVIEW RESOLVED (2026-11-22, Titusz → reject, consume, bound at 256):** both questions are
> answered and one more rule is decided; all three are in decisions.md and each has a `normal` entry
> in issues.md. (1) `alg_minhash_256` reports an error for an empty list on every surface; the
> breaking signature change is approved. (2) The first `finalize()` consumes a hasher whatever its
> outcome; `@iscc/wasm` is the surface to fix. (3) `bits` above 256 is rejected in iscc-lib (the
> former `low` issue, now `normal`; reported upstream as iscc/iscc-core#142). The "Next" section
> below predates these answers: there is `normal` work again, so `## Step: NONE` no longer applies.
> Also done in the same commit: target.md and the Swift spec say the release tags once, and
> target.md and the agent API-break rules name `streaming::SumHasher` as Tier 2 (both `low` issues
> removed).

## 2026-11-22 — Review of: `encode_component` data-edge section on all 12 surfaces

**Verdict:** PASS

**Summary:** `data-edge.json` and its 4 vendored copies gained an `encode_component` section with a
short-digest row (error, `diverges`) and an exact-length row (`AAAQAAICAMCAKBQH`). All 12 harnesses,
the C/C++ header generator and its tests handle it; no library file changed. Every surface was
re-run by the reviewer and is green, and the `normal` [audit] issue is closed.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge` passes (7 passed; also `--no-default-features`,
    `text-processing`, `--all-features`), and the CI CRAP lines exit 0 (`↑ 0 regressed`; the new
    helpers score 20.0 at most, limit 30)
- [x] `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py` and
    `tests/test_vendored_fixtures.py` pass (185 passed); `-k encode_component -v` shows schema,
    iscc_lib and the iscc-core cross-check passing for both vectors
- [x] After `uv run python scripts/gen_data_edge_vectors.py`,
    `git status --porcelain crates/iscc-ffi` prints nothing
- [x] nodejs `npm ci && npx napi build --platform && npm test` in a `git archive` copy under `/tmp`
    (238 pass, 0 fail); wasm `wasm-pack test --node crates/iscc-wasm --features conformance` (127
    passed in the main suite); java `mvn test` (`DataEdgeTest` 8 run, 0 failures; 122 total); go
    `CGO_ENABLED=0 go test ./...` ok (also `GOARCH=386`, `go vet`, `gofmt`); ruby
    `bundle exec rake test` (168 runs, 0 failures); dotnet (155 passed, run as
    `dotnet test packages/dotnet/Iscc.Lib.Tests/`, the CI form)
- [x] c `test_iscc` (150 passed, 0 failed) and cpp `test_iscc` (cmake ASAN build, 121 passed, 0
    failed), both `data_edge/encode_component/*` rows print PASS; kotlin JUnit XML:
    `DataEdgeTest tests="6" failures="0" errors="0"`; swift `swift test --filter DataEdge` (11
    tests, 0 failures)
- [x] The JSON count one-liner exits 0 (the 5 copies are byte-identical), and `mise run check` exits
    0

Beyond next.md (1 of 3 probes used): iscc-core 1.5.0 under Python 3.10 and 3.15 returns `AAAQCAQDAQ`
for the short digest and `AAAQAAICAMCAKBQH` for the exact one, so both vector notes and the
`diverges` marker are true on both builds. Also green: `cargo clippy --workspace --all-targets`,
`uv run ty check`, ruff `S`/`C901`, and the `NativeMethods.g.cs` / `iscc.h` freshness diff.

**Issues found:**

- (none filed) One nit: the C and C++ commits do not compile on their own (the struct arrives with
    the final commit that regenerates the header), the same order as the `alg_cdc_chunks` fan-out
- Fan-out check: one behaviour, decided on 2026-11-22; no surface made a design choice and no core
    or binding source file changed (`git diff --stat` over `src/` trees is empty)

**Codex review:** ran over the whole advance range (`--base` the define-next commit): "No actionable
defects found."

**Next:** nothing `normal` is left that the loop can decide. `alg_minhash_256([])` and the WASM
hasher `finalize()` rule both wait for Titusz (see the marker above). If neither is answered, the
honest next.md is `## Step: NONE` naming those two blockers. Do not pick a `low` issue: every open
one is untriaged or human-only. Once Titusz answers, each decision is one small step: the core or
WASM change, a decisions.md entry, and for minhash one `alg_minhash_256` section in `data-edge.json`
(another 12-surface fan-out; Java's `Map.of` section table is at its 10-pair limit and needs
`Map.ofEntries` for the next section).

**Notes:**

- state.md still says "9 sections" in four places and lists the closed issue; update-state owns it
- The decision "encode_component rejects a digest shorter than bit_length/8" now names the vectors
    that pin it instead of a "Next" line
- CRAP lists 7 functions as new and 4 as removed against the baseline (cfg twins among them);
    nothing regressed and the gate exits 0, so the baseline was left alone
- The WASM harness passes header fields as JS numbers and leaves the range check to the binding;
    every other harness reads them into the native 8-bit or `int` type first
