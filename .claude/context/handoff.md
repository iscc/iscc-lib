## 2026-02-24 — Review of: Create `iscc-jni` JNI bridge crate scaffold

**Verdict:** PASS

**Summary:** Clean, well-scoped scaffold. The `iscc-jni` crate follows the established hub-and-spoke
pattern, compiles cleanly without a JDK, and exports a correct JNI-named `conformanceSelftest`
function. The module docstring provides a thorough recipe (naming convention, error handling,
binding steps) for future implementors. No regressions, no quality gate circumvention.

**Verification:**

- [x] `cargo check -p iscc-jni` exits 0 — clean compile
- [x] `cargo clippy -p iscc-jni -- -D warnings` exits 0 — no warnings
- [x] `cargo clippy --workspace --all-targets -- -D warnings` exits 0 — workspace-wide clean
- [x] `crates/iscc-jni/Cargo.toml` contains `crate-type = ["cdylib"]` and `publish = false`
- [x] `crates/iscc-jni/src/lib.rs` contains `extern "system"` JNI function for
    `conformance_selftest` with correct JNI mangled name
- [x] Root `Cargo.toml` lists `"crates/iscc-jni"` in workspace members
- [x] Root `Cargo.toml` has `jni = "0.21"` in `[workspace.dependencies]`
- [x] Existing tests unaffected: `cargo test -p iscc-lib` — 206 unit + 31 integration + 22 text util
    tests pass
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Next:** The JNI scaffold is in place. Next steps for the Java binding track: implement the
remaining 22 Tier 1 JNI bridge functions (gen functions, text utilities, codec, streaming hashers,
algorithm primitives), add `throw_and_default` as a real helper once the first gen binding needs it.
Java wrapper classes, Maven build, and JDK devcontainer setup are separate later steps.
Alternatively, the define-next agent may prioritize the remaining `[normal]` performance issues
(codec header `Vec<bool>` elimination, `DataHasher` buffer allocation) or `[low]` robustness issues
from issues.md — those offer measurable improvement with benchmark validation.

**Notes:** The `jni` v0.21 crate pulls in `thiserror` v1 as a transitive dependency alongside the
workspace's `thiserror` v2 — this is unavoidable until the `jni` crate updates. The
`#[unsafe(no_mangle)]` attribute (Rust 2024 edition requirement) was correctly used instead of the
deprecated `#[no_mangle]`. The `throw_and_default` error handling helper is documented as a code
template rather than implemented — a pragmatic choice given dead_code lint constraints and no JVM
runtime in the devcontainer.
