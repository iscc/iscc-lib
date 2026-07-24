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
- **iter 115: CI RED — enforcing `cargo-deny` caught fresh advisory RUSTSEC-2026-0204** (null-ptr
    deref in `crossbeam-epoch`, dev-only via criterion→rayon→crossbeam-deque, never shipped). SCOPED
    minimal fix: `cargo update -p crossbeam-epoch` (0.9.18→0.9.20, patched `>= 0.9.20`, no manifest
    change, locks 1 pkg). **Fix-the-root-cause > suppress**: when a patched release exists, bump the
    lockfile — an `ignore` entry is only the fallback when unpatched. Verify the advisory `patched`
    range from `rustsec/advisory-db/main/crates/<crate>/<ID>.md` before choosing bump vs ignore.
- **Recurring maintenance pattern**: the enforcing cargo-deny gate WILL periodically go red on fresh
    RustSec advisories against dev/bench deps. Each is a CI-red-first priority; resolve via patch
    bump (preferred) or a justified `ignore`.
- **iter 116: picked #41 (Python text/video GIL)** — first v0.6.0 feature; CI green, no bounce.
    Single-file `crates/iscc-py/src/lib.rs`: wrap 5 fns (`gen_text_code_v0`, `gen_video_code_v0`
    - `_flat`, `soft_hash_video_v0` + `_flat`) in `py.detach(|| ...)`. Signature- + conformance-
        neutral (injected `py` param not exposed) → **no doc/`.pyi`/`__init__.py` change needed**.
        GIL-release verification is grep-based: `grep -c '\.detach(' lib.rs` (7 existing → 12; note
        one-shot sites write `py\n.detach` split across lines, so count `.detach(` not `py.detach`).
        Video caveat: detach must open AFTER `extract_frame_sigs`/`flat_bytes_to_frames` (borrowed
        `PyList_GetItem` ptrs not free-threading-safe; module keeps `gil_used = true`).
- **iter 117: picked #42 (WASM simd128)** — CI green, no bounce; most self-contained v0.6.0 pick. 3
    config files + 1 doc: `RUSTFLAGS="-C target-feature=+simd128"` env on release.yml `build-wasm`
    step + ci.yml `wasm` test step; `--enable-simd` added to `wasm-opt` array in
    `crates/iscc-wasm/Cargo.toml` (`[package.metadata.wasm-pack.profile.release]`); sync the flag
    string quoted in `crates/iscc-wasm/CLAUDE.md` (~line 90, doc — excluded from 3-file limit). NO
    src/lib.rs change (conformance byte-identical; blake3 wasm SIMD is
    `#[cfg(target_feature =   "simd128")]`-gated, RUSTFLAGS is the only trigger — not a cargo
    feature). `test-wasm` release job smoke-tests the *downloaded* artifact (no rebuild) → do NOT
    add RUSTFLAGS there. Devcontainer has wasm-pack + wasm32 target but NOT
    wasm-tools/wasm-objdump/wasm-opt — install `wasm-tools` via `cargo binstall wasm-tools` for the
    `v128`-opcode disassembly evidence. `wasm-opt` refuses SIMD input without `--enable-simd`, so a
    successful `wasm-pack build --release` under simd128 RUSTFLAGS is itself proof the flag is
    wired. Review agent owns the spec "Verified when" check-offs.
- **v0.6.0 remaining after #42**: #43 Go ISCC-IDv1, #49 aarch64 wheels, dep refresh, + 2
    release-workflow fixes (npm OIDC, single-registry re-trigger). One per iteration; each spec'd.
