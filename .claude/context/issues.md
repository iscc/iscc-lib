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

**Status (2026-11-20):** Titusz: **not yet** — no milestone is set after **v0.8.0**; he decides what
comes next after the v0.8.0 release. The two hardening gates the 2026-06-18 hold waited on have both
landed and are enforcing (`ci.yml` → `cargo crap … --fail-regression --fail-above` with a committed
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

## JNI pending-exception guard has no test `low` [review]

`throw_and_default` and `throw_state_error` in `crates/iscc-jni/src/lib.rs` return without throwing
while an exception is pending (iter 230). No committed test reaches the guard: with it removed,
`mvn clean test` stays green. A deterministic trigger exists: `new_object` leaves a constructor's
exception pending, so a child JVM whose `-cp` starts with a directory holding a shadow
`io.iscc.iscc_lib.IsccDecodeResult` with a throwing `(IIII[B)V` constructor makes `isccDecode` reach
the guard. Measured: at HEAD the caller sees the constructor's exception, without the guard the
`IllegalArgumentException`; `-Xcheck:jni` is silent either way. The shadow class must not be a Maven
test source (it would replace the real class for every test): compile it at test time.

Resolved when a Java test fails with the guard removed from `throw_and_default`.

## napi `Buffer` parameters leak a reference on non-Buffer object input `low` [review]

napi 3.11.0's `Buffer::from_napi_value` (`napi-3.11.0/src/bindgen_runtime/js_values/buffer.rs:489`)
creates a reference before `napi_get_buffer_info` and returns on its failure without deleting it. So
passing an `ArrayBuffer`, plain array or other non-Buffer object to any `Buffer` parameter of
`@iscc/lib` throws as expected but keeps the object referenced for the life of the Node environment.
Every `Buffer` parameter in `crates/iscc-napi/src/lib.rs` has this shape. Pre-existing (iter 204).
Unverified fixes: a napi release that frees the reference, or `#[napi(strict)]`.

Resolved when repeated `ArrayBuffer` calls to a `Buffer`-taking export show no reference or heap
growth, and every existing Buffer/Uint8Array input still works.

## Ruby integer list elements accept a Float and truncate it `normal` [review]

Magnus' integer conversions apply Ruby's implicit `to_int`. So `IsccLib.gen_audio_code_v0([1.9])`
returns the code of `[1]`, and `alg_minhash_256` does the same, where Python raises `TypeError`. It
follows Ruby's own convention (`[10, 20, 30][1.9]` is `20`), so it may be acceptable as host idiom.
Pre-existing, measured in iter 229. The doc comment on `array_to_i32_vec` in
`crates/iscc-rb/src/lib.rs` says "`TypeError` for a non-Integer", which does not hold for a Float.
No `data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong-typed elements).

**Decided (Titusz, 2026-11-21):** keep the implicit conversion as Ruby host idiom; fix the docs.

Resolved when `docs/ruby-api.md` → Error Handling states the implicit conversion and the doc comment
on `array_to_i32_vec` matches it.

## The pkg-config example in the C/C++ how-to says `Version: 0.2.0` `normal` [review]

The line is typed by hand and is not a `scripts/version_sync.py` target, which covers only the
Maven, Gradle and SPM snippets in docs (0.7.0 is current). Pre-existing, found in iteration 244.

Resolved when the example shows no version that no check keeps current: a placeholder the reader
fills in, or the line is a `version_sync.py` target.

## Published Ruby source gems up to 0.7.0 do not build `low` [review]

The source gem on rubygems.org was built with a bare `gem build` up to 0.7.0: it declares no
`rb_sys` dependency and ships a workspace-inheriting `Cargo.toml` without the core (measured for
0.7.0 in iteration 238). The repository now builds a standalone source gem (`ruby-source-gem` CI
job), but published versions cannot be changed. Yanking the broken `ruby`-platform gems is a
registry action for Titusz. CID must not act on this.

Resolved when a release with the repaired source gem is on rubygems.org and it installs from the
registry into a clean gem home on a Ruby series without a precompiled gem
(`GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install iscc-lib`, then `IsccLib.conformance_selftest`).

## How-to examples show the Sum-Code prefix as `KAA` `normal` [review]

`gen_sum_code_v0` composes a Data-Code and an Instance-Code, so without `wide` its result has
SubType SUM and starts with `ISCC:KUA` (measured in iteration 238). The Sum-Code examples in the
how-tos under `docs/howto/` and in `docs/java-api.md` carry the comment `"ISCC:KAA..."`, and the
`iscc` field comment in `SumCodeResult.java` says `"ISCC:KAC..."`. Pre-existing and cosmetic. The
`gen_iscc_code_v0` examples next to them use the same comment and were not checked.

Resolved when each Sum-Code example comment shows a prefix the call can return.

## The Ruby floor is past end of life and current Rubies get no precompiled gem `normal` [human]

**Decided (Titusz, 2026-11-21):** raise the Ruby floor from 3.1 to 3.3. Ruby 3.1 reached end of life
on 2025-03-26 and 3.2 on 2026-03-31 (endoflife.date). The Python 3.10 and .NET 8 floors stay
(decisions.md, 2026-11-21).

The floor is in `crates/iscc-rb/iscc-lib.gemspec` (`required_ruby_version`), the `ruby` job in
`ci.yml` (`ruby-version: '3.1'`), the minitest hold in `crates/iscc-rb/Gemfile`, and the docs
(`crates/iscc-rb/README.md`, `docs/howto/ruby.md`, `crates/iscc-rb/CLAUDE.md`,
`specs/ruby-bindings.md`). The cross-gem step in `release.yml` builds precompiled gems for 3.1, 3.2
and 3.3 only, so a user on Ruby 3.4 or newer always compiles from source.

Resolved when the gemspec, CI and docs say `>= 3.3`, CI tests the floor, and the cross-gem list
covers every supported Ruby series from 3.3 up that the cross-gem image can build (the docs name the
list).

**Spec:** `.claude/context/specs/ruby-bindings.md`

## `ci-cd.md` describes a tag trigger the release workflow does not have `normal` [review]

`.github/workflows/release.yml` has `workflow_dispatch` as its only trigger, and `target.md` says "a
tag push does not trigger the workflow". Three places in `.claude/context/specs/ci-cd.md` say
otherwise: the Trigger cell of the Release row in "Workflow Files", the "Tag trigger as convenience"
bullet under "Release Workflow — Selective Publishing" → Design Principles, and the
`push: tags: [v*.*.*]` lines in the "Trigger Configuration" YAML block. Pre-existing. Realign the
spec without escalation (decision of 2026-11-20).

Resolved when the three places describe `workflow_dispatch` as the only trigger.

**Spec:** `.claude/context/specs/ci-cd.md` → "Release Workflow — Selective Publishing"

## Go canonical `meta` JSON differs from the core in key order, `-0` and lone surrogates `normal` [review]

`parseMetaJSON` (`packages/go/code_meta.go`) keeps three differences from RFC 8785, measured against
`iscc_lib` in iteration 251. `encoding/json` sorts keys by UTF-8 bytes, JCS by UTF-16 code units:
for `{"\ue000":1,"\ud83d\ude00":2}` Go keeps this key order, the core swaps it. Go writes `-0.0` as
`-0`, the core as `0`. Go turns a lone surrogate escape into U+FFFD, the core rejects the input.
Pre-existing. The other numbers probed match (`1.0`, `1e21`, `1e-7`, `9007199254740993`).

Resolved when Go returns the core's result for the three inputs and each is a `data-edge.json`
vector.

## C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection the FFI no longer has `normal` [review]

The C FFI reads a NULL pointer with length 0 as an empty input on every pointer+length parameter
(decision of 2026-11-21). `detail::safe_data` in `packages/cpp/include/iscc/iscc.hpp`, its gate
`tests/test_cpp_safe_data.py`, and the `max(Count, 1)` padding of `PinnedRows` in
`packages/dotnet/Iscc.Lib/IsccLib.cs` now only keep pointers non-null and change no result. Dropping
the padding supersedes that half of the 2026-11-20 decision on .NET jagged arrays (its null check
stays). **Decided (Titusz, 2026-11-21):** remove both, in the release whose FFI change made them
dead.

Resolved when both are removed with the C++ and .NET empty-input tests still green.

## `@iscc/wasm` converts non-number values to numbers instead of rejecting them `normal` [review]

The wasm-bindgen glue applies JavaScript number conversion before the range check runs. Measured on
a `--target nodejs` build in iteration 256: `gen_audio_code_v0([1, null, 3])` returns the code of
`[1, 0, 3]`, `text_trim("abc", "2")` returns `"ab"`, `text_trim("abc", true)` returns `"a"`.
`undefined` and a non-numeric string become `NaN` and are rejected. `@iscc/lib` (napi) throws on
each of these inputs. Pre-existing, and TypeScript callers are stopped by the `.d.ts`. No
`data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong argument types).

Resolved when either each numeric parameter of `crates/iscc-wasm/src/lib.rs` rejects a non-number,
pinned by a Node test on a built package, or `docs/howto/wasm.md` → "Error handling" states the
conversion.

## JNI `NativeLoader` rejects an unknown OS or architecture before the `java.library.path` fallback `normal` [review]

`NativeLoader.load()` calls `detectOs()` and `detectArch()` first, and both throw
`UnsatisfiedLinkError` for a value they do not know, so `System.loadLibrary("iscc_jni")` is never
tried. Measured on the published 0.7.0 JAR in iteration 257 with a working library on
`java.library.path`: `-Dos.arch=riscv64` fails with "Unsupported architecture: riscv64" and
`-Dos.name=FreeBSD` with "Unsupported OS: FreeBSD", while a recognized pair without a bundled
library (Windows on aarch64) loads it. Unverified: musl Linux probably fails too, because the
bundled glibc library fails inside `System.load` and only `IOException` leads to the fallback.
Pre-existing; the Java how-to and the JNI README describe it.

Resolved when either an unknown OS or architecture reaches the `System.loadLibrary` fallback, pinned
by a Java test, and the two docs say so, or a decisions.md entry keeps the limit on purpose.

## `docs/architecture.md` sections below the graph cover half the surfaces `normal` [review]

"Crate Summary" gives `iscc-ffi` → "Source", while each GitHub release attaches prebuilt
`iscc-ffi-v*-<target>` archives. The test matrix names `cargo test` as the WASM runner, which runs
no `#[wasm_bindgen_test]` (CI runs `wasm-pack test --node`). Pre-existing.

Resolved when each of the three sections covers every surface the graph shows or says it is an
excerpt, and the `iscc-ffi` and WASM rows match the release assets and the CI command.
