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

## `iscc_decode` reads an 88-bit ISCC-ID that iscc-core rejects `low` [review]

**Triage:** untriaged. Only a caller who decodes a hand-built ISCC-ID with length index 3 would see
it; no generator on any surface emits an 88-bit ID.

Measured in iteration 301 on the Python binding against iscc-core 1.5.0 (CPython 3.13), with an ID
header of length index 3 over an 11-byte body, for subtype 0 version 0 and subtype 1 version 1:
`iscc_lib.iscc_decode` returns the tuple, `iscc_core.iscc_decode` raises "88 is not a valid LN".
Length indexes 0, 1, 2 and 4 (64, 72, 80 and 96 bits) decode on both, and 5 and above fail on both.
Cause: `codec::decode_length` returns `length * 8 + 64` for an ID and `codec::encode_length` accepts
64 to 96 in steps of 8, while iscc-core's `LN` enum has no 88. Pre-existing, and outside the
decision of 2026-11-22, which covers Meta through Flake. Not measured: the Go port, and
`encode_component(6, …, 88, …)` on either side.

Resolved when a `data-edge.json` vector pins the outcome for an 88-bit ID: rejected as in iscc-core,
or accepted and marked `diverges` with a decisions.md entry that says why.

## `gen_mixed_code_v0` with `bits` below 8 panics in the Go port and in debug builds of the core `low` [review]

**Triage:** untriaged. A Go caller who passes a `bits` value from 0 to 7 to `GenMixedCodeV0` gets a
runtime panic instead of an error, and a Rust user gets one in a dev-profile build; the published
release builds of the Rust-backed bindings return an error.

Measured in iteration 303 with two 256-bit Text-Codes and `bits` 0 and 4. Go: panic "index out of
range [0] with length 0" (`softHashCodesV0` in `code_content_mixed.go` writes `entry[0]` into a
slice of `bits / 8` bytes). Rust: `soft_hash_codes_v0` in `lib.rs` computes `nbytes - 1` with
`nbytes = 0`, which panics with "attempt to subtract with overflow" under overflow checks (the debug
Python extension raises `PanicException`); the release wheel 0.7.0 returns `ValueError`. iscc-core
1.5.0 raises `ValueError`. With `bits = 8` every one of them returns an error. Pre-existing in both.
The other `gen_*_v0` functions and both hashers return an error for 0 and 4.

Resolved when both return an error for `bits` below 8, pinned by a `data-edge.json` row in the
existing `gen_mixed_code_v0` section (`error`, and no `diverges`, since iscc-core raises too).

## `encode_component` docs on five surfaces give the `bit_length` range without the ISCC-ID case `low` [review]

**Triage:** untriaged. A caller who builds an ISCC-ID unit from the C FFI, Java, Python, Ruby or Go
docs would read that 72 or 80 is invalid and that 32 or 128 is valid; the call itself returns the
right result or a clear error.

The `bit_length` parameter is described as "multiple of 32, 32 to 256" in `docs/c-ffi-api.md` and
the `iscc_encode_component` `///` in `crates/iscc-ffi/src/lib.rs`, in `docs/java-api.md` and the
`IsccLib.java` Javadoc, in `_lowlevel.pyi`, in `docs/ruby-api.md` and in the comment in
`packages/go/codec.go`. For MainType ID the core takes 64 to 96 in steps of 8, which only
`docs/rust-api.md` says. Pre-existing, reported by advance in iteration 307
(`git grep -n '32 to 256'`).

Resolved when each of these descriptions names the ID range or limits its range to Meta through
Flake, with the two generated files (`iscc.h`, `NativeMethods.g.cs`) regenerated.

## `specs/documentation.md` says every code example is tabbed, and the how-to guides have no tabs `low` [review]

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
