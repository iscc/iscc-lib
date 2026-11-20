---
name: v080-iteration-ledger
description: Per-iteration scoping ledger for v0.8.0 (iters 195-235) with traps, probes and the open-issue picture
metadata:
  type: project
---

**Status 2026-10-17:** v0.6.0 shipped 2026-07-30, v0.7.0 shipped 2026-10-17 (iscc-core 1.5.0
Image-Code fix, npm OIDC). Active milestone: **v0.8.0 ISCC-SEQ** (target.md). The v0.6.0 Phase
section below and the Unicode-freeze entries are history — the freeze was removed 2026-07-30. See
[ISCC-SEQ phase](iscc-seq-phase.md) for the v0.8.0 slice plan and traps (195 = Rust core; 196 =
CI-red fix; 197 = Python seq binding; 198 = core `iscc_clean` trim parity; 199 = Go strict base32;
200 = Go `isccClean`; 201 = Rust+Go case-fold alias fix; 202 = Go seq; 203 = text-utils trim parity
Rust+Go; 204 = napi+WASM seq; 205 = C FFI seq; 206 = .NET seq; 207 = C++ seq; 208 = JNI seq; 209 =
Ruby seq; 210 = UniFFI seq, last surface; 211 = 33→35 doc sweep + seq doc entries, fan-out ~30 doc
files; 212 = Ruby `RArray` fix + json bump; 213 = .NET+C++ embedded-NUL rejection (decided
2026-11-19; .NET throws `IsccException` at `ToNativeUtf8`, C++ one `detail` helper); 214 = npm
`license`/`description` for `@iscc/wasm` (Cargo.toml) + `@iscc/lib` (package.json had the same
badge-markdown description — npm derives it from the README when missing; probe sibling packages for
a defect class); 215 = data-edge.json seed + Rust/Python harnesses (NEEDS_WORK: CRAP); 216 = harness
repair; 217 = data-edge fan-out G1 (6 surfaces; container crash after Go+Node → restart re-scoped
the other 4 with Go/Node criteria carried over); 218 = G2 Swift+Kotlin + Ruby key fix; 219 = G3
C+C++ generated header, see [data-edge phase](data-edge-phase.md); 220 = .NET one jagged-array pin
helper over all 6 sites (issue named 3 — finish-the-pattern widened it); 221 = C FFI docs
`iscc_`-prefixed type names (c-ffi-api.md + ffi README; issue named only the structs — `Ffi*Hasher`,
README and the doc's cbindgen cmd lacking `--config` widened it) + pytest deriving names from
`iscc.h`; 222 = Python `encode_seq` docstrings — 3 Python copies (wrapper, `.pyi`, PyO3 `///` in
`iscc-py/src/lib.rs` = `_lowlevel.__doc__`; issue named 2); 223 = phantom repo paths in specs (wasm
issue named 1 file; a backticked-path existence loop over `specs/*.md` found go `version.go`,
`go-api.md`, `nodejs-api.md` too — the loop is a reusable probe); 224 =
`scripts/check_spec_paths.py` (trees + backticked paths vs `git ls-files` — NOT disk: untracked
napi/wasm build output made a disk probe falsely green; `GENERATED`/`NOT_PATHS` dicts, no in-spec
marker; phantom tree lines often have prose twins — grep basenames outside the tree); 225 =
`alg_simhash([])` rejects in core + Go (behaviour files: simhash.rs + 4 Go; pass-through bindings
need only their zero-pinning tests flipped + comment-only docs — NOT a fan-out; CRAP baseline regen
for the new branch). 226 = `alg_simhash` data-edge section, all 12 surfaces in ONE step (a new
section is unsplittable: unhandled-section failure + generator fail-closed + vendored copies); 2
vectors (`[]` error + `[f00f,3c3c]`→`fc3f` tie, so handlers decode inputs). 227 = C++ `safe_data` on
ALL 9 vector `.data()` sites (issue named 5; FFI len-0 short-circuits made 4 non-defects, widened
anyway) via one template helper; source grep pins sites a `{}` test can't. 228 = spec gate leniency
(2 files: `*`→`[^/]*`, `**`→`.*`; `NOT_PATHS` backtick-only; replay tests on a `_known()` set +
monkeypatched `tracked_paths`). 229 = Ruby `i32` Bignum (Magnus 0.8.2: only `RBignum::to_i32` lacks
the lower check; u32/usize fine; out-of-i32 is a native-range limit → per-binding tests, NOT
data-edge). 230 = both JNI issues (null guard + error mapping): the issue's `exception_check` fix
was a no-op — jni 0.22.4 `JString::new` catches+CLEARS OOM into `JniCall(NoMemory)` ("JNI call
failed"), so map by error TYPE (trait over jni/IsccError/&str) at all 90 sites; also found JNI docs
say finalized hasher throws IAE (code: ISE). Read the dep source before trusting any issue's fix.
231 = iscc-ffi `mod tests` → `src/tests.rs` (verbatim move; csbindgen reads only `src/lib.rs`, so
exports stay put; gate = regen `iscc.h` + `NativeMethods.g.cs` → `git status` empty). 232 =
Swift+Python data-edge compare strictness (2 test files; other 9 harnesses probed strict; Swift
negatives must parse JSON TEXT — Swift literals are native Bool, not NSNumber; Darwin `as? Int`
takes `false`, `as? Bool` takes 0/1, macOS proof only in CI). 233 = Tier 1 doc-parity gate
(`check_tier1_docs.py`): source parsed from lib.rs column-0 `pub fn`/`pub const`/named `pub use`;
FULL (one-way, casing-normalised) + EXACT (marker blocks in rust-core.md + iscc-lib CLAUDE.md) +
EXEMPT registry, fail-closed discovery at ≥30 distinct symbols; it landed red: java-api lacked
`genSumCodeV0`+`META_TRIM_META` despite saying "All 35" — doc fixes rode along. 234 = WASM
`alg_cdc_chunks` → `Vec<js_sys::Uint8Array>` (wasm-bindgen 0.2.126 `Vec<T>` for JsValue-repr T,
slices.rs:546; the only `to_value` return in iscc-wasm); picked over the gate-hole step because
230–233 were all tooling. 235 = security pass (rustix lock, Go x/text v0.41.0 +
`toolchain go1.26.11` line, CI Node 20→22, devcontainer GO_VERSION, ci-cd.md runtime row); probe
kit, all read-only: `go run …/osv-scanner/v2/cmd/osv-scanner@latest scan source -r .` (skips
gitignored napi lock — fresh-resolve npm instead; `cargo deny` missed a RUSTSEC osv saw),
govulncheck `-show verbose`, `dotnet list <csproj> package --vulnerable`, osv `/v1/query` for
Maven/Actions, endoflife.date API for runtime EOLs. Data-edge strict-compare issue needs an
output-subset DECISION first: .NET `TextCodeResult` has only `Iscc` too. Next: that decision step,
gate holes (fail-closed parser + register 4 sub-threshold docs) + Java how-to fix, then part 2
sweep. 236 = data-edge output-subset rule (spec + validator + C/C++ generator, harnesses next):
C/C++ generator `gen_iscc` had the SAME silent key drop as the 4 named harnesses
(finish-the-pattern: 6 surfaces, not 4); allowed keys DERIVED from `data.json` gen outputs (no hand
list — drift issue); uncompared keys fixed statically per harness; advance can't write decisions.md
→ review records. Next: 4-harness strict fan-out (.NET/Java/napi/WASM, negative test each), then
Java how-to + Ruby gem docs line, then gate holes. Unfiled: ref truncates mismatched digest lengths,
Rust/Go reject — needs a human decision. Trap: the issue says C/C++ fail `alg_simhash([])` with the
NULL error, but `iscc_alg_simhash` short-circuits `num_digests == 0` to the core — re-probe before
restating. Still open: gate leniency, Tier 1 parity check; setuptools bump resolves to 84.2.1 via
`uv lock --upgrade-package setuptools` (dry-run 223); napi Buffer leak is `low` — skip;
Ruby/JNI/ffi-split are `normal`). **Dependabot soupsieve is human-blocked**: 3.0.2/4.x need Python
\>=3.11.5 but `uv.lock` resolves `>=3.10` — escalate, don't scope. 237 = 4-harness data-edge strict
compare fan-out (.NET/Java/napi/WASM, test-only): all four return bare ISCC strings for
text/mixed/iscc → static uncompared sets text→`characters`, mixed→`parts`, iscc→none; negatives must
be ones the OLD intersect passed (`{}` actual was already rejected → use actual `{characters}`;
key-outside-set = `parts` on text with correct iscc, defeats a global set); .NET's only object
vector has `except.dotnet` → needed a synthetic positive. Next: Java how-to + Ruby gem docs line,
then drift-issue gate holes. A 303 from a Maven/portal probe proves nothing: `curl -sL` to follow it
(220: Kotlin red was a portal outage). 238 = Java how-to `genSumCodeV0` fix + Ruby gem range wording
(docs-only, 3 files + a pytest that derives `Ruby {min} to {max}` from release.yml cross-gem
`ruby-versions` — a version range in docs is a derived fact, so it ships with a parity test).
Wording branch only; never claim 3.4/4.0 support (source build unmeasured). Next: drift-issue gate
holes (fail-closed lib.rs parser + register rust.md/c-cpp.md/rust-api.md/architecture.md), then part
3 count sweep. 239 = Ruby source gem made buildable (issue branch 1, NOT "drop the source gem":
`ruby-bindings.md` L164/L323 mandate a source gem, so dropping it amends a spec criterion → Titusz).
Facts: crates.io core can't work (HEAD uses `encode_seq`, absent in 0.7.0) → carry the core in the
gem via a staging script (`cargo package -p iscc-lib` normalizes its manifest; `conformance.rs`
needs `tests/data.json`); WRONG (239 review): cross-gem greps the FIRST line containing `rb_sys`,
unanchored, so an unconditional gemspec rb_sys dep (version-less PATH line) broke the lookup; rb_sys
0.9.130 needs Ruby >=2.7; `--local` install fails once rb_sys is a dep; never name the carried dir
`vendor/`/`tmp/` (gitignored, swept into platform gems). Next: drift gate holes, then part 3 count
sweep. 240 = repair of 239 on top of unpushed `d299106`: rb_sys gemspec dep conditional on
`File.directory?(File.join(__dir__, "core"))` (staged tree only) → lock back to `d299106~1`,
platform gems keep zero runtime deps; test models the action's REAL pipeline + fire test; `run()`
stdout-only. Lesson: model a third-party action from its `action.yml`, never from my own paraphrase.
Next: drift gate holes, then part 3 count sweep.
