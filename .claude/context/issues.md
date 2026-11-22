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

## `alg_minhash_256([])` returns 32 bytes of `0xFF` where iscc-core raises, with no decision `normal` [audit]

Verified by the iteration-270 audit at 34303a6. The reference computes `min([...])` over the
features (`reference/iscc-core/iscc_core/minhash.py:14-16`), so an empty list raises `ValueError`.
The core's `minhash` falls back to `MAXH` (`crates/iscc-lib/src/minhash.rs:156-164`), so
`alg_minhash_256` returns all `0xFF`. Five hand-written tests pin the divergence
(`tests/test_algorithm_primitives.rs:146`, `iscc-ffi/src/tests.rs:784`, `IsccLibTest.java:932`,
`packages/cpp/tests/test_iscc.cpp:688`, `packages/go/minhash_test.go:79`); `data-edge.json` has no
`alg_minhash_256` section, and no decision or edge-vector rule mentions it. The closest decision
(2026-11-20) made `alg_simhash` reject an empty list, the opposite direction.

**Why it matters:** a Tier 1 function silently differs from the normative reference on one input,
and the per-surface tests freeze it without anyone having chosen it. **Suggested fix:** decide:
reject empty input in the core (as `alg_simhash` does) or keep `0xFF` with a decisions.md entry;
either way pin it with one `data-edge.json` vector instead of per-surface tests. **Scope estimate:**
1-2 non-test files plus the vector fan-out.

Resolved when the empty-input behavior is a recorded decision pinned by a shared vector.

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

## target.md and the Swift spec criterion say the release workflow force-updates the tag `low` [review]

**Triage:** untriaged. An agent that reads the Swift section of `target.md` would expect a release
to move the tag and could plan work around a mechanism that does not exist; users are unaffected.

`target.md` → Swift ("force-update tag for checksum", and the criterion "Release workflow
force-updates the tag after updating checksum in `Package.swift`") and the same criterion under
"Verification Criteria" in `specs/swift-bindings.md` describe Ferrostar's pattern. `release.yml`
commits the checksum first and creates the tag once; an existing tag is never moved (learnings →
"prepare-release", and the spec's own Release Flow since iteration 277). The loop may realign the
sub-spec line (decision of 2026-11-20), but `target.md` is Titusz's to edit.

**Spec:** `.claude/context/target.md` → Swift bindings; `.claude/context/specs/swift-bindings.md` →
Verification Criteria

Resolved when both criteria say the workflow tags the checksum commit once and moves no tag.

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

## The Rust and Go conformance runners disagree on malformed vectors `normal` [audit]

Filed by the iteration-280 audit. Each side now parses the fixture in one place
(`crates/iscc-lib/src/conformance.rs`, `packages/go/conformance.go`), and both reject a bare-hex
stream, a missing `iscc` output and a short inputs list. What is left is a value of the wrong JSON
type or range. Measured in iteration 290 at 90e09661, one case per call, each with the correct
expected output (iteration 291 moved the Rust reads into parsers without changing them):

| Malformed input                 | Rust runner              | Go runner          |
| ------------------------------- | ------------------------ | ------------------ |
| `bits` as `64.0` or `64.5`      | parse failure            | passes (truncated) |
| description `null`              | parse failure            | passes             |
| `meta` a number or an array     | passes (read as no meta) | mismatch (as text) |
| non-string entry in a code list | passes (entry skipped)   | parse failure      |
| pixel `0.5` or `-256`           | parse failure            | passes (truncated) |

Go's `parseConformanceData` also returns an error for a top-level key that is not an object of
cases; Rust ignores unknown keys. A `bits` or pixel above the target width is cast (`as u32`,
`as u8`, `as i32`) on the Rust side without a range check.

**Why it matters:** each "mirror" selftest still passes vectors the other one rejects, and the Rust
in-crate tests inherit the loose readings (a non-string code-list entry is skipped silently).

**Suggested fix:** settle one rule for a value of the wrong JSON type or range (a parse failure is
the simplest), record it, and pin every row on both sides.

**Scope estimate:** 2 non-test files (the two parser files).

Resolved when both runners give the same verdict, without a panic, on every row above, each pinned
by a test.

## `@iscc/wasm` hasher `finalize()` consumes the hasher on some argument errors but not others `normal` [audit]

Verified by the iteration-249/260/270 audits; unchanged at db1fe55. `DataHasher.finalize`,
`InstanceHasher.finalize` and `SumHasher.finalize` (the three `finalize` methods in
`crates/iscc-wasm/src/lib.rs`) run `opt_u32`/`opt_bool` *before* `inner.take()`. So a `bits` that
the JS-side check rejects (`-1`, `1.5`, `"x"`) throws and leaves the hasher usable. A `bits` that
the core rejects (e.g. `13`) throws *after* `take()` and consumes it. `docs/howto/wasm.md:338` says
"After calling `finalize()`, the hasher is consumed". Go decided that the first `Finalize` consumes
even when it fails (decisions.md, 2026-11-21). No other surface states its rule, and no test pins
either branch.

**Why it matters:** whether a caller can retry after a bad argument depends on which validator
caught it. That is an implementation accident, and copying it to other surfaces makes it the API.

**Suggested fix:** decide the cross-binding rule (follow Go: the first `finalize` consumes, whatever
its outcome, or validate everything before consuming) and record it. Apply it to the three WASM
hashers, pin both error kinds with a test, and state the rule in `wasm.md`.

**Scope estimate:** 2 non-test files.

Resolved when a rejected `bits` leaves every WASM hasher in the same state, whichever check rejects
it, as decided and pinned by a test.

## target.md and the agent prompts name `codec` as the whole of Tier 2 `low` [review]

**Triage:** untriaged. An agent that reads `target.md` or the API-break rule in its own prompt would
treat `streaming::SumHasher` as outside every tier and outside the SemVer lock; users are
unaffected, the published docs are correct.

Since iteration 288 the docs, `specs/rust-core.md` and decisions.md (2026-11-22) classify
`iscc_lib::streaming::SumHasher` as Tier 2 beside `codec`. `target.md` ("**Tier 2 API** — `codec`
module (Rust-only, not bound)") still names `codec` alone, and its item list also lacks
`decode_units` and the enum-typed `codec::encode_component`. The API-break rules in
`.claude/agents/advance.md` and `.claude/agents/review.md` say "Tier 1 or Tier 2 `codec` symbol".
All three files are Titusz's to edit. He may also overrule the classification (for example make
`SumHasher` Tier 1, which changes the count of 35).

**Spec:** `.claude/context/target.md` → Tier 2 API

Resolved when `target.md` and both agent rules name `streaming::SumHasher` as Tier 2, or a new
decisions.md entry classifies it differently.

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

## `version:check` passes a stale second doc snippet and skips a vanished target silently `normal` [audit]

Verified by the iteration-290 audit at b3869b91. `_get_doc_dep_version`
(`scripts/version_sync.py:250-263`) returns the first Maven, else Gradle, else SPM match, so in
`README.md` (Maven :103, SPM :139, Gradle :150) and `docs/index.md` (4 versioned snippets) only one
snippet is checked; `sync_mode` skips the file when that one is current and never repairs the rest.
`check_mode` and `sync_mode` (:315-350) `continue` without a message when a target file is missing
or its getter finds nothing, so a renamed file or reworded line drops out of the gate. Tests cover
only the npm lockfile and the real tree.

**Why it matters:** `version-check` is the only CI guard over 22 version references; it stays green
while a release ships docs with mixed versions. **Suggested fix:** the doc getter returns every
match and reports a mismatch if any differs; sync rewrites when any differs; "file missing" and
"pattern not found" become `MISMATCH` in check mode. Pin each with a test on a synthetic doc.
**Scope estimate:** 1 non-test file.

Resolved when `version_sync.py --check` fails on a doc with one current and one stale snippet and on
a target whose file or pattern is gone, each pinned by a test.

## Go `EncodeComponent` truncates a short digest where every Rust-backed surface raises `normal` [audit]

Verified by the iteration-290 audit at b3869b91. `packages/go/codec.go:508-515` clamps the body to
`len(digest)`, so `EncodeComponent(0, 0, 0, 64, <4 bytes>)` returns a unit whose header says 64 bits
over a 4-byte body. The core rejects it (`crates/iscc-lib/src/lib.rs:219-226`, "digest length \<
bit_length/8", pinned at :2155) and every binding passes that error through. iscc-core slices
(`codec.py:43-45`), so Go matches the reference and Rust diverges; no decision records either, and
`data-edge.json` has no `encode_component` section.

**Why it matters:** the one pure-Go surface silently emits malformed codes for an input the other
eleven reject. **Suggested fix:** record the rule in decisions.md (rejection is the safer one), add
an `encode_component` short-digest row to `data-edge.json`, make Go return an `iscc:` error. **Scope
estimate:** 1-2 non-test files plus the vector fan-out.

Resolved when the short-digest behavior is decided and pinned by a shared vector that Go passes.
