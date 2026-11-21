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

## One-shot gen functions re-implement their streaming counterparts `normal` [audit]

Verified by the iteration-249 and iteration-260 audits (3040298).

- `gen_instance_code_v0` (`crates/iscc-lib/src/lib.rs:907-908`) calls `blake3::hash(data)` and then
    `utils::multi_hash_blake3(data)`, which hashes the same bytes again (`utils.rs:164`): two full
    BLAKE3 passes per call. The multihash format is also written in `streaming.rs:39-56`.
- `gen_sum_code_v0` in `crates/iscc-wasm/src/lib.rs` copies `SumHasher::update`/`finalize`
    (`streaming.rs:172-203`), comment included; the same file already wraps `SumHasher`.
- `gen_meta_code_v0` parses `meta` twice (`lib.rs:372` via `parse_meta_json`, then `:406-408`), and
    `json_to_data_url` (`:322-328`) repeats the parse+canonicalize body with another error string.

**Why it matters:** each pair can drift apart, and the instance hot path does twice the hashing.
**Suggested fix:** derive `datahash` from the single digest (one shared multihash helper), call
`SumHasher` from the WASM one-shot function, and have `parse_meta_json` return what the data URL
step needs. **Scope estimate:** 3 non-test files; mind the iai-callgrind baseline.

Resolved when each computation exists once and the conformance and perf gates stay green.

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

## `@iscc/wasm` docs show a Node.js setup the published package cannot run `normal` [audit]

Verified by the iteration-270 audit at 34303a6. The npm package is built with
`wasm-pack build --target web` (`release.yml:427`). Its default `init()` builds a URL from
`import.meta.url` and calls `fetch()` on it, which Node's `fetch` refuses for `file:` URLs.

- `docs/howto/wasm.md` → "Node.js" (:58-72) imports from `"@iscc/wasm"` and calls `gen_text_code_v0`
    with no init at all, after telling the reader to build with `--target nodejs` (the reader
    installed the npm package and has no build). :42 says most bundlers initialize WASM
    automatically, which a web-target package never does.
- `crates/iscc-wasm/README.md:28-37`: the quick start's `await init()` fails under Node, and the
    next line says "Both browser and Node.js targets are supported."
- `docs/howto/wasm.md:82` and `README.md:56` say every gen function returns a string;
    `gen_sum_code_v0` returns `WasmSumCodeResult` (`lib.rs:348-353`, and the how-to's own example
    reads `result.iscc`). The README's Streaming line omits `SumHasher`.

**Why it matters:** the registry page and how-to are the first thing a Node user copies, and both
paths fail on the published artifact. **Suggested fix:** show the Node path for the web build
(`initSync({ module: readFileSync(...) })` or bytes passed to `init`), probed against a packed
release build; drop the bundler and "both targets" claims; correct the return-type sentences.
**Scope estimate:** 2 non-test files.

Resolved when the Node example runs against a `--target web` release build and both pages state
`gen_sum_code_v0`'s result type.

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

## Go `GenMetaCodeV0` never enforces the 128,000-byte meta payload limit `normal` [audit]

Verified by the iteration-280 audit at db1fe55. `packages/go/codec.go:22` declares
`MetaTrimMeta = 128_000`, and `packages/go/README.md:117` and `docs/howto/go.md:468` document it as
the "max decoded payload size in bytes for the meta element". No code reads it:
`grep -rn MetaTrimMeta packages/go` finds only the declaration. `GenMetaCodeV0`
(`packages/go/code_meta.go:53-69`) decodes the Data-URL or canonicalizes the JSON and hashes the
whole payload, whatever its size. The Rust core rejects a decoded payload over the limit, and checks
before decoding too (`crates/iscc-lib/src/lib.rs:363-385`). iscc-core raises `ValueError`
(`reference/iscc-core/iscc_core/code_meta.py:72-76`).

**Why it matters:** the pure-Go port returns a code where the reference and every Rust-backed
binding return an error, and its docs promise a limit it does not apply. No vector pins the limit,
so the conformance gates cannot see the gap.

**Suggested fix:** in `GenMetaCodeV0`, reject a payload longer than `MetaTrimMeta` with the core's
message (and add the core's pre-decode bound on the Data-URL length). Pin it with Go tests at
128,000 and 128,001 bytes for both meta forms. Consider a shared `data-edge.json` vector if the
edge-vector rules allow inputs of this size.

**Scope estimate:** 1 non-test file.

Resolved when an oversized meta payload is an error in Go, as it is in the core, and a test pins the
boundary.

## vcpkg portfile pins the SHA512 hashes of the v0.2.0 tarballs `normal` [audit]

Verified by the iteration-280 audit at db1fe55. `packages/cpp/portfile.cmake` builds its download
URL from `${VERSION}` (`:43-45`, so 0.7.0 from `vcpkg.json:3`, which `version_sync.py` keeps in
step). The five `ISCC_SHA512` values (`:14-38`) have not changed since commit `d140555c`, where they
were added for v0.2.0. Measured: the published `iscc-ffi-v0.2.0-x86_64-unknown-linux-gnu.tar.gz`
hashes to `ebed9b50…`, the portfile value, and the v0.7.0 tarball to `26f396ed…`. So
`vcpkg_download_distfile` fails its hash check on every platform for every release after 0.2.0.
Neither `version_sync.py`, `release.yml` nor any test touches the hashes, and no CI job runs the
port. `docs/architecture.md:110` and `packages/cpp/CLAUDE.md:13` present the port as a distribution
channel.

**Why it matters:** the vcpkg path has been broken for five releases and nothing would notice the
next one. Every release silently adds one more version that cannot install.

**Suggested fix:** make the hashes a release step (`release.yml` writes them from the built
tarballs, as it does for the Swift checksum), or drop the hard-coded hashes and the port claim. Add
a test that ties the portfile to the release artifacts, or a check that fails while the hashes
belong to another version.

**Scope estimate:** 2-3 non-test files.

Resolved when the portfile's hashes match the tarballs of the version in `vcpkg.json`, and a gate
keeps them matched.

## Rust and Go per-function tests re-parse `data.json` beside the conformance runners `normal` [audit]

Verified by the iteration-280 audit at db1fe55. The rewritten runners
(`crates/iscc-lib/src/conformance.rs`, `packages/go/conformance.go`) parse every `gen_*` section,
but parallel parsers remain:

- Rust: nine `test_gen_*_conformance` tests in `crates/iscc-lib/src/lib.rs:1317-1953` (plus
    `codec.rs:1806`) `include_str!` the fixture and decode inputs by hand. The meta dispatch
    (`lib.rs:1336-1341`) and the `stream:` decoding (`:1840`, `:1884`) repeat
    `conformance.rs:126-131` and `:228-231`.
- Go: about ten `*_test.go` files `os.ReadFile` the monorepo path instead of the embedded copy.
    `code_meta_test.go:49-63` copies `metaArg`, and `code_data_test.go:37` and
    `code_instance_test.go:37` copy `decodeStream`. `conformance.go` itself parses the document
    twice through `parseConformanceData` (`:29`) and `parseSeqConformanceData` (`:345`), with two
    vector types.
- The runners disagree on malformed vectors. Rust `strip_prefix("stream:")?` rejects a bare hex
    string, while Go `TrimPrefix` (`conformance.go:270`) accepts it. A missing `iscc` output is a
    parse failure in Rust (`conformance.rs:111-113`, pinned `:391`), but Go's `expectedISCC` reports
    a mismatch. The Go failure-arm tests leave out both cases.

**Why it matters:** a `data.json` schema change has to be fixed in three or four places per
language, and the two "mirror" selftests already give different verdicts on the same fixture.

**Suggested fix:** expose the runners' case parsers to the in-crate and in-package tests, and keep
only the checks of the extra output fields (`metahash`, `datahash`, …) in the per-function tests.
Use one Go parse with `Outputs json.RawMessage`. Settle one malformed-vector policy and pin it on
both sides.

**Scope estimate:** needs audit-scope step (≤8 files); Rust and Go can be separate steps.

Resolved when each language parses `data.json` inputs in one place and both runners give the same
verdict on the same malformed case.

## `@iscc/wasm` hasher `finalize()` consumes the hasher on some argument errors but not others `normal` [audit]

Verified by the iteration-249/260/270 audits; unchanged at db1fe55. `DataHasher.finalize`,
`InstanceHasher.finalize` and `SumHasher.finalize` (`crates/iscc-wasm/src/lib.rs:703-718`,
`758-770`, and the SumHasher method after them) run `opt_u32`/`opt_bool` *before* `inner.take()`. So
a `bits` that the JS-side check rejects (`-1`, `1.5`, `"x"`) throws and leaves the hasher usable. A
`bits` that the core rejects (e.g. `13`) throws *after* `take()` and consumes it.
`docs/howto/wasm.md:338` says "After calling `finalize()`, the hasher is consumed". Go decided that
the first `Finalize` consumes even when it fails (decisions.md, 2026-11-21). No other surface states
its rule, and no test pins either branch.

**Why it matters:** whether a caller can retry after a bad argument depends on which validator
caught it. That is an implementation accident, and copying it to other surfaces makes it the API.

**Suggested fix:** decide the cross-binding rule (follow Go: the first `finalize` consumes, whatever
its outcome, or validate everything before consuming) and record it. Apply it to the three WASM
hashers, pin both error kinds with a test, and state the rule in `wasm.md`.

**Scope estimate:** 2 non-test files.

Resolved when a rejected `bits` leaves every WASM hasher in the same state, whichever check rejects
it, as decided and pinned by a test.

## Rust public API docs misclassify `IsccError`, omit `SumHasher`, and contradict `gen_sum_code_v0` `normal` [audit]

Verified by the iteration-280 audit at db1fe55. `crates/iscc-lib/src/lib.rs:8-14` makes `codec`,
`streaming` and `types` public, and `IsccError` is defined at the crate root (`:51`).

- `docs/architecture.md:141-144` names `codec` as the only Tier 2 module, and nothing classifies
    `streaming`/`types`. `iscc_lib::streaming::SumHasher` (`streaming.rs:157`) is public and bound
    in Python and WASM, yet it is absent from `docs/rust-api.md`, `architecture.md` and the Tier 1
    doc gate.
- `docs/architecture.md:162-163`: "The core library takes bytes, not file paths". But Tier 1
    `gen_sum_code_v0` takes a path (`lib.rs:1059-1068`).
- `docs/rust-api.md:678-687` presents `IsccError` as a `codec` type ("Import codec types with
    `use iscc_lib::codec::…`").
- Root `CLAUDE.md:154` says the bound surface is "`iscc_lib::api`", a module that does not exist.
    `CLAUDE.md:217` and the crate doc `lib.rs:3` say all 10 `gen_*_v0` functions match `data.json`
    (`gen_sum_code_v0` has no vectors).

**Why it matters:** these are the pages a Rust user and every agent read to learn what is covered by
SemVer and what the bindings may bind. A public type outside every tier list cannot be reasoned
about at the v1.0.0 stability cut.

**Suggested fix:** classify `streaming::SumHasher` (Tier 1 re-export or documented Tier 2). Fix the
three sentences in `architecture.md`/`rust-api.md`, and point `CLAUDE.md` and the crate doc at the
crate-root re-exports and the right vector count. A repo-wide `git grep` on each phrase (learnings:
false claims outlive their pages).

**Scope estimate:** 4-5 non-test files.

Resolved when every public path of `iscc-lib` is classified in one tier list and the cited sentences
match `lib.rs`.
