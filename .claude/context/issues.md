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

## Ruby 4.0 gets no precompiled gem `low` [review]

**Triage:** untriaged. Every user on Ruby 4.0, the current stable series, installs the source gem
and needs a Rust toolchain and `libclang-dev`; the docs say so.

The cross-gem image is pinned to rb-sys 0.9.123, whose newest cross Ruby is 3.4.5. Ruby 4.0 needs
rb_sys 0.9.124 or newer, and the hold comment in `crates/iscc-rb/Gemfile` says that version breaks
`RbSys::ExtensionTask`. Unverified since the hold was written.

Resolved when the cross-gem list includes 4.0 and the docs name the range, or a decisions.md entry
keeps 4.0 on the source gem.

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

## The devcontainer and CI count different instructions on three iai rows `low` [human]

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
