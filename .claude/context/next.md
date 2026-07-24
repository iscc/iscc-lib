# Next Work Package

## Step: Activate BLAKE3's wasm32 SIMD backend in @iscc/wasm (finish issue #42)

## Goal

Add the missing piece that makes the already-landed `simd128` flags actually select BLAKE3's
hand-written `wasm32` SIMD backend instead of the portable scalar path: a direct
`blake3 = { workspace = true, features = ["wasm32_simd"] }` dependency on `crates/iscc-wasm`. This
is a **reframe** of issue #42 (first NEEDS_WORK, iter 117) — the prior premise ("RUSTFLAGS `simd128`
alone activates the backend") was proven wrong by the review agent + Codex against the blake3 1.8.3
source; the corrected design enables the gating `blake3/wasm32_simd` Cargo feature. Completes the
last open WASM v0.6.0 target gap.

## Scope

- **Modify**: `crates/iscc-wasm/Cargo.toml` — add
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` under `[dependencies]`.
- **Modify (docs, excluded from 3-file limit)**:
    - `crates/iscc-wasm/CLAUDE.md` — in the build/module section, document that iscc-wasm carries a
        direct `blake3` dependency solely to enable the `wasm32_simd` Cargo feature (feature-unifies
        for the wasm-only build); note it must not be removed as "unused".
    - `.claude/context/specs/wasm-bindings.md` — correct the now-inaccurate prose only: the "WASM
        SIMD" section says the backend is "selected at compile time via `target_feature = "simd128"`"
        and calls it a "Pure build-flag change". Amend to state the backend also requires the
        `blake3/wasm32_simd` **Cargo feature** (a direct dep on iscc-wasm), in addition to the
        `simd128` target-feature. Do NOT touch the `**Verified when:**` checkboxes.
- **Reference**: `.claude/context/handoff.md` (iter-117 review + recommended fix),
    `.claude/context/issues.md` (#42 review note), `.github/workflows/ci.yml` +
    `.github/workflows/release.yml` (RUSTFLAGS already landed — keep), the blake3 1.8.3 source
    (`build.rs` `is_wasm32_simd()`/`build_wasm32_simd()`; `src/platform.rs` `detect()` returning
    `WASM32_SIMD` under `#[cfg(blake3_wasm32_simd)]`).

## Not In Scope

- **Do NOT revert** the already-landed `RUSTFLAGS="-C target-feature=+simd128"` (ci.yml/release.yml)
    or `--enable-simd` (Cargo.toml wasm-opt array) — both remain required (wasm32_simd.rs uses
    `core::arch::wasm32` v128 intrinsics that need the target-feature to compile; wasm-opt needs
    `--enable-simd` to accept the output).
- Do NOT check off the four `specs/wasm-bindings.md` "Verified when" boxes or delete issue #42 — the
    review agent owns box-checking and issue resolution after verifying the fix.
- Do NOT add computation/logic or `use blake3` to `iscc-wasm/src/lib.rs`; the dep is
    feature-unification only. (No `unused_crate_dependencies` lint is enabled, so no silencer is
    needed — confirmed no `[lints]` section and no `#![warn(...)]` in lib.rs.)
- Do NOT touch `crates/iscc-lib/Cargo.toml` or the root workspace `blake3 = "1"` — enabling the
    feature there would (harmlessly but needlessly) unify onto native builds; keep it scoped to the
    wasm crate.
- Do NOT bump blake3 or any other dependency (dependency refresh is a separate v0.6.0 issue), and do
    not pick up the other v0.6.0 items (#43 Go ISCC-IDv1, #49 aarch64 wheels, release-infra fixes).

## Implementation Notes

- The one-line dependency addition is the whole fix. Root workspace already declares `blake3 = "1"`
    (locked 1.8.3, has a `wasm32_simd` feature); `iscc-lib` uses `blake3.workspace = true`. Add the
    same workspace dep to `iscc-wasm` but with `features = ["wasm32_simd"]`.
- Why this works: blake3 `build.rs` runs `build_wasm32_simd()` (emits
    `cargo:rustc-cfg=blake3_wasm32_simd`) only when `is_wasm32() && is_wasm32_simd()`, where
    `is_wasm32_simd()` = `defined("CARGO_FEATURE_WASM32_SIMD")`. iscc-wasm compiles to
    `wasm32-unknown-unknown`, so with the feature on, the cfg is emitted and `Platform::detect()`
    returns `WASM32_SIMD` (compile-time, no runtime check). No other binding depends on iscc-wasm.
- Why native builds are unaffected: on non-wasm targets `is_wasm32()` is false, so build.rs never
    emits the cfg and `wasm32_simd.rs` (which is `#[cfg(blake3_wasm32_simd)]`-gated) is never
    compiled. Feature-unification onto a host build of blake3 is therefore inert.
- The honest, deterministic proof that the backend is compiled in is the `cargo tree` feature-graph
    check below (blake3 shows `wasm32_simd` for the wasm32 target). Prefer this over `v128`
    opcode-counting, which is a false-positive (LLVM auto-vectorizes the portable path too).
- Optional supporting evidence for the CLAUDE.md/spec note (not a gating criterion, noisy): a
    before/after `SumHasher` throughput measurement on a few-MB buffer under
    `wasm-pack test --node`. Only capture it if quick; the review agent owns the spec-box sign-off.

## Verification

- Feature wired (deterministic before/after):
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -f "{p} {f}" -i blake3` output contains
    `wasm32_simd` (i.e. piping to `grep -q wasm32_simd` exits 0). Was `default,std` before this
    step.
- Conformance preserved on the SIMD build:
    `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    passes (9 conformance + 78 unit tests, output byte-identical).
- Release build compiles under the feature and wasm-opt accepts it:
    `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release crates/iscc-wasm --features conformance`
    exits 0.
- Native build unaffected: `cargo test -p iscc-lib` passes (host build; `blake3_wasm32_simd` cfg not
    emitted).
- Landed flags still present: `grep -q 'target-feature=+simd128' .github/workflows/ci.yml`,
    `grep -q 'target-feature=+simd128' .github/workflows/release.yml`, and
    `grep -q 'enable-simd' crates/iscc-wasm/Cargo.toml` all exit 0.
- `mise run format` reports clean (no diffs to re-stage).

## Done When

`cargo tree` shows blake3's `wasm32_simd` feature enabled for the wasm32 build, conformance passes
on the SIMD build, the release build compiles, native tests are unaffected, and the
previously-landed `simd128`/`--enable-simd` flags remain in place.
