# Decision Records

Log of judgment calls that a future reader could not reconstruct from the code alone: deviations
from the iscc-core reference or a spec, design trade-offs between real alternatives, accepted
performance regressions, justified suppressions, API-shape choices. Written by the review agent when
approving such a call; human entries welcome. Distinct from learnings.md: learnings capture
operational gotchas, decisions capture design rationale.

**A recorded decision is binding.** CID agents must not re-open, re-derive or re-litigate a
trade-off settled here. Superseding one takes a new fact and a new dated entry that says what
changed.

**Size.** One entry per review, at most 12 lines. The file has a 400-line budget; `tools/cid.py`
rotates the oldest entries into `decisions-archive.md` when it is exceeded (oldest-first, no
judgment applied), so entries are written to be read now, not to be permanent. Rotation does not
retire a ruling: archived **titles** stay in every agent's context alongside the live ones, and an
archived decision binds exactly as much as one still in this file. Only the bodies leave.

Entry format:

```markdown
## <YYYY-MM-DD> — <title>

**Decision:** <what was decided>
**Why:** <the reasoning, including constraints that forced it>
**Alternatives:** <what was rejected and why>
**Context:** <iteration N / commit sha / discussion reference>
```

<!-- Append entries below this line -->

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

## 2026-07-26 — Declared Unicode version stays 16.0.0, backported rather than lowered to 15.1.0

**Decision:** the declared Unicode data version remains **16.0.0**. Lowering it to 15.1.0 to
preserve CPython ≤ 3.13 output was proposed and **rejected by Titusz**. The upstream `iscc-core` fix
backports 16.0.0 to older Pythons (`unicodedata2==16.0.0` for `python_version < '3.14'`), converging
the whole supported range on one behaviour rather than pulling 3.14 back. **Why:** freezing upstream
is a behavioural break for *someone* no matter which version is chosen, so "preserve the installed
base" is not decisive. Measured facts behind the call: `iscc-core` declares
`requires-python = ">=3.9,<4.0"`, which spans **four** Unicode versions (3.9/3.10 → 13.0, 3.11 →
14.0, 3.12 → 15.0, 3.13 → 15.1, 3.14 → 16.0), with deltas of +838 / +4,489 / +627 / +5,185 newly
retained code points. So the 3.14 break is the fifth instance of an ongoing non-determinism, not a
new regression, and no freeze point restores compatibility with everything. Two further points
decided it: the 15.1 → 16.0 delta contains **6 living scripts** (Gurung Khema, Kirat Rai, Todhri, Ol
Onal, Garay, Sunuwar), and freezing at 15.1 would strip them entirely — so any document written
wholly in one of those scripts collapses to `""` and all such documents collide on the same
degenerate Text-Code, a correctness hazard rather than an aesthetic one; and the 3.13 cohort being
protected is small in practice, since Unicode 16 shipped 2024-09 and CPython 3.14 shipped 2025-10,
so little real content containing the 16.0 additions has been ISCC'd on 3.13. A large *code point*
count was being weighted as if it were a large *content* count. **Alternatives:** freeze at 15.1.0 —
rejected per above, though it does dominate 16.0.0 against every historical cohort (divergence 5,954
/ 5,116 / 627 / 0 / 5,185 by CPython version, vs 11,139 / 10,301 / 5,812 / 5,185 / 0 for 16.0.0);
freeze at 15.1.0 *and* pin the normalization crate to matching tables — rejected, it gives up free
upgrades of table dependencies for a residual the sentinel design (below) eliminates anyway; add a
`unicode_version` compatibility option so historical ISCCs stay reproducible — rejected, ISO
conformance requires a single normative behaviour and it is exactly the API complexity Titusz ruled
out. **Consequence:** ISCCs minted on CPython ≤ 3.13 for text containing Unicode 16.0 additions are
not reproducible, by decision rather than by accident. Both `iscc-lib` and the upstream proposal
declare 16.0.0. **Context:** interactive session 2026-07-26, after Titusz pushed back on the 15.1.0
proposal.

## 2026-07-26 — The freeze rule maps unassigned code points to a noncharacter sentinel (SUPERSEDES the category override)

**Decision:** **supersedes the category-override decision recorded earlier on 2026-07-26.** Code
points unassigned in Unicode 16.0.0 are **mapped to the noncharacter `U+FFFF` before
normalization**, not deleted and not merely reclassified. The category filter in `text_clean` /
`text_collapse` stays **exactly as the reference defines it** — no predicate change, no
`is_unassigned_in_unicode16` call at the filter step — because `U+FFFF` is category `Cn` and the
existing category-`C` filter already removes it. In Rust this is a one-token change from the
iteration-133 code, inside the same fused iterator: `.filter(|&c| !is_unassigned_in_unicode16(c))`
becomes `.map(|c| if is_unassigned_in_unicode16(c) { '\u{FFFF}' } else { c })`. **Why:** the
category override was conformant on single code points but **not** on sequences. Measured against a
uniform-Unicode-16.0-tables reference on a runtime with Unicode 17.0 normalization tables (what
`unicode-normalization` 0.1.25 ships), the override differed on 1 of 1,114,112 single code points
and **10 of 140 sequence cases**. The failure mode is worse than the count: `U+A7F1` is unassigned
in 16.0 but has a *compatibility decomposition to `S`* under 17.0, so it dissolves during NFKC
before the filter sees it and **injects a spurious letter** — `e U+A7F1 U+0301` produced
`U+0065 U+015A` (`eŚ`) where the reference gives `U+0065 U+0301`. The sentinel fixes this because
`U+FFFF` is a noncharacter: category `Cn`, `ccc = 0`, and no decomposition — **permanently**, under
Unicode's Noncharacter stability policy, so it can never gain an assignment or a decomposition in
any future version. It holds the original character's position through normalization, so
composition-blocking and `Final_Sigma` context match the reference exactly, and it is then removed
by the reference's own filter. If the input already contains `U+FFFF` it is itself unassigned in
16.0, so it maps to the sentinel and is stripped — which is what the reference does with it too. No
ambiguity. **Verified before the decision** — three designs swept against a uniform-16.0 reference
built from `unicodedata2==16.0.0`, evaluated on `unicodedata2==17.0.0`:

| design                         | single code points differing | sequence cases differing |
| ------------------------------ | ---------------------------- | ------------------------ |
| pre-filter (iteration 133)     | 0 of 1,114,112               | **42 of 140**            |
| category override (superseded) | **1** (`U+A7F1`)             | **10 of 140**            |
| sentinel map (this decision)   | **0**                        | **0**                    |

The pre-filter's 0-of-1,114,112 on single code points is precisely the false assurance that
motivated rewording criterion 4 — it passes a per-code-point sweep while failing 42 sequence cases.
**Alternatives:** category override — superseded, see the table; pre-strip only the residual set
before normalizing — rejected, it reintroduces the adjacency bug for exactly those code points; pin
`unicode-normalization` to 16.0-era tables so nothing post-16.0 can decompose — rejected, it gives
up requirement 2's free upgrades and needs an old crate tracked indefinitely; a private-use code
point instead of a noncharacter — rejected, private-use characters are category `Co` (not stripped
by the `C` filter... they are, but they are legitimately usable in input and could collide), whereas
noncharacters are permanently reserved and are the semantically correct choice. **Consequence:** the
trade-off asserted in the superseded entry — "exact reference-order conformance and perfect
table-version invariance are not simultaneously achievable" — **is false**, and the spec text
derived from it is corrected. The sentinel achieves both: residual **0**, and criterion 4 can assert
exact equivalence to uniform Unicode 16.0.0 tables over single code points *and* sequence classes.
Performance is neutral versus iteration 133 (a `map` replacing a `filter` in the same fused
normalization iterator — same passes, same allocations) and strictly simpler than the override,
which would have added a per-character test to the filter predicate. The vendored 731-range table
and `scripts/gen_unicode16_unassigned.py` are unchanged. Implementations must confirm their
normalization library passes `U+FFFF` through unchanged (required of any conformant implementation,
but cheap to assert in a test). **Context:** interactive session 2026-07-26. Supersedes "The Unicode
16.0.0 freeze rule is a category override, not a pre-normalization filter" (same day) and the
criterion-4 entry's residual framing.

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

## 2026-07-26 — Sentinel conformance accepted on sequence evidence; criterion-4 sweep still owed

**Decision:** The `U+FFFF` sentinel map (iteration 148) is accepted as conformant, and
`docs/unicode.md` is allowed to state unqualified agreement with CPython 3.14, on the strength of a
**sequence-class** differential rather than the full criterion-4 sweep, which remains a separate
step. **Why:** the class the pre-filter broke is adjacency-sensitive sequences, and that is exactly
what was measured in review — 127 Unicode-16.0-unassigned code points × 10 contexts (1,270 cases)
compared against a `unicodedata2==16.0.0` reimplementation of `iscc-core`'s `text_clean` /
`text_collapse`: the sentinel scored **0 mismatches**, the deleted iteration-133 filter **504**
(exactly the four adjacency-sensitive contexts: canonical composition, Hangul jamo, `Final_Sigma`,
diaeresis). Analytically the equivalence is total, not statistical: any code point unassigned in
16.0 and `U+FFFF` are both `Cn`, `ccc = 0`, undecomposable, uncased and not `Case_Ignorable`, so
they are indistinguishable to every step of both pipelines. **Alternatives:** hold the docs claim
qualified until the 1,112,064-value sweep lands — rejected, the qualifier existed specifically to
hedge the sequence class that is now proven, and leaving it would document a defect that no longer
exists; verdict NEEDS_WORK pending the sweep — rejected, next.md scoped the sweep out deliberately
and the sweep's remaining value is *regression* protection, not initial proof. **Consequence:** the
criterion-4 runnable check must still cover **both** single code points and sequence classes; if it
ever contradicts this entry, this entry loses. **Context:** iteration 148 review, commit `7acf0fa`;
probe harness not committed (throwaway `tests/tmp_probe.rs`, removed after the run).

## 2026-07-26 — The Go boundary suite is left to red at the go1.27 bump, not version-gated

**Decision:** `packages/go/unicode_boundary_test.go` keeps an **unconditional** three-entry skip map
(the cases Go's Unicode 15.0 tables cannot pass) and is deliberately *not* build-tag-gated for
go1.27, even though bumping the toolchain will turn 5 currently-green cases red. **Why:**
`packages/go` implements no freeze rule, so it passes the two post-16.0 vectors (`U+20C1`, `U+A7F1`)
only by accident — its 15.0 tables classify them `Cn` and the category-`C` filter drops them. Under
go1.27 (`x/text`'s `tables17.0.0.go` is `//go:build go1.27`; the stdlib tables move too) both become
assigned and the vectors fail, as does the sequence case
`text_clean/test_0006_seq_ua7f1_no_decomposition_leak`. That red is precisely the signal that the
731-range freeze table must land in `packages/go/utils.go`, which `decisions.md` 2026-07-26 ("Go
skips the Unicode-16 boundary vectors until go1.27") already says Go needs regardless of the
toolchain. **Alternatives:** version-gate the skip map with a `go1.27` build tag (raised as a P1 by
the Codex review of iteration 150) — rejected: it converts a designed failure signal into silent
pre-emptive test skipping, and would let a toolchain bump land with Go quietly non-conformant;
vendor the 15.0→16.0 delta now — already rejected by the 2026-07-26 ruling on throwaway-cost
grounds. **Consequence:** the go1.27 bump is a *bundled* step — toolchain + freeze table + skip-map
removal in one commit. Risk of a surprise is low: `.github/workflows/ci.yml` pins Go via
`go-version-file: packages/go/go.mod`, so nothing moves without editing the `go 1.26.1` directive. A
checklist is recorded under the Unicode issue in `issues.md`. **Context:** iteration 150 review,
commit `d12ceb1`; `x/text` build tags read from the local module cache (`v0.40.0`).

## 2026-07-26 — Binding boundary suites link the canonical fixture instead of vendoring a copy

**Decision:** The C# and Kotlin boundary suites read `crates/iscc-lib/tests/unicode_boundary.json`
through a build-config indirection — an MSBuild
`<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json" Link="testdata\unicode_boundary.json">`
item for `.NET` and an `iscc.fixtureDir` system property for Gradle — even though both packages
already vendor a tracked *copy* of the sibling `data.json` in the same test tree. The Kotlin `Test`
task additionally declares the fixture with
`inputs.file(…).withPathSensitivity(PathSensitivity.NONE)`. **Why:** every tracked copy is a
byte-identity liability that `tests/test_vendored_fixtures.py` must police, and eight surfaces × one
more copy is eight more chances to drift; linking removes the failure mode instead of gating it.
Nothing published depends on the copy — test sources ship in neither the NuGet package nor the Maven
artifact, so self-containment buys nothing here. The path sensitivity is `NONE` because the absolute
fixture path differs between checkouts and must not by itself invalidate the task. **Alternatives:**
vendor a copy in each package and register it in `VENDORED_COPIES`, matching the existing
`data.json` precedent — rejected on drift-surface grounds, and the precedent is itself historical
(`data.json` predates the drift gate); leave the Gradle input undeclared and rely on
`cleanTest test` by convention — rejected, a convention that must be remembered is not a gate, and
review measured the failure: after one green run, mutating the fixture left `./gradlew test`
`UP-TO-DATE` and silently skipped all 13 boundary tests. **Consequence:** `data.json` and
`unicode_boundary.json` are handled asymmetrically inside `packages/dotnet` and `packages/kotlin`;
that is deliberate, not an oversight. Swift remains the one surface expected to need a real vendored
copy (SwiftPM resource bundling), and it must be registered in `VENDORED_COPIES`. **Context:**
iteration 154 (`84ce2f1`) plus the review fix; hazard independently reported by the Codex review and
confirmed by a three-run Gradle probe.

## 2026-07-27 — `Final_Sigma` is frozen by pre-substitution, not by vendoring the case-mapping table

**Decision:** `text_collapse` keeps delegating the bulk of lowercasing to `str::to_lowercase()`.
Only the *conditional* `Final_Sigma` mapping is taken over: `to_lowercase_unicode16` walks the
(sentinel-mapped, NFD-normalized) string, replaces each `U+03A3` with `U+03C2`/`U+03C3` decided from
a vendored Unicode 16.0.0 `Cased` / `Case_Ignorable` table, and only then calls std — which can
never see a `U+03A3` and so can never run its own 17.0-table sigma branch. The vendored `Cased`
table deliberately stores `Cased` **minus** `Case_Ignorable` (152 ranges / 4,311 code points), which
is not the UCD-defined `Cased` set. **Why:** bare `str::to_lowercase()` reads the *compiler's*
tables (rustc 1.97 ships Unicode 17.0, which moved `U+0295` from `Ll` to `Lo`), so hash output was a
function of the rustc version — measured as 3 divergent rows at exactly `U+0295` in a 17,793,024-
comparison differential against `iscc-core` on uniform 16.0.0 tables. Pre-substitution is the
smallest change that removes that dependence: the freeze is scoped to the one property that actually
diverges, and every unconditional mapping (e.g. `U+0130`) stays with std. The subtracted
`Case_Ignorable` set is what the two behavioural probes in `scripts/gen_unicode16_case.py` can
measure, and the restriction is unobservable because the `Final_Sigma` scan skips case-ignorable
code points before it ever tests casedness — in CPython's `handle_capital_sigma` and in the port
alike. **Alternatives:** vendor the full Unicode 16.0.0 lowercase *mapping* table and stop using
`str::to_lowercase()` entirely — rejected as a large, mostly-dead table for a divergence that does
not exist today (the same sweep shows 0 unconditional-mapping divergences across all 1,112,064
scalars); pin the rustc toolchain — rejected, it makes a build-tool version part of the conformance
contract and does nothing for downstreams building from source. **Consequence:** the *unconditional*
half of lowercasing is still supplied by whatever Unicode tables rustc ships. That is an accepted,
currently-zero residual whose only guard is the differential sweep, which is exactly why criterion 4
of the Unicode issue (wiring the sweep in as a permanent, fail-closed check) must land next. A
future rustc that changes a non-sigma lowercase mapping would silently change output until then.
**Context:** iteration 156 (`a7e84c8`); sweep re-run independently at review, 0 divergences; tables
independently cross-checked against CPython 3.14 category data (`Lu∪Ll∪Lt ⊆ Cased`,
`Mn∪Me∪Cf∪Lm∪Sk ⊆ Case_Ignorable`, residuals exactly `Other_Uppercase`/`Other_Lowercase` and the 17
UAX #29 MidLetter/MidNumLet/Single_Quote code points).

## 2026-07-27 — The Unicode sweep gate trusts an mtime freshness guard, not a build-input hash

**Decision:** `scripts/unicode_sweep.py` decides whether the extension it measures is current by
comparing `_lowlevel.abi3.so`'s mtime against the newest `*.rs` under `crates/iscc-lib/src` and
`crates/iscc-py/src`, and nothing else. `Cargo.toml`, `Cargo.lock`, the rustc version and every
transitive Unicode-table crate are outside the guard; so is a missing source directory
(`max(..., default=0.0)` passes vacuously). The authoritative execution paths — the `unicode-sweep`
CI job and `mise run unicode:sweep` — both rebuild unconditionally before sweeping, so the guard
only has to catch the local "edited Rust, then ran the script directly" case. **Why:** the guard is
a convenience backstop on a path that is not the gate. Making it complete means either hashing the
full cargo build-input closure (duplicating cargo's own fingerprint logic) or shelling out to
`rustc -vV` plus `cargo metadata` on every run — real complexity to protect an invocation the docs
do not recommend. **Alternatives:** widen the mtime set to `Cargo.toml`/`Cargo.lock` — cheap, but
still blind to a toolchain-only bump, so it would buy a false sense of completeness; drop the guard
and always rebuild inside the script — rejected, it makes the script un-runnable without a ~21 s
maturin build and removes the fast fail-fast signal. **Consequence:** a `cargo update` or rustc
upgrade that touches no `.rs` file can produce a false green from a bare
`uv run scripts/unicode_sweep.py`. That is exactly the upgrade scenario the sweep exists to
validate, so the operating rule is: **always re-run the sweep through `mise run unicode:sweep`**,
never the script directly. Recorded in learnings.md and filed for hardening in issues.md.
**Context:** iteration 157 (`186287e`); blind spot flagged by the advance agent's handoff,
independently reported by the Codex review, and reproduced at review (a missing source directory
also passes the guard vacuously).

## 2026-07-27 — The sweep gate's rebuild precondition is a trusted caller assertion, not an observation

**Decision:** `scripts/unicode_sweep.py` refuses to run unless its `argv` carries `--rebuilt`
(`check_rebuilt`, the first statement of `main`). Only two callers pass it: the `unicode:sweep` mise
task and the `unicode-sweep` CI step, each immediately after an unconditional
`maturin develop --release`. The flag asserts nothing the script verifies — a human who exports it
without rebuilding is trusted — and the mtime `check_extension_fresh` observation is deliberately
kept behind it as cheap redundancy for the "edited a `.rs`, forgot to rebuild" case. **Why:** the
2026-07-27 mtime entry above left the operating rule ("always run through `mise run unicode:sweep`")
as convention only, which is unenforced by construction; a `cargo update` or rustc bump touches no
`.rs` file, so a bare invocation reported a false green on exactly the upgrade the gate exists to
validate. Turning the convention into a refusal costs one flag and closes the hole for every path
that matters, because the two authoritative paths rebuild first by construction. **Alternatives:**
widen the mtime set to `Cargo.toml`/`Cargo.lock`/`rust-toolchain*` — still blind to a toolchain-only
bump, so it buys false completeness (already rejected above); embed `rustc -vV` plus a dependency
fingerprint in the extension and compare it — the only *observing* fix, rejected because it adds an
unbound public symbol to the Python surface and breaks the 32-symbol Tier 1 story for a hazard the
refusal already covers; make the script shell out to `maturin` itself — rejected, it removes the ~2
s fail-fast signal and makes the script un-runnable without a ~21 s build. **Consequence:** the gate
now trades a *silent* failure mode for a *loud* one — the residual (`--rebuilt` passed without a
rebuild) is a deliberate act rather than an accident, and is stated in the `check_rebuilt`
docstring. `sweep()` also stopped retaining one record per divergence: it counts every divergence
but keeps at most `MAX_REPORTED_DIVERGENCES` (20) samples, so a broad regression prints diagnostics
instead of OOM-killing CI. The success line `TOTAL 17793024 comparisons, 0 divergences` is unchanged
and stays byte-frozen. **Context:** iteration 158 (`aedb61e`); both blind spots were filed by the
iteration-157 review and independently reported by that iteration's Codex review. Verified at review
by a bare-invocation probe (exit 1, empty stdout), two near-miss flag forms, a full green sweep, and
an in-process `main()` run over 30 scalars with a wrong oracle (20 sample lines +
`showing first 20 of 480 divergences` + the exact TOTAL line, return code 1).

## 2026-07-27 — A vector fixture reaches a JSON-less surface as a generated, tracked source file

**Decision:** the C FFI conformance surface consumes the canonical
`crates/iscc-lib/tests/unicode_boundary.json` through a checked-in, generated C header
(`crates/iscc-ffi/tests/unicode_boundary_vectors.h`, rendered by the PEP 723
`scripts/gen_ffi_boundary_vectors.py`) rather than by parsing the fixture at test time. The header
is deliberately **not** registered in the `VENDORED_COPIES` table of
`tests/test_vendored_fixtures.py`; its equivalent guarantee is a pytest anchor asserting
`render(fixture) == tracked_header` byte for byte. **Why:** `test_iscc.c` links only libc and the
FFI `cdylib` — adding a JSON reader means either a new third-party C dependency in the CI gcc line
(`gcc … -liscc_ffi -lpthread -ldl -lm`, which the C FFI job runs verbatim) or a hand-rolled parser
whose bugs would be indistinguishable from conformance failures. Generation moves the parsing to
Python, where it is already exercised, and leaves C with a plain `static const` array. Placing the
header beside its includer makes a quoted `#include` resolve with no new `-I`, so the CI job is
untouched. **Alternatives:** hand-pin the 12 expected strings in `test_iscc.c` — rejected, it is the
silent-drift failure mode the whole propagation effort exists to prevent (the fixture is the single
source of truth for 9 surfaces); hand-roll a JSON parser in C — rejected, more untested code than
the thing under test; register the header in `VENDORED_COPIES` — rejected, that gate asserts
*byte-identity with the fixture* and discovers copies by basename, so a derived artifact cannot
satisfy it; generate the header at build time instead of tracking it — rejected, it would put a
Python dependency in the C job and remove the reviewable diff. **Consequence:** two rules now travel
with any future JSON-less surface (C++ is the next candidate): the generated file must be pure
ASCII, LF-only, one trailing newline — otherwise the prek hygiene hooks rewrite it and red the no-op
gate — and non-ASCII UTF-8 must be escaped as 3-digit octal, never `\x`, because C hex escapes are
greedy and unbounded. **Context:** iteration 159 (`ea69169`), criterion 3 of the Unicode issue, 9 of
11 surfaces. Verified at review by decoding the tracked header's octal escapes back to code points
and diffing against the fixture, and by five mutations (expected value, dropped case, version macro,
hand-edited header, fixture vector added without regeneration) — every one reds.

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
