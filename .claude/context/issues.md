# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files.

<!-- Add issues below this line -->

## `packages/go` lowercases without `Final_Sigma` context — Greek text non-conformant `critical` [human]

**Titusz 2026-07-26: fix this next, ahead of the freeze-rule work below.** It is a wrong-output bug
on ordinary text in a package already published as v0.5.0, the fix is small and needs no new
dependency, and landing it unblocks the `Final_Sigma` boundary vector for Go. Ships in v0.6.0; no
separate patch release.

`packages/go/utils.go:117` collapses text with `strings.ToLower`, which applies unconditional simple
case mapping. The reference lowercases with Python `str.lower()` and the Rust core with
`str::to_lowercase()`; both apply the conditional `Final_Sigma` special case (`Σ` → `ς` when
preceded by a cased character and not followed by one). Go therefore emits `σ` where the reference
emits `ς`.

Verified 2026-07-26 by running `TextCollapse` against `iscc_lib.text_collapse` (Rust core, which
agrees with `iscc-core` here):

| input   | reference / Rust | `packages/go` | note                           |
| ------- | ---------------- | ------------- | ------------------------------ |
| `ΑΣΒ`   | `ασβ`            | `ασβ`         | Σ followed by cased Β — agrees |
| `ΑΣ`    | **`ας`**         | **`ασ`**      | word-final Σ — diverges        |
| `ΛΟΓΟΣ` | **`λογος`**      | **`λογοσ`**   | ordinary Greek word            |

Unlike the Unicode-16 table issue this is **not** bounded by version drift or rare code points:
Greek word-final sigma is pervasive (`-ος`, `-ης`, `-ας` endings), so any uppercase or mixed-case
Greek text yields a different Text-Code and Meta-Code from Go than from every other implementation.
The vendored conformance vectors contain no Greek, which is why CI never caught it.

**Fix:** `golang.org/x/text` is already a dependency, and `cases.Lower(language.Und)` implements the
conditional mapping. Verified equivalent to the reference on all five probe inputs above plus
`ΑΣ\u{0378}Β` → `ας\u{0378}β`. Replace `strings.ToLower(norm.NFD.String(text))` in `TextCollapse`
with a package-level `cases.Caser` (construct once — `cases.Lower` allocates a transformer per
call), and add Greek regression cases to `utils_test.go`. Audit `TextClean` and the codec helpers
for other `strings.ToLower` uses at the same time; `codec_test.go:390` is test-only and fine.

**Blocks:** the `Final_Sigma` boundary vector in the freeze-rule work package below cannot be
enabled for Go until this lands. It is *not* covered by the go1.27 exception — `U+0378` is
unassigned in Go's Unicode 15.0 tables too, so that vector is table-independent and this bug is its
only blocker.

**Spec:** `.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance
contract" → "Case mapping must be context-sensitive in every implementation"

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
green). The Kotlin plugin bump raised the consumer Kotlin floor to **Kotlin 2.3 or newer**; that
follow-up is closed — the floor is documented in the root README, `packages/kotlin/README.md` and
`docs/howto/kotlin.md` (iter 132) and specified in `specs/kotlin-bindings.md`. ✅ Slice 6 — Go module
(iter 129; `golang.org/x/text` 0.34.0 → 0.40.0 direct, `github.com/klauspost/cpuid/v2` 2.0.12 →
2.4.0 indirect, new indirect `golang.org/x/sys` v0.47.0 pulled in by cpuid; `go 1.26.1` consumer
floor and `zeebo/blake3` v0.2.4 untouched — all three are the latest published versions; no
hold-back needed). The `x/text` bump was proven **output-neutral**, not just vector-green:
`TextClean`/`TextCollapse` are byte-identical across all 1,112,032 code points under 0.34.0 and
0.40.0 (the `unicode/norm` tables files are unchanged between the two releases; only invalid-rune
bookkeeping was refactored). Also wired `packages/kotlin/README.md` into `scripts/version_sync.py`
`TARGETS` (21 targets now), closing the last unmanaged stale version string (`0.3.1` → `0.5.0`). ✅
Slice 7 — Ruby manifests (iter 130; `crates/iscc-rb/Gemfile.lock` refreshed by `bundle update`: rake
13.4.2, standard 1.56.0, rubocop 1.88.2, rubocop-minitest 0.40.0 + transitives — each verified to be
the newest published version; zero `standardrb` fallout, 111 tests green, `bundle outdated --strict`
clean). `rb_sys` tightened from `~> 0.9` to an **exact** `0.9.123` in the `Gemfile` under a
`# held:` comment, plus a second `# held:` for `minitest ~> 5.0` — both hold-backs verified from
registry metadata this review (`gem specification rb_sys -v <v> --remote`: 0.9.123 →
`rake-compiler-dock = 1.10.0`, 0.9.124 → 1.11.0, 0.9.128 → 1.12.0, so the pin genuinely guards
`tag: 0.9.123` of `oxidize-rb/actions/cross-gem` in `release.yml`; rubygems v1 API: minitest 6.0.x
requires Ruby 3.2 or newer, above the gem's declared floor of 3.1.0).
`crates/iscc-rb/iscc-lib.gemspec` needed no change — it declares no dev dependencies, only the
human-owned `required_ruby_version`. **This closes every locally-verifiable ecosystem slice.** 🔄
Slice 8 (ruff 0.16 adoption) — **sub-slice A done** (iter 131): the 78 config-free findings cleared
— 36 lone `...` stub bodies deleted from `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`
(`PIE790`+`PYI048` double-report the same line) and 6 `RUF059` unused unpackings `_`-prefixed in
`tests/test_new_symbols.py`. Docstring-only stub bodies verified downstream-safe against `ty`,
`mypy 1.18 --strict` and `pyright 1.1.407` (the wheel ships `py.typed` alongside the stub). Baseline
104 → 26. ✅ **Sub-slice B done** (iter 134): `[tool.ruff.lint] extend-select = ["S", "C901"]` added
to `pyproject.toml`, so `uv run ruff check` (pre-commit, `mise run lint`, CI) enforces the security
and complexity gates for the first time and the 14 load-bearing `# noqa: S603/S607` in
`tools/`/`scripts/` are *recognised* instead of reported unused. Two genuinely dead directives were
deleted: the `S603` on the static-argv `subprocess.run` in `tools/metrics.py` `git_sha()` (verified
dead under **both** 0.15.22 and 0.16.0 via `--select S --ignore-noqa` — so the "0.16 refined S603"
framing in the advance handoff is inaccurate; the directive was simply invisible while `RUF100` was
unselected) and a stale `PLC0415` in `tools/cid.py`. The two pre-push `--select S` / `--select C901`
hooks were deliberately kept as redundancy. All 15 `RUF100` cleared; **12 findings left**: `I001` 8
\+ `RUF022` 1 (isort src-root decision — sub-slice C), `RUF007` 1
(`scripts/gen_unicode16_unassigned.py`), `PLW1510` 1 (`scripts/test_install.py`), `EXE001` 1
(`tools/cid.py` shebang without exec bit). Worth bundling into sub-slice C: add `RUF100` to
`extend-select` as well (green at HEAD today) so stale directives can never accumulate unseen again.
✅ **Sub-slice C done** (iter 135): the isort cluster is cleared.
`[tool.ruff] src = [".", "crates/iscc-py/python"]` makes `iscc_lib` first-party and
`[tool.ruff.lint.isort] combine-as-imports = true` keeps the `_lowlevel` re-export block a single
statement (without it the `__init__.py` fix is a 110-line shatter instead of one deleted blank
line). `extend-select` is now `["S", "C901", "I", "RUF022", "RUF100"]`, so import order, `__all__`
order and stale `# noqa` directives are enforced under the pinned 0.15.22 — and `ruff check --fix`
auto-sorts imports at commit time. The 7 mechanical fixes were applied with the *pinned* ruff and an
explicit `--select I,RUF022` (never a blanket `--fix`); all 14 load-bearing `# noqa: S603/S607`
survive. `uvx ruff@0.16.0 check .` is down to **exactly 3 findings**. Verified in review that
per-file invocation — how the pre-commit hook actually calls ruff, with filenames and no
`--force-exclude` — is a no-op on this tree, so hook-mode and `ruff check .` agree. ✅ **Sub-slice D
done** (iter 136): the last three findings cleared — `RUF007` via `itertools.pairwise` in
`scripts/gen_unicode16_unassigned.py` (generator re-run, `crates/iscc-lib/src/utils/unicode16.rs`
byte-identical), `PLW1510` via an explicit `check=False` in `scripts/test_install.py`
(behaviour-preserving — `check=False` is the `subprocess.run` default and 20 call sites inspect
`result.returncode`), and `EXE001` via the exec bit on `tools/cid.py` (mode `100755` committed with
`git update-index --chmod=+x`; all 10 `mise.toml` tasks invoke it as `uv run tools/cid.py`, so the
bit is additive only, and it is now the only shebang'd tracked `.py` file).
`uvx ruff@0.16.0 check .` exits **0** for the first time. ✅ **Sub-slice E done** (iter 137): the
`ruff<0.16` pin and its `# held:` comment are gone from `pyproject.toml`;
`uv lock --upgrade-package ruff` moved 0.15.22 → 0.16.0 and nothing else moved in the lock. Zero
lint findings and zero reformats at the flip — sub-slices A–D had pre-cleared everything. Known
behaviour change: `ruff format` now also checks Python code blocks in Markdown, so the bare
invocation used by `mise run lint` and CI widened from 25 to 153 files (all 129 tracked `.md` files
already clean). The resulting local/CI gate-parity gap was closed in iter 138 by widening the prek
`ruff-format` hook to `types_or: [python, markdown]` (`ruff-check` stays Python-only — ruff 0.16
formats Markdown fences but does not lint them); one residual `.pyi` hole is tracked separately
below. **Slice 8 CLOSED — ruff 0.16 adoption fully landed.**

Verified already-current and needing no bump: `.pre-commit-config.yaml` (pre-commit-hooks v6.0.0,
mdformat 1.0.0), `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`,
`taiki-e/install-action@v2`, `ruby/setup-ruby@v1`, `obi1kenobi/cargo-semver-checks-action@v2`;
`mise.toml` has no `[tools]` section; `crates/iscc-napi/package.json` (`@napi-rs/cli: ^3` floats
over the 3.x line, covers 3.7.4) and `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`
(`Microsoft.NET.Test.Sdk 17.*`, `xunit 2.*`, `xunit.runner.visualstudio 2.*` wildcards) — both
re-checked iter 129, editing them would be churn.

✅ Slice 9 — `.github/workflows/release.yml` GHA refs (iter 140): the last nine stale refs bumped to
their current floating majors as a pure find/replace of full `uses:` values — `checkout` v4→v7,
`download-artifact` v4→v8 **paired in the same commit with** `upload-artifact` v4→v7, `setup-java`
v4→v5, `setup-node` v4→v7, `setup-dotnet` v4→v6, `setup-python` v5→v7, `softprops/action-gh-release`
v2→v3, `cache` v4→v6 (73 of 97 `uses:` lines; nothing else in the file touched, guard invariant and
artifact wiring byte-identical). The issue text's claim that this file needs `setup-uv@v9.0.0` was
**wrong** — `release.yml` invokes no `uv`/`uvx` command and has no `setup-uv` step. Verification is
static (the file is `workflow_dispatch`-only): review independently re-confirmed all nine tags
resolve, every `with:` key is still a declared `inputs` key in each new major's `action.yml`,
`cache@v6` still declares the `cache-hit` output the workflow reads, all nine are `node24`, and read
every intervening major's release notes — no default change bites here (`setup-node@v5+`
auto-caching needs a `packageManager` field or lockfile, neither of which this repo has;
`checkout@v6+`'s `$RUNNER_TEMP` credential file keeps plain `git push` working;
`download-artifact@v8`'s strict `digest-mismatch: error` is intentionally kept). Rationale +
accepted risk → `decisions.md` 2026-07-25.

**All nine ecosystem/tooling slices are closed.** Remaining under this issue are the major bumps,
**all of which Titusz authorized for CID on 2026-07-26** — one per step, gates green, no bundling:
xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper 8.12.1, JUnit 6.x, plus the `jni` 0.22 and
`magnus` 0.8 migrations (source rewrites in `crates/iscc-jni/src/lib.rs` and
`crates/iscc-rb/src/lib.rs` respectively — take these one crate per step, they are real API
migrations and the riskiest items here; if either turns out to need an upstream-behaviour ruling,
park it and say so rather than guessing). Never run `ruff@0.16 check --fix .` — it deletes
load-bearing `# noqa` directives.

**Known constraint (verified iter 126):** the `proc-macro-error2 v2.0.1` future-incompat warning
(`extern crate proc_macro is private and cannot be re-exported`) emitted on every `cargo test` /
`cargo bench` comes from `iai-callgrind-macros` → `iai-callgrind` (dev-only; trace it with
`cargo tree -i proc-macro-error2 --target all`), **not** from the magnus/rb-sys subtree. No fixed
release exists (`iai-callgrind` 0.16.1 is latest), so it stays a warning until upstream ships a fix
— re-check when bumping `iai-callgrind` (the pin must stay in lockstep with the CI-installed
`iai-callgrind-runner` version).

## Gate-script remainders deliberately deferred at iteration 146 `low` [review]

The two gate blind-spot issues (`--check-action-inputs` hardening, `check_docs_nav.py` comment
blindness) were closed in iteration 146 and verified by real-regression mutation in review. Three
items were deliberately left open, each contingent on something that has not happened yet — CID must
not act on them unprompted:

1. **A zero-resolved run still exits 0.** Iteration 146 made the condition *visible*
    (`action-inputs: resolved <R> of <T> action refs`, verified `resolved 0 of 18` behind a dead
    proxy) but not fatal. Flipping the exit code is a policy change against `decisions.md`
    2026-07-26, which accepted all-skipped-as-green by design and filed the stricter rule as a
    follow-up "if skips are ever observed in practice". **Trigger:** the first `release-workflow`
    CI run whose log shows a non-zero skip count. **Needs Titusz** — it turns third-party
    availability into a merge blocker.
2. **Job-level `uses:` (reusable-workflow calls) is still unscanned** — only `job.steps[*].uses`.
    `release.yml` has none at HEAD; the `owner/repo/.github/workflows/x.yml@ref` form also needs a
    different URL shape than `action_yml_urls` builds. **Trigger:** a reusable workflow is added to
    any workflow file.
3. **The YAML 1.1 boolean fix is one-sided.** Non-string `with:` keys are now skipped, but the
    *metadata* side is not normalized: an action declaring an input literally named `on`/`off`/
    `yes`/`no` parses as `True`/`False`, so a workflow passing the **quoted** `"on"` would still be
    reported undeclared. No such action exists among the 18 refs. **Trigger:** the false positive
    is actually observed.

## Pin `rubygems/configure-rubygems-credentials` off the `@main` branch (RULED) `normal` [human]

**RULED by Titusz 2026-07-26 — take option (a): `@v2.1.0` with an inline `# exact tag:` comment.**
The repo's tags-never-SHAs convention stands; this does not become the first SHA-pinned action.
Rationale in `decisions.md` (2026-07-26). **CID is authorized to make this change** — it is a
one-line edit to `.github/workflows/release.yml` line 895 plus the comment. Verification is static
(the workflow is `workflow_dispatch`-only) plus the first real gem publish. Background below.

`.github/workflows/release.yml` line 895 uses `rubygems/configure-rubygems-credentials@main` — a
**floating branch**, the only unpinned `uses:` in the repo. That step runs in the RubyGems publish
job with `id-token: write` and mints the OIDC credential that pushes the gem, so whatever `main`
resolves to on release day executes with publish authority.

Facts checked at review (iter 140):

- `@main` is what upstream's README shows in **every** example, so the current form is vendor-
    documented, not an oversight. The same README, however, carries a NOTE recommending consumers
    "replace the version ranges in the Actions with specific SHA hashes … [so the workflow] does not
    pick up a new version if any of these Actions were compromised."
- Upstream publishes only exact tags — `v1.0.0`, `v2.0.0`, `v2.1.0`; there is **no floating `v2`**
    (`git/ref/tags/v2` → 404).
- `main` is *ahead of* `v2.1.0` (`6861877…` vs `dc5a8d8…`), so moving to the tag is a small rollback
    to the newest release, not a major bump.
- The step passes **no `with:` keys** here (pure OIDC, no `role-to-assume`/`api-token`), so any of
    `@v2.1.0` / a SHA / `@main` is input-compatible — this is purely a trust-anchor choice.

The two candidate fixes conflicted with each other and with existing convention: (a) `@v2.1.0` + an
inline `# exact tag:` comment, mirroring the `astral-sh/setup-uv@v9.0.0` precedent and keeping the
repo's "tags, never SHAs" convention intact, or (b) a SHA pin as upstream recommends, which would
make this the first SHA-pinned action in the repo and reopens the convention decided in
`decisions.md` 2026-07-25. Left untouched in iteration 140 by that step's `Not In Scope`. **(a) was
chosen** — the deciding argument being that 97 other `uses:` refs are already mutable floating
majors, including the `checkout` and `upload-artifact` steps that build and carry the very artifact
being published, so SHA-pinning this one step buys little while splitting the convention. If SHA
pinning is ever adopted it should be adopted repo-wide.

## Declare and gate a Unicode data version (DECIDED) `normal` [human]

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

Not introduced by the iter-129 `golang.org/x/text` refresh — that bump was verified byte-identical
across all 1,112,032 code points. Pre-existing and previously unnoticed.

**DECIDED by Titusz 2026-07-25 — declare a Unicode version in the spec and gate it with post-15
conformance vectors.** Rationale in `decisions.md` (2026-07-25, "Unicode data version is declared
and gated, not chased").

**RESOLVED 2026-07-25 (interactive session) — the version is 16.0.0, with a freeze rule going
forward.** The measurement step ran interactively (pin-15.1 / pin-16 / pin-17 / freeze-at-15.1 all
compared); Titusz chose the compromise below. Rationale in `decisions.md` (2026-07-25, "Unicode data
version 16.0.0 with a freeze rule").

1. **Declared Unicode data version: 16.0.0** (= CPython 3.14's `unicodedata`). The backward-compat
    glitch is accepted: inputs containing any of the 5,185 code points assigned between 15.1 and
    16.0 (realistically: the 7 emoji added in Unicode 16) hash differently than `iscc-core` on
    CPython ≤ 3.13 produced historically.
2. **Freeze rule:** code points unassigned in Unicode 16.0.0 are removed from the input **before any
    normalization or category lookup** in `text_clean` / `text_collapse`, via a vendored table (731
    ranges covering 819,533 code points, generated from `unicodedata2==16.0.0`). Output becomes
    invariant under all future Unicode table versions in every language — Unicode 17+ characters
    are stripped no matter what the runtime ships, and the 16→17 normalization drift (measured:
    exactly 1 code point) is moot because removal precedes normalization. Table dependencies become
    freely upgradable; the differential sweep remains the gate on every bump.
3. **Runtimes with tables older than 16.0 need real 16.0 tables** (the freeze rule cannot add
    knowledge the runtime lacks). Rust core: current deps already suffice — categories are 16.0,
    and the 17.0 normalization tables are output-equivalent to 16.0 once the freeze rule runs first
    — so **no dependency change, only the freeze filter**. Go package: blocked on go1.27 (~Aug
    2026, Unicode 17 in stdlib and x/text); until then Go is on 15.0 tables and will fail the new
    boundary vectors — either vendor the 15.0→16.0 assigned-delta (5,813 code points with their
    16.0 categories) or skip the boundary vectors for Go with a tracking note.
4. **Boundary conformance vectors** (Rust suite + every binding): a 15.1→16 emoji that must be
    retained (e.g. U+1FAE9), a Unicode-16 character with a canonical decomposition (one of the 56
    normalization-drift code points, e.g. U+113C5), and a post-16.0 character the freeze rule must
    strip (e.g. U+20C1 SAUDI RIYAL SIGN, assigned in Unicode 17).

**Implementation order for CID:** ✅ (a1) vendored unassigned-ranges table (731 ranges, checked-in
PEP 723 generator `scripts/gen_unicode16_unassigned.py`) + freeze filter in the Rust core — done
iter 133; `text_clean`/`text_collapse` strip before normalization, 5 boundary/invariant tests,
regeneration is a no-op diff, CRAP + iai gates green. 🔄 (a2) the full-code-space differential sweep
proving equivalence to uniform Unicode 16.0 tables — see the sequence caveat in the issue below,
which must be settled first. 🔄 (b) boundary vectors wired into the Rust suite and all bindings (Go:
see caveat in point 3); the spec already names 16.0.0 and the freeze rule. **(b) Rust half done iter
141**: `crates/iscc-lib/tests/unicode_boundary.json` (ASCII-escaped, `data.json`-shaped, 4 single
code points × `text_clean`/`text_collapse`) + loader `tests/test_unicode_boundary.rs` (1 ungated
shape/content guard + 2 `text-processing`-gated vector tests). Sequence vectors deliberately
excluded pending the ordering ruling; rationale → `decisions.md` 2026-07-25. **Remaining for (b):**
copy the fixture into the 11 bindings' conformance tests and the four sibling `data.json` locations.
**Both blockers are now cleared** (2026-07-26): the ordering ruling is recorded above, and the Go
decision below. Sequence vectors can now be added — the sentinel map makes iscc-lib match the
reference on them. Do the sentinel conversion **first**, since it changes the expected output of any
sequence vector.

**Go — RULED by Titusz 2026-07-26: skip, do not vendor the delta.** `packages/go` skips the Unicode
16.0 boundary vectors with an explicit tracking note (and an issue filed here) rather than vendoring
the 5,813-code-point 15.0→16.0 assigned delta, because go1.27 lands ~Aug 2026 and would make that
table throwaway code. When go1.27 ships, Go needs **both** the newer stdlib/`x/text` tables *and*
the 731-range freeze table — go1.27 fixes Go's missing 16.0 knowledge, not its need to remove
post-16.0 characters. Rationale in `decisions.md` (2026-07-26, "Go skips the Unicode-16 boundary
vectors until go1.27"). This unblocks criterion 3 for the other 10 bindings. ✅ (c) user-facing
documentation — `docs/unicode.md` "Text Processing and Unicode" (iter 143; site nav +
`ORDERED_PAGES` + `llms.txt`, plus a Unicode-tables note in `docs/howto/go.md`) states the declared
version, the freeze rule, the four boundary vectors and both divergences. Keep it in sync when the
Go decision or the ordering ruling lands.

**Upstream:** iscc/iscc-core — filed 2026-07-25 as <https://github.com/iscc/iscc-core/issues/137>
("text_clean/text_collapse output depends on the CPython version"). Reproduced there with
`iscc-core` 1.3.0 itself: CPython 3.13 gives `ISCC:AAARDZ4ASOMVXBRR` / `ISCC:EAA7VW5ZOQZ3XEMT`,
CPython 3.14 gives `ISCC:AAARDZ5SS6NVXBLT` / `ISCC:EAA3RXNBOM77TGM5` for the same input — the 3.14
pair is what the Rust core produces. The fix to propose upstream is the same architecture:
`unicodedata2==16.0.0; python_version < '3.14'` + import shim (wheels cover cp39–cp313, the whole
supported range), the freeze-at-16 pre-filter (pure Python, same 731 vendored ranges), and boundary
vectors in `data.json`. Per ISO 24138 Annex D the reference implementation is normative, so the
`iscc-core` release adopting this settles the standard's answer.

**Spec:** `.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance
contract"

## Convert the freeze rule from a pre-filter to a sentinel map (RULED) `normal` [human]

**RULED by Titusz 2026-07-26 — the freeze rule maps unassigned code points to the noncharacter
`U+FFFF` before normalization; ISO conformance is a hard constraint; the declared version stays
16.0.0.** Rationale in `decisions.md` (2026-07-26, "The freeze rule maps unassigned code points to a
noncharacter sentinel" + "Declared Unicode version stays 16.0.0"). `specs/rust-core.md` requirements
1 and 4, its accepted-divergence section and its **Verified when** list are already updated — the
spec is the authority; this entry is the work package.

Two intermediate proposals from the same session were superseded and must not be implemented: a
category override (fails 10 of 140 sequence cases — see the table below) and lowering the declared
version to 15.1.0 (rejected: it strips the 6 living scripts added in Unicode 16.0, collapsing any
document written wholly in one of them to `""` so all such documents collide on one degenerate
Text-Code).

The check Titusz asked for: IEP-0003 (Normative, ISO 24138:2024,
<https://github.com/iscc/iscc-ieps/blob/main/ieps/iep-0003.md>) defines conformance as
output-equivalence with the reference implementation, *not* prose-matching its 5-step Processing
clause. So the iter-133 pre-filter was **not** a wording defect — it was real non-conformance, and
unlike the accepted 15.1→16.0 single-code-point glitch it diverges from **every** reference version
including CPython 3.14. "Accept + enumerate the delta" was therefore rejected.

### Work package

1. In `crates/iscc-lib/src/utils.rs`, change the pre-normalization pass in `text_clean` (~line 100)
    and `text_collapse` (~line 176) from a `filter` to a `map` onto the sentinel — one token,
    inside the same fused iterator:

    ```rust
    // from:
    .filter(|&c| !is_unassigned_in_unicode16(c))
    // to:
    .map(|c| if is_unassigned_in_unicode16(c) { '\u{FFFF}' } else { c })
    ```

    The category filters (`is_c_category`, `is_cmp_category`) are **left alone** — `U+FFFF` is `Cn`,
    so they already strip it. The vendored table and `scripts/gen_unicode16_unassigned.py` do not
    change. Name the sentinel as a `const` with a comment on why a *noncharacter* specifically.

2. Update both function docstrings — they currently give the pre-normalization ordering as the
    reason for invariance, which is wrong on both counts.

3. Add regression tests for all four **Verified when** cases, including
    `text_clean("e\u{A7F1}\u{0301}") == "e\u{0301}"` (the case a category override gets wrong) and
    an assertion that the normalizer passes `U+FFFF` through unchanged.

4. Both hot-path gates apply (iai-callgrind 10% Ir budget, CI-only CRAP `--fail-regression`). Expect
    Ir to be **flat**: a `map` replaces a `filter` in the same iterator chain — same passes, same
    allocations, no filter-predicate change. A significant move in either direction means the
    implementation is not the one specified.

5. Widen the deliberately-scoped CPython-3.14 sentence in `docs/unicode.md` back to unqualified
    agreement, replace its freeze-rule description with the sentinel mechanism, and state the
    accepted divergence class (b) plus why 15.1.0 was rejected. The placeholder recorded in
    `decisions.md` 2026-07-26 is discharged.

6. Add a short **"How much does this matter?"** section to `docs/unicode.md` giving readers the
    proportionality up front, so the page does not read more alarming than the issue is: Data-Code
    and Instance-Code are unaffected; Meta-Code and Text-Code are similarity-preserving so affected
    codes stay Hamming-close and similarity matching still works; newly assigned code points are
    rare in real text; the residual exposure is exact-match lookups on short inputs and the exact
    `name` / `description` fields. Source: `decisions.md` 2026-07-26, "Unicode determinism is a
    bounded-severity issue".

**Cross-binding prerequisite:** when the boundary vectors are later wired into the bindings, the
`Final_Sigma` case is blocked for `packages/go` by the `strings.ToLower` bug filed at the top of
this file — a separate defect from the go1.27 table exception, and one that affects ordinary Greek
text. Land that fix first or enable that one vector for Go last.

**Measured before the ruling** — three designs swept against a uniform-Unicode-16.0 reference
(`unicodedata2==16.0.0`), evaluated on Unicode 17.0 tables (what `unicode-normalization` 0.1.25
ships):

| design                         | single code points | sequence cases |
| ------------------------------ | ------------------ | -------------- |
| pre-filter (iteration 133)     | 0 of 1,114,112     | **42 of 140**  |
| category override (superseded) | **1** (`U+A7F1`)   | **10 of 140**  |
| sentinel map (decided)         | **0**              | **0**          |

The `1,114,112` denominator is what that Python harness could reach, since a Python `str` can hold
surrogates. Do **not** carry it into the Rust sweep: `&str` cannot, so the criterion-4 denominator
is the **1,112,064 Unicode scalar values** (see `specs/rust-core.md` requirement 4).

The pre-filter scoring 0 on single code points while failing 42 sequence cases is exactly the
false-assurance failure criterion 4 now forbids. The override's failure is worse than its count:
`U+A7F1` is unassigned in 16.0 but decomposes to `S` under 17.0, so `e U+A7F1 U+0301` produced `eŚ`
— a Unicode-17 character injecting a spurious letter into the output.

**Follow-up, separate step:** wire the criterion-4 sweep into the repo as a runnable check asserting
**zero** divergence over single code points *and* sequence classes.

**Still to do by a human — not CID:** upstream <https://github.com/iscc/iscc-core/issues/137> must
be updated to propose the sentinel mechanism (map unassigned-in-16.0 to `U+FFFF` before normalizing,
keep the existing category-`C` filter) together with
`unicodedata2==16.0.0; python_version < '3.14'`. Both the earlier sequence-delta framing and the
pre-normalization-removal framing should be withdrawn. Worth stating explicitly upstream:
`iscc-core` output has never been deterministic across its declared `>=3.9,<4.0` range (Unicode
13.0/14.0/15.0/15.1/16.0 by Python version), so this is a determinism fix, and the behavioural break
it implies is accepted deliberately.

**Spec:** `.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance
contract" (requirements 1 and 4)

<details><summary>Original finding (iteration 133 review) — kept for context</summary>

The freeze filter landed in iter 133 exactly as specified (strip Unicode-16.0.0-unassigned code
points **before** normalization). Stripping before normalization changes character **adjacency**, so
it enables contextual transforms that `iscc-core` — which removes the same code points *after* NFKC,
via the category-`C` filter — still blocks. Verified in this review against reference semantics
(divergences did **not** exist before iter 133, and they persist even when both sides run identical
Unicode 16.0 data, e.g. CPython 3.14):

| Input                   | iscc-lib (iter 133) | `iscc-core`     | Effect                            |
| ----------------------- | ------------------- | --------------- | --------------------------------- |
| `text_clean("e͸́")`      | `U+00E9`            | `U+0065 U+0301` | canonical composition unblocked   |
| `text_clean("ᄀ͸ᅡ")`     | `U+AC00`            | `U+1100 U+1161` | Hangul jamo composition unblocked |
| `text_collapse("ΑΣ͸Β")` | `α σ β`             | `α ς β`         | `Final_Sigma` context changed     |

So Meta-Code **and** Text-Code can differ for these inputs. Consequences:

1. **Spec criterion 4** ("a full-code-space differential sweep proves the freeze-rule implementation
    output-equivalent to uniform Unicode 16.0.0 tables") is false as worded — a per-code-point
    sweep over all 1,112,032 code points passes and gives false assurance. Either reword it to
    "equivalent for single code points, with the sequence-adjacency delta enumerated and accepted",
    or specify a sequence-aware sweep (base+Cn+mark, jamo+Cn+jamo, Σ+Cn+cased at minimum).
2. **The accepted-divergence paragraph** in `specs/rust-core.md` currently covers only "runtimes
    with non-16.0 tables" and "characters assigned between 15.1 and 16.0". This class is neither —
    it should be named explicitly if it is accepted.
3. **The upstream proposal** (<https://github.com/iscc/iscc-core/issues/137>) must specify
    *pre-normalization* removal, or an `iscc-core` that adopts the freeze rule with post-
    normalization removal will still disagree with iscc-lib on these inputs.
4. **The public docs page carries a placeholder qualifier** (added iter 143): `docs/unicode.md` says
    CPython 3.14 "agrees with iscc-lib on the single-code-point behaviour described on this page"
    and is deliberately silent about sequences. The draft claimed unqualified agreement — false for
    this class — so the ruling must revisit that sentence (widen it back, or state the accepted
    sequence delta). Rationale → `decisions.md` 2026-07-26.

No alternative ordering preserves table-version invariance (post-normalization removal reintroduces
the 16→17 drift the rule exists to prevent), so this is expected to be a wording/scoping decision
rather than a redesign. Settle it **before** step (b) wires boundary vectors into 11 bindings and 5
`data.json` copies — expected outputs would otherwise be re-derived twice.

</details>

*(The closing premise above was **wrong**, and so was the conclusion drawn from it. The choice is
not between orderings, so nothing is traded: mapping to the `U+FFFF` sentinel keeps removal exactly
where the reference does it, preserving table-version invariance **and** exact conformance at the
same time. The acceptance criterion is therefore **zero** divergence — there is no residual to
measure or accept — and this was a redesign, not the wording/scoping decision the premise
predicted.)*

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

## Make the CI job table in `specs/ci-cd.md` exhaustive `normal` [human]

The CI job table in `.claude/context/specs/ci-cd.md` has **14 rows against 21 real jobs** in
`.github/workflows/ci.yml` (22 check names, verified against the check-runs API at iteration 145).
The review agent carried this as a note for three iterations because the review protocol forbids
spec edits without a `[human]`-sourced issue — this entry is that authorization.

**Titusz 2026-07-26: make it exhaustive.** The table should list every job, so a reader can use it
to verify CI coverage instead of guessing whether an omission is deliberate. Add the missing rows
and, if the drift is likely to recur, consider whether `scripts/check_docs_nav.py`-style gating is
warranted — but that is a judgment call for the step, not a requirement.

**Spec:** `.claude/context/specs/ci-cd.md` → CI job table

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.

## Migrate npm publishing to OIDC Trusted Publishing `low` [human]

**DEFERRED by Titusz 2026-07-26 — not for v0.6.0.** The v0.6.0 release stays on `NPM_TOKEN`, which
was rotated and is valid until 2026-09-16. Downgraded to `low` so CID skips it; revisit before the
release after v0.6.0, or sooner if the token expiry becomes a problem again. CID must **not**
prepare the YAML diff — an unused OIDC-shaped diff sitting in the tree would be its own hazard.

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
