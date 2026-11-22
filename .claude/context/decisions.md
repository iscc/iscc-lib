# Decision Records

Log of judgment calls that a future reader could not reconstruct from the code alone: deviations
from the iscc-core reference or a spec, design trade-offs between real alternatives, accepted
performance regressions, justified suppressions, API-shape choices. Written by the review agent when
approving such a call; human entries welcome. Distinct from learnings.md: learnings capture
operational gotchas, decisions capture design rationale.

**A recorded decision is binding.** CID agents must not re-open, re-derive or re-litigate a
trade-off settled here. Superseding one takes a new fact and a new dated entry that says what
changed.

**Size.** One entry per review, at most 12 lines. The file has a 400-line budget; `tools/cid.py`
rotates the oldest entries into `decisions-archive.md` when it is exceeded (oldest-first, no
judgment applied), so entries are written to be read now, not to be permanent. Rotation does not
retire a ruling: archived **titles** stay in every agent's context alongside the live ones, and an
archived decision binds exactly as much as one still in this file. Only the bodies leave.

Entry format:

```markdown
## <YYYY-MM-DD> — <title>

**Decision:** <what was decided>
**Why:** <the reasoning, including constraints that forced it>
**Alternatives:** <what was rejected and why>
**Context:** <iteration N / commit sha / discussion reference>
```

<!-- Append entries below this line -->

## 2026-11-20 — The Ruby source gem is staged standalone: carried core, workspace `Cargo.lock`, conditional `rb_sys`

**Decision:** the `ruby`-platform gem is built only by `scripts/build_ruby_source_gem.py`. It stages
a tree outside the repository: a generated `Cargo.toml` (concrete keys, empty `[workspace]`), the
core from `cargo package -p iscc-lib` under `core/`, and the workspace `Cargo.lock`. The gemspec
declares `rb_sys ~> 0.9` only when `core/` sits next to it. **Why:** the gem must build with nothing
but its own files, and the lock makes an install resolve the crate versions CI tested. An
unconditional `rb_sys` dependency adds a version-less line to `Gemfile.lock` ahead of the pin that
`cross-gem@v1` greps, and gives the precompiled gems a runtime dependency. **Alternatives:**
depending on the published `iscc-lib` crate (the gem could only be released after crates.io, and a
checkout could not test it); publishing no source gem (Ruby 3.4+ and unlisted platforms would need a
checkout); an unconditional dependency plus a rewritten lock-link step in `release.yml` (nothing
tests that workflow before a release). **Context:** iterations 239-240.

## 2026-11-21 — Per-crate `CLAUDE.md` files carry no file lists; a rule is stated against a file name

**Decision:** the directory trees and `File`/`Module` tables are removed from every
`crates/*/CLAUDE.md` and `packages/*/CLAUDE.md`. A fact that a list entry carried stays only when it
is a rule or the file's origin ("generated, do not edit", "must sit next to `Cargo.toml`",
"gitignored build output"), written as a sentence that names the file. **Why:** the lists repeated
`git ls-files`. Every listed path existed, so the drift was missing entries, and an existence check
would have passed while fixing nothing. **Alternatives:** extending `scripts/check_spec_paths.py` to
these files with a completeness check (needs a parser and an exemption list for three formats, and
every new file would then need a `CLAUDE.md` edit); keeping the lists unchecked (breaks the "Derived
facts are checked or not written" hygiene rule). **Context:** iteration 246.

## 2026-11-21 — Spec trees stay as partial lists with a gated note; aggregate lists are removed

**Decision:** the structure trees in `.claude/context/specs/` stay. Each is followed by a paragraph
containing "not every file", and `scripts/check_spec_paths.py` fails on a tree without it. Entries
below a `GENERATED` prefix are dropped. The version-sync target table in `ci-cd.md` is replaced by a
pointer to `TARGETS` in `scripts/version_sync.py` (review edit: it listed 10 of 21 targets and said
"all targets listed above"). The one- and two-row "Version Sync" tables in the binding specs stay:
each states where that binding's version must live and claims no completeness. **Why:** a tree
comment carries a requirement ("DO NOT EDIT", where SPM, JNA and NuGet files live) and the gate
already fails on a stale entry, so only the claim of completeness had to go. **Alternatives:**
removing all trees with the parser, its floors and about 8 tests (every requirement comment
rewritten as prose in 9 specs); a completeness check per tree; completing the version-sync table
with a parity test against `TARGETS`. **Context:** iteration 247.

## 2026-11-21 — No role uses the advisor; the audit spend ceiling is 30 USD

**Decision:** no CID role session gets the server-side advisor: `session_settings` in `tools/cid.py`
sets no `advisorModel` (and so no longer blanks `DISABLE_TELEMETRY`), and the agent files carry no
advisor guidance. The audit's `--max-budget-usd` ceiling rises from 15 to 30. **Why:** human
decision. The iteration-240 audit hit its 15 USD ceiling at 15.03 USD in the middle of verification
and filed nothing; 5.64 USD of it was the advisor, because each of the Workflow's 9 subagents
consulted it 2-5 times, re-reading its transcript uncached. The advisor was also about 40-45% of
each define-next run's cost. **Context:** human decision (Titusz, interactive session, 2026-11-21);
supersedes the advisor parts of the 2026-11-17 and 2026-11-20 entries.

## 2026-11-21 — Video frame signatures of unequal length are rejected, where iscc-core truncates

**Decision:** `soft_hash_video_v0` / `gen_video_code_v0` in the Rust core and `SoftHashVideoV0` /
`GenVideoCodeV0` in Go return an error when a frame signature's length differs from the first one,
checked in input order before deduplication. Equal lengths other than 380 are handled as before. The
Python binding keeps its own check: it flattens the frames into one buffer before calling the core,
and raises `ValueError` with the same wording. **Why:** human issue. A longer later signature
indexed out of bounds, which aborts the host process under `panic = "abort"`; a shorter one gave a
digest that depended on input order. The reference's `zip(*sigs)` truncates to the shortest
signature and returns a digest (measured at 64 bits for `[380, 381]`, `[381, 380]` and
`[380, 379]`), so a ragged input is a `diverges` case in `data-edge.json`. **Alternatives:**
truncating like the reference (hashes malformed input silently); requiring exactly 380 (changes
equal-length inputs). **Context:** iteration 250.

## 2026-11-21 — The C FFI reads `(NULL, 0)` as an empty input on every pointer+length parameter

**Decision:** Every pointer+length input of `iscc-ffi` is borrowed through `slice_or_empty` (row
arrays through `rows_or_empty`). Length 0 is an empty slice whatever the pointer is. NULL with a
non-zero length is the error "<name> must not be NULL", inner rows included (`frame_sigs[1]`). A
count of 0 therefore reaches the core, and the caller sees the core's own error or result. A NULL
`const char*` string stays an error. **Why:** audit issue. Half the exports rejected `(NULL, 0)` and
half accepted it, a NULL inner row was undefined behaviour, and because `fixed` over an empty span
and `std::vector::data()` yield NULL, each wrapper needed sentinels. **Alternatives:** rejecting
NULL everywhere (keeps sentinels in every wrapper and hides the core's error); deciding per export
(the split that existed). **Context:** iteration 255. The review made the indexed row name lazy
(`format_args!`) after measuring 2x on `iscc_alg_simhash` with 1,000 rows.

## 2026-11-21 — Out-of-range integer arguments are per-binding tests, not `data-edge.json` vectors

**Decision:** napi and WASM take every numeric parameter as a JS `number` (`f64`) and range-check it
before narrowing; JNI rejects a negative `int` for `bits`, `nbytes` and `width`. Python keeps
`OverflowError` and Ruby `RangeError`, the classes each host raises itself when an integer
conversion fails, and the docs name them. No `data-edge.json` vector covers these inputs. **Why:**
audit issue. A value such as `2**32 + 64` for `bits` cannot be passed through the typed signatures
of the Rust, C, C++, Go, .NET, Swift and Kotlin harnesses: to them it is a wrong argument type,
which `edge-vectors.md` → Rules leaves to per-binding tests. **Alternatives:** a vector with
`except` entries for seven surfaces (mostly exceptions); wrapping Python/Ruby errors into
`ValueError`/`RuntimeError` (hides the host's own class). **Context:** iteration 256.

## 2026-11-21 — Triage: Python 3.10 and .NET 8 floors stay, Ruby moves to 3.3; CI adds 3.15 and .NET 10

**Decision:** the Python floor stays 3.10 (abi3-py310) and the .NET floor stays `net8.0`; the Ruby
floor rises to 3.3 (`normal` issue). CI tests Python 3.10 and 3.15 (floor and newest), and runs the
net8.0 build on the .NET 8 and .NET 10 runtimes (`dotnet-test` matrix, `dotnet` aggregator). Ruby
keeps Magnus' implicit `to_int` for integer lists (host idiom; docs fixed instead), and the C++
`safe_data` and .NET `PinnedRows` padding workarounds are removed. **Why:** human decision. A floor
that is forward-compatible (one abi3 wheel, one net8.0 assembly) costs nothing to keep once CI also
tests a supported runtime; Ruby needs a precompiled gem per series, so an end-of-life floor costs
build work. **Context:** human decision (Titusz, interactive session, 2026-11-21), low-issue triage.

## 2026-11-21 — Knowledge gaps: define-next researches on the web; experiments run as spikes

**Decision:** define-next may use WebSearch/WebFetch for behaviour outside the repository. A gap
that only an experiment can close becomes a `**Spike:**` step: advance runs the experiment as the
whole step and reports the answer with its evidence. Experiments never change the repository: they
run in a `mktemp -d` directory (a throwaway `git worktree` there when files must change), install
tools only there, and remove it all before the handoff. Review checks the cleanup and records the
answer in learnings.md. **Why:** human decision. A research role inside define-next would bypass
review and lose its findings; routing experiments through advance and review keeps them verified and
durable at no new runner cost. A riskiest-assumption section in every next.md was rejected: template
headings get filled by default. **Revisit:** remove both if about 50 iterations pass with no spike
and no web lookup. **Context:** human decision (Titusz, interactive session, 2026-11-21).

## 2026-11-21 — Go hashers: `Push` returns `error`, and the first `Finalize` consumes even when it fails

**Decision:** `DataHasher`/`InstanceHasher` set `finalized` at the entry of `Finalize`, so a call
that fails on an invalid `bits` still consumes the hasher; `Push` gained an `error` return.
`TextTrim` returns `""` for `nbytes <= 0` and keeps its `(string, int) string` signature. **Why:**
consuming on any first `Finalize` is what Rust (`finalize(self)`) and the Python binding (`take()`
before encoding) do, and it removes the retry that would append the Data-Code tail feature twice. An
`error` return keeps statement calls compiling; it breaks only a stored method value or a caller's
own `interface{ Push([]byte) }`, accepted in 0.x. **Alternatives:** flag set only on success (needs
a non-mutating feature list, and differs from every sibling); a panic or a silent no-op in `Push`;
`TextTrim` taking `uint` or returning an error (breaks callers passing an `int`); an exported
sentinel error (the package has none). **Context:** iteration 261.

## 2026-11-21 — `alg_cdc_chunks` accepts every `avg_chunk_size` from 2 up; large parameters saturate

**Decision:** Rust and Go compute iscc-core's CDC parameters for every `u32` and reject nothing new.
`max_size` (in Go also `center_size`) saturates at the platform integer, and a mask of 32 or more
bits is `u32::MAX`. **Why:** audit issue: from 2^29 the core panicked in debug builds and returned
`mask_s = 0` in release. iscc-core's pure-Python build accepts every value, so a bound would add an
error and a `diverges` marker for no gain (its compiled wheel raises `OverflowError` from 2^29,
found in iteration 264: see the next entry). Saturation changes no result: the sizes are only used
as `min(size, len(buffer))`, and the gear pattern stays below 2^32 (every gear value is below 2^31),
so the bits a wider mask adds are never set. Measured: both equal iscc-core on 210 values around
each point where `bits` changes. **Alternatives:** an upper bound such as `avg_chunk_size <= 2^28`
(the audit's other suggestion); a `u64` pattern and masks (slower hot loop, same chunks).
**Context:** iteration 263.

## 2026-11-21 — No shared `alg_cdc_chunks` vector from 2^29 up; no upper bound on `avg_chunk_size`

**Decision:** `data-edge.json` carries no `alg_cdc_chunks` vector with `avg_chunk_size >= 2^29`, and
iscc-lib adds no upper bound. Neither iscc-core build is the reference in that range. **Why:** ISO
24138 fixes the average chunk size at 1024, so other values are not an expected use. The range has
no memory-safety stake: the bounds of the unsafe scan come from the buffer length
(`min(size, len(buffer))`), not from the parameter. Validation runs once per call, outside the hot
loop, and the Data-Code path skips it, so a bound would cost nothing and buy nothing. The Rust and
Go unit tests pin the large-value parameters. The wheel's `OverflowError` is a side effect of
`uint32_t` in `cdc.pxd`, reported as iscc/iscc-core#141; nothing here waits on it. **Alternatives:**
vectors with a reference-check skip where the compiled build is loaded (a permanent harness special
case for an unused input); rejecting from 2^29 up like the wheel (copies an accidental overflow to
12 surfaces). **Revisit:** if iscc-core defines a deliberate bound, follow it. **Context:** human
decision (Titusz, interactive session, 2026-11-21), answering the iteration 264 escalation.

## 2026-11-21 — 32-bit coverage: a Go `GOARCH=386` CI step, no native 32-bit Rust leg

**Decision:** CI runs the Go suite once with `GOARCH=386`. The Rust core gets no native 32-bit test
leg. Bare-metal (`no_std`) support is postponed past v0.8.0 (postponed.md). **Why:** the Go port is
separate code that CI executes on amd64 only. The Rust core already runs on a 32-bit target on every
push: the WASM job executes the conformance and data-edge suites on `wasm32`. Measured 2026-11-21:
all 442 core tests pass on `i686-unknown-linux-musl` (debug build, overflow checks on), and
`GOARCH=386 CGO_ENABLED=0 go test ./...` passes. A native Rust leg needs extra setup, because the
bench dev-dependencies do not build there as they are: `alloca` (via `criterion`) needs a C
cross-compiler, and `iai-callgrind-runner` does not compile for `wasm32-wasip1`. **Alternatives:**
an `i686` `cargo test` leg with a C cross toolchain; leaving the Go port untested on 32-bit.
**Context:** human decision (Titusz, interactive session, 2026-11-21).

## 2026-11-21 — `@iscc/wasm` reads number arrays element by element through `Reflect.get`

**Decision:** `number_vec` reads `length` and each index through `Reflect::get` / `get_u32`, the
js-sys imports marked `catch`. The cost is accepted: about 2× the marshalling time of 0.7.0
(`gen_audio_code_v0`, release build, Node 22: 3,000 elements 0.79 → 1.65 ms, 1M elements 243 → 525
ms). An iterable without `length` (`Set`, generator) is rejected; 0.7.0 read it as empty. **Why:** a
JS exception that crosses a Rust frame without `catch` leaves the instance unusable after a few
thousand throws, and no built-in does the type check in one crossing (`Float64Array.from` coerces
`"2"` to 2). The extra 0.3 µs per element is small next to decoding audio. **Alternatives:** a
typed-array fast path (`dyn_ref` + copy; more code for one input form); an `inline_js` helper that
checks in JS (one crossing, but adds a `snippets/` directory to the published package). **Revisit:**
when a user reports number-array throughput. **Context:** iteration 269.

## 2026-11-21 — The development Ruby is a `mise.toml` `[tools]` pin, the only tool pinned there

**Decision:** the root `mise.toml` pins `ruby = "3.4.13"`; the standardrb hooks, the devcontainer
setup and the docs run `mise exec -- …` and name no version (`tests/test_ruby_gem_docs.py`). The pin
test reads `mise.toml` with a regex, as the docs-nav gate does (`tomllib` is missing on Python
3.10). **Why:** the gem floor (3.3) is above Debian Bookworm's Ruby 3.1, and before the pin three
files spelled two versions. mise fetches a prebuilt Ruby in about 13 s, so setup needs no compiler
step. **Accepted cost:** every `mise run <task>` on a machine without that Ruby installs it first
(measured with an empty `MISE_DATA_DIR`: `mise run version:check` downloaded 92 MB), and inside the
repo the pinned Ruby precedes any system Ruby on PATH. CI does not use mise, so the pin never
reaches it. **Alternatives:** a supported Ruby in the `Dockerfile` (a source build or a third-party
apt repository, and the hooks would still need one named version); pinning Rust, Python and Node too
(out of scope: the image and `uv` own those). **Context:** iteration 271.

## 2026-11-21 — Ruby integer parameters keep Ruby's implicit `to_int` conversion

**Decision (Titusz):** an integer parameter or integer list element in `iscc-rb` accepts a Float or
any `to_int` object and truncates it toward zero; `docs/ruby-api.md` states it. **Why:** Ruby host
idiom (`Array#[]` does the same), given by Magnus. **Rejected:** a strict Integer check to match
Python's `TypeError`. No `data-edge.json` vector (wrong-typed element). **Context:** iteration 277.

## 2026-11-21 — Long strings in the generated C edge header are buffers filled at runtime

**Decision:** `gen_data_edge_vectors.py` writes every string over 4,095 bytes (the ISO C99 minimum
literal length) as a zeroed `static char` buffer, and the header's `iscc_edge_init()` fills it with
`memcpy` from literal pieces; both harnesses call it once. **Why:** MSVC rejects a long literal, and
gcc `-Woverlength-strings` checks this limit locally. **Rejected:** a `static const char` array
initializer (at least 4 characters per byte: the 128 KB meta vector alone would put the header over
the 256 KB added-file hook limit); adjacent literal pieces (MSVC caps the joined literal at 65,535
bytes); limiting the test programs to gcc and clang. **Accepted cost:** the buffers are not `const`,
and a harness that forgets the call reads NUL bytes. C fails on that; C++ would not notice while the
only long row expects an error, so a pytest asserts the call in both sources. **Context:** 282.

## 2026-11-22 — napi dev dependencies install from a committed lockfile with `npm ci`

**Decision:** `crates/iscc-napi/package-lock.json` is tracked, CI and both `release.yml` napi jobs
run `npm ci`, `version_sync.py` owns the lock's two root `version` fields, and the release
dependency pass regenerates the lock. **Why:** an unlocked install fetched a transitive version two
minutes after its publication and got a 404 (red CI on `bd79145`), and the publish job installed an
unlocked tree while it held the OIDC token. **Rejected:** a `min-release-age` `.npmrc` (npm 10.9 on
Node 22 ignores the key). **Open:** the locked cli 3.10 cannot build in the 9p checkout (issues.md).

## 2026-11-22 — vcpkg SHA512s are written by `prepare-release` from the `build-ffi` artifacts, before the tag

**Decision:** `prepare-release` needs `build-ffi`, downloads its `ffi-*` artifacts and runs
`scripts/update_vcpkg_hashes.py`, then commits `portfile.cmake` with `Package.swift` before it tags.
`publish-ffi` and `pack-nuget` need `prepare-release`; `publish-ffi` sets `overwrite_files: false`.
**Why:** the portfile pinned the v0.2.0 hashes for five releases; vcpkg consumers pin tags, so the
tagged commit must carry the hashes of the archives it publishes. **Rejected:** a post-publish job
committing to `main` (the tag keeps stale hashes); dropping the port (amends the cpp spec, Titusz).
**Accepted cost:** a failing `build-ffi` leg blocks the tag and every publish job. **Context:** 285.

## 2026-11-22 — `publish-ffi` checks asset state only; an uploaded FFI archive is never compared or replaced

**Decision:** `publish-ffi` deletes the assets of its staged archives that are not in state
`uploaded`, uploads with `overwrite_files: false`, then fails unless every staged name is `uploaded`
(`scripts/ffi_release_assets.py`). It compares neither size nor digest. **Why:** an `ffi` re-trigger
rebuilds the archives in a new run, so its bytes can differ from those whose SHA512s the tagged
portfile pins; the asset on the release is the correct one. **Rejected:** a size or digest check
(fails every legitimate re-trigger); verifying without pruning (a rerun never recovers). **Not
proven:** that GitHub lists a live `starter` asset and lets it be deleted (a 502 cannot be forced).
If it is not listed, the upload gets a 422 or the verify step reports it missing: the job fails
either way. **Context:** 287.

## 2026-11-22 — `streaming::SumHasher` is Tier 2, and Tier 2 is under the SemVer contract

**Decision:** every public `iscc-lib` path has a tier. Tier 1 is the 35 crate-root symbols.
`IsccError`/`IsccResult` and the `types` result structs are crate-root support types of Tier 1 and
are not counted. Tier 2 is `codec` plus `streaming::SumHasher`. A module path to a re-exported item
(`types::X`, `streaming::DataHasher`, `codec::encode_seq`) is the same item, not a second one. Tier
2 has the same SemVer contract as Tier 1, not "may change in MINOR releases". **Why:** `SumHasher`
has been public since v0.5.0 and Python and WASM wrap it, and `cargo-semver-checks` checks the whole
public surface, so it is locked in fact; `rust-core.md` already put `codec` under the lock.
**Rejected:** Tier 1 (a crate-root re-export bound on every surface changes target.md's count of 35,
which is Titusz's call); hiding it (`pub(crate)` or `#[doc(hidden)]` breaks the two bindings or
removes a published type). target.md and the agent API-break rules name both since 2026-11-22.

## 2026-11-22 — A wrong-type or out-of-range vector value makes a conformance case unparsable

**Decision:** in the `data.json` selftest runners (`conformance.rs`, `conformance.go`), a value of
the wrong JSON type, a fraction or a number outside its argument's range makes the case unparsable
(Rust `None`, Go `errUnparsable`); nothing is coerced, truncated, skipped or defaulted. Strings
(`name`, `description`, `text`, stream, `bytes:`, code entries) reject `null`; `meta` is `null`, a
string or an object; `bits` fits `u32`, a pixel is 0..=255, `cv` and frame entries fit `i32`. Every
top-level key except `_metadata` maps to an object of case objects, or the selftest fails. **Why:**
the two runners must agree, and each JSON reader coerces differently (Go reads `null` as a zero
value and truncated floats; serde_json dropped non-string list entries). No real vector is affected.
**Rejected:** coercing to the nearest valid value (hides a corrupt fixture). **Not covered:** the
data-edge harnesses; `-0` as a pixel, `cv` or frame entry (serde_json reads it as a float and
rejects it, Go reads 0); Go's case-insensitive key match. Both left as they are: no vendored vector
can hold either, and `data.json` is gated byte-identical to iscc-core's. **Context:** 293.

## 2026-11-22 — encode_component rejects a digest shorter than bit_length/8

**Decision:** the Tier 1 `encode_component` raises when `len(digest) < bit_length / 8`, on every
surface (Rust `lib.rs`, so the ten Rust-backed bindings, and Go `EncodeComponent`, wording
`digest length N < bit_length/8 (M)`). iscc-core slices instead
(`encode_component(0, 0, 0, 64, b"\x01\x02\x03\x04")` returns `AAAQCAQDAQ`): Rust and Go diverge
from it here on purpose. The gen functions keep the reference's behaviour: they go through the
internal encode path (Rust `codec::encode_component`, Go `encodeComponentInternal`), which clamps
the body to the digest. **Why:** a header that claims more bits than its body holds is a malformed
unit, and eleven surfaces already raised; Go was the one that truncated. **Rejected:** slicing like
iscc-core (hands out malformed units); rejecting gen `bits` beyond the digest in the same change (a
separate behaviour, shared with iscc-core). **Pinned by:** the `encode_component` section of
`data-edge.json`, run on all 12 surfaces (296). **Context:** 295.

## 2026-11-22 — `alg_minhash_256` rejects an empty feature list

**Decision:** (Titusz) the Tier 1 `alg_minhash_256` reports an error for an empty `features` list on
every surface. The function gains the surface's error channel (Rust `IsccResult<Vec<u8>>`, Go
`([]byte, error)`). This breaking Tier 1 signature change is approved: the step flags
`**API-BREAK:**` and proceeds. **Why:** iscc-core raises `ValueError` (`min()` of an empty list) and
`alg_simhash` already rejects an empty list (2026-11-20). 32 bytes of `0xFF` look like a real digest
and make every empty input identical, and nobody had chosen that value. Titusz does not weigh a
breaking signature change as a cost before 1.0. **Alternatives:** keep `0xFF` and record the
divergence (no break, but a silent difference from the reference). **Context:** answer to the
iteration 296 escalation. Performance is no factor: one length check per call, and the three
internal callers always pass at least one feature.

## 2026-11-22 — The first `finalize()` consumes a hasher on every surface, whatever its outcome

**Decision:** (Titusz) the first `finalize()` call on a `DataHasher`, `InstanceHasher` or
`SumHasher` consumes it, also when the call fails, whichever check rejects the argument. A later
call reports the finalized state. `@iscc/wasm` takes the inner hasher before it validates `bits`,
`wide` and `add_units`. **Why:** Rust (`finalize(self)`), the Python binding and Go (2026-11-21)
already behave this way; in WASM the outcome depended on whether the JS-side check or the core
caught the value. **Alternatives:** validate every argument before consuming (allows a retry, but
differs from the other surfaces). **Context:** answer to the iteration 296 escalation. Not measured
then: the napi, JNI, .NET, Ruby, UniFFI and C FFI hashers.

## 2026-11-22 — A unit is at most 256 bits: a larger `bits` or `bit_length` is rejected

**Decision:** (Titusz) iscc-lib rejects `bits` above 256 on every surface. The bound covers every
gen function, every hasher `finalize` and the public `encode_component`; decoding a unit header
longer than 256 bits fails, as it does on iscc-core. **Why:** ISO 24138 units are 32 to 256 bits.
iscc-core 1.5.0 accepts `bits=288` in six gen functions and returns a code that its own
`iscc_decode` rejects (`288 is not a valid LN`), reported as iscc/iscc-core#142. Rejecting is a
deliberate divergence on the encode side and follows the reference on the decode side.
**Alternatives:** keep the reference's output and state the range on the API pages. **Context:**
interactive session after iteration 296. Titusz ruled on `bits`; including `encode_component` and
the decode side is the session's reading (`encode_component(0, 0, 0, 512, <64 bytes>)` decodes on
iscc-lib and fails on iscc-core). The internal clamp of 2026-11-22 stays.

## 2026-11-22 — Hasher `finalize()` arguments are converted after the state is taken

**Decision:** on the five bindings whose framework converts a typed parameter before the method body
runs, `finalize` takes its arguments raw and converts them after `take()`: `JsValue` (WASM),
`Option<Unknown>` (napi), `jint` then `non_negative` (JNI), a private `Deferred` enum (Python),
`Value` then `u32::try_convert` (Ruby). Exception classes and messages stay. One message changes:
napi reports a non-number as `Invalid bits: expected a number` (the WASM wording). A call the
language rejects before the method runs (unknown keyword, wrong arity) is no `finalize()` call and
leaves the hasher usable; that is the review's reading of the ruling of 2026-11-22. **Why:** the
ruling puts the consume step first, and a typed parameter cannot be ordered after it.
**Alternatives:** Python `Option<u32>` with `bits=None` (turns `finalize(None)` from `TypeError`
into a 64-bit code); napi `f64` checked after `take()` only (a string is still rejected before the
body). **Context:** iteration 297 review.

## 2026-11-22 — A core signature change lands with the binding edits the compiler forces

**Decision:** iteration 298 passed with code changes in 10 non-test files (3 core, 7 binding
`lib.rs`) and neither escape valve declared. A Tier 1 signature change in `iscc-lib` and the edit it
forces in each binding crate are one step, on four conditions the review checks: a decisions.md
entry approves the break, each binding maps the result exactly as a sibling wrapper already does
(here `alg_simhash`), no binding changes beyond that call, and there is one commit per surface. Go
and shared vectors stay separate steps. **Why:** the workspace does not compile in between, so no
smaller unit passes clippy or the pre-push gates. The cap and "a core change never rides along with
a fan-out" (2026-11-19) keep open design questions out of mechanical steps; here Titusz had settled
the design and the binding edits carry no choice. **Alternatives:** NEEDS_WORK (nothing to split or
repair); a second function beside the old one (keeps the `0xFF` behaviour the ruling removes).
**Context:** iteration 298 review. define-next names this ground in Scope when it applies.

## 2026-11-22 — Conan SHA256s live in `conandata.yml`, keyed by target triple, written by the vcpkg hash script

**Decision:** `packages/cpp/conandata.yml` holds one SHA256 per target triple under `sha256:`, with
no version key. `scripts/update_vcpkg_hashes.py` renders the whole file in the pass that writes the
vcpkg SHA512s and keeps its name. The recipe reads the hash in `build()` and raises when a target
has none, so `None` never reaches `download()`. **Why:** one archive pass means a missing archive
writes neither file and the tagged commit carries both hash sets; without a version key
`version_sync.py` needs no Conan data rule, and the file is stale between a version bump and the
release exactly as the portfile is. **Alternatives:** a dict in `conanfile.py` (generated values
inside hand-written code); Conan's `sources: {version: …}` layout (a second version to sync, old
entries to prune); a second script or a neutral script name (duplicates the archive checks, or
leaves the decision of 2026-11-22 naming a missing file). **Context:** iteration 305 review.
