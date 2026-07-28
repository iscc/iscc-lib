# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files.

**This is a defect tracker, not a work queue.** An entry states the remaining problem and how to
tell it is fixed, in **40 lines or fewer**, and is deleted once nothing is left to fix. Never record
progress inside an entry — no per-iteration checklists, no "✅ slice N done", no plan of remaining
sub-steps. Git history records what was done; a step queue here removes the gap analysis that
define-next exists to perform. If only part of a problem remains, rewrite the entry to describe that
smaller problem. Design rationale belongs in `decisions.md`, operational gotchas in `learnings.md`,
and user-facing behaviour in `docs/`.

<!-- Add issues below this line -->

## Dependency review and refresh across the project `normal` [human]

Planned for the **v0.6.0** release. No automated dependency updates are configured (no
Dependabot/Renovate), so manifests drift between releases.

Three items remain. `uniffi` 0.32 and `criterion` 0.8 are human-gated: uniffi needs the Swift +
Kotlin bindings regenerated and re-verified (no Swift toolchain assumption may be baked in), and
criterion needs rustc 1.86 > the declared `rust-version = "1.85"`, i.e. an MSRV policy call. The
`release.yml` action refresh is CID-doable on static evidence (per the 2026-07-25 decision) but is
never exercised by a CI run, so it must lean on `scripts/check_release_workflow.py` and
`--check-action-inputs` rather than on a green pipeline.

**Constraints that still bind:** wheels stay `abi3-py310`; PyO3 bumps only together with
re-verifying `gil_used = true` semantics and the `py.detach` call sites; `rb_sys` in `Gemfile.lock`
must match the `oxidize-rb/actions/cross-gem` Docker image tag (exact-pinned `0.9.123`);
quality-gate CI tool pins bump together with their baselines. Never run `ruff@0.16 check --fix .` —
it deletes load-bearing `# noqa` directives. All quality gates and conformance vectors must pass on
the refreshed set.

**Known upstream wart (not actionable):** the `proc-macro-error2 v2.0.1` future-incompat warning on
every `cargo test` / `cargo bench` comes from `iai-callgrind-macros`, is dev-only, and has no fixed
release. Re-check when bumping `iai-callgrind` (which must stay in lockstep with the CI-installed
`iai-callgrind-runner`).

**Spec:** `.claude/context/specs/ci-cd.md` → "Dependency Freshness"

## JNI `build_byte_array` re-implements a non-deprecated upstream helper `normal` [review]

`crates/iscc-jni/src/lib.rs` grew a hand-rolled `build_byte_array` at iteration 168 because next.md
listed `Env::byte_array_from_slice` as deprecated in jni 0.22. It is **not**: jni-0.22.4
`src/env.rs:3349-3360` carries no `#[deprecated]` attribute (the note that was read belongs to
`set_object_array_element` two functions above), and its body is exactly `JByteArray::new` +
`set_region` over a *transmuted* `&[i8]` — no allocation.

The local helper instead collects a fresh `Vec<i8>` per call, adding one allocation and one full
copy to every returned `byte[]`. Worst case is `algCdcChunks`, which calls it once per chunk (≈1000
allocations per MiB of input); `isccDecode`, `algSimhash`, `algMinhash256` and `softHashVideoV0`
return digest-sized arrays where the cost is negligible.

Fix: call `env.byte_array_from_slice(bytes)` at the four sites (or reduce the helper to a delegating
one-liner) and update the two `crates/iscc-jni/CLAUDE.md` rows that teach `build_byte_array`.
Resolved when `Vec<i8>` no longer appears in the crate,
`mvn clean test -f crates/iscc-jni/java/pom.xml` still reports 82/82, and
`cargo clippy -p iscc-jni --all-targets -- -D warnings` exits 0.

## Seven of the 33 JNI natives have no Java test `normal` [review]

`IsccLib.java` declares 33 native methods; `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/`
never calls seven of them — `conformanceSelftest`, `encodeBase64`, `textRemoveNewlines`,
`isccDecompose`, `algSimhash`, `algMinhash256`, `softHashVideoV0`. The jni 0.21→0.22 migration
rewrote all 33 signatures, so a green 82-test suite proved only 26 of them; the seven were verified
by hand at review time and work, but nothing guards them.

Two of the gaps are structural, not cosmetic: `conformanceSelftest` is the only `jboolean` return in
the crate (jni-sys 0.4 changed that alias to Rust `bool`), and `isccDecompose` is one of three
functions returning a JVM-allocated array whose *element class* Java never checks — HotSpot does not
type-check native return values, so a wrong array class survives element reads. Assert
`getClass().getName()` (`[Ljava.lang.String;`, `[[B`) in at least one test.

Resolved when every native declared in `IsccLib.java` is called by at least one JUnit test and
`mvn clean test -f crates/iscc-jni/java/pom.xml` is green.

## Binding docs teach APIs the code no longer uses `normal` [review]

Two doc surfaces drifted during the dependency refresh and now teach removed APIs to agents:

- `crates/iscc-rb/CLAUDE.md:108` still presents `RString::from_slice` as the way to copy a slice
    into a Ruby string; `crates/iscc-rb/src/lib.rs` has used `ruby.str_from_slice` since the magnus
    0.8 bump (iteration 167).
- `.claude/context/specs/java-bindings.md` says the binding uses "the `jni` crate (v0.21)" (line 22)
    and describes `src/lib.rs` as "~1060 lines" in two places; it is jni 0.22 and 1168 lines since
    iteration 168.

**HUMAN REVIEW REQUESTED**: the second bullet edits a human-owned spec file. The Ruby doc fix needs
no authorization. Resolved when both files match the code.

**Spec:** `.claude/context/specs/java-bindings.md` → "Why JNI (not JNA or Panama FFI)"

## go1.27 bump reds the Go boundary suite unless the freeze table lands with it `normal` [review]

**Trigger-on-bump record — do not scope a step until go1.27 is available (~Aug 2026).**

Bumping `packages/go/go.mod` past `go 1.26.1` **must** land the 731-range freeze table in
`packages/go/utils.go` in the *same* step. Go currently passes the two post-16.0 boundary vectors
for the wrong reason: its Unicode 15.0 tables classify `U+20C1` and `U+A7F1` as `Cn`, so the
category-`C` filter drops them, coinciding with the freeze rule's output by accident. Under go1.27
both become assigned (`x/text`'s `tables17.0.0.go` is `//go:build go1.27`; the stdlib `unicode`
tables move too), so 5 currently-green cases flip red — `text_clean` + `text_collapse` for `U+20C1`
and `U+A7F1`, plus `text_clean/…_seq_ua7f1_no_decomposition_leak` — while the 3 documented skips
become unnecessary.

Do **not** version-gate the skip map to hide this; the red is the intended signal.
`go-version-file: packages/go/go.mod` pins CI to the go.mod directive, so nothing flips without a
deliberate bump.

## Update the upstream iscc-core thread with the sentinel mechanism `low` [human]

**Human-only — CID must not act on this.** The Unicode data version work is complete on this side:
the sentinel freeze rule, the fail-closed differential sweep gate, boundary vectors on all 11 native
surfaces plus pure-Go, and user documentation in `docs/unicode.md` all landed.

<https://github.com/iscc/iscc-core/issues/137> still describes the earlier framings. The thread
needs updating to propose the **sentinel** mechanism —
`unicodedata2==16.0.0; python_version < '3.14'` + import shim, map unassigned code points to
`U+FFFF` before `unicodedata.normalize` using the same 731 vendored ranges, leave the category-`C`
filter alone, and add boundary vectors to `data.json`. Both the sequence-delta framing and the
pre-normalization-removal framing should be withdrawn.

Worth stating explicitly upstream: `iscc-core` output has never been deterministic across its
declared `>=3.9,<4.0` range (Unicode 13.0/14.0/15.0/15.1/16.0 by Python version), so this is a
determinism fix and the behavioural break it implies is accepted deliberately. Per ISO 24138 Annex D
the reference implementation is normative, so the `iscc-core` release adopting this settles the
standard's answer.

Close this issue once the thread is updated.

**Upstream:** iscc/iscc-core **Spec:** `.claude/context/specs/rust-core.md` → "Unicode data version
is part of the conformance contract"

## Gate-script remainders deliberately deferred at iteration 146 `low` [review]

Three items left open when the gate blind-spot issues closed, each contingent on something that has
not happened yet — CID must not act on them unprompted:

1. **A zero-resolved run still exits 0.** The condition is *visible*
    (`action-inputs: resolved <R> of <T> action refs`) but not fatal. Flipping the exit code is a
    policy change against `decisions.md` 2026-07-26, which accepted all-skipped-as-green by design.
    **Trigger:** the first `release-workflow` CI run whose log shows a non-zero skip count. **Needs
    Titusz** — it turns third-party availability into a merge blocker.
2. **Job-level `uses:` (reusable-workflow calls) is still unscanned** — only `job.steps[*].uses`.
    `release.yml` has none at HEAD; the `owner/repo/.github/workflows/x.yml@ref` form also needs a
    different URL shape than `action_yml_urls` builds. **Trigger:** a reusable workflow is added to
    any workflow file.
3. **The YAML 1.1 boolean fix is one-sided.** Non-string `with:` keys are now skipped, but the
    *metadata* side is not normalized: an action declaring an input literally named `on`/`off`/
    `yes`/`no` parses as `True`/`False`, so a workflow passing the **quoted** `"on"` would still be
    reported undeclared. No such action exists among the 18 refs. **Trigger:** the false positive
    is actually observed.

## Release core as v1.0.0 (stability commitment) `low` [human]

Human-driven release: cut **v1.0.0** as the first stability-committed release of the lockstep
workspace (per the 1.0.0 decision). This is the one release allowed to break the 0.4.0 API freely;
afterward 1.x is locked under strict SemVer. Drive via the `/release` skill — do NOT let the CID
loop cut this release autonomously.

**Status (2026-06-18):** Titusz decided to **hold** the v1.0.0 cut and stay on 0.4.x for now — land
the CRAP `--fail-above` and `cargo deny` hardening gates first, then flip the `cargo-semver-checks`
gate to enforcing as part of the eventual cut.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Migrate npm publishing to OIDC Trusted Publishing `low` [human]

**DEFERRED by Titusz 2026-07-26 — not for v0.6.0.** The v0.6.0 release stays on `NPM_TOKEN`, which
was rotated (`iscc-lib-ci-2026`, granular `@iscc` scope) and is valid until 2026-09-16. Revisit
before the release after v0.6.0, or sooner if the token expiry becomes a problem again. CID must
**not** prepare the YAML diff — an unused OIDC-shaped diff sitting in the tree would be its own
hazard.

The v0.5.0 release failed both npm publishes because `NPM_TOKEN` had expired (npm caps write-token
expiry at 90 days). npm now supports OIDC Trusted Publishing — the same keyless mechanism already
used for crates.io, PyPI and RubyGems.

**Scope when it lands:** update the two npm publish jobs in `.github/workflows/release.yml` to
publish via OIDC (drop `NODE_AUTH_TOKEN`/`NPM_TOKEN`, rely on the existing `id-token: write` +
`npm publish --provenance`). **Human-gated:** a Trusted Publisher must be configured for each
package on npmjs.com before the token can be removed.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
