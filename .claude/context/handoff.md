## 2026-03-07 — Add C++ CI job to ci.yml

**Done:** Added a `cpp` CI job to `.github/workflows/ci.yml` that builds and tests the C++
header-only wrapper with AddressSanitizer enabled. The job follows the standard preamble pattern
(checkout + rust-toolchain + rust-cache), installs cmake, builds the FFI shared library, configures
CMake with ASAN, builds, and runs the C++ test binary. Placed after `ruby` and before `bench`.

**Files changed:**

- `.github/workflows/ci.yml`: Added `cpp` job (C++ cmake, ASAN, test) — 22 lines between `ruby` and
    `bench` jobs

**Verification:**

- `mise run check` — all 15 pre-commit hooks pass (YAML valid, formatting clean)
- Local simulation: `cargo build -p iscc-ffi` + cmake configure/build/run with ASAN — all 52 C++
    tests pass, zero ASAN errors
- `grep -c 'cpp:' .github/workflows/ci.yml` returns 1 (job key exists)
- `grep 'SANITIZE_ADDRESS=ON' .github/workflows/ci.yml` finds the ASAN flag

**Next:** Push to trigger CI and verify the `cpp` job passes on GitHub Actions. Then proceed to
release bundling — add `iscc.hpp` to FFI release tarballs in `release.yml`. After that, package
manager manifests (vcpkg.json, conanfile.py) and documentation (README.md, howto/c-cpp.md).

**Notes:** The YAML template from next.md was used verbatim — it matched the existing job patterns
perfectly. g++ is pre-installed on `ubuntu-latest` so only cmake needs explicit installation. The CI
job count is now 13 (was 12): version-check, rust, python-test, python, nodejs, wasm, c-ffi, dotnet,
java, go, ruby, cpp, bench.
