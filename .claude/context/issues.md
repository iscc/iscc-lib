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

## JNI streaming-hasher functions dereference Java-supplied handle without null guard `normal` [human]

`dataHasherUpdate`/`dataHasherFinalize`/`instanceHasherUpdate`/`instanceHasherFinalize` in
`crates/iscc-jni/src/lib.rs` dereference the `jlong` handle via `unsafe { &mut *(ptr as *mut …) }`
with no `ptr != 0` guard (only the `*Free` functions check). A null or stale handle passed from Java
is UB. Pre-existing (not introduced by the jni 0.22 migration) and mitigated by the Java wrapper
owning the handle lifecycle, but a one-line `if ptr == 0` → throw guard per function would make the
boundary robust against misuse of the public JNI surface. Surfaced during the PR #44 review sweep
(2026-07-30).

Resolved when each of the four functions throws on a zero handle instead of dereferencing it, pinned
by a Java test per function.

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

## `crates/iscc-ffi/src/lib.rs` is past the crate's own split threshold `normal` [review]

`crates/iscc-ffi/CLAUDE.md` says "do not split into submodules unless the file exceeds ~2500 lines";
`lib.rs` is now 2870 (1715 of exported code, ~1155 of `mod tests` from line 1716). Constraint on any
split: `build.rs` feeds csbindgen only `input_extern_file("src/lib.rs")`, so moving `extern "C"`
functions into submodules silently drops them from `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
unless `build.rs` lists the new files. Moving only `mod tests` to `src/tests.rs` avoids that.

Resolved when `lib.rs` is back under the documented threshold (or the threshold is deliberately
changed in `crates/iscc-ffi/CLAUDE.md`), and `cbindgen` + `cargo build -p iscc-ffi` leave `iscc.h`
and `NativeMethods.g.cs` byte-identical.

## JNI exports report a pending JVM exception as `IllegalArgumentException` `normal` [review]

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

## Ruby `i32` list elements silently truncate negative Bignums `normal` [review]

`IsccLib.gen_audio_code_v0([-(2**63)])` returns the code of `[0]`, where other out-of-range elements
raise `RangeError`. Magnus 0.8.2 `RBignum::to_i32` (`r_bignum.rs:194`) checks only the upper bound
before `as i32`, so Bignums from -2**63 to -(2**62)-1 truncate. It reaches every `i32` element
conversion in `crates/iscc-rb/src/lib.rs` (`gen_audio_code_v0`, `gen_video_code_v0`,
`soft_hash_video_v0`), contradicting `docs/ruby-api.md` Error Handling. Pre-existing; Codex, iter
212\. `u32` elements were not probed.

Resolved when a negative Bignum element in those three functions raises `RangeError` (e.g. convert
through `i64`, narrow with `i32::try_from`), pinned next to the `RangeError` cases in
`crates/iscc-rb/test/test_array_args.rb`.

## `scripts/check_spec_paths.py` accepts paths the specs do not have `normal` [review]

The spec path gate (iteration 224) is looser than its docstring, in three ways (Codex, iter 224; the
first two confirmed with `resolves()` on the real index):

1. **`*` crosses `/`**: `fnmatch.fnmatchcase` lets `*` match a slash, so `packages/go/*.json`
    resolves through `testdata/data.json` although `packages/go/` holds no JSON file. A spec can
    name the wrong directory and stay green.
2. **`NOT_PATHS` applies to tree entries**: the exemption for the removed
    `scripts/gen_ffi_boundary_vectors.py` (cited as history in `edge-vectors.md`) also lets a tree
    entry `gen_ffi_boundary_vectors.py` under a `scripts/` root pass, where the line asserts the
    file exists.
3. **The historical Go replay test reads the live index**:
    `test_old_go_tree_reports_its_eleven_phantoms` and `test_main_exits_nonzero_on_phantom` resolve
    a frozen tree against `tracked_paths(ROOT)`, so renaming `packages/go/dct.go` (which no current
    spec lists) fails them with a twelfth error. `test_real_repo_passes` already covers the live
    index.

Resolved when `*` stays within one path segment while `**` recurses (the six `**` spans in
`swift-bindings.md` still resolve), `NOT_PATHS` is honoured only for backticked spans, the two
replay tests use a fixed known-path set, and one test pins each of the three cases.

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

## `alg_simhash` accepts an empty list; C++ list wrappers pass NULL for an empty vector `normal` [human]

On `alg_simhash([])` the surfaces disagree: the Rust core and the Go port return 32 zero bytes (an
undocumented constant, `alg_simhash_inner` / `AlgSimhash`), so do the bindings that pass the list
through; C and C++ fail with the FFI's null-pointer error; iscc-core crashes with `IndexError`.
Titusz decided 2026-11-20 (decisions.md): `alg_simhash` rejects an empty list. Only the public
checked function changes; `alg_simhash_inner` keeps its internal contract. **Trap in Go:** the
generators in `code_content_audio.go`, `code_content_mixed.go` and `code_meta.go` call the public
`AlgSimhash`, discard its error and rely on the 32 zero bytes for an empty list (short audio). They
must move to an unexported unchecked helper, as Rust's generators use `alg_simhash_inner`, or Go
output changes silently.

Separately, five wrappers in `packages/cpp/include/iscc/iscc.hpp` (`gen_mixed_code_v0`,
`gen_iscc_code_v0`, `gen_video_code_v0`, `alg_simhash`, `soft_hash_video_v0`) pass `ptrs.data()`,
NULL when empty, where `encode_seq` uses `detail::safe_data`. So `{}` fails with the FFI's NULL
message instead of reaching the core's own check (iter 219).

Order: the core and Go change lands first and alone (decisions.md 2026-11-19, "Core changes go first
and alone"), then the C++ fix and the vector reach all surfaces.

Resolved when the Rust core and the Go port reject `alg_simhash([])` with an error, an `alg_simhash`
`[]` vector with `error` in `data-edge.json` passes on all 12 surfaces, and the five C++ wrappers
pass a non-NULL pointer, each pinned by a test on `{}`.

**Spec:** `.claude/context/specs/rust-core.md` → Tier 1 table (`alg_simhash`)
