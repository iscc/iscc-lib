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

## `data-edge.json` harnesses accept a value that only resembles the expected one `normal` [human]

Swift: `compareValue` in `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift` compares with
`String ==`, which uses canonical equivalence (`"K" == "\u{212A}"` is `true`, checked on 6.1.2), and
reads expected numbers with `as? Int`, which also accepts a JSON `false`/`true` from
`JSONSerialization`. Python: the harness template conflates `0` and `False`. No current vector has a
non-ASCII or Boolean expected output, so nothing slips through today; the first one added would get
a weaker check on these surfaces with no harness edit to warn anyone (`edge-vectors.md` → "A vector
costs one edit"). Found by Codex in iter 218. Promoted by Titusz 2026-11-20.

Resolved when the Swift compare matches strings by Unicode scalars and rejects Boolean-backed
numbers (on Linux and macOS), and the Python harness rejects a `bool` where a number is expected and
vice versa.

**Spec:** `.claude/context/specs/edge-vectors.md` → "Rules"
