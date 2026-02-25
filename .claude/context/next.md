# Next Work Package

## Step: Fix WASM release build (wasm-opt bulk-memory)

## Goal

Fix the WASM release build failure in the release workflow caused by `wasm-opt` rejecting
`memory.copy` instructions without bulk-memory support enabled. This unblocks npm publishing for
both `@iscc/wasm` and `@iscc/lib` packages.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-wasm/Cargo.toml` (add wasm-pack profile metadata to configure wasm-opt)
- **Reference**: `.github/workflows/release.yml` (line 260 — the failing `wasm-pack build` command),
    `.claude/context/state.md` (error description)

## Not In Scope

- Re-triggering the release workflow or re-publishing v0.0.1 (that's a human-gated operation)
- Configuring OIDC trusted publishing for crates.io (registry-side human task)
- Setting up NPM_TOKEN or npm provenance (separate publishing infrastructure)
- Modifying the release workflow YAML itself — the fix belongs in the crate's Cargo.toml
- Bumping the version to 0.0.2

## Implementation Notes

The release workflow `Build WASM package` job fails because `wasm-opt` (run automatically by
`wasm-pack build --release`) does not enable the WebAssembly bulk-memory proposal by default. Rust's
LLVM backend emits `memory.copy` instructions (from `copy_within`, `clone`, etc.) which require bulk
memory support. The `wasm-opt` post-processing step rejects these instructions.

**Fix**: Add a `[package.metadata.wasm-pack.profile.release]` section to
`crates/iscc-wasm/Cargo.toml` that configures wasm-opt to accept bulk-memory instructions:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-O", "--enable-bulk-memory"]
```

This is the standard wasm-pack mechanism for passing flags to wasm-opt during release builds. The
`-O` flag applies standard optimizations (equivalent to the default behavior), and
`--enable-bulk-memory` allows the bulk memory operations that Rust emits.

**Why Cargo.toml instead of release.yml**: The wasm-pack metadata section is the documented,
portable way to configure wasm-opt. It works for any `wasm-pack build --release` invocation (local
or CI), not just the release workflow. The `release.yml` command line doesn't need to change.

**Why not `-all`**: While `-all` enables all WASM features, `--enable-bulk-memory` is more targeted
and documents exactly which feature is needed. If other features are needed later, they can be added
explicitly.

**CI impact**: The regular CI WASM test job (`wasm-pack test --node`) does not use `--release`, so
wasm-opt is not involved and these tests are unaffected by this change.

## Verification

- `grep -q 'enable-bulk-memory' crates/iscc-wasm/Cargo.toml` exits 0 (config present)
- `grep -q 'wasm-pack.profile.release' crates/iscc-wasm/Cargo.toml` exits 0 (correct section)
- `cargo check -p iscc-wasm` passes (Cargo.toml is valid)
- `wasm-pack build --target web --release crates/iscc-wasm` succeeds (if wasm-pack is available;
    install via `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` if needed)
- `mise run check` passes (all pre-commit/pre-push hooks clean)

## Done When

The `crates/iscc-wasm/Cargo.toml` contains a `[package.metadata.wasm-pack.profile.release]` section
with `--enable-bulk-memory` flag, `wasm-pack build --release` succeeds locally, and all existing
quality gates pass.
