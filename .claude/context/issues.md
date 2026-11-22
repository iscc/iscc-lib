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

## Published Ruby source gems up to 0.7.0 do not build `low` [review]

The source gem on rubygems.org was built with a bare `gem build` up to 0.7.0: it declares no
`rb_sys` dependency and ships a workspace-inheriting `Cargo.toml` without the core (measured for
0.7.0 in iteration 238). The repository now builds a standalone source gem (`ruby-source-gem` CI
job), but published versions cannot be changed. Yanking the broken `ruby`-platform gems is a
registry action for Titusz. CID must not act on this.

Resolved when a release with the repaired source gem is on rubygems.org and it installs from the
registry into a clean gem home on a Ruby series without a precompiled gem
(`GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install iscc-lib`, then `IsccLib.conformance_selftest`).

## `release.yml` loads only one precompiled Ruby binary before publishing `low` [review]

**Triage:** untriaged. A Ruby 3.4 user (or anyone not on x86_64 Linux) would get a precompiled gem
that fails at `require` if a cross-build ever produces a broken binary for that series or platform.

`build-gem` builds Ruby 3.3 and 3.4 binaries for five platforms. `test-gem` installs the
`x86_64-linux` gem on Ruby 3.3 and runs `conformance_selftest`, so the 3.4 binary and the other
platforms are published without being loaded once. The 3.4 cross-build is new in iteration 270 and
no run has exercised it (`release.yml` is `workflow_dispatch`-only). Pre-existing in kind: the 3.1
and 3.2 binaries were published the same way.

Resolved when `test-gem` loads the gem on every series in the cross-gem `ruby-versions` list (a
matrix whose list a test ties to that input), or a decisions.md entry keeps the single smoke test.

## Ruby 4.0 gets no precompiled gem `low` [review]

**Triage:** untriaged. Every user on Ruby 4.0, the current stable series, installs the source gem
and needs a Rust toolchain and `libclang-dev`; the docs say so.

The cross-gem image is pinned to rb-sys 0.9.123, whose newest cross Ruby is 3.4.5. Ruby 4.0 needs
rb_sys 0.9.124 or newer, and the hold comment in `crates/iscc-rb/Gemfile` says that version breaks
`RbSys::ExtensionTask`. Unverified since the hold was written.

Resolved when the cross-gem list includes 4.0 and the docs name the range, or a decisions.md entry
keeps 4.0 on the source gem.

## `@iscc/wasm` converts byte inputs and traps on a non-string `low` [review]

**Triage:** untriaged. A plain-JavaScript caller who passes a string or a plain array where a
`Uint8Array` is declared gets a different valid code and no error; the `.d.ts` stops TypeScript
callers.

Measured on a `--target nodejs` release build in iteration 268. `gen_data_code_v0("abc")` returns
the code of three zero bytes, and `gen_data_code_v0([1, "2", null, 300])` the code of
`[1, 2, 0, 44]`. A number where a string is declared (`gen_text_code_v0(5)`,
`gen_meta_code_v0("a", 7)`) throws `RuntimeError: memory access out of bounds` instead of an error
that names the parameter. Pre-existing: the glue converts every `&[u8]` and `&str` parameter before
Rust runs. No `data-edge.json` vector is owed (`edge-vectors.md` → Rules: wrong argument types).

Resolved when either byte and string parameters reject a wrong type with an error that names the
parameter, or `docs/howto/wasm.md` → "Error handling" states the conversion.

## `@iscc/wasm` list inputs read through serde let a throwing element kill the instance `low` [review]

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

## JNI `NativeLoader` leaves an empty temp directory behind on every JVM start `low` [review]

**Triage:** untriaged. Every application that loads the bundled library leaves one empty
`iscc-jni-<random>` directory in `java.io.tmpdir` per JVM start; a command-line tool or a service
that restarts often piles them up.

`loadFromResource` registers the extracted file for `deleteOnExit` first and its directory second.
The JVM deletes in reverse order of registration, so it tries the directory while the file is still
inside, fails silently, and then deletes the file. Measured in iteration 278: one `mvn test` run
with a bundled library (two JVMs load it) raised the number of empty `/tmp/iscc-jni-*` directories
from 12 to 14. Pre-existing. Not measured: Windows, where a loaded DLL cannot be deleted at all.

Resolved when a JVM that loaded the bundled library leaves no `iscc-jni-*` directory behind on
Linux, pinned by a child-JVM test, or a decisions.md entry accepts the leak.

## No CI job runs a test suite on Windows `low` [review]

**Triage:** untriaged. A Windows user of any binding would hit a defect that depends on the platform
(C `long` width, path handling, line endings), because no run before publishing can see it.

`ci.yml` has 21 jobs on `ubuntu-latest` and one on `macos-15` (the Swift job). `release.yml` builds
Windows artifacts (Python wheel, napi, JNI, C FFI, Kotlin native among them), but only `test-nuget`
runs on `windows-latest`, and it calls `conformance_selftest` alone; `test-wheels` runs the Python
wheel on Linux only. The video exception class fixed in iteration 265 differed on Windows only and
was found by reading the code. Pre-existing, measured 2026-11-21.

Resolved when either CI runs at least the Python suite on `windows-latest`, or a decisions.md entry
keeps CI on Linux and macOS on purpose.

## `alg_minhash_256([])` returns 32 bytes of `0xFF` instead of an error `normal` [human]

Decided on 2026-11-22 (decisions.md, "`alg_minhash_256` rejects an empty feature list"): an empty
`features` list is an error on every surface, and the breaking signature change is approved. Found
by the iteration-270 audit. Today the core's `minhash` falls back to `MAXH`
(`crates/iscc-lib/src/minhash.rs`), so `alg_minhash_256` returns all `0xFF` where iscc-core raises
`ValueError` (`reference/iscc-core/iscc_core/minhash.py:14-16`). Five hand-written tests pin the old
value (`tests/test_algorithm_primitives.rs:146`, `iscc-ffi/src/tests.rs:784`,
`IsccLibTest.java:932`, `packages/cpp/tests/test_iscc.cpp:688`, `packages/go/minhash_test.go:79`),
and `data-edge.json` has no `alg_minhash_256` section. The Rust signature is `fn(&[u32]) -> Vec<u8>`
and Go's is `AlgMinhash256([]uint32) []byte`: neither can report an error yet. The three internal
callers (`gen_data_code_v0`, `DataHasher::finalize`, `soft_hash_text_v0`) always pass at least one
feature; their output and their iai rows must not move.

**Spec:** `.claude/context/specs/rust-core.md` and `.claude/context/specs/go-bindings.md` (the
signature rows of `alg_minhash_256`)

Resolved when every surface reports an error for an empty list, one `alg_minhash_256` section in
`data-edge.json` pins the error row and a non-empty row on all 12 surfaces, and the five tests of
the `0xFF` value are gone.

## `crates/iscc-py/CLAUDE.md` still lists test files one by one `low` [review]

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

## Version-sync file lists in `docs/development.md` and the release skill are partial `low` [review]

**Triage:** untriaged. A maintainer who stages a release commit from the release skill's list of
"expected candidates" would leave synced files out (the napi lockfile since iteration 284), and the
`version-check` CI job would fail on that commit; users are unaffected.

`docs/development.md` ("`version:sync` updates these files automatically") lists 9 files, and
`.claude/skills/release/SKILL.md` → Step 4.1 lists 15 paths, 13 of them sync targets. `TARGETS` in
`scripts/version_sync.py` has 22 entries. Both lists lack `crates/iscc-napi/package-lock.json`, the
Swift and Kotlin targets, `docs/index.md` and the Swift and Kotlin how-to pages and READMEs; the
docs table also lacks the Ruby, .NET, vcpkg and Conan manifests. Pre-existing as a pattern, and
iteration 284 added one more missing row. The skill also tells the reader to stage what
`git diff --name-only` shows, which is why nothing has gone wrong so far.

Resolved when neither page carries a file list that can fall behind `TARGETS` (point to the list in
`scripts/version_sync.py`, as the specs do), or a test ties each list to `TARGETS`.

## vcpkg port fails one post-build check `low` [review]

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

## iai baseline for `bench_instance_code` is twice the measured cost `low` [review]

**Triage:** untriaged. No user is affected; a later change that brings the second BLAKE3 pass back
into `gen_instance_code_v0` would pass the perf gate unnoticed.

Iteration 289 halved the instruction count of `gen_instance_code_v0` (`bytes_1m` 5,236,470 →
2,621,027 Ir, `bytes_64k` 333,349 → 169,266). `.iai-baseline.json` still holds the old values, and
`scripts/iai_regression.py --check` fails only above +10% of the baseline. On these two rows the
gate now accepts any cost up to twice the current one, and no test can see a double hash. The step
left the baseline alone on purpose: every refresh so far was a `cid(human)` commit, and state.md
calls a refresh a human call (nine other rows read 3–8% high from toolchain drift).

Resolved when `.iai-baseline.json` is refreshed from the `iai-baseline` artifact of a CI `perf` run
on a commit at or after 1eec5edb (`python3 scripts/iai_regression.py --update --from-dir <dir>`), so
that both `bench_instance_code` rows are within the tolerance of a current run.

## Gen functions accept `bits` above 256 and return a unit that no decoder reads `normal` [human]

Decided on 2026-11-22 (decisions.md, "A unit is at most 256 bits"): iscc-lib rejects it. Measured on
2026-11-22 with `bits=288` on the Python binding: `gen_meta_code_v0`, `gen_text_code_v0`,
`gen_audio_code_v0`, `gen_data_code_v0` and `gen_instance_code_v0` return a code whose header says
288 bits over a 32-byte body, and `iscc_decode` rejects it ("truncated ISCC body: expected 36 bytes,
got 32"). The image, video and mixed functions already reject it. Go returned the same code as the
Python binding for `gen_data_code_v0(b"abc", 512)` in iteration 295. The public
`encode_component(0, 0, 0, 512, <64 bytes>)` returns a unit that iscc-lib decodes and iscc-core
rejects ("512 is not a valid LN"). Cause: `codec::encode_length` accepts any multiple of 32,
`codec::decode_length` has no upper bound, and the internal encode path clamps the body to the
digest (kept on purpose, decisions.md 2026-11-22). Most API pages say only "multiple of 32". Not
measured: the hashers' `finalize(bits)` on any surface.

**Upstream:** iscc/iscc-core, filed as https://github.com/iscc/iscc-core/issues/142 (iscc-core 1.5.0
returns the same undecodable codes for meta, text, audio, video, data and instance)

Resolved when every gen function, every hasher `finalize` and `encode_component` reject a unit
length above 256 on all 12 surfaces, a unit header longer than 256 bits fails to decode, a
`data-edge.json` vector pins the rejection (marked `diverges`, naming the decision), and the API
pages state the range of 32 to 256.

## No test pins "a failed first `finalize` consumes" on C FFI, C++, .NET, Swift and Kotlin `low` [review]

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
