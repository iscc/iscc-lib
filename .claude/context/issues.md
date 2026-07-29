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

## `docs/c-ffi-api.md` type names don't match the generated `iscc.h` `normal` [review]

The page documents every FFI struct under an unprefixed name (`IsccDecodeResult`, `IsccByteBuffer`,
`IsccByteBufferArray`, …), but cbindgen prefixes exported types in `crates/iscc-ffi/include/iscc.h`
— the real declarations are `iscc_IsccDecodeResult`, `iscc_IsccByteBuffer`, etc. (typedef at
`iscc.h:79/104`; no unprefixed alias exists). So every C snippet on the page — including the
`iscc_gen_iscc_id_v1` decode example added in iter 190 (`IsccDecodeResult d = iscc_decode(...)`,
`docs/c-ffi-api.md:398`) — fails to compile verbatim against the header produced by the documented
generation command. This is a pre-existing, page-wide convention, not a regression from the IDv1
sweep, which is why the sweep step (docs-only, example follows page style) was accepted.

Resolved when the page's type names match the generated header (either rename all snippets to the
`iscc_`-prefixed forms, or emit an unprefixed alias in the cbindgen config and document that) and a
representative example compiles against `crates/iscc-ffi/include/iscc.h`.

## Ruby `gen_iscc_id_v1` breaks validation order for arguments exceeding i64 `normal` [review]

The Ruby native fn (`crates/iscc-rb/src/lib.rs`) takes its three params as `i64`, so Magnus narrows
each Ruby Integer to `i64` **during argument marshalling, before** the ordered `checked()` helper
runs. For arbitrary-precision inputs above `i64::MAX` this violates the normative ts→hub→realm
first-failure order and the documented `RuntimeError` contract. Verified on `iscc_core` 1.3.0 parity
build:

- `gen_iscc_id_v1(1<<52, 1<<100, 2)` → reference reports `Timestamp overflow` (timestamp is the
    first invalid field); Ruby raises `RangeError: bignum too big to convert into 'long long'`.
- `gen_iscc_id_v1(1<<70, 0, 0)` → timestamp is out of range, contract says `RuntimeError`; Ruby
    raises `RangeError`.

All i64-representable inputs (every realistic timestamp — `< 2^63` µs ≈ 292K years — and every
hub_id/realm) validate correctly in order, so practical impact is nil; this is a
contract-conformance gap, not a functional bug. napi/wasm are exempt (f64 params hold the values
before `checked`); jni/Go/ffi are exempt (statically-typed narrow args cannot exceed the width at
the call site). Only Ruby's arbitrary-precision + implicit i64 marshalling exposes it.

Resolved when out-of-range params raise `RuntimeError` in ts→hub→realm order for any magnitude —
e.g. validate magnitude in `lib/iscc_lib.rb` before the native call, or accept `magnus::Integer` and
range-check before `to_i64`. Add a test asserting `(1<<52, 1<<100, 2)` reports timestamp and
`(1<<70, 0, 0)` raises `RuntimeError`. Check uniffi's Ruby-analogue surfaces when they land.

**Spec:** `.claude/context/specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)"

## Go codec input cleaning diverges from iscc-core `iscc_clean` `normal` [review]

The Rust half landed in iter 191 (shared `codec::iscc_clean` routed through all four Rust sites +
`tests/codec_clean.rs`); the pure-Go port (`packages/go`) still has the four parallel gaps and keeps
the issue open. The reference routes codec input through `iscc_clean`
(`reference/iscc-core/iscc_core/codec.py:644`, reached via `normalize_multiformat` at `:363`), which
strips dashes and surrounding whitespace and accepts a case-insensitive `iscc:` scheme. Go does none
of this, so documented-valid inputs raise. Reference behaviour (verified live against `iscc_core`
1.3.0):

- `ISCC:KACY-PXW4-45FT-…` → 4 units (the hyphen-separated form is blessed by `codec.py:381-382`)
- `"  ISCC:AAAYPXW445FTYNJ3  "` → 1 unit
- `"iscc:AAAYPXW445FTYNJ3"` → 1 unit

Four Go sites clean ad-hoc: `codec.go:482` (`isccDecompose`), `code_content_mixed.go:23`,
`code_iscc.go:27` and `codec.go:592` (`isccNormalize`) — a hand-written, non-mechanical port, so it
is a separate step from the Rust fix, not a fan-out of it.

Porting subtlety: in the reference's no-colon branch (`codec.py:656-661`) dashes are stripped **only
when the string is not multibase-prefixed**. An unconditional dash strip would corrupt
`f`/`b`/`v`/`z`/`u`-prefixed input. Multiformat decoding itself stays a documented non-goal — file
it separately if ever wanted.

Resolved when a shared Go `isccClean` helper is routed through all four Go sites and a differential
test covers the dash, whitespace and lowercase-scheme forms for `isccDecompose`, decode,
`genIsccCodeV0` and `genMixedCodeV0`. (Rust surface: landed iter 191 but has an outstanding
empty-input regression — see handoff NEEDS_WORK; the Go port must not copy that gap.)

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
