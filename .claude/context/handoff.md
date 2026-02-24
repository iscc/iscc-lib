## 2026-02-24 — Create `iscc-jni` JNI bridge crate scaffold

**Done:** Bootstrapped the `iscc-jni` crate with workspace integration, JNI dependency, and a
working `conformanceSelftest` JNI bridge function. The error handling pattern (`throw_and_default`)
is documented in the module docstring with a ready-to-use code template for future gen function
bindings.

**Files changed:**

- `crates/iscc-jni/Cargo.toml`: New crate — cdylib, publish=false, depends on iscc-lib + jni
    (workspace)
- `crates/iscc-jni/src/lib.rs`: JNI bridge with `conformanceSelftest` extern "system" function,
    module docstring documenting naming convention, error handling pattern, and binding recipe
- `Cargo.toml`: Added `"crates/iscc-jni"` to workspace members, added `jni = "0.21"` to workspace
    dependencies

**Verification:**

- `cargo check -p iscc-jni` — exits 0 (clean compile)
- `cargo clippy -p iscc-jni -- -D warnings` — exits 0 (no warnings)
- `cargo clippy --workspace --all-targets -- -D warnings` — exits 0 (workspace-wide clean)
- `cargo test -p iscc-lib` — 206 unit + 31 integration + 22 text util tests pass (no regressions)
- `mise run check` — all 14 pre-commit hooks pass
- Cargo.toml: `crate-type = ["cdylib"]`, `publish = false` confirmed
- lib.rs: `extern "system"` function with `#[unsafe(no_mangle)]` confirmed
- Root Cargo.toml: workspace members includes `"crates/iscc-jni"`, workspace deps includes `jni`

**Next:** Implement the remaining 22 Tier 1 JNI bindings (gen functions, text utilities, codec,
streaming hashers, algorithm primitives). The `throw_and_default` helper documented in the module
docstring should be added as a real function when the first gen function binding is implemented — it
was deferred because Rust's dead_code lint rejects unused functions and no JVM is available in the
devcontainer for testing.

**Notes:** The `throw_and_default` error handling helper is documented as a code template in the
module docstring rather than implemented as a function. This was a deliberate choice: (1) clippy
with `-D warnings` rejects unused functions, (2) we cannot reference it from tests without a JVM,
and (3) adding `#[allow(dead_code)]` is prohibited by project rules. The pattern is fully documented
and ready to be instantiated when the first gen function binding is added. The `jni` crate (v0.21.1)
compiled as pure Rust without needing a JDK — only runtime JVM loading requires JDK installation.
Rust 2024 edition requires `#[unsafe(no_mangle)]` (not `#[no_mangle]`), which was applied correctly.
