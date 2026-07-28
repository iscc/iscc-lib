---
name: dep-refresh-reviews
description: Per-slice review recipes for the v0.6.0 "Dependency review and refresh" issue (Rust lockfile, Python uv.lock, Rust direct pins, GitHub Actions) — what to verify and where each slice actually breaks
metadata:
  type: project
---

# Dependency-refresh review recipes (v0.6.0 `[human]` issue, sliced per-ecosystem)

**Why:** the refresh is deliberately sliced so each slice has its own verification surface; using
the generic "config/lockfile-only" shortcut on these under-verifies them. **How to apply:** find the
slice that matches the diff, run its gate set, and verify the hold-back claims rather than trusting
the advance handoff. Progress log lives in `.claude/context/issues.md`.

## Slice 1 — Rust `Cargo.lock` (`cargo update`, no `-p`) — iter 124

Verify `git diff --name-only -- Cargo.toml` is EMPTY (pins untouched: uniffi 0.31 / pyo3 0.29 /
criterion 0.5 / iai-callgrind 0.16 / magnus 0.7 / jni 0.21 / napi 3), then run the full 4-gate set
`mise run test` + `lint` + `audit` + `bench:iai:check` — not the lockfile-only shortcut, since a
bare `cargo update` can shift Ir and pull new transitive licenses. `cargo-deny` is the main risk.
Perf-gate tools are NOT in a fresh container — install per the learnings.md Tooling note first.
GOTCHA: next.md's pin grep `'uniffi = { version = "0.31"'` (inline table) FAILS — the manifest uses
the plain string `uniffi = "0.31"`. That is a next.md defect, not an advance miss; verify the actual
pin form.

## Slice 2 — Python `uv.lock` (`uv lock --upgrade`) — iter 125

`git diff --name-only` must be `uv.lock` + (at most) `pyproject.toml`, NO source. Rebuild the
extension first: `uv sync --group dev` UNINSTALLS the editable `iscc-lib`, so follow with
`uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml`. Then `uv lock --check` +
`mise run test` / `lint` / `check` + `uv run prek run --all-files --hook-stage pre-push` (the `ty`
jump is the risk — 0.0.18 → 0.0.63 passed) + `zensical build` + `gen_llms_full.py`.

A single dev-tool hold-back pin (e.g. `ruff<0.16`) is legitimate scope discipline, NOT gate
weakening: it defers adoption of NEW default lint rules while the held minor still enforces the
exact prior rule set. VERIFY the hold-back is genuine rather than a mask — it is cheap:
`uvx ruff@0.16.0 check .` reproduced the claimed 104 errors (72 in `_lowlevel.pyi`) exactly. Require
an inline `# held: …` comment stating the reason plus a deferred-adoption follow-up.

## Slice 3 — Rust direct pins (`Cargo.toml` pin text changes) — iter 126

Same 4-gate set as slice 1, plus `cargo bench --no-run` (what CI's `Bench (compile check)` runs) — a
dev-only bump can still red clippy via a NEW deprecation (criterion 0.6 deprecated
`criterion::black_box`; `-D warnings` makes it fatal; the fix is `use std::hint::black_box`).

VERIFY HOLD-BACK CLAIMS, do not take them on faith: `cargo info <crate>@<ver>` prints `rust-version`
(criterion 0.8 needs 1.86 vs our declared 1.85 ✓), and a `grep -c` of the named API in the affected
binding proves a claimed migration cost is real (`exception::runtime_error` ×5 in iscc-rb;
`JNIEnv|GlobalRef|AutoLocal` ×41 in iscc-jni). Confirm taplo (`mise run check`) preserved the
`# held:` comments and left no rewrite in the tree.

## Slice 4 — GitHub Actions in `ci.yml` / `docs.yml` — iter 127

Prove the diff is purely mechanical:

```
git diff <base>..HEAD -- .github/ | grep -E '^[+-]' \
  | grep -vE '^(\+\+\+|---)|^[+-] *(- )?uses: '
```

should return only comment lines. Then run the next.md greps + `mise run check` + the REAL gate:

```
gh api "repos/iscc/iscc-lib/commits/<sha>/check-runs?per_page=100" \
  --jq '[.check_runs[]|select(.conclusion!="success")]|length'
```

→ 0. No ellipsis in the sha — `8f76d48...` yields HTTP 422 (Codex caught this in a handoff).

Independently confirm each bumped ref is really the latest major
(`gh api repos/<o>/<r>/releases/latest --jq .tag_name`) AND that a floating `@vN` tag exists
(`gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`) — the two are NOT the same claim.
`astral-sh/setup-uv` publishes no floating major past v7, so `@v9` fails to resolve and `@v9.0.0` is
correct (see `decisions.md` 2026-07-25). Also inventory ALL refs
(`grep -hoE 'uses: [^ ]+' … | sort | uniq -c`) to catch ones the bump table missed. `docs.yml` runs
only on push to `main` → statically verified only; `release.yml` is a separate deferred slice (see
the release.yml entry in [[MEMORY]]).

Bookkeeping: `ci.yml` sets `cancel-in-progress: true` per ref, and every develop commit triggers two
runs (push + `pull_request` from the open develop→main PR) — so check-run totals read ~2× the job
count, and a follow-up push cancels the prior sha's run.

## Slice 5 — JVM manifests (`pom.xml` + `build.gradle.kts`) — iter 128

Local commands are byte-identical to CI's `java:` / `kotlin:` jobs, so run them:
`cargo build -p iscc-jni && mvn test -f crates/iscc-jni/java/pom.xml` (69 tests) and
`cargo build -p iscc-uniffi && ./gradlew test` from `packages/kotlin` (9 tests).

**THE finding to look for in any published-binding toolchain bump:** a KGP bump raises the
*consumer* compiler floor. `kotlin("jvm")` 2.4.10 stamps `mv=[2,4,0]` into the jar — check with
`unzip` + `javap -v -p <class> | grep -E 'major version|mv='` — and puts `kotlin-stdlib:2.4.10` in
the published POM. Empirical floor test (~2 min, do it — Codex and I both flagged this, and only the
test gave the precise boundary): `./gradlew publishToMavenLocal`, then a throwaway `/tmp/kconsumer`
with `kotlin("jvm") version "<X>"` + `mavenLocal()` + an `import`, built via
`/workspace/iscc-lib/packages/kotlin/gradlew -p /tmp/kconsumer compileKotlin`. Result: 2.1.10 and
2.2.21 FAIL, 2.3.21 PASSES (~one minor of tolerance). The transitive stdlib errors on its own, so
`languageVersion` pinning alone is NOT a fix. Clean up `~/.m2/repository/io/iscc` afterwards.
Verdict called PASS_WITH_NOTES + `normal` `[review]` issue, not NEEDS_WORK — nothing ships from
develop (release.yml is `workflow_dispatch`), and the floor is a support-policy call for the owner
(`decisions.md` 2026-07-25).

Other slice-5 gotchas: `mvn -Prelease package -DskipTests` does NOT resolve `maven-gpg-plugin` or
`central-publishing-maven-plugin` (verify-phase / deploy-goal) — check `~/.m2` before believing a
"release plugins resolved" claim; prove versions exist with `repo1.maven.org` `.pom` HTTP 200.
junit-jupiter ≥ 5.12 under Gradle 8.12.1 legitimately needs
`testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.x.y")` — confirm it does not leak
into the published artifact via `./gradlew generatePomFileForMavenPublication`
(`build/publications/maven/pom-default.xml` should list only kotlin-stdlib + jna). Gradle flakes on
this bind mount (`Unable to delete file …/build/kotlin/…`, `NoSuchFileException …/build/reports/…`)
— check `build/test-results/test/*.xml` and re-run after `./gradlew clean` before calling a failure.

## Slice 6 — Go module (`packages/go/go.mod`/`go.sum`) — iter 129

Gate set: `CGO_ENABLED=0 go test -C packages/go -count=1 ./...`, `go vet -C packages/go ./...`,
`go mod tidy -C packages/go -diff` (exit 0 = tidy), + `mise run check`. No Rust/clippy needed (pure
Go, no cgo).

**Green vectors are NOT enough for a `golang.org/x/text` bump** — it ships the Unicode tables that
feed `norm.NFKC`/`norm.NFD` in `packages/go/utils.go`, and all 50 vendored vectors are Unicode ≤ 15.
Prove output-neutrality exhaustively instead (~2 min):

```
# /tmp/godiff/go.mod: replace github.com/iscc/iscc-lib/packages/go => /workspace/iscc-lib/packages/go
# main.go: for cp 0x20..0x10FFFF (skip D800-DFFF) print hex(TextClean) + hex(TextCollapse)
go run -C /tmp/godiff . > /tmp/new.tsv
printf '\nreplace golang.org/x/text => golang.org/x/text v<OLD>\n' >> /tmp/godiff/go.mod
go mod tidy -C /tmp/godiff && go run -C /tmp/godiff . > /tmp/old.tsv && diff /tmp/new.tsv /tmp/old.tsv
```

0.34.0 → 0.40.0 was byte-identical on all 1,112,032 code points AND on invalid-UTF-8 byte sequences
(where 0.40.0's `isInvalid()` refactor actually landed). Root cause of the non-event:
`unicode/norm/tables15.0.0.go` + `tables17.0.0.go` are unchanged between releases and `tables17` is
`//go:build go1.27`, so Unicode 15.0.0 tables are used either way.

Other slice-6 checks: confirm every module is really latest via
`curl -s https://proxy.golang.org/<module>/@v/list | sort -V | tail -3`; read each dep's `go`
directive from `$(go env GOMODCACHE)/<mod>@<ver>/go.mod` to prove no pressure on the `go 1.26.1`
consumer floor; a NEW indirect is legitimate if the bumped dep's own go.mod requires it (cpuid 2.4.0
→ `golang.org/x/sys`).

**Byproduct finding (iter 129, filed as its own `normal` `[review]` issue):** Go stdlib `unicode`
15.0.0 + `unicode.C` including unassigned (Cn) vs Rust `unicode-general-category` 16.0.0 /
`unicode-normalization` 17.0.0 → 5,813 code points where Go and Rust `text_clean` disagree. Go
matches `iscc-core` (Python 3.13 = Unicode 15.1); **the Rust core is the outlier**. Repro: `Ɤ`
U+A7CB. Do not attribute this to a dep bump without running the differential above.

## Slice 7 — Ruby manifests (`crates/iscc-rb/Gemfile` + `Gemfile.lock`) — iter 130

Gate set is byte-identical to CI's `ruby:` job (`ci.yml` ~187-212) — run all four from a subshell
(`bundler has no -C`): `(cd crates/iscc-rb && bundle exec standardrb / rake compile / rake test)`
(111 runs, 299 assertions) + `bundle outdated --strict` (exit 0 = nothing left within constraints).
Needs `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`. `rake compile` may print
`gmake: Clock skew detected` on this bind mount — cosmetic.

**Verify the two hold-backs from upstream metadata, not the handoff.** Version + Ruby floor:
`curl -s https://rubygems.org/api/v1/versions/<gem>.json | jq '.[] | {number, ruby_version}'` (the
**v1** endpoint carries `ruby_version`; the v2 per-version endpoint returns `null`) → minitest 6.0.x
really is `>= 3.2` vs the gem's `required_ruby_version >= 3.1.0`. Transitive pin:
`gem specification rb_sys -v <ver> --remote | grep -A8 'name: rake-compiler-dock'` → 0.9.123 →
`= 1.10.0`, 0.9.124 → 1.11.0, 0.9.128 → 1.12.0, so the exact pin is genuinely load-bearing against
`tag: 0.9.123` in `release.yml`.

Two Ruby-specific traps worth re-checking on any future Gemfile change:

- Tightening `~> 0.9` → `= 0.9.123` rewrites the lock's `DEPENDENCIES` line to `rb_sys (= 0.9.123)`.
    cross-gem's configure step is
    `grep rb_sys Gemfile.lock | head -n 1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+'` — safe here (the
    `GEM specs` line sorts first, and the fallback regex extracts 0.9.123 either way), but check the
    action source before assuming any lock-format change is inert.
- CI uses `ruby/setup-ruby` `bundler-cache: true` (frozen install). Prove the lock is consistent
    with `BUNDLE_FROZEN=true bundle install --local` before passing, not just `bundle exec`.

`crates/iscc-rb/iscc-lib.gemspec` declares **no** dev dependencies — only `required_ruby_version` (a
human-owned consumer floor), so the gemspec side of this slice is a no-op by construction.

## Slice 8 — ruff 0.16 adoption (sub-sliced A–E) — iter 131+

Run the unpinned linter with `uvx ruff@0.16.0 …`; it never touches `uv.lock`, so `ruff<0.16` in
`pyproject.toml` must still be there at the end of any sub-slice before D. Gate set per sub-slice:
`uvx ruff@0.16.0 check . --statistics` (count must match next.md exactly) + `uv run ruff check .` /
`ruff format --check .` (pinned 0.15 must stay green) +
`uv run ruff check --select S --force-exclude` + `--select C901` + `uv run ty check` +
`uv run pytest -q` + `mise run check`.

- **The `--select S`/`C901` runs are still the whole point** even though slice B put both in
    `extend-select`: the two pre-push hooks (`.pre-commit-config.yaml` ~93-106) are what actually
    red a push. Any `ruff --fix` that deletes a `# noqa: S603/S607` in `tools/`/`scripts/` opens a
    hole — always run `git diff --stat -- tools/ scripts/` and grep the counts (`tools/cid.py`: 6 ×
    S603, 4 × S607)
- **Sub-slice A (iter 131, PASS)**: 36 lone `...` deleted from `_lowlevel.pyi` (PIE790+PYI048
    double-report one line → N errors = N/2 deletions), 6 RUF059 `_`-prefixed. 104 → 26 findings
- **Verify a `.pyi` edit against mypy AND pyright, not just `ty`** — the wheel ships `py.typed` next
    to `_lowlevel.pyi`, so it is a consumer-facing artifact and `ty` is the only repo gate on it. 30
    seconds: `uvx mypy@1.18.2 --strict <copy>.pyi` and `uvx pyright@1.1.407 <path>`. Both accept
    docstring-only stub bodies. Also AST-check bodies rather than trusting greps: `ast.parse` +
    assert every `FunctionDef.body` is a single string-constant `Expr`
- **Sub-slice B (iter 134, PASS) was a gate *strengthening***:
    `[tool.ruff.lint] extend-select = ["S", "C901"]` cleared all 15 RUF100 *and* promoted both scans
    into `ruff check` (pre-commit, `mise run lint`, CI). It must be `extend-select`, NEVER `select`
    (which replaces ruff's `E4`/`E7`/`E9`/`F` defaults — assert with tomllib that no `select` key
    exists). A step that instead deletes the noqas or adds `per-file-ignores`/`ignore` for them →
    NEEDS_WORK. Keep the `tests/**` per-file-ignores (S101/S603/S607). 26 → 12 findings
- **THE probe for any `# noqa` deletion** (~5s, settles it):
    `uv run ruff check --select <rule> --ignore-noqa --output-format concise` lists the real
    violations with suppressions off — if the deleted directive's line is absent, it was dead
    weight; if present, deletion opens a hole. `uv run ruff check --select S,RUF100` and
    `--extend-select RUF100` show unused directives under the *pinned* ruff. Iter 134 finding:
    `S603` never fires on a fully static list-literal argv in 0.15.22 *or* 0.16.0 (only on
    `["git", "add", rel]` / `["git", *args]`), so the handoff's "0.16 refined S603" attribution was
    wrong — the real change is that 0.16 default-selects `RUF100`
- **Sub-slice C (iter 135, PASS) was also a strengthening**:
    `[tool.ruff] src = [".", "crates/iscc-py/python"]` +
    `[tool.ruff.lint.isort] combine-as-imports = true` +
    `extend-select = [… "I", "RUF022", "RUF100"]`, 7 mechanical fixes via
    `uv run ruff check --fix --select I,RUF022 .`. 12 → 3. **The C-specific probe** (~10s, nothing
    else catches it): the pre-commit `ruff-check` hook runs `uv run ruff check --fix` with *filenames*
    and no `--force-exclude`, so hook-mode ≠ `ruff check .` in principle. Re-run it by hand on the
    touched paths and assert `git diff --stat` is empty — proves `src`/isort resolve identically
    per-file. Also diff `__all__` as a SET across `HEAD~1..HEAD` (`ast.parse`, not a grep) — RUF022
    reorders 49 strings and a dropped symbol would be invisible in a reordering diff
- **Sub-slice D (iter 136, PASS)** — the 3 one-liners; 3 → **0**, `uvx ruff@0.16.0 check .` exits 0.
    Three review-specific probes, all cheap: (1) `RUF007` was in the **checked-in generator**
    `scripts/gen_unicode16_unassigned.py`, so re-run it (`uv run --script …`, ~40s) and assert
    `git status --porcelain crates/` is empty — `pairwise(xs)` ≡ `zip(xs, xs[1:])` for every length
    incl. 0/1, so it is provably behaviour-preserving. (2) `PLW1510` → `check=False` is the
    `subprocess.run` default, so also behaviour-preserving; the retained `# noqa: S603` must be
    proven still-LIVE with `--select S603 --ignore-noqa` (the inverse of the deletion probe — a
    retained-but-dead directive would red `RUF100`). (3) `EXE001` on `tools/cid.py` was fixed as a
    **mode-only** commit: `git diff HEAD~1 --summary` → `mode change 100644 => 100755`, blob hash
    identical. `git status` cannot show it (`core.fileMode=false` on the 9p mount) — verify with
    `git ls-files -s <path>`, and check nothing invokes the script directly
    (`grep -n cid.py mise.toml` → all 10 tasks use `uv run tools/cid.py`, so the bit is additive)
- **Sub-slice E (iter 137, PASS_WITH_NOTES)** — the pin drop; ruff is 0.16.0 project-wide, slice 8
    CLOSED. Zero findings and zero reformats at the flip. Recipes worth reusing on ANY
    single-package relock: (1) prove nothing else moved with
    `git diff HEAD~1..HEAD -- uv.lock | grep -E '^[+-]name = '` → **empty** (the `name =` line of
    the bumped package is context, not a change) plus `git diff -U0 … | grep -c '^@@'` → 2; (2) a
    linter/formatter bump can change **file discovery**, not just rules — diff the file COUNT of the
    bare command before/after (`ruff format --check`: 25 → 153, ruff 0.16 formats Python fences
    inside Markdown); (3) check the new version's `requires-python` against the CI matrix floor
    (`pypi.org/pypi/<pkg>/<ver>/json`) — ruff 0.16.0 is `>=3.7`, so the 3.10 leg is safe
- **GATE-PARITY TRAP found in that review (now a `normal` `[review]` issue):** the widened
    `ruff format` surface is enforced ONLY by CI — the prek `ruff-check`/`ruff-format` hooks declare
    `types: [python]` and pre-push never runs `ruff format`. Probe (~10s, keeps the file untracked
    and deletes it after): write a repo-local `probe.md` with `x=1` in a `python` fence →
    `uv run prek run ruff-format --files probe.md` says `(no files to check) Skipped` while
    `uv run ruff format --check probe.md` exits 1. Whenever a tool bump widens a bare-command
    surface, always ask which LOCAL gate covers the new surface
- The pin's `# held:` reason went stale three times ("104 new errors" while 12 remained, "12" while
    3 remained, "3" while 0 remained), and iter 137 left a fourth stale comment elsewhere in
    `pyproject.toml` ("under the pinned ruff" in the `extend-select` block). After ANY pin removal,
    `grep -n 'pin\|held\|<0\.' pyproject.toml` for prose that outlived it — comment-only review fix

## Remaining slices

All locally-verifiable slices are done (1-8; slice 8 closed iter 137). napi `package.json`
(`@napi-rs/cli: ^3`) and dotnet `.csproj` (`17.*`/`2.*` wildcards) were re-checked current iter 129
— no edit needed. xunit.v3 3.x + Test.Sdk 18.x CLOSED iter 164 (below). Gradle wrapper 9.6.1 CLOSED
iter 165, JUnit 6.1.2 CLOSED iter 166 (both below), **magnus 0.8 CLOSED iter 167** (gates + the
corrected floor facts → `binding-reviews.md` "Ruby Binding Review"; lock delta was magnus +
magnus-macros + magnus's own build-dep `rb-sys-env` 0.1.2→0.2.3, all reaching only `iscc-rb` per
`cargo tree -i`). **jni 0.22 CLOSED iter 168** — 0.22.4, `rust-version = 1.85` == workspace MSRV so
no floor moved; lock delta (+jni-macros/simd_cesu8/simdutf8, -cesu8/thiserror 1.x/windows-sys 0.45)
reaches only `iscc-jni`; review recipe → `binding-reviews.md` "JNI crate review". What is left:
`release.yml` GHA refs (not CI-exercised, but CID-doable on static evidence per the 2026-07-25
decision), `uniffi` 0.32 and `criterion` 0.8 (both human-gated). Watch for the slice-5 lesson in any
published binding: a runtime/toolchain floor moving silently.

## JVM test-framework major (iter 166, JUnit 5.14.4 → 6.1.2, Gradle + Maven) — ~10 min

JUnit 6 gives Platform/Jupiter/Vintage ONE version number, so `junit-platform-launcher` jumps 1.14.4
→ 6.1.2 alongside `junit-jupiter`; a leftover 1.x pin is the obvious tell. Recipe:

- `packages/kotlin/gradlew -p packages/kotlin -q dependencies --configuration testRuntimeClasspath`
    — every `org.junit.*` row must read the new version (proves the bump took effect, not a cached
    5.x resolution).
- **Derive the expected case total from the FIXTURE**, cheaper and stronger than archiving the
    pre-bump tree: parse `crates/iscc-jni/java/target/surefire-reports/TEST-*.xml`, bucket
    `name()[N]` dynamic cases per factory, and require each bucket to equal the matching `data.json`
    per-function vector count (20/5/3/5/3/2/4/3/5 = 50) + 19 static `@Test` = 69;
    `UnicodeBoundaryTest` = 7+5 dynamic + 1 static = 13. Total 82.
- **`mvn test` reuses previously compiled test classes** ("Nothing to compile — all classes are up
    to date"), so it can pass without ever compiling against the new framework. Run `mvn clean test`
    once: expect `Compiling 4 … / Compiling 2 source files with javac [debug target 17]`.
- Consumer surface: `gradlew -p packages/kotlin -q generatePomFileForMavenPublication`, then
    `grep -c junit build/publications/maven/pom-default.xml` must be **0** (Gradle publishes
    `components["java"]`, so `testImplementation`/`testRuntimeOnly` never leak); the Maven side is
    guarded by `<scope>test</scope>`. Java floor 17 = CI's `temurin 17` = local JDK, no floor move.
- Surefire 3.5.6 auto-resolves the aligned launcher — an explicit pom launcher dependency is NOT
    needed and its presence would be a smell.

## Build-tool wrapper major (iter 165, Gradle 8.12.1 → 9.6.1) — ~15 min

A wrapper bump ships a **binary blob** (`gradle-wrapper.jar`) plus tool-generated scripts. Never
review it by reading the diff — prove provenance twice, both cheap:

1. **Publisher checksum:**
    `curl -sSL https://services.gradle.org/distributions/gradle-<v>-wrapper.jar.sha256` (needs
    `-L`; the bare URL 301s) must equal `sha256sum` of the file AND of the committed blob
    (`git cat-file blob <hash> | sha256sum`).
2. **Independent regeneration:**
    `mkdir /tmp/wraptest && printf 'rootProject.name = "x"\n' >  /tmp/wraptest/settings.gradle.kts && packages/kotlin/gradlew -p /tmp/wraptest wrapper  --gradle-version <v>`
    — without a settings file the `wrapper` task FAILS. Then diff all four artifacts;
    `gradlew.bat` matches only after `sed 's/\r$//'` (Gradle emits 82 CRLF lines, the repo stores
    LF via `.gitattributes` + `mixed-line-ending`). This also proves any new
    `gradle-wrapper.properties` keys (9.x adds `retries`/`retryBackOffMs`) are tool defaults.

Then: `cargo build -p iscc-uniffi` → `gradlew -p packages/kotlin clean build`, read
`build/test-results/test/*.xml` for `tests="9"` / `tests="13"` with `skipped="0"`, and
`grep -in deprecat` the captured log (that KGP diagnostic is the point of the bump). **Also probe
the release-only path** `gradlew -p packages/kotlin publishMavenPublicationToStagingRepository`: it
writes to a local `build/staging-deploy`, `signMavenPublication` is SKIPPED without
`MAVEN_GPG_PASSPHRASE`, so `release.yml`'s never-CI'd publish step is fully verifiable offline.

GOTCHAS: Gradle 9 writes `build/reports/problems` at the END of a build, so a **concurrent** build
(the background Codex run) makes `clean` fail "Unable to delete directory … New files were found" —
re-run sequentially, it is not a defect. KGP's session dir `packages/kotlin/.kotlin/sessions` is
untracked and un-ignored but exists only during a build. Gradle 9 deletes the distribution zip after
extraction, so advance cannot hash the cached zip — the wrapper-jar checksum above is the better
provenance anyway.

## dotnet test-framework major (iter 164, xunit v2 → v3 + Test.Sdk 18) — ~10 min

Review recipe for any **test-framework** bump, .NET or otherwise:

- **The floor in next.md is not the check — the pre-bump tree is.**
    `git archive HEAD~1 | tar -x -C   /tmp/dnold`, then
    `dotnet test /tmp/dnold/packages/dotnet/Iscc.Lib.Tests/ -e   LD_LIBRARY_PATH=$PWD/target/debug`
    (reuse the real `target/debug`, no second cargo build). v2 and v3 both gave **exactly 104** —
    that equality, not "104 ≥ 85", is what proves `[MemberData]` rows did not silently collapse. A
    collapse of the 12 boundary rows would still have cleared 85.
- `--list-tests` counts test **methods** (53), not rows. For the row breakdown run
    `--logger "console;verbosity=detailed"` and `grep -cE '^\s+Passed\s+'`: 50 conformance + 41
    smoke + 13 boundary = 104. Boundary is 13 = 12 fixture vectors (7 `text_clean` + 5
    `text_collapse`) + 1 metadata-guard fact — do not read 13 as a 13th vector.
- Cheap scope checks: `git diff HEAD~1..HEAD --name-only -- packages/dotnet/` (must not list
    `Iscc.Lib/Iscc.Lib.csproj`), and grep the csproj for
    `TestingPlatformDotnetTestSupport|UseMicrosoftTestingPlatformRunner` (must be absent — `-e` is a
    VSTest feature; see `decisions.md` 2026-07-27).
- Run the CI sequence from a clean tree (`rm -rf packages/dotnet/*/bin packages/dotnet/*/obj`, then
    `dotnet build …csproj` and `dotnet test …/`) — CI always starts fresh. `dotnet build` must print
    `0 Warning(s)`; there is no `Directory.Build.props`, `global.json`, `.sln` or
    `TreatWarningsAsErrors`, so analyzer warnings never fail a build and must be read off the log.
- **Floating wildcards (`3.*`, `18.*`) and no lock file** are the pre-existing .NET style: CI can
    resolve a newer 3.x/18.x than was reviewed. Not a defect to file, but say so in the handoff.

## Dependency/tool-bump review reflexes (moved from MEMORY.md index, iter 149)

- **A tool bump can widen a gate's FILE DISCOVERY, not just its rules** (iter 137: ruff 0.16 made
    `ruff format --check` go 25 → 153 files). Diff the count, then ask **which local gate covers the
    new surface**. Relock proof: `git diff HEAD~1..HEAD -- uv.lock | grep -E '^[+-]name = '`.
- **Verify a `# held:` claim from registry metadata, never from the handoff** (iters 126/130, ~30s):
    `cargo info <crate>@<ver>`, `gem specification <gem> -v <ver> --remote`,
    `https://rubygems.org/api/v1/versions/<gem>.json`. A wrong stated reason survives as folklore.
- **`cargo tree -i <crate>` prints "nothing to print"** for proc-macro / target-specific deps — add
    `--target all`. This disproved an advance-handoff attribution: `proc-macro-error2` comes from
    dev-only `iai-callgrind-macros`, NOT the magnus/rb-sys subtree.
