# Handoff

## 2026-07-24 — Review of: Enable WASM simd128 in the @iscc/wasm release + CI builds (issue #42)

**Verdict:** NEEDS_WORK

**Summary:** The advance agent faithfully implemented exactly what next.md specified — step-level
`RUSTFLAGS: -C target-feature=+simd128` on the release `build-wasm` + CI `wasm` steps and
`--enable-simd` in the wasm-opt array — and every literal verification criterion passes. However,
next.md's core technical premise is factually wrong for the locked `blake3 1.8.3`: setting the
`simd128` target-feature alone does **not** activate BLAKE3's `wasm32` SIMD backend. That backend is
gated behind the `blake3/wasm32_simd` **Cargo feature** (build.rs emits the `blake3_wasm32_simd` cfg
solely from `CARGO_FEATURE_WASM32_SIMD`), which is not enabled anywhere in the graph
(`blake3 = "1"`, default features). So BLAKE3 stays on `Platform::Portable` — the stated Goal ("so
`blake3` uses its `wasm32` SIMD backend instead of the portable scalar fallback") is not achieved.
The change is a safe, beneficial prerequisite (it's necessary and enables LLVM auto-vectorization),
but issue #42 is **not resolved**.

**Verification:**

- [x] `grep -q 'target-feature=+simd128' .github/workflows/release.yml` — PASS
- [x] `grep -q 'target-feature=+simd128' .github/workflows/ci.yml` — PASS
- [x] `grep -q 'enable-simd' crates/iscc-wasm/Cargo.toml` — PASS
- [x] `grep -q 'enable-simd' crates/iscc-wasm/CLAUDE.md` — PASS
- [x] `wasm-pack test --node ... --features conformance` under SIMD compile — PASS (9 conformance +
    78 unit, output byte-identical)
- [x] `wasm-pack build --target web --release` exits 0 (wasm-opt accepts SIMD input) — PASS
- [x] `wasm-tools print ... | grep -c v128` > 0 — literally PASS (1993), **but see below**: v128
    presence is a FALSE-POSITIVE for the goal (LLVM auto-vectorizes the portable path)
- [x] `mise run format` / `mise run check` — PASS (all 15 hooks clean)
- [ ] **GOAL — BLAKE3 uses its `wasm32` SIMD backend** — FAIL. `Platform::detect()` returns
    `WASM32_SIMD` only under the `blake3_wasm32_simd` cfg, which requires the `blake3/wasm32_simd`
    Cargo feature (unset). BLAKE3 stays `Platform::Portable`.

**Issues found:**

- **Goal not met (blocking):** `RUSTFLAGS="-C target-feature=+simd128"` is necessary but NOT
    sufficient to activate BLAKE3's hand-written WASM SIMD backend under blake3 1.8.x. Verified by
    reading `blake3-1.8.3/build.rs` (`is_wasm32_simd()` = `defined("CARGO_FEATURE_WASM32_SIMD")`;
    the `blake3_wasm32_simd` cfg is emitted only when that feature is on) and
    `blake3-1.8.3/src/platform.rs` (`detect()` returns `WASM32_SIMD` only under
    `#[cfg(blake3_wasm32_simd)]`, else `Portable`). Our graph has `blake3 = "1"` with default
    features only, so the backend is never compiled in.
- **Weak verification criterion:** spec "Verified when" #4 accepts "`v128` opcodes in the
    disassembly" as evidence, but LLVM auto-vectorization of the portable BLAKE3/CDC/minhash code
    emits `v128` too — so opcode-counting cannot distinguish "BLAKE3 SIMD backend active" from
    "portable path auto-vectorized". The reliable signal is a before/after `SumHasher` throughput
    measurement on a few-MB buffer (which the spec offers as the alternative for #4). Left the four
    spec "Verified when" boxes UNCHECKED — the goal is not actually met.

**Codex review:** **Confirmed and correct — this is the load-bearing finding.** Codex \[P2\]: "With
the locked `blake3 1.8.3` dependency, this target flag alone does not select BLAKE3's SIMD backend:
its build script enables `blake3_wasm32_simd` only when the `wasm32_simd` Cargo feature is active...
The published artifact therefore keeps using `Platform::Portable`; any observed `v128` instructions
may merely be LLVM auto-vectorization. Wire `blake3/wasm32_simd` into the WASM build in addition to
these flags." I independently verified this against the blake3 source and it is accurate.

**Next:** Re-scope #42 to add the missing piece. Recommended fix: give `crates/iscc-wasm/Cargo.toml`
a **direct** `blake3 = { workspace = true, features = ["wasm32_simd"] }` dependency. Because
features unify across the build graph and iscc-wasm only ever compiles to `wasm32` (and no other
binding depends on iscc-wasm), this enables `blake3_wasm32_simd` for the wasm build only, without
touching `iscc-lib` or affecting native builds. Keep the already-landed
`RUSTFLAGS="-C target-feature=+simd128"` and `wasm-opt --enable-simd` — they remain required (so
`wasm32_simd.rs` compiles and wasm-opt accepts the output). **Verification must go beyond `v128`
opcode counting**: capture a before/after `SumHasher` throughput number on a few-MB buffer to prove
`Platform::WASM32_SIMD` is actually selected (blake3 does not expose the chosen platform publicly,
so throughput is the honest signal). After landing, the review agent can check the four
`specs/wasm-bindings.md` "Verified when" boxes and delete issue #42. The already-committed
release.yml/ci.yml/Cargo.toml/CLAUDE.md changes are correct and can be built on — do not revert
them.

**Notes:**

- No push this cycle (NEEDS_WORK). origin/develop stays at `2ffc8f9`; the unpushed batch
    (update-state + define-next + advance + this review) will ride out with the follow-up fix.
- Scope was clean and disciplined: the diff touched only the four intended files (+ advance memory +
    handoff). No gate circumvention anywhere in the `origin/develop..HEAD` batch. No source changes,
    no API break, conformance byte-identical. The problem is purely that the change is incomplete,
    not that anything is wrong with what shipped.
- The built `pkg/` is self-gitignored (`pkg/.gitignore` = `*`) — no stray artifacts.
- Remaining v0.6.0 backlog after #42 is truly fixed: #43 Go ISCC-IDv1, #49 aarch64 wheels,
    dependency refresh, plus the two release-infra items (npm OIDC, single-registry re-trigger). All
    `normal` `[human]`, one per iteration.
