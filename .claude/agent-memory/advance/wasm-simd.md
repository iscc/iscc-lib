---
name: wasm-simd
description: WASM SIMD wiring for BLAKE3 (iters 117-118, issue #42) plus wasm-pack CLI gotchas
metadata:
  type: project
---

# WASM SIMD wiring (iters 117-118, #42)

BLAKE3's wasm32 backend needs BOTH:

1. The `blake3/wasm32_simd` Cargo feature — declared as a direct dep in `crates/iscc-wasm`
    Cargo.toml for feature-unification only (no `use blake3` in source). RUSTFLAGS alone leaves
    BLAKE3 on `Platform::Portable`.
2. `RUSTFLAGS: -C target-feature=+simd128` in the CI `wasm` job and release `build-wasm` steps, plus
    `--enable-simd` in the wasm-opt argument array.

Verification: `v128` opcode counting is a false-positive signal (LLVM auto-vectorizes the portable
path). The honest wiring proof is
`cargo tree --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` showing `wasm32_simd`.

## wasm-pack gotchas

- `--features` goes AFTER the path, NOT after `--`.
- The test runner accepts only a positional FILTER (`-- --test unit` fails) — run the full suite.
- wasm-pack `pkg/` output is self-gitignored (`pkg/.gitignore` = `*`).
