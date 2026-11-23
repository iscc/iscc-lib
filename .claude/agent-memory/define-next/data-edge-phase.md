---
name: data-edge-phase
description: v0.8.0 data-edge.json slice plan (iter 215 onward), probed seed outcomes, packaging traps
metadata:
  type: project
---

Second v0.8.0 criterion (added by the human 2026-11-19): `crates/iscc-lib/tests/data-edge.json` run
by Rust + 11 bindings.

**Why:** spec `specs/edge-vectors.md` orders it: file + Rust + Python harnesses → one fan-out for
JSON-parsing bindings → C/C++ (generated table, recover `scripts/gen_ffi_boundary_vectors.py` from
commit `ea69169`).

**How to apply:**

- 215 scoped the seed + Rust/Python harnesses + schema test; deferred the vendored copies
    (Go/.NET/Kotlin/Swift dirs next to data.json) to the fan-out.
- `iscc_decode` output encoding was fixed in 215: `[mt, st, vs, length_index, "bytes:<hex>"]`.
- Packaging trap: `crates/iscc-lib/tests/` ships in the .crate (`cargo package --list`), so the file
    needs `exclude` in Cargo.toml and a runtime read (not `include_str!`) in the Rust test.
- Probed 2026-11-19 (iscc_core 1.5.0): the ref ACCEPTS padding/spare-bit forms on `iscc_decode` too
    (not only `encode_seq`); ref `gen_mixed_code_v0([])` raises AssertionError; ref seq NUL raises
    binascii.Error. gen_text NUL agrees (`ISCC:EAASKDNZNYGUUF5A`, 10 chars).
- Fan-out needs per-surface `except`: .NET/C++ `error` on NUL strings, C `skip`.
- 215 NEEDS_WORK: Rust test helpers score CRAP at 0% coverage (cc²+cc) → cc ≤ 5 per fn, else CI-only
    `--fail-above` red. 216 = repair (section fn table, split check_vector/check_doc; Python
    `pytest.raises` on raw call only; `REJECTIONS` += `IndexError`, proven by an `iscc_decode("")`
    vector). Any harness `tests/*.rs` step must put CI's two CRAP lines in Verification.
- 216 PASS. Fan-out split into 3 surface groups (217): G1 = Node/WASM/Java/Go/.NET/Ruby (toolchains
    on `$PATH`; vendored copies only Go+.NET, others read canonical path); G2 = Swift+Kotlin (Swift
    needs tarball install; vendor their copies WITH the harness — the unregistered-copy gate fires
    otherwise); G3 = C+C++ (generator from `ea69169`). Then a published-artifact-absence step
    (sdist, gem, npm, Go zip carries testdata like data.json).
- Go read 217: `isccClean` rejects `""`, `softHashCodesV0`/`GenIsccCodeV0` reject \<2 → Go expected
    green. Stop rule in next.md: a real behavioural failure leaves that surface out (no `except`, no
    Go/core fix inside a fan-out).
- 217 restart: the devcontainer crashed ~17:35 during the first advance session (Go ~10 min, Node ~5
    min of real work); the human committed Node during recovery. Diagnose a stalled iteration from
    `~/.claude/projects/-workspace-iscc-lib/*.jsonl` (first/last timestamps per session) before
    shrinking a step. Re-scoped next.md = WASM/Java/.NET/Ruby + Go/Node criteria CARRIED OVER, so
    review verifies the unreviewed commits instead of pushing them unchecked.
- 218 = G2 Swift+Kotlin + Ruby missing-key fix (handoff rule: full-record surfaces compare EVERY
    expected key; only bare-string/one-field results wrap to `{iscc}`). Probed: Swift cached at
    `/tmp/swifttc/swift-6.1.2-RELEASE-debian12` (present 2026-11-19), Gradle 9.6.1 cached; Kotlin
    `build.gradle.kts` has no `testLogging` → assert on JUnit XML, never exit code. Swift
    `Package.swift` test target lists resources explicitly (`.copy`) — a vendored copy needs a line
    there. Next: G3 C/C++ (generator from `ea69169`).
- 219 = G3 C+C++: ONE generated header `crates/iscc-ffi/tests/data_edge_vectors.h` (C99 + C++17,
    positional init) from `scripts/gen_data_edge_vectors.py` (adapted from `ea69169`); strings carry
    explicit length (NUL vectors; C++ `checked_c_str` must see them); generator resolves `except`
    per row for `c`/`cpp` and fails closed on unknown sections (pytest pins it). C++ test target
    gets the include dir PRIVATE. Probed: C FFI gen_text returns only the iscc string (no
    `characters`); C++ gen results are `{iscc}` structs; C++ gen wrappers use `ptrs.data()` (NULL on
    empty) vs `encode_seq`'s `safe_data` — left out of scope. `release.yml:784` also compiles
    `test_iscc.c` from a checkout, so a quote-include next to it needs no workflow edit. Trap:
    `test_iscc.cpp` comments already say `data.json … test_0001_…` → a "no vector named" grep must
    use data-edge slugs, not `test_0NNN`. After 219: published-artifact-absence (Go/Swift reading is
    `low`, human).
- 226 = first NEW section after the seed (`alg_simhash`). A new section is one unsplittable fan-out
    (every harness + generator renderer + 4 vendored copies). Add one positive vector beside an
    error-only one, or no harness ever exercises its input decoding. Python `outcome()` passes raw
    JSON inputs, so `bytes:` inputs need a per-section preparer. `/tmp/swifttc` disappears between
    container restarts; recipe in `packages/swift/CLAUDE.md`.
- 300 = `alg_minhash_256` section (2 rows, no except/diverges). Reuse a prior section fan-out's
    next.md via `git show <define-next commit>:.claude/context/next.md` — cheapest scoping path.
