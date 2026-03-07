/// @file iscc.hpp
/// @brief Idiomatic C++17 header-only wrapper over the ISCC C FFI.
///
/// Provides RAII resource management, std::string / std::vector<uint8_t> types,
/// and exception-based error handling. All functions live in namespace iscc.
/// Wraps every symbol from iscc.h with zero runtime overhead.
///
/// Usage:
///   #include <iscc/iscc.hpp>
///   auto result = iscc::gen_meta_code_v0("Title");
///   std::cout << result.iscc << "\n";

#pragma once

extern "C" {
#include "iscc.h"
}

#include <cstdint>
#include <optional>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace iscc {

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

/// Exception thrown when a C FFI call fails.
class IsccError : public std::runtime_error {
public:
    explicit IsccError(const std::string& msg) : std::runtime_error(msg) {}
};

namespace detail {

/// Return a non-null pointer for empty byte vectors.
/// Some C FFI functions require non-null data pointers even for zero length.
inline const uint8_t* safe_data(const std::vector<uint8_t>& v) {
    static const uint8_t sentinel = 0;
    return v.empty() ? &sentinel : v.data();
}

/// Return a non-null pointer for empty int32 vectors.
/// Some C FFI functions require non-null data pointers even for zero length.
inline const int32_t* safe_data(const std::vector<int32_t>& v) {
    static const int32_t sentinel = 0;
    return v.empty() ? &sentinel : v.data();
}

/// Check the thread-local error from the C FFI and throw if set.
inline void check_error() {
    const char* err = iscc_last_error();
    if (err) {
        throw IsccError(err);
    }
}

/// Check that a pointer returned by a C FFI call is non-null.
inline void check_ptr(const void* ptr) {
    if (!ptr) {
        check_error();
        // Fallback if no error message was set
        throw IsccError("ISCC FFI returned null");
    }
}

// ---------------------------------------------------------------------------
// RAII guards for C FFI resources
// ---------------------------------------------------------------------------

/// RAII guard for a heap-allocated C string (char*).
struct UniqueString {
    char* ptr;
    explicit UniqueString(char* p) : ptr(p) {}
    ~UniqueString() { if (ptr) iscc_free_string(ptr); }
    UniqueString(const UniqueString&) = delete;
    UniqueString& operator=(const UniqueString&) = delete;
    UniqueString(UniqueString&& o) noexcept : ptr(std::exchange(o.ptr, nullptr)) {}

    /// Convert to std::string and return.
    std::string to_string() const { return ptr ? std::string(ptr) : std::string(); }
};

/// RAII guard for a NULL-terminated string array (char**).
struct UniqueStringArray {
    char** ptr;
    explicit UniqueStringArray(char** p) : ptr(p) {}
    ~UniqueStringArray() { if (ptr) iscc_free_string_array(ptr); }
    UniqueStringArray(const UniqueStringArray&) = delete;
    UniqueStringArray& operator=(const UniqueStringArray&) = delete;
    UniqueStringArray(UniqueStringArray&& o) noexcept : ptr(std::exchange(o.ptr, nullptr)) {}

    /// Convert to vector of strings.
    std::vector<std::string> to_vec() const {
        std::vector<std::string> result;
        if (ptr) {
            for (char** p = ptr; *p != nullptr; ++p) {
                result.emplace_back(*p);
            }
        }
        return result;
    }
};

/// RAII guard for an IsccByteBuffer.
struct UniqueByteBuffer {
    iscc_IsccByteBuffer buf;
    explicit UniqueByteBuffer(iscc_IsccByteBuffer b) : buf(b) {}
    ~UniqueByteBuffer() { if (buf.data) iscc_free_byte_buffer(buf); }
    UniqueByteBuffer(const UniqueByteBuffer&) = delete;
    UniqueByteBuffer& operator=(const UniqueByteBuffer&) = delete;
    UniqueByteBuffer(UniqueByteBuffer&& o) noexcept : buf(o.buf) {
        o.buf.data = nullptr;
        o.buf.len = 0;
    }

    /// Copy data to a vector.
    std::vector<uint8_t> to_vec() const {
        if (buf.data && buf.len > 0) {
            return std::vector<uint8_t>(buf.data, buf.data + buf.len);
        }
        return {};
    }
};

/// RAII guard for an IsccByteBufferArray.
struct UniqueByteBufferArray {
    iscc_IsccByteBufferArray arr;
    explicit UniqueByteBufferArray(iscc_IsccByteBufferArray a) : arr(a) {}
    ~UniqueByteBufferArray() { if (arr.buffers) iscc_free_byte_buffer_array(arr); }
    UniqueByteBufferArray(const UniqueByteBufferArray&) = delete;
    UniqueByteBufferArray& operator=(const UniqueByteBufferArray&) = delete;
    UniqueByteBufferArray(UniqueByteBufferArray&& o) noexcept : arr(o.arr) {
        o.arr.buffers = nullptr;
        o.arr.count = 0;
    }

    /// Convert to vector of byte vectors.
    std::vector<std::vector<uint8_t>> to_vec() const {
        std::vector<std::vector<uint8_t>> result;
        if (arr.buffers) {
            for (size_t i = 0; i < arr.count; ++i) {
                auto& b = arr.buffers[i];
                if (b.data && b.len > 0) {
                    result.emplace_back(b.data, b.data + b.len);
                } else {
                    result.emplace_back();
                }
            }
        }
        return result;
    }
};

} // namespace detail

// ---------------------------------------------------------------------------
// Result types
// ---------------------------------------------------------------------------

/// Result from gen_meta_code_v0.
struct MetaCodeResult {
    std::string iscc;
};

/// Result from gen_text_code_v0.
struct TextCodeResult {
    std::string iscc;
};

/// Result from gen_image_code_v0.
struct ImageCodeResult {
    std::string iscc;
};

/// Result from gen_audio_code_v0.
struct AudioCodeResult {
    std::string iscc;
};

/// Result from gen_video_code_v0.
struct VideoCodeResult {
    std::string iscc;
};

/// Result from gen_mixed_code_v0.
struct MixedCodeResult {
    std::string iscc;
};

/// Result from gen_data_code_v0.
struct DataCodeResult {
    std::string iscc;
};

/// Result from gen_instance_code_v0.
struct InstanceCodeResult {
    std::string iscc;
};

/// Result from gen_iscc_code_v0.
struct IsccCodeResult {
    std::string iscc;
};

/// Result from gen_sum_code_v0.
struct SumCodeResult {
    std::string iscc;
    std::string datahash;
    uint64_t filesize;
    std::vector<std::string> units;
};

/// Result from iscc_decode.
struct DecodeResult {
    uint8_t maintype;
    uint8_t subtype;
    uint8_t version;
    uint8_t length;
    std::vector<uint8_t> digest;
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum byte length for the name field after trimming.
inline uint32_t meta_trim_name() { return iscc_meta_trim_name(); }

/// Maximum byte length for the description field after trimming.
inline uint32_t meta_trim_description() { return iscc_meta_trim_description(); }

/// Maximum byte length for the meta field payload after decoding.
inline uint32_t meta_trim_meta() { return iscc_meta_trim_meta(); }

/// Default read buffer size for streaming I/O (4 MB).
inline uint32_t io_read_size() { return iscc_io_read_size(); }

/// Sliding window width for text n-gram generation.
inline uint32_t text_ngram_size() { return iscc_text_ngram_size(); }

// ---------------------------------------------------------------------------
// Text utilities
// ---------------------------------------------------------------------------

/// Clean and normalize text for display.
inline std::string text_clean(const std::string& text) {
    detail::UniqueString s(iscc_text_clean(text.c_str()));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

/// Remove newlines and collapse whitespace to single spaces.
inline std::string text_remove_newlines(const std::string& text) {
    detail::UniqueString s(iscc_text_remove_newlines(text.c_str()));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

/// Trim text so its UTF-8 encoded size does not exceed nbytes.
inline std::string text_trim(const std::string& text, size_t nbytes) {
    detail::UniqueString s(iscc_text_trim(text.c_str(), nbytes));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

/// Normalize and simplify text for similarity hashing.
inline std::string text_collapse(const std::string& text) {
    detail::UniqueString s(iscc_text_collapse(text.c_str()));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

// ---------------------------------------------------------------------------
// Encoding utilities
// ---------------------------------------------------------------------------

/// Encode bytes as base64url (RFC 4648 section 5, no padding).
inline std::string encode_base64(const std::vector<uint8_t>& data) {
    detail::UniqueString s(iscc_encode_base64(detail::safe_data(data), data.size()));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

/// Encode bytes as base64url from raw pointer.
inline std::string encode_base64(const uint8_t* data, size_t len) {
    detail::UniqueString s(iscc_encode_base64(data, len));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

/// Convert a JSON string into a data: URL with JCS canonicalization.
inline std::string json_to_data_url(const std::string& json) {
    detail::UniqueString s(iscc_json_to_data_url(json.c_str()));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

/// Encode raw ISCC header components and digest into a base32 ISCC unit string.
inline std::string encode_component(uint8_t mtype, uint8_t stype, uint8_t version,
                                    uint32_t bit_length,
                                    const std::vector<uint8_t>& digest) {
    detail::UniqueString s(iscc_encode_component(mtype, stype, version, bit_length,
                                                  digest.data(), digest.size()));
    detail::check_ptr(s.ptr);
    return s.to_string();
}

// ---------------------------------------------------------------------------
// Codec functions
// ---------------------------------------------------------------------------

/// Decode an ISCC unit string into header components and raw digest.
inline DecodeResult iscc_decode(const std::string& code) {
    iscc_IsccDecodeResult dr = ::iscc_decode(code.c_str());
    if (!dr.ok) {
        iscc_free_decode_result(dr);
        detail::check_error();
        throw IsccError("ISCC decode failed");
    }
    DecodeResult result;
    result.maintype = dr.maintype;
    result.subtype = dr.subtype;
    result.version = dr.version;
    result.length = dr.length;
    if (dr.digest.data && dr.digest.len > 0) {
        result.digest.assign(dr.digest.data, dr.digest.data + dr.digest.len);
    }
    iscc_free_decode_result(dr);
    return result;
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
inline std::vector<std::string> iscc_decompose(const std::string& iscc_code) {
    detail::UniqueStringArray arr(::iscc_decompose(iscc_code.c_str()));
    detail::check_ptr(arr.ptr);
    return arr.to_vec();
}

/// Generate sliding window n-grams from a string.
inline std::vector<std::string> sliding_window(const std::string& seq, uint32_t width) {
    detail::UniqueStringArray arr(::iscc_sliding_window(seq.c_str(), width));
    detail::check_ptr(arr.ptr);
    return arr.to_vec();
}

// ---------------------------------------------------------------------------
// Algorithm primitives
// ---------------------------------------------------------------------------

/// Compute a SimHash digest from an array of byte digests.
inline std::vector<uint8_t> alg_simhash(const std::vector<std::vector<uint8_t>>& digests) {
    std::vector<const uint8_t*> ptrs;
    std::vector<size_t> lens;
    ptrs.reserve(digests.size());
    lens.reserve(digests.size());
    for (const auto& d : digests) {
        ptrs.push_back(detail::safe_data(d));
        lens.push_back(d.size());
    }
    detail::UniqueByteBuffer buf(iscc_alg_simhash(ptrs.data(), lens.data(), digests.size()));
    if (!buf.buf.data) {
        detail::check_error();
        throw IsccError("alg_simhash failed");
    }
    return buf.to_vec();
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
inline std::vector<uint8_t> alg_minhash_256(const std::vector<uint32_t>& features) {
    detail::UniqueByteBuffer buf(iscc_alg_minhash_256(features.data(), features.size()));
    if (!buf.buf.data) {
        detail::check_error();
        throw IsccError("alg_minhash_256 failed");
    }
    return buf.to_vec();
}

/// Split data into content-defined chunks using gear rolling hash.
inline std::vector<std::vector<uint8_t>> alg_cdc_chunks(const std::vector<uint8_t>& data,
                                                         bool utf32,
                                                         uint32_t avg_chunk_size) {
    detail::UniqueByteBufferArray arr(
        iscc_alg_cdc_chunks(detail::safe_data(data), data.size(), utf32, avg_chunk_size));
    if (!arr.arr.buffers) {
        detail::check_error();
        throw IsccError("alg_cdc_chunks failed");
    }
    return arr.to_vec();
}

/// Split data into content-defined chunks from raw pointer.
inline std::vector<std::vector<uint8_t>> alg_cdc_chunks(const uint8_t* data, size_t len,
                                                         bool utf32,
                                                         uint32_t avg_chunk_size) {
    detail::UniqueByteBufferArray arr(iscc_alg_cdc_chunks(data, len, utf32, avg_chunk_size));
    if (!arr.arr.buffers) {
        detail::check_error();
        throw IsccError("alg_cdc_chunks failed");
    }
    return arr.to_vec();
}

/// Compute a similarity-preserving hash from video frame signatures.
inline std::vector<uint8_t> soft_hash_video_v0(
    const std::vector<std::vector<int32_t>>& frame_sigs, uint32_t bits) {
    std::vector<const int32_t*> ptrs;
    std::vector<size_t> lens;
    ptrs.reserve(frame_sigs.size());
    lens.reserve(frame_sigs.size());
    for (const auto& f : frame_sigs) {
        ptrs.push_back(detail::safe_data(f));
        lens.push_back(f.size());
    }
    detail::UniqueByteBuffer buf(
        iscc_soft_hash_video_v0(ptrs.data(), lens.data(), frame_sigs.size(), bits));
    if (!buf.buf.data) {
        detail::check_error();
        throw IsccError("soft_hash_video_v0 failed");
    }
    return buf.to_vec();
}

// ---------------------------------------------------------------------------
// Gen functions
// ---------------------------------------------------------------------------

/// Generate a Meta-Code from name and optional metadata.
inline MetaCodeResult gen_meta_code_v0(
    const std::string& name,
    const std::optional<std::string>& description = std::nullopt,
    const std::optional<std::string>& meta = std::nullopt,
    uint32_t bits = 64) {
    const char* desc_ptr = description ? description->c_str() : nullptr;
    const char* meta_ptr = meta ? meta->c_str() : nullptr;
    detail::UniqueString s(iscc_gen_meta_code_v0(name.c_str(), desc_ptr, meta_ptr, bits));
    detail::check_ptr(s.ptr);
    return MetaCodeResult{s.to_string()};
}

/// Generate a Text-Code from plain text content.
inline TextCodeResult gen_text_code_v0(const std::string& text, uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_text_code_v0(text.c_str(), bits));
    detail::check_ptr(s.ptr);
    return TextCodeResult{s.to_string()};
}

/// Generate an Image-Code from pixel data.
inline ImageCodeResult gen_image_code_v0(const std::vector<uint8_t>& pixels,
                                          uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_image_code_v0(detail::safe_data(pixels),
                                                   pixels.size(), bits));
    detail::check_ptr(s.ptr);
    return ImageCodeResult{s.to_string()};
}

/// Generate an Image-Code from raw pixel pointer.
inline ImageCodeResult gen_image_code_v0(const uint8_t* pixels, size_t len,
                                          uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_image_code_v0(pixels, len, bits));
    detail::check_ptr(s.ptr);
    return ImageCodeResult{s.to_string()};
}

/// Generate an Audio-Code from a Chromaprint feature vector.
inline AudioCodeResult gen_audio_code_v0(const std::vector<int32_t>& cv,
                                          uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_audio_code_v0(cv.data(), cv.size(), bits));
    detail::check_ptr(s.ptr);
    return AudioCodeResult{s.to_string()};
}

/// Generate a Video-Code from frame signature data.
inline VideoCodeResult gen_video_code_v0(
    const std::vector<std::vector<int32_t>>& frame_sigs, uint32_t bits = 64) {
    std::vector<const int32_t*> ptrs;
    std::vector<size_t> lens;
    ptrs.reserve(frame_sigs.size());
    lens.reserve(frame_sigs.size());
    for (const auto& f : frame_sigs) {
        ptrs.push_back(detail::safe_data(f));
        lens.push_back(f.size());
    }
    detail::UniqueString s(
        iscc_gen_video_code_v0(ptrs.data(), lens.data(), frame_sigs.size(), bits));
    detail::check_ptr(s.ptr);
    return VideoCodeResult{s.to_string()};
}

/// Generate a Mixed-Code from multiple Content-Code strings.
inline MixedCodeResult gen_mixed_code_v0(const std::vector<std::string>& codes,
                                          uint32_t bits = 64) {
    std::vector<const char*> ptrs;
    ptrs.reserve(codes.size());
    for (const auto& c : codes) {
        ptrs.push_back(c.c_str());
    }
    detail::UniqueString s(iscc_gen_mixed_code_v0(ptrs.data(), codes.size(), bits));
    detail::check_ptr(s.ptr);
    return MixedCodeResult{s.to_string()};
}

/// Generate a Data-Code from raw byte data.
inline DataCodeResult gen_data_code_v0(const std::vector<uint8_t>& data,
                                        uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_data_code_v0(detail::safe_data(data), data.size(), bits));
    detail::check_ptr(s.ptr);
    return DataCodeResult{s.to_string()};
}

/// Generate a Data-Code from raw pointer.
inline DataCodeResult gen_data_code_v0(const uint8_t* data, size_t len,
                                        uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_data_code_v0(data, len, bits));
    detail::check_ptr(s.ptr);
    return DataCodeResult{s.to_string()};
}

/// Generate an Instance-Code from raw byte data.
inline InstanceCodeResult gen_instance_code_v0(const std::vector<uint8_t>& data,
                                                uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_instance_code_v0(detail::safe_data(data),
                                                      data.size(), bits));
    detail::check_ptr(s.ptr);
    return InstanceCodeResult{s.to_string()};
}

/// Generate an Instance-Code from raw pointer.
inline InstanceCodeResult gen_instance_code_v0(const uint8_t* data, size_t len,
                                                uint32_t bits = 64) {
    detail::UniqueString s(iscc_gen_instance_code_v0(data, len, bits));
    detail::check_ptr(s.ptr);
    return InstanceCodeResult{s.to_string()};
}

/// Generate a composite ISCC-CODE from individual unit codes.
inline IsccCodeResult gen_iscc_code_v0(const std::vector<std::string>& codes,
                                        bool wide = false) {
    std::vector<const char*> ptrs;
    ptrs.reserve(codes.size());
    for (const auto& c : codes) {
        ptrs.push_back(c.c_str());
    }
    detail::UniqueString s(iscc_gen_iscc_code_v0(ptrs.data(), codes.size(), wide));
    detail::check_ptr(s.ptr);
    return IsccCodeResult{s.to_string()};
}

/// Generate a composite ISCC-CODE from a file path (Data-Code + Instance-Code).
inline SumCodeResult gen_sum_code_v0(const std::string& path, uint32_t bits = 64,
                                      bool wide = false, bool add_units = false) {
    iscc_IsccSumCodeResult sr = iscc_gen_sum_code_v0(path.c_str(), bits, wide, add_units);
    if (!sr.ok) {
        iscc_free_sum_code_result(sr);
        detail::check_error();
        throw IsccError("gen_sum_code_v0 failed");
    }
    SumCodeResult result;
    result.iscc = sr.iscc ? std::string(sr.iscc) : std::string();
    result.datahash = sr.datahash ? std::string(sr.datahash) : std::string();
    result.filesize = sr.filesize;
    if (sr.units) {
        for (char** p = sr.units; *p != nullptr; ++p) {
            result.units.emplace_back(*p);
        }
    }
    iscc_free_sum_code_result(sr);
    return result;
}

// ---------------------------------------------------------------------------
// Conformance
// ---------------------------------------------------------------------------

/// Run all conformance tests against vendored test vectors.
inline bool conformance_selftest() {
    return iscc_conformance_selftest();
}

// ---------------------------------------------------------------------------
// Streaming types
// ---------------------------------------------------------------------------

/// RAII streaming Data-Code hasher.
class DataHasher {
    iscc_FfiDataHasher* handle_;

public:
    /// Create a new streaming Data-Code hasher.
    DataHasher() : handle_(iscc_data_hasher_new()) {
        detail::check_ptr(handle_);
    }

    ~DataHasher() {
        if (handle_) {
            iscc_data_hasher_free(handle_);
        }
    }

    // Move-only
    DataHasher(DataHasher&& o) noexcept : handle_(std::exchange(o.handle_, nullptr)) {}
    DataHasher& operator=(DataHasher&& o) noexcept {
        if (this != &o) {
            if (handle_) iscc_data_hasher_free(handle_);
            handle_ = std::exchange(o.handle_, nullptr);
        }
        return *this;
    }
    DataHasher(const DataHasher&) = delete;
    DataHasher& operator=(const DataHasher&) = delete;

    /// Push data into the hasher.
    void update(const uint8_t* data, size_t len) {
        bool ok = iscc_data_hasher_update(handle_, data, len);
        if (!ok) {
            detail::check_error();
            throw IsccError("DataHasher update failed");
        }
    }

    /// Push data into the hasher from a vector.
    void update(const std::vector<uint8_t>& data) {
        update(data.data(), data.size());
    }

    /// Finalize and return the Data-Code result.
    DataCodeResult finalize(uint32_t bits = 64) {
        detail::UniqueString s(iscc_data_hasher_finalize(handle_, bits));
        detail::check_ptr(s.ptr);
        return DataCodeResult{s.to_string()};
    }
};

/// RAII streaming Instance-Code hasher.
class InstanceHasher {
    iscc_FfiInstanceHasher* handle_;

public:
    /// Create a new streaming Instance-Code hasher.
    InstanceHasher() : handle_(iscc_instance_hasher_new()) {
        detail::check_ptr(handle_);
    }

    ~InstanceHasher() {
        if (handle_) {
            iscc_instance_hasher_free(handle_);
        }
    }

    // Move-only
    InstanceHasher(InstanceHasher&& o) noexcept : handle_(std::exchange(o.handle_, nullptr)) {}
    InstanceHasher& operator=(InstanceHasher&& o) noexcept {
        if (this != &o) {
            if (handle_) iscc_instance_hasher_free(handle_);
            handle_ = std::exchange(o.handle_, nullptr);
        }
        return *this;
    }
    InstanceHasher(const InstanceHasher&) = delete;
    InstanceHasher& operator=(const InstanceHasher&) = delete;

    /// Push data into the hasher.
    void update(const uint8_t* data, size_t len) {
        bool ok = iscc_instance_hasher_update(handle_, data, len);
        if (!ok) {
            detail::check_error();
            throw IsccError("InstanceHasher update failed");
        }
    }

    /// Push data into the hasher from a vector.
    void update(const std::vector<uint8_t>& data) {
        update(data.data(), data.size());
    }

    /// Finalize and return the Instance-Code result.
    InstanceCodeResult finalize(uint32_t bits = 64) {
        detail::UniqueString s(iscc_instance_hasher_finalize(handle_, bits));
        detail::check_ptr(s.ptr);
        return InstanceCodeResult{s.to_string()};
    }
};

} // namespace iscc
