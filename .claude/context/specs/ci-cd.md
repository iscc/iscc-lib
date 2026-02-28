# Spec: CI/CD, Dev Tooling, and Release Publishing

Continuous integration, quality gates, developer tooling, and selective package publishing.

## Workflow Files

| Workflow    | File          | Trigger                                | Purpose                       |
| ----------- | ------------- | -------------------------------------- | ----------------------------- |
| **CI**      | `ci.yml`      | push to `main`/`develop`, PR to `main` | Quality gates for all targets |
| **Release** | `release.yml` | `v*.*.*` tag or `workflow_dispatch`    | Build and publish packages    |
| **Docs**    | `docs.yml`    | push to `main`                         | Build and deploy docs site    |

## CI Workflow — Quality Gates

Runs on every push to `main` or `develop` and every PR targeting `main`. All jobs must pass before
merge.

| Job         | What it checks                                                                        |
| ----------- | ------------------------------------------------------------------------------------- |
| **Rust**    | `cargo fmt --check`, `cargo clippy --workspace -D warnings`, `cargo test --workspace` |
| **Python**  | `ruff check`, `ruff format --check`, `pytest`                                         |
| **Node.js** | napi build, `npm test`                                                                |
| **WASM**    | `wasm-pack test --node`                                                               |
| **C FFI**   | cbindgen header generation, gcc compile, C test run                                   |
| **Java**    | JNI `cargo build`, `mvn test` (49 tests including conformance vectors)                |
| **Go**      | `CGO_ENABLED=0 go test`, `go vet` (pure Go, no Rust toolchain)                        |
| **Version** | `python scripts/version_sync.py --check` for manifest version consistency             |
| **Bench**   | `cargo bench --no-run` compile-only benchmark verification                            |

CI does NOT use `mise` — it calls `cargo`, `uv`, and tools directly. Standard action set:
`dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`, `astral-sh/setup-uv@v4`,
`actions/setup-python@v5`, `actions/setup-node@v4`.

## Release Workflow — Selective Publishing

Single workflow supporting both coordinated full releases and selective per-registry publishing.

### Design Principles

- **One workflow file** — all release logic lives in `release.yml`
- **`workflow_dispatch` is the primary release mechanism** — manual trigger from the GitHub Actions
    UI with checkboxes to select which registries to publish to
- **Tag trigger as convenience** — pushing a `v*.*.*` tag publishes all registries (equivalent to
    checking all boxes)
- **Shared version** — all packages share the workspace version from root `Cargo.toml`; version is
    bumped once in a single commit before releasing
- **Independent jobs** — each registry's build+publish pipeline is a self-contained job chain; one
    registry failing does not block others
- **Idempotent** — publishing a version that already exists on a registry skips gracefully (not
    fails the workflow)

### Trigger Configuration

```yaml
on:
  push:
    tags: [v*.*.*]
  workflow_dispatch:
    inputs:
      crates-io:
        description: Publish iscc-lib to crates.io
        type: boolean
        default: false
      pypi:
        description: Publish iscc-lib to PyPI
        type: boolean
        default: false
      npm:
        description: Publish @iscc/lib and @iscc/wasm to npm
        type: boolean
        default: false
      maven:
        description: Publish iscc-lib to Maven Central
        type: boolean
        default: false
```

### Job Conditions

Each job chain activates on either a tag push or its corresponding checkbox:

- **crates.io jobs**: `if: startsWith(github.ref, 'refs/tags/v') || inputs.crates-io`
- **PyPI jobs** (build-wheels, build-sdist, publish-pypi):
    `if: startsWith(github.ref, 'refs/tags/v') || inputs.pypi`
- **npm jobs** (build-napi, build-wasm, publish-npm-lib, publish-npm-wasm):
    `if: startsWith(github.ref, 'refs/tags/v') || inputs.npm`
- **Maven jobs** (build-jni, assemble-jar, publish-maven):
    `if: startsWith(github.ref, 'refs/tags/v') || inputs.maven`

Tag pushes activate all jobs (no inputs are set, but the tag condition passes for all).

### Authentication

| Registry          | Method                       | Secret/Action                                                                       |
| ----------------- | ---------------------------- | ----------------------------------------------------------------------------------- |
| **crates.io**     | OIDC trusted publishing      | `rust-lang/crates-io-auth-action@v1`                                                |
| **PyPI**          | OIDC trusted publishing      | `pypa/gh-action-pypi-publish@release/v1`                                            |
| **npm**           | Token (`NPM_TOKEN` secret)   | `NODE_AUTH_TOKEN` env var                                                           |
| **Maven Central** | GPG signing + Sonatype token | `MAVEN_GPG_PRIVATE_KEY`, `MAVEN_GPG_PASSPHRASE`, `MAVEN_USERNAME`, `MAVEN_PASSWORD` |

All jobs that use OIDC require `permissions: id-token: write`. Maven Central uses the Sonatype
Central Portal via `central-publishing-maven-plugin` with GPG-signed artifacts.

### Idempotency

Each publish job handles "already published" gracefully:

- **crates.io**: check version with `cargo info iscc-lib` before publishing; skip if matches
- **PyPI**: check version via PyPI JSON API before publishing; skip if exists
- **npm**: check version with `npm view` before publishing; skip if exists
- **Maven Central**: check version via Maven Central search API before publishing; skip if exists

### Release Protocol

Lockstep versioning with selective publishing. All packages share a single version number (from
`workspace.package.version` in root `Cargo.toml`), but only the registries with actual changes need
to be published for a given release. Some registries may skip version numbers — this is expected and
acceptable.

**Rationale:** All binding crates are thin FFI wrappers over the same Rust core. Independent
versioning adds overhead without benefit for a single-maintainer project where the bindings have no
independent logic. Users never need a compatibility matrix — the version number is the compatibility
story.

**Full release (all registries):**

1. Bump version in root `Cargo.toml` (`workspace.package.version`)
2. Run `mise run version:sync` to propagate to non-Cargo manifests
3. Run `mise run version:check` to validate consistency
4. Commit: `git commit -m "Release X.Y.Z"`
5. Tag: `git tag vX.Y.Z`
6. Push: `git push && git push --tags`
7. Tag push triggers `release.yml` — all registry jobs activate

**Selective release (single registry):**

1. Bump version in root `Cargo.toml` (if not already at the target version)
2. Run `mise run version:sync` and `mise run version:check`
3. Commit and push (no tag). If on `develop`, merge to `main` first via PR
4. Go to GitHub Actions → Release → Run workflow
5. Check only the registries that need publishing (e.g., `pypi: true`)
6. Unchecked registries skip entirely — no builds, no publish attempts

**When to use selective publishing:**

- Binding-layer bugfix (e.g., Python `__init__.py` wrapper fix) — publish only PyPI
- Rust core change that affects all bindings — full release via tag
- First release of a binding — selective publish for that registry only

## Build Matrices

### Python wheels (maturin)

| OS             | Target                  | Python     |
| -------------- | ----------------------- | ---------- |
| ubuntu-latest  | x86_64                  | abi3-py310 |
| ubuntu-latest  | aarch64                 | abi3-py310 |
| macos-14       | universal2-apple-darwin | abi3-py310 |
| windows-latest | x64                     | abi3-py310 |

One wheel per platform covers Python 3.10 through 3.14+ (abi3 stable ABI).

### Node.js native addons (napi-rs)

| OS             | Target                    |
| -------------- | ------------------------- |
| ubuntu-latest  | x86_64-unknown-linux-gnu  |
| ubuntu-latest  | aarch64-unknown-linux-gnu |
| macos-14       | aarch64-apple-darwin      |
| macos-14       | x86_64-apple-darwin       |
| windows-latest | x86_64-pc-windows-msvc    |

### WASM (wasm-pack)

Single build on ubuntu-latest, platform-independent.

## Package Names and Registries

| Package   | Registry      | Name                                                               |
| --------- | ------------- | ------------------------------------------------------------------ |
| Rust core | crates.io     | `iscc-lib`                                                         |
| Python    | PyPI          | `iscc-lib`                                                         |
| Node.js   | npm           | `@iscc/lib`                                                        |
| WASM      | npm           | `@iscc/wasm`                                                       |
| Java      | Maven Central | `io.iscc:iscc-lib`                                                 |
| Go        | pkg.go.dev    | `github.com/iscc/iscc-lib/packages/go` (tag-based, no publish job) |

### Java JNI native libraries (cargo + Maven)

| OS             | Target                    | Native dir     | Library           |
| -------------- | ------------------------- | -------------- | ----------------- |
| ubuntu-latest  | x86_64-unknown-linux-gnu  | linux-x86_64   | libiscc_jni.so    |
| ubuntu-latest  | aarch64-unknown-linux-gnu | linux-aarch64  | libiscc_jni.so    |
| macos-14       | aarch64-apple-darwin      | macos-aarch64  | libiscc_jni.dylib |
| macos-14       | x86_64-apple-darwin       | macos-x86_64   | libiscc_jni.dylib |
| windows-latest | x86_64-pc-windows-msvc    | windows-x86_64 | iscc_jni.dll      |

Native libraries are bundled inside the JAR under `META-INF/native/{platform}/` and extracted at
runtime by `NativeLoader.java`.

## Version Management

All packages share the workspace version from root `Cargo.toml`. Some manifests derive the version
automatically at build time; others require explicit synchronization.

### Version source of truth

`[workspace.package] version` in the root `Cargo.toml`.

### Automatic propagation (no sync needed)

| Manifest                            | Mechanism                                                                       |
| ----------------------------------- | ------------------------------------------------------------------------------- |
| All `Cargo.toml` members            | `version.workspace = true` — Cargo resolves at build/publish time               |
| `crates/iscc-py/pyproject.toml`     | `dynamic = ["version"]` — maturin reads from `Cargo.toml` at build time         |
| `crates/iscc-wasm/pkg/package.json` | CI script in `release.yml` extracts version from root `Cargo.toml` during build |

### Manual propagation (sync script required)

| Target                          | What is synced                       |
| ------------------------------- | ------------------------------------ |
| `pyproject.toml`                | Root project version (dev workspace) |
| `crates/iscc-napi/package.json` | npm package version                  |
| `crates/iscc-jni/java/pom.xml`  | Maven artifact version               |
| `mise.toml`                     | Default `--version` flag             |
| `scripts/test_install.py`       | Registry check fallback version      |
| `README.md`                     | Maven dependency snippet             |
| `crates/iscc-jni/README.md`     | Maven dependency snippet             |
| `docs/howto/java.md`            | Maven dependency snippet             |
| `docs/java-api.md`              | Maven + Gradle dependency snippets   |

Go modules use git tags for versioning — no manifest version field to sync.

### Sync tooling

| Task            | Command                  | Purpose                                                  |
| --------------- | ------------------------ | -------------------------------------------------------- |
| `version:sync`  | `mise run version:sync`  | Read workspace version, update all 9 sync targets        |
| `version:check` | `mise run version:check` | Validate all targets match; fail if mismatch (run in CI) |

The sync script (`scripts/version_sync.py`) reads the canonical version from root `Cargo.toml` and
updates all targets listed above. It is called before every release commit (see Release Protocol
above). CI runs `version:check` on every push to catch drift.

## Dev Tooling

### Tool Management

**mise** (`mise.toml`) manages tool versions and task definitions. **uv** manages the Python
environment. Neither is used in CI — CI installs tools via GitHub Actions.

### Task Runner (mise tasks)

```bash
mise run test      # Run all tests (cargo test + pytest)
mise run lint      # Format checks + clippy + ruff
mise run format    # Apply formatting (pre-commit auto-fix hooks)
mise run check     # Run all pre-commit hooks
```

### Pre-commit Hooks (prek)

Git hooks managed by **prek** (Rust-based drop-in for `pre-commit`), configured in
`.pre-commit-config.yaml`.

**Pre-commit stage** (fast, auto-fix on every commit): file hygiene (line endings, trailing
whitespace, YAML/JSON/TOML validation), `cargo fmt`, `ruff check --fix`, `ruff format`, `taplo fmt`,
`yamlfix`, `mdformat`.

**Pre-push stage** (thorough quality gates): `cargo clippy`, `cargo test`, `ty check`, Ruff security
scan (`S` rules), Ruff complexity check (`C901`), `pytest` with coverage enforcement.

**Pre-format before committing:** Run `mise run format` before `git add` and `git commit` to prevent
commit failures from hook-applied formatting changes.

### Docs Site

**zensical** builds and deploys documentation to `lib.iscc.codes` via GitHub Pages. `docs.yml`
workflow triggers on push to `main`.

## Verification Criteria

### CI

- [x] All quality gate jobs run on push to `main`/`develop` and on PRs to `main`
- [x] Rust job checks fmt, clippy (workspace-wide), and tests
- [x] Python job checks ruff and runs pytest
- [x] Node.js job builds napi addon and runs tests
- [x] WASM job runs wasm-pack tests
- [x] C FFI job generates headers, compiles, and runs C test program
- [x] Java job builds JNI crate and runs Maven tests
- [x] Go job runs CGO_ENABLED=0 go test and go vet (pure Go, no Rust toolchain)
- [x] Version job runs scripts/version_sync.py --check for manifest consistency
- [x] Bench job runs cargo bench --no-run for compile-only benchmark verification
- [x] CI does not use `mise` — calls tools directly

### Release

- [x] `workflow_dispatch` trigger with boolean inputs for each registry (crates-io, pypi, npm,
    maven)
- [x] Tag push `v*.*.*` triggers all publish jobs
- [x] `workflow_dispatch` with only `pypi: true` builds and publishes only Python wheels
- [x] `workflow_dispatch` with only `crates-io: true` publishes only to crates.io
- [x] `workflow_dispatch` with only `npm: true` builds and publishes only npm packages
- [x] `workflow_dispatch` with only `maven: true` builds and publishes only to Maven Central
- [x] Each registry's jobs are independent — failure in one does not block others
- [x] crates.io uses OIDC trusted publishing (no API key secret)
- [x] PyPI uses OIDC trusted publishing (no API key secret)
- [x] npm uses `NPM_TOKEN` repository secret
- [x] Maven Central uses GPG signing + Sonatype Central Portal credentials
- [x] Python wheels use abi3-py310 (one wheel per platform for Python 3.10+)
- [x] Java JAR bundles 5-platform native libraries under `META-INF/native/`
- [x] Publishing an existing version skips gracefully instead of failing
- [x] All build artifacts uploaded via `actions/upload-artifact@v4`

### Version Sync

- [x] Version defined once in root `Cargo.toml` `[workspace.package]`
- [x] All Cargo crates inherit version via `version.workspace = true`
- [x] Python version derived automatically via maturin `dynamic = ["version"]`
- [x] WASM version synced automatically via CI script in release.yml
- [x] `mise run version:sync` updates all 9 sync targets (manifests, docs, scripts)
- [x] `mise run version:check` validates all targets match (run in CI on every push)
