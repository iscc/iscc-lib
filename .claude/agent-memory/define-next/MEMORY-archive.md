# Define-Next Agent Memory — Archive

Archived scoping decisions from completed project phases. Moved here to reduce per-invocation
context loading. Full history preserved in git.

See MEMORY.md for current active entries.

## C# / .NET Binding Details (archived iteration 10 — phase complete)

- Three layers: C FFI → P/Invoke (NativeMethods.g.cs, csbindgen) → Idiomatic wrapper (IsccLib.cs)
- Package lives in `packages/dotnet/`, DLL name `"iscc_ffi"`
- csbindgen in build.rs, NativeMethods.g.cs committed (929 lines, 47 externs, 6 structs)
- Wrapper scoping: batched by marshaling complexity (string→string → byte[]→string → structs →
    arrays → streaming IDisposable)
- Jagged array marshaling: GCHandle.Alloc per inner array + fixed on outer + finally cleanup
- SafeHandle + IDisposable for IsccDataHasher / IsccInstanceHasher
- 91 tests (41 smoke + 50 conformance), System.Text.Json for data.json parsing
- NuGet pipeline: pack-nuget → test-nuget → publish-nuget in release.yml
- Cross-arch find bug fix: scope path pattern by target name `-path "*-${target}/*"`
- .csproj relative paths: count `../` from csproj location, not project root

## Streaming SumHasher (#37) + Python GIL release (#39) — archived iter 93, all closed

- **iter 88: SumHasher CORE** (`iscc_lib::streaming::SumHasher`). Composes the existing
    `DataHasher`+`InstanceHasher` (holds both, `update` feeds the same slice to both);
    `finalize(bits, wide, add_units) -> SumCodeResult` ports `gen_sum_code_v0`'s composition (calls
    `crate::gen_iscc_code_v0`); `gen_sum_code_v0` refactored to read file → drive the hasher.
    Intentionally **NOT** promoted to crate-root Tier 1 — Tier 1 (32 symbols) means "bound in all
    languages", but #37 only adds it to Python+WASM, so a count bump would imply false cross-binding
    parity. Kept README/rust-core.md/target.md counts at 32.
- **iter 89: Python SumHasher WRAPPER** — `crates/iscc-py/src/lib.rs` (PySumHasher mirrors
    PyDataHasher `Option<inner>`), `__init__.py` (wrapper class + `__all__`), `_lowlevel.pyi`.
    Stream handling lives in the Python wrapper; `_lowlevel` update takes `&[u8]` only.
- **iter 90: WASM SumHasher WRAPPER** — `crates/iscc-wasm/src/lib.rs`. Holds
    `Option<iscc_lib::streaming::SumHasher>` (full path, no re-export); finalize maps core
    `SumCodeResult` → `WasmSumCodeResult` (cast `filesize: u64 as f64`). Tests in `tests/unit.rs`
    via `wasm-pack test --node`. Closed #37 across all bindings.
- **iter 91: Python GIL release (#39)** — wrapped pure-Rust compute in `py.allow_threads(...)` at 7
    sites in `crates/iscc-py/src/lib.rs` (4 one-shot fns + 3 `update()` methods; added
    `py: Python<'_>` to the latter, invisible to Python). Keep `PyDict` construction OUTSIDE the
    closure. `allow_threads` is the PyO3 0.23 name (NOT `detach`, which is 0.25+).
- `gen_sum_code_v0` has a full test suite in lib.rs; streaming.rs has its own `#[test]`s; Python
    streaming tests live in project-root `tests/test_streaming.py`.
