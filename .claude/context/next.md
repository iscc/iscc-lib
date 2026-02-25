# Next Work Package

## Step: Add Java JNI cross-platform build to release workflow

## Goal

Add `build-jni` and `assemble-jar` jobs to `release.yml` so the release pipeline builds JNI native
libraries for all 5 target platforms and bundles them into a JAR under `META-INF/native/`. This is
the prerequisite for Maven Central publishing and fulfills the Java target requirement "Native
libraries load correctly on Linux, macOS, and Windows."

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**:
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java` — path convention:
        `META-INF/native/{os}-{arch}/{libname}` where os ∈ {linux, macos, windows}, arch ∈ {x86_64,
        aarch64}, libname ∈ {libiscc_jni.so, libiscc_jni.dylib, iscc_jni.dll}
    - `crates/iscc-jni/Cargo.toml` — cdylib crate-type, `publish = false`
    - `crates/iscc-jni/java/pom.xml` — Maven build config (no changes needed)
    - Existing `build-napi` matrix in `release.yml` — pattern to follow for cross-compilation

## Not In Scope

- Maven Central publishing job (requires GPG signing + Sonatype credentials — human-gated)
- Adding `maven` idempotency check (no registry to check against yet)
- Modifying `pom.xml` — Maven includes `src/main/resources/` by default
- Running or testing the workflow on actual CI (requires a release trigger)
- `version_sync.py` changes for Maven SNAPSHOT handling

## Implementation Notes

**Structure:** Add two new jobs and one new input to the existing workflow:

1. **`workflow_dispatch.inputs.maven`** — boolean checkbox, default false, description "Build Java
    JAR with bundled native libraries"

2. **`build-jni` job** — matrix build across 5 platforms. Follow the `build-napi` pattern exactly:

    - Condition: `if: startsWith(github.ref, 'refs/tags/v') || inputs.maven`
    - Matrix (match NativeLoader naming convention):

    | os               | target                      | native-dir       | lib-name            |
    | ---------------- | --------------------------- | ---------------- | ------------------- |
    | `ubuntu-latest`  | `x86_64-unknown-linux-gnu`  | `linux-x86_64`   | `libiscc_jni.so`    |
    | `ubuntu-latest`  | `aarch64-unknown-linux-gnu` | `linux-aarch64`  | `libiscc_jni.so`    |
    | `macos-14`       | `aarch64-apple-darwin`      | `macos-aarch64`  | `libiscc_jni.dylib` |
    | `macos-13`       | `x86_64-apple-darwin`       | `macos-x86_64`   | `libiscc_jni.dylib` |
    | `windows-latest` | `x86_64-pc-windows-msvc`    | `windows-x86_64` | `iscc_jni.dll`      |

    - Steps: checkout, rust-toolchain with target, rust-cache, cross-compiler for aarch64-linux (same
        `gcc-aarch64-linux-gnu` pattern as build-napi),
        `cargo build -p iscc-jni --release   --target ${{ matrix.target }}`
    - Upload the built library as artifact named `jni-${{ matrix.native-dir }}` (using the
        NativeLoader directory name). The artifact path depends on the target:
        `target/${{ matrix.target }}/release/${{ matrix.lib-name }}`
    - Include `native-dir` and `lib-name` as matrix variables so artifact naming is clean

3. **`assemble-jar` job** — depends on `build-jni`, runs on `ubuntu-latest`:

    - Condition: same as build-jni
    - Checkout
    - Setup Java (temurin 17)
    - Download all `jni-*` artifacts into a staging directory
    - For each platform, copy the native library to
        `crates/iscc-jni/java/src/main/resources/META-INF/native/{native-dir}/{libname}`
    - Run `mvn package -DskipTests -f crates/iscc-jni/java/pom.xml`
    - Upload the resulting JAR as artifact `iscc-lib-jar`

**Key details:**

- The `cargo build --release` output path is `target/{target}/release/{libname}` — NOT
    `target/release/`
- On Windows the library is `iscc_jni.dll` (no `lib` prefix)
- On Linux/macOS the library is `libiscc_jni.so` / `libiscc_jni.dylib`
- Cross-compilation for aarch64-linux requires `gcc-aarch64-linux-gnu` and the
    `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER` env var (same as build-napi)
- Maven `src/main/resources/` is included in the JAR automatically — no pom.xml changes needed
- Use `actions/upload-artifact@v4` and `actions/download-artifact@v4` consistently (v4, matching the
    rest of the workflow)
- The assemble-jar download step should use `pattern: jni-*` and `merge-multiple: false` (each
    artifact downloads into its own subdirectory named after the artifact) so the copy script can
    iterate over them. Alternatively, use separate download steps per platform, or download with
    `merge-multiple: true` if the lib-name collision between platforms is avoided by the directory
    structure in the artifact

## Verification

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -c 'build-jni' .github/workflows/release.yml` returns >= 1
- `grep -c 'assemble-jar' .github/workflows/release.yml` returns >= 1
- `grep 'inputs.maven' .github/workflows/release.yml` finds the trigger condition
- `grep 'META-INF/native' .github/workflows/release.yml` finds the resource path
- `grep -c 'jni-' .github/workflows/release.yml` returns >= 2 (artifact upload and download)
- The build-jni matrix has exactly 5 entries (one per platform)
- `mise run check` passes (pre-commit hooks clean)

## Done When

All verification criteria pass — the release workflow contains valid build-jni and assemble-jar jobs
with a 5-platform matrix matching the NativeLoader path conventions.
