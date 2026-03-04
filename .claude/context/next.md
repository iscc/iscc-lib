# Next Work Package

## Step: Add release smoke tests for all binding pipelines

## Goal

Add smoke-test jobs between build and publish steps in `release.yml` for all 6 binding pipelines
(PyPI, npm-lib, npm-wasm, RubyGems, Maven, FFI). This catches broken artifacts — missing symbols,
cross-compilation failures, packaging errors — before they reach end users. Resolves the sole
remaining `normal`-priority issue.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**:
    - `crates/iscc-ffi/tests/test_iscc.c` (C test program for FFI smoke test)
    - `crates/iscc-napi/package.json` (napi build/test scripts)
    - `crates/iscc-jni/java/pom.xml` (Maven test config, `java.library.path` setup)

## Not In Scope

- Adding smoke tests for crates.io (already runs `cargo test` before publishing)
- Changing existing build or publish job logic (only add new test jobs and update `needs:`)
- Creating new test scripts or test files (use existing test suites and `conformance_selftest()`)
- Full multi-platform smoke tests (test only linux-x86_64 artifacts on ubuntu-latest)
- Modifying the `assemble-jar` job or its dependencies

## Implementation Notes

### Pattern

Each smoke test job follows the same pattern:

1. `needs: [build-*]` — waits for the build job
2. Runs on `ubuntu-latest`
3. Downloads the linux-x86_64 build artifact
4. Sets up the language runtime
5. Installs the artifact and runs a quick verification
6. Gates the corresponding `publish-*` job (update its `needs:` array)

### Per-Pipeline Details

**1. `test-wheels` (PyPI)**

- `needs: [build-wheels]`
- `actions/setup-python@v5` with python 3.10
- Download artifact `wheels-ubuntu-latest-x86_64`
- `pip install dist/*.whl`
- `python -c "from iscc_lib import conformance_selftest; assert conformance_selftest()"`
- Update `publish-pypi.needs` to `[build-wheels, build-sdist, test-wheels]`

**2. `test-napi` (npm @iscc/lib)**

- `needs: [build-napi]`
- `actions/setup-node@v4` with node 22
- Download artifact `napi-x86_64-unknown-linux-gnu` into a working directory
- Directly `require()` the `.node` binary file and call `conformanceSelftest()` — this tests the raw
    native addon without needing `index.js` generation or `npm install`
- The .node file is named `iscc-lib.linux-x64-gnu.node` (derived from napi config `name` + platform
    triple)
- Update `publish-npm-lib.needs` to `[build-napi, test-napi]`

**3. `test-wasm` (npm @iscc/wasm)**

- `needs: [build-wasm]`
- `actions/setup-node@v4` with node 22
- Download artifact `wasm-pkg` into `pkg/`
- Write a small inline ESM smoke test as a `.mjs` file that:
    - Reads `pkg/iscc_wasm_bg.wasm` as bytes via `fs.readFileSync()`
    - Imports `init` and `conformance_selftest` from `pkg/iscc_wasm.js`
    - Calls `await init(wasmBytes)` to initialize (bypasses `fetch()` which doesn't work in Node)
    - Asserts `conformance_selftest()` returns `true`
- The build uses `--target web` which generates ESM — the smoke script must be `.mjs` with top-level
    `await`
- Update `publish-npm-wasm.needs` to `[build-wasm, test-wasm]`

**4. `test-gem` (RubyGems)**

- `needs: [build-gem]`
- `ruby/setup-ruby@v1` with ruby 3.3
- Download artifact `gem-x86_64-linux`
- `gem install gems/*.gem` (installs the precompiled platform gem)
- `ruby -e "require 'iscc_lib'; raise 'Conformance failed' unless IsccLib.conformance_selftest"`
- Update `publish-rubygems.needs` to `[build-gem, test-gem]`

**5. `test-jni` (Maven Central)**

- `needs: [build-jni]`
- `actions/checkout@v4` (needs pom.xml and test sources)
- `actions/setup-java@v4` with temurin JDK 17
- Download JNI artifacts (pattern `jni-*`) and copy to
    `crates/iscc-jni/java/src/main/resources/META-INF/native/` (same copy logic as `publish-maven`)
- `mvn test -f crates/iscc-jni/java/pom.xml` — runs the full Java test suite including conformance
    vectors against the built native libs
- Update `publish-maven.needs` to `[build-jni, test-jni]`

**6. `test-ffi` (FFI / GitHub Releases)**

- `needs: [build-ffi]`
- `actions/checkout@v4` (needs `iscc.h` and `tests/test_iscc.c`)
- Download artifact `ffi-x86_64-unknown-linux-gnu`
- Extract the `.tar.gz` archive
- Compile the C test program:
    `cc -o test_iscc crates/iscc-ffi/tests/test_iscc.c   -Icrates/iscc-ffi/include -L<lib-dir> -liscc_ffi -lpthread -ldl -lm`
- Run with `LD_LIBRARY_PATH=<lib-dir> ./test_iscc`
- Use the header from the extracted archive (not from repo) to test the distributed artifact
    integrity
- Update `publish-ffi.needs` to `[build-ffi, test-ffi]`

### Conditional Execution

Each test job must have the same `if:` condition as its corresponding build job so it only runs when
that pipeline is triggered. Copy the `if:` from the build job.

## Verification

- `grep -cP '^\s{2}test-' .github/workflows/release.yml` outputs `6` (six test job definitions)
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -A2 'publish-pypi:' .github/workflows/release.yml | grep -q 'test-wheels'` exits 0
- `grep -A2 'publish-npm-lib:' .github/workflows/release.yml | grep -q 'test-napi'` exits 0
- `grep -A2 'publish-npm-wasm:' .github/workflows/release.yml | grep -q 'test-wasm'` exits 0
- `grep -A2 'publish-rubygems:' .github/workflows/release.yml | grep -q 'test-gem'` exits 0
- `grep -A2 'publish-maven:' .github/workflows/release.yml | grep -q 'test-jni'` exits 0
- `grep -A2 'publish-ffi:' .github/workflows/release.yml | grep -q 'test-ffi'` exits 0

## Done When

All 6 smoke test jobs are defined in `release.yml`, each gates its corresponding publish job via
`needs:`, and all verification criteria pass.
