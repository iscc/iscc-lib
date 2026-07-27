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
2. **Freeze rule (as revised 2026-07-26 and implemented iter 148):** code points unassigned in
    Unicode 16.0.0 are **replaced by the noncharacter sentinel `U+FFFF`** before normalization in
    `text_clean` / `text_collapse`, via a vendored table (731 ranges covering 819,533 code points,
    generated from `unicodedata2==16.0.0`); the unchanged category-`C` filter then removes the
    sentinel exactly where the reference removes unassigned code points. Output becomes invariant
    under all future Unicode table versions in every language (`U+FFFF` is permanently `Cn`,
    `ccc = 0`, undecomposable) **and** conformant on multi-code-point sequences. Table dependencies
    become freely upgradable; the differential sweep remains the gate on every bump. *(The original
    wording of this point — removal **before** normalization — was the iteration-133 design; it was
    measured non-conformant on 42 of 140 sequence cases and superseded. See `decisions.md`
    2026-07-26, "The freeze rule maps unassigned code points to a noncharacter sentinel".)*
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
PEP 723 generator `scripts/gen_unicode16_unassigned.py`) + the freeze rule in the Rust core — table
and filter done iter 133, **converted to the ruled `U+FFFF` sentinel map iter 148**
(`UNASSIGNED_SENTINEL` const + `map` at both call sites, all docstrings/comments corrected, 6
regression tests, `.crap-baseline.json` refreshed, iai flat at −3.9% Ir; the vendored table and the
generator's output are byte-identical throughout). ✅ **(a2) DONE (iter 157) — the criterion-4
differential sweep is a permanent, fail-closed gate.** `scripts/unicode_sweep.py` compares
`text_clean` / `text_collapse` against the installed `iscc-core` on CPython 3.14 (uniform Unicode
16.0.0 tables, no freeze rule of its own, so post-16.0 code points are `Cn` to it exactly as the
sentinel makes them for us) over 1,112,064 scalars × 8 contexts × 2 functions = **17,793,024
comparisons, 0 divergences** (~60 s). It fails closed on the oracle's `unidata_version`, a stale
extension, the scalar count and the comparison count — asserting both counts *before* the success
line `TOTAL 17793024 comparisons, 0 divergences`, so a zero-case run cannot read green. Run it as
`mise run unicode:sweep` (rebuild-then-sweep) or via the standalone `unicode-sweep` CI job (21st
job, CPython 3.14 + `--release`; standalone because the `python-test` 3.10 leg could only skip). Ten
pytest cases in `tests/test_unicode_sweep.py` pin the machinery, including a review-added guard that
the eight contexts genuinely discriminate the superseded delete-filter design — only `base_mark` /
`jamo` / `sigma` do, so an eight-context set swapped for ASCII shapes would keep the arithmetic pin
green while exposing nothing. The sequence half of the criterion is covered for *every* scalar, not
only the unassigned ones. **Residual the gate protects:** only the *conditional* `Final_Sigma`
mapping is frozen; every unconditional lowercase mapping still comes from rustc's tables (zero
divergence measured today — see `decisions.md` 2026-07-27). 🔄 (b) boundary vectors wired into the
Rust suite and all bindings (Go: see caveat in point 3); the spec already names 16.0.0 and the
freeze rule. **(b) Rust half done iter 141**: `crates/iscc-lib/tests/unicode_boundary.json`
(ASCII-escaped, `data.json`-shaped, 4 single code points × `text_clean`/`text_collapse`) + loader
`tests/test_unicode_boundary.rs` (1 ungated shape/content guard + 2 `text-processing`-gated vector
tests). ✅ **Sequence vectors done iter 149**: the four multi-code-point cases below are in the
fixture (`text_clean` 7 cases, `text_collapse` 5) and pinned in the test source by a
`SEQUENCE_VECTORS` const plus a second **ungated** guard (`test_boundary_fixture_sequence_vectors`)
asserting exactly-one-match, expected-equality and delete-filter-inequality per row; review
mutation-probed all four failure modes (wrong output, dropped case, swapped code point, duplicated
case) and each one reds the suite with `text-processing` off. Documented in `docs/unicode.md`.

| section         | input                      | expected             | delete filter gives  |
| --------------- | -------------------------- | -------------------- | -------------------- |
| `text_clean`    | `e\u0378\u0301`            | `e\u0301`            | `\u00E9`             |
| `text_clean`    | `\u1100\u0378\u1161`       | `\u1100\u1161`       | `\uAC00`             |
| `text_clean`    | `e\uA7F1\u0301`            | `e\u0301`            | `\u00E9`             |
| `text_collapse` | `\u0391\u03A3\u0378\u0392` | `\u03B1\u03C2\u03B2` | `\u03B1\u03C3\u03B2` |

**Do not relabel row 3 as `e\u015A`** — that value (this table's earlier "must NOT be" wording,
which iteration 149 propagated verbatim into code and docs before review caught it) is what the
superseded *category-override* design yields, because `U+A7F1` is `<super> 0053` under Unicode 17
tables. A *delete filter* removes the code point before normalization, so row 3 collapses to the
same `\u00E9` as row 1. Both wrong values differ from the expected output, so the guard holds either
way.

**Remaining for (b):** copy the fixture into the 11 bindings' conformance tests and the four sibling
`data.json` locations. **Every new tracked copy must be registered in the `VENDORED_COPIES` table of
`tests/test_vendored_fixtures.py`** (byte-identity gate landed iter 152) and must keep the canonical
basename `data.json` / `unicode_boundary.json` — the gate discovers copies by basename, so a copy
renamed to anything else is invisible to it. All blockers are cleared: the sentinel conversion has
landed (it determined the expected outputs above), the Go ruling is below, and the Go `Final_Sigma`
bug that blocked the `Final_Sigma` vector was fixed in iter 147 (`packages/go/utils.go` now uses
`cases.Lower(language.Und)`; the per-call `cases.Caser` is deliberate — do not hoist it). ✅
**Propagation slice 1 done (iter 150): Python + pure-Go.** `tests/test_unicode_boundary.py` reads
the canonical fixture by relative path (12 vectors + a metadata guard, all green);
`packages/go/unicode_boundary_test.go` `//go:embed`s a byte-identical vendored copy at
`packages/go/testdata/unicode_boundary.json` and runs 9 of 12, skipping exactly the 3 ruled
table-dependent cases via a `"<section>/<case>"` skip map whose keys are guarded against fixture
renames. Neither suite carries a `delete_filter_output` oracle and neither needs one — the four
sequence vectors' expected outputs are the decomposed sentinel-design values (`0065 0301`,
`1100 1161`, `03B1 03C2 03B2`), already unequal to the delete-filter results, so plain equality
against `outputs.result` reds on a delete-filter regression (mutation-verified in review). ✅
**Propagation slice 2 done (iter 151): WASM + Ruby.** Both read the **canonical** fixture with no
vendored copy — `crates/iscc-wasm/tests/unicode_boundary.rs` via `include_str!` (3
`#[wasm_bindgen_test]` fns: metadata guard + one loop per section, ungated by the `conformance`
feature; on the host target it compiles to 0 tests, `wasm-pack test --node` is the only runner) and
`crates/iscc-rb/test/test_unicode_boundary.rb` via `File.expand_path` + `define_method` per case
(fresh `BOUNDARY_JSON`/`BOUNDARY_DATA` constants — `rake test` loads all test files into one
process). All 12 vectors green on both with **zero skips**; CI runs both jobs and compiles the Ruby
extension first, so the local stale-`.so` hazard cannot reach CI. Review mutation-probed both: a
delete-filter-shaped expected value reds the named vector, a deleted case reds the metadata guard. ✅
**Propagation slice 3 done (iter 153): napi + JNI/Java.** Both read the **canonical** fixture with
no vendored copy — `crates/iscc-napi/__tests__/unicode_boundary.test.mjs` via `readFileSync` on
`join(__dirname, '..', '..', 'iscc-lib', 'tests', 'unicode_boundary.json')` (metadata guard + one
`it` per case per section) and
`crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/UnicodeBoundaryTest.java` via a gson
`@BeforeAll` on `Path.of("../../iscc-lib/tests/unicode_boundary.json")` (surefire's working
directory is the maven basedir, so the same relative shape `IsccLibTest` already uses for
`data.json` resolves even from a repo-root `mvn -f …/pom.xml`). 13 tests each, all 12 vectors,
**zero skips**; both ride the existing CI jobs unchanged (`npm test` globs `__tests__/*.test.mjs`
after `npx napi build --platform`; surefire auto-discovers `*Test.java` after
`cargo build -p iscc-jni`), so CI always tests a freshly built native and the local stale-artifact
hazard cannot reach it. Review mutation-probed both the same two ways as slice 2. ✅ **Propagation
slice 4 done (iter 154): C# + Kotlin.** Both read the **canonical** fixture with no vendored copy —
`packages/dotnet/Iscc.Lib.Tests/UnicodeBoundaryTests.cs` via a csproj
`<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json" Link="testdata\unicode_boundary.json">`
item (`Lazy<JsonElement>` + `[Fact]` metadata guard + two `[Theory]`/`[MemberData]` methods; xunit
puts the case name in the failed-test display name) and
`packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/UnicodeBoundaryTest.kt` via an `iscc.fixtureDir`
system property set in `build.gradle.kts` (gson + `@TestFactory`/`DynamicTest` so each vector is its
own `testcase` in the Gradle XML). 13 tests each, all 12 vectors, **zero skips**; both ride the
existing CI jobs unchanged. Review mutation-probed both the same two ways as slices 2/3. Review also
added `inputs.file(…).withPathSensitivity(PathSensitivity.NONE)` to the Kotlin `Test` task — without
it a post-run fixture edit left `./gradlew test` `UP-TO-DATE` and silently skipped all 13 boundary
tests (CI was never affected: fresh checkout, no build-dir cache). **3 surfaces left:** C FFI
(`tests/test_iscc.c` has no JSON reader and no text-function coverage at all — needs a generated
vector table or a hand-rolled reader), C++ (not buildable in this container — no `cmake`), Swift (no
toolchain in this container; it is the one that would add a tracked vendored copy, which **must** be
registered in `VENDORED_COPIES`), plus the four sibling `data.json` copies. (The running tally
counts the 11 native bindings named in `docs/unicode.md`; `packages/go` is the separate pure-Go port
and UniFFI is the shared mechanism behind Kotlin and Swift, not an independent surface. Note that
`git ls-files | grep unicode_boundary` under-counts the tally — the Java/C#/Kotlin suites are named
`UnicodeBoundaryTest.java`, `UnicodeBoundaryTests.cs` and `UnicodeBoundaryTest.kt`.)

**Go — RULED by Titusz 2026-07-26: skip, do not vendor the delta.** `packages/go` skips the Unicode
16.0 boundary vectors with an explicit tracking note (and an issue filed here) rather than vendoring
the 5,813-code-point 15.0→16.0 assigned delta, because go1.27 lands ~Aug 2026 and would make that
table throwaway code. When go1.27 ships, Go needs **both** the newer stdlib/`x/text` tables *and*
the 731-range freeze table — go1.27 fixes Go's missing 16.0 knowledge, not its need to remove
post-16.0 characters. Rationale in `decisions.md` (2026-07-26, "Go skips the Unicode-16 boundary
vectors until go1.27"). This unblocks criterion 3 for the other 10 bindings.

**go1.27 bump checklist (raised by the iter-150 Codex review, confirmed in review):** bumping
`packages/go/go.mod` past `go 1.26.1` **reds the Go boundary suite** and must land the freeze table
in `packages/go/utils.go` in the *same* step. Go passes the two post-16.0 vectors today for the
wrong reason: its Unicode 15.0 tables classify `U+20C1` and `U+A7F1` as `Cn`, so the category-`C`
filter drops them — coinciding with the freeze rule's output by accident. Under go1.27 both become
assigned (`x/text`'s `tables17.0.0.go` is `//go:build go1.27`, verified in the module cache; the
stdlib `unicode` tables move too), so 5 currently-green cases flip red (`text_clean` +
`text_collapse` for `U+20C1` and `U+A7F1`, plus `text_clean/…_seq_ua7f1_no_decomposition_leak`,
which yields `e U+015A`) while the 3 skips become unnecessary. Do **not** version-gate the skip map
to hide this — the red is the intended signal, and `go-version-file: packages/go/go.mod` pins CI to
the go.mod directive so nothing flips without a deliberate bump.

✅ (c) user-facing documentation — `docs/unicode.md` "Text Processing and Unicode" (iter 143; site
nav + `ORDERED_PAGES` + `llms.txt`, plus a Unicode-tables note in `docs/howto/go.md`) states the
declared version, the freeze rule, the four boundary vectors and both divergences. Keep it in sync
when the Go decision or the ordering ruling lands.

**Upstream:** iscc/iscc-core — filed 2026-07-25 as <https://github.com/iscc/iscc-core/issues/137>
("text_clean/text_collapse output depends on the CPython version"). Reproduced there with
`iscc-core` 1.3.0 itself: CPython 3.13 gives `ISCC:AAARDZ4ASOMVXBRR` / `ISCC:EAA7VW5ZOQZ3XEMT`,
CPython 3.14 gives `ISCC:AAARDZ5SS6NVXBLT` / `ISCC:EAA3RXNBOM77TGM5` for the same input — the 3.14
pair is what the Rust core produces. The fix to propose upstream is the same architecture:
`unicodedata2==16.0.0; python_version < '3.14'` + import shim (wheels cover cp39–cp313, the whole
supported range), the freeze-at-16 **sentinel map** (pure Python, same 731 vendored ranges — map to
`"￿"` before `unicodedata.normalize`, leave the existing category-`C` filter alone), and boundary
vectors in `data.json`. Per ISO 24138 Annex D the reference implementation is normative, so the
`iscc-core` release adopting this settles the standard's answer.

**Still to do by a human — not CID:** the upstream issue thread must be updated to propose the
**sentinel** mechanism; both the earlier sequence-delta framing and the pre-normalization-removal
framing should be withdrawn. Worth stating explicitly upstream: `iscc-core` output has never been
deterministic across its declared `>=3.9,<4.0` range (Unicode 13.0/14.0/15.0/15.1/16.0 by Python
version), so this is a determinism fix, and the behavioural break it implies is accepted
deliberately.

**Spec:** `.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance
contract"

## Harden the Unicode differential sweep gate `normal` [review]

Two blind spots in `scripts/unicode_sweep.py` (landed iter 157). Both are safe to fix in one step;
neither affects the gate's authoritative CI form, which rebuilds unconditionally.

1. **The freshness guard cannot see a dependency-only or toolchain-only change.**
    `check_extension_fresh` compares the `.so` mtime against the newest `*.rs` under
    `crates/iscc-lib/src` + `crates/iscc-py/src` only. A `cargo update` that moves
    `unicode-normalization` / `unicode-general-category`, or a rustc upgrade, touches no `.rs` file
    — so a bare `uv run scripts/unicode_sweep.py` reports a **false green** from a stale extension.
    That is precisely the upgrade the sweep exists to validate. A missing source directory also
    passes vacuously (`max(..., default=0.0)`, reproduced at review). Options: widen the mtime set
    to `Cargo.toml` / `Cargo.lock` / `rust-toolchain*`, embed `rustc -vV` + a dependency
    fingerprint in the built extension and compare it, or make the direct invocation refuse to run
    outside `mise run unicode:sweep`. Whatever is chosen, keep the fast fail-fast signal — do not
    make the script always shell out to a ~21 s maturin build. Rationale for the current shape and
    the interim operating rule (*always* run through `mise run unicode:sweep`) → `decisions.md`
    2026-07-27.

2. **The divergence list is unbounded.** `sweep()` retains every `Divergence` tuple even though
    `main()` prints at most `MAX_REPORTED_DIVERGENCES`. A broad regression (e.g. a toolchain change
    to a common case mapping) can retain millions of tuples with two strings each — several GB —
    and OOM-kill the CI process before it emits any diagnostics, turning the most informative
    failure into the least. Fix: count divergences separately and retain only the first
    `MAX_REPORTED_DIVERGENCES` samples. `tests/test_unicode_sweep.py`
    `test_sweep_reports_divergence_with_wrong_oracle` asserts `len(divergences) == count` and must
    be updated in the same step.

Both were reported by the iteration-157 Codex review; (1) was independently flagged in the advance
handoff and reproduced at review.

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
