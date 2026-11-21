# decisions.md archive

Entries rotated out of `decisions.md` by the CID runner, oldest first, when the log exceeded its
line budget. Never loaded by an agent — reference for humans only.

## 2026-07-24 — Audit role triggered by iteration cadence, not IDLE

**Decision:** The codebase audit role runs after every 10th completed iteration (plus on demand via
`mise run cid:audit`), rather than on IDLE like meta-improve or on a calendar. **Why:** Debt
accumulates fastest during busy development phases — exactly when IDLE never fires. A counter from
iterations.jsonl survives restarts and needs no clock. Findings land in issues.md right before the
next define-next, so the loop consumes them immediately. **Alternatives:** IDLE-coupling (audits
would pause during the phases that create the debt); manual-only (relies on human discipline, which
decays over months); every 5 iterations (more cost, re-finds issues the loop has not yet had time to
fix). **Context:** Interactive session with Titusz, 2026-07-24; implemented alongside this file.

## 2026-07-24 — Audit is find-and-file only, runner-enforced

**Decision:** The audit role may only change issues.md and its own agent memory; the trusted runner
(`enforce_audit_safety` in tools/cid.py) reverts anything else. Fixes flow through the normal
define-next → advance → review cycle. The audit orchestrates a parallel finder/adversarial-verify
workflow rather than using the generic high-stakes harness. **Why:** Free-form "refactoring
sessions" are where agentic projects introduce regressions — refactoring without a gate is churn.
Audit finds, the gated loop fixes. Adversarial verification (2-of-3 skeptics) keeps false positives
from steering iterations. The high-stakes skill defends a single deliverable; the audit needs
multi-dimension discovery — a different harness shape. **Alternatives:** Letting the audit fix
directly (bypasses the review gate); high-stakes skill per audit (rigor lands in the wrong place,
higher cost). **Context:** Interactive session with Titusz, 2026-07-24.

## 2026-07-24 — Scope-cap escape valve: 8 files, only for audit-cited steps

**Decision:** define-next may scope up to 8 non-test, non-doc files (instead of 3) only when the
step cites an `[audit]`-tagged issue; the review agent verifies the citation and fails the step
otherwise. **Why:** The 3-file cap systematically selects against structural fixes — the fix for an
abstraction is always bigger than a point patch, so cross-cutting cleanups (e.g. across the 8
binding crates) were unpickable by design. A hard ceiling keeps refactor steps reviewable in one
sitting. **Alternatives:** No exception (structural debt stays unpickable); uncapped audit steps
(loses the mechanical tripwire that kept 120 iterations reviewable); pre-decomposing every refactor
into 3-file steps (many refactors cannot be split while keeping every intermediate tree green).
**Context:** Interactive session with Titusz, 2026-07-24.

## 2026-07-24 — Tightening decode input-validation is not a SemVer break

**Decision:** Adding the `tail.len() > nbytes` "too long" rejection to the Tier 1,
stability-committed `iscc_decode` (and, iter 120, Go `IsccDecode`) is classified as a robustness bug
fix, not an API break — no `**API-BREAK:**` flag required, and it need not wait for the v1.0.0 cut.
**Why:** The signature and return-tuple shape are unchanged; only the set of *accepted* inputs
shrinks, and every input newly rejected was already malformed (trailing base32 chars that silently
aliased a canonical code, e.g. `ISCC:...AB` == `ISCC:...ABAA`). Narrowing the accepted domain to
reject previously- mis-decoded garbage is a correctness fix, and every conformance vector still
round-trips. **Alternatives:** Treat as breaking and defer to v1.0.0 (would leave a codec-wide
aliasing bug live across all 11 bindings for months, for no user benefit); keep truncating silently
(data-integrity hazard — two distinct strings decode identically). Note for the eventual
`cargo-semver-checks` enforcing gate: input-domain narrowing is invisible to signature-based semver
tooling, so this call is a human judgment, not a tool result. **Context:** iter 121, commit 044d0cc
(Rust); iter 120 (Go).

## 2026-07-25 — `astral-sh/setup-uv` pinned to the exact tag `@v9.0.0`

**Decision:** Every other GitHub Action in `ci.yml` / `docs.yml` is pinned to a floating major
(`@v7`, `@v5`, …), but `astral-sh/setup-uv` is pinned to the exact release tag `@v9.0.0`, with a
two-line YAML comment above each of the two occurrences explaining why. **Why:** upstream stopped
publishing floating major tags after `v7` — `git/matching-refs/tags/v` lists v1–v7 plus exact
v8.x/v9.0.0 tags only, and setup-uv's own README pins exact refs. `@v9` does not resolve; CI proved
it (`Unable to resolve action astral-sh/setup-uv@v9` on commit ebac57f). The accepted cost is that
setup-uv patch releases are no longer picked up automatically, so this pin must be bumped by hand in
each dependency-refresh pass (it recurs in `release.yml`). **Alternatives:** stay on `@v4` (leaves
the action 5 majors and a node runtime behind, defeating the refresh); pin to `@v7` to keep a
floating tag (freezes on an EOL major purely for convention); pin to a commit SHA (inconsistent with
the other 15 refs in these files, and next.md explicitly excluded SHA pinning). **Context:** iter
127, commits ebac57f → 8f76d48; deviation from next.md's `@v9` recommendation.

## 2026-07-25 — JVM manifest refresh accepted despite the raised Kotlin consumer floor

**Decision:** The iter-128 JVM dependency refresh (11 pins across `pom.xml` and `build.gradle.kts`)
was approved and pushed to `develop` even though the `kotlin("jvm")` 2.1.10 → 2.4.10 bump verifiably
breaks consumers compiling with Kotlin < 2.3 (published metadata `mv=[2,4,0]`; 2.1.10 and 2.2.21
fail, 2.3.21 passes). The support-floor question was split out as a `normal` `[review]` issue with
HUMAN REVIEW REQUESTED instead of blocking the slice. **Why:** the break cannot reach a user from
`develop` — `io.iscc:iscc-lib-kotlin` only ships via the human-dispatched `release.yml`, so there is
a natural gate before harm, and pushing lets the 41-job CI validate the other 10 pins (the real gate
for this slice). Choosing the floor is a support-policy decision of the same class as MSRV and
`java-version: '17'`, which the step's own scope explicitly reserved for the owner; a
reviewer-forced revert would have made that choice by default. **Alternatives:** NEEDS_WORK + revert
the Kotlin line to 2.1.x (silently picks "preserve the old floor" without the owner, and denies the
other pins their CI validation); pin `compilerOptions.languageVersion` to 2.1 (insufficient — the
transitive `kotlin-stdlib:2.4.10` in the published POM raises the same error on its own); HUMAN
REVIEW REQUESTED on the handoff to pause the runner (disproportionate — the loop still has
actionable refresh slices, and the issue carries the escalation). **Context:** iter 128, commit
be0a497; found independently by this review and by the Codex second opinion, then confirmed
empirically against consumer projects on Kotlin 2.1.10 / 2.2.21 / 2.3.21.

## 2026-07-25 — Go `x/text` refresh accepted on exhaustive differential evidence; Unicode divergence split out

**Decision:** The iter-129 Go module refresh (`golang.org/x/text` 0.34.0 → 0.40.0) was approved on
the strength of an exhaustive differential — `TextClean`/`TextCollapse` are byte-identical for all
1,112,032 code points under both versions — rather than on the green conformance vectors alone. The
Unicode-version divergence uncovered while establishing that baseline (Rust core keeps the 5,813
code points assigned in Unicode 16/17; Go and `iscc-core` strip them, so Meta/Text codes differ) was
filed as its own `normal` `[review]` issue with HUMAN REVIEW REQUESTED instead of downgrading this
verdict. **Why:** `x/text` ships Unicode tables and feeds NFKC in every `Gen*CodeV0`, so a
vector-green result proves only that the 50 vendored vectors are unaffected — all of them predate
Unicode 16, which is exactly the region where implementations disagree. The differential converts
"no vector regressed" into "no input can regress", which is the claim the refresh actually needs.
The divergence itself is pre-existing, is not reachable from this diff, and picking a Unicode
version is a conformance-policy call with an upstream (ISO 24138 / `iscc-core`) dimension that will
flip direction again when CPython ships Unicode 16 — not something a dependency-refresh slice may
settle. **Alternatives:** accept on vectors alone (cheap, but would have left the divergence
undetected and the bump unjustified in the one dimension that matters); NEEDS_WORK on the slice
until the Unicode question is resolved (punishes a clean, output-neutral refresh for a defect it did
not introduce, and stalls the remaining slices); pin the Rust core to Unicode 15 immediately
(unilaterally sets conformance policy, and re-breaks when the reference moves). **Context:** iter
129, commit 4404fba; divergence reproduced against Go, the Rust core via the Python binding, and
`iscc-core` 1.3.0 side by side.

## 2026-07-25 — ruff 0.16 findings are fixed at the source, never suppressed; stub bodies go docstring-only

**Decision:** The ruff 0.16 adoption resolves every finding by changing the offending code or, where
that is impossible, by an explicit lint-config decision — never by `# noqa`, `per-file-ignores` or
`ignore` entries. Concretely for slice A: the 36 lone `...` placeholders were **deleted** from the
published `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`, making a docstring the entire stub body,
and that becomes the recorded convention for every future Tier 1 symbol. **Why:** `_lowlevel.pyi`
ships in the wheel next to `py.typed`, so it is a consumer-facing artifact — the removal was
therefore validated beyond the repo's own gate (`ty`) against the two dominant third-party checkers,
`mypy 1.18 --strict` and `pyright 1.1.407`, both clean. Suppressing instead would have been
indistinguishable from gate weakening under this project's rules, and would have locked `ruff<0.16`
in place permanently. The same principle pre-decides the hardest remaining finding: the 15 `RUF100`s
sit on the `# noqa: S603/S607` directives that the pre-push `ruff check --select S` hook depends on,
so they must be resolved by widening `[tool.ruff.lint] select` (making `S`/`C901` first-class), not
by deleting the directives that keep the security gate green. **Alternatives:** add
`PIE790`/`PYI048` to `per-file-ignores` for `*.pyi` (one line, but it is a suppression and leaves
the stub in a form ruff will keep flagging); keep the `ruff<0.16` pin indefinitely (freezes the
Python toolchain and defers an ever-growing diff); run `ruff@0.16 check --fix .` wholesale (would
silently delete the security `# noqa`s and red the pre-push `S` gate). **Context:** iter 131, commit
0544797; whole-tree 0.16 findings 104 → 26, pin deliberately retained.

## 2026-07-25 — Kotlin bindings track the current compiler; supported consumer floor is Kotlin 2.3+

**Decision:** `io.iscc:iscc-lib-kotlin` keeps `kotlin("jvm") 2.4.10` and declares **Kotlin 2.3 or
newer** as its supported consumer floor. The floor is documented in `packages/kotlin/README.md`,
`docs/howto/kotlin.md`, the root README Kotlin section and `specs/kotlin-bindings.md`; every future
compiler bump that moves the floor must update all four in the same step. **Why:** the Kotlin
compiler stamps a metadata version into the published classes and puts a matching `kotlin-stdlib` in
the POM's compile scope, and Kotlin only tolerates about one minor of forward metadata — so "publish
with a current compiler" and "support very old consumers" are mutually exclusive. Tracking the
compiler is the JVM-ecosystem norm; the alternative buys old-consumer support at the price of a
permanent two-part hold-back. Nothing has shipped yet (only a human-dispatched `release.yml`
publishes the artifact), so documenting the floor costs nothing but doc lines. **Alternatives:**
hold `kotlin("jvm")` at 2.1.x — rejected: it requires constraining the transitive `kotlin-stdlib`
too (pinning `compilerOptions.languageVersion` alone is empirically insufficient) and freezes the
Kotlin toolchain indefinitely; compile with 2.3.x to obtain a 2.2 floor — rejected: one extra minor
of consumer reach does not justify a soft hold-back plus its own verification step. **Context:**
interactive session with Titusz, 2026-07-25, resolving the `[review]` issue filed at iter 128
(empirical matrix: 2.1.10 FAIL, 2.2.21 FAIL, 2.3.21 PASS).

## 2026-07-25 — The unpinned Unicode version is raised upstream with iscc/iscc-core

**Decision:** file a GitHub issue on `iscc/iscc-core` reporting that ISO 24138 pins no Unicode data
version, so `iscc-core` produces different ISCCs for identical input depending on the CPython
version it runs under, and asking whether the standard should pin one. **Why:** it is a determinism
defect in the reference regardless of the position iscc-lib takes, every reimplementation will
rediscover it independently, and the upstream answer directly informs which version this project
declares. We hold the evidence already (whole-code-space differential sweep, minimal repro at
U+A7CB). **Alternatives:** settle our own position first and file a proposal — rejected: sequences
two slow decisions where the upstream input is most useful before ours is fixed; keep it local —
rejected: leaves the reference non-deterministic and the spec gap unrecorded. **Context:**
interactive session with Titusz, 2026-07-25.

## 2026-07-25 — Ruff `S`/`C901` enter the default select; the two pre-push hooks stay as redundancy

**Decision:** the security (`S`) and complexity (`C901`) rule sets moved into
`[tool.ruff.lint] extend-select` in `pyproject.toml`, so plain `ruff check` (pre-commit,
`mise run lint`, CI) enforces them — and the `security` / `complexity` pre-push hooks in
`.pre-commit-config.yaml`, which pass `--select S` / `--select C901` explicitly, were deliberately
**kept** even though they are now redundant with the project config. **Why:** two independent
reasons to keep the duplication. (1) The hooks pass `--select` on the command line, so they keep
enforcing both gates regardless of what a future `pyproject.toml` edit does to the selection — a
config regression cannot silently disable the security scan. (2) They name the failing gate in the
push output ("Security scan (Ruff S rules)"), which a generic `ruff check` failure does not.
`extend-select` rather than `select` because `select` *replaces* ruff's `E4`/`E7`/`E9`/`F` defaults
and would silently drop pyflakes coverage. Side effect that motivated the change: the 14
load-bearing `# noqa: S603/S607` directives in `tools/`/`scripts/` are only *recognised* when `S` is
selected — otherwise ruff 0.16 reports them all as unused (`RUF100`) and `--fix` deletes them,
reddening the pre-push gate. **Alternatives:** delete the `# noqa` directives to satisfy `RUF100` —
rejected, it disables the S gate at those call sites; add `per-file-ignores` for `tools/`/`scripts/`
— rejected as gate-weakening by scope exclusion; drop the now-redundant pre-push hooks — rejected
per the two reasons above. **Context:** CID iteration 134 (`82b8e37`), ruff 0.16 adoption sub-slice
B of the v0.6.0 dependency-refresh issue.

## 2026-07-25 — ruff 0.16's Markdown formatting reach is accepted, not excluded

**Decision:** dropping the `ruff<0.16` pin widened the bare `uv run ruff format --check` (the exact
command in `mise run lint` and `ci.yml`) from 25 files to 153, because ruff 0.16 formats Python code
blocks embedded in Markdown. No `[tool.ruff.format] exclude` / `extend-exclude` was added to keep
ruff Python-only — the widening is accepted as free coverage for documentation examples. **Why:**
all 129 tracked `.md` files are already 0.16-clean (measured before and after the bump), so the
widening cost zero churn, and the project's documentation is full of executable Python examples that
nothing else formats — `mdformat` owns Markdown structure but never touches code-fence contents.
Adding an exclude to preserve the old surface would have been speculative config written to avoid a
problem that does not exist. **Alternatives:** add `exclude`/`extend-exclude` to pin the surface at
25 files — rejected as speculative and as scope exclusion that would dodge a check rather than fix
anything; leave the `ruff<0.16` pin in place — rejected, the hold-back reason (3 outstanding
findings) was fully cleared in sub-slices A–D. **Consequence:** local and CI surfaces now disagree —
the prek `ruff-check`/`ruff-format` hooks declare `types: [python]` and pre-push never runs
`ruff format`, so a mis-formatted Python snippet in a `.md` passes `mise run check` and reds CI.
Filed as a `normal` `[review]` issue with two candidate fixes (widen the hook types, or mirror the
bare command as a pre-push hook); until it is closed, run `uv run ruff format --check` by hand after
editing Markdown that contains Python. **Context:** CID iteration 137 (`5ad1e17`), ruff 0.16
adoption sub-slice E — the final piece of slice 8 of the v0.6.0 dependency-refresh issue.

## 2026-07-25 — Gate parity via widened prek hook types, not a duplicate pre-push check

**Decision:** the local/CI ruff gate-parity gap opened by ruff 0.16 was closed with option (a) from
the 2026-07-25 "Markdown formatting reach" entry — the `.pre-commit-config.yaml` `ruff-format` hook
declares `types_or: [python, markdown]`. Option (b), a bare `ruff format --check` pre-push hook with
`pass_filenames: false` mirroring the CI command, was rejected. The `ruff-check` hook stays
`types: [python]`. **Why:** (a) makes the gate *auto-fixing* at the moment the file is edited, which
is what every other formatter hook in this repo does, whereas (b) only detects — and detects one
push later, after the author has moved on. (b) would also re-scan all 153 files on every push for a
class of change that is already covered pre-commit. `ruff-check` stays Python-only because ruff 0.16
formats Markdown fences but does not *lint* them (`.md` paths print "No Python files found" and exit
0), so widening it buys zero enforcement and adds per-commit warning noise. **Alternatives:** do
both (a) and (b) for defence in depth — rejected, a second copy of an auto-fixing gate is pure
latency; add `[tool.ruff.format] exclude` to shrink CI back to 25 files — already rejected in the
earlier entry as scope exclusion. **Consequence / correction:** the claim that the hook surface is a
"strict superset" of CI's is **false**. Both see 153 files, but different ones — prek classifies
`.pyi` as the `pyi` type, not `python`, so the hook misses the published
`crates/iscc-py/python/iscc_lib/_lowlevel.pyi` while picking up a tracked-but-gitignored
`.claude/plans/*.md` that CI's recursive discovery skips. Adding `pyi` to both ruff hooks makes
local a genuine superset; filed as a `normal` `[review]` issue rather than fixed in review, because
it changes what two gates reject and belongs in a scoped work package. **Context:** CID iteration
138 (`b599816`) review, closing the iter-137 gate-parity issue.

## 2026-07-25 — Blanket `!cancelled() && !failure()` guards on every release.yml job

**Decision:** all 28 non-`prepare-release` jobs in `.github/workflows/release.yml` now carry
`if: ${{ !cancelled() && !failure() && (<existing registry condition>) }}`, replacing the bare
condition. `prepare-release` itself keeps the unguarded `if: inputs.version != ''`. **Why:** a bare
`if:` still carries an implicit `success()` requirement on `needs`, and GitHub propagates
`prepare-release`'s *skipped* status transitively down the whole `needs` chain on a registry-only
dispatch — so the documented `gh workflow run release.yml -f <registry>=true` recovery path built
artifacts and then silently published nothing (verified empirically 2026-06-18 on a `-f npm=true`
run). The guard neutralises only the propagated *skip*: `!failure()` is evaluated against the job's
own `needs`, so a publish job still refuses to run after a genuinely failed build or test.
`prepare-release` stays unguarded on purpose — guarding it would make the tag-pushing job fire on
registry-only dispatches. **Alternatives:** `always()` — rejected, it publishes after a failed build
or test; restructure `needs` so publish jobs do not depend on `prepare-release` — rejected, the
dependency is what serialises tag creation before publication on a real release; leave it broken and
document `gh run rerun --failed` as the only recovery — rejected, `--failed` cannot re-run `skipped`
jobs and cannot help once build artifacts have expired. **Safety invariant established in review:**
relaxing the implicit `success()` is only safe because every job's `needs` chain is gated by the
same registry flag or a superset (`build-ffi` is `ffi || nuget`, feeding both `test-ffi` and
`pack-nuget`). A future job whose dependency is gated by a *different* flag would now run against
artifacts that were never built instead of skipping — any new job or registry must preserve this.
**Consequence:** the fix is verified statically only (`yaml.safe_load` + a shape regex over all 29
`if:` values, `actionlint@v1.7.7`, `check-yaml`, `yamlfix`); no CID push or CI run exercises
`release.yml`, so first real-world confirmation comes with the next release or registry re-trigger.
`.claude/skills/release/SKILL.md` says so explicitly rather than claiming the path is proven.
**Context:** CID iteration 139 (`8c525cf`), closing the `[human]` issue "Fix broken single-registry
re-trigger in release.yml".

## 2026-07-25 — `release.yml` action majors refreshed on static evidence only, `digest-mismatch` left strict

**Decision:** the nine stale `uses:` refs in `.github/workflows/release.yml` were bumped to their
current floating majors (`checkout@v7`, `download-artifact@v8`, `upload-artifact@v7`,
`setup-java@v5`, `setup-node@v7`, `setup-dotnet@v6`, `setup-python@v7`,
`softprops/action-gh-release@v3`, `cache@v6`) and shipped **without any runtime confirmation**, and
`actions/download-artifact@v8`'s new `digest-mismatch: error` default was deliberately *not*
overridden back to `warn`. **Why:** `release.yml` is `workflow_dispatch`-only, so no CI or CID push
can ever exercise it — waiting for empirical proof would mean never bumping it, and the alternative
(a throwaway release) costs a real tag. The evidence substituted for a run is (a) six of the nine
majors already green in `ci.yml` with byte-identical `with:` blocks since iter 127, (b) every
`with:` key the workflow passes still declared under `inputs` in each new major's `action.yml`
fetched at the tag ref, (c) the one step output the workflow reads (`cache@v6` → `cache-hit`) still
declared, and (d) every intervening major's release notes read for *default* changes, not just input
removals. Strict digest checking is the right default for a publish pipeline: a corrupted artifact
should fail the run, not emit a warning that publishes anyway. **Alternatives:** pin to commit SHAs
for auditability — rejected, the repo convention is floating majors (only `setup-uv@v9.0.0` is
exact, and only because upstream stopped publishing majors); bump `upload-artifact` and
`download-artifact` in separate commits for bisectability — rejected, the two must interoperate, so
splitting them guarantees one broken intermediate commit; set `digest-mismatch: warn` to preserve
iter-139 behaviour exactly — rejected, that trades a real integrity check for a cosmetic no-change
diff. **Consequence:** first real-world confirmation is the next release run. If a publish job fails
on a digest mismatch, the fix is to investigate the artifact, not to relax the input. The static
checks that stand in for a run are currently re-typed by hand each iteration; landing them as a
`scripts/` gate is filed as a `[review]` issue. **Context:** CID iteration 140 (`ed2a06a`) review,
closing the `release.yml` remainder of the v0.6.0 dependency-refresh issue.

## 2026-07-25 — release.yml artifact matching is a deliberately strict approximation of glob intersection

**Decision:** `references_match` in `scripts/check_release_workflow.py` resolves a
`download-artifact` reference against an `upload-artifact` name by trying **both** directions —
either glob, with the other side's `*` characters collapsed, must `fullmatch` the other — rather
than the single direction prescribed in next.md ("regex from the upload name; download with its `*`
removed must fullmatch"). **Why:** the prescribed one-directional rule cannot resolve the real
workflow. `wheels-*` (a download `pattern:`) against the matrix-expanded literal
`wheels-ubuntu-latest-x86_64` needs the download→upload direction; `gem-x86_64-linux` (a literal
download) against the `gem-*` upload — `build-gem` uses a plain `matrix.platform` list with no
`include`, so its upload name keeps a wildcard after expansion — needs the upload→download
direction. Neither direction alone resolves all 21 expanded download references at HEAD, so
next.md's own "resolves all 20 downloads" claim was untested; review re-proved the failure before
crediting the deviation. **Alternatives:** implement true glob intersection (product-automaton or
`*`-normalised DP) — rejected as over-engineering for a 34-upload / 21-download file whose only
wildcards come from matrix spans; add `include:` blocks to `build-gem` so every upload expands to a
literal — rejected, `Not In Scope` forbids editing `release.yml` and a gate must not dictate the
shape of the artifact it gates; drop check 2 — rejected, it is half the point of the gate.
**Consequence:** the matcher errs **strict**, never lax. A future pairing that wildcards on both
sides in different positions (`gem-*` upload vs a hypothetical `*-linux` download) overlaps in
reality but is reported as an error — independently flagged by the Codex review this iteration and
now stated in the function's docstring. If that day comes the fix is a real intersection, never a
looser skeleton match: a false negative here means a release publishes an artifact that was never
built. **Context:** CID iteration 142 (`bcb3128`), closing checks 1–2 of the `[review]` issue "Land
the `release.yml` static checks as an executable gate".

## 2026-07-26 — The release-workflow action-input gate fails open on transport trouble

**Decision:** `--check-action-inputs` treats a 404 on **both** `action.yml` and `action.yaml` as a
hard error (exit 1) but degrades every other network failure — timeout, connection refused, HTTP
403/429/5xx — to a `warning: skipped <ref>: <reason>` line on stderr and exit 0, so a fully
rate-limited CI run is green and indistinguishable from a real pass by job status alone. **Why:**
the gate runs on every push to `develop`, i.e. on every CID iteration; a gate that goes red because
`raw.githubusercontent.com` hiccupped would train the loop to ignore it, and the failure it guards
against (an action major silently dropping an input) is a release-day risk, not a per-push one. The
404 case is carved out because it is not transport noise — it means the ref or sub-path in
`release.yml` is wrong, which is exactly a real defect. **Alternatives:** fail on any non-200 —
rejected, turns third-party availability into a merge blocker; require at least *N* refs to resolve
before reporting success — rejected as unspecified scope for this step, but it is the natural
hardening if skips are ever observed in practice, and it is filed as a follow-up; authenticate with
`GITHUB_TOKEN` to raise the rate limit — rejected, it couples an offline-capable script to CI
secrets and `gh` availability. **Consequence:** the CI job's *log* is the signal, not its status
badge; a reviewer checking this gate must confirm zero `warning: skipped` lines. Verified in review
that the intended failure class is really caught: reverting `actions/download-artifact@v8` to `@v3`
in a temp copy produced 14 errors, and `@v999` produced the 404 error. **Context:** CID iteration
144 (`5bd9b74`).

## 2026-07-26 — The docs page-list gate parses `zensical.toml` nav with a regex, not `tomllib`

**Decision:** `scripts/check_docs_nav.py` extracts nav pages by slicing the `nav = [ ... ]` block
and `re.findall`-ing quoted `.md` values, accepting that a *commented-out* nav entry is still
counted as present. **Why:** the gate's anchor test is a pytest test, and CI's `python-test` matrix
pins `['3.10', '3.14']` — `tomllib` is 3.11+, so a `tomllib` parse would make the gate unimportable
on half the matrix and silently drop it from CI on the older leg. Vendoring `tomli` for one
regex-sized job adds a dependency to a script whose whole point is being stdlib-only and
network-free, and the repo already parses manifests by regex in `scripts/version_sync.py`.
**Alternatives:** `tomllib` guarded by a `sys.version_info` fallback to `tomli` — rejected, two code
paths for one list and a new dependency; drop the 3.10 matrix leg — rejected, `abi3-py310` wheels
make 3.10 a supported runtime and the leg is the only thing testing it; strip comments before
matching — *not* rejected on merit, simply out of scope for this step and now filed as the follow-up
issue. **Consequence:** the gate is one-directional against comment-outs specifically: deleting a
nav entry fires, commenting one out does not. Verified in review that every non-comment mutation
fires — the real pre-fix six-page `llms.txt` drift, a dropped nav entry, a dropped `ORDERED_PAGES`
entry, a new page on disk, and a ghost page in a list — and that all three malformed-input paths (no
`nav` block, an unimportable `gen_llms_full.py`, a missing `llms.txt`) fail closed with a non-zero
exit. **Context:** CID iteration 145 (`22c873c`).

## 2026-07-26 — `rubygems/configure-rubygems-credentials` is pinned to the exact tag `@v2.1.0`, not a SHA

**Decision:** the only unpinned `uses:` in the repo moves from `@main` to `@v2.1.0` with an inline
`# exact tag:` comment, mirroring the `astral-sh/setup-uv@v9.0.0` precedent. The repo's
tags-never-SHAs convention (`decisions.md` 2026-07-25) stands; this does not become the first
SHA-pinned action. **Why:** branch → tag is where the real risk reduction is. The step mints the
OIDC credential with gem-publish authority, but 97 other `uses:` refs in `release.yml` are already
mutable floating majors, including the `checkout` and `upload-artifact` steps that build and carry
the very artifact being published — a compromise there is equally fatal, so SHA-pinning this one
step buys little while splitting the convention. Upstream publishes no floating `v2`, and `main` is
*ahead of* `v2.1.0`, so this is a small rollback to the newest release, not a major bump. The step
passes no `with:` keys, so the form is purely a trust-anchor choice with no input-compatibility
risk. **Alternatives:** a SHA pin as upstream's README recommends — rejected for the
convention-split reason above, and it needs a manual bump process the repo has nowhere else; leave
`@main` because every upstream example shows it — rejected, upstream's own NOTE advises consumers
against floating refs. **Consequence:** verification stays static (`workflow_dispatch`-only) plus
the first real gem publish. If SHA pinning is ever adopted it should be adopted repo-wide, not for
this step alone. **Context:** interactive session 2026-07-26, ruling on the parked `[review]` issue
from iteration 140.

## 2026-07-26 — Unicode determinism is a bounded-severity issue; keep the fix minimal

**Decision:** Unicode-version indeterminism is treated as a **correctness-hygiene** issue, not a
high-severity defect. The freeze rule, its boundary vectors and the criterion-4 sweep are the
complete intended response. Do **not** escalate further: no algorithm versioning, no compatibility
modes, no per-call Unicode-version options, no further iterations spent re-deriving the trade-off.
If a future step proposes work here beyond what `specs/rust-core.md` already requires, that is scope
creep and should be declined with a pointer to this entry. **Why** (Titusz 2026-07-26, and the
reason the earlier framing over-weighted it):

- **(a) Data-Code and Instance-Code are unaffected.** They hash raw bytes; no Unicode processing is
    involved. So the two units that carry exact-identity semantics are entirely outside the blast
    radius, and an ISCC-CODE's Data/Instance halves stay stable even when its Meta/Content halves
    move.
- **(b) The affected units are similarity-preserving.** Meta-Code (simhash) and Text-Code (minhash
    over 13-grams) smooth over small input differences by construction: a single differing code
    point shifts the code by a small Hamming distance, so similarity matching — the thing these
    units exist for — keeps working. Only exact-identifier equality changes.
- **(c) Newly assigned code points are not present in text data at scale.** Each Unicode version
    adds a few thousand code points dominated by historic scripts, minority scripts and
    legacy-computing symbols; real-world text containing them is rare, and rarer still in the window
    between a Unicode release and a runtime adopting it.

**Residual exposure, stated so it is not rediscovered as a surprise:** exact-match ISCC lookups on
**short** inputs, where n-gram smoothing has little to work with (a title consisting only of an
affected character is the worst case, and degenerates to `""`), and the returned `name` /
`description` fields, which are exact strings rather than similarity hashes. Both are narrow and
neither changes the decision. **Alternatives:** treat it as a conformance-critical defect and pursue
exhaustive guarantees — rejected, that is what produced four iterations of parked rulings and the
15.1-vs-16.0 detour; document it as a known wart and implement nothing — rejected, the sentinel fix
is one token and the sweep is cheap, so there is no reason to leave a known non-conformance in
place. **Consequence:** the specified work is bounded and finite. This entry is the stop signal, and
the same proportionality argument is now stated for users in `docs/unicode.md` and for upstream in
<https://github.com/iscc/iscc-core/issues/137#issuecomment-5082815029>. **Context:** interactive
session 2026-07-26, Titusz halting further Unicode bikeshedding.

## 2026-07-27 — Agent prompts state no token costs, and separate finding from filing

**Decision:** two standing rules for every CID prompt and context pack. (1) Never frame inlined
content as a cost, a billing event, or a context-limit risk — state the fact and leave accounting to
`tools/cid.py`. (2) Instructions that narrow *what to look for* are forbidden in a finding pass; the
severity bar belongs at the filing step. **Why:** the Claude Fable 5 guide documents that surfacing
token budgets makes a model trim its own work, and the Opus 5 guide documents that a review prompt
saying "be conservative" is followed literally and reports less — "ask it to report everything and
filter in a separate pass instead". Both misfires were live: `review` was told it "runs closest to
the context limit … Guard it" (it is the quality gate and the costliest role, 103 turns /
$7.89 at iteration 161), and `audit` suppressed nits in the finder prompt *and* had its
skeptics refute them by default. **Evidence:** audit iterations 130/140/150/160 filed zero issues
for $11.53. **Alternatives:** leave the audit alone and read 4/4 clean passes as real — rejected,
the double suppression makes a clean pass uninformative either way. **Context:** interactive
session, 2026-07-27, against the platform prompting guides for Opus 5 and Fable 5.

## 2026-07-27 — The xunit v3 test project stays in VSTest runner mode

**Decision:** `packages/dotnet/Iscc.Lib.Tests` moves to `xunit.v3` 3.x + `Microsoft.NET.Test.Sdk`
18.x but keeps the classic VSTest runner (`xunit.runner.visualstudio` 3.x, no
`TestingPlatformDotnetTestSupport` / `UseMicrosoftTestingPlatformRunner`). The only csproj change
beyond the package swap is `<OutputType>Exe</OutputType>`, which v3 requires because its test
projects are stand-alone executables. **Why:** the `dotnet` CI job passes the native library path
with `dotnet test -e LD_LIBRARY_PATH=…`, and `-e` is a VSTest feature; Microsoft.Testing.Platform
mode would force a CI-invocation rewrite for no test-coverage gain. VSTest mode also kept the
migration to zero test-source edits. **Alternatives:** adopt MTP runner mode now — rejected, it
changes the CI contract and the P/Invoke library-path mechanism in the same step as a framework
major. **Context:** iteration 164; v2 and v3 both report exactly 104 results.

## 2026-07-28 — JNI exports return `#[repr(transparent)]` wrappers and keep hand-thrown exceptions

**Decision:** the 33 `extern "system"` functions in `crates/iscc-jni/src/lib.rs` return jni wrapper
types (`JString`, `JObject`, `JByteArray`, `JObjectArray`) instead of the raw `jstring`/`jobject`
aliases, and `throw_and_default`/`throw_state_error` keep calling `env.throw_new` themselves,
returning `Ok(T::default())` into `.resolve::<ThrowRuntimeExAndDefault>()`. **Why:** jni 0.22's
`EnvOutcome::resolve` requires `T: Default`, which raw pointer aliases do not implement; the
wrappers are `#[repr(transparent)]`, so the ABI Java sees is unchanged. The policy's `on_error`
calls `Env::exception_check` first (jni-0.22.4 `src/errors/policy.rs:188`) and returns the default
without throwing when an exception is already pending, so the `IllegalArgumentException` /
`IllegalStateException` contract asserted by six Java tests survives untouched and the policy only
fires on a genuine panic or unhandled `Err`. **Alternatives:** a custom `ErrorPolicy` per exception
class — rejected, it would move the exception choice away from the call site that knows which one
applies, for no behavioural gain. **Context:** iteration 168, jni 0.21 → 0.22 migration.

## 2026-07-28 — uniffi 0.32 accepted with a raised source-build floor on unpublished `iscc-uniffi`

**Decision:** the workspace pins `uniffi = "0.32"` even though its default graph
(`cargo_metadata 0.23.1` → rustc 1.86, `cargo-platform 0.3.3` → 1.91) makes `iscc-uniffi`
unbuildable on the inherited `rust-version = "1.85"`. **Why:** `iscc-uniffi` is `publish = false`
and exists only to emit the Swift/Kotlin bindings; the published crate still passes
`cargo +1.85.0 check -p iscc-lib --locked`, every CI job builds on stable, and no gate or consumer
promise moves. Holding at 0.31 to protect a floor nothing verifies would have blocked the last
authorized v0.6.0 dependency item. **Alternatives:** raise the workspace `rust-version` — rejected,
that is the published MSRV promise and Titusz's call; pin the transitives back — rejected, uniffi
0.32 requires them. **Context:** iteration 172; the now-false inherited declaration is filed as a
`normal` issue proposing an explicit per-crate `rust-version` on `iscc-uniffi`.

## 2026-07-29 — JS-number ISCC-IDv1 surfaces take all params as `f64` and validate before narrowing

**Decision:** on the two JS-number surfaces (napi, wasm) `gen_iscc_id_v1` takes `timestamp`,
`hub_id` and `realm` all as `f64` — not the core's `u16`/`u8` — and a per-file `checked()` rejects
non-finite / non-integral / negative / out-of-range values before narrowing. **Why:** napi and wasm
coerce a JS `number` into the declared integer type (ToUint32 / truncation) *before* the wrapper
runs, so integer params make the low and non-integral end unvalidatable — an out-of-contract input
would silently mint a valid-but-wrong ID (a collision hazard). Only `f64` preserves the original
value for a range check. **Alternatives:** keep integer params and rely on core — rejected, core
only guards the high end and never sees the pre-truncated value; a shared validation crate —
rejected, ~5 duplicated lines beat a new crate boundary. **Scope:** JS-number surfaces only;
ffi/jni/rb/uniffi are typed-int with no coercion hazard and must not copy the pattern. **Context:**
iteration 181, #43 fan-out; resolves the napi coercion issue for both surfaces at once.
