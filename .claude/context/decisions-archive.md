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

## 2026-07-25 — Unicode data version is declared and gated, not chased

**Decision:** the project **declares an explicit Unicode data version in `specs/rust-core.md`** and
enforces it with conformance vectors covering post-Unicode-15 code points, rather than pinning the
Rust core to whatever Unicode version the reference implementation happens to use. The concrete
version (15.1.0 vs 16.0.0) is decided by a follow-up measurement step that reports the cost of each
candidate; the policy — declare + gate — is settled now. **Why:** `iscc-core` inherits CPython's
`unicodedata` tables, so the reference itself emits different ISCCs on 3.13 (Unicode 15.1) and 3.14
(Unicode 16) for the same input. There is no stable target to chase: matching the reference today
means re-pinning tomorrow, and the Rust/Go alignment inverts again when Go 1.27 lands. Declaring a
version converts an invisible, undetected divergence (5,813 disagreeing code points, no gate) into a
stated contract that CI enforces in all 12 bindings. **Alternatives:** pin the Rust core to the
reference's current version — rejected: needs an older `unicode-general-category` or a vendored
category table, forces a re-pin on every reference runtime change, and still leaves the drift
ungated; document the divergence as out-of-contract — rejected: weakens the project's central
"output matching iscc-core" claim for real-world text (newly assigned scripts and emoji) and adds no
gate, so the next drift is again silent. **Context:** interactive session with Titusz, 2026-07-25,
resolving the `[review]` issue filed at iter 129.

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

## 2026-07-25 — Unicode data version 16.0.0 with a freeze rule

**Decision:** the declared Unicode data version is **16.0.0** (CPython 3.14's tables), combined with
a **freeze rule**: code points unassigned in Unicode 16.0.0 are removed before any normalization or
category lookup in `text_clean`/`text_collapse`, via a vendored table of 731 unassigned ranges.
Runtimes with older tables get real 16.0 data (`iscc-core` adds
`unicodedata2==16.0.0; python_version < '3.14'`, wheels cover cp39–cp313); runtimes with newer
tables need nothing, ever — the freeze rule strips post-16 characters regardless of what the runtime
ships, and normalization drift is impossible because removal precedes normalization. **Why:**
measurements showed all table drift between 15.1, 16 and 17 comes from newly assigned code points
(5,185 + 4,803 category changes, 56 + 1 normalization changes, zero changes to existing characters),
so stripping the frozen unassigned set makes output permanently version-independent with zero
ongoing maintenance — while pinning 16.0 as the baseline keeps modern coverage (Unicode 16 emoji,
CJK Ext I) and matches current CPython natively. The backward-compat glitch (inputs with 15.1→16
characters hash differently than on CPython ≤ 3.13 historically) is accepted; codes stay
hamming-close. **Alternatives:** pin 15.1 everywhere — rejected: `unicodedata2==15.1.0` has no
wheels past cp312 (old releases never gain new wheels), no Rust crate carries 15.1 category tables,
and Go stdlib tables cannot be pinned at all; freeze at 15.1 without the 16 baseline — rejected:
gives up Unicode 15.1/16 additions already in real-world use for no extra safety; pin 16 without the
freeze rule — rejected: forces a coordinated re-pin at every future Unicode release (next: CPython
3.15 / go1.27 ship Unicode 17 in 2026) with a permanent `unicodedata2` treadmill. Precedent:
IDNA2003 froze at Unicode 3.2. **Context:** interactive session with Titusz, 2026-07-25 — compromise
proposed by Titusz after comparing pin-old/pin-new/freeze options; supersedes the open version
question in the 2026-07-25 "declared and gated, not chased" entry.

## 2026-07-25 — The freeze rule's pre-normalization ordering is kept despite sequence divergence

**Decision:** accept the iter-133 implementation of the Unicode 16.0.0 freeze rule (strip
16.0-unassigned code points *before* normalization in `text_clean`/`text_collapse`) even though the
review measured a new class of divergence from `iscc-core` that the decision record did not
anticipate: because removal happens before normalization, it changes character *adjacency* and
unblocks contextual transforms the reference still blocks — canonical composition
(`text_clean("e\u{0378}\u{0301}")` → `U+00E9` vs `U+0065 U+0301`), Hangul jamo composition, and
Rust's `Final_Sigma` lowercasing in `text_collapse` (`…σ…` vs `…ς…`). The divergence is independent
of Unicode table version: it persists when both sides run 16.0 data. **Why:** the alternative —
removing unassigned code points *after* normalization, as `iscc-core` does — reintroduces exactly
the normalization drift the freeze rule exists to eliminate (measured 16→17: U+A7F1 NFKC-maps to `S`
before any category filter can drop it), so pre-normalization removal is the only ordering that
delivers table-version invariance. The affected inputs require an unassigned code point wedged
between a base and a combining mark / jamo / cased letter, which no natural text produces, and the
divergence disappears once `iscc-core` adopts the same ordering — which is what the upstream
proposal asks for. **Alternatives:** revert to post-normalization removal — rejected, defeats the
purpose of the rule; strip before normalization but re-insert a blocking sentinel (e.g. U+034F
COMBINING GRAPHEME JOINER) — rejected as a novel, unspecified behaviour that would itself have to be
mirrored by every implementation; block the merge pending a human ruling — rejected, the
implementation matches the approved spec exactly and the open question is spec *wording*, not code.
**Consequence:** spec criterion 4's "output-equivalent to uniform Unicode 16.0.0 tables" is false as
literally worded (a per-code-point sweep cannot see this class), and the accepted-divergence
paragraph does not cover it. Both are filed as a `normal` `[review]` issue with HUMAN REVIEW
REQUESTED, to be settled before boundary vectors are wired into the bindings. **Context:** CID
iteration 133 review.

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

## 2026-07-25 — Unicode 16.0 boundary vectors ship as a second fixture file, duplicating the inline assertions

**Decision:** the declared-Unicode-version boundary vectors live in a new
`crates/iscc-lib/tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs`, not in the
vendored `crates/iscc-lib/tests/data.json`; the four equivalent inline assertions in
`crates/iscc-lib/src/utils.rs` are deliberately **kept**, so the same expectations are asserted
twice; the fixture covers **single code points only**, no sequences; and `conformance_selftest()`
was **not** extended to run it. **Why:** `data.json` is a re-vendored upstream artifact whose vector
counts are hardcoded in Rust core and WASM tests, so merging project-authored vectors into it would
break the next re-vendor and force count-assert churn in two crates. The inline `utils.rs` tests
cover the private `is_unassigned_in_unicode16` helper (which the public-API fixture cannot reach)
and deleting them would move coverage and force a `.crap-baseline.json` refresh in a step that
otherwise touches no source file. Sequences are excluded because the open `[review]` issue
"Freeze-rule ordering diverges from iscc-core on sequences" is unresolved — any `base + Cn + mark`,
`jamo + Cn + jamo` or `Σ + Cn + cased` expectation would have to be re-derived after the ruling,
twice over once the fixture is copied into 11 bindings. `conformance_selftest()` is a Tier 1 symbol
exposed in all 12 bindings; changing what it runs is an API-semantics decision, not a test-fixture
decision. **Alternatives:** append the vectors to `data.json` — rejected (re-vendor + count
asserts); replace the inline assertions with the fixture — rejected (private-helper coverage, CRAP
baseline churn); include sequence vectors now — rejected (blocked on the parked ruling); extend
`conformance_selftest()` — rejected, deferred to its own scoped package. **Consequence:** the
duplication is the specified end state, not debt; a future reader finding the same four code points
in two places should not "clean up" either copy. Because the vector tests compare the implementation
against the fixture, the fixture's own content is guarded by an ungated assertion on the exact
non-ASCII code-point set per section (added in review) — without it a weakened fixture would still
pass every test. **Context:** CID iteration 141 (`918f8f4`), Rust half of `specs/rust-core.md`
criterion 3; binding propagation (step b) stays blocked on the ordering ruling and the Go
Unicode-15.0-tables decision.

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

## 2026-07-26 — The public Unicode page scopes its `iscc-core` agreement claim to single code points

**Decision:** `docs/unicode.md` states that CPython 3.14 "agrees with iscc-lib **on the
single-code-point behaviour described on this page**" and says nothing at all about multi-code-point
sequences, even though a divergence class there is known and measured. **Why:** the freeze rule
strips Unicode-16-unassigned code points *before* normalization while `iscc-core` strips them
*after*, which changes character adjacency and therefore unblocks contextual transforms (canonical
composition, Hangul jamo composition, `Final_Sigma`) — so an unqualified "3.14 agrees" is provably
false, and it was published in the first draft of this page (caught in review by both the reviewer
and the Codex second opinion). The corrective wording had to remove the false claim without
*asserting* the sequence behaviour, because whether that behaviour is a bug or an accepted deviation
is an open `HUMAN REVIEW REQUESTED` item (issues.md, "Freeze-rule ordering diverges from iscc-core
on sequences") whose ruling also decides the upstream proposal's wording. **Alternatives:** document
the sequence divergence now — rejected, it would publish a user-facing commitment to behaviour
Titusz has not ruled on, and next.md explicitly excluded it; drop the CPython 3.14 sentence entirely
— rejected, "3.14 ships 16.0 tables" is the single most useful fact for a Python adopter; leave the
blanket claim and file an issue — rejected, the page's whole purpose is divergence accuracy.
**Consequence:** the qualifier is a placeholder. When the ordering ruling lands, this sentence must
be revisited in the same step — either widened back to an unqualified "agrees" (if the ordering
changes) or expanded with the accepted sequence delta. **Context:** CID iteration 143 (`5b8b8f8` +
the review fixup).

## 2026-07-26 — "Practical guidance" quantifies the Unicode 16 delta instead of calling Latin text safe

**Decision:** the page's guidance names the full **5,185** code points assigned in Unicode 16.0 —
Egyptian Hieroglyph extensions, seven new scripts, 7 emoji and 32 Latin-script additions including
`U+A7CB` — and limits the "unaffected" promise to ASCII plus the long-established repertoire of
common scripts. **Why:** the draft (following next.md's Implementation Notes verbatim) claimed
"ASCII and Latin text is never affected" and characterised the affected set as "realistically the
seven emoji". Both are false: `U+A7CB` LATIN CAPITAL LETTER RAMS HORN is a Unicode 16 Latin addition
and is the *exact* character in this project's own recorded Go-vs-Rust divergence reproduction, so
the page — and the `docs/howto/go.md` note repeating it — told Go users their Latin corpus was safe
against the one interop failure the project has actually demonstrated. Re-measured in review by
diffing assigned-set dumps of `unicodedata2` 15.1.0 vs 16.0.0. **Alternatives:** NEEDS_WORK and a
full define-next/advance cycle — rejected, the defect is two sentences of prose with the correct
figures already measured, and holding the batch would have kept an otherwise accurate page
unpublished for two iterations; leave the claim and file an issue — rejected, a false safety claim
must not reach the published site. **Consequence:** guidance about "which inputs are affected" on
this page is now numeric and source-checkable; any future edit must re-measure rather than restate.
**Context:** CID iteration 143, review fixup on top of `5b8b8f8`.

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

## 2026-07-26 — The Unicode 16.0.0 freeze rule is a category override, not a pre-normalization filter

**Decision:** `is_unassigned_in_unicode16` moves out of the pre-normalization pass and into the
existing category-filter predicate of `text_clean` and `text_collapse`: a code point unassigned in
Unicode 16.0.0 is **treated as general category `C`** at the step where the reference removes
category `C`, instead of being stripped before normalization. The vendored 731-range table and its
generator (`scripts/gen_unicode16_unassigned.py`) are unchanged; only the call site moves. **Why:**
IEP-0003 (Normative, ISO 24138:2024) defines conformance as pure output-equivalence — "an
implementation … shall be regarded as conforming … as long as it creates the same Text-Code as the
reference implementation for the same UTF-8 encoded text input" — not as prose-matching its 5-step
Processing clause. The pre-normalization filter (iteration 133) therefore was not a wording defect
but real non-conformance: stripping before NFD/NFKC changes character adjacency, unblocking
canonical composition, Hangul jamo composition and `Final_Sigma` context that the reference still
blocks. Crucially this class diverges from **every** reference version including CPython 3.14,
unlike the single-code-point 15.1→16.0 glitch accepted on 2026-07-25, which resolves once
`iscc-core` ships Unicode 16 tables — so it would have been permanent non-conformance of
`text_clean`, `text_collapse`, Meta-Code and Text-Code. Note that `Cn ⊂ C`, so removing unassigned
code points is already mandated by Processing step 4; iteration 133 moved that removal rather than
adding one, and the override moves it back. Verified before the ruling that the override reproduces
the reference exactly on all three known divergent inputs (`text_clean("e͸́")` → `U+0065 U+0301`,
`text_clean("ᄀ͸ᅡ")` → `U+1100 U+1161`, `text_collapse("ΑΣ͸Β")` → `…U+03C2…`) and on the Cn-free
controls, while the pre-filter diverges on all three. **Alternatives:** keep the pre-filter and
accept the divergence, naming the class in the spec and proposing a norm change upstream — rejected
by Titusz, conformance to the ISO norm is a hard constraint; measure the residual sweep under both
designs before changing code — rejected as an extra iteration for a change that is strictly cheaper
and provably conformant; post-normalization removal via the runtime's own tables with no vendored
table — rejected, that is the table-version dependence the freeze rule exists to remove.
**Consequence:** exact reference-order conformance and perfect table-version invariance are not
simultaneously achievable, and this ruling makes conformance the exact property and invariance the
*measured* one. With the filter after normalization, a post-16.0 code point that carries a
decomposition under the runtime's tables decomposes before the filter sees it; the canonical/NFD
side of that residual was measured at exactly 1 code point, the NFKC side is unmeasured. Bounding it
is now the job of criterion 4 (reworded — see below). Performance and complexity both improve: the
change deletes one full pass and one `String` allocation from each function and adds one `&&` to a
predicate that already runs per character, so the iai-callgrind Ir budget should move down, not up.
**Context:** interactive session 2026-07-26, unblocking the parked issue from CID iteration 133.
IEP-0003 <https://github.com/iscc/iscc-ieps/blob/main/ieps/iep-0003.md>.

## 2026-07-26 — Criterion 4 asserts a bounded residual, not blanket table equivalence

**Decision:** `specs/rust-core.md` requirement 4 stops claiming that a full-code-space sweep "proves
the freeze-rule implementation output-equivalent to uniform Unicode 16.0.0 tables". It instead
requires the sweep to **enumerate** every code point and sequence class whose output differs from a
uniform-Unicode-16.0.0-tables reference, with the resulting set either empty or explicitly listed
and accepted. It stays a hard gate on any Unicode-table dependency bump. **Why:** as literally
worded the old claim was false, and worse, cheaply "provable" — a per-code-point sweep over all
1,112,032 code points passes while the sequence-adjacency divergences above go undetected, which is
the false-assurance failure mode. Under the category-override ruling the honest residual is narrow
and real (post-16.0 code points carrying a decomposition under the runtime's tables), so the
criterion now has a true property to measure. **Alternatives:** specify a mandatory sequence-aware
sweep (base+Cn+mark, jamo+Cn+jamo, Σ+Cn+cased) as the gate — not rejected on merit and retained as
the recommended shape of the sweep, but the acceptance criterion is the enumerated residual, not the
sweep's shape; delete criterion 4 — rejected, dependency bumps are exactly when this drift would
land unnoticed. **Consequence:** the accepted-divergence paragraph now covers two classes only —
runtimes with non-16.0 tables, and characters assigned between 15.1 and 16.0. The sequence-adjacency
class is **removed** from it, because the override eliminates that divergence rather than accepting
it. The scoped "single-code-point behaviour" qualifier in `docs/unicode.md` can be widened back to
unqualified agreement with CPython 3.14 in the same step. **Context:** interactive session
2026-07-26; supersedes the placeholder recorded 2026-07-26.

## 2026-07-26 — Go skips the Unicode-16 boundary vectors until go1.27

**Decision:** `packages/go` skips the Unicode 16.0 boundary conformance vectors with an explicit
tracking note and issue, rather than vendoring the 15.0→16.0 assigned delta. The vectors are enabled
for Go once go1.27 ships (Unicode 17 tables in stdlib and `x/text`, ~Aug 2026), at which point Go
also needs the 731-range freeze table so post-16.0 characters are still removed. **Why:**
`packages/go` is an independent pure-Go implementation on Unicode 15.0 tables (`x/text` gates its
17.0 tables behind `//go:build go1.27`), so it cannot pass the vectors today. go1.27 is weeks out,
which makes a vendored 5,813-code-point delta table throwaway code with its own generator and drift
surface. **Alternatives:** vendor the 15.0→16.0 assigned delta with 16.0 categories now — rejected
on throwaway cost; block criterion 3 until go1.27 — rejected, it would park the other ten bindings
behind Go for no reason. **Consequence:** criterion 3 can close for 10 of 11 bindings; Go carries a
documented, dated gap. The freeze table is needed in Go either way — go1.27 fixes Go's *missing*
16.0 knowledge, not its need to strip post-16.0 characters. **Context:** interactive session
2026-07-26, ruling on the parked point 3 of the Unicode issue.
