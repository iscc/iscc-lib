## 2026-03-07 — Review of: Add C++ CI job to ci.yml

**Verdict:** PASS

**Summary:** The advance agent added a 22-line `cpp` CI job to `ci.yml` that matches the YAML
template from next.md verbatim. The job follows the established preamble pattern (checkout +
rust-toolchain + rust-cache), installs cmake, builds FFI, configures CMake with ASAN, builds, and
runs C++ tests. All verification criteria pass. Scope discipline is perfect — only `ci.yml` was
modified.

**Verification:**

- [x] `mise run check` passes — all 15 pre-commit hooks pass (YAML valid, formatting clean)
- [x] Local simulation succeeds — `cargo build -p iscc-ffi` + cmake configure/build/run with ASAN:
    all 52 C++ tests pass, zero ASAN errors
- [x] `grep -c 'cpp:' .github/workflows/ci.yml` returns 1 — job key exists
- [x] `grep 'SANITIZE_ADDRESS=ON' .github/workflows/ci.yml` finds the ASAN flag
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [ ] CI push triggers `cpp` job and it passes — deferred to push step (cannot verify pre-push)

**Issues found:**

- (none)

**Codex review:** No issues found. Codex confirmed the workflow matches the project layout and
reproduces the intended CMake build/test flow.

**Next:** Continue with the C++ issue — next logical step is release bundling: add `iscc.hpp` to FFI
release tarballs in `release.yml`. After that: package manager manifests (`vcpkg.json`,
`conanfile.py`), `packages/cpp/README.md`, and `docs/howto/c-cpp.md` documentation update. The C++
issue in issues.md remains open until all 4 sub-items are done.

**Notes:** CI job count is now 13 (was 12). The `cpp` job is placed between `ruby` and `bench` for
logical grouping. The YAML was used verbatim from the define-next template — no deviation. The
deferred CI verification criterion will be confirmed when we push.
