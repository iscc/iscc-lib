<!-- assessed-at: 98e296402e55ce707aed6a278fc32cbf8a329dcb -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode contract gated; propagation at 9 of 11 binding surfaces

Iteration 159 propagated the Unicode 16.0.0 boundary vectors into the C FFI test through a
generated, tracked C header, taking criterion 3 from 8 to 9 of 11 surfaces. I re-verified the slice
by two methods the review did not use — decoding the header's octal escapes back into strings and
diffing against the canonical fixture, and driving the built `libiscc_ffi.so` through `ctypes`. Only
C++ and Swift remain ungated, and neither is buildable in this container.

## Rust Core Crate

**Status**: partially met — criteria 1, 2, 4 met; criterion 3 at 9 of 11 surfaces

- **Incremental scope**: `git diff 0b8f2e2..HEAD --stat -- . ':!.claude'` = **6 files**, all C FFI
    propagation — new: `scripts/gen_ffi_boundary_vectors.py` (+135),
    `tests/test_gen_ffi_boundary_vectors.py` (+94),
    `crates/iscc-ffi/tests/unicode_boundary_vectors.h` (+65, generated); edited:
    `crates/iscc-ffi/tests/test_iscc.c` (+36), `crates/iscc-ffi/CLAUDE.md` (+5), `docs/unicode.md`
    (+4/−2). **Zero Rust source files changed**, zero baseline files changed. `.claude/context/specs/`
    diff is **empty** — no spec moved under me.
- Tier 1 surface unchanged: **32** symbols, all 10 `gen_*_v0` conformant against
    `crates/iscc-lib/tests/data.json`. `#[test]` count in `crates/iscc-lib/**/*.rs` re-counted at
    **342** (unchanged). No `unsafe` in the core.
- **Criterion 1 (declared version 16.0.0 + sentinel freeze) — MET.** `utils/unicode16.rs` (731
    unassigned ranges) + `utils/unicode16_case.rs` (152 `Cased`, 452 `Case_Ignorable`), both
    regenerable and byte-stable.
- **Criterion 2 (`Final_Sigma` case freeze, landed 156) — MET.** Untouched this iteration.
- **Criterion 4 (differential sweep as a fail-closed gate, landed 157, hardened 158) — MET.**
    Untouched this iteration; the `unicode-sweep` CI job is green on the tip.
- **Criterion 3 (boundary vectors on every surface) — 9 of 11.** Gated: Python, Node.js, WASM, Java,
    Ruby, Kotlin, C#, Go, **C FFI (new)**. Ungated: **C++**, **Swift**.
- `specs/rust-core.md` L149-157 is still **STALE** (claims Go has the `Final_Sigma` defect — fixed
    at 147 — and that `str::to_lowercase()` behaves like the reference, falsified by measurement at
    156); criterion boxes 1/3/4 unchecked though 1 and 4 are met. Human-owned file; not edited.

## Python Bindings

**Status**: met — Unicode-gated since 150; also carries the new C-header drift gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- `pytest --collect-only -q` → **399** tests (was 393). The +6 are exactly
    `tests/test_gen_ffi_boundary_vectors.py`: the byte-exact regeneration anchor, an
    ASCII/LF/no-`\x` hygiene case, an octal-escape unit case, a mutation case proving the gate
    fires, and two fail-closed cases (wrong `unicode_data_version`, empty section). No prior case
    removed; `tests/test_unicode_sweep.py` still **14**.
- This is what carries the C FFI drift gate into CI — CI runs `pytest` but never the generator, so
    the anchor `render(fixture) == tracked_header` is the only thing standing between a fixture edit
    and a silently stale header. I confirmed `uv run --script scripts/gen_ffi_boundary_vectors.py`
    leaves `git status --porcelain` empty.
- The full 17.8M-comparison sweep stays out of `pytest`, `mise run test` and the pre-push hooks —
    **do not wire it in**.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched this iteration.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips,
    snake_case exports. The local `.node` addon is gitignored — the recurring "checked-in stale
    artifact" claim stays refuted.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips; runs
    only under `wasm-pack test --node` (the CI `wasm` job).

## C FFI

**Status**: met — Unicode-gated this iteration (surface 9 of 11)

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs (unchanged); cbindgen
    header plus csbindgen C# generation still run from `build.rs`. No FFI source changed.
- `tests/test_iscc.c` is now **495** lines (was 459). A new helper `run_unicode_boundary_section()`
    drives both text functions over the generated vectors and frees every returned string; section
    29 adds three metadata guards (version `16.0.0`, counts 7 and 5) plus the 12 vectors. The review
    measured `80 passed, 0 failed` with the **verbatim** CI gcc line — no new `-I` was needed
    because the quoted `#include` resolves beside the includer.
- `tests/unicode_boundary_vectors.h` (65 lines, tracked, generated) renders the canonical
    `crates/iscc-lib/tests/unicode_boundary.json` as two `static const struct` arrays. Non-ASCII
    UTF-8 is emitted as **3-digit octal**, deliberately never `\x` (C hex escapes are greedy and
    unbounded).
- **Independently re-verified here, by methods the review did not use:** (1) I parsed the tracked
    header, decoded every octal escape back to a Python string and diffed name/input/expected
    against the fixture — 7 + 5 entries, **byte-identical, zero mismatches**; (2) I loaded
    `target/debug/libiscc_ffi.so` through `ctypes` and ran all 12 vectors through `iscc_text_clean`
    / `iscc_text_collapse` — **0 divergences**. The generator's `render()` also fails closed on a
    wrong `unicode_data_version`, a missing/empty section, a multi-input case, and a non-ASCII
    render.
- The generated header is correctly **absent** from `VENDORED_COPIES` (a derived artifact cannot
    satisfy a byte-identity gate); rationale is recorded in `decisions.md` 2026-07-27.
- No build-skip hazard: the `c-ffi` job is one plain `gcc` invocation plus the cbindgen freshness
    check, so nothing can report UP-TO-DATE and skip (unlike the Gradle trap found at 154).

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met as bindings; Java, Ruby, C#, Kotlin Unicode-gated — **C++ and Swift still ungated**

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    file moved this iteration.
- C# (154, 13 tests), Kotlin (154, 13 tests, fixture registered as a Gradle task **input**), Java
    (153, surefire CWD = pom basedir), Ruby (151, 12 vectors) — all zero-skip.
- `packages/go` sits at **177** `func Test`; its vendored `testdata/unicode_boundary.json` is
    byte-identical to the canonical file. **The go1.27 checklist item stands and is recorded nowhere
    but here:** go1.27 brings Unicode 17.0 tables, so Go will reacquire the `Final_Sigma` defect the
    core shed at 156 unless an equivalent case freeze lands there too. Standing ruling
    (`decisions.md` 2026-07-26): the Go skip map stays **unconditional**; the red is the intended
    trigger. CI pins Go via `go-version-file: packages/go/go.mod` (`go 1.26.1`).
- **C++ is the cheapest remaining surface and is nearly pre-wired.** Its
    `packages/cpp/tests/test_iscc.cpp` (397 lines) has no JSON reader, but
    `packages/cpp/CMakeLists.txt:19` already puts `../../crates/iscc-ffi/include` on the interface
    include path, and `include/iscc/iscc.hpp` already wraps both functions (`iscc::text_clean` L251,
    `iscc::text_collapse` L272). The header generated for C at 159 can be **reused as-is** — one
    extra include directory, no second generated artifact, no `VENDORED_COPIES` entry.
- **Swift is structurally different.** `Tests/IsccLibTests/ConformanceTests.swift` (215 lines)
    already parses JSON with `JSONSerialization` against a vendored `data.json` in the test
    directory, so it wants a *tracked vendored copy* of `unicode_boundary.json` (SwiftPM
    `resources:`) which **must** be registered in `VENDORED_COPIES`.
- Neither is locally buildable: `cmake` and `swift` are both **ABSENT** in this container (`gcc`,
    `g++`, `go` present). Both slices are CI-proof-only.

## Documentation

**Status**: met

- `docs/unicode.md` updated for the new surface: the propagation paragraph now names the C FFI test
    program and its generated header, and states that a pytest gate asserts byte-exact regeneration.
    Accurate against the code.
- `crates/iscc-ffi/CLAUDE.md` gained the header's path, the regeneration command and the
    do-not-hand-edit rule.
- Page-list machinery green: **23** documentation pages (24 tracked `docs/**/*.md` minus
    `docs/includes/abbreviations.md`, a snippet) consistent across `zensical.toml` nav,
    `ORDERED_PAGES` and `docs/llms.txt`. 12 crate/package READMEs; 12 crate/package CLAUDE.md files;
    11 `docs/howto/*.md`; `scripts/version_sync.py --check` **21/21** targets at 0.5.0.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
- `.iai-baseline.json` and `.crap-baseline.json` are **byte-untouched** (CRAP re-counted at **105**
    entries) — correct, since no Rust source moved and the new Python is fully covered by its own
    tests.

## CI/CD and Publishing

**Status**: partially met — green, and covering every line of working-tree code

- **Latest CI: GREEN.** check-runs API on `aa01784` (= `origin/develop`): **45 check-runs, 23
    distinct check names, 0 non-success, 0 in progress.** HEAD is `98e2964`, exactly **one** commit
    ahead (`cid(log): iteration 159`), and `git diff --stat origin/develop..HEAD -- . ':!.claude'`
    is **empty** — the green run genuinely covers all code, including the new C FFI slice.
- Job shape unchanged: **21 job keys → 22 jobs** (`python-test` is a `[3.10, 3.14]` matrix; `python`
    is an `if: always()` aggregator) **→ 23 check names**, matching the API exactly. The `c-ffi` job
    (`C FFI (cbindgen, gcc, test)`) needed no edit to pick up the new vectors.
- PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0" — **not shipped**; version is still
    **0.5.0**. (This is why runs appear 2×.)
- Iteration 159 completed cleanly: `iterations.jsonl` shows all four roles `OK` and an
    `iteration_summary` with verdict **PASS** (214 turns, 2,867 s). Working tree clean.
- `release.yml` / `docs.yml` byte-unchanged: 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only; `rubygems/configure-rubygems-credentials@main` still unpinned at
    `release.yml:895`. **`specs/ci-cd.md` remains drifted** — job table still 14 rows against 21
    real job keys. Owned by the existing human-authorized issue.
- Enforcing gates green: iai-callgrind perf (>10% Ir, baseline untouched), coverage + CRAP (105
    entries; `--fail-regression` is **CI-only**, so a green `mise run check` proves nothing),
    `cargo-deny` (live advisory DB — can red with no code change), docs page-list parity, and the
    Unicode sweep. `cargo-semver-checks` informational.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment; major dependency bumps **one per step** (xunit 3.x, Test.Sdk 18.x, Gradle wrapper,
    JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source rewrites, one crate per step); make the
    `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
None opened and none closed this iteration. Nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)".** (a1) and (a2) are ✅; the
    remainder is **(b)** propagation, now updated in place to record 9 of 11 surfaces with C++ and
    Swift outstanding.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Propagation slice 6 — the C++ boundary vectors.** It is the cheapest of the two remaining surfaces
by a wide margin, because the artifact it needs already exists: reuse
`crates/iscc-ffi/tests/unicode_boundary_vectors.h` rather than generating a second copy.
`packages/cpp/CMakeLists.txt:19` already exposes `../../crates/iscc-ffi/include`, so the slice is
one added include directory (`../../crates/iscc-ffi/tests`) plus a vector loop in
`packages/cpp/tests/test_iscc.cpp` calling `iscc::text_clean` / `iscc::text_collapse` (both already
wrapped in `include/iscc/iscc.hpp`). No new generator, no new tracked artifact, and — since the
header is derived — **no `VENDORED_COPIES` entry**; the existing pytest regeneration anchor already
covers drift for both consumers. Expect the `cpp` job (cmake + ASAN) to be the only proof: `cmake`
is absent in this container, so the advance agent must not claim a local run.

Swift is the harder remainder and should follow, not lead: it needs a *tracked vendored copy* of
`unicode_boundary.json` under `packages/swift/Tests/IsccLibTests/` declared as a SwiftPM
`resources:` entry, registered in `VENDORED_COPIES`, and read through `Bundle.module` — and `swift`
is likewise absent locally.

Lighter human-authorized alternatives if a smaller step is wanted: pin
`rubygems/configure-rubygems-credentials` to `@v2.1.0` + `# exact tag:` comment (one line, still
`@main` at `release.yml:895`); or make the `specs/ci-cd.md` job table exhaustive (14 rows vs 21 job
keys).

Standing hazards for any fixture work: never write `unicode_boundary.json` through the Write/Edit
tools (the `\uXXXX` escapes decode to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`);
do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A`; keep any generated file pure
ASCII, LF-only, one trailing newline, or the prek hygiene hooks will rewrite it and red the
regeneration gate. Adding a *vector* to the canonical fixture remains a separate, deliberate slice —
**ten** consumers now assert exactly 7 `text_clean` + 5 `text_collapse`, so a new row reds them all
at once. Propagation invariant re-verified this iteration:
`git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** tracked paths, matching
`VENDORED_COPIES`.
