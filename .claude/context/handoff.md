## 2026-02-24 — Review of: Add Java CI job

**Verdict:** PASS

**Summary:** Clean, minimal addition of a `java` CI job to `.github/workflows/ci.yml`. The 15-line
job follows the exact same pattern as the existing 5 CI jobs (checkout, Rust toolchain, Rust cache)
plus `actions/setup-java@v4` with Temurin JDK 17. All verification criteria pass — build, tests,
YAML validation, clippy, and pre-commit hooks.

**Verification:**

- [x] `cargo build -p iscc-jni` exits 0 — native library built successfully
- [x] `mvn test -f crates/iscc-jni/java/pom.xml` passes all 46 conformance tests (0 failures, BUILD
    SUCCESS)
- [x] The new `java` job appears in `.github/workflows/ci.yml` with correct structure (6 steps)
- [x] YAML is valid (`uv run python -c "import yaml; yaml.safe_load(...)"` exits 0)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` remains clean
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Next:** Address performance issues from issues.md (DataHasher buffer allocation [normal], codec
header parsing [normal]), the native library loader class for JAR distribution, per-crate READMEs,
or Go bindings.

**Notes:** All 6 binding targets now have CI jobs (Rust, Python, Node.js, WASM, C FFI, Java). The
Java CI job is the last binding target to get CI coverage. Go bindings are not yet started so no Go
CI job is needed yet.
