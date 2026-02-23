# API Surface, Compatibility, and Safety

Polyglot Rust projects succeed or fail on three things that are easy to under-specify early: (1) a
clear API surface policy, (2) a compatibility contract per layer, and (3) a disciplined safety model
for FFI boundaries. Treat all three as part of the public API.

## API Surface Policy

The Rust core API is **not automatically public API**. Polyglot projects fail when the Rust core
grows organically, bindings expose "just a bit more", and the public surface explodes unevenly
across languages.

**Tiered API model:**

| Tier                                   | Scope                                      | Rule                                        |
| -------------------------------------- | ------------------------------------------ | ------------------------------------------- |
| **Tier 1** (crate root `pub` fns)      | Stable entrypoints, bound in all languages | SemVer-governed, changes require MAJOR bump |
| **Tier 2** (`pub mod codec`)           | Rust-only (enums, header encode/decode)    | May change in MINOR releases                |
| **Internal** (`pub(crate)` or private) | Never exposed to bindings                  | Free to change                              |

Tier 1 includes 22 public symbols: 9 gen functions, 4 text utilities, 4 algorithm primitives, 1 soft
hash, 1 encoding utility, 1 codec operation, 2 streaming types, and 1 diagnostic function. See
`specs/rust-core.md` for the complete listing.

**Concrete pattern:**

```rust
// crates/iscc-lib/src/lib.rs

// Tier 1 public API — bound in all languages:
// - 9 gen_*_v0 functions (code generation)
// - text_clean, text_remove_newlines, text_trim, text_collapse (text utils)
// - sliding_window, alg_minhash_256, alg_cdc_chunks, alg_simhash (algorithms)
// - soft_hash_video_v0 (soft hash)
// - encode_base64, iscc_decompose (encoding/codec)
// - DataHasher, InstanceHasher (streaming)
// - conformance_selftest (diagnostics)

pub mod codec;     // Tier 2: Rust-only (MainType, SubType, encode/decode_header, etc.)
pub(crate) mod cdc;           // Internal module (alg_cdc_chunks re-exported at crate root)
pub(crate) mod minhash;       // Internal module (alg_minhash_256 re-exported at crate root)
pub(crate) mod simhash;       // Internal module (alg_simhash, sliding_window re-exported)
```

Tier 1 functions are public at the crate root (`iscc_lib::text_clean`, `iscc_lib::gen_meta_code_v0`,
etc.). Bindings wrap these crate-root functions. Internal modules remain `pub(crate)` — only
selected functions are re-exported as public API.

**Discipline rules:**

- Adding to Tier 1 is a deliberate act requiring review
- Bindings must not define semantics — they translate the core API to the target language's idiom
- Don't expose Rust enums directly in bindings (they break when variants are added)
- Don't share mutable state across FFI boundaries

## FFI Stability Contract

Define and document stability guarantees per layer early. Without this, every release is risky.

| Layer                       | Stability   | Breaking Change Policy                        |
| --------------------------- | ----------- | --------------------------------------------- |
| Rust core API (`iscc::api`) | Stable      | SemVer MAJOR bump                             |
| Python API                  | Stable      | SemVer MAJOR bump, no class renames in MINOR  |
| Node.js API                 | Stable      | SemVer MAJOR bump                             |
| C ABI                       | Pinned      | Only additive changes unless ABI version bump |
| WASM exports                | Best-effort | Versioned by package version                  |

**C ABI versioning (recommended):**

```c
// iscc.h
#define ISCC_ABI_VERSION 1

int iscc_abi_version(void);  // Runtime compatibility check
```

This allows downstream languages (Go via cgo, Java via Panama, C# via P/Invoke) to verify they're
linked against a compatible version at runtime.

**Versioning semantics:**

- `0.x`: API may change, but C ABI remains additive
- `1.0`: Rust + Python APIs stabilized
- Pre-releases (`-alpha`, `-beta`): Rust + Python only, no guarantees for other bindings

## Compatibility Policy (write this down early)

Decide and document, in-repo, what you support:

- **Rust**: MSRV policy (e.g., "N-2 stable" or fixed MSRV) and how/when it's bumped.
- **Python**: minimum Python (e.g., `>=3.10`) and whether you publish `abi3` wheels.
- **Node.js**: minimum Node LTS version(s) and whether you ship native-only, WASM-only, or both.
- **WASM**: which targets you support (`wasm32-unknown-unknown` for browsers; optionally WASI for
    server runtimes).
- **C ABI**: whether the C surface is stable/semver'd, and what stability guarantees you offer.

Practical recommendation: keep dev tooling current (via `mise.toml`), but keep runtime compatibility
explicit (via `rust-version`, `requires-python`, and CI matrices that enforce what you claim).

## Feature Flags Strategy (avoid "everything everywhere")

Feature flags are the main lever to keep bindings small and predictable:

- Make the core crate (`iscc`) the "truth" and keep its default feature set lean.
- Put language/runtime specifics behind features (`python`, `node`, `wasm`, `ffi`) in *binding
    crates*, not in the core crate when possible.
- Avoid enabling "convenience features" by default in bindings (e.g., `serde_json`, `tokio`) unless
    the public API truly requires them.

## FFI Safety Checklist (Python/Node/C/WASM)

At every boundary, standardize these rules:

- **No panics across FFI**: never let a Rust panic unwind into Python/Node/C; convert to an error or
    abort deterministically.
- **Ownership and lifetimes**: never hand foreign runtimes pointers to Rust stack data; use owned
    allocations and clear "who frees what" rules.
- **Opaque handles over structs**: prefer `*mut T` handles with constructor/destructor functions
    over exposing Rust struct layouts.
- **String and buffer conventions**: explicitly define encoding (`UTF-8`), null-termination rules
    (C), and whether buffers are length-prefixed.
- **Threading**: document thread-safety for each exported type/function (especially for Node and
    Python GIL interactions).

For C specifically, use `#[repr(C)]` only for *C-facing* types, keep them minimal, and avoid
exposing `Vec<T>`/`String>` directly.

### Memory Ownership Table

| Resource               | Owner   | Freed By | Notes                                |
| ---------------------- | ------- | -------- | ------------------------------------ |
| Rust struct            | Rust    | Rust     | Exposed via opaque handle (`*mut T`) |
| Byte buffer (output)   | Rust    | Rust     | Access via `(ptr, len)` pair         |
| String (output)        | Rust    | Rust     | UTF-8, null-terminated for C         |
| Error message          | Rust    | Rust     | Thread-local last-error pattern      |
| Foreign string (input) | Foreign | Foreign  | Copied to Rust-owned on ingress      |
| Foreign buffer (input) | Foreign | Foreign  | Borrowed for duration of call        |

**Hard rule**: The side that allocates memory is the side that frees it.

## Error Model (make it identical everywhere)

Polyglot projects get brittle when each binding invents its own error taxonomy. Define one canonical
error model in the core crate and map it consistently to every runtime.

**Recommended Rust shape (core crate):**

- One public error type: `iscc::Error` (e.g., `thiserror` enum)
- Stable, machine-readable codes (e.g., `ErrorCode` enum) separate from human messages
- A clear split between:
    - `InvalidInput` (bad caller data)
    - `Unsupported` / `NotImplemented` (feature gating / optional algorithms)
    - `External` (I/O, model, codec, optional dependencies)
    - `Internal` (bugs; should be rare)

**Binding mappings (keep predictable):**

- **Python (PyO3)**: raise custom exceptions (e.g., `IsccError` base + subclasses by code)
- **Node (napi-rs)**: throw JS `Error` with a stable `code` string (and message)
- **WASM (wasm-bindgen)**: return `Result<T, JsValue>` where error is a structured object
    `{ code, message }`
- **C FFI**: return an `int` status code + expose `iscc_last_error_message()` for the message

Practical rule: never stringify errors too early. Preserve `code` until the outermost layer where it
turns into an exception / error object.

## Conformance Tests (official ISO vectors, cross-language)

Treat conformance as a first-class deliverable: before adding new bindings, ensure the Rust core and
every binding can run the same canonical test vectors.

**Source of truth**: the ISO reference implementation provides JSON conformance vectors in
`iscc_core/data.json` (upstream repo `iscc/iscc-core`), mirrored as a raw file here:
[`iscc_core/data.json`](https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json).

**Data model (what to expect):**

- Top-level keys are function names (e.g., `gen_data_code_v0`)
- Each function contains named test cases with `inputs` and `outputs`
- Some suites mark whether a function is required vs optional for conformance
- Raw byte inputs/outputs are embedded as hex strings with a prefix (documented as `stream:` /
    `bytes` for byte streams, and `hex:` for byte outputs).

**Recommended approach for iscc-lib:**

1. **Vendor a pinned snapshot** of the JSON into the repo (avoid network in tests) and update it via
    a small script when you intentionally bump conformance data.
2. Build a tiny **decoder** for vector inputs:
    - If value is a string with `stream:` / `bytes` / `hex:` prefix, decode hex → bytes
    - Otherwise treat it as normal JSON (string/number/bool/object)
3. Create a single **Rust conformance test harness** that:
    - Iterates all required functions
    - Calls your Rust API with decoded inputs
    - Compares outputs in a normalized way (e.g., ISCC strings, hex for bytes)
4. Make every binding run the *same* vectors by reusing the vendored JSON:
    - Python: `pytest` parametrized over vectors
    - Node: `vitest`/`jest` parametrized over vectors
    - WASM: run vectors under a JS test runner (and/or `wasmtime` for WASI if you support it)

This gives you a regression net that catches subtle cross-language drift (encoding, rounding, option
defaults) immediately.

## Cross-Binding Parity Tests

Conformance vectors verify that each binding matches the ISO reference. Parity tests verify that
**all bindings match each other** — a subtly different and equally important guarantee.

**What parity tests catch that conformance vectors miss:**

- UTF-8 normalization differences between runtimes
- Float/rounding behavior (JavaScript `Number` vs Python `float` vs Rust `f64`)
- Default parameter mismatches (a binding accidentally defaults `wide=True` while another defaults
    `wide=False`)
- Error code mapping inconsistencies (Rust returns `InvalidInput`, Python raises `ValueError`, Node
    throws `TypeError`)

**Recommended structure:**

```
tests/
├── vectors/
│   └── data.json              # Vendored conformance vectors (shared)
├── rust/                       # cargo test loads vectors
├── python/                     # pytest parametrized over vectors
├── node/                       # vitest parametrized over vectors
└── wasm/                       # vitest + wasm-pack test
```

**Key rule:** Every binding loads the **identical JSON file** and compares outputs in a normalized
form. If a binding adds a convenience wrapper (e.g., Python accepts `str` where Rust accepts
`&[u8]`), the parity test must bypass the wrapper and test the underlying call.
