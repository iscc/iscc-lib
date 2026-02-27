# Learnings

Accumulated knowledge from CID iterations. Each review agent appends findings here.

## Architecture

- Core crate must be pure Rust (no PyO3 dependency) so it can publish to crates.io independently
- Use `crates/` directory pattern with `workspace.dependencies` for centralized version management
- Only `iscc::api` (Tier 1) is bound to foreign languages; internal modules use `pub(crate)`
- Sync core with streaming interface: `new() -> update(&[u8]) -> finalize() -> Result<T>`

## Reference Implementation

- Official conformance vectors:
    `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- Reference Python package: `iscc/iscc-core` on GitHub
- Prior Rust work in `bio-codes/iscc-sum`: CDC, BLAKE3, streaming pattern, 50-130x speedup over
    Python

## Tooling

- mise for tool versions and task running
- maturin + PyO3 for Python bindings (abi3-py310 for single wheel per platform)
- uv for Python environment management
- Release profile: `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`

## Process

- `gen_meta_code_v0` in iscc-core has no `extra` parameter — only `name, description, meta, bits`

- Chromaprint feature vectors are signed `i32` (not `u32` or `f32`) — `gen_audio_code_v0` takes
    `&[i32]` because conformance vectors include negative values

- `gen_instance_code_v0` accepts a `bits` parameter in the Python reference (default 64)

- `gen_iscc_code_v0` takes `(codes, wide: bool)` — Python uses `wide=False` for 128-bit vs 256-bit
    output

- ST_ISCC SubType values are 0-7 (TEXT=0..MIXED=4, SUM=5, NONE=6, WIDE=7) — they share values 0-4
    with ST_CC, making a unified Rust SubType enum with values 0-7 correct for header encoding

- `codec` module is Tier 2 (`pub mod codec`) per notes/04, not `pub(crate)` — Tier 2 items are
    public Rust API but not exposed through FFI bindings

- Conformance test pattern: `include_str!("../tests/data.json")` + `serde_json::Value` for flexible
    parsing; `"stream:"` prefix in test vectors denotes hex-encoded byte data (empty after prefix =
    empty bytes); `hex` crate decodes test vector data

- `gen_instance_code_v0` is the simplest gen function: BLAKE3 hash → `encode_component` → "ISCC:"
    prefix. Good first implementation to establish patterns before tackling CDC/MinHash complexity

- `soft_hash_meta_v0` interleaves name/description SimHash digests in 4-byte chunks (8 chunks total
    = 32 bytes)

- `gen_text_code_v0` uses MinHash (NOT SimHash):
    `text_collapse → sliding_window(13) → xxh32 →   alg_minhash_256`. The `xxhash-rust` crate
    (feature `xxh32`) provides the hash function. The `minhash` module is ported from
    `bio-codes/iscc-sum` with MPA/MPB constants inlined

- `gen_data_code_v0` also uses MinHash (`alg_minhash_256`) plus CDC — the minhash module is shared

- `gen_meta_code_v0` normalizes name with `text_clean → text_remove_newlines → text_trim(128)` and
    description with `text_clean → text_trim(4096)` (no newline removal for description)

- `soft_hash_audio_v0` uses multi-stage SimHash: overall 4B + quarters 4×4B + sorted-thirds 3×4B =
    32B. Python reference uses `more_itertools.divide` (not `numpy.array_split`), but semantics are
    identical (first `len % n` parts get one extra element)

- `alg_simhash` output length matches input digest length — 4-byte digests in → 4-byte SimHash out.
    This makes it reusable for audio (4B digests) vs text/meta (32B BLAKE3 digests)

- `gen_mixed_code_v0` takes `&[&str]` (ISCC code strings, optional "ISCC:" prefix). The
    `soft_hash_codes_v0` helper prepares nbytes-length entries from `raw[0]` (header first byte) +
    body truncated to `nbytes-1`, then feeds to `alg_simhash`. Zero-padding handles short bodies

- `MainType` derives `Ord`/`PartialOrd` — discriminant order (Meta=0..Instance=4) matches required
    ISCC-CODE unit sort order, so derived ordering is correct

- `encode_units` uses bitfield (bit0=Content, bit1=Semantic, bit2=Meta) to encode optional unit
    combination as index 0–7 for the ISCC-CODE header length field

- `iscc-core/dct.py` uses Nayuki's fast recursive DCT (not naive O(n²) DCT-II). The Nayuki algorithm
    produces exact 0.0 for uniform inputs (via integer subtraction), which is critical for
    conformance. The Cython-compiled version returns raw f64 (no `int(round())` truncation)

- Image-Code 8×8 block extraction uses offset-by-1 positions `(0,0),(1,0),(0,1),(1,1)` (heavily
    overlapping), not offset-by-8. Always verify pseudocode against actual Python reference

- JSON metadata canonicalization requires RFC 8785 (JCS) — `serde_json` sorted-key serialization is
    insufficient because JCS has specific number formatting rules (`1.0` → `1`, `1e20` →
    `100000000000000000000`). The `serde_json_canonicalizer` crate provides full JCS compliance.
    Existing iscc-core test vectors only use string-valued JSON objects, so the divergence was not
    caught until float-valued metadata was tested (issue iscc/iscc-core#131)

- `IsccError::NotImplemented` variant was removed — all 9 gen functions are implemented

- PyO3 bindings: `maturin develop` needs `VIRTUAL_ENV` set to the project venv when the CID agent
    env differs (e.g., `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop ...`)

- maturin `pyproject.toml` requires `dynamic = ["version"]` when version comes from Cargo.toml —
    omitting this causes maturin to refuse building

- PyO3 `#[pyo3(signature = (data, bits=64))]` syntax provides Python-side default arguments cleanly

- PyO3 constants: register with `m.add("NAME", value)?` in module init (not `wrap_pyfunction!`).
    Re-export in `__init__.py` with `NAME as NAME` pattern for explicit re-export. Type stubs use
    `NAME: int` with inline docstrings

- `ty` type checker (pre-push hook) cannot resolve native extension module imports — PyO3/maturin
    modules need `.pyi` type stubs (e.g., `_lowlevel.pyi`) alongside `py.typed` for `ty` to pass

- PyO3 type mappings for slices: `&[&str]` → accept `Vec<String>` then convert via
    `codes.iter().map(|s| s.as_str()).collect()`. `&[i32]` → accept `Vec<i32>` then pass `&cv`.
    `&[u8]` and `&str` can be received directly by reference

- All 9 `gen_*_v0` PyO3 bindings follow identical thin-wrapper pattern: `#[pyfunction]` +
    `#[pyo3(signature)]` + `map_err(PyValueError)` + `PyDict::new(py)` with `set_item` per field.
    Optional fields use `if let Some(v)` to omit absent keys (matching iscc-core dict behavior)

- Python conformance tests share `data.json` with Rust tests via relative path
    (`Path(__file__).parent.parent / "crates/iscc-lib/tests/data.json"`). Use
    `pytest.param(...,   id=name)` for readable test IDs that match the JSON keys

- CI workflow: do NOT use `mise` in GitHub Actions — call `cargo`, `uv`, and tools directly. Use
    `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2` + `astral-sh/setup-uv@v4` +
    `actions/setup-python@v5` as the standard action set

- maturin must be in root `pyproject.toml` dev dependencies (not just
    `crates/iscc-py/pyproject.toml` build-requires) because CI runs `uv sync --group dev` from root
    only

- Python module name is `iscc_lib` (matching PyPI package name `iscc-lib`), not `iscc` — maturin
    `module-name` must be `iscc_lib._lowlevel`

- Criterion 0.5.x is the target version (not 0.8.x which has a different API). Use
    `criterion_group!`/`criterion_main!` macros with `harness = false` in `[[bench]]` section.
    `group.throughput()` sets throughput for subsequent benches in the group — call it before each
    `bench_with_input` when sizes differ

- pytest-benchmark: streaming functions (`gen_data_code_v0`, `gen_instance_code_v0`) need lambda
    wrappers with `io.BytesIO` to create fresh stream objects per iteration — iscc-core consumes the
    stream. Run benchmarks explicitly via `pytest benchmarks/python/ --benchmark-only` (not included
    in default testpaths)

- napi-rs type mappings differ from PyO3: owned `String` (not `&str`), `Buffer` (not `&[u8]`),
    `Option<T>` with `.unwrap_or()` for defaults (no native default parameter support). Use
    `#[napi(js_name = "snake_case")]` to prevent auto-conversion to camelCase

- All 9 `gen_*_v0` napi bindings follow identical thin-wrapper pattern: `#[napi(js_name)]` +
    `napi::Error::from_reason()` for error mapping. Mirrors the PyO3 pattern exactly

- Node.js conformance tests share `data.json` via relative path from `__tests__/` directory. Use
    Node.js built-in `node:test` + `node:assert` (zero extra dependencies).
    `Buffer.from(hex, 'hex')` replaces the `hex` crate for decoding stream test vectors

- wasm-bindgen type mappings: `&str` and `&[u8]` work directly (like PyO3, unlike napi-rs which
    needs owned `String`/`Buffer`). For `&[&str]` and `&[Vec<i32>]`, use `JsValue` +
    `serde_wasm_bindgen::from_value()` — wasm-bindgen cannot express nested JS arrays natively.
    Error mapping uses `JsError::new(&e.to_string())`

- WASM crate uses `cdylib` crate-type and `publish = false` (published via npm, not crates.io) —
    same pattern as the notes/02 architecture document specifies

- wasm-pack `--features` must go AFTER the path argument, NOT after `--`. `--` passes args to the
    test runner (wasm-bindgen-test-runner), not to cargo. Correct:
    `wasm-pack test --node crates/iscc-wasm --features conformance`

- WASM crate needs `crate-type = ["cdylib", "rlib"]` (not just `cdylib`) to support integration
    tests — Rust can't link `cdylib` for the test harness. wasm-pack uses `cdylib` for packaging and
    `rlib` for test compilation. This is the standard pattern for WASM crates with tests

- WASM conformance tests use `include_str!` for compile-time data embedding (WASM has no filesystem
    access at runtime). Path from `crates/iscc-wasm/tests/` to data is
    `../../iscc-lib/tests/data.json`

- C FFI crate uses thread-local `RefCell<Option<CString>>` for error storage — `iscc_last_error()`
    returns a pointer valid until the next gen call on the same thread. No extra dependencies needed
    beyond `iscc-lib` (pure C ABI using only `std::ffi`)

- C FFI type mappings: `&[u8]` → `*const u8` + `usize` len, `&[i32]` → `*const i32` + `usize` len,
    `&[&str]` → `*const *const c_char` + `usize` count, `Option<&str>` → nullable `*const c_char`.
    Helper functions `ptr_to_str`/`ptr_to_optional_str` centralize validation

- C test program links against cdylib (`.so`) with `LD_LIBRARY_PATH` at runtime. gcc needs
    `-lpthread -ldl -lm` for Rust runtime deps. For empty-data tests, pass `&single_byte` with
    `len=0` (not NULL) to satisfy Rust's non-null slice pointer requirement

- zensical documentation: `uv run zensical build` produces `site/` directory (already in
    `.gitignore`). Config lives in `zensical.toml`, docs in `docs/`. The `pymdownx.smartsymbols`
    extension doesn't convert `---` to em dashes — use Unicode `—` directly

- Rust `gen_*_v0` functions return dedicated `*CodeResult` structs (e.g., `MetaCodeResult`,
    `TextCodeResult`) with `.iscc` plus additional fields matching iscc-core dict returns. Python
    bindings return `PyDict` with all fields; wasm/napi/ffi bindings still extract `.iscc` only

- mkdocstrings + griffe: set `paths` to the parent directory containing the package (e.g.,
    `crates/iscc-py/python` not `crates/iscc-py/python/iscc_lib`) so griffe resolves imports
    correctly. Use `allow_inspection = false` to force static analysis from `.pyi` stubs when PyO3
    embedded docstrings lack parameter annotations

- mdformat-mkdocs mangles mkdocstrings `:::` directives with inline options (collapses indentation,
    escapes underscores). Workaround: move all mkdocstrings options to `zensical.toml` global config
    and keep the `:::` directive minimal (e.g., `::: iscc_lib` with no inline options)

- napi-rs cross-compilation for aarch64-linux: install `gcc-aarch64-linux-gnu` via apt-get and set
    `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc` env var. The env var is
    harmlessly ignored on non-arm64 matrix entries

- WASM npm package version patching: wasm-pack generates `pkg/package.json` with the Cargo crate
    name — use a Node.js script to read workspace version from root `Cargo.toml` and set both `name`
    and `version` fields. The regex `/^version\s*=\s*"(.+?)"/m` matches the first version line in
    the TOML (workspace version)

- iscc-core `gen_meta_code_v0` normalizes description before the meta/text branch (line 57-59 in
    `code_meta.py`). The Rust implementation must do the same — description normalization is NOT
    text-path-only

- `build_meta_data_url` uses `data_encoding::BASE64` (with padding) to match Python's
    `DataURL.from_byte_data()`. `@context` key → `application/ld+json`, otherwise `application/json`

- Result struct pattern: `#[non_exhaustive]` + `pub use types::*` at crate root. Binding crates use
    `.map(|r| r.iscc)` for backward-compatible string extraction until dict returns are implemented

- Python hybrid result pattern: `IsccResult(dict)` with `__getattr__` → `__getitem__` delegation in
    pure Python `__init__.py`, wrapping `_lowlevel` PyO3 dicts. 9 typed subclasses with class-level
    annotations provide IDE completion. No Rust changes needed — the wrapper layer is entirely
    Python

- `ty` type checker does NOT support `hasattr()`-based type narrowing —
    `if hasattr(data, "read"):   data.read()` fails with `call-non-callable`. Use `isinstance`
    inversion instead: `if not isinstance(data, bytes): data = data.read()`. This gives `ty` proper
    narrowing while preserving duck-typing behavior at runtime

- Tier 1 API promotion pattern: change `pub(crate) mod X` → `pub mod X` in `lib.rs`, change selected
    functions from `pub(crate) fn` → `pub fn`, add `pub use X::{fn1, fn2}` re-exports at crate root
    for flat imports. `pub(crate)` items in a `pub` module remain invisible outside the crate — no
    leakage risk

- Pre-push hook runs `cargo clippy --workspace --all-targets` (workspace-wide), while local
    validation typically uses `cargo clippy -p iscc-lib`. Newer clippy lints (e.g.,
    `cloned_ref_to_slice_refs` in Rust 1.93+) may only surface in `--all-targets` mode which
    includes integration tests. Always verify with workspace-wide clippy before pushing

- `DataHasher` streaming CDC: the `prev_chunk` pattern (hash all chunks except last, carry last as
    tail) is critical for correctness across `update()` boundaries. Byte-at-a-time streaming
    produces identical results to one-shot because CDC handles sub-minimum-size input by returning
    the entire buffer as one chunk

- `DataHasher` buffer optimization: persistent `buf: Vec<u8>` replaces per-call `to_vec()`/
    `concat()`. Key pattern: `extend_from_slice` → CDC → extract `tail_len` (usize) before
    `drop(chunks)` → `copy_within(tail_start.., 0)` + `truncate(tail_len)`. The explicit `drop` is
    needed because CDC chunks borrow from `self.buf`

- `InstanceHasher` constructs multihash directly from BLAKE3 digest
    (`format!("1e20{}", hex::encode(...))`) — avoids calling `multi_hash_blake3` which would
    redundantly rehash the same data. The `1e20` prefix is the BLAKE3 multihash header (codec 0x1e,
    length 0x20=32 bytes)

- `conformance_selftest()` uses `passed &= run_*_tests()` (bitwise AND-assign on `bool`) to
    accumulate pass/fail without short-circuiting — all 9 sections always execute, providing
    complete diagnostic output. Each helper uses closure-returning-`Option<bool>` pattern to catch
    parse errors without panicking

- PyO3 FFI boundary pattern for Rust `assert!`/`panic!`: pre-validate inputs in the `#[pyfunction]`
    wrapper and return `PyValueError` before calling the Rust function. Example: `sliding_window`
    checks `width < 2` to avoid a Rust panic propagating across FFI. Use `map_err(PyValueError)` for
    functions that return `Result`; use pre-validation for functions that use `assert!`

- PyO3 `#[pyclass]` streaming hasher pattern: use `Option<Inner>` where `update()` calls
    `inner.as_mut().ok_or(already finalized)?` and `finalize()` calls `inner.take().ok_or(...)`.
    Python wrapper class in `__init__.py` adds `BinaryIO` support and optional constructor `data`
    parameter. Import lowlevel class with underscore prefix (`_DataHasher`) to avoid name collision
    with the wrapper class. All 30 Tier 1 Python symbols: 41 total in `__all__` (30 API + 10 result
    type classes + `__version__`)

- CID workflow: `next.md` now requires a `## Not In Scope` section and the review handoff uses a
    structured `[x]`/`[ ]` verification grid. Verification criteria should be boolean-testable
    (runnable commands) whenever possible. Review agent commits `iterations.jsonl` alongside other
    context files

- CID workflow: verification `grep` criteria must match the specific problematic pattern, not a
    generic substring that also matches the replacement code. E.g., `grep 'data = data\.read()'` to
    detect unbounded reads, not `grep 'isinstance(data, bytes)'` which also matches the new inner
    bytearray→bytes conversion

- napi-rs build artifacts (`index.js`, `index.d.ts`, `*.node`, `node_modules/`) belong in the crate
    directory (napi-rs convention) — gitignore them via `crates/iscc-napi/.gitignore`, don't
    redirect `--output-dir` since it breaks CI artifact paths and `napi prepublish`

- napi-rs streaming class pattern: `#[napi(js_name = "ClassName")]` struct + `#[napi(constructor)]`
    \+ `#[napi(js_name = "finalize")] pub fn finalize_code(...)` (avoids conflict with napi-rs
    `ObjectFinalize` trait). `pub` visibility + `Default` impl resolve clippy
    dead_code/new_without_default lints that arise because napi macro glue is only generated for
    cdylib targets, not `--all-targets`

- npm package name is `@iscc/lib` (scoped under `@iscc` org), not `@aspect-build/iscc-lib` — verify
    package names against `crates/iscc-napi/package.json` when writing documentation metadata

- zensical template overrides: `overrides/main.html` must extend `base.html` (not `main.html`)
    because zensical resolves `main.html` to the override first, creating a cycle. Also, `page.meta`
    can be `None` on pages without YAML front matter — use `{% if page.meta and   page.meta.X %}`
    guards instead of `page.meta.X or fallback` chains

- JNI crate pattern: `cdylib` only (no `staticlib` — JVM loads shared libs), `publish = false`
    (published via Maven, not crates.io), `extern "system"` calling convention. Rust 2024 edition
    requires `#[unsafe(no_mangle)]` instead of `#[no_mangle]`. JNI function names encode package
    underscores as `_1` (e.g., `iscc_lib` → `iscc_1lib`). The `jni` crate v0.21 compiles as pure
    Rust — no JDK needed at build time

- API hardening pattern for Tier 1 functions: split into validated `pub fn foo() -> IsccResult<T>`
    and unchecked `pub(crate) fn foo_inner() -> T`. Internal callers use `_inner` to skip redundant
    validation. Binding crates use `map_err` to convert `IsccError` to their native error type
    (PyValueError, napi::Error, JsError, FFI last_error). This avoids cascading return-type changes
    to internal helpers

- `decode_length` returns multiples of 32 bits for standard ISCC MainTypes (Meta, Semantic, Content,
    Data, Instance, Flake), multiples of 64 for ISCC-CODE, and multiples of 8 for ID —
    `bit_length / 8` in `iscc_decode` is always safe without remainder checking

## Go/wazero Bridge

- Go module path is `github.com/iscc/iscc-lib/packages/go`, package name `iscc`. Go is installed via
    mise (`go = "latest"` in `[tools]`) — requires `mise exec --` prefix to run Go commands

- wazero v1.11.0 (pure-Go WASM runtime, no CGO). WASM binary embedded via `//go:embed iscc_ffi.wasm`
    — must be pre-built and copied to `packages/go/` before tests. TestMain skips gracefully if
    missing

- `iscc_last_error()` returns a borrowed pointer — Go bridge must NOT call `iscc_free_string` on it.
    Other string-returning FFI functions (like `iscc_text_clean`) return owned strings that must be
    freed

- `text_clean` does NOT collapse double spaces within a line — use NFKC ligature normalization
    (e.g., fi ligature U+FB01 → "fi") for test cases instead of space-collapsing expectations

- WASM i32 alignment: `iscc_alloc(0)` returns a dangling pointer with alignment 1, which panics in
    `slice::from_raw_parts` for `*const i32` (needs alignment 4). `writeI32Slice` must allocate
    minimum 4 bytes for empty slices to ensure proper alignment. `writeBytes` (u8) is fine with
    alignment 1

- Go conformance test pattern: `json.RawMessage` for deferred parsing + helper functions
    (`parseBits`, `parseStreamData`, `parseF64Array`, `f64ToI32`, `f64ToByte`). Meta test vectors
    with dict values need `json.Marshal` to serialize back to string before passing to FFI

- WASM sret ABI for struct returns: `iscc_free_byte_buffer` and `iscc_free_byte_buffer_array` take
    the struct by pointer (single i32 param) on wasm32, not as flattened fields. The sret pointer
    from the function call can be reused directly as the free function's argument — no extra alloc
    needed. `IsccByteBuffer` and `IsccByteBufferArray` are both 8 bytes (2×i32). Individual buffers
    in an array are at `buffersPtr + i*8`

- Go `IsccDecode` returns `*DecodeResult` (pointer to struct, not value) — idiomatic Go for
    non-trivial return types. The sret cleanup sequence is: copy digest to Go memory →
    `iscc_free_decode_result(sret_ptr)` → `dealloc(sret_ptr, 16)`. Order matters: digest data is
    freed by `iscc_free_decode_result`, so it must be copied first

- Go `packages/go/iscc_ffi.wasm` is tracked in git (release build, ~683KB). When FFI exports change
    in `crates/iscc-ffi/src/lib.rs`, the binary must be rebuilt
    (`cargo build -p iscc-ffi --target   wasm32-wasip1 --release`), copied to `packages/go/`, and
    recommitted. CI builds a fresh debug binary for testing (overwrites committed release in CI env
    only)

- All 6 language bindings (Python, Node.js, WASM, C FFI, Java JNI, Go/wazero) now have 30/30 Tier 1
    symbols as of iteration 12

## Publishing

- Workspace version is `0.0.1` (experimental). Version is defined once in root `Cargo.toml`
    `[workspace.package]` and inherited by all crates. napi `package.json` version must be kept in
    sync manually
- ISCC Foundation URL is `https://iscc.io` (not `iscc.foundation` or other variants) — use this in
    all documentation, README, and docs site links
- WASM npm package name is `@iscc/wasm` (not `@iscc/iscc-wasm`) — the release workflow patches
    `pkg/package.json` after `wasm-pack build`
- crates.io OIDC trusted publishing requires the crate to exist first (no "pending publisher" like
    PyPI). First publish uses a one-time API token, then configure trusted publishing on the crate's
    settings page
- PyPI supports pending trusted publishers — can configure OIDC before first publish at pypi.org
    Account Settings → Publishing
- npm does not support OIDC — uses `NPM_TOKEN` repository secret. Packages are scoped under `@iscc`
    org
- Maven Central (for Java JNI bindings) requires GPG signing + Sonatype credentials. Namespace
    verification needed for `io.iscc`. Deferred until Java bindings are functional

## Branching

- CID loop runs on `develop` branch; `main` is protected (requires PR + passing CI)

- All CID agents are branch-agnostic — they commit/push to whatever branch is checked out. The
    `update-state` agent uses `git branch --show-current` for CI status queries (not hardcoded
    `main`)

- CI triggers on push to both `main` and `develop`; docs deploy only from `main`; releases are
    tag-based (branch-agnostic)

- Use `mise run pr:main` to create a PR from `develop` → `main` when ready to merge

- Python `IntEnum` classes (`MT`, `ST`, `VS`) in `__init__.py`: pure Python, no Rust changes. `ST`
    has `TEXT = 0` alias for `NONE` (IntEnum allows duplicate values — first definition is
    canonical). `iscc_decode` wrapper converts raw integers to IntEnum types. `core_opts` is a
    `SimpleNamespace` mapping attribute names to existing constants. `__all__` has 45 entries after
    this addition

- `GABTQLB6CQ6ILWLO` is NOT a valid 64-bit Data-Code — it has 8-byte body but header indicates
    128-bit/16-byte digest. Use `GAA2XTPPAERUKZ4J` for a valid 64-bit Data-Code with digest
    `\xab\xcd\xef\x01\x23\x45\x67\x89`. define-next should validate test vectors before including
    them in verification criteria

- C FFI `iscc_decode` length index for 64-bit codes is 1 (not 0): `decode_length` uses
    `(length_index + 1) * 32` for standard MainTypes (index 0 = 32-bit, index 1 = 64-bit). The
    next.md spec had this wrong; the advance agent corrected it

- C FFI C test compilation requires cbindgen header generation before gcc can build:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output <path>/iscc.h`. The
    generated header is not committed — CI generates it dynamically

- WASM binding constants use `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` — the JS export name is
    uppercase despite the Rust function being `meta_trim_name()`. Always verify documented API names
    against actual `js_name` attributes in `crates/iscc-wasm/src/lib.rs`, not just next.md specs

## Go Bindings — Pure Go Rewrite

- Go bindings are being rewritten from WASM/wazero bridge to pure Go. The WASM approach required
    committing a ~700KB binary to git and inflating `check-added-large-files` to 1024KB — build
    artifacts do not belong in git tracking

- The Rust core is ~6,300 lines of pure algorithms that translate directly to Go. Key Go
    dependencies: `github.com/zeebo/blake3` (BLAKE3), xxHash library (feature hashing),
    `golang.org/x/text/unicode/norm` (NFKC/NFD). Standard library covers base32/base64/hex/json

- The pure Go approach eliminates: WASM binary in git, wazero runtime dependency, WASM interpreter
    overhead, cross-compilation target for WASM in CI, inflated large-file threshold

- The existing public API surface (30 Tier 1 symbols) must be preserved. The `Runtime` struct
    pattern from wazero is replaced by package-level functions (more idiomatic Go). Existing test
    functions need adaptation to remove `Runtime` ceremony

- Port order should follow dependency graph: codec (standalone) → text utils (standalone) →
    algorithms (CDC, MinHash, SimHash, DCT, WTA-Hash — all standalone) → gen\_\*\_v0 functions
    (compose algorithms) → streaming hashers → conformance selftest

- Pure Go codec module (`packages/go/codec.go`) is complete — type enums, varnibble header
    encoding/decoding, base32/base64, length encode/decode, unit encode/decode, `EncodeComponent`,
    `IsccDecompose`, `IsccDecode`. Uses only Go stdlib (no external deps). 48 tests passing

- Go codec `EncodeComponent` takes `uint8` for enum fields (not typed `MainType`/`SubType`) to match
    the Rust Tier 1 API pattern. Internal `encodeComponentInternal` wraps this for typed callers
    within the package

- Go `IsccDecompose` does NOT strip dashes (matching Rust `iscc_decompose`). Only `IsccDecode`
    strips dashes. This is intentional — decompose operates on normalized base32 strings

- Conformance test vectors are the correctness contract — as long as Go passes the same vectors as
    Rust and Python, the implementations are equivalent. The existing `iscc_test.go` (1,353 lines,
    46 test functions) provides comprehensive coverage

- Pure Go text utils module (`packages/go/utils.go`) is complete — `TextClean`, `TextCollapse`,
    `TextTrim`, `TextRemoveNewlines` + 2 internal helpers. Uses `golang.org/x/text/unicode/norm`
    (first external dependency). 21 tests passing

- Go dependency management: `go get` adds dependencies as `// indirect` regardless of whether
    they're directly imported — always run `go mod tidy` afterward to fix annotations
