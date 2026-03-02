#ifndef ISCC_H
#define ISCC_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

/**
 * Opaque FFI wrapper around `iscc_lib::DataHasher`.
 *
 * Enforces finalize-once semantics via `Option<Inner>`. Not `#[repr(C)]` —
 * C callers interact only through function pointers.
 */
typedef struct iscc_FfiDataHasher iscc_FfiDataHasher;

/**
 * Opaque FFI wrapper around `iscc_lib::InstanceHasher`.
 *
 * Enforces finalize-once semantics via `Option<Inner>`. Not `#[repr(C)]` —
 * C callers interact only through function pointers.
 */
typedef struct iscc_FfiInstanceHasher iscc_FfiInstanceHasher;

/**
 * Result of `gen_sum_code_v0` — composite Data+Instance ISCC-CODE with file metadata.
 *
 * On success, `ok` is `true` and all fields are populated.
 * On error, `ok` is `false`, string pointers are `NULL`, and `filesize` is 0.
 * Caller must free with `iscc_free_sum_code_result()`.
 */
typedef struct iscc_IsccSumCodeResult {
  /**
   * Whether the operation succeeded.
   */
  bool ok;
  /**
   * Composite ISCC-CODE string (heap-allocated, caller frees via `iscc_free_sum_code_result`).
   */
  char *iscc;
  /**
   * Hex-encoded BLAKE3 multihash of the file (heap-allocated).
   */
  char *datahash;
  /**
   * Byte length of the file.
   */
  uint64_t filesize;
} iscc_IsccSumCodeResult;

/**
 * Heap-allocated byte buffer returned to C callers.
 *
 * On success, `data` points to a contiguous byte array of `len` bytes.
 * On error, `data` is `NULL` and `len` is 0. Callers must free with
 * `iscc_free_byte_buffer()`.
 */
typedef struct iscc_IsccByteBuffer {
  /**
   * Pointer to the byte data (`NULL` if error).
   */
  uint8_t *data;
  /**
   * Number of bytes.
   */
  uintptr_t len;
} iscc_IsccByteBuffer;

/**
 * Result of decoding an ISCC unit string.
 *
 * On success, `ok` is `true` and all fields are populated.
 * On error, `ok` is `false` and all fields are zeroed (check `iscc_last_error()`).
 * Caller must free with `iscc_free_decode_result()`.
 */
typedef struct iscc_IsccDecodeResult {
  /**
   * Whether the decode succeeded.
   */
  bool ok;
  /**
   * MainType enum value (0–7).
   */
  uint8_t maintype;
  /**
   * SubType enum value (0–7).
   */
  uint8_t subtype;
  /**
   * Version enum value.
   */
  uint8_t version;
  /**
   * Length index from the header.
   */
  uint8_t length;
  /**
   * Raw digest bytes truncated to the encoded bit-length.
   */
  struct iscc_IsccByteBuffer digest;
} iscc_IsccDecodeResult;

/**
 * Array of byte buffers (for `iscc_alg_cdc_chunks`).
 *
 * Contains `count` elements pointed to by `buffers`. Callers must free
 * with `iscc_free_byte_buffer_array()`.
 */
typedef struct iscc_IsccByteBufferArray {
  /**
   * Pointer to array of `IsccByteBuffer` elements.
   */
  struct iscc_IsccByteBuffer *buffers;
  /**
   * Number of buffers.
   */
  uintptr_t count;
} iscc_IsccByteBufferArray;

/**
 * Maximum byte length for the name field after trimming.
 */
 uint32_t iscc_meta_trim_name(void);

/**
 * Maximum byte length for the description field after trimming.
 */
 uint32_t iscc_meta_trim_description(void);

/**
 * Maximum byte length for the meta field payload after decoding.
 */
 uint32_t iscc_meta_trim_meta(void);

/**
 * Default read buffer size for streaming I/O (4 MB).
 */
 uint32_t iscc_io_read_size(void);

/**
 * Sliding window width for text n-gram generation.
 */
 uint32_t iscc_text_ngram_size(void);

/**
 * Allocate `size` bytes of WASM-side memory.
 *
 * Returns a pointer the host can write into. The host must call
 * `iscc_dealloc` to free this memory. Returns a dangling (non-null)
 * pointer for `size == 0`.
 *
 * # Safety
 *
 * The caller must ensure `size` bytes are actually needed and must free
 * the allocation with `iscc_dealloc(ptr, size)`.
 */
 uint8_t *iscc_alloc(uintptr_t size);

/**
 * Free `size` bytes of WASM-side memory at `ptr`, previously allocated
 * by `iscc_alloc`.
 *
 * No-op if `ptr` is null. Passing `size == 0` is a no-op because
 * `iscc_alloc(0)` returns a dangling pointer that was never allocated.
 *
 * # Safety
 *
 * - `ptr` must have been returned by `iscc_alloc(size)` with the same `size`.
 * - Each allocation must be freed exactly once.
 */
 void iscc_dealloc(uint8_t *ptr, uintptr_t size);

/**
 * Generate a Meta-Code from name and optional metadata.
 *
 * # Parameters
 *
 * - `name`: required, null-terminated UTF-8 string
 * - `description`: optional (NULL means not provided)
 * - `meta`: optional (NULL means not provided)
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * All non-null pointer arguments must point to valid null-terminated UTF-8 strings.
 */

char *iscc_gen_meta_code_v0(const char *name,
                            const char *description,
                            const char *meta,
                            uint32_t bits);

/**
 * Generate a Text-Code from plain text content.
 *
 * # Parameters
 *
 * - `text`: required, null-terminated UTF-8 string
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `text` must point to a valid null-terminated UTF-8 string.
 */
 char *iscc_gen_text_code_v0(const char *text, uint32_t bits);

/**
 * Generate an Image-Code from pixel data.
 *
 * # Parameters
 *
 * - `pixels`: pointer to 1024 grayscale pixel values (32x32)
 * - `pixels_len`: number of bytes in the pixel buffer
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `pixels` must point to a valid buffer of at least `pixels_len` bytes.
 */
 char *iscc_gen_image_code_v0(const uint8_t *pixels, uintptr_t pixels_len, uint32_t bits);

/**
 * Generate an Audio-Code from a Chromaprint feature vector.
 *
 * # Parameters
 *
 * - `cv`: pointer to signed 32-bit integer Chromaprint features
 * - `cv_len`: number of elements in the feature vector
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `cv` must point to a valid buffer of at least `cv_len` `i32` elements.
 */
 char *iscc_gen_audio_code_v0(const int32_t *cv, uintptr_t cv_len, uint32_t bits);

/**
 * Generate a Video-Code from frame signature data.
 *
 * # Parameters
 *
 * - `frame_sigs`: array of pointers to frame signature arrays
 * - `frame_lens`: array of lengths for each frame signature
 * - `num_frames`: number of frames
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * - `frame_sigs` must point to an array of `num_frames` valid pointers
 * - `frame_lens` must point to an array of `num_frames` lengths
 * - Each `frame_sigs[i]` must be valid for `frame_lens[i]` `i32` elements
 */

char *iscc_gen_video_code_v0(const int32_t *const *frame_sigs,
                             const uintptr_t *frame_lens,
                             uintptr_t num_frames,
                             uint32_t bits);

/**
 * Generate a Mixed-Code from multiple Content-Code strings.
 *
 * # Parameters
 *
 * - `codes`: array of pointers to null-terminated ISCC code strings
 * - `num_codes`: number of code strings
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `codes` must point to an array of `num_codes` valid null-terminated UTF-8 strings.
 */
 char *iscc_gen_mixed_code_v0(const char *const *codes, uintptr_t num_codes, uint32_t bits);

/**
 * Generate a Data-Code from raw byte data.
 *
 * # Parameters
 *
 * - `data`: pointer to raw byte data
 * - `data_len`: number of bytes
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `data` must point to a valid buffer of at least `data_len` bytes.
 */
 char *iscc_gen_data_code_v0(const uint8_t *data, uintptr_t data_len, uint32_t bits);

/**
 * Generate an Instance-Code from raw byte data.
 *
 * # Parameters
 *
 * - `data`: pointer to raw byte data
 * - `data_len`: number of bytes
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `data` must point to a valid buffer of at least `data_len` bytes.
 */
 char *iscc_gen_instance_code_v0(const uint8_t *data, uintptr_t data_len, uint32_t bits);

/**
 * Generate a composite ISCC-CODE from individual unit codes.
 *
 * # Parameters
 *
 * - `codes`: array of pointers to null-terminated ISCC unit code strings
 * - `num_codes`: number of code strings
 * - `wide`: if true and exactly 2 codes (Data+Instance) with 128+ bits, produce wide output
 *
 * # Returns
 *
 * Heap-allocated ISCC string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `codes` must point to an array of `num_codes` valid null-terminated UTF-8 strings.
 */
 char *iscc_gen_iscc_code_v0(const char *const *codes, uintptr_t num_codes, bool wide);

/**
 * Clean and normalize text for display.
 *
 * Applies NFKC normalization, removes control characters (except newlines),
 * normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
 * leading/trailing whitespace.
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `text` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char *iscc_text_clean(const char *text);

/**
 * Remove newlines and collapse whitespace to single spaces.
 *
 * Converts multi-line text into a single normalized line.
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `text` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char *iscc_text_remove_newlines(const char *text);

/**
 * Trim text so its UTF-8 encoded size does not exceed `nbytes`.
 *
 * Multi-byte characters that would be split are dropped entirely.
 * Leading/trailing whitespace is stripped from the result.
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `text` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char *iscc_text_trim(const char *text, uintptr_t nbytes);

/**
 * Normalize and simplify text for similarity hashing.
 *
 * Applies NFD normalization, lowercasing, removes whitespace and characters
 * in Unicode categories C (control), M (mark), and P (punctuation), then
 * recombines with NFKC normalization.
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `text` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char *iscc_text_collapse(const char *text);

/**
 * Encode bytes as base64url (RFC 4648 section 5, no padding).
 *
 * # Parameters
 *
 * - `data`: pointer to raw byte data
 * - `data_len`: number of bytes
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `data` must point to a valid buffer of at least `data_len` bytes.
 */
 char *iscc_encode_base64(const uint8_t *data, uintptr_t data_len);

/**
 * Convert a JSON string into a `data:` URL with JCS canonicalization.
 *
 * Uses `application/ld+json` media type when the JSON contains an `@context`
 * key, otherwise `application/json`.
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * `json` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char *iscc_json_to_data_url(const char *json);

/**
 * Encode raw ISCC header components and digest into a base32 ISCC unit string.
 *
 * Takes integer type identifiers and a raw digest, returns a base32-encoded
 * ISCC unit string (without "ISCC:" prefix).
 *
 * # Parameters
 *
 * - `mtype`: MainType enum value (0–7)
 * - `stype`: SubType enum value (0–7)
 * - `version`: Version enum value
 * - `bit_length`: bit length of the digest (multiple of 32)
 * - `digest`: pointer to raw digest bytes
 * - `digest_len`: number of bytes in the digest buffer
 *
 * # Returns
 *
 * Heap-allocated C string on success, `NULL` on error.
 * Caller must free with `iscc_free_string()`.
 *
 * # Safety
 *
 * If `digest_len > 0`, `digest` must point to a valid buffer of at least
 * `digest_len` bytes.
 */

char *iscc_encode_component(uint8_t mtype,
                            uint8_t stype,
                            uint8_t version,
                            uint32_t bit_length,
                            const uint8_t *digest,
                            uintptr_t digest_len);

/**
 * Generate a composite ISCC-CODE from a file path (Data-Code + Instance-Code).
 *
 * Single-pass file I/O feeds both DataHasher and InstanceHasher, then composes
 * the ISCC-CODE internally.
 *
 * # Parameters
 *
 * - `path`: null-terminated UTF-8 file path
 * - `bits`: hash bit length (typically 64)
 * - `wide`: if true, use 256-bit combination for ISCC-CODE
 *
 * # Returns
 *
 * `IsccSumCodeResult` struct. Check `ok` to determine success.
 * Caller must free with `iscc_free_sum_code_result()`.
 *
 * # Safety
 *
 * `path` must point to a valid null-terminated UTF-8 string, or be null.
 */
 struct iscc_IsccSumCodeResult iscc_gen_sum_code_v0(const char *path, uint32_t bits, bool wide);

/**
 * Free an `IsccSumCodeResult` previously returned by `iscc_gen_sum_code_v0`.
 *
 * Releases the `iscc` and `datahash` strings. No-op for NULL pointers.
 *
 * # Safety
 *
 * `result` must be a value returned by `iscc_gen_sum_code_v0`.
 * Each result must only be freed once.
 */
 void iscc_free_sum_code_result(struct iscc_IsccSumCodeResult result);

/**
 * Decode an ISCC unit string into header components and raw digest.
 *
 * Returns an `IsccDecodeResult` struct. Check `ok` to determine success.
 * Strips an optional "ISCC:" prefix before decoding.
 *
 * # Returns
 *
 * `IsccDecodeResult` with decoded fields. On error, `ok` is `false`.
 * Caller must free with `iscc_free_decode_result()`.
 *
 * # Safety
 *
 * `iscc` must point to a valid null-terminated UTF-8 string, or be null.
 */
 struct iscc_IsccDecodeResult iscc_decode(const char *iscc);

/**
 * Free an `IsccDecodeResult` previously returned by `iscc_decode`.
 *
 * Releases the digest buffer if non-null. No-op if digest is already null.
 *
 * # Safety
 *
 * `result` must be a value returned by `iscc_decode`.
 * Each result must only be freed once.
 */
 void iscc_free_decode_result(struct iscc_IsccDecodeResult result);

/**
 * Run all conformance tests against vendored test vectors.
 *
 * Returns `true` if all tests pass, `false` if any fail.
 */
 bool iscc_conformance_selftest(void);

/**
 * Decompose a composite ISCC-CODE into individual ISCC-UNITs.
 *
 * Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
 * The optional "ISCC:" prefix is stripped before decoding.
 * Returns a NULL-terminated array of heap-allocated C strings.
 *
 * # Returns
 *
 * NULL-terminated array on success, `NULL` on error (check `iscc_last_error()`).
 * Caller must free with `iscc_free_string_array()`.
 *
 * # Safety
 *
 * `iscc_code` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char **iscc_decompose(const char *iscc_code);

/**
 * Generate sliding window n-grams from a string.
 *
 * Returns a NULL-terminated array of overlapping substrings of `width`
 * Unicode characters, advancing by one character at a time.
 *
 * # Returns
 *
 * NULL-terminated array on success, `NULL` on error (check `iscc_last_error()`).
 * Caller must free with `iscc_free_string_array()`.
 *
 * # Safety
 *
 * `seq` must point to a valid null-terminated UTF-8 string, or be null.
 */
 char **iscc_sliding_window(const char *seq, uint32_t width);

/**
 * Compute a SimHash digest from an array of byte digests.
 *
 * The output length matches the input digest length (e.g., 4-byte digests
 * produce a 4-byte SimHash). Returns an empty 32-byte buffer for empty input.
 *
 * # Parameters
 *
 * - `digests`: array of pointers to digest byte arrays
 * - `digest_lens`: array of lengths for each digest
 * - `num_digests`: number of digests
 *
 * # Returns
 *
 * `IsccByteBuffer` with the SimHash result. On error, `.data` is `NULL`.
 * Caller must free with `iscc_free_byte_buffer()`.
 *
 * # Safety
 *
 * - `digests` must point to an array of `num_digests` valid byte pointers
 * - `digest_lens` must point to an array of `num_digests` lengths
 * - Each `digests[i]` must be valid for `digest_lens[i]` bytes
 */

struct iscc_IsccByteBuffer iscc_alg_simhash(const uint8_t *const *digests,
                                            const uintptr_t *digest_lens,
                                            uintptr_t num_digests);

/**
 * Compute a 256-bit MinHash digest from 32-bit integer features.
 *
 * Uses 64 universal hash functions with bit-interleaved compression to
 * produce a 32-byte similarity-preserving digest.
 *
 * # Parameters
 *
 * - `features`: pointer to `u32` feature values
 * - `features_len`: number of features
 *
 * # Returns
 *
 * `IsccByteBuffer` with 32 bytes. On error, `.data` is `NULL`.
 * Caller must free with `iscc_free_byte_buffer()`.
 *
 * # Safety
 *
 * `features` must point to a valid buffer of at least `features_len` `u32` elements.
 */
 struct iscc_IsccByteBuffer iscc_alg_minhash_256(const uint32_t *features, uintptr_t features_len);

/**
 * Split data into content-defined chunks using gear rolling hash.
 *
 * Returns at least one chunk (empty bytes for empty input). When `utf32`
 * is true, aligns cut points to 4-byte boundaries.
 *
 * # Parameters
 *
 * - `data`: pointer to raw byte data
 * - `data_len`: number of bytes
 * - `utf32`: if true, align cuts to 4-byte boundaries
 * - `avg_chunk_size`: target average chunk size in bytes
 *
 * # Returns
 *
 * `IsccByteBufferArray` with chunks. On error, `.buffers` is `NULL`.
 * Caller must free with `iscc_free_byte_buffer_array()`.
 *
 * # Safety
 *
 * `data` must point to a valid buffer of at least `data_len` bytes.
 */

struct iscc_IsccByteBufferArray iscc_alg_cdc_chunks(const uint8_t *data,
                                                    uintptr_t data_len,
                                                    bool utf32,
                                                    uint32_t avg_chunk_size);

/**
 * Compute a similarity-preserving hash from video frame signatures.
 *
 * Returns raw bytes of length `bits / 8`. Errors if `frame_sigs` is empty.
 *
 * # Parameters
 *
 * - `frame_sigs`: array of pointers to frame signature arrays (`i32`)
 * - `frame_lens`: array of lengths for each frame signature
 * - `num_frames`: number of frames
 * - `bits`: hash bit length (typically 64)
 *
 * # Returns
 *
 * `IsccByteBuffer` with the hash result. On error, `.data` is `NULL`.
 * Caller must free with `iscc_free_byte_buffer()`.
 *
 * # Safety
 *
 * - `frame_sigs` must point to an array of `num_frames` valid pointers
 * - `frame_lens` must point to an array of `num_frames` lengths
 * - Each `frame_sigs[i]` must be valid for `frame_lens[i]` `i32` elements
 */

struct iscc_IsccByteBuffer iscc_soft_hash_video_v0(const int32_t *const *frame_sigs,
                                                   const uintptr_t *frame_lens,
                                                   uintptr_t num_frames,
                                                   uint32_t bits);

/**
 * Create a new streaming Data-Code hasher.
 *
 * Returns an opaque pointer. The caller must eventually call
 * `iscc_data_hasher_free()` to release the memory.
 */
 struct iscc_FfiDataHasher *iscc_data_hasher_new(void);

/**
 * Push data into a streaming DataHasher.
 *
 * Returns `true` on success, `false` on error (e.g., already finalized
 * or NULL pointer). Check `iscc_last_error()` for the error message.
 *
 * # Safety
 *
 * - `hasher` must be a valid pointer from `iscc_data_hasher_new()`, or NULL.
 * - `data` must point to at least `data_len` valid bytes.
 */

bool iscc_data_hasher_update(struct iscc_FfiDataHasher *hasher,
                             const uint8_t *data,
                             uintptr_t data_len);

/**
 * Finalize a streaming DataHasher and return an ISCC string.
 *
 * Consumes the inner hasher state. After this call, subsequent `update`
 * or `finalize` calls will fail. The caller must still call
 * `iscc_data_hasher_free()` to release the wrapper, and `iscc_free_string()`
 * to release the returned string.
 *
 * # Safety
 *
 * `hasher` must be a valid pointer from `iscc_data_hasher_new()`, or NULL.
 */
 char *iscc_data_hasher_finalize(struct iscc_FfiDataHasher *hasher, uint32_t bits);

/**
 * Free a DataHasher previously created by `iscc_data_hasher_new()`.
 *
 * NULL is a no-op. Each pointer must be freed exactly once.
 *
 * # Safety
 *
 * `hasher` must be a valid pointer from `iscc_data_hasher_new()`, or NULL.
 */
 void iscc_data_hasher_free(struct iscc_FfiDataHasher *hasher);

/**
 * Create a new streaming Instance-Code hasher.
 *
 * Returns an opaque pointer. The caller must eventually call
 * `iscc_instance_hasher_free()` to release the memory.
 */
 struct iscc_FfiInstanceHasher *iscc_instance_hasher_new(void);

/**
 * Push data into a streaming InstanceHasher.
 *
 * Returns `true` on success, `false` on error (e.g., already finalized
 * or NULL pointer). Check `iscc_last_error()` for the error message.
 *
 * # Safety
 *
 * - `hasher` must be a valid pointer from `iscc_instance_hasher_new()`, or NULL.
 * - `data` must point to at least `data_len` valid bytes.
 */

bool iscc_instance_hasher_update(struct iscc_FfiInstanceHasher *hasher,
                                 const uint8_t *data,
                                 uintptr_t data_len);

/**
 * Finalize a streaming InstanceHasher and return an ISCC string.
 *
 * Consumes the inner hasher state. After this call, subsequent `update`
 * or `finalize` calls will fail. The caller must still call
 * `iscc_instance_hasher_free()` to release the wrapper, and
 * `iscc_free_string()` to release the returned string.
 *
 * # Safety
 *
 * `hasher` must be a valid pointer from `iscc_instance_hasher_new()`, or NULL.
 */
 char *iscc_instance_hasher_finalize(struct iscc_FfiInstanceHasher *hasher, uint32_t bits);

/**
 * Free an InstanceHasher previously created by `iscc_instance_hasher_new()`.
 *
 * NULL is a no-op. Each pointer must be freed exactly once.
 *
 * # Safety
 *
 * `hasher` must be a valid pointer from `iscc_instance_hasher_new()`, or NULL.
 */
 void iscc_instance_hasher_free(struct iscc_FfiInstanceHasher *hasher);

/**
 * Free a string previously returned by any `iscc_gen_*` function.
 *
 * # Safety
 *
 * `ptr` must be a pointer returned by one of the `iscc_gen_*_v0` functions,
 * or `NULL` (which is a no-op). Each pointer must only be freed once.
 */
 void iscc_free_string(char *ptr);

/**
 * Free a NULL-terminated string array returned by `iscc_decompose` or
 * `iscc_sliding_window`.
 *
 * Walks the array, freeing each string via `CString::from_raw`, then frees
 * the array itself. `NULL` is a no-op.
 *
 * # Safety
 *
 * `arr` must be a pointer returned by `iscc_decompose` or `iscc_sliding_window`,
 * or `NULL`. Each array must only be freed once.
 */
 void iscc_free_string_array(char **arr);

/**
 * Free a byte buffer returned by `iscc_alg_simhash`, `iscc_alg_minhash_256`,
 * or `iscc_soft_hash_video_v0`.
 *
 * No-op if `buf.data` is `NULL`.
 *
 * # Safety
 *
 * `buf` must be a value returned by one of the algorithm primitive functions.
 * Each buffer must only be freed once.
 */
 void iscc_free_byte_buffer(struct iscc_IsccByteBuffer buf);

/**
 * Free a byte buffer array returned by `iscc_alg_cdc_chunks`.
 *
 * Frees each buffer's data, then the array itself. No-op if `arr.buffers`
 * is `NULL`.
 *
 * # Safety
 *
 * `arr` must be a value returned by `iscc_alg_cdc_chunks`.
 * Each array must only be freed once.
 */
 void iscc_free_byte_buffer_array(struct iscc_IsccByteBufferArray arr);

/**
 * Return the last error message from the current thread.
 *
 * Returns a pointer to a null-terminated string valid until the next
 * `iscc_gen_*` call on the same thread. Returns `NULL` if no error
 * has occurred.
 *
 * The returned pointer must NOT be freed by the caller.
 */
 const char *iscc_last_error(void);

#endif  /* ISCC_H */
