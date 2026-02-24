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

## [normal] `iscc_decompose` panics on malformed/truncated input

In `crates/iscc-lib/src/codec.rs:463-526`, the `iscc_decompose` function uses unchecked slicing in
multiple locations:

- Line 475: `&body[..nbytes]` — no check that `body.len() >= nbytes`
- Line 486: `&body[..16]` — no check that `body.len() >= 16` (wide mode)
- Line 488: `&body[16..32]` — no check that `body.len() >= 32` (wide mode)
- Line 501: `&body[idx * 8..]` — no check for dynamic unit indexing
- Lines 511/518: `&body[body.len() - 16..]` / `&body[body.len() - 8..]` — panics if
    `body.len() < 16`

This is a Tier 1 public API. Malformed base32 input can produce truncated bodies that trigger panics
instead of returning `IsccError::InvalidInput`.

Fix: add bounds checks before each slice operation and return `IsccError::InvalidInput` for
truncated bodies.

**Source:** [human]

## [normal] `soft_hash_codes_v0` accepts too-short Content-Codes (diverges from reference)

In `crates/iscc-lib/src/lib.rs:574-603`, the `soft_hash_codes_v0` function does not validate that
each input Content-Code has a decoded length >= `bits`. Instead, it pads shorter bodies with zeros
(lines 596-598).

Reference behavior (`reference/iscc-core/iscc_core/code_content_mixed.py:88-90`):

```python
unit_lengths = [ic.decode_length(t[0], t[3]) for t in code_tuples]
if not all(ul >= bits for ul in unit_lengths):
    raise AssertionError(f"Code to short for {bits}-bit length")
```

The Rust code silently accepts invalid inputs and produces a digest that differs from what the
reference would reject. This can hide caller bugs and is a correctness divergence.

Fix: after `decode_header`, call `decode_length(mtype, ln, stype)` and verify `>= bits`. Return
`IsccError::InvalidInput` if any code is too short.

**Source:** [human]

## [normal] `gen_meta_code_v0` treats empty Data-URL payload as "no meta"

In `crates/iscc-lib/src/lib.rs:183-184`, when a Data-URL is provided but its decoded payload is
empty (`b""`), the Rust code maps it to `None`, routing into the "no meta" branch (name/description
path).

Reference behavior (`reference/iscc-core/iscc_core/code_meta.py:62-68`): Python's `if meta:` is True
for any non-empty string (including a Data-URL with empty payload), so it enters the meta branch,
computes `soft_hash_meta_v0(name, b"")` and `multi_hash_blake3(b"")`, and returns the `meta` field
with the Data-URL.

This produces different `metahash` values and different output fields for the same input.

Fix: treat "meta string present" as a distinct case even when the decoded payload is empty bytes. An
empty `Vec<u8>` should still be `Some(vec![])`, not `None`.

**Source:** [human]

## [normal] `alg_simhash` panics on mismatched digest sizes

In `crates/iscc-lib/src/simhash.rs:13-31`, `alg_simhash` uses the first digest's length
(`hash_digests[0].as_ref().len()`) for iteration bounds without validating that all digests have the
same length. If a later digest is shorter, line 27 (`bytes[byte_idx]`) panics with an out-of-bounds
index. This is a Tier 1 public API (`pub fn alg_simhash`).

Fix: validate all digests have equal length, or use each digest's own length with min-clamping.
Return `IsccError` (requires changing the return type to `IsccResult<Vec<u8>>`).

**Source:** [human]

## [normal] `sliding_window` panics on `width < 2` via `assert!`

In `crates/iscc-lib/src/simhash.rs:53`, `sliding_window` uses `assert!(width >= 2, ...)` which
panics on invalid input. This is a Tier 1 public API bound to all languages. Panics in library code
are DoS vectors when inputs come from untrusted sources.

The internal `sliding_window_bytes` at line 71 has the same issue.

Fix: return `IsccResult<Vec<String>>` for the public function (or handle gracefully). For the
`pub(crate)` variant, a `debug_assert!` or early return may suffice.

**Source:** [human]

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

## [low] `sliding_window` allocates O(n) Strings for n-gram generation

In `crates/iscc-lib/src/simhash.rs:52-63`, `sliding_window` returns `Vec<String>`, allocating a new
`String` for every n-gram. For large text inputs this means O(n) heap allocations and O(n × width)
total copied characters. This impacts `gen_text_code_v0` and metadata simhash computation — two of
the most frequently called functions.

The reference Python implementation uses generators. Rust could hash n-grams on the fly without
materializing all substrings, or at minimum use `&str` slices into the original char buffer.

Fix: refactor to either (a) an iterator that yields slices and feeds directly into the hash
computation, or (b) a pre-allocated buffer reused across n-grams. After fixing, re-run
`cargo bench -p iscc-lib` and compare `gen_text_code_v0` and `gen_meta_code_v0` timings against the
baseline to validate the improvement.

**Source:** [human]

## [low] Codec header parsing expands bytes to `Vec<bool>`

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

## [low] `DataHasher::update` copies input data on every call

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
