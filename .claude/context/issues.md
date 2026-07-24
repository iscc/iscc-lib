# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files.

<!-- Add issues below this line -->

## Dependency review and refresh across the project `normal` [human]

Planned for the **v0.6.0** release. No automated dependency updates are configured (no
Dependabot/Renovate), so manifests drift between releases. Review and refresh third-party
dependencies across the full surface: root `Cargo.toml` workspace deps + `Cargo.lock`,
`pyproject.toml` + `uv.lock`, `crates/iscc-napi/package.json`, `crates/iscc-rb/Gemfile` + gemspec,
`crates/iscc-jni/java/pom.xml`, `packages/kotlin/build.gradle.kts`, `packages/dotnet/*/*.csproj`,
`packages/go/go.mod`, plus tooling pins: `mise.toml`, `.pre-commit-config.yaml`, and GitHub Actions
versions / pinned CI tools in `.github/workflows/`. Patch/minor bumps by default; evaluate majors
individually and document any deliberately held-back version next to its pin. **Known pinning
constraints:** PyO3 bumps only together with re-verifying `gil_used = true` semantics and the
`py.detach` call sites (interacts with #41); rb_sys in `Gemfile.lock` must match the
`oxidize-rb/actions/cross-gem` Docker image tag; wheels stay `abi3-py310`; quality-gate CI tool pins
(e.g. `cargo-crap`) bump together with their baselines. All quality gates and conformance vectors
must pass on the refreshed set.

**Spec:** `.claude/context/specs/ci-cd.md` → "Dependency Freshness"

**Progress (sliced per-ecosystem):** ✅ Slice 1 — Rust `Cargo.lock` refreshed via `cargo update`
(iter 124; ~100 crates to latest semver-compatible, all `Cargo.toml` pins held, all gates green).
Remaining: Rust direct-pin evaluation (document hold-back reasons for uniffi 0.32 / pyo3 #41 /
criterion / iai-callgrind / magnus / jni / napi majors), Python `uv.lock`, per-binding manifests
(napi `package.json`, rb `Gemfile`/gemspec, jni `pom.xml`, kotlin `build.gradle.kts`, dotnet
`.csproj`, go `go.mod`), and tooling pins (`mise.toml`, `.pre-commit-config.yaml`, GHA versions).

## Release core as v1.0.0 (stability commitment) `low` [human]

Human-driven release: cut **v1.0.0** as the first stability-committed release of the lockstep
workspace (per the 1.0.0 decision). This is the one release allowed to break the 0.4.0 API freely;
afterward 1.x is locked under strict SemVer. Drive via the `/release` skill — do NOT let the CID
loop cut this release autonomously. Ideally land both the `cargo-semver-checks` and `iai-callgrind`
gate issues above first so 1.0.0 ships with enforcement active.

**Status (2026-06-18):** Titusz decided to **hold** the v1.0.0 cut and stay on 0.4.x for now — land
the CRAP `--fail-above` and `cargo deny` hardening gates first, then flip the `cargo-semver-checks`
gate to enforcing as part of the eventual cut. Remains `low` `[human]`; CID must not cut it
autonomously.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.

## Migrate npm publishing to OIDC Trusted Publishing `normal` [human]

The v0.5.0 release failed both npm publishes (`@iscc/lib`, `@iscc/wasm`) because the `NPM_TOKEN`
secret had expired (npm caps write-token expiry at 90 days). npm now supports **OIDC Trusted
Publishing** — the same keyless mechanism already used for crates.io, PyPI, and RubyGems — which
removes `NPM_TOKEN` entirely and eliminates this expiry class of failure. npm's own token UI
recommends it for CI/CD.

**Scope:** update the two npm publish jobs in `.github/workflows/release.yml` (`Publish @iscc/lib`,
`Publish @iscc/wasm`) to publish via OIDC (drop `NODE_AUTH_TOKEN`/`NPM_TOKEN`, rely on the existing
`id-token: write` permission + `npm publish --provenance`). **Human-gated:** requires configuring a
Trusted Publisher for each package on npmjs.com (link repo `iscc/iscc-lib` + the release workflow)
before the token can be removed — CID can prepare the YAML diff but must not delete `NPM_TOKEN`
until the npm-side trusted publisher is live and a publish has succeeded.

Interim mitigation already in place: release skill Step 1.6 checks npm token expiry pre-flight, and
the token was rotated (`iscc-lib-ci-2026`, granular `@iscc` scope, expires 2026-09-16).

## Fix broken single-registry re-trigger in release.yml `normal` [human]

`gh workflow run release.yml --ref main -f <registry>=true` is documented as the way to re-publish a
single failed registry, but it **silently publishes nothing** for npm/pypi/maven. Root cause: with
no `version` input, `prepare-release` (`if: inputs.version != ''`) is skipped, and GitHub propagates
that skip down the `needs` chain to any job whose `if:` lacks a `!cancelled() && !failure()` guard.
Only the `build-*` jobs and `publish-crates-io` currently have that guard; the `test-*` and
`publish-*` jobs for npm (`test-napi`, `publish-npm-lib`, `test-wasm`, `publish-npm-wasm`), pypi
(`test-wheels`, the PyPI publish), and maven (`test-jni`, `assemble-jar`, the Maven publishes) do
not, so they skip. Verified empirically on 2026-06-18 (a `-f npm=true` run built artifacts then
skipped every test/publish job).

**Fix:** add `${{ !cancelled() && !failure() && (<existing condition>) }}` guards to all `test-*`
and `publish-*` job `if:` conditions, matching `publish-crates-io` (line ~128). Then
`-f <registry>=true` re-triggers will publish as documented. Until then, recover failed publishes
with `gh run rerun <run-id> --failed` (works because failed jobs reran cleanly for v0.5.0). Update
the release skill's "Re-triggering a Failed Registry" section and the `release-workflow.md` memory
once fixed.
