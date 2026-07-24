# Next Work Package

## Step: Enable WASM simd128 in the @iscc/wasm release + CI builds (issue #42)

## Goal

Ship `@iscc/wasm` compiled with WASM SIMD so `blake3` uses its `wasm32` `simd128` backend instead of
the portable scalar fallback, and exercise the same SIMD configuration in CI. Pure build-flag change
— conformance output is byte-identical; the Instance-Code / `datahash` leg of `gen_sum_code_v0` /
`SumHasher` gains the most.

## Scope

- **Modify**:
    - `.github/workflows/release.yml` — the `build-wasm` job's `Build WASM package` step (line ~411):
        add `env: RUSTFLAGS: "-C target-feature=+simd128"` so the published artifact is SIMD-built.
    - `.github/workflows/ci.yml` — the `wasm` job's `Run tests` step (line ~108): add the same
        `RUSTFLAGS` env so CI compiles/tests the SIMD configuration that ships.
    - `crates/iscc-wasm/Cargo.toml` — add `--enable-simd` to the
        `[package.metadata.wasm-pack.profile.release]` `wasm-opt` array (line 26) so `wasm-opt`
        accepts (and does not strip) the SIMD instructions during the release build.
    - `crates/iscc-wasm/CLAUDE.md` (doc, does not count toward the 3-file limit) — line ~90 quotes the
        exact `wasm-opt = [...]` array; update it to include `--enable-simd` so the doc stays in sync
        with `Cargo.toml`.
- **Reference**:
    - `.claude/context/specs/wasm-bindings.md` → "WASM SIMD (`simd128`)" (the spec + its four
        "Verified when" boxes — leave the boxes unchecked; the review agent checks them).
    - `.github/workflows/release.yml` lines 399–455 (`build-wasm` + `test-wasm` jobs) for the exact
        step shape. Note `test-wasm` smoke-tests the *downloaded* artifact and does NOT rebuild — do
        not add RUSTFLAGS there.

## Not In Scope

- **Do not touch `crates/iscc-wasm/src/lib.rs`** — no source change is needed; SIMD is selected at
    compile time via `target_feature = "simd128"`, output is byte-identical.
- Do not add a second scalar-fallback build artifact or any runtime SIMD feature detection — a
    single SIMD build is intended (simd128 is baseline in all supported browsers/Node).
- Do not add RUSTFLAGS to the `test-wasm` release job (it runs the already-built artifact, no
    rebuild).
- Do not check off the "Verified when" boxes in `specs/wasm-bindings.md` — the review agent owns
    spec check-offs after verifying.
- Do not pick up the other release-workflow issues (npm OIDC migration, single-registry re-trigger)
    or the other v0.6.0 packages (#43 Go ISCC-IDv1, #49 aarch64 wheels, dependency refresh) — one
    per iteration.
- Do not bump the workspace version or touch `version_sync.py`.

## Implementation Notes

- **RUSTFLAGS mechanism (not a cargo feature):** blake3 selects its wasm SIMD implementation via
    `#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]`. Setting
    `RUSTFLAGS="-C target-feature=+simd128"` is the correct and only trigger — no `Cargo.toml`
    dependency/feature edit for blake3 is needed.

- Add the env at **step level** (both jobs build only wasm, so scoping it to the step is clean):

    ```yaml
      - name: Build WASM package
        env:
          RUSTFLAGS: -C target-feature=+simd128
        run: wasm-pack build --target web --release crates/iscc-wasm --features
          conformance
    ```

    Mirror the same `env:` on the ci.yml `Run tests` step (the `wasm-pack test --node ...` line).

- **`--enable-simd` placement:** insert right after `-O3`, matching the spec example →
    `["-O3", "--enable-simd", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]`.
    `wasm-opt` refuses to parse SIMD (`v128`) instructions unless `--enable-simd` is passed, so the
    release build would FAIL without this flag once the input contains SIMD — that failure/success
    is itself the signal that the flag is wired correctly.

- The devcontainer has `wasm-pack` and the `wasm32-unknown-unknown` target installed, but NOT
    `wasm-tools`/`wasm-objdump`. For the disassembly evidence (spec "Verified when" #4), install
    `wasm-tools` via `cargo binstall wasm-tools` (cargo binstall is available; used previously for
    cargo-deny). `wasm-pack build --release` downloads its own `wasm-opt` (binaryen).

## Verification

- `grep -q 'target-feature=+simd128' .github/workflows/release.yml` (exit 0)
- `grep -q 'target-feature=+simd128' .github/workflows/ci.yml` (exit 0)
- `grep -q 'enable-simd' crates/iscc-wasm/Cargo.toml` (exit 0)
- `grep -q 'enable-simd' crates/iscc-wasm/CLAUDE.md` (exit 0 — doc stays in sync)
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    passes all conformance vectors (byte-identical output under a SIMD compile).
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web --release crates/iscc-wasm --features conformance`
    exits 0 (proves `wasm-opt --enable-simd` accepts the SIMD input).
- SIMD opcode evidence: after that release build,
    `wasm-tools print crates/iscc-wasm/pkg/iscc_wasm_bg.wasm | grep -c 'v128'` returns a count > 0
    (install `wasm-tools` via `cargo binstall wasm-tools` if absent) — satisfies spec "Verified
    when" #4 (disassembly showing `v128` opcodes).
- `mise run format` leaves the tree clean (YAML/TOML/mdformat hooks pass).

## Done When

The release and CI wasm builds compile with `-C target-feature=+simd128`, `Cargo.toml`'s wasm-opt
flags include `--enable-simd`, conformance still passes on the SIMD build, and the produced
`iscc_wasm_bg.wasm` disassembly contains `v128` opcodes.
