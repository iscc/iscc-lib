---
icon: lucide/book-open
description: C FFI API reference for the ISCC shared library.
---

# C FFI API Reference

C-compatible shared library (`libiscc_ffi`) exposing all ISCC (ISO 24138:2024) code generation
functions, algorithm primitives, codec operations, text utilities, and streaming hashers as
`extern "C"` symbols. Usable from C, C++, C#, and any language with C interop.

## Build

```bash
cargo build -p iscc-ffi --release
```

The shared library is written to `target/release/` as `libiscc_ffi.so` (Linux), `libiscc_ffi.dylib`
(macOS), or `iscc_ffi.dll` (Windows).

Generate the C header with [cbindgen](https://github.com/mozilla/cbindgen):

```bash
cbindgen --crate iscc-ffi --output iscc.h
```

## Memory Model

- Functions returning `char*` produce heap-allocated strings. Free with `iscc_free_string()`.
- Functions returning `char**` produce NULL-terminated string arrays. Free with
    `iscc_free_string_array()`.
- Functions returning `IsccByteBuffer` produce heap-allocated byte buffers. Free with
    `iscc_free_byte_buffer()`.
- Functions returning `IsccByteBufferArray` produce heap-allocated buffer arrays. Free with
    `iscc_free_byte_buffer_array()`.
- On error, pointer-returning functions return `NULL`. Check `iscc_last_error()` for the message.
- Streaming hashers return opaque pointers. Free with the corresponding `_free()` function.

## Types

### IsccByteBuffer

Heap-allocated byte buffer.

```c
typedef struct {
    uint8_t *data;  // Pointer to byte data (NULL on error)
    size_t len;     // Number of bytes
} IsccByteBuffer;
```

### IsccByteBufferArray

Array of byte buffers (returned by `iscc_alg_cdc_chunks`).

```c
typedef struct {
    IsccByteBuffer *buffers;  // Pointer to array of IsccByteBuffer elements
    size_t count;             // Number of buffers
} IsccByteBufferArray;
```

### IsccDecodeResult

Result of decoding an ISCC unit string (returned by `iscc_decode`).

```c
typedef struct {
    bool ok;               // Whether the decode succeeded
    uint8_t maintype;      // MainType enum value (0-7)
    uint8_t subtype;       // SubType enum value (0-7)
    uint8_t version;       // Version enum value
    uint8_t length;        // Length index from the header
    IsccByteBuffer digest; // Raw digest bytes
} IsccDecodeResult;
```

---

## Constants

Getter functions for algorithm constants.

### iscc_meta_trim_name

Maximum byte length for the name field after trimming.

```c
uint32_t iscc_meta_trim_name(void);
```

Returns `128`.

---

### iscc_meta_trim_description

Maximum byte length for the description field after trimming.

```c
uint32_t iscc_meta_trim_description(void);
```

Returns `4096`.

---

### iscc_io_read_size

Default read buffer size for streaming I/O.

```c
uint32_t iscc_io_read_size(void);
```

Returns `4194304` (4 MB).

---

### iscc_text_ngram_size

Sliding window width for text n-gram generation.

```c
uint32_t iscc_text_ngram_size(void);
```

Returns `13`.

---

## Code Generation

All 9 `iscc_gen_*_v0` functions return a heap-allocated ISCC string (prefixed with `ISCC:`) on
success, or `NULL` on error. Free the result with `iscc_free_string()`.

### iscc_gen_meta_code_v0

Generate a Meta-Code from name and optional metadata.

```c
char* iscc_gen_meta_code_v0(
    const char *name,
    const char *description,
    const char *meta,
    uint32_t bits
);
```

| Parameter     | Type          | Description                                                        |
| ------------- | ------------- | ------------------------------------------------------------------ |
| `name`        | `const char*` | Title or name of the content (required, trimmed to 128 bytes)      |
| `description` | `const char*` | Optional description text (NULL = not provided, trimmed to 4096 B) |
| `meta`        | `const char*` | Optional JSON string or `data:` URL (NULL = not provided)          |
| `bits`        | `uint32_t`    | Bit length of the generated code (default: 64, multiple of 32)     |

---

### iscc_gen_text_code_v0

Generate a Text-Code from plain text content.

```c
char* iscc_gen_text_code_v0(const char *text, uint32_t bits);
```

| Parameter | Type          | Description                                                    |
| --------- | ------------- | -------------------------------------------------------------- |
| `text`    | `const char*` | Plain text content (required)                                  |
| `bits`    | `uint32_t`    | Bit length of the generated code (default: 64, multiple of 32) |

---

### iscc_gen_image_code_v0

Generate an Image-Code from pixel data.

```c
char* iscc_gen_image_code_v0(
    const uint8_t *pixels,
    size_t pixels_len,
    uint32_t bits
);
```

| Parameter    | Type             | Description                                             |
| ------------ | ---------------- | ------------------------------------------------------- |
| `pixels`     | `const uint8_t*` | 1024 grayscale pixel values (32x32 image, values 0-255) |
| `pixels_len` | `size_t`         | Number of bytes in the pixel buffer                     |
| `bits`       | `uint32_t`       | Bit length of the generated code (default: 64, max 256) |

---

### iscc_gen_audio_code_v0

Generate an Audio-Code from a Chromaprint feature vector.

```c
char* iscc_gen_audio_code_v0(
    const int32_t *cv,
    size_t cv_len,
    uint32_t bits
);
```

| Parameter | Type             | Description                                                    |
| --------- | ---------------- | -------------------------------------------------------------- |
| `cv`      | `const int32_t*` | Chromaprint signed integer fingerprint vector                  |
| `cv_len`  | `size_t`         | Number of elements in the feature vector                       |
| `bits`    | `uint32_t`       | Bit length of the generated code (default: 64, multiple of 32) |

---

### iscc_gen_video_code_v0

Generate a Video-Code from frame signature data.

```c
char* iscc_gen_video_code_v0(
    const int32_t *const *frame_sigs,
    const size_t *frame_lens,
    size_t num_frames,
    uint32_t bits
);
```

| Parameter    | Type                     | Description                                                    |
| ------------ | ------------------------ | -------------------------------------------------------------- |
| `frame_sigs` | `const int32_t *const *` | Array of pointers to frame signature arrays                    |
| `frame_lens` | `const size_t*`          | Array of lengths for each frame signature                      |
| `num_frames` | `size_t`                 | Number of frames                                               |
| `bits`       | `uint32_t`               | Bit length of the generated code (default: 64, multiple of 32) |

Each frame signature is a 380-element `int32_t` array (MPEG-7 frame signature).

---

### iscc_gen_mixed_code_v0

Generate a Mixed-Code from multiple Content-Code strings.

```c
char* iscc_gen_mixed_code_v0(
    const char *const *codes,
    size_t num_codes,
    uint32_t bits
);
```

| Parameter   | Type                 | Description                                                    |
| ----------- | -------------------- | -------------------------------------------------------------- |
| `codes`     | `const char *const*` | Array of ISCC Content-Code strings (optional `ISCC:` prefix)   |
| `num_codes` | `size_t`             | Number of code strings (must be >= 2)                          |
| `bits`      | `uint32_t`           | Bit length of the generated code (default: 64, multiple of 32) |

---

### iscc_gen_data_code_v0

Generate a Data-Code from raw byte data.

```c
char* iscc_gen_data_code_v0(
    const uint8_t *data,
    size_t data_len,
    uint32_t bits
);
```

| Parameter  | Type             | Description                                                    |
| ---------- | ---------------- | -------------------------------------------------------------- |
| `data`     | `const uint8_t*` | Pointer to raw byte data                                       |
| `data_len` | `size_t`         | Number of bytes                                                |
| `bits`     | `uint32_t`       | Bit length of the generated code (default: 64, multiple of 32) |

---

### iscc_gen_instance_code_v0

Generate an Instance-Code from raw byte data.

```c
char* iscc_gen_instance_code_v0(
    const uint8_t *data,
    size_t data_len,
    uint32_t bits
);
```

| Parameter  | Type             | Description                                                    |
| ---------- | ---------------- | -------------------------------------------------------------- |
| `data`     | `const uint8_t*` | Pointer to raw byte data                                       |
| `data_len` | `size_t`         | Number of bytes                                                |
| `bits`     | `uint32_t`       | Bit length of the generated code (default: 64, multiple of 32) |

---

### iscc_gen_iscc_code_v0

Generate a composite ISCC-CODE from individual unit codes.

```c
char* iscc_gen_iscc_code_v0(
    const char *const *codes,
    size_t num_codes,
    bool wide
);
```

| Parameter   | Type                 | Description                                                     |
| ----------- | -------------------- | --------------------------------------------------------------- |
| `codes`     | `const char *const*` | Array of ISCC unit code strings (optional `ISCC:` prefix)       |
| `num_codes` | `size_t`             | Number of code strings (must include Data-Code + Instance-Code) |
| `wide`      | `bool`               | Enable 256-bit wide mode (Data+Instance only)                   |

---

## Text Utilities

Text processing functions for normalization and cleaning. All return heap-allocated strings. Free
with `iscc_free_string()`.

### iscc_text_clean

Clean and normalize text for display. Applies NFKC normalization, removes control characters (except
newlines), normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips leading/trailing
whitespace.

```c
char* iscc_text_clean(const char *text);
```

---

### iscc_text_remove_newlines

Remove newlines and collapse whitespace to single spaces. Converts multi-line text into a single
normalized line.

```c
char* iscc_text_remove_newlines(const char *text);
```

---

### iscc_text_trim

Trim text so its UTF-8 encoded size does not exceed `nbytes`. Multi-byte characters that would be
split are dropped entirely.

```c
char* iscc_text_trim(const char *text, size_t nbytes);
```

---

### iscc_text_collapse

Normalize and simplify text for similarity hashing. Applies NFD normalization, lowercasing, removes
whitespace and characters in Unicode categories C (control), M (mark), and P (punctuation), then
recombines with NFKC normalization.

```c
char* iscc_text_collapse(const char *text);
```

---

## Algorithm Primitives

Low-level algorithm functions for SimHash, MinHash, content-defined chunking, and video hashing.

### iscc_sliding_window

Generate sliding window n-grams from a string. Returns a NULL-terminated array of overlapping
substrings of `width` Unicode characters.

```c
char** iscc_sliding_window(const char *seq, uint32_t width);
```

| Parameter | Type          | Description                               |
| --------- | ------------- | ----------------------------------------- |
| `seq`     | `const char*` | Input string                              |
| `width`   | `uint32_t`    | Window width in Unicode characters (>= 2) |

Free the result with `iscc_free_string_array()`.

---

### iscc_alg_simhash

Compute a SimHash digest from an array of byte digests. The output length matches the input digest
length.

```c
IsccByteBuffer iscc_alg_simhash(
    const uint8_t *const *digests,
    const size_t *digest_lens,
    size_t num_digests
);
```

| Parameter     | Type                     | Description                             |
| ------------- | ------------------------ | --------------------------------------- |
| `digests`     | `const uint8_t *const *` | Array of pointers to digest byte arrays |
| `digest_lens` | `const size_t*`          | Array of lengths for each digest        |
| `num_digests` | `size_t`                 | Number of digests                       |

Free the result with `iscc_free_byte_buffer()`.

---

### iscc_alg_minhash_256

Compute a 256-bit MinHash digest from 32-bit integer features. Uses 64 universal hash functions with
bit-interleaved compression.

```c
IsccByteBuffer iscc_alg_minhash_256(
    const uint32_t *features,
    size_t features_len
);
```

| Parameter      | Type              | Description               |
| -------------- | ----------------- | ------------------------- |
| `features`     | `const uint32_t*` | Pointer to feature values |
| `features_len` | `size_t`          | Number of features        |

Returns an `IsccByteBuffer` with 32 bytes. Free with `iscc_free_byte_buffer()`.

---

### iscc_alg_cdc_chunks

Split data into content-defined chunks using gear rolling hash.

```c
IsccByteBufferArray iscc_alg_cdc_chunks(
    const uint8_t *data,
    size_t data_len,
    bool utf32,
    uint32_t avg_chunk_size
);
```

| Parameter        | Type             | Description                              |
| ---------------- | ---------------- | ---------------------------------------- |
| `data`           | `const uint8_t*` | Pointer to raw byte data                 |
| `data_len`       | `size_t`         | Number of bytes                          |
| `utf32`          | `bool`           | If true, align cuts to 4-byte boundaries |
| `avg_chunk_size` | `uint32_t`       | Target average chunk size in bytes       |

Free the result with `iscc_free_byte_buffer_array()`.

---

### iscc_soft_hash_video_v0

Compute a similarity-preserving hash from video frame signatures.

```c
IsccByteBuffer iscc_soft_hash_video_v0(
    const int32_t *const *frame_sigs,
    const size_t *frame_lens,
    size_t num_frames,
    uint32_t bits
);
```

| Parameter    | Type                     | Description                                 |
| ------------ | ------------------------ | ------------------------------------------- |
| `frame_sigs` | `const int32_t *const *` | Array of pointers to frame signature arrays |
| `frame_lens` | `const size_t*`          | Array of lengths for each frame signature   |
| `num_frames` | `size_t`                 | Number of frames                            |
| `bits`       | `uint32_t`               | Hash bit length                             |

Returns raw bytes of length `bits / 8`. Free with `iscc_free_byte_buffer()`.

---

## Codec Operations

Encoding, decoding, and decomposition of ISCC codes.

### iscc_encode_base64

Encode bytes as base64url (RFC 4648 section 5, no padding).

```c
char* iscc_encode_base64(const uint8_t *data, size_t data_len);
```

Free the result with `iscc_free_string()`.

---

### iscc_json_to_data_url

Convert a JSON string into a `data:` URL with JCS canonicalization. Uses `application/ld+json` media
type when the JSON contains an `@context` key, otherwise `application/json`.

```c
char* iscc_json_to_data_url(const char *json);
```

Free the result with `iscc_free_string()`.

---

### iscc_encode_component

Encode raw ISCC header components and digest into a base32 ISCC unit string (without `ISCC:`
prefix).

```c
char* iscc_encode_component(
    uint8_t mtype,
    uint8_t stype,
    uint8_t version,
    uint32_t bit_length,
    const uint8_t *digest,
    size_t digest_len
);
```

| Parameter    | Type             | Description                               |
| ------------ | ---------------- | ----------------------------------------- |
| `mtype`      | `uint8_t`        | MainType enum value (0-7)                 |
| `stype`      | `uint8_t`        | SubType enum value (0-7)                  |
| `version`    | `uint8_t`        | Version enum value                        |
| `bit_length` | `uint32_t`       | Bit length of the digest (multiple of 32) |
| `digest`     | `const uint8_t*` | Pointer to raw digest bytes               |
| `digest_len` | `size_t`         | Number of bytes in the digest buffer      |

Free the result with `iscc_free_string()`.

---

### iscc_decode

Decode an ISCC unit string into header components and raw digest. Strips an optional `ISCC:` prefix.

```c
IsccDecodeResult iscc_decode(const char *iscc);
```

Check `result.ok` to determine success. Free with `iscc_free_decode_result()`.

---

### iscc_decompose

Decompose a composite ISCC-CODE into individual ISCC-UNITs.

```c
char** iscc_decompose(const char *iscc_code);
```

Returns a NULL-terminated array of heap-allocated strings. Free with `iscc_free_string_array()`.

---

## Streaming

Streaming hashers for processing large data incrementally. Each hasher follows the lifecycle:
`_new()` → `_update()` (repeated) → `_finalize()` → `_free()`.

!!! warning

    `_finalize()` consumes the hasher state. After finalizing, subsequent `_update()` or `_finalize()`
    calls will fail. You must still call `_free()` to release the wrapper.

### DataHasher

Streaming Data-Code hasher using content-defined chunking and MinHash.

```c
// Create a new DataHasher
FfiDataHasher* iscc_data_hasher_new(void);

// Push data into the hasher (returns true on success)
bool iscc_data_hasher_update(
    FfiDataHasher *hasher,
    const uint8_t *data,
    size_t data_len
);

// Finalize and return the ISCC string (free with iscc_free_string)
char* iscc_data_hasher_finalize(FfiDataHasher *hasher, uint32_t bits);

// Free the hasher (NULL is a no-op)
void iscc_data_hasher_free(FfiDataHasher *hasher);
```

### InstanceHasher

Streaming Instance-Code hasher using BLAKE3.

```c
// Create a new InstanceHasher
FfiInstanceHasher* iscc_instance_hasher_new(void);

// Push data into the hasher (returns true on success)
bool iscc_instance_hasher_update(
    FfiInstanceHasher *hasher,
    const uint8_t *data,
    size_t data_len
);

// Finalize and return the ISCC string (free with iscc_free_string)
char* iscc_instance_hasher_finalize(FfiInstanceHasher *hasher, uint32_t bits);

// Free the hasher (NULL is a no-op)
void iscc_instance_hasher_free(FfiInstanceHasher *hasher);
```

---

## Diagnostics

### iscc_conformance_selftest

Run all conformance tests against vendored test vectors.

```c
bool iscc_conformance_selftest(void);
```

Returns `true` if all tests pass, `false` if any fail.

---

## Memory Management

Every allocation from the FFI layer must be freed with the matching function. Passing `NULL` is
always a no-op.

| Function                      | Frees                                                      |
| ----------------------------- | ---------------------------------------------------------- |
| `iscc_free_string`            | `char*` from any `iscc_gen_*`, text utility, or codec call |
| `iscc_free_string_array`      | `char**` from `iscc_decompose` or `iscc_sliding_window`    |
| `iscc_free_byte_buffer`       | `IsccByteBuffer` from algorithm primitives                 |
| `iscc_free_byte_buffer_array` | `IsccByteBufferArray` from `iscc_alg_cdc_chunks`           |
| `iscc_free_decode_result`     | `IsccDecodeResult` from `iscc_decode`                      |

```c
void iscc_free_string(char *ptr);
void iscc_free_string_array(char **arr);
void iscc_free_byte_buffer(IsccByteBuffer buf);
void iscc_free_byte_buffer_array(IsccByteBufferArray arr);
void iscc_free_decode_result(IsccDecodeResult result);
```

### iscc_alloc / iscc_dealloc

Low-level memory allocation helpers (primarily for WASM host integration).

```c
uint8_t* iscc_alloc(size_t size);
void iscc_dealloc(uint8_t *ptr, size_t size);
```

`iscc_alloc` returns a pointer to `size` bytes. Free with `iscc_dealloc(ptr, size)` using the same
size. Returns a non-null dangling pointer for `size == 0`.

---

## Error Handling

On error, pointer-returning functions return `NULL` and store the error message in thread-local
storage. Retrieve it with `iscc_last_error()`:

```c
const char* iscc_last_error(void);
```

Returns a pointer to the last error message on the current thread, or `NULL` if no error has
occurred. The pointer is valid until the next `iscc_*` call on the same thread. **Do not free** the
returned pointer.

```c
#include <stdio.h>

char *result = iscc_gen_text_code_v0("Hello World", 64);
if (result == NULL) {
    const char *err = iscc_last_error();
    fprintf(stderr, "Error: %s\n", err);
} else {
    printf("ISCC: %s\n", result);
    iscc_free_string(result);
}
```
