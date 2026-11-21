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

**Triage:** untriaged — a maintainer who drops the `exception_check()` guard from the JNI throw
helpers gets no failing test; Java callers would then see a constructor or JVM exception replaced by
`IllegalArgumentException: Java exception was thrown`.

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

## Ruby integer list elements accept a Float and truncate it `low` [review]

**Triage:** untriaged — a Ruby caller who passes Float elements where integers are expected gets the
code of the truncated values instead of an error.

Magnus' integer conversions apply Ruby's implicit `to_int`. So `IsccLib.gen_audio_code_v0([1.9])`
returns the code of `[1]`, and `alg_minhash_256` does the same, where Python raises `TypeError`. It
follows Ruby's own convention (`[10, 20, 30][1.9]` is `20`), so it may be acceptable as host idiom.
Pre-existing, measured in iter 229. The doc comment on `array_to_i32_vec` in
`crates/iscc-rb/src/lib.rs` says "`TypeError` for a non-Integer", which does not hold for a Float.
No `data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong-typed elements).

Resolved when either a Float element raises `TypeError` in every integer-list parameter, pinned in
`crates/iscc-rb/test/test_array_args.rb`, or `docs/ruby-api.md` → Error Handling states the implicit
conversion and the doc comment matches it.

## The pkg-config example in the C/C++ how-to says `Version: 0.2.0` `low` [review]

**Triage:** untriaged — a reader who copies the `iscc-ffi.pc` example from `docs/howto/c-cpp.md` →
"pkg-config" gets a file that reports 0.2.0 for whatever release they installed, so
`pkg-config --modversion` and `--atleast-version` answer wrongly.

The line is typed by hand and is not a `scripts/version_sync.py` target, which covers only the
Maven, Gradle and SPM snippets in docs (0.7.0 is current). Pre-existing, found in iteration 244.

Resolved when the example shows no version that no check keeps current: a placeholder the reader
fills in, or the line is a `version_sync.py` target.

## Published Ruby source gems up to 0.7.0 do not build `low` [review]

**Triage:** untriaged — a user who runs `gem install iscc-lib` on Ruby 3.4 or newer, or on a
platform without a precompiled gem, gets a build failure until a release ships the repaired source
gem.

The source gem on rubygems.org was built with a bare `gem build` up to 0.7.0: it declares no
`rb_sys` dependency and ships a workspace-inheriting `Cargo.toml` without the core (measured for
0.7.0 in iteration 238). The repository now builds a standalone source gem (`ruby-source-gem` CI
job), but published versions cannot be changed. Yanking the broken `ruby`-platform gems is a
registry action for Titusz. CID must not act on this.

Resolved when a release with the repaired source gem is on rubygems.org and it installs from the
registry into a clean gem home on a Ruby series without a precompiled gem
(`GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install iscc-lib`, then `IsccLib.conformance_selftest`).

## How-to examples show the Sum-Code prefix as `KAA` `low` [review]

**Triage:** untriaged — a reader who compares their own output with the comment in a how-to sees
`ISCC:KUA…` where the docs say `"ISCC:KAA..."`.

`gen_sum_code_v0` composes a Data-Code and an Instance-Code, so without `wide` its result has
SubType SUM and starts with `ISCC:KUA` (measured in iteration 238). The Sum-Code examples in the
how-tos under `docs/howto/` and in `docs/java-api.md` carry the comment `"ISCC:KAA..."`, and the
`iscc` field comment in `SumCodeResult.java` says `"ISCC:KAC..."`. Pre-existing and cosmetic. The
`gen_iscc_code_v0` examples next to them use the same comment and were not checked.

Resolved when each Sum-Code example comment shows a prefix the call can return.

## Three supported-version floors are past end of life `low` [review]

**Triage:** untriaged — a decision for Titusz: CI tests these surfaces only on runtimes that no
longer get security fixes, and the packages promise to support them. CID must not raise a floor.

End-of-life dates from endoflife.date, read 2026-11-20:

- **Python 3.10** (abi3-py310; `python-version: '3.10'` in `ci.yml` and `release.yml`): 2026-10-01
- **.NET 8** (`dotnet-version: '8.0'`, devcontainer `--channel 8.0`): 2026-11-10
- **Ruby 3.1** (gemspec `>= 3.1.0`, CI `ruby-version: '3.1'`, devcontainer apt `ruby`): 2025-03-26.
    Ruby 3.2 in the cross-gem list: 2026-03-31

Not end-of-life: Java 17, MSRV 1.85, the Go `go 1.26.1` directive (CI tests on the `toolchain`
line). Raising the Python floor changes the abi3 tag of the wheel; raising .NET or Ruby changes the
target framework or the gemspec.

Resolved when each of the three floors is either raised by the human or kept on purpose (a
decisions.md entry), and CI tests on a supported runtime wherever the floor moved.

**Spec:** `.claude/context/specs/ci-cd.md` → "Dependency Freshness" → Policy

## `ci-cd.md` describes a tag trigger the release workflow does not have `low` [review]

**Triage:** untriaged — a contributor or agent who reads `ci-cd.md` expects a pushed `v*.*.*` tag to
publish every registry. Nothing happens, or someone adds the trigger to match the spec.

`.github/workflows/release.yml` has `workflow_dispatch` as its only trigger, and `target.md` says "a
tag push does not trigger the workflow". Three places in `.claude/context/specs/ci-cd.md` say
otherwise: the Trigger cell of the Release row in "Workflow Files", the "Tag trigger as convenience"
bullet under "Release Workflow — Selective Publishing" → Design Principles, and the
`push: tags: [v*.*.*]` lines in the "Trigger Configuration" YAML block. Pre-existing. Realign the
spec without escalation (decision of 2026-11-20).

Resolved when the three places describe `workflow_dispatch` as the only trigger.

**Spec:** `.claude/context/specs/ci-cd.md` → "Release Workflow — Selective Publishing"

## `target.md` states a version-sync target count that is out of date `low` [review]

**Triage:** untriaged — an agent that reads the CI/CD criterion in `target.md` takes "all 12 sync
targets" as the number to verify, while `TARGETS` in `scripts/version_sync.py` holds 21. Only the
human edits `target.md`; CID must not act on this.

The criterion itself is met: `mise run version:check` covers every entry in `TARGETS`. The number is
a derived fact of the kind the decision of 2026-11-21 removed from the specs. Found while scoping
iteration 248.

Resolved when the `target.md` line names no count ("every entry in `TARGETS`") or the current one.

**Spec:** `.claude/context/target.md` → "CI/CD, Dev Tooling, and Publishing"

## Go canonical `meta` JSON differs from the core in key order, `-0` and lone surrogates `low` [review]

**Triage:** untriaged — a Go caller whose `meta` JSON has an emoji key next to a key from
U+E000–U+FFFF, a `-0.0` value or a lone `\ud800` escape gets a different Meta-Code, metahash and
data-URL than the other surfaces return (for the third input they return an error).

`parseMetaJSON` (`packages/go/code_meta.go`) keeps three differences from RFC 8785, measured against
`iscc_lib` in iteration 251. `encoding/json` sorts keys by UTF-8 bytes, JCS by UTF-16 code units:
for `{"\ue000":1,"\ud83d\ude00":2}` Go keeps this key order, the core swaps it. Go writes `-0.0` as
`-0`, the core as `0`. Go turns a lone surrogate escape into U+FFFD, the core rejects the input.
Pre-existing. The other numbers probed match (`1.0`, `1e21`, `1e-7`, `9007199254740993`).

Resolved when Go returns the core's result for the three inputs and each is a `data-edge.json`
vector.

## C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection the FFI no longer has `low` [review]

**Triage:** untriaged — a maintainer of the C++ or .NET wrapper keeps two workarounds and a source
gate for a condition that cannot occur. Callers see no difference.

The C FFI reads a NULL pointer with length 0 as an empty input on every pointer+length parameter
(decision of 2026-11-21). `detail::safe_data` in `packages/cpp/include/iscc/iscc.hpp`, its gate
`tests/test_cpp_safe_data.py`, and the `max(Count, 1)` padding of `PinnedRows` in
`packages/dotnet/Iscc.Lib/IsccLib.cs` now only keep pointers non-null and change no result. Dropping
the padding supersedes that half of the 2026-11-20 decision on .NET jagged arrays (its null check
stays). Keeping both on purpose is a valid outcome too.

Resolved when both are removed with the C++ and .NET empty-input tests still green, or a
decisions.md entry keeps them on purpose.

## Out-of-range integer arguments wrap silently on JS, raise undocumented classes elsewhere `normal` [audit]

napi/WASM take `u32`/`i32` (`iscc-napi/src/lib.rs:138,162,292,373,397`); only `gen_iscc_id_v1` runs
`checked()`. Measured: bits `2**32+64` gives the bits-64 code, `text_trim("abc",-1)` → `"abc"`,
`gen_audio_code_v0([2**40])` wraps to 0. JNI casts `bits as u32` unchecked
(`iscc-jni/src/lib.rs:230…`). Python raises `OverflowError` vs documented `ValueError`
(`docs/howto/python.md:540`); Ruby `RangeError` (`iscc-rb/src/lib.rs:62-70`) vs documented
`RuntimeError` (`docs/ruby-api.md:82`). **Fix:** route JS numerics through `checked()`, reject
negative JNI `bits`, map or document Py/Rb classes; one negative + 2^32+x test per surface. **Scope
estimate:** 5-6 non-test files.

## Published-package docs contradict what ships `normal` [audit]

`README.md:190,285` print `result.iscc` from a string return; WASM snippet lacks `await init()`
(package is `--target web`, `release.yml:427`). Java/Kotlin docs say no Maven Central/no loader
(`docs/howto/java.md:6,29`, `crates/iscc-jni/README.md:27`); Swift READMEs say build from source
though `Package.swift:18-20` fetches an XCFramework; `docs/architecture.md:30,178` (five bindings,
napi AsyncTask); `docs/development.md:181` + `CLAUDE.md:207` claim pytest coverage enforcement, no
`--cov` exists. **Fix:** correct each claim against the code. **Scope estimate:** ~11 files — needs
audit-scope step (≤8 files); do README + how-tos first.
