## 2026-11-22 — Review of: Conan recipe verifies the SHA256 of the FFI archive it downloads

**Verdict:** PASS

**Summary:** The Conan recipe now passes a pinned SHA256 from `packages/cpp/conandata.yml` to
`download()` and raises when a target has no hash. `scripts/update_vcpkg_hashes.py` writes those
hashes in the same archive pass as the vcpkg SHA512s, and `prepare-release` stages the file in the
release commit. Every criterion reproduced, the diff stays inside the scope (3 code files and one
generated data file), and the `[audit]` issue is closed.

**Verification:**

- [x] `uv run pytest tests/test_update_vcpkg_hashes.py tests/test_ffi_release_assets.py` passes — 59
    passed together with `tests/test_check_release_workflow.py`
- [x] `python3 scripts/update_vcpkg_hashes.py <v0.7.0 archives> --check` exits 0 and
    `git status --porcelain packages/cpp/portfile.cmake` prints nothing — freshly downloaded
    archives; the five SHA256s also equal `sha256sum` and GitHub's asset `digest` fields
- [x] `conan create` succeeds with the committed hashes and fails with one altered digit — Conan
    2.34.0 on `git archive` copies: exit 0 with "Sources correctly downloaded"; with the last digit
    changed, exit 1 with "sha256 hash failed … Provided hash: …c40c1 Computed hash: …c40c0"
- [x] `git grep -n "sha256=" packages/cpp/conanfile.py` — line 97, the `download(` call
- [x] `uv run scripts/check_release_workflow.py .github/workflows/release.yml` and
    `mise run version:check` exit 0
- [x] `mise run check` exits 0 and leaves `packages/cpp/conandata.yml` unchanged (md5 `cf57c25f…`
    before and after); `ty check` and ruff `S`/`C901` are clean

Probes beyond the list: the write mode on the real archives changes neither file ("updated 0 of 5
portfile hashes and 0 Conan hashes"). The script fails on Python 3.9 (`itertools.pairwise`, a line
from iteration 285), which does not matter: `python3` on the `macos-15` runner image is 3.15.2.

**Issues found:**

- Three doc lines still said the release commit carries only the vcpkg hashes (`specs/ci-cd.md`,
    `specs/swift-bindings.md`, `docs/development.md`). Fixed in this review: they now say "the vcpkg
    and Conan hashes".
- Not filed: no test and no CI job executes the recipe. The tests read it with `ast`, so they cannot
    see `_archive_sha256` stop raising. A missing target is still caught: the tests tie the
    `conandata.yml` keys and the `_target_triple` triples to the `build-ffi` matrix. A CI
    `conan create` would fail between a version bump and the release, when the archive does not
    exist yet.
- Not filed (nits): `render_conandata` sorts whole lines, not names (same order for today's
    triples); `validate()` does not check for a pinned hash, `build()` does.

**Codex review:** no actionable findings. It re-ran the 59 tests, compared the committed hashes with
the release archives, and confirmed that the recipe packages with valid hashes and rejects a wrong
or missing hash.

**Next:** two `normal` `[audit]` issues still block v0.8.0. Take the Rust conformance runner issue
first: one section-to-runner table in `conformance.rs`, the duplicate loops removed from `lib.rs`,
and shared strict readers for the WASM tests (2 non-test files plus 3 WASM test files). Then the
spec and API-doc drift issue, which needs an audit-scope step or a split into specs and docs.

**Notes:**

- New decision: "Conan SHA256s live in `conandata.yml`, keyed by target triple, written by the vcpkg
    hash script". Adding it rotated the two oldest decisions into `decisions-archive.md`.
- `conandata.yml` holds the v0.7.0 hashes. They go stale at the next version bump and are rewritten
    by `prepare-release`, exactly as the portfile's SHA512s are. Do not scope that as drift.
- Conan end-to-end check, about one minute, needs network:
    `CONAN_HOME=/tmp/x uv run --no-project --with conan conan create <git archive copy of packages/cpp>`
    after `conan profile detect`.
