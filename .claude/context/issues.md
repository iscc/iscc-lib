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

## Go codec input cleaning diverges from iscc-core `iscc_clean` `normal` [review]

The pure-Go port (`packages/go`) does not clean codec input the way the reference does. The
reference routes codec input through `iscc_clean` (`reference/iscc-core/iscc_core/codec.py:762`,
reached via `normalize_multiformat` at `:364`), which strips dashes and surrounding whitespace and
accepts a case-insensitive `iscc:` scheme. Go does none of this, so documented-valid inputs raise.
Reference behaviour (verified live against `iscc_core` 1.3.0):

- `ISCC:KACY-PXW4-45FT-…` → 4 units (the hyphen-separated display form)
- `"  ISCC:AAAYPXW445FTYNJ3  "` → 1 unit
- `"iscc:AAAYPXW445FTYNJ3"` → 1 unit

Four Go sites clean ad-hoc: `codec.go:482` (`isccDecompose`), `code_content_mixed.go:23`,
`code_iscc.go:27` and `codec.go:592` (`isccNormalize`) — a hand-written, non-mechanical port, so it
is a separate step from the Rust fix, not a fan-out of it. Rust `codec::iscc_clean` is the model.
Porting subtleties:

- In the no-colon branch (`codec.py:774-779`) dashes are stripped **only when the string is not
    multibase-prefixed** — an unconditional strip corrupts `f`/`b`/`v`/`z`/`u`-prefixed input.
- Reject a cleaned `""` before decode, else `isccDecompose` silently returns an empty slice for
    `"   "`/`"-"`/`"iscc:"`/`"----"`.
- Trim exactly Python's `str.strip()` set: `char::is_whitespace` plus U+001C–U+001F (Rust
    `is_py_strip_char`). Per the Go docs (unprobed), `unicode.IsSpace`/`strings.TrimSpace` follow
    `White_Space` and so miss those four.

Since iter 199 Go's strict `decodeBase32` also rejects a trailing `\n` that `encoding/base32` used
to skip, so `IsccDecode("ISCC:GAAAAAAAAA\n")` now errors where the reference strips it: the port
must trim before decode. Multiformat decoding itself stays a documented non-goal — file it
separately if ever wanted.

Resolved when a shared Go `isccClean` helper is routed through all four Go sites and a differential
test covers the dash, whitespace (including a U+001C–U+001F case), lowercase-scheme and
empty-cleaned forms for `isccDecompose`, decode, `genIsccCodeV0` and `genMixedCodeV0`.

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

## Track upstream decision on Unicode version drift `low` [human]

**Human-only — CID must not act on this.** iscc-lib removed its Unicode 16.0.0 pin (sentinel freeze
rule, frozen `Final_Sigma` tables, boundary vectors, differential sweep) on 2026-07-30 by decision
of Titusz: the core now applies the same steps as the reference with whatever Unicode tables its
dependencies ship, and cross-version compatibility measures are postponed until upstream decides how
`iscc-core` will handle Unicode version drift.

<https://github.com/iscc/iscc-core/issues/137> tracks the upstream question (`iscc-core` output has
never been deterministic across its declared `>=3.9,<4.0` range — Unicode 13.0/14.0/15.0/15.1/16.0
by Python version; per ISO 24138 Annex D the reference is normative, so the release that addresses
it settles the standard's answer). When upstream lands a mechanism, revisit whether iscc-lib adopts
the same one.

Close this issue when upstream has decided and a follow-up decision for iscc-lib is recorded.

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

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.

## JNI streaming-hasher functions dereference Java-supplied handle without null guard `low` [human]

`dataHasherUpdate`/`dataHasherFinalize`/`instanceHasherUpdate`/`instanceHasherFinalize` in
`crates/iscc-jni/src/lib.rs` dereference the `jlong` handle via `unsafe { &mut *(ptr as *mut …) }`
with no `ptr != 0` guard (only the `*Free` functions check). A null or stale handle passed from Java
is UB. Pre-existing (not introduced by the jni 0.22 migration) and mitigated by the Java wrapper
owning the handle lifecycle, but a one-line `if ptr == 0` → throw guard per function would make the
boundary robust against misuse of the public JNI surface. Surfaced during the PR #44 review sweep
(2026-07-30).

## `@iscc/wasm` on npm has no license and a README-badge description `normal` [human]

`npm view @iscc/wasm@0.7.0 license` is empty and its `description` is the README's badge markdown
(`[![CI](…)`), while `@iscc/lib` shows `Apache-2.0`. Cause: `crates/iscc-wasm/Cargo.toml` declares
no `license`/`description` (wasm-pack derives `pkg/package.json` from it), and the release
workflow's "Set npm package name and version" step (`.github/workflows/release.yml`, `build-wasm`)
patches only `name`, `repository` and `version`. npm and package scanners therefore treat the
package as unlicensed.

Resolved when a `wasm-pack build` of `crates/iscc-wasm` (or the release patch step) yields a
`package.json` with `"license": "Apache-2.0"` and a plain-text description, verified by inspecting
the generated `pkg/package.json`. Takes effect on npm with the next release.

## Dependabot alert: Ruby `json` gem in `crates/iscc-rb/Gemfile.lock` `normal` [human]

GitHub Dependabot alert #1 (low): `json` 2.21.1 is affected by "JSON::ResumableParser#partial_value
dereferences a freed input buffer" (vulnerable `>= 2.20.0, <= 2.21.1`, patched in 2.21.2). The lock
file pins the Ruby binding's development/test environment only — the published gem does not pin
`json`. Constraint: `rb_sys` is deliberately held at 0.9.123 (see `crates/iscc-rb/CLAUDE.md` and the
held comments in the Gemfile), so update `json` alone (`bundle update json --conservative` in
`crates/iscc-rb`) rather than refreshing all gems.

Resolved when `Gemfile.lock` pins `json` >= 2.21.2 with `rb_sys` unchanged, the Ruby CI job is
green, and `gh api repos/iscc/iscc-lib/dependabot/alerts/1 --jq .state` reports `fixed`.

## Ruby conformance test depends on the process locale `normal` [human]

`crates/iscc-rb/test/test_conformance.rb:13` reads `data.json` with `File.read(DATA_JSON)`, which
uses `Encoding.default_external`. With `LANG`/`LC_ALL` unset (as in the devcontainer) Ruby reads the
UTF-8 file as US-ASCII and the whole suite aborts with `Encoding::InvalidByteSequenceError` before
any test runs. CI passes only because its runners have a UTF-8 locale.

Resolved when every Ruby test that reads a JSON fixture passes an explicit `encoding: "UTF-8"` and
`env -u LANG -u LC_ALL bundle exec rake test` passes in `crates/iscc-rb`.

## Base32 decoding accepts non-ASCII case-fold aliases `normal` [review]

Both decoders upper-case with Unicode rules before decoding, so a non-ASCII letter whose uppercase
is ASCII decodes as an alias of a canonical code: two strings for one ISCC, exactly what the strict
base32 decision (decisions.md 2026-11-18) exists to prevent. iscc-core rejects every such input
(`b32decode` raises "string argument should contain only ASCII characters"), so the fix *matches*
the reference. Probed 2026-11-18 (`iscc_core` 1.5.0, the `iscc_lib` Python binding, Go):

| Input (canonical form)                                 | iscc-core | Rust    | Go      |
| ------------------------------------------------------ | --------- | ------- | ------- |
| `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGı` (U+0131, `…CNGI`) | Err       | aliases | aliases |
| `ISCC:GAAWDLQNG6ßDPF2` (`…NG6SSDPF2`)                  | Err       | aliases | Err\*   |
| `ISCC:GAAT5ﬁNQWRC4HQV` (`…T5FINQWRC4HQV`)              | Err       | aliases | Err\*   |

\* per the iter-199 advance handoff (Go `strings.ToUpper` leaves `ß`/`ﬁ` unchanged), not re-probed.
U+017F (`ſ` → `S`) behaves like U+0131 (advance-probed). Sites: Rust `codec.rs:520`
(`decode_base32`), `codec.rs:834` (upper-cases seq units *before* `decode_base32`), `lib.rs:229`
(prefix check); Go `codec.go:404` (`decodeBase32` — its re-encode compare runs against the
already-upper-cased string, so it cannot see this) and `codec.go:453` (`checkPrefix`). Fix:
ASCII-only upper-casing (`to_ascii_uppercase`; in Go, reject bytes >= 0x80 or upper-case ASCII
only), so non-ASCII bytes reach the decoder and fail as invalid symbols. The Rust change is on the
iai-gated codec path — run `mise run bench:iai:check`.

Resolved when Rust and Go both reject the three inputs above and a U+017F case with a
`base32 decode error`, pinned in `test_strict_base32.rs` and the Go strict tests.

**Spec:** `.claude/context/specs/rust-core.md` → "Codec Operations"

## Dependabot alerts: `soupsieve` and `setuptools` in `uv.lock` `normal` [review]

GitHub reports 5 open alerts on `uv.lock` (2026-11-18): `soupsieve` 2.9.1 has #6 high (ReDoS, fixed
3.0.2), #4 high (polynomial-time descendant combinator, fixed **4.0.0**), #5 and #3 medium;
`setuptools` 83.0.0 has #2 low (fixed 84.1.0). Both are dev-environment only: `soupsieve` arrives
via `beautifulsoup4` ← `mdformat-web` ← `mdformat-mkdocs[recommended]` (dev group), so no published
artifact pins them. Check that `beautifulsoup4` accepts `soupsieve` 4.x before bumping; refresh only
these packages (`uv lock --upgrade-package <pkg>`), not the whole lock.

Resolved when `uv.lock` pins `soupsieve` >= 4.0.0 and `setuptools` >= 84.1.0, `mise run check`
(mdformat) is green, and `gh api 'repos/iscc/iscc-lib/dependabot/alerts?state=open'` lists none of
#2-#6.

## `text_trim` / `text_remove_newlines` keep U+001C–U+001F that iscc-core strips `normal` [review]

The reference text utilities use Python whitespace: `text_trim` ends in `.strip()`
(`reference/iscc-core/iscc_core/code_meta.py:184`) and `text_remove_newlines` is
`" ".join(text.split())` (`:199`). Python's whitespace set is Rust's `char::is_whitespace` plus the
four information separators U+001C–U+001F. The Rust ports use `str::trim` and `split_whitespace`
(`crates/iscc-lib/src/utils.rs:115,125,132`), so both Tier-1 functions keep those four characters.
Probed 2026-11-18 (CPython 3.13, `iscc_core` 1.5.0) over every code point: exactly these 4 differ.
`text_trim("\x1eab\x1fcd\x1e", 64)` gives `'ab\x1fcd'` in iscc_core and the input unchanged in
iscc_lib; `text_remove_newlines` of the same string gives `'ab cd'` vs the input unchanged.

Meta-Code and Text-Code output did not diverge on the probed inputs: `text_clean` and the
`text_collapse` category filter drop every `Cc` character, these four included, before any trim. So
the divergence reaches callers of the two utilities themselves, through every binding. The codec
already trims the Python set (`codec::is_py_strip_char`, private): reuse it rather than writing a
second predicate. The text utilities sit on the iai-gated Meta-Code benches, so run
`mise run bench:iai:check` in the same step. The Go text utilities are unprobed and may share it.

Resolved when `text_trim` and `text_remove_newlines` match iscc-core on U+001C–U+001F, and a Python
differential over the live-derived `str.strip()` set (as in `tests/test_seq.py`) covers both.

**Spec:** `.claude/context/specs/rust-core.md` → "Text Utility Functions"
