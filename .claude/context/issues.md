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
