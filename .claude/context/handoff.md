# Handoff

## 2026-07-24 — Review of: Restore linux/aarch64 Python wheels in release.yml (#49)

**Verdict:** PASS

**Summary:** The advance agent added a native-ARM `manylinux*_aarch64` wheel target to the Python
release build in `.github/workflows/release.yml` — a fourth `build-wheels` matrix entry
(`ubuntu-24.04-arm`/`aarch64`/`python3.10`) plus a matrixified `test-wheels` job that import-tests
both the x86_64 and aarch64 wheels before publish. Single-file source change, exactly as next.md and
the plan specified; matches the pre-written `ci-cd.md` "Build Matrices" spec. Resolves issue #49.

**Verification:**

- [x] `uv run python -c "import yaml; yaml.safe_load(...)"` — exit 0 (release.yml still valid YAML)
- [x] `grep -q 'ubuntu-24.04-arm'` — 3 occurrences (build matrix + test matrix os + test artifact)
- [x] `target: aarch64` + `ubuntu-24.04-arm` both present in build-wheels matrix — MATRIX_OK
- [x] `test-wheels` resolves `wheels-ubuntu-24.04-arm-aarch64` — literal artifact string present,
    matches `build-wheels` upload name `wheels-${{ matrix.os }}-${{ matrix.target }}`
- [x] three original build-wheels entries verbatim — `ubuntu-latest`/`x86_64`,
    `macos-14`/`universal2-apple-darwin`, `windows-latest`/`x64` (lines 174-182)
- [x] `mise run check` — all 15 pre-commit hooks Passed (yamlfix/mdformat accepted; no file dirtied)
- [x] End-to-end chain consistent — `publish-pypi` "Download all artifacts" uses `pattern: wheels-*`
    - `merge-multiple: true`, so the new aarch64 artifact is auto-collected and published; its
        `needs: [build-wheels, build-sdist, test-wheels]` is untouched (matrixed job keeps one job id)

**Issues found:** (none) — scope is exactly `.github/workflows/release.yml` + handoff + advance
memory (1 non-test/non-doc file, well within budget). Gate-circumvention scan over all 4 unpushed
commits is clean: no suppressions, no threshold/hook weakening, no scope-exclusion. The change is
purely additive.

**Codex review:** Clean. "The new native ARM wheel build and matching smoke-test matrix entry are
wired consistently. Artifact naming, runner architecture, maturin target selection, and publish
dependencies remain correct." No actionable findings.

**Next:** One CID-doable v0.6.0 issue remains: **project-wide dependency review/refresh** (root
`Cargo.toml`/lock, `pyproject.toml`/`uv.lock`, binding manifests: napi `package.json`, rb
`Gemfile`/gemspec, jni `pom.xml`, kotlin `build.gradle.kts`, dotnet `.csproj`, go `go.mod`; tooling
pins: `mise.toml`, `.pre-commit-config.yaml`, GHA versions). Patch/minor by default, majors
evaluated individually. Mind the documented pinning constraints (`ci-cd.md` "Dependency Freshness"):
PyO3 bumps only together with re-verifying `gil_used`/`py.detach` (#41); rb_sys must match the
`oxidize-rb/actions/cross-gem` Docker tag; wheels stay `abi3-py310`; quality-gate tool pins (e.g.
`cargo-crap`) bump together with their committed baselines. This one likely exceeds the 3-file scope
cap — it is a spread of manifests, so define-next should scope it as several smaller per-ecosystem
steps (Rust deps; Python deps; each binding manifest group; tooling pins) rather than one mega-diff,
since it does NOT cite an `[audit]` issue (the 8-file escape valve does not apply).

**Notes:**

- Issue #49 deleted from issues.md (resolved). The `ci-cd.md` spec already carried the
    `ubuntu-24.04-arm`/`aarch64` "Build Matrices" row and description (pre-written as the target
    when the human filed the issue), so no spec edit was needed — the implementation now matches it.
- **Runtime confirmation is release-time only:** the aarch64 build/test path executes only under a
    `workflow_dispatch` release with `pypi` selected — it is NOT exercised by CID pushes, so the
    verification bar is static (per next.md). The first real aarch64 wheel builds/tests at the next
    v0.6.0 release. If `actions/setup-python@v5` or the manylinux aarch64 container misbehaves on
    ARM, it will surface then; nothing more CID can do without a release.
- **Pre-existing (not this diff):** `test-wheels` still lacks the `!cancelled() && !failure()`
    guard, i.e. the aarch64 row inherits the tracked single-registry re-trigger bug (open `normal`
    `[human]` issue "Fix broken single-registry re-trigger"). Out of scope here; flagged for that
    issue's step.
- CI is green on develop tip; this push adds only the release.yml change (won't run until a
    release).
- `.claude/context/iterations.jsonl` is runner-managed; left unstaged per protocol.
