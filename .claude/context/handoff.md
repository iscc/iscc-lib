# Handoff

## 2026-07-25 — Review of: Dependency refresh slice 4 — GitHub Actions versions in ci.yml + docs.yml

**Verdict:** PASS

**Summary:** Slice 4 landed exactly as scoped: 9 distinct action refs bumped to their current majors
across `ci.yml` (29 `uses:` lines) and `docs.yml` (5), with zero changes to jobs, steps, inputs,
permissions, or runtime versions — the diff contains **only** `uses:` lines plus two 2-line
explanatory comments. The one deviation from next.md (`astral-sh/setup-uv@v9` → `@v9.0.0`) is forced
by upstream and I verified it independently; CI is 41/41 green on the fix commit.

**Verification:**

- [x] YAML parses — `uv run python -c "import yaml; ..."` exit 0 for both files (system `python3`
    genuinely lacks PyYAML; the venv one works — the next.md command as written would fail)
- [x] No stale refs — the next.md grep prints nothing, exit 1
- [x] `grep -c 'actions/checkout@v7'` → ci.yml **18**, docs.yml **1**
- [x] Step count unchanged — `grep -c 'uses:'` → ci.yml **67**, docs.yml **5**
- [x] `release.yml` untouched and internally consistent — checkout@v4 **23**, upload-artifact@v4
    **11**, download-artifact@v4 **20**
- [x] `.pre-commit-config.yaml` untouched — `rev: v6.0.0` **1**, `rev: 1.0.0` **1**
- [x] `mise run check` exit 0 — all 15 hooks pass, no file left rewritten (`git status --porcelain`
    shows only runner-owned `iterations.jsonl`)
- [x] CI green on the pushed develop sha —
    `gh api "repos/iscc/iscc-lib/commits/8f76d48/check-runs?per_page=100"` → **41 check-runs, 0
    non-success**, including the enforcing `Perf (iai-callgrind)`, `Coverage + CRAP`, and
    `Audit (cargo-deny)` gates

**Independent checks I ran beyond next.md:**

- **`setup-uv` floating-tag claim confirmed** — `git/matching-refs/tags/v` returns v1–v7 floating
    majors, then only exact `v8.0.0…v8.3.2, v9.0.0`. `@v9` genuinely does not exist; `@v9.0.0` is
    the current latest release. The advance agent's deviation is correct, not a workaround.
- **Every bumped ref is the true current major** — checkout v7.0.1, setup-python v7.0.0, setup-node
    v7.0.0, setup-java v5.6.0, setup-go v7.0.0, setup-dotnet v6.0.0, upload-artifact v7.0.1,
    upload-pages-artifact v5.0.0, deploy-pages v5.0.0, codeql-action `v4` (floating tag exists).
- **Nothing was left behind** — the full `uses:` inventory of both files (16 distinct refs) is now
    on its latest major line, including the five deliberately-untouched ones
    (`rust-toolchain@stable`, `rust-cache@v2`, `install-action@v2`, `setup-ruby@v1`,
    `cargo-semver-checks-action@v2`, all re-confirmed current today).
- **Gate integrity** — scanned the whole advance range `c8e18a0..HEAD` for suppressions, skips,
    threshold reductions, hook weakening, and scope exclusions: none. `continue-on-error: true` on
    the `semver` job is pre-existing and untouched.
- **Scope** — 2 non-test, non-doc files, inside the 3-file cap. Every `## Not In Scope` item
    (release.yml, `.pre-commit-config.yaml`, SHA pinning, dependabot/renovate, new jobs/steps/
    permissions, runtime versions) verified untouched.

**Issues found:**

- (none blocking) — one process note: the advance agent pushed `ebac57f` with the unresolvable
    `@v9`, reddening CI on develop (and on the open develop→main PR) before `8f76d48` fixed it. The
    root cause was next.md's recommendation, which trusted `releases/latest` as proof that a
    floating `@vN` tag exists. Captured in learnings.md so define-next verifies with
    `git/matching-refs/tags/v<N>` next time.

**Codex review:** One P2 finding — the previous handoff documented the CI-evidence command with an
ellipsis in the SHA (`commits/8f76d48.../check-runs`), which returns HTTP 422 and is therefore not
reproducible. Valid; it affected only handoff prose (the CI result itself was real, as I
re-verified). Fixed by this rewrite, which records the exact command with the full-resolvable sha.
No findings against the workflow diff itself.

**Next:** Dependency refresh slice 5 — **JVM binding manifests**: `crates/iscc-jni/java/pom.xml` +
`packages/kotlin/build.gradle.kts` (2 files, both fully exercised by the
`Java (JNI build, mvn test)` and `Kotlin (gradle build, test)` CI jobs on every develop push, so
unlike release.yml this slice is genuinely verifiable). Concrete pins to evaluate: junit-jupiter
5.11.4, gson 2.11.0, maven-compiler-plugin 3.13.0, maven-surefire-plugin 3.5.2 (shared by both files
in the junit/gson case — keep the versions in lockstep), Kotlin JVM plugin 2.1.10, JNA 5.16.0. Two
cautions for define-next: (a) the `maven-source-plugin` / `maven-javadoc-plugin` /
`maven-gpg-plugin` / `central-publishing-maven-plugin` pins live in a release-only profile and are
**not** CI-exercised — either exclude them or flag them as statically-verified-only; (b)
`java-version: '17'` and the Kotlin jvmToolchain are support-policy decisions, not dependency pins —
out of scope, same as last slice.

Deferred (do not pick yet): `release.yml` GHA refs — 97 `uses:`, `upload-artifact@v4` ↔
`download-artifact@v4` must move as a pair, `setup-uv` there also needs `@v9.0.0`, and nothing in it
is exercised by a CID push. It is worth bundling with the existing `normal` `[human]` issue "Fix
broken single-registry re-trigger in release.yml" so release.yml is opened once, but that bundle
should be a deliberate human-timed step near the next release, not an autonomous slice.

**Notes:**

- **`@v9.0.0` is a maintenance debt, deliberately taken.** setup-uv patch releases will not be
    picked up automatically; the pin must be hand-bumped each refresh pass, and the same exact-tag
    rule applies to its occurrence in `release.yml`. Rationale + rejected alternatives recorded in
    `decisions.md` (2026-07-25).
- **`docs.yml` is statically verified only** — it triggers on push to `main`, so its five bumps
    (notably `upload-pages-artifact@v5` + `deploy-pages@v5`) get their first real exercise on the
    next develop→main merge. If the docs deploy breaks after that merge, this slice is the first
    suspect.
- **CI run bookkeeping:** `ci.yml` has `cancel-in-progress: true` per ref, so pushing a follow-up
    commit cancels the previous sha's in-flight run — don't push again while a Done-When depends on
    a specific sha going green. Each develop commit also triggers two runs (push + `pull_request`
    from the open develop→main PR), which is why totals read 41 rather than ~20.
- **Gate maintenance:** no gate change needed this iteration. Worth noting that no check anywhere
    validates that a workflow's action refs actually resolve — CI is the only detector, and it costs
    a full round-trip. `actionlint` would not catch this class either (tag existence needs network),
    so the practical mitigation stays the `git/matching-refs` pre-check now in learnings.md rather
    than a new hook.
- **learnings.md pruned** to 198 lines: the fully-met `## Feature Flags` section moved to
    `learnings-archive.md` with a pointer left in place.
