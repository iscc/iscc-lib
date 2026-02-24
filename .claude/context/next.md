# Next Work Package

## Step: Create `iscc-jni` JNI bridge crate scaffold

## Goal

Bootstrap the JNI bridge crate that will expose all 23 Tier 1 ISCC symbols to Java. This establishes
the crate skeleton, workspace integration, and the JNI function-mapping pattern by implementing the
simplest function (`conformance_selftest`) as a working proof-of-concept.

## Scope

- **Create**: `crates/iscc-jni/Cargo.toml`, `crates/iscc-jni/src/lib.rs`
- **Modify**: root `Cargo.toml` (add `iscc-jni` to workspace members, add `jni` to workspace
    dependencies)
- **Reference**: `crates/iscc-ffi/Cargo.toml` (crate structure pattern),
    `crates/iscc-ffi/src/lib.rs` (error handling pattern), `notes/02-language-bindings.md` (binding
    architecture), `crates/iscc-lib/src/lib.rs` (Tier 1 API surface)

## Not In Scope

- Java wrapper classes (`IsccLib.java`, loader class) — these come in a later step
- Maven/Gradle build configuration — needs Java-side structure first
- Implementing all 23 Tier 1 symbols — only `conformance_selftest` in this step to establish pattern
- Devcontainer Dockerfile changes (JDK/Maven) — separate step
- CI workflow for Java — needs full Java stack first
- README/docs Java content — needs working bindings first
- `notes/02-language-bindings.md` Java section — can be added alongside the Java wrapper step

## Implementation Notes

**Crate structure** follows the hub-and-spoke model from `notes/02`:

```
crates/iscc-jni/
├── Cargo.toml          # cdylib, depends on iscc-lib + jni
└── src/
    └── lib.rs          # JNI bridge functions
```

**Cargo.toml**:

- `crate-type = ["cdylib"]` — produces a shared library loadable by JVM
- `publish = false` — published via Maven Central JAR, not crates.io
- Dependencies: `iscc-lib = { path = "../iscc-lib" }` and `jni = { workspace = true }`
- Use `workspace.package` fields (version, edition, rust-version, authors, license, repository)

**Root Cargo.toml**:

- Add `"crates/iscc-jni"` to workspace members list
- Add `jni = "0.21"` to `[workspace.dependencies]`

**JNI function naming convention**: JNI requires specific mangled names for native methods. For a
Java class `io.iscc.iscc_lib.IsccLib` with native method `conformanceSelftest`, the Rust function
must be named `Java_io_iscc_iscc_1lib_IsccLib_conformanceSelftest` (underscores in package names
become `_1` in JNI). Use `#[no_mangle]` and `extern "system"` calling convention.

**Pattern to establish** (using `conformance_selftest` as the exemplar):

```rust
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jboolean;

#[no_mangle]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_conformanceSelftest(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    let result = iscc_lib::conformance_selftest();
    result as jboolean
}
```

**Error handling pattern**: For functions that return `IsccResult<T>`, catch the error and throw a
Java exception via `env.throw_new("java/lang/IllegalArgumentException", &msg)`, then return a
default value. Document this pattern in a module-level docstring for future implementors. Add a
helper function (e.g., `throw_and_default`) to centralize this — similar to the C FFI crate's
`set_last_error` + `result_to_c_string` helpers.

**Important**: The `jni` crate compiles as pure Rust — no JDK needed at compile time. The advance
agent can verify compilation with `cargo check -p iscc-jni` and `cargo clippy` without having JDK
installed. Runtime testing with Java will come in a later step.

## Verification

- `cargo check -p iscc-jni` exits 0 (crate compiles)
- `cargo clippy -p iscc-jni -- -D warnings` exits 0 (no warnings)
- `cargo clippy --workspace --all-targets -- -D warnings` exits 0 (workspace-wide clean)
- `crates/iscc-jni/Cargo.toml` contains `crate-type = ["cdylib"]` and `publish = false`
- `crates/iscc-jni/src/lib.rs` contains `extern "system"` JNI function for `conformance_selftest`
- Root `Cargo.toml` lists `"crates/iscc-jni"` in workspace members
- Root `Cargo.toml` has `jni` in `[workspace.dependencies]`
- Existing tests unaffected: `cargo test -p iscc-lib` still passes all tests

## Done When

All verification criteria pass — the `iscc-jni` crate compiles cleanly within the workspace, exports
a JNI-compatible `conformance_selftest` function, and introduces no regressions.
