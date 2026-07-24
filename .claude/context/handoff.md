# Handoff

## 2026-07-24 — Review of: Activate BLAKE3's wasm32 SIMD backend in @iscc/wasm (finish issue #42)

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent added the missing piece from the iter-117 NEEDS_WORK: a direct
`blake3 = { workspace = true, features = ["wasm32_simd"] }` dep on `crates/iscc-wasm`, which
feature-unifies so blake3's build.rs emits the `blake3_wasm32_simd` cfg on the wasm32 build and
`Platform::detect()` returns `WASM32_SIMD`. All six next.md criteria pass; scope was disciplined
(only the four intended files + advance memory/handoff), no source changes, conformance
byte-identical, no gate circumvention. Issue #42 is resolved and its four spec boxes are checked.
One P3 doc imprecision (also caught by Codex) was fixed directly.

**Verification:**

- [x] Feature wired —
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -f "{p} {f}" -i blake3` shows
    `blake3 v1.8.3 default,std,wasm32_simd` (was `default,std`) — PASS
- [x] Conformance preserved on SIMD build —
    `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    exit 0 (78 unit + conformance binary, byte-identical) — PASS
- [x] Release build + wasm-opt —
    `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release crates/iscc-wasm --features conformance`
    exit 0 ("ready to publish"); released `.wasm` has 5370 `v128` opcodes (was 1993 pre-feature) —
    PASS
- [x] Native build unaffected — `cargo test -p iscc-lib` exit 0 (269+28+22+1, 0 failed); host build
    is inert (build.rs skips the cfg off-wasm) — PASS
- [x] Landed flags intact — `grep -q 'target-feature=+simd128'` passes on ci.yml + release.yml;
    `grep -q 'enable-simd' crates/iscc-wasm/Cargo.toml` passes — PASS
- [x] `mise run format` clean; `mise run check` exit 0, all 15 hooks Passed (mdformat clean this
    cycle — no reformat needed) — PASS

**Issues found:**

- (fixed directly) **Doc imprecision** in the freshly-written SIMD prose: both `iscc-wasm/CLAUDE.md`
    and `specs/wasm-bindings.md` claimed the `simd128` target-feature is required "so the backend's
    v128 intrinsics compile". Empirically false — a no-`RUSTFLAGS`
    `cargo build -p iscc-wasm --target   wasm32-unknown-unknown` compiles fine, because blake3's
    SIMD functions carry `#[target_feature(enable = "simd128")]` (verified in
    blake3-1.8.3/src/wasm32_simd.rs). Corrected both docs: the Cargo feature activates the backend;
    the global RUSTFLAGS broadens simd128 across the whole crate (auto-vec of CDC/xxh32/minhash +
    inlining) and remains set in CI/release, and `--enable-simd` is still required for wasm-opt.
    Behavior-neutral; the flags themselves stay.

**Codex review:** One [P3] non-blocking finding, confirmed and actioned: "enabling
`blake3/wasm32_simd` is sufficient [to compile the backend] because BLAKE3's SIMD entry points carry
`#[target_feature(enable = "simd128")]`; a no-flag build succeeds and emits `v128` … the global flag
should not be documented as required to compile the backend." I verified this against the blake3
source and by building without RUSTFLAGS, and fixed the wording in both `CLAUDE.md` and the spec.

**Next:** #42 is done — WASM is complete for v0.6.0. Remaining v0.6.0 backlog (all `normal`
`[human]`, one per iteration): **#43 Go bindings ISCC-IDv1** (recommended next — concrete feature
with a known conformance vector `ISCC:MAIGHFECJMOPMIAB`, unblocks iscc-monitor's ADR-0011), #49
aarch64 Python wheels, dependency review/refresh, and the two release-infra items (npm OIDC
migration, single-registry re-trigger fix). Spec-only backlog: `low` items stay parked.

**Notes:**

- Feature unification detail: the `wasm32_simd` feature does unify onto blake3 for the host
    workspace build too (`cargo tree -p iscc-wasm` on the host target shows it), but it is fully
    inert there — blake3's build.rs only emits the `blake3_wasm32_simd` cfg under `is_wasm32()`, so
    no native code path or output changes. iscc-lib and the other bindings never depend on
    iscc-wasm, so their standalone builds don't even enable the feature.
- No throughput bench was captured (next.md marked it optional/noisy). The cargo-tree feature-graph
    check is the definitive proof the backend is wired in; the 1993→5370 `v128`-opcode jump
    corroborates. Box #4 signed off on the wiring proof + disassembly, not a throughput number.
- Batch pushed to origin/develop: this cycle's push carries the iter-117 NEEDS_WORK batch (which was
    correctly held) + iter-118 define-next/advance/review as one unit. Pre-push hooks ran clean.
- `.claude/context/iterations.jsonl` left unstaged (runner-owned).
