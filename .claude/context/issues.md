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
`iscc.h`; no unprefixed alias exists). So every C snippet on the page, including the
`iscc_gen_iscc_id_v1` decode example, fails to compile verbatim against the header produced by the
documented generation command. Pre-existing and page-wide.

Resolved when the page's type names match the generated header (`iscc_`-prefixed snippets, or a
documented unprefixed cbindgen alias) and a representative example compiles against `iscc.h`.

## iscc-core mints 88-bit ISCC-IDv0 codes it cannot decode `low` [review]

`iscc_core.encode_length` accepts an 88-bit `ID` body, but the `LN` enum (`constants.py`) has no 88:
on `iscc_core` 1.3.0 `gen_iscc_id_v0(code, 1, wallet, uc=20000)` → `ISCC:MEBUAGCA47NDLBDXUCOAC`, and
`iscc_decode` on it raises `ValueError: 88 is not a valid LN`. iscc-lib decodes it. **No local
change**: ISCC-IDv0 is out of scope, and matching would replicate a defect, so a conformance sweep
must not "fix" it.

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

Three items, each waiting on a trigger — CID must not act on them unprompted:

1. **A zero-resolved run still exits 0** (`action-inputs: resolved <R> of <T>` is visible, not
    fatal). Flipping it reverses decisions.md 2026-07-26, so it **needs Titusz**. **Trigger:** a
    `release-workflow` CI log with a non-zero skip count.
2. **Job-level `uses:` (reusable workflows) is unscanned**; `…/.github/workflows/x.yml@ref` needs a
    different URL shape than `action_yml_urls`. **Trigger:** a reusable workflow is added.
3. **The YAML 1.1 boolean fix is one-sided**: an action input named `on`/`off`/`yes`/`no` parses as
    a bool on the metadata side, so a quoted `"on"` would be reported undeclared. **Trigger:** the
    false positive is observed.

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
the `encode_seq` case-fold vectors in `crates/iscc-lib/tests/data-edge.json` pin it. The Python
docstrings still name only padding and spare bits as differences from iscc_core: `_lowlevel.pyi:56`
says "Base32 decoding is strict: `=` padding and non-zero spare bits are rejected", and the
`iscc_lib/__init__.py` wrapper says nothing about it.

Resolved when both `encode_seq` docstrings name non-ASCII case-fold aliases next to padding and
spare bits as inputs iscc-lib rejects and iscc_core accepts.

**Spec:** `.claude/context/specs/rust-core.md` → "Codec Operations" ("Base32 input is strict")

## Dependabot alerts: `soupsieve`, `setuptools` (`uv.lock`) `normal` [review]

GitHub reports open alerts on `uv.lock` (2026-11-18): `soupsieve` 2.9.1 has #6 high (ReDoS, fixed
3.0.2), #4 high (polynomial-time descendant combinator, fixed **4.0.0**), #5 and #3 medium;
`setuptools` 83.0.0 has #2 low (fixed 84.1.0). Both are dev-environment only: `soupsieve` arrives
via `beautifulsoup4` ← `mdformat-web` ← `mdformat-mkdocs[recommended]` (dev group), so no published
artifact pins them. Check that `beautifulsoup4` accepts `soupsieve` 4.x before bumping; refresh only
these packages (`uv lock --upgrade-package <pkg>`), not the whole lock.

Resolved when `uv.lock` pins `soupsieve` >= 4.0.0 and `setuptools` >= 84.1.0, `mise run check`
(mdformat) is green, and `gh api 'repos/iscc/iscc-lib/dependabot/alerts?state=open'` lists none of
#2-#6.

## napi `Buffer` parameters leak a reference on non-Buffer object input `low` [review]

napi 3.11.0's `Buffer::from_napi_value` (`napi-3.11.0/src/bindgen_runtime/js_values/buffer.rs:489`)
creates a reference before `napi_get_buffer_info` and returns on its failure without deleting it. So
passing an `ArrayBuffer`, plain array or other non-Buffer object to any `Buffer` parameter of
`@iscc/lib` throws as expected but keeps the object referenced for the life of the Node environment.
Every `Buffer` parameter in `crates/iscc-napi/src/lib.rs` has this shape. Pre-existing; Codex, iter
204\. Unverified fixes: a napi release that frees the reference, a conversion that takes none, or
`#[napi(strict)]` (check it still accepts plain `Uint8Array`).

Resolved when repeated `ArrayBuffer` calls to a `Buffer`-taking export show no reference or heap
growth, and every existing Buffer/Uint8Array input still works.

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

`throw_and_default` in `crates/iscc-jni/src/lib.rs` throws `IllegalArgumentException`
unconditionally, also when a failed JNI call (`new_string`, `byte_array_from_slice`, …) already left
an exception pending. So a heap-exhaustion `OutOfMemoryError` reaches Java as "invalid input", and
`ThrowNew` with an exception pending breaks the JNI spec. Codex reproduced it in iter 208 (not
re-verified): `-Xmx16m`, `encodeSeq` over 500,000 valid units →
`IllegalArgumentException: JNI call failed`. Crate-wide pattern; the 2026-07-28 decision
(hand-thrown exceptions) must stay intact. Unverified fix: return the default without throwing when
`env.exception_check()` is true.

Resolved when a JNI-allocation failure reaches Java as the original `OutOfMemoryError` (test under a
small `-Xmx`, or argue why untestable) and the existing exception tests still pass.

## Ruby `i32` list elements silently truncate negative Bignums `low` [review]

`IsccLib.gen_audio_code_v0([-(2**63)])` returns the code of `[0]`, where other out-of-range elements
raise `RangeError`. Magnus 0.8.2 `RBignum::to_i32` (`r_bignum.rs:194`) checks only the upper bound
before `as i32`, so Bignums from -2**63 to -(2**62)-1 truncate. It reaches every `i32` element
conversion in `crates/iscc-rb/src/lib.rs` (`gen_audio_code_v0`, `gen_video_code_v0`,
`soft_hash_video_v0`), contradicting `docs/ruby-api.md` Error Handling. Pre-existing; Codex, iter
212\. `u32` elements were not probed.

Resolved when a negative Bignum element in those three functions raises `RangeError` (e.g. convert
through `i64`, narrow with `i32::try_from`), pinned next to the `RangeError` cases in
`crates/iscc-rb/test/test_array_args.rb`.

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

## Swift `data-edge.json` compare is looser than byte equality `low` [review]

**Triage:** untriaged. Nobody hits it today: the Rust harness pins the same bytes strictly.

`compareValue` in `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift` compares with Swift
`String ==`, which uses canonical equivalence (`"K" == "\u{212A}"` is `true`, checked on 6.1.2), and
reads expected numbers with `as? Int`, which also accepts a JSON `false`/`true` from
`JSONSerialization`. So a Swift-only marshalling bug that returned a canonically equivalent
non-ASCII string would pass this harness. Found by Codex in iter 218; the Python template conflates
`0`/`False` too.

Resolved when the Swift compare matches strings by Unicode scalars and rejects Boolean-backed
numbers, on both Linux and macOS.

## Go and Swift ship `data-edge.json` despite "absent from every published artifact" `low` [review]

**Triage:** untriaged. No user is harmed (one extra test file in a download); the question is which
of two spec lines wins.

`edge-vectors.md` requires a copy "next to every vendored copy of `data.json`" and also "The file is
absent from every published artifact". The Rust crate meets the second with `exclude` in
`crates/iscc-lib/Cargo.toml`. Go cannot: the module zip that `proxy.golang.org` serves carries all
of `packages/go/`, `testdata/` included, so `packages/go/testdata/data-edge.json` (iter 217) ships.
Swift Package Manager clones the whole repository, canonical copy included. Possible readings: (a)
"published artifact" means built or packaged output, and a source module is exempt; (b) Go reads the
canonical file through `../../crates/iscc-lib/tests/` like its `code_*_test.go` do, and the Go copy
is dropped. (b) leaves the Swift case open.

Resolved when Titusz picks a reading and the spec says how a source-distributed surface meets it.

**Spec:** `.claude/context/specs/edge-vectors.md` → "The File", "Verified When"

## C++ list wrappers pass NULL for an empty vector `low` [review]

**Triage:** untriaged. `gen_mixed_code_v0`/`gen_iscc_code_v0` still throw `iscc::IsccError` on `{}`,
with the FFI's NULL message (iter 219); `gen_video_code_v0`, `alg_simhash`, `soft_hash_video_v0` are
unprobed. All five in `iscc.hpp` pass `ptrs.data()`, NULL when empty, where `encode_seq` uses
`detail::safe_data`. Resolved when they pass a non-NULL pointer and a test on `{}` pins each.
