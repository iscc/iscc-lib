# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files. A `low`
entry with `**Triage:** untriaged` is a review finding the human has not weighed yet: promote it to
`normal`, or delete the line to park it. The human moves an entry that does not matter for the
current release to `postponed.md`, which the loop never acts on.

**This is a defect tracker, not a work queue.** An entry states the remaining problem and how to
tell it is fixed, in **40 lines or fewer**, and is deleted once nothing is left to fix. Never record
progress inside an entry — no per-iteration checklists, no "✅ slice N done", no plan of remaining
sub-steps. Git history records what was done; a step queue here removes the gap analysis that
define-next exists to perform. If only part of a problem remains, rewrite the entry to describe that
smaller problem. Design rationale belongs in `decisions.md`, operational gotchas in `learnings.md`,
and user-facing behaviour in `docs/`.

<!-- Add issues below this line -->

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

## `docs/development.md` tells maintainers to release by pushing a tag `normal` [review]

`docs/development.md` → "Publish" says to run `git tag vX.Y.Z` and `git push origin vX.Y.Z` on
`main`, and that "the tag push triggers `.github/workflows/release.yml`". The workflow has
`workflow_dispatch` as its only trigger, so the push starts nothing. It also does harm:
`prepare-release` does not move a tag that already exists (its checkout fetches every tag, `git tag`
fails and the step continues), so a later dispatch leaves the hand-pushed tag on the commit before
the `Package.swift` checksum commit. The registry table under the same heading lists five of the
packages the workflow publishes, and the checkbox sentence four of the eight checkboxes.
Pre-existing, found in iteration 259 while the same claim was removed from the specs.

In `.claude/context/specs/swift-bindings.md`, the heading "Release Process — Force-Update Tag
Pattern" and its intro paragraph still name Ferrostar's force-update approach. The Release Flow
below it creates the tag once, after the checksum commit, and moves none. Realign without escalation
(decision of 2026-11-20).

Resolved when the "Publish" section describes the dispatch with a `version` input, names every
registry chain or says the table is an excerpt, tells the reader not to push the tag by hand, and
the Swift heading and intro match the flow.

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

## Docs overstate language coverage and what ISO 24138 specifies `normal` [audit]

Verified by the iteration-249 audit, re-checked against the tree 2026-11-21. These pages are what
users and LLM tools read first.

- `docs/llms.txt:5` names 6 languages (Python, Node.js, WASM, Go, Java, C FFI) and line 9 three
    install commands; the project ships 11 surfaces (also Ruby, .NET, C++, Swift, Kotlin)
- `docs/index.md:17,26` and `docs/benchmarks.md:15` say all 10 `gen_*_v0` functions are specified
    (or defined) by ISO 24138; `gen_sum_code_v0` is an iscc-lib addition, not part of the standard

**Suggested fix:** list every surface and its install command in `llms.txt` (or point to the install
table); say 9 functions are standard and `gen_sum_code_v0` is a convenience on top. **Scope
estimate:** 3 non-test files.

Resolved when the three pages state the coverage and the standard's scope correctly.

## No shared vector pins `alg_cdc_chunks` for `avg_chunk_size` from 2^29 up `normal` [audit]

`data-edge.json` has an `alg_cdc_chunks` section on all 12 surfaces, with averages up to 2^29 - 1.
The range from 2^29 up, where the core panicked before iteration 263, has no shared vector, because
iscc-core 1.5.0 disagrees with itself there. Its compiled wheel (cp310 to cp314; `cdc.pxd` types the
maximum chunk size, `avg * 8`, as `uint32_t`) raises
`OverflowError: value too large to convert to uint32_t`. Its pure-Python build returns one chunk for
a short input, as iscc-lib does. Python 3.15 has no wheel and installs the pure build. Measured in
iteration 264 for 2^29, 2147483647 and 4294967295 with
`uv run --no-project --python 3.10 --with iscc-core==1.5.0` (loads `cdc.cpython-310-*.so`) against
`--python 3.15` (loads `cdc.py`). CI runs pytest on both, so the reference check in
`tests/test_data_edge.py` fails on one leg for such a vector, with or without `diverges`.

**Blocked on a decision (Titusz):** which iscc-core build is the reference where the two disagree.
The decision of 2026-11-21 accepts every value, which matches the pure build. Options: (a) report
upstream and leave the large vectors out until the wheel is fixed; (b) add them now and have the
Python harness skip the reference check where the compiled build is loaded; (c) reject from 2^29 up,
like the wheel.

Facts for the vector. An input that fits in JSON is shorter than `min_size` (`avg / 4`), so the
expected output is one chunk equal to the input: the vector pins acceptance, and the parameters stay
pinned by the Rust and Go unit tests. Java `algCdcChunks` takes a signed `int`, so 2147483647 is the
largest value every surface takes.

**Upstream:** iscc/iscc-core. The compiled and the pure-Python build of `alg_cdc_chunks` differ for
`avg_chunk_size >= 2^29`.

Resolved when a decisions.md entry names the reference build, and either the section carries vectors
for 2^29 and 2147483647 that every harness runs, or that entry keeps them out.

## No CI job executes the 32-bit code paths of the core or the Go port `low` [review]

**Triage:** untriaged — a user on a 32-bit target (Go `386`/`arm`, Rust `wasm32`) would hit a later
edit that breaks the integer-width handling, with CI green.

`algCdcParams` subtracts in `uint32` and clamps to `math.MaxInt` because `int(avgSize)` is negative
on a 32-bit `int` (iteration 263). CI's Go job runs amd64 only, where the old signed form is also
correct, so reverting it fails no CI test. The Rust `max_size` saturation for a 32-bit `usize` is
compiled by the `wasm32` check and executed nowhere. Measured locally:
`GOARCH=386 CGO_ENABLED=0 go test ./...` runs on an amd64 host and passes.

Resolved when CI runs the Go suite once with `GOARCH=386`, or a decisions.md entry leaves 32-bit
targets untested on purpose.

## One-shot gen functions re-implement their streaming counterparts `normal` [audit]

Verified by the iteration-249 and iteration-260 audits (3040298).

- `gen_instance_code_v0` (`crates/iscc-lib/src/lib.rs:907-908`) calls `blake3::hash(data)` and then
    `utils::multi_hash_blake3(data)`, which hashes the same bytes again (`utils.rs:164`): two full
    BLAKE3 passes per call. The multihash format is also written in `streaming.rs:39-56`.
- `gen_sum_code_v0` in `crates/iscc-wasm/src/lib.rs:258-285` copies `SumHasher::update`/`finalize`
    (`streaming.rs:172-203`), comment included; the same file already wraps `SumHasher`.
- `gen_meta_code_v0` parses `meta` twice (`lib.rs:372` via `parse_meta_json`, then `:406-408`), and
    `json_to_data_url` (`:322-328`) repeats the parse+canonicalize body with another error string.

**Why it matters:** each pair can drift apart, and the instance hot path does twice the hashing.
**Suggested fix:** derive `datahash` from the single digest (one shared multihash helper), call
`SumHasher` from the WASM one-shot function, and have `parse_meta_json` return what the data URL
step needs. **Scope estimate:** 3 non-test files; mind the iai-callgrind baseline.

Resolved when each computation exists once and the conformance and perf gates stay green.

## Python video frame values raise a platform-dependent exception class `normal` [audit]

Verified by the iteration-260 audit at 3040298. `extract_frame_sigs`
(`crates/iscc-py/src/lib.rs:86-95`) reads each value with `PyLong_AsLong` (32-bit `long` on Windows,
64-bit elsewhere) and maps a failed `i32::try_from` to `ValueError`. So
`gen_video_code_v0([[2**31, ...]])` and `soft_hash_video_v0` raise `ValueError` on Linux/macOS and
`OverflowError` on Windows, while `gen_audio_code_v0` raises `OverflowError` everywhere, the class
the 2026-11-21 decision keeps for Python integer conversion failures. `tests/test_int_range.py` has
no video case.

**Suggested fix:** read with `PyLong_AsLongLong` and raise `OverflowError` outside the `i32` range;
add video cases to `tests/test_int_range.py`. **Scope estimate:** 1 non-test file.

Resolved when an out-of-range frame value raises `OverflowError` on every platform, pinned by a
test.

## Specs and crate CLAUDE.md files describe shipped work as pending and name missing items `normal` [audit]

Verified by the iteration-260 audit at 3040298. CID-fixable (decision of 2026-11-20: realign).

- `specs/rust-core.md:360-371`: `Version` "has only V0" and every surface "fails on any ISCC-IDv1";
    `codec.rs:105-131` defines `V1` and admits `(Id, V1)`.
- `specs/python-bindings.md:301-304`: text and video paths "still hold the GIL";
    `iscc-py/src/lib.rs` wraps them in `.detach(` (:138, :190, :217, :241).
- `specs/go-bindings.md`: the IDv1 rename "exists only on `develop`" (it shipped in 0.6.0);
    "Streaming functions also accept `io.Reader`" and the two `io.Reader` bullets (no such API in
    `packages/go`).
- `crates/iscc-napi/CLAUDE.md:36,66`, `crates/iscc-py/CLAUDE.md:66`, `crates/iscc-wasm/CLAUDE.md:43`
    name `iscc_lib::Error`; the type is `IsccError`.
- `packages/go/CLAUDE.md`: :91-117 keeps a test-file list (missing three files) that the 2026-11-21
    decision removed; :142 says most packages wrap `iscc-ffi` (only C++ and .NET do); :129-131 omits
    the `packages/go/vX.Y.Z` tag the module proxy needs (`release.yml:115-116`).

**Why it matters:** define-next and advance read these as truth, so stale claims become scoped work.
**Suggested fix:** rewrite each statement in the present tense to match the code. **Scope
estimate:** 7 files, needs audit-scope step (≤8 files).

Resolved when each cited statement matches the tree.
