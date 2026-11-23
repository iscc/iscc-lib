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

## Ruby 4.0 gets no precompiled gem `low` [review]

**Postponed:** 2026-11-23, revisit for v0.8.1 (needs an rb_sys upgrade, too risky right before
v0.8.0).

**Triage:** untriaged. Every user on Ruby 4.0, the current stable series, installs the source gem
and needs a Rust toolchain and `libclang-dev`; the docs say so.

The cross-gem image is pinned to rb-sys 0.9.123, whose newest cross Ruby is 3.4.5. Ruby 4.0 needs
rb_sys 0.9.124 or newer, and the hold comment in `crates/iscc-rb/Gemfile` says that version breaks
`RbSys::ExtensionTask`. Unverified since the hold was written.

Resolved when the cross-gem list includes 4.0 and the docs name the range, or a decisions.md entry
keeps 4.0 on the source gem.

## `@iscc/wasm` list inputs read through serde let a throwing element kill the instance `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. Only a caller whose array holds an element getter (or a proxy) that throws,
passed several thousand times to one instance, would hit it; ordinary arrays are unaffected.

`serde_wasm_bindgen::from_value` reads array elements through js-sys getters without `catch`, so a
JavaScript exception unwinds through the Rust frames. Measured in iteration 269 from Node on release
builds, with an array whose index 1 is a throwing getter: after 5,000 to 8,200 such calls every
export fails with `RuntimeError: memory access out of bounds`. Affected: `gen_video_code_v0` and
`soft_hash_video_v0` (inner arrays), `gen_mixed_code_v0`, `gen_iscc_code_v0` and `encode_seq`.
Pre-existing: the build of `3ef83ce` behaves the same. The number-array parameters (`cv`,
`features`) are safe since iteration 269 (`number_vec` reads through `Reflect::get`). No
`data-edge.json` vector is owed (a getter is not JSON-expressible).

Resolved when a throwing element on each of these parameters gives an ordinary error and leaves the
instance usable, pinned by a test per parameter, or a decisions.md entry keeps the limit on purpose.

## `crates/iscc-py/CLAUDE.md` still lists test files one by one `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. An agent that reads the "Test Patterns" list as complete would look for
Python binding tests in six modules only; the code is unaffected.

"Test Patterns" names six test modules with a description each (`test_smoke.py`,
`test_new_symbols.py` and four more), while the suite under `tests/` has more binding test modules
than that (`test_gil.py`, `test_seq.py`, `test_data_edge.py` among them). It is the kind of partial
file list the decision of 2026-11-21 removed from the per-crate `CLAUDE.md` files. Pre-existing,
reported by advance in iteration 275. No other `crates/*/CLAUDE.md` or `packages/*/CLAUDE.md` file
has such a list.

Resolved when the section states its conventions as rules on file names and lists no modules.

## Generated C header fills long strings from 4,095-byte literals, over one documented MSVC limit `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. Someone who builds `test_iscc.c` with MSVC would get a compile error (C2026)
if the older of two Microsoft pages is right; gcc and clang builds are unaffected.

`scripts/gen_data_edge_vectors.py` writes a string over `C_LITERAL_MAX` (4,095 bytes) as a static
buffer that `iscc_edge_init()` fills from literal pieces of up to 4,095 bytes (iteration 282).
Microsoft documents two limits for one quoted piece: 16,380 bytes on the error C2026 page, and 2,048
bytes under "Maximum String Length" in its C language reference (last updated 2021). The pieces fit
the first and exceed the second. Not measured: the devcontainer and CI have no MSVC.

Resolved when either every literal in the header is at most 2,047 bytes (a lower `C_LITERAL_MAX`,
which the literal-length test follows), or an MSVC build of `test_iscc.c` shows that 4,095-byte
pieces compile.

## `napi build` fails in the devcontainer checkout with the locked `@napi-rs/cli` 3.10.12 `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. Anyone who builds `@iscc/lib` in the devcontainer on a Windows host (the
checkout is a 9p bind mount) gets exit 1, and the CID roles cannot build napi in the checkout; CI
runners and users of the published package are unaffected.

Measured in iteration 284 at 33d9b43: after `npm ci`, `npx napi build --platform` in
`crates/iscc-napi` compiles and then fails with "Filesystem transaction failed and its recovery
state could not be removed". The nested errors are `ENOENT` from `link()`: the cli publishes its
outputs through hard links, which the drvfs 9p mount does not handle. Each failed run leaves
untracked files in the crate directory (`.*.tmp`, one of them a 20 MB copy of the addon, and
`.napi-rs-filesystem-transaction.swp/`), and the `.gitignore` does not cover them. The same commit
builds and passes its 238 tests in a `git archive` copy under `/tmp` and in the CI `nodejs` job. The
step did not choose the version: `^3` resolved to 3.10.12 for every fresh install already. The
earlier untracked local lock held 3.5.1, which works. Not measured: the first cli version that
fails.

Resolved when `npm ci && npx napi build --platform && npm test` passes in the devcontainer checkout
and leaves `git status` clean, or a decisions.md entry keeps napi builds out of the checkout.

## vcpkg port fails one post-build check `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. Whoever submits the port to the vcpkg registry (`specs/cpp-bindings.md` →
Distribution) would be asked to fix it first; an overlay-port install works.

Measured in iteration 285 with a fresh vcpkg on x64-linux:
`vcpkg install iscc --overlay-ports=<dir>` (a directory with `portfile.cmake` and `vcpkg.json`)
downloads the v0.7.0 archive, passes the hash check and installs the headers and libraries, exit 0.
Post-build validation then warns "mismatching number of debug and release binaries" and reports
"Found 1 post-build check problem(s)": the archives carry release binaries only, and the portfile
sets neither `VCPKG_BUILD_TYPE` nor a policy. Pre-existing. Not measured on Windows or macOS. No CI
job installs the port.

Resolved when the install prints no post-build check problem on x64-linux, or a decisions.md entry
keeps the port as an overlay port only.

## No test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift and Kotlin `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. No user is affected today; a later reordering in `iscc-ffi`, `iscc-uniffi` or
the .NET wrapper could bring back a hasher that stays usable after a failed `finalize`, against what
the how-to pages promise, and no suite would see it.

Iteration 297 stated the rule on the how-to pages of every surface and pinned it with tests on WASM,
napi, JNI, Python and Ruby; Go has its test since the decision of 2026-11-21. The surfaces named
above comply by code order: `take()` (C FFI, UniFFI) or the finalized flag (.NET) comes before the
core call, C++ wraps the C call, and `bits` is unsigned by type. That was checked by reading. Their
suites test a second `finalize` only after a *successful* first one (`crates/iscc-ffi/src/tests.rs`,
the tests in `crates/iscc-uniffi/src/lib.rs`, `SmokeTests.cs`). The Rust core needs no test:
`finalize(self)` moves the hasher.

Resolved when the C FFI, C++, .NET and UniFFI suites each have a test in which `finalize(13)` fails
and a following `finalize(64)` reports the finalized state.

## `specs/documentation.md` says every code example is tabbed, and the how-to guides have no tabs `low` [review]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. No user is affected; define-next could scope tabs into the eleven how-to
pages from a criterion that describes a layout the site never had.

**Spec:** `.claude/context/specs/documentation.md` → "Per-Language Code Examples" and the
verification criterion "All code examples use tabbed multi-language format".

The spec says all code examples use the tabbed format and that "all how-to guides use this pattern".
Tabs exist on two pages only: `docs/index.md` and `docs/tutorials/getting-started.md` (plus the
Maven/Gradle install tabs in `docs/java-api.md`). Each how-to guide covers one language and uses
plain fences (`git grep -c '=== "' docs/howto` finds nothing). Pre-existing, seen in iteration 307
next to the sentences that step rewrote.

Resolved when the section and the criterion name the pages that show several languages side by side,
as a rule and without a page list that can fall behind (decision of 2026-11-20 on criteria that name
a mechanism that does not exist).

## The devcontainer and CI count different instructions on three iai rows `low` [human]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

`.iai-baseline.json` holds the devcontainer value on three rows, so the CI `perf` gate accepts a
cost of 1.3 to 1.5 times what CI measures on them before it fails; the other 13 rows hold the CI
value. No user is affected.

Measured 2026-11-22 at 90565d4b with rustc 1.100.0 on both sides (CI run 45048776810):

| Row                              | CI        | Devcontainer | Difference |
| -------------------------------- | --------- | ------------ | ---------- |
| `bench_image_code.gradient_1024` | 1,521,274 | 2,105,741    | +38%       |
| `bench_iscc_code.four_units`     | 10,985    | 12,669       | +15%       |
| `bench_mixed_code.two_codes`     | 9,309     | 10,677       | +15%       |

The other 13 rows agree within 5%. CI runs on `ubuntu-latest`, now Ubuntu 26.04 (glibc 2.43,
valgrind 3.26); the devcontainer is Debian 12 (glibc 2.36, valgrind 3.19). The three rows spend much
of their cost in system-library code. With the CI values, `mise run bench:iai:check` fails in the
devcontainer with no code change, so Titusz chose to keep the devcontainer values on these rows
(2026-11-22); the `--update` paragraph of the `scripts/iai_regression.py` docstring says how to
measure them at each refresh. A runner image update can move the CI values again. Not measured:
whether glibc or valgrind accounts for the difference.

Resolved when the `perf` job and the devcontainer measure in the same system-library environment
(for example the job runs in a container of the devcontainer's base image), all 16 rows come from
one run, and the docstring paragraph about the three rows is gone.

## A release recovery can publish an FFI archive or XCFramework zip that the tag's hash rejects `low` [human]

**Postponed:** 2026-11-23, revisit after the v0.8.0 release.

**Triage:** untriaged. Affects the vcpkg, Conan or SwiftPM users of one release, and only after a
recovery from a failed release run; a release that succeeds in one run is consistent.

The tagged commit pins the hashes of the five FFI archives (`packages/cpp/portfile.cmake`,
`packages/cpp/conandata.yml`) and the checksum of `IsccLib.xcframework.zip` (`Package.swift`). Two
paths of `release.yml` put a rebuilt file, which has another hash, on a release whose tag exists:

- A dispatch with `ffi=true` rebuilds the archives, and `publish-ffi` uploads every archive the
    release lacks. It checks names and upload state only (decisions.md 2026-11-22), so it succeeds.
- A re-run of `prepare-release` after the tag was pushed (for example when "Create GitHub Release"
    failed) rebuilds the XCFramework, because a failed job saves no cache (unless the job restored
    the zip from the cache at its start and the entry is still there). It commits the new checksum
    to `main` after the tag, leaves the tag in place and uploads the new zip. A dispatch with
    `version=` on an existing tag does the same, and commits FFI hashes that no published archive
    has.

The release skill tells the agent to avoid both paths and to compare the asset hashes with the tag
("Assets that the tag pins by hash"); the workflow does not refuse them. Reasoned from the workflow
text during a review on 2026-11-23, not reproduced: no release has failed this way.

Resolved when `publish-ffi` refuses to upload an archive whose hash the tagged `conandata.yml` does
not hold, `prepare-release` fails before its commit step when the tag exists and pins other
checksums than the run produced, and a test or `scripts/check_release_workflow.py` covers both.
