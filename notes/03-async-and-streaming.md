# Async and Streaming Strategy

ISCC operations (hashing, chunking, similarity digests) are **CPU-bound, not I/O-bound**. The core
library takes bytes, not file paths — file I/O is the caller's responsibility. This distinction
drives every async/streaming decision.

> **Validated by iscc-sum**: The existing `bio-codes/iscc-sum` implementation uses synchronous Rust
> with streaming `update()/result()` and achieves 50-130x speedup over the Python reference. Adding
> async would have added complexity for zero performance gain.

## Core Principle: Sync Core, Streaming Interface, Per-Binding Async Adaptation

```
                       ┌────────────────────────────────┐
                       │       iscc (core crate)        │
                       │   Sync only. No tokio/async.   │
                       │   Streaming: update()/finalize()│
                       │   Parallelism: rayon (optional) │
                       └──────────────┬─────────────────┘
                                      │
          ┌───────────────┬───────────┼───────────┬───────────────┐
          │               │           │           │               │
    ┌─────┴─────┐   ┌────┴────┐ ┌────┴────┐ ┌────┴────┐   ┌─────┴─────┐
    │  iscc-py  │   │iscc-node│ │iscc-wasm│ │iscc-ffi │   │ iscc-cli  │
    │  Sync API │   │ Promise │ │  Sync   │ │ init/   │   │   Sync    │
    │ (GIL rel.)│   │ (async) │ │ exports │ │ update/ │   │  (rayon)  │
    └───────────┘   └─────────┘ └─────────┘ │finalize │   └───────────┘
                                            └─────────┘
```

## Per-Layer Decisions

| Layer                     | Approach                                                   | Rationale                                                                                                                  |
| ------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| **Rust core (`iscc`)**    | Sync only, no tokio/async-std                              | CPU-bound hashing has no awaitable operations. Async would infect the API with `Send + Sync` constraints for zero benefit. |
| **Streaming pattern**     | `new()` → `update(&[u8])` → `finalize() → Result<T>`       | Proven in iscc-sum. Works for CDC, BLAKE3, MinHash. Same pattern as `std::io::Write` and `blake3::Hasher`.                 |
| **Parallelism**           | `rayon` for batch operations (optional, feature-gated)     | Already used in iscc-sum for directory traversal. Does not require async.                                                  |
| **Python (`iscc-py`)**    | Sync API, release GIL during `update()` calls              | PyO3 auto-releases GIL. Callers use `asyncio.to_thread()` or `concurrent.futures` if they need async.                      |
| **Node.js (`iscc-node`)** | napi-rs `AsyncTask` wrapper around sync core               | Offloads CPU work to libuv thread pool. JS API returns `Promise<T>`. This is what SWC, Biome, and Oxc do.                  |
| **WASM (`iscc-wasm`)**    | Sync exports                                               | No threading in browser WASM. Streaming via `update()/finalize()`.                                                         |
| **C FFI (`iscc-ffi`)**    | `iscc_ctx_new()` / `_update()` / `_finalize()` / `_free()` | Standard streaming C API. Enables Go, Java, C# integration.                                                                |
| **CLI (`iscc-cli`)**      | Sync + rayon for parallelism                               | File I/O is synchronous; rayon parallelizes across files.                                                                  |

## Streaming API Shape

The streaming pattern is the same across all layers, adapted to each language's idiom:

**Rust core:**

Two streaming types matching iscc-core naming: `DataHasher` (for Data-Code) and `InstanceHasher`
(for Instance-Code).

```rust
pub struct DataHasher { /* ... */ }

impl DataHasher {
    pub fn new() -> Self { /* ... */ }
    pub fn update(&mut self, data: &[u8]) { /* ... */ }
    pub fn finalize(self, bits: u32) -> Result<DataCodeResult, IsccError> { /* ... */ }
}

pub struct InstanceHasher { /* ... */ }

impl InstanceHasher {
    pub fn new() -> Self { /* ... */ }
    pub fn update(&mut self, data: &[u8]) { /* ... */ }
    pub fn finalize(self, bits: u32) -> Result<InstanceCodeResult, IsccError> { /* ... */ }
}
```

**C FFI:**

```c
iscc_data_hasher_t*     iscc_data_hasher_new(void);
int                     iscc_data_hasher_update(iscc_data_hasher_t* ctx, const uint8_t* data, size_t len);
int                     iscc_data_hasher_finalize(iscc_data_hasher_t* ctx, uint32_t bits, /* out params */);
void                    iscc_data_hasher_free(iscc_data_hasher_t* ctx);

iscc_instance_hasher_t* iscc_instance_hasher_new(void);
int                     iscc_instance_hasher_update(iscc_instance_hasher_t* ctx, const uint8_t* data, size_t len);
int                     iscc_instance_hasher_finalize(iscc_instance_hasher_t* ctx, uint32_t bits, /* out params */);
void                    iscc_instance_hasher_free(iscc_instance_hasher_t* ctx);
```

**Python (via PyO3):**

```python
dh = DataHasher()
dh.update(chunk1)
dh.update(chunk2)
result = dh.finalize(bits=64)

ih = InstanceHasher()
ih.update(chunk1)
ih.update(chunk2)
result = ih.finalize(bits=64)
```

**Node.js (via napi-rs):**

```typescript
const result = await generateDataCode(buffer);  // AsyncTask wraps sync core
// Or streaming:
const dh = new DataHasher();
dh.update(chunk1);
dh.update(chunk2);
const result = dh.finalize(64);
```

## Why NOT Async in the Core

1. **No awaitable operations**: Hashing and chunking are pure computation on byte slices.
2. **Runtime infection**: `async fn` in the core forces all consumers to bring a runtime (tokio),
    including WASM and C FFI where that makes no sense.
3. **PyO3 async is immature**: PyO3's `async fn` support adds complexity and is less battle-tested
    than sync + GIL release.
4. **Rule of thumb**: Never expose Rust async across FFI boundaries. Adapt async outside the core.

## Exception: Remote ML Inference

If iscc-lib adds Semantic-Code with remote ML model inference (HTTP calls, GPU offloading), that
should live in a **separate optional crate** (`iscc-semantic`) with its own async runtime, not in
the core. The core remains sync and pure.
