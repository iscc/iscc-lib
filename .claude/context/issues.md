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
(iter 124; ~100 crates to latest semver-compatible, all `Cargo.toml` pins held, all gates green). ✅
Slice 2 — Python `uv.lock` refreshed via `uv lock --upgrade` (iter 125; 40 pkgs incl. iscc-core
1.2.2→1.3.0, ty 0.0.18→0.0.63, maturin 1.14.1, prek 0.4.11; one documented hold-back `ruff<0.16` in
`pyproject.toml` — 0.16 adds 104 new default-lint errors, deferred to a dedicated adoption step; all
gates green). ✅ Slice 3 — Rust direct-pin evaluation (iter 126; `criterion` 0.5→0.7 with the bench
import migrated to `std::hint::black_box`, and inline `# held:` reasons committed next to the
`criterion` 0.8 / `jni` 0.22 / `magnus` 0.8 / `uniffi` 0.32 hold-backs plus a `pyo3`/#41 note; all
gates green, perf within +1.96%). ✅ Slice 4 — GitHub Actions in `.github/workflows/ci.yml` +
`docs.yml` (iter 127; 9 distinct refs bumped to current majors — checkout v7, setup-python v7,
setup-node v7, setup-java v5, setup-go v7, setup-dotnet v6, upload-artifact v7, upload-sarif v4,
upload-pages-artifact v5 + deploy-pages v5 paired; `astral-sh/setup-uv` pinned to the **exact** tag
`@v9.0.0` because upstream publishes no floating major past v7 — see `decisions.md` 2026-07-25; CI
41/41 green on `8f76d48`). ✅ Slice 5 — JVM manifests (iter 128; `pom.xml`: junit-jupiter 5.14.4,
gson 2.14.0, compiler 3.15.0, surefire 3.5.6, source 3.4.0, javadoc 3.12.0, gpg 3.2.8, with
`central-publishing-maven-plugin` held at 0.7.0 under a `held:` comment — its `deploy` goal only
runs in a real Central publish; `build.gradle.kts`: kotlin("jvm") 2.4.10, jna 5.19.1, junit/gson in
lockstep, plus a required `testRuntimeOnly junit-platform-launcher:1.14.4`; mvn 69/69 and gradle 9/9
green). **The Kotlin plugin bump raised the consumer Kotlin floor — see the follow-up issue below.**
✅ Slice 6 — Go module (iter 129; `golang.org/x/text` 0.34.0 → 0.40.0 direct,
`github.com/klauspost/cpuid/v2` 2.0.12 → 2.4.0 indirect, new indirect `golang.org/x/sys` v0.47.0
pulled in by cpuid; `go 1.26.1` consumer floor and `zeebo/blake3` v0.2.4 untouched — all three are
the latest published versions; no hold-back needed). The `x/text` bump was proven
**output-neutral**, not just vector-green: `TextClean`/`TextCollapse` are byte-identical across all
1,112,032 code points under 0.34.0 and 0.40.0 (the `unicode/norm` tables files are unchanged between
the two releases; only invalid-rune bookkeeping was refactored). Also wired
`packages/kotlin/README.md` into `scripts/version_sync.py` `TARGETS` (22 targets now), closing the
last unmanaged stale version string (`0.3.1` → `0.5.0`). ✅ Slice 7 — Ruby manifests (iter 130;
`crates/iscc-rb/Gemfile.lock` refreshed by `bundle update`: rake 13.4.2, standard 1.56.0, rubocop
1.88.2, rubocop-minitest 0.40.0 + transitives — each verified to be the newest published version;
zero `standardrb` fallout, 111 tests green, `bundle outdated --strict` clean). `rb_sys` tightened
from `~> 0.9` to an **exact** `0.9.123` in the `Gemfile` under a `# held:` comment, plus a second
`# held:` for `minitest ~> 5.0` — both hold-backs verified from registry metadata this review
(`gem specification rb_sys -v <v> --remote`: 0.9.123 → `rake-compiler-dock = 1.10.0`, 0.9.124 →
1.11.0, 0.9.128 → 1.12.0, so the pin genuinely guards `tag: 0.9.123` of
`oxidize-rb/actions/cross-gem` in `release.yml`; rubygems v1 API: minitest 6.0.x requires Ruby 3.2
or newer, above the gem's declared floor of 3.1.0). `crates/iscc-rb/iscc-lib.gemspec` needed no
change — it declares no dev dependencies, only the human-owned `required_ruby_version`. **This
closes every locally-verifiable ecosystem slice.** 🔄 Slice 8 (ruff 0.16 adoption) — **sub-slice A
done** (iter 131): the 78 config-free findings cleared — 36 lone `...` stub bodies deleted from
`crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (`PIE790`+`PYI048` double-report the same line) and 6
`RUF059` unused unpackings `_`-prefixed in `tests/test_new_symbols.py`. Docstring-only stub bodies
verified downstream-safe against `ty`, `mypy 1.18 --strict` and `pyright 1.1.407` (the wheel ships
`py.typed` alongside the stub). Baseline 104 → **26 findings left**, each needing a decision:
`RUF100` 15 + `EXE001` 1 + `PLW1510` 1 in `tools/`/`scripts/` (the `RUF100`s are the load-bearing
`# noqa: S603/S607` — 0.16 calls them unused only because `S` is not in the default select, so
deleting them would red the pre-push `ruff check --select S` gate; the fix is a lint-config decision
such as adding `S`/`C901` to `[tool.ruff.lint] select`), plus `I001` 8 + `RUF022` 1 needing an isort
src-root decision. The `ruff<0.16` pin stays until the tree is clean under 0.16.

Verified already-current and needing no bump: `.pre-commit-config.yaml` (pre-commit-hooks v6.0.0,
mdformat 1.0.0), `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`,
`taiki-e/install-action@v2`, `ruby/setup-ruby@v1`, `obi1kenobi/cargo-semver-checks-action@v2`;
`mise.toml` has no `[tools]` section; `crates/iscc-napi/package.json` (`@napi-rs/cli: ^3` floats
over the 3.x line, covers 3.7.4) and `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`
(`Microsoft.NET.Test.Sdk 17.*`, `xunit 2.*`, `xunit.runner.visualstudio 2.*` wildcards) — both
re-checked iter 129, editing them would be churn. Remaining: `.github/workflows/release.yml` GHA
refs (97 `uses:`; `upload-artifact@v4` ↔ `download-artifact@v4` must move together, `setup-uv` needs
`@v9.0.0`, only truly exercised by a release run — consider bundling with the existing release.yml
`if:`-guard fix issue), plus the deferred majors: xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle
wrapper 8.12.1 and JUnit 6.x (each its own step). A dedicated step should adopt ruff 0.16 (run
`ruff check --fix` for the 60 auto-fixable, hand-fix the rest — mostly `_lowlevel.pyi` stub-style
PIE790/PYI048/RUF022 — and drop the `ruff<0.16` pin); this is the best-scoped next slice since it is
self-contained and fully locally verifiable. Separately, the `jni` 0.22 and `magnus` 0.8 migrations
each need their own step (source rewrite in `crates/iscc-jni/src/lib.rs` and
`crates/iscc-rb/src/lib.rs` respectively).

**Known constraint (verified iter 126):** the `proc-macro-error2 v2.0.1` future-incompat warning
(`extern crate proc_macro is private and cannot be re-exported`) emitted on every `cargo test` /
`cargo bench` comes from `iai-callgrind-macros` → `iai-callgrind` (dev-only; trace it with
`cargo tree -i proc-macro-error2 --target all`), **not** from the magnus/rb-sys subtree. No fixed
release exists (`iai-callgrind` 0.16.1 is latest), so it stays a warning until upstream ships a fix
— re-check when bumping `iai-callgrind` (the pin must stay in lockstep with the CI-installed
`iai-callgrind-runner` version).

## Kotlin binding silently raised the consumer Kotlin floor to 2.3 `normal` [review]

Iteration 128 bumped `kotlin("jvm")` 2.1.10 → 2.4.10 in `packages/kotlin/build.gradle.kts` (an
in-scope dependency-refresh bump). Side effect: the published jar now carries Kotlin metadata
version `mv=[2,4,0]` (`javap -v -p build/libs/…/AudioCodeResult.class | grep mv=`) and the published
POM declares `kotlin-stdlib:2.4.10` in `compile` scope (was 2.1.10).

**Verified empirically** (iter 128 review) with a throwaway consumer project resolving
`io.iscc:iscc-lib-kotlin:0.5.0` from `mavenLocal`:

- Kotlin **2.1.10** consumer → `compileKotlin` FAILS:
    `Module was compiled with an incompatible   version of Kotlin. The binary version of its metadata is 2.4.0, expected version is 2.1.0`
    — raised for both `iscc-lib-kotlin-0.5.0.jar` and the transitive `kotlin-stdlib-2.4.10.jar`
- Kotlin **2.2.21** consumer → FAILS identically
- Kotlin **2.3.21** consumer → BUILD SUCCESSFUL (Kotlin tolerates ~one minor ahead)

So the supported-consumer floor moved from Kotlin ≥ 2.0/2.1 to ≥ 2.3, undocumented. Nothing has
shipped — this only reaches users at the next Maven Central publish of `io.iscc:iscc-lib-kotlin`, so
**resolve before the next release**. Two options, both small:

1. **Accept and document** — state "requires Kotlin 2.3+" in `packages/kotlin/README.md`,
    `docs/howto/kotlin.md`, the root README Kotlin section, and record the support policy in the
    spec. Matches JVM-ecosystem norms and costs a few doc lines.
2. **Preserve the old floor** — hold `kotlin("jvm")` at 2.1.x with a `// held:` comment (the
    project's existing hold-back convention). Note that pinning `compilerOptions.languageVersion`
    alone is **not** sufficient: the transitive `kotlin-stdlib:2.4.10` in the published POM
    triggers the same error independently, so the stdlib version would have to be constrained too.

**Spec:** `.claude/context/specs/kotlin-bindings.md` — it documents no consumer Kotlin version
floor. **HUMAN REVIEW REQUESTED**: picking the supported-consumer Kotlin version is a support-policy
decision (the same class as MSRV and `java-version: '17'`, which `next.md` deliberately keeps out of
dependency-refresh steps). A CID agent should not set it unilaterally; the review agent recommends
option 1.

## Rust core diverges from `iscc-core` and the Go package on Unicode 16/17 characters `normal` [review]

The Unicode data version is unpinned and differs per implementation, so `text_clean` /
`text_collapse` — and therefore Meta-Code, Text-Code and the returned `name`/`description` fields —
disagree across implementations for any text containing a character assigned after Unicode 15.

| Implementation                            | Unicode version                                       |
| ----------------------------------------- | ----------------------------------------------------- |
| Go stdlib `unicode` (Go 1.26.1)           | 15.0.0                                                |
| Go `x/text/unicode/norm`                  | 15.0.0 (its `tables17.0.0.go` is `//go:build go1.27`) |
| Python 3.13 `unicodedata` (→ `iscc-core`) | 15.1.0                                                |
| Rust `unicode-general-category` 1.1.0     | **16.0.0**                                            |
| Rust `unicode-normalization` 0.1.25       | **17.0.0**                                            |

Go's `unicode.C` range table **includes unassigned code points (Cn)**, and
`TextClean`/`TextCollapse` strip category C — so every code point assigned in Unicode 16/17 is
dropped by Go and by the Python reference, but kept by the Rust core. A full sweep of the code space
(iter 129 review) found **5,813 code points** where Go and Rust `text_clean` disagree and **5,750**
where `text_collapse` disagrees.

**Reproduction** (`Ɤ` = U+A7CB LATIN CAPITAL LETTER RAMS HORN, added in Unicode 16):

```text
s = "The quick brown fox Ɤ jumps over the lazy dog and keeps running far away"

go       meta=ISCC:AAARDZ4ASOMVXBRR  text=ISCC:EAA7VW5ZOQZ3XEMT  name="The quick brown fox jumps …"
iscc-core meta=ISCC:AAARDZ4ASOMVXBRR text=ISCC:EAA7VW5ZOQZ3XEMT  name='The quick brown fox jumps …'
rust     meta=ISCC:AAARDZ5SS6NVXBLT  text=ISCC:EAA3RXNBOM77TGM5  name='The quick brown fox Ɤ jumps …'
```

Go matches the reference; **the Rust core is the outlier**, and all 10 non-Go bindings inherit its
output. No gate catches this: every vendored conformance vector predates Unicode 16.

The divergence is inherently unstable in both directions — when CPython ships Unicode 16 (3.14),
`iscc-core` will move to the Rust side and away from Go. So this is not simply "bump/pin one crate";
ISO 24138 does not pin a Unicode version, which is arguably an upstream spec gap worth raising with
`iscc/iscc-core` once this project decides its own position.

**Options:** (a) pin the Rust core to the reference's Unicode version (needs an older
`unicode-general-category` / a vendored category table, and re-pins on every reference upgrade); (b)
declare a Unicode version in the spec and add conformance vectors covering post-15 code points so
all implementations are checked against it; (c) document the divergence as accepted and
out-of-contract for post-15 characters.

Not introduced by the iter-129 `golang.org/x/text` refresh — that bump was verified byte-identical
across all 1,112,032 code points. Pre-existing and previously unnoticed.

**Spec:** `.claude/context/specs/rust-core.md` → conformance / `iscc-core` output-compatibility
claim. **HUMAN REVIEW REQUESTED**: choosing a Unicode version (or accepting the divergence) is a
conformance-policy decision with an upstream dimension; a CID agent should not pick it unilaterally.

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
