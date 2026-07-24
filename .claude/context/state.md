<!-- assessed-at: 8930cba8b21a78f96f5efa150a52f07043b0f835 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI RED: CRAP regression gate fails on iter-121 `iscc_decode` change (baseline not updated)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 121 landed the Rust core `iscc_decode` trailing-byte rejection
(`crates/iscc-lib/src/lib.rs`), closing the codec-wide alias gap for the stability-committed core
and all 11 delegating bindings — the fix is correct and conformance-safe. **However, CI is now RED
on the develop tip (`880ec5e`):** the added branch raised `iscc_decode`'s cyclomatic complexity
above its committed CRAP baseline, and the enforcing `Coverage + CRAP` job fails its
`--fail-regression` gate (`↑ 1 regressed`). Every other CI job is green. This slipped through
because the review agent's `mise run check` (pre-commit hooks) does not run the CRAP regression gate
— it is CI-only. **Fixing CI (updating `.crap-baseline.json`) is the top priority.**

## Rust Core Crate

**Status**: partially met — **CI RED** (CRAP regression on `iscc_decode`)

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- **Trailing-byte hardening LANDED (iter 121, prior `[review]` issue RESOLVED & deleted)**:
    `iscc_decode` (`crates/iscc-lib/src/lib.rs`) now has both a "too short" (`tail.len() < nbytes`,
    line 235) and a new "too long" (`tail.len() > nbytes`, line 241) rejection branch, so trailing
    padding (canonical + `"AA"`) errors instead of aliasing. Docstring updated (line 225), new test
    `test_..._rejects_trailing_bytes` (asserts "too long" at ~line 2029), "too short" test (~line
    1554\) intact, `iscc_decompose` body loop (~line 962) untouched. Review verdict PASS; conformance
    holds. Note: ISCC-IDv1 (`ISCC:MAIGHFECJMOPMIABAA`) is rejected earlier at the header level in
    the Rust core (`codec::Version` is V0-only) — the Rust alias gap applied to all V0 codes;
    ISCC-IDv1 support remains Go-only.
- **CI-BLOCKING REGRESSION (introduced by the fix above)**: the `Coverage + CRAP` job's Phase-3
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` step
    exits 1 with `↑ 1 regressed`. The regressed function is `iscc_decode`: baseline is
    `cyclomatic 4.0 / crap 4.11` (`.crap-baseline.json` line ~277), and the added branch pushes it
    above that. This is a legitimate, well-tested complexity increase far below the 30.0
    `--fail-above` threshold — the fix is to **refresh the `iscc_decode` entry in
    `.crap-baseline.json`** (regenerate the baseline, or hand-update cyclomatic/coverage/crap), not
    to revert the guard. `--fail-above` is NOT the trigger (max CRAP is `gen_meta_code_v0` at 22.3,
    under the 30.0 cap).
- **Perf gate — COMPLETE, ENFORCING** (unchanged): `iai_benches.rs` (iai-callgrind 0.16, 11
    `bench_*` → 16 cases), `Perf (iai-callgrind)` job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` is
    `continue-on-error:   true`, reports `success`; enforcing v1.0.0 criterion (`rust-core.md`
    semver box `[ ]`) stays unmet — held. `decisions.md` (2026-07-24) records that tightening decode
    input-validation is not a SemVer break, for the eventual enforcing gate.
- Workspace version is `0.5.0`.

## Python Bindings

**Status**: partially met — one v0.6.0 target gap open (aarch64 wheels)

- Core met: all symbols exported, Python 3.10 + 3.14 CI jobs GREEN, ruff clean, streaming
    `SumHasher` wrapper present. PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- **GIL release COMPLETE for all heavyweight compute paths (issue #41 RESOLVED)**: `py.detach` wraps
    pure-Rust compute in all entry points (data/instance/image/sum + text/video + 3 streaming
    `update()`), **12 total detach sites**. Video detach opens strictly AFTER frame-signature
    extraction. Meta/audio/mixed stay attached by design.
- **Gap (v0.6.0, issue #49)**: `linux/aarch64` (`manylinux_2_17_aarch64`) wheels are not built —
    only x86_64/universal2/win_amd64 ship. Target requires Linux x86_64 **and aarch64** wheels. Plan
    at `.claude/plans/restore-linux-aarch64-python-wheels.md`.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; Node.js CI job GREEN. Bundled model in
    `package.json`.

## WASM Bindings

**Status**: met — issue #42 (SIMD) resolved iteration 118, CI-verified

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class, `WASM (wasm-pack test)`
    job GREEN.
- **SIMD backend active (issue #42 DONE)**: `crates/iscc-wasm/Cargo.toml` carries a direct
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` dep (exists solely for feature
    unification — no `use blake3` in source, must not be pruned as "unused"). `simd128` RUSTFLAGS
    (ci.yml + release.yml) and `--enable-simd` wasm-opt flag remain in place. All four
    `specs/wasm-bindings.md` "WASM SIMD" boxes checked.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; C FFI CI job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; Java (Maven) CI job GREEN.

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120

- Core met: pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` CI job GREEN,
    `CGO_ENABLED=0` holds.
- **ISCC-IDv1 support DONE (issue #43)**: `packages/go/iscc_id.go` adds `EncodeIsccID` /
    `DecodeIsccID` + `IsccIDv1Result`, `codec.go` adds `VSV1` const and a MainType-ID-only Version=1
    relaxation in `decodeHeader`. All 5 `specs/go-bindings.md` "verified when" boxes checked.
- **Trailing-byte hardening DONE (iter 120)**: `IsccDecode` (`codec.go`) has both a "too short"
    (line ~594) and a "too long" (line ~597) rejection branch; `DecodeIsccID` inherits via
    delegation; `IsccDecompose` untouched. The equivalent Rust-core fix (iter 121, above) is
    independent — Go reimplements the codec natively, not via FFI.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; Ruby CI job GREEN; version synced.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over C FFI; C#/.NET CI job GREEN.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan; C++ CI job GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations; shared by Swift + Kotlin.

## Swift Bindings

**Status**: met

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build; Swift CI job GREEN.
    XCFramework checksum updated for v0.5.0.

## Kotlin Bindings

**Status**: met

- packages/kotlin/ with JNA-loaded UniFFI bindings, 9 desktop+Android targets; Kotlin CI job GREEN.

## README

**Status**: met

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes.

## Per-Crate READMEs

**Status**: met

- READMEs present for all 12 crates/packages; registry metadata references them.

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, 11 language howto guides, tabbed examples, llms-full.txt, benchmarks page with speedup
    factors all present.
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic only.

## Benchmarks

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2), Bench (compile check) CI job GREEN, 18
    pytest-benchmark functions, speedup factors published (1.3x-158x) in docs/benchmarks.md.
- Second iai-callgrind harness enforcing >10% Ir regression gate (green).

## CI/CD and Publishing

**Status**: partially met — **CI RED** on the develop tip (Coverage + CRAP regression)

- **LATEST CI RUN — FAILURE.** develop tip `880ec5e` (HEAD `8930cba` is a +1 log-only commit;
    origin/develop == `880ec5e`). The **only** failing job is
    `Coverage + CRAP (cargo llvm-cov +   cargo crap)` — it fails on both workflow runs for the
    commit: https://github.com/iscc/iscc-lib/actions/runs/30113161728/job/89547333867 and
    https://github.com/iscc/iscc-lib/actions/runs/30113159676/job/89547326183. All other jobs (Rust,
    all 12 bindings incl. Go, Perf, Audit (cargo-deny), Semver, Version consistency, WASM, Bench,
    cargo-crap) report `success`.
- **Root cause**: the enforcing `--fail-regression` CRAP gate detected `↑ 1 regressed` — the
    `iscc_decode` change (iter 121) raised its cyclomatic complexity above the `.crap-baseline.json`
    entry (`cyclomatic 4.0 / crap 4.11`). Fix = refresh that baseline entry (the added complexity is
    legitimate and well below the 30.0 `--fail-above` threshold). The CRAP regression gate is not
    part of `mise run check`/pre-commit, so it was not caught before push.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` CI job GREEN.
    NOTE: a future live-advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore when a patch exists).
- **Gap (v0.6.0, CI/CD)**: dependency freshness — no Dependabot/Renovate config
    (`.github/dependabot.yml`, `renovate.json` both absent).
- **Gap (v0.6.0, CI/CD)**: Python wheel matrix must cover aarch64 (see Python section, issue #49).
- v0.5.0 published; release workflow with per-registry toggles + version sync (16 targets) in place.
    Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated / `normal`.

## Open Issues (issues.md lists 6 — 0 critical, 4 normal, 2 low; all `[human]`)

The iter-121 CRAP CI failure is **not** an issue.md entry — it is a live CI regression that the next
step must fix directly (top priority). No open issue is CI-blocking. Any open issue keeps the
project IN_PROGRESS. There is currently **no CID-actionable `[review]` issue** (the Rust-core
trailing-byte `[review]` issue was resolved and deleted this iteration).

v0.6.0-scoped (`normal`, `[human]`, each with a spec):

- Restore linux/aarch64 Python wheels (#49)
- Dependency review and refresh across the project

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**TOP PRIORITY — fix CI (RED).** The `Coverage + CRAP` job fails its `--fail-regression` gate
because the just-landed `iscc_decode` trailing-byte guard raised the function's cyclomatic
complexity above its committed baseline. Update the `iscc_decode` entry in `.crap-baseline.json` to
the new value — regenerate the baseline from a fresh `cargo llvm-cov` + `cargo crap` run (preferred,
so cyclomatic/coverage/crap are exact), or hand-edit the single entry at line ~277. The new score is
legitimate and far below the 30.0 `--fail-above` threshold, so no source revert is warranted; the
robustness fix stays. Verify by re-running the `Coverage + CRAP` job (or
`cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` locally)
until it exits 0. Guard for the future: the CRAP regression gate is CI-only (not in
`mise run check`), so any change that adds a branch/loop to a covered function needs a baseline
refresh in the same step.

After CI is green: the remaining CID-doable v0.6.0 targets are #49 (aarch64 Python wheels; plan at
`.claude/plans/restore-linux-aarch64-python-wheels.md`) and the project-wide dependency
review/refresh (both `normal` `[human]`, both actionable). Release-reliability issues (npm OIDC,
single-registry re-trigger) stay human-gated. Do NOT cut v1.0.0 or flip the `Semver` gate to
enforcing — both deliberately held by Titusz. Watch for the enforcing `Audit (cargo-deny)` gate
turning red on a fresh live advisory (prefer `cargo update -p <crate>` over a `deny.toml` ignore).
