# Postponed Issues

Issues the human moved out of `issues.md` because they are not relevant to the release being worked
on. **The CID loop never acts on an entry here**: no role reads this file except the audit, which
reads it only so it does not file the same problem again.

**Only the human moves entries.** To postpone, move the whole entry here unchanged and add a
`**Postponed:**` line under its heading (date, and the release or event to revisit it at). To
reactivate, move it back into `issues.md` and delete that line. Entries keep the `issues.md` format;
the file has no line budget, because no role loads it into its context.

<!-- Postponed entries below this line -->

## Track upstream decision on Unicode version drift `low` [human]

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit when iscc-core#137 is decided.

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

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit when one of the three triggers below
fires.

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

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit when Titusz sets the milestone after
v0.8.0.

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

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit together with the v1.0.0 release.

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

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit after v0.8.0, if the JNI throw helpers
change.

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

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit at the next napi upgrade, or after
reporting it to napi-rs.

napi 3.11.0's `Buffer::from_napi_value` (`napi-3.11.0/src/bindgen_runtime/js_values/buffer.rs:489`)
creates a reference before `napi_get_buffer_info` and returns on its failure without deleting it. So
passing an `ArrayBuffer`, plain array or other non-Buffer object to any `Buffer` parameter of
`@iscc/lib` throws as expected but keeps the object referenced for the life of the Node environment.
Every `Buffer` parameter in `crates/iscc-napi/src/lib.rs` has this shape. Pre-existing (iter 204).
Unverified fixes: a napi release that frees the reference, or `#[napi(strict)]`.

Resolved when repeated `ArrayBuffer` calls to a `Buffer`-taking export show no reference or heap
growth, and every existing Buffer/Uint8Array input still works.

## The Rust core does not build for bare-metal (`no_std`) targets `normal` [human]

**Postponed:** 2026-11-21, not relevant to v0.8.0; revisit when Titusz sets the milestone after
v0.8.0.

`iscc-lib` needs `std`, so it cannot be used on microcontrollers. A spike on 2026-11-21 (scratch
copy of `b94e095`, interactive session) showed the change is small: with it the core builds for
`thumbv7em-none-eabihf`, and all 442 core tests still pass on the host. What the spike did:

- A `std` feature, on by default, that forwards to `blake3`, `data-encoding`, `hex`, `serde_json`,
    `thiserror` and `unicode-normalization` (each declared with `default-features = false`, the
    middle three with `alloc`). Every binding crate uses the default features and is unaffected.
- `#![cfg_attr(not(any(feature = "std", test)), no_std)]`, `extern crate alloc`, and one `alloc`
    import line per module (46 lines added, 11 changed across the 11 files in `src/`).
- `libm` for `log2`/`round` in `cdc.rs` and `cos` in `dct.rs`. The conformance vectors and the
    210-value CDC parameter test match with it.
- `gen_sum_code_v0` (file path) and the `eprintln!` output of `conformance_selftest` only with
    `std`.

Not solved by the spike: `meta-code` needs `std`, because `serde_json_canonicalizer` uses `std::io`.
`gen_meta_code_v0` and `json_to_data_url` on bare metal need an own JCS serializer or an upstream
change. Nothing was executed on a device or an emulator. A heap allocator is required (`alloc`), and
16-bit targets stay out (the code assumes `usize` of 32 bits or more).

Open choices for the real change: keep the `std` float functions when `std` is on, so the current
code path stays as it is; whether Meta-Code on bare metal is in scope. A Rust user with
`default-features = false` loses `gen_sum_code_v0` until they add `std` (accepted in 0.x, needs a
release note).

Resolved when `cargo build -p iscc-lib --no-default-features --target thumbv7em-none-eabihf` (also
with `--features text-processing`) is a CI step, the feature is documented in the crate README and
`specs/rust-core.md`, and a decisions.md entry records the choice on Meta-Code.

**Spec:** `.claude/context/specs/rust-core.md`
