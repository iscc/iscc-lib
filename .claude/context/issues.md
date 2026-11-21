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
while an exception is pending (iter 230). The committed tests cover the `OutOfMemoryError` mapping
and the zero-handle guards, but none reaches the guard: with it removed, `mvn clean test` stays
green. The iter-230 handoff found no deterministic trigger; one exists. `new_object` leaves a
constructor's exception pending, so a child JVM whose `-cp` starts with a directory holding a shadow
`io.iscc.iscc_lib.IsccDecodeResult` with a throwing `(IIII[B)V` constructor makes `isccDecode` reach
the guard. Measured: at HEAD the caller sees the constructor's exception, without the guard the
`IllegalArgumentException`. HotSpot 17 `-Xcheck:jni` prints no warning either way, so only the
exception class shows the difference. The shadow class must not be a Maven test source (it would
replace the real class for every test); compile it at test time or keep it outside `src/test/java`.

Resolved when a Java test fails with the guard removed from `throw_and_default`.

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

## Ruby integer list elements accept a Float and truncate it `low` [review]

**Triage:** untriaged — a Ruby caller who passes Float elements where integers are expected gets the
code of the truncated values instead of an error.

Magnus' integer conversions start with `Integer::try_convert`, which applies Ruby's implicit
`to_int`. So `IsccLib.gen_audio_code_v0([1.9])` returns the code of `[1]`, and `alg_minhash_256`
does the same, where Python raises `TypeError`. It follows Ruby's own convention
(`[10, 20, 30][1.9]` is `20`), so it may be acceptable as host idiom; `NaN` raises
`FloatDomainError`. Pre-existing, measured in iter 229. The doc comment on `array_to_i32_vec` in
`crates/iscc-rb/src/lib.rs` says "`TypeError` for a non-Integer", which does not hold for a Float. A
wrong-typed element stays in per-binding tests (`edge-vectors.md` → Rules), so no `data-edge.json`
vector is owed.

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
0.7.0 in iteration 238). The repository now builds a standalone source gem
(`scripts/build_ruby_source_gem.py`, install-checked by the `ruby-source-gem` CI job), but published
versions cannot be changed. Yanking the broken `ruby`-platform gems is a registry action for Titusz;
a yank would turn the build failure into a "no matching gem" error. CID must not act on this.

Resolved when a release with the repaired source gem is on rubygems.org and it installs from the
registry into a clean gem home on a Ruby series without a precompiled gem
(`GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install iscc-lib`, then `IsccLib.conformance_selftest`).

## How-to examples show the Sum-Code prefix as `KAA` `low` [review]

**Triage:** untriaged — a reader who compares their own output with the comment in a how-to sees
`ISCC:KUA…` where the docs say `"ISCC:KAA..."`.

`gen_sum_code_v0` composes a Data-Code and an Instance-Code, so without `wide` its result has
SubType SUM and starts with `ISCC:KUA` (measured in iteration 238:
`ISCC:KUABGAWXH2VYUQWIVYLQGD2NZH6TM` for the Java how-to's example file). The Sum-Code examples in
the how-tos under `docs/howto/` and in `docs/java-api.md` carry the comment `"ISCC:KAA..."`, and the
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
`push: tags: [v*.*.*]` lines in the "Trigger Configuration" YAML block. Pre-existing. The decision
of 2026-11-20 on spec criteria that name a mechanism that does not exist applies: realign the spec,
no escalation.

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

## Go Meta-Code differs from every other surface when metadata contains `&`, `<`, `>` or U+2028/9 `normal` [human]

`parseMetaJSON` (`packages/go/code_meta.go`) canonicalizes the `meta` JSON with `json.Marshal`. Go's
encoder writes `&`, `<` and `>` as `\u0026`, `\u003c`, `\u003e`, and U+2028/U+2029 as
`\u2028`/`\u2029` even with `SetEscapeHTML(false)`. RFC 8785 (JCS), which iscc-core and the Rust
core use, emits all five raw. So for `{"title":"Tom & Jerry"}` the Go package hashes different
bytes: `GenMetaCodeV0` returns a different Meta-Code and metahash, and `JsonToDataUrl` a different
data-URL, than Rust, Python and the other bindings. Measured 2026-11-21: Go prints
`{"title":"Tom \u0026 Jerry \u003c3"}`, `iscc_lib.json_to_data_url` encodes the raw `&` and `<`. The
published Go 0.7.0 has the defect; no conformance vector carries these characters, so CI is green.
Found by the iteration-240 audit (its workflow was killed before verification).

Resolved when Go's canonical bytes match RFC 8785 for these characters and a shared vector
(`data-edge.json` or equivalent) with `&`, `<`, `>`, U+2028 and U+2029 in `meta` passes on every
surface, Go included.

## Core video hashing panics or returns a wrong digest on frame signatures of unequal length `normal` [human]

`soft_hash_video_v0` (`crates/iscc-lib/src/lib.rs`) sizes its column sum from `frame_sigs[0]` and
indexes it with every deduplicated signature. A later signature longer than the first indexes out of
bounds and panics; the release profile has `panic = "abort"`, so the host process (Node, JVM, .NET,
Ruby, Go via FFI, ...) dies. A shorter one is silently summed into a wrong digest, and since the
BTreeSet sorts signatures while `cols` comes from input position 0, the result depends on input
order. Only the Python binding validates frame lengths before calling the core; every other surface
reaches the core unchecked, through `soft_hash_video_v0` and `gen_video_code_v0`. Found by the
iteration-240 audit and confirmed by reading the code 2026-11-21.

Resolved when the core returns `InvalidInput` for signatures of unequal length (no panic, no
digest), a shared error vector covers it on every surface, and the Python-only frame check is
removed or reduced to the core's error.

## C FFI treats `(NULL, 0)` two opposite ways and never checks inner pointers `normal` [audit]

`crates/iscc-ffi/src/lib.rs` rejects NULL with len 0 in image/audio/data/instance/`encode_base64`
(357-360, 513-516) but accepts it in `encode_component`, `decode_seq`, `alg_cdc_chunks`, … (799-807,
1311-1320); tests pin both (`tests.rs:131` vs `:629`). Pointer-array loops (438-445, 1232-1239,
1403-1410) pass inner pointers to `from_raw_parts` unchecked (UB on NULL). The split cost 7 .NET
sentinel branches (`IsccLib.cs:131…502`) and C++ `detail::safe_data`. **Fix:** one `slice_or_empty`
helper (len 0 → `&[]`, NULL + len > 0 → error) used everywhere, incl. inner pointers; flip the len-0
null tests. **Scope estimate:** 2-4 non-test files.

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

## Pytest gate spends ~25 s of ~44 s on benchmark loops and doc-tree copies `normal` [audit]

Gate rose 25.6 → 44.1 s over four `metrics.jsonl` rows. `tests/test_benchmarks.py` runs timing loops
every gate (13.8 s; 0.7 s with `--benchmark-disable`), duplicating `benchmarks/python/`.
`tests/test_check_tier1_docs.py:54-60` copies lib.rs + 91 docs off 9p per test, incl. 13 cases that
fire before any doc is read (`scripts/check_tier1_docs.py:282-289`). **Fix:**
`addopts = "--benchmark-disable"`; copy only what each case reads. **Scope estimate:** 1-2 files.
