# Next Work Package

## Step: Add Java CI job

## Goal

Add a Java CI job to `.github/workflows/ci.yml` so the JNI bridge and conformance tests are verified
on every push/PR, matching the existing pattern for all other binding targets (Rust, Python,
Node.js, WASM, C FFI).

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml`
- **Reference**: `crates/iscc-jni/java/pom.xml` (Maven config, `java.library.path` setting),
    `crates/iscc-jni/Cargo.toml` (crate name for `cargo build`), existing CI jobs in `ci.yml` for
    pattern consistency

## Not In Scope

- Native library loader class for JAR distribution (future step)
- Maven Central publishing configuration or release workflow changes
- Go CI job (Go bindings not started)
- Java documentation pages (`docs/howto/java.md`)
- README updates for Java
- Per-crate READMEs
- Modifying the Java test code or JNI bridge code

## Implementation Notes

Add a new `java` job following the same structure as the existing 5 jobs. The job should:

1. Use `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2` — same as
    all other jobs in the workflow.
2. Use `actions/setup-java@v4` with `distribution: 'temurin'` and `java-version: '17'` — do NOT rely
    on apt-get. The setup-java action also provides Maven.
3. Build the JNI native library: `cargo build -p iscc-jni`
4. Run Maven tests with the library path pointing to the correct location. The `pom.xml` at
    `crates/iscc-jni/java/pom.xml` has Surefire configured with
    `<argLine>-Djava.library.path=${project.basedir}/../../../target/debug</argLine>` — this
    resolves relative to the `pom.xml` directory and should work both locally and in CI. Run:
    `mvn test -f crates/iscc-jni/java/pom.xml`
5. Job name: `Java (JNI build, mvn test)` to match the naming convention of other jobs.

**Important per learnings:** Do NOT use `mise` in GitHub Actions — call `cargo` and `mvn` directly.

**Library path concern:** The `${project.basedir}` Maven property resolves to the directory
containing `pom.xml` (i.e., `crates/iscc-jni/java/`). The path
`${project.basedir}/../../../target/debug` expands to the repo root's `target/debug/` where
`cargo build -p iscc-jni` places `libiscc_jni.so`. This should work without modification. If the
advance agent is unsure, they can add a diagnostic step `ls -la target/debug/libiscc_jni*` after the
cargo build step.

## Verification

- `cargo build -p iscc-jni` exits 0 locally
- `mvn test -f crates/iscc-jni/java/pom.xml` passes all 46 conformance tests locally
- The new `java` job appears in `.github/workflows/ci.yml` with correct structure
- YAML is valid: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`
- `cargo clippy --workspace --all-targets -- -D warnings` remains clean

## Done When

The `java` job is added to `ci.yml` with checkout, Rust toolchain, Rust cache, Java setup, cargo
build, and mvn test steps — and all local verification criteria pass.
