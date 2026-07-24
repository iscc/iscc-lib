# Handoff

## 2026-07-24 — Activate BLAKE3's wasm32 SIMD backend in @iscc/wasm (finish issue #42)

**Done:** Added the missing piece from the iter-117 NEEDS_WORK review: a direct
`blake3 = { workspace = true, features = ["wasm32_simd"] }` dependency on `crates/iscc-wasm`, which
feature-unifies onto iscc-lib's blake3 for the wasm-only build so blake3's build.rs emits the
`blake3_wasm32_simd` cfg and `Platform::detect()` returns `WASM32_SIMD` instead of `Portable`.
Corrected the now-inaccurate prose in `crates/iscc-wasm/CLAUDE.md` and
`.claude/context/specs/wasm-bindings.md` (both previously claimed the `simd128` target-feature alone
selects the backend).

**Files changed:**

- `crates/iscc-wasm/Cargo.toml`: added direct `blake3` workspace dep with `wasm32_simd` feature,
    with a comment explaining it is feature-unification only and must not be removed as "unused"
- `Cargo.lock`: mechanical — `blake3` added to the `iscc-wasm` package dependency list (no version
    changes)
- `crates/iscc-wasm/CLAUDE.md`: build section now documents the two required pieces (Cargo feature +
    `simd128` target-feature) and why native builds are unaffected
- `.claude/context/specs/wasm-bindings.md`: "WASM SIMD" prose corrected — backend requires the
    `blake3/wasm32_simd` Cargo feature in addition to `target_feature = "simd128"`; "Pure build-flag
    change" reworded. The four `**Verified when:**` checkboxes left UNCHECKED per next.md (review
    agent owns box-checking)

**Verification:**

- Feature wired (deterministic before/after): before this change,
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -f "{p} {f}" -i blake3` showed
    `blake3 v1.8.3 default,std`; after, it shows `blake3 v1.8.3 default,std,wasm32_simd`
    (`grep -q wasm32_simd` exits 0) — PASS
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    — PASS (9 conformance + 78 unit, 0 failed)
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release crates/iscc-wasm --features conformance`
    — PASS ("Your wasm pkg is ready to publish"; wasm-opt accepted the SIMD output)
- Native build unaffected: `cargo test -p iscc-lib` exit 0 (269 + 28 + 22 + 1 tests, 0 failed)
- Landed flags intact: `grep -q 'target-feature=+simd128'` passes on both `.github/workflows/ci.yml`
    and `.github/workflows/release.yml`; `grep -q 'enable-simd' crates/iscc-wasm/Cargo.toml` passes
- `mise run format` clean (no re-stage needed); `mise run check` exit 0, all 15 hooks Passed

**Next:** Review agent verifies and (per next.md) can check the four `specs/wasm-bindings.md`
"Verified when" boxes and delete issue #42. Then the remaining v0.6.0 backlog: #43 Go ISCC-IDv1, #49
aarch64 wheels, dependency refresh, and the two release-infra items (npm OIDC, single-registry
re-trigger).

**Notes:**

- No source code changed — `iscc-wasm/src/lib.rs` untouched (the dep is feature-unification only, no
    `use blake3`, as next.md required). No API surface or hot-path change on native targets:
    blake3's build.rs skips the cfg on non-wasm targets, so the unified feature is inert there — no
    native bench run needed.
- The optional before/after `SumHasher` throughput measurement was NOT captured — next.md marked it
    optional/noisy and not a gating criterion; the deterministic `cargo tree` feature-graph check is
    the honest wired-in proof. If the review agent wants throughput evidence for the spec box #4
    sign-off, it needs a temporary wasm-side timing harness (not present in the test suite).
- `.claude/context/iterations.jsonl` is locally modified by the CID runner — intentionally left
    unstaged.
