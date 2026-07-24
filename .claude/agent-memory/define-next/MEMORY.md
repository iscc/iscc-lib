# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next".
- Critical issues always take priority regardless of feature trajectory.
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale;
    always read issues.md directly (review agent can miscount).
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in the same crate/2 files).
- **IDLE is valid** when all target sections met and only `low` issues remain — don't invent work.
    But an already-specced, locally-verifiable target gap is NOT idle work.
- **Prefer boolean-verifiable prerequisites over high-impact-but-risky infra fixes** when both are
    available and no feature is in-progress. But infra/release fixes CAN be locally verifiable (napi
    bundled loader #38, cargo-deny reads Cargo.lock not artifacts) — don't default to "release-only
    → too risky".
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — if the Write tool is blocked, use `cat > file << 'EOF'` via
    Bash.
- Run `mise run format` before committing next.md + memory (pre-push mdformat rejects the batch).

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports, bound in all languages.
- Go bindings are pure Go (no CGO/WASM/binaries). `gen_iscc_code_v0` vectors have no `wide` — pass
    `false`. `"stream:<hex>"` prefix = hex-encoded byte data.
- **data.json copies** (all identical, update together): `crates/iscc-lib/tests/`,
    `packages/go/   testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`. Binding crates with
    hardcoded vector-count asserts (Rust core + WASM) must be bumped when vendoring new vectors.
- **SumHasher** lives at `iscc_lib::streaming::SumHasher` (NOT Tier 1; count stays 32).

## Dev Environment Constraints

- **No Swift toolchain / no shellcheck** in the Linux devcontainer — `swift test` + shell lint are
    CI/macOS only.
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`.
- UniFFI 0.31.0; SPM module name MUST be `iscc_uniffiFFI`; UniFFI can't export `const` (getters) or
    `usize`/borrowed/generic exports.
- cargo-crap 0.2.2, cargo-llvm-cov 0.8.7, valgrind 3.19 + iai-callgrind-runner ARE installed.
    cargo-deny is NOT preinstalled but installs via `cargo binstall cargo-deny@0.19.9 --force`.

## CI/Release, Docs, Gotchas

- Release: `workflow_dispatch` with 9 per-registry checkboxes; version_sync.py manages 16 targets
    (`--check` exits 1 on mismatch). `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb` in
    Rust CI). XCFramework cache key must hash all build inputs.
- Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need an entry per new howto
    guide (template `docs/howto/dotnet.md`; collapsible `??? tip "Build from source"`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; Gson Maven groupId `com.google.code.gson`.
- "10 gen functions" vs "9 conformance functions" — no blanket 9→10 find/replace (corrupts
    conformance-scoped files). See learnings.md.
- Language API name styles (doc examples): Ruby/Rust `snake_case`; C#/Go/Java `PascalCase`; JS/
    Swift/Kotlin `camelCase`; Swift/Kotlin take named `bits`/`u` params.

## v1.0.0 Hardening Phase — COMPLETE (iters 86–114); detail in MEMORY-archive.md + learnings.md

All autonomous v1.0.0-hardening gates landed and are enforcing/green: module-visibility narrowing,
SumHasher (#37), PyO3 0.29 (#1), npm bundled loader (#38), `cargo-semver-checks` (informational — do
NOT flip to enforcing until the human-gated v1.0.0 cut), CRAP Phases 1–3 + `--fail-above`,
iai-callgrind perf gate (#3), and `cargo-deny` supply-chain gate (#114). Residual facts:

- **Semver + Coverage/CRAP + Perf + Audit are the 4 quality gates.** CRAP + Perf + Audit enforcing;
    Semver `continue-on-error: true` until v1.0.0 cut.
- **Baselines are committed + refreshed by deliberate `mise run` reviewed commits**
    (`.crap-baseline   .json`, `.iai-baseline.json`) — never auto-committed from CI (push race).
    Regenerate from CI's artifact if flapping; don't widen `--epsilon`.
- **cargo-deny** reads Cargo.lock + metadata (NOT compiled) → local `cargo deny check` green is
    authoritative. `deny.toml`: config v2, `[graph] all-features`, `yanked/multiple-versions`,
    `private = { ignore = true }`, license allow-list (Unicode-3.0/Zlib/BSL-1.0/MPL-2.0). Dead
    pre-0.14 schema traps + license-graph detail in learnings.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released to all registries; all 12 bindings meet core criteria. Human raised target.md bar
    - filed 5 `normal` `[human]` v0.6.0 issues, each with a spec: #41 Python text/video GIL release,
        #42 WASM simd128 build flags, #43 Go ISCC-IDv1 encode/decode, #49 restore linux/aarch64 Python
        wheels, "Dependency review and refresh" (add Dependabot/Renovate). Plus 2 `normal` release-
        workflow reliability issues (npm OIDC, single-registry re-trigger). v1.0.0 cut +
        Semver-enforcing still HELD by Titusz (`low`).
- **iters 115–119 DONE (detail in MEMORY-archive.md)**: 115 cargo-deny advisory bump
    (crossbeam-epoch, CI-red-first); 116 #41 Python GIL detach; 117→118 #42 WASM SIMD (reframe:
    needs the `blake3/wasm32_simd` Cargo feature, not just RUSTFLAGS); 119 #43 Go ISCC-IDv1.
- **Recurring**: the enforcing cargo-deny gate WILL periodically go red on fresh RustSec advisories
    vs dev/bench deps — CI-red-first priority; prefer `cargo update -p <crate>` (patch bump) over a
    `deny.toml` ignore when a patched release exists (check `patched` range in advisory-db first).
- **iters 120–122 DONE (trailing-byte hardening saga; detail in learnings.md + MEMORY-archive.md)**:
    120 Go `IsccDecode` "too long" branch (`[review]`); 121 Rust-core `iscc_decode` "too long" guard
    (11 bindings inherit) — but the added branch tripped the CI-only CRAP `--fail-regression` gate;
    122 CI-RED-FIRST `.crap-baseline.json` refresh via `mise run crap:baseline`. **Root lesson: CRAP
    regression gate is CI-ONLY (not in `mise run check`/pre-commit)** — any step adding a
    branch/loop to a covered fn MUST refresh the baseline in the SAME step (this is exactly how iter
    121 slipped).
- **iter 123 DONE (#49 aarch64 Python wheels, CI GREEN)**: 1 file `.github/workflows/release.yml` —
    added `ubuntu-24.04-arm`/`aarch64`/`python3.10` `build-wheels` entry (native ARM, NOT QEMU) +
    matrixified `test-wheels`. **Release-only infra → verification is STATIC** (pyyaml `safe_load`
    via `uv run python` + grep presence + `mise run check`); CID can't dispatch a real ARM release.
    Dev env: no actionlint/yamllint/system-pyyaml; pyyaml IS reachable via `uv run python`.
- **iter 124: CI GREEN (30/30) → started the dep refresh, sliced per-ecosystem** (handoff mandate:
    it spans ~12 manifests, does NOT cite `[audit]`, so no 8-file valve — must be several small
    steps: Rust lock → Rust direct pins → Python `uv.lock` → each binding-manifest group → tooling
    pins). **Slice 1 = pure `cargo update`** (Cargo.lock only, a generated file → 0 source files).
    Safe because caret ranges stay put: `blake3 1.8.3→1.8.5`, `napi 3.8.3→3.11.0`,
    `uniffi   0.31.0→0.31.2` (0.32.0 available but HELD — needs Swift/Kotlin re-verify),
    `wasm-bindgen   0.2.125→0.2.126`, ~100 transitive. **Do NOT touch `Cargo.toml` pins this slice**
    (uniffi/pyo3-#41/ criterion/iai/magnus-rb_sys/jni/napi all held for individual eval). Two real
    risks: (a) cargo-deny can flip red on a new transitive license/advisory → run `mise run audit`,
    prefer `cargo update -p X --precise <patched>` over a deny.toml ignore; (b) iai perf gate
    (CI-only, ≤10% Ir) can drift on blake3 → `mise run bench:iai:check`, refresh
    `.iai-baseline.json` via `mise run bench:iai:baseline` in-step if legit. CRAP untouched (no
    source change). Dev env has cargo-deny 0.19.9 + libclang-14 + valgrind, so
    `mise run test/lint/audit/bench:iai:check` all run locally.
- **iter 125: CI GREEN (slice 1 landed clean) → dep-refresh slice 2 = Python `uv.lock`** (handoff's
    "cleaner mirror" candidate over Rust direct-pin eval). Run `uv lock --upgrade` at repo ROOT
    (regenerates `/uv.lock`, 2035 lines, generated → 0 source files). **Two separate uv projects:**
    root `/uv.lock` (dev tools + `iscc-core` + zensical/docs — the real one) and
    `crates/iscc-py/uv.lock` (7 lines, NO runtime deps → refresh is a no-op; don't touch). **Dev
    deps are all UNCONSTRAINED** (`"ruff"`, `"pytest"`, `"ty"`, `"mdformat"`, `"zensical"`… — no
    version pins), so `--upgrade` pulls absolute latest → biggest risk is a tool major changing
    behavior (ruff rules, mdformat reformat, ty type errors, zensical/mkdocstrings docs break).
    Handling: pin the ONE offending tool back in `pyproject.toml` `[dependency-groups] dev` with an
    inline hold-back comment (keeps diff lockfile-only), NEVER disable a rule/skip a test/weaken a
    gate. Verify: `uv lock --check` (working-tree consistency check, survives commit) +
    `mise run test/lint/check` + docs (`uv run zensical build`,
    `uv run python scripts/gen_llms_full.py`). CI Python job installs from the lock via
    `uv sync --group dev`; `docs.yml` runs zensical. `iscc-core` conformance is vs vendored
    `data.json` (authoritative) — comparative tests must still pass. uv 0.11.32.
- **v0.6.0 remaining after slice 2**: rest of dep refresh (Rust direct pins, binding manifests,
    tooling pins) + 2 release-workflow fixes (npm OIDC, single-registry re-trigger, both
    human-gated). One slice/issue per iteration; each spec'd.
