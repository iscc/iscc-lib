# Issues

Tracked issues and feature requests for the CID workflow.

## Purpose

This file is a lightweight, append-only channel for humans and agents to flag problems that
define-next can prioritize. It bridges the gap between ad-hoc observations and the formal
state→target planning loop.

## Format

Each issue is an H2 heading with a priority tag, description, and source attribution:

```markdown
## [priority] Issue title

Description of the problem, including context and any relevant file paths or error messages.

**Source:** [human|review|advance]
**Spec:** .claude/context/target.md#section-name (optional — only if rooted in a spec gap)
**Upstream:** iscc/iscc-core (optional — only if the fix belongs in the upstream reference)
```

### Spec field

The optional `**Spec:**` field links an issue to a specific section in target.md or a sub-spec file
that needs updating. Source determines what happens:

- **`[human]` + `Spec:`** — the review agent updates the referenced spec directly when resolving the
    issue. The human created the issue, so the spec change is implicitly authorized.
- **`[review]`/`[advance]` + `Spec:`** — the review agent flags with `HUMAN REVIEW REQUESTED` and
    describes the proposed spec change in the issue description. It does NOT modify target.md. The
    human must approve and make the change.

### Upstream field

The optional `**Upstream:**` field marks an issue as belonging to an external repository (e.g.,
`iscc/iscc-core`). Upstream issues are always human-gated — filing on GitHub is a visible public
action that requires review regardless of source:

- Any issue with `**Upstream:**` triggers `HUMAN REVIEW REQUESTED` in the CID loop
- The issue description should include concrete evidence: failing conformance vectors, expected vs
    actual output, specific code references in the upstream repo
- The issue stays in issues.md as a draft until the human reviews it and either asks an interactive
    session to file it via `gh issue create -R <repo>` or files it manually
- After filing, update the issue description with the GitHub issue URL and delete it from issues.md

## Priority Levels

- **critical** — must be addressed in the next iteration; overrides normal gap analysis
- **normal** — considered alongside state→target gaps; define-next weighs it against other work
- **low** — pick up when no higher-priority work remains

## Management Rules

- **Append only** — add new issues at the end of this file
- **Resolution** — the review agent deletes resolved issues after verifying the fix (history lives
    in git)
- **Source tags** — agents that add issues must include a source tag: `[human]`, `[review]`, or
    `[advance]`
- **Scope** — track any problems that affect correctness, architecture, maintainability, or
    performance; not style preferences or minor nits

<!-- Add issues below this line -->

## [low] `alg_dct` allows non-power-of-two even lengths

In `crates/iscc-lib/src/dct.rs:19`, the input validation checks for non-empty and even length but
does not enforce power-of-two. The recursive Nayuki algorithm assumes repeated halving to 1, so
inputs like 6, 10, or 12 produce incorrect output. The docstring at line 15 says "Input length must
be a power of 2 (or 1)" but this is not enforced.

This is `pub(crate)` and current callers always use 32, so risk is low, but it's a correctness
landmine if `alg_dct` is reused for other sizes.

Fix: change the check to `n.is_power_of_two()` (which also handles n=0 and n=1 correctly).

**Source:** [human]

## [low] `alg_wtahash` panics on input vectors shorter than 380 elements

In `crates/iscc-lib/src/wtahash.rs:287`, `alg_wtahash` indexes into `vec[i]` and `vec[j]` using
permutation table indices up to 379 without checking `vec.len() >= 380`. Short input vectors cause a
panic.

This is `pub(crate)` (internal only), and the sole caller (`soft_hash_video_v0`) always provides
380-element vectors from MPEG-7 frame signatures, so the risk is low in practice.

Fix: add `if vec.len() < 380 { return error }` guard at function entry.

**Source:** [human]

## [normal] Codec header parsing expands bytes to `Vec<bool>`

In `crates/iscc-lib/src/codec.rs:121-127`, `bytes_to_bits` expands every byte into 8 `bool` values
and returns `Vec<bool>`. This is used in `decode_header` (line 245) which is called on every codec
operation (decompose, mixed-code hashing, conformance checks, etc.).

Headers are small (typically 2-8 bytes) so the per-call cost is modest, but the pattern is avoidable
— varnibble decoding can work directly on bytes with bitwise operations, eliminating the
intermediate allocation entirely.

Fix: replace `bytes_to_bits` + index-based varnibble decoding with direct bitwise extraction from
the byte slice. After fixing, re-run `cargo bench -p iscc-lib` and compare `gen_mixed_code_v0` and
`iscc_decompose` timings against the baseline.

**Source:** [human]

## [normal] `DataHasher::update` copies input data on every call

In `crates/iscc-lib/src/streaming.rs:88-93`, every `update()` call either copies the input via
`data.to_vec()` or concatenates tail + data via `[self.tail.as_slice(), data].concat()`. The tail is
also re-copied at line 108.

For high-throughput streaming scenarios (large files with many small `update()` calls), this creates
significant allocation overhead. A reusable internal buffer with `Vec::reserve` +
`extend_from_slice` (or a ring buffer for the tail) would avoid repeated allocations.

Fix: replace per-call allocations with a persistent internal buffer that grows as needed and is
reused across calls. After fixing, re-run `cargo bench -p iscc-lib` and compare `gen_data_code_v0`
timings against the baseline, and also benchmark `DataHasher` streaming with various chunk sizes.

**Source:** [human]

## [low] Evaluate unofficial TypeScript port branciard/iscc-core-ts

An unofficial TypeScript implementation of ISCC exists at `branciard/iscc-core-ts`. Two actions:

1. **Conformance check**: verify whether it passes the official `data.json` test vectors. If it
    does, it could be referenced as a community implementation. If not, note the gaps.
2. **Documentation mention**: if conformant (or partially conformant), mention it in the iscc-lib
    documentation site (e.g., in an "Ecosystem" or "Related Projects" section) as an independent
    community port alongside iscc-lib's own bindings.

This is not urgent — iscc-lib's own Node.js/WASM bindings will serve the JS/TS ecosystem. But
acknowledging community implementations builds goodwill and helps adopters find options.

**Source:** [human]

## [normal] iscc-wasm: `alg_cdc_chunks` silently returns null on serialization failure

In `crates/iscc-wasm/src/lib.rs:249`, `alg_cdc_chunks` uses
`serde_wasm_bindgen::to_value(&chunks).unwrap_or(JsValue::NULL)` which silently swallows
serialization errors and returns `null`. This is inconsistent with the crate's general "throw on
error" approach and makes failures invisible to callers.

Fix: change the return type to `Result<JsValue, JsError>` and propagate the serde error, or build a
`js_sys::Array` explicitly.

**Source:** [human]

## [normal] iscc-ffi: Video functions allocate/copy every frame signature

In `crates/iscc-ffi/src/lib.rs`, both `iscc_gen_video_code_v0` (lines 369-376) and
`iscc_soft_hash_video_v0` (lines 919-926) allocate a new `Vec<Vec<i32>>` by copying every frame
signature via `.to_vec()`. For videos with hundreds or thousands of frames, this creates significant
allocation overhead.

The underlying `iscc_lib` functions require `&[Vec<i32>]`, so the wrapper must materialize the data.
If `iscc_lib` could accept `&[&[i32]]` instead, the FFI layer could avoid per-frame allocations.

Fix: consider changing the `iscc_lib` video API to accept `&[&[i32]]` (borrowed slices), then update
all FFI/binding wrappers to pass slices directly. After fixing, benchmark video code generation with
varying frame counts.

**Source:** [human]

## [low] iscc-jni: All exceptions mapped to `IllegalArgumentException`

In `crates/iscc-jni/src/lib.rs:34`, the `throw_and_default` function always throws
`java/lang/IllegalArgumentException` for all error types. State violations (e.g., hasher already
finalized) should throw `IllegalStateException` instead.

Fix: add a `throw_state_error` variant that throws `IllegalStateException` and use it for
state-related errors (finalized hashers, etc.).

**Source:** [human]

## [low] iscc-py: Missing `__version__` attribute

`crates/iscc-py/python/iscc_lib/__init__.py` does not expose a `__version__` attribute. The Python
reference (`iscc-core`) exports `__version__`, and standard Python tooling expects it for runtime
version detection.

Fix: add `__version__` via `importlib.metadata.version("iscc-lib")` or let maturin inject it.

**Source:** [human]

## [low] iscc-py: Module docstring references wrong package name

In `crates/iscc-py/src/lib.rs:1`, the module docstring says `iscc._lowlevel` but the actual module
name is `iscc_lib._lowlevel` (per `pyproject.toml` `[tool.maturin].module-name`).

Fix: update the docstring to reference `iscc_lib._lowlevel`.

**Source:** [human]

## [low] iscc-wasm: `conformance_selftest` unconditionally exported increases binary size

In `crates/iscc-wasm/src/lib.rs:197`, `conformance_selftest()` is exported via `#[wasm_bindgen]`
without a feature gate. This pulls embedded JSON test vectors and parsing logic into every WASM
binary, increasing bundle size for browser consumers who don't need diagnostics.

Fix: gate the export behind a Cargo feature (e.g., `feature = "conformance"`) that is off by default
for production builds.

**Source:** [human]

## [low] iscc-wasm: Stale CLAUDE.md says DataHasher/InstanceHasher not yet bound

In `crates/iscc-wasm/CLAUDE.md:130-131`, the documentation states "DataHasher and InstanceHasher
(streaming types) are not yet bound." Both are now fully exported in `lib.rs` with constructor,
`update()`, and `finalize()` methods.

Fix: update CLAUDE.md to reflect the current state of the bindings.

**Source:** [human]
