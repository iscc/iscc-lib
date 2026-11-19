# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files. A `low`
entry with `**Triage:** untriaged` is a review finding the human has not weighed yet: promote it to
`normal`, or delete the line to park it.

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

## Python `encode_seq` docstrings omit the case-fold divergence from iscc-core `normal` [human]

Since iteration 201, `encode_seq` upper-cases ASCII only, so a non-ASCII case-fold alias such as
`gaawdlqng6ſsdpf2` (likewise `ı`, `ß`, `ﬁ`) fails, while iscc-core 1.5.0 `encode_seq` accepts it
through Unicode `str.upper()` (`codec.py:503`). Titusz confirmed this divergence (decisions.md
2026-11-18, "Codec input is upper-cased ASCII-only"; `rust-core.md` "Base32 input is strict"), and
`tests/test_seq.py` `STRICT_BASE32_DIVERGENCES` already pins it. The Python docstrings still name
only padding and spare bits as differences from iscc_core: `_lowlevel.pyi:56` says "Base32 decoding
is strict: `=` padding and non-zero spare bits are rejected", and the `iscc_lib/__init__.py` wrapper
says nothing about it.

Resolved when both `encode_seq` docstrings name non-ASCII case-fold aliases next to padding and
spare bits as inputs iscc-lib rejects and iscc_core accepts.

**Spec:** `.claude/context/specs/rust-core.md` → "Codec Operations" ("Base32 input is strict")

## Dependabot alerts: `soupsieve`, `setuptools` (`uv.lock`) and `json` (`Gemfile.lock`) `normal` [review]

GitHub reports 5 open alerts on `uv.lock` (2026-11-18): `soupsieve` 2.9.1 has #6 high (ReDoS, fixed
3.0.2), #4 high (polynomial-time descendant combinator, fixed **4.0.0**), #5 and #3 medium;
`setuptools` 83.0.0 has #2 low (fixed 84.1.0). Both are dev-environment only: `soupsieve` arrives
via `beautifulsoup4` ← `mdformat-web` ← `mdformat-mkdocs[recommended]` (dev group), so no published
artifact pins them. Also open: #1 low, the `json` gem in `crates/iscc-rb/Gemfile.lock` (fixed
2.21.2). Check that `beautifulsoup4` accepts `soupsieve` 4.x before bumping; refresh only these
packages (`uv lock --upgrade-package <pkg>`), not the whole lock.

Resolved when `uv.lock` pins `soupsieve` >= 4.0.0 and `setuptools` >= 84.1.0, `mise run check`
(mdformat) is green, `Gemfile.lock` pins `json` >= 2.21.2, and
`gh api 'repos/iscc/iscc-lib/dependabot/alerts?state=open'` lists none of #1-#6.

## napi `Buffer` parameters leak a reference on non-Buffer object input `low` [review]

napi 3.11.0's `Buffer::from_napi_value`
(`~/.cargo/registry/src/*/napi-3.11.0/src/bindgen_runtime/js_values/buffer.rs:489-501`) calls
`napi_create_reference` BEFORE `napi_get_buffer_info`, and the `?` on a failed buffer-info check
returns without deleting the reference. So a JS caller passing an `ArrayBuffer`, plain array or
other non-Buffer object to any `Buffer` parameter of `@iscc/lib` gets the expected throw, but the
rejected object stays strongly referenced for the life of the Node environment. Every `Buffer`
parameter in `crates/iscc-napi/src/lib.rs` has this shape (`encode_component`, `decode_seq`,
`gen_image_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`, `encode_base64`, `alg_simhash`,
`alg_cdc_chunks`, `DataHasher`/`InstanceHasher` `update`); found by Codex in iter 204, pre-existing
— not introduced by the seq binding.

Possible fixes, none verified: a napi release that frees the reference on failure, a parameter type
whose conversion takes no reference, or `#[napi(strict)]` validation (check whether it changes which
inputs, e.g. plain `Uint8Array`, are accepted). Resolved when a loop of repeated `ArrayBuffer` calls
to a `Buffer`-taking export shows no reference/heap growth, and every existing Buffer/Uint8Array
input still works.

## `crates/iscc-ffi/src/lib.rs` is past the crate's own split threshold `low` [review]

`crates/iscc-ffi/CLAUDE.md` says "do not split into submodules unless the file exceeds ~2500 lines";
`lib.rs` is now 2870 (1715 of exported code, ~1155 of `mod tests` from line 1716). Constraint on any
split: `build.rs` feeds csbindgen only `input_extern_file("src/lib.rs")`, so moving `extern "C"`
functions into submodules silently drops them from `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
unless `build.rs` lists the new files. Moving only `mod tests` to `src/tests.rs` avoids that.

Resolved when `lib.rs` is back under the documented threshold (or the threshold is deliberately
changed in `crates/iscc-ffi/CLAUDE.md`), and `cbindgen` + `cargo build -p iscc-ffi` leave `iscc.h`
and `NativeMethods.g.cs` byte-identical.

## .NET string-array pinning is copied three times; the gen copies pass NULL for `[]` `normal` [review]

`GenMixedCodeV0`, `GenIsccCodeV0` and `EncodeSeq` (`packages/dotnet/Iscc.Lib/IsccLib.cs`) each carry
the same ~35-line unsafe pin loop: UTF-8 conversion, `GCHandle.Alloc(…, Pinned)`,
`fixed (byte** …)`, and a free in `finally`. Only `EncodeSeq` guards a null array or element and has
an empty-array sentinel. The two gen copies run `fixed` over an empty `byte*[]`, which yields NULL,
so `[]` surfaces the FFI's "must not be NULL" message instead of the core's error. next.md deferred
both in iter 206 to keep the seq step to one file.

Resolved when one private helper does the pinning for all three methods, and an empty or null input
to the two gen methods fails the same way as `EncodeSeq` (tests assert the core's message for `[]`).

## JNI exports report a pending JVM exception as `IllegalArgumentException` `low` [review]

Every `extern "system"` function in `crates/iscc-jni/src/lib.rs` routes JNI-environment errors
(`new_string`, `byte_array_from_slice`, `build_string_array`, `extract_string_array`, …) through
`throw_and_default`, which calls `env.throw_new(IllegalArgumentException)` unconditionally. When the
failed JNI call already left an exception pending (Java heap exhaustion → `OutOfMemoryError`), the
caller gets an `IllegalArgumentException` instead, so code that catches `IllegalArgumentException`
as "invalid input" will discard valid data. Codex reproduced it in iter 208 (not re-verified): with
`-Xmx16m`, `encodeSeq` over 500,000 copies of a valid unit threw
`IllegalArgumentException: JNI call failed`. This is the crate-wide pattern, not a seq regression.
Calling `ThrowNew` while an exception is pending also breaks the JNI spec. The 2026-07-28 decision
keeps hand-thrown exceptions at the call site, and a fix must not change that.

Possible fix (unverified): `throw_and_default` / `throw_state_error` return the default without
throwing when `env.exception_check()` reports a pending exception. Resolved when a JNI-allocation
failure reaches Java as the original `OutOfMemoryError` (test under a small `-Xmx`, or argue why
untestable), and the existing `IllegalArgumentException` / `IllegalStateException` tests still pass.

## Ruby `i32` list elements silently truncate negative Bignums `low` [review]

`IsccLib.gen_audio_code_v0([-(2**63)])` returns `ISCC:EIAQAAAAAAAAAAAA`, the code of `[0]`, and
`[-(2**62) - 5]` returns a code too, where every other out-of-range element raises `RangeError`
(`[2**40]`, `[-(2**40)]`). Cause: Magnus 0.8.2 `RBignum::to_i32`
(`~/.cargo/registry/src/*/magnus-0.8.2/src/r_bignum.rs:194`) checks only `res > i32::MAX` after
`rb_num2long`, then narrows with `as i32`; a Fixnum takes a path that checks both bounds, so only
Bignums from -2**63 to -(2**62)-1 slip through. It reaches every `i32` element conversion in
`crates/iscc-rb/src/lib.rs`: `gen_audio_code_v0` elements and the video frame elements
(`gen_video_code_v0`, `soft_hash_video_v0`). Pre-existing (the old `Vec<i32>` argument used the same
conversion); found by Codex in iter 212. `docs/ruby-api.md` Error Handling promises `RangeError` for
out-of-range list elements, which is false for these inputs. `u32` elements were not probed.

Resolved when a negative Bignum element in those three functions raises `RangeError` (for example,
convert through `i64` and narrow with `i32::try_from`), pinned by a test next to the existing
`RangeError` cases in `crates/iscc-rb/test/test_array_args.rb`.

## `wasm-bindings.md` names a committed `crates/iscc-wasm/package.json` that does not exist `normal` [review]

`.claude/context/specs/wasm-bindings.md` lists `package.json # npm package config (@iscc/wasm)` in
the crate tree (L28) and a Version Sync row for `crates/iscc-wasm/package.json` (L165). No such file
exists, and `scripts/version_sync.py` has no WASM target. The real mechanism (`ci-cd.md:355`):
wasm-pack writes `pkg/package.json` from `crates/iscc-wasm/Cargo.toml` (version, `license`,
`description`), and release.yml's "Set npm package name and version" step patches `name`,
`repository` and `version`. Iter 214 made `Cargo.toml` the single source of npm metadata; a future
step that "restores" the spec's file would create a second, unsynced source. File-list fact — no
design change.

Resolved when `wasm-bindings.md` drops the tree line and drops or realigns the Version Sync row to
the `Cargo.toml` → wasm-pack → release.yml path.

**Spec:** `.claude/context/specs/wasm-bindings.md` → "Crate Structure", "Version Sync"

## Evaluate the larger-step trial after iteration 224 `low` [human]

Since 2026-11-19 a default step may touch 6 non-test files and a fan-out brings one decided
behaviour to all surfaces in one iteration (decisions.md 2026-11-19, "Steps are sized by outcome").
This is a trial. After iteration 224, compare iterations 215-224 with 196-214: verdicts
(`mise run cid:stats`), CI results of the pushed review commits, and cost per iteration.

Keep the rules if at most 2 of the 10 iterations ended NEEDS_WORK and no fan-out push turned CI red.
Otherwise revert the commit that introduced the decisions.md entry. Resolved when Titusz records
"kept" or "reverted" in that decisions.md entry and deletes this issue.

## Derived facts in guidance and docs are written by hand and drift `normal` [human]

Counts, file lists and symbol lists are repeated across `docs/`, the READMEs, `CLAUDE.md` files and
the specs with nothing checking them. Adding `encode_seq` / `decode_seq` (Tier 1 33 → 35) took a
hand sweep of 35 files in iteration 211, and `wasm-bindings.md` listed a file that does not exist.
The two facts that have a parity hook in `.pre-commit-config.yaml` (docs page list, CI job table) do
not drift. Rule: `.claude/context/README.md` → "Hygiene rules" → "Derived facts are checked or not
written".

Resolved when both hold:

- a pre-commit parity check fails when the Tier 1 symbol list in any doc or README that enumerates
    it differs from the public API at the `iscc-lib` crate root
- counts, file lists and line numbers that no check covers are gone from `.claude/context/specs/`
    and the per-crate `CLAUDE.md` files, replaced by wording that cannot go stale ("all Tier 1
    functions") or brought under a check
