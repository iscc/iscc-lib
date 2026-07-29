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

## ISCC-IDv1 is unsupported outside Go, and Go uses superseded names `normal` [human]

GitHub: https://github.com/iscc/iscc-lib/issues/43 — **a v0.6.0 release blocker.**

The core `codec::Version` now has `V0` **and** `V1` (`#[non_exhaustive]`,
`crates/iscc-lib/src/codec.rs:104-106`), so the core generic decode already accepts version 1 — the
low-level `_iscc_decode("ISCC:MAIGHFECJMOPMIAB")` returns `(6, 0, 1, 0, <8 bytes>)`. The remaining
drop-in-compatibility gap is **per-surface enum wrappers** that predate V1: the Python `VS` IntEnum
(`crates/iscc-py/python/iscc_lib/__init__.py:82-85`) still defines only `V0`, so
`iscc_lib.iscc_decode` raises `1 is not a valid VS`. Every surface with its own version enum needs
the same widening plus a round-trip test (`iscc_decode(gen_iscc_id_v1(...)["iscc"])`).

Separately, the minting function `gen_iscc_id_v1` is missing everywhere, and `packages/go` carries
the only implementation, under a superseded name and reversed parameter order (`EncodeIsccID`),
alongside a `DecodeIsccID` / `IsccIDv1Result` pair that must be **deleted** rather than renamed —
`iscc-core` has no IDv1 decoder and the generic path covers it.

Canonical definition, validation rules, codec changes, the `#[non_exhaustive]` requirement, the IDv0
exclusion, test placement and the full "Verified when" list are in
`.claude/context/specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)".

Two repo-wide defects follow from the change:

- The Tier 1 symbol count reads 32 across `docs/`, `notes/`, per-crate and per-package `CLAUDE.md`
    and `README.md`, and in three non-Markdown sources: `crates/iscc-uniffi/src/lib.rs:3`,
    `crates/iscc-rb/src/lib.rs:7` (`Symbols (32 of 32):`, whose enumerated list also needs the new
    name) and `.claude/agents/advance.md:146`, which wraps as `Tier 1 (32\nsymbols)` so a
    `grep "32 symbols"` reports it clean. Four sites were already stale beforehand
    (`crates/iscc-wasm/CLAUDE.md`, `docs/java-api.md`, `notes/00-overview.md`, an archive file).
    `specs/*.md` and `target.md` are already at 33; historical records (`iterations.jsonl`,
    `state.md`, agent memory) are out of scope — no role may rewrite them.
- Hand-maintained per-symbol API docs lack the `gen_iscc_id_v1` entry and the field-extraction
    recipe: `docs/rust-api.md`, `docs/java-api.md`, `docs/ruby-api.md`, `docs/c-ffi-api.md` and the
    11 `docs/howto/*.md` pages. `docs/api.md` is mkdocstrings autodoc. A new docs page would drag in
    the four-list parity check in `scripts/check_docs_nav.py`.

Resolved when the generic decode path accepts Version 1 on all 11 surfaces, `gen_iscc_id_v1` exists
under the canonical name, Go's `DecodeIsccID` / `IsccIDv1Result` are gone, the pytest differential
test against `iscc_core` passes, and no shipped-artifact Tier 1 count still reads 32.

**Spec:** `.claude/context/specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)"

## Codec input cleaning diverges from iscc-core `iscc_clean` `normal` [review]

The reference routes codec input through `iscc_clean` (`reference/iscc-core/iscc_core/codec.py:644`,
reached via `normalize_multiformat` at `:363`), which strips dashes and surrounding whitespace and
accepts a case-insensitive `iscc:` scheme. Our codec does none of this, so documented-valid inputs
raise. Verified live against `iscc_core` 1.3.0:

- `ISCC:KACY-PXW4-45FT-…` → reference returns 4 units, ours raises `invalid symbol at 4`
- `"  ISCC:AAAYPXW445FTYNJ3  "` → reference returns 1 unit, ours raises `invalid length at 24`
- `"iscc:AAAYPXW445FTYNJ3"` → reference returns 1 unit, ours raises `invalid symbol at 4`

The reference docstring (`codec.py:381-382`) explicitly blesses the hyphen-separated form, so this
is an unintended drop-in-compatibility gap, not a deliberate tightening — no decision record exists
for it.

Four Rust sites clean ad-hoc: `codec.rs:485` (`iscc_decompose`), `lib.rs:810` (`gen_mixed_code_v0`),
`lib.rs:896` (`gen_iscc_code_v0`) and `lib.rs:222` (`iscc_normalize`, feeding `iscc_decode`). The
fourth strips the prefix and dashes but not whitespace, and rejects a lowercase scheme — so
`iscc_decode` matches only the dash form and still diverges on the other two. `packages/go` is a
hand-written port with the same four gaps at `codec.go:482`, `code_content_mixed.go:23`,
`code_iscc.go:27` and `codec.go:592` (`isccNormalize`) — two parallel fixes, not one.

Porting subtlety: in the reference's no-colon branch (`codec.py:656-661`) dashes are stripped **only
when the string is not multibase-prefixed**. An unconditional `.replace('-', "")` would corrupt
`f`/`b`/`v`/`z`/`u`-prefixed input. Multiformat decoding itself stays a documented non-goal
(`crates/iscc-lib/src/lib.rs:216-219`) — file it separately if ever wanted.

Resolved when a shared private `iscc_clean` helper is routed through all four Rust sites and the
four Go sites, and a differential test covers the dash, whitespace and lowercase-scheme forms for
`iscc_decompose`, `iscc_decode`, `gen_iscc_code_v0` and `gen_mixed_code_v0`.

**Spec:** `.claude/context/specs/rust-core.md` → "Codec Operations"

## iscc-core mints 88-bit ISCC-IDv0 codes it cannot decode `low` [review]

`iscc_core.encode_length` accepts an 88-bit `ID` body but the `LN` enum
(`reference/iscc-core/iscc_core/constants.py:166-178`) has 64/72/80/96 and no 88, so the reference
cannot decode what its own generator produced. Reproduced on `iscc_core` 1.3.0:
`gen_iscc_id_v0(code, 1, wallet, uc=20000)` → `ISCC:MEBUAGCA47NDLBDXUCOAC`, and `iscc_decode` on
that string raises `ValueError: 88 is not a valid LN`.

iscc-lib decodes it fine (`decode_length(MainType::Id, …)` is `length * 8 + 64`,
`crates/iscc-lib/src/codec.rs:370`), so we are the more correct side. **No local change** — matching
the reference here would replicate a defect, and ISCC-IDv0 is permanently out of scope (deprecated,
never in ISO 24138, no backward compatibility owed). Recorded only so a future conformance sweep
does not "fix" our decoder to match.

Resolved when filed upstream and the outcome noted.

**Upstream:** iscc/iscc-core

## iai text benchmarks are ASCII-only, so the perf gate is blind to the Unicode freeze `normal` [review]

`is_unassigned_in_unicode16` (`crates/iscc-lib/src/utils.rs`) runs a binary search over 731 ranges
for every character in `text_clean` / `text_collapse`, with a single fast path:
`cp < UNASSIGNED_RANGES[0].0` (`0x0378`). Every iai text input is pure ASCII — `synthetic_text()` in
`crates/iscc-lib/benches/iai_benches.rs` repeats `"The quick brown fox jumps over the lazy dog. "`,
and `bench_meta_code` uses `"Die Unendliche Geschichte"` — so `bench_text_code.chars_1000` and
`bench_meta_code.*` never leave the fast path. The green `Perf (iai-callgrind)` gate is therefore no
evidence that the freeze rule is cost-free for Greek, Cyrillic, CJK or Indic text, which pays ~10
comparisons per character. `to_lowercase_unicode16`'s `Vec<char>` + `Vec<bool>` allocations are
unmeasured for the same reason (they only run when the input contains `U+03A3`).

Resolved when at least one non-ASCII text case exists in `iai_benches.rs` (e.g. a CJK and a Greek
`chars_1000` variant) with its entry in `.iai-baseline.json`, regenerated via
`mise run bench:iai:baseline` on a valgrind-capable machine.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

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

**Status (2026-07-28):** Titusz reaffirmed **not yet** — the project is working toward **v0.6.0**,
and v1.0.0 comes after it. The two hardening gates the 2026-06-18 hold waited on have both landed
and are enforcing (`ci.yml` → `cargo crap … --fail-regression --fail-above` with a committed
`.crap-baseline.json`, and `Audit (cargo-deny)`), so they are no longer the blocker. What remains
for the cut itself: flip `cargo-semver-checks` from `continue-on-error: true` to enforcing, and
settle the MSRV question (see the MSRV issue below). Do not re-derive this hold from the gate status
— it is a release-sequencing decision, not a technical one.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## The declared MSRV is asserted but never verified `low` [human]

**Human-gated — a v1.0.0 prerequisite, not v0.6.0 work. CID must not act on this unprompted.**

The workspace declares `rust-version = "1.85"` (root `Cargo.toml`, inherited by all 8 crates), but
nothing tests that the crate actually builds on 1.85: every Rust job in `ci.yml` uses
`dtolnay/rust-toolchain@stable`, there is no `rust-toolchain.toml`, and `resolver = "2"` is not
MSRV-aware. The claim is therefore unfalsifiable today — tolerable at 0.5.0, not at v1.0.0, where
MSRV becomes part of the stability commitment.

**Scope when it lands:** add a CI job on `dtolnay/rust-toolchain@1.85` running
`cargo check -p iscc-lib`. It must skip dev-dependencies (`cargo check`, not `cargo test`) — the
dev-dependency graph needs rustc 1.86 by design (criterion 0.8, landed 171), and that is not an MSRV
violation. It must also never use `--workspace`: `crates/iscc-uniffi` declares its own
`rust-version = "1.91"` (iteration 173, `publish = false`), which is truthful, not a violation.

Resolved when either the job exists and passes, or `rust-version` is dropped/raised deliberately.

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
