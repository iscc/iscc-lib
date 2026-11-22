/// @file test_iscc.cpp
/// @brief Smoke tests for the iscc C++17 header-only wrapper.
///
/// Links against the iscc-ffi shared library and validates all wrapper
/// patterns against known expected values from the C test suite.
/// Exits 0 if all tests pass, non-zero on any failure.

#include <iscc/iscc.hpp>

#include "data_edge_vectors.h"

#include <cassert>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <string>
#include <vector>

static int tests_passed = 0;
static int tests_failed = 0;

/// Report a test pass.
static void pass(const char* name) {
    std::printf("PASS: %s\n", name);
    tests_passed++;
}

/// Report a test failure.
static void fail(const char* name, const char* detail) {
    std::printf("FAIL: %s — %s\n", name, detail);
    tests_failed++;
}

/// Assert two strings are equal.
static void assert_str_eq(const std::string& actual, const std::string& expected,
                           const char* name) {
    if (actual == expected) {
        pass(name);
    } else {
        std::string msg = "got \"" + actual + "\", expected \"" + expected + "\"";
        fail(name, msg.c_str());
    }
}

/// Assert a string starts with a given prefix.
static void assert_starts_with(const std::string& actual, const std::string& prefix,
                                const char* name) {
    if (actual.substr(0, prefix.size()) == prefix) {
        pass(name);
    } else {
        std::string msg = "got \"" + actual + "\", expected prefix \"" + prefix + "\"";
        fail(name, msg.c_str());
    }
}

/// Assert two size_t values are equal.
static void assert_eq(size_t actual, size_t expected, const char* name) {
    if (actual == expected) {
        pass(name);
    } else {
        char buf[256];
        std::snprintf(buf, sizeof(buf), "got %zu, expected %zu", actual, expected);
        fail(name, buf);
    }
}

/// Assert a boolean is true.
static void assert_true(bool val, const char* name) {
    if (val) {
        pass(name);
    } else {
        fail(name, "expected true, got false");
    }
}

/// Decode a lowercase hex string to bytes.
static std::vector<uint8_t> from_hex(const std::string& hex) {
    std::vector<uint8_t> out;
    for (size_t i = 0; i + 1 < hex.size(); i += 2) {
        out.push_back(static_cast<uint8_t>(std::stoul(hex.substr(i, 2), nullptr, 16)));
    }
    return out;
}

/// Assert that a callable throws IsccError whose message contains `needle`.
template <typename F>
static void assert_throws_with(F&& f, const std::string& needle, const char* name) {
    try {
        f();
    } catch (const iscc::IsccError& e) {
        std::string msg = e.what();
        if (msg.find(needle) != std::string::npos) {
            pass(name);
        } else {
            std::string detail = "message \"" + msg + "\" lacks \"" + needle + "\"";
            fail(name, detail.c_str());
        }
        return;
    }
    fail(name, "expected IsccError, nothing thrown");
}

/// Build a std::string from an edge-vector string, keeping any embedded NUL.
static std::string edge_string(const iscc_edge_str& s) {
    return s.ptr ? std::string(s.ptr, s.len) : std::string();
}

/// Build a string vector from an edge-vector list (an empty list stays empty).
static std::vector<std::string> edge_strings(const iscc_edge_list& list) {
    std::vector<std::string> out;
    for (size_t i = 0; i < list.count; ++i) {
        out.emplace_back(list.items[i], list.lens[i]);
    }
    return out;
}

/// Build a byte vector from an expected edge-vector byte string.
static std::vector<uint8_t> edge_bytes(const iscc_edge_str& s) {
    const std::string bytes = edge_string(s);
    return std::vector<uint8_t>(bytes.begin(), bytes.end());
}

/// Build one byte vector per item of an edge-vector digest list (an empty list stays empty).
static std::vector<std::vector<uint8_t>> edge_byte_lists(const iscc_edge_list& list) {
    std::vector<std::vector<uint8_t>> out;
    for (size_t i = 0; i < list.count; ++i) {
        const char* item = list.items[i];
        out.emplace_back(item, item + list.lens[i]);
    }
    return out;
}

/// Build one int32 vector per frame of an edge-vector frame list (an empty list stays empty).
static std::vector<std::vector<int32_t>> edge_frames(const iscc_edge_frames& list) {
    std::vector<std::vector<int32_t>> out;
    for (size_t i = 0; i < list.count; ++i) {
        const int32_t* frame = list.frames[i];
        out.emplace_back(frame, frame + list.lens[i]);
    }
    return out;
}

/// Check one iscc_decode edge row against its cpp expectation (rejection = IsccError).
static bool check_edge_iscc_decode(const iscc_edge_iscc_decode_vector& v) {
    const std::string input = edge_string(v.input);
    iscc::DecodeResult r;
    try {
        r = iscc::iscc_decode(input);
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && r.maintype == v.maintype && r.subtype == v.subtype &&
           r.version == v.version && r.length == v.length && r.digest == edge_bytes(v.digest);
}

/// Check one encode_seq edge row against its cpp expectation (rejection = IsccError).
static bool check_edge_encode_seq(const iscc_edge_encode_seq_vector& v) {
    const std::vector<std::string> units = edge_strings(v.units);
    std::vector<uint8_t> seq;
    try {
        seq = iscc::encode_seq(units);
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && seq == edge_bytes(v.seq);
}

/// Check one alg_simhash edge row against its cpp expectation (rejection = IsccError).
static bool check_edge_alg_simhash(const iscc_edge_alg_simhash_vector& v) {
    const std::vector<std::vector<uint8_t>> digests = edge_byte_lists(v.digests);
    std::vector<uint8_t> simhash;
    try {
        simhash = iscc::alg_simhash(digests);
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && simhash == edge_bytes(v.simhash);
}

/// Check one gen_text_code_v0 edge row against its cpp expectation.
static bool check_edge_gen_text_code_v0(const iscc_edge_gen_text_code_v0_vector& v) {
    const std::string text = edge_string(v.text);
    std::string iscc;
    try {
        iscc = iscc::gen_text_code_v0(text, v.bits).iscc;
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && iscc == edge_string(v.iscc);
}

/// Check one gen_mixed_code_v0 edge row against its cpp expectation.
static bool check_edge_gen_mixed_code_v0(const iscc_edge_gen_mixed_code_v0_vector& v) {
    const std::vector<std::string> codes = edge_strings(v.codes);
    std::string iscc;
    try {
        iscc = iscc::gen_mixed_code_v0(codes, v.bits).iscc;
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && iscc == edge_string(v.iscc);
}

/// Check one gen_iscc_code_v0 edge row against its cpp expectation.
static bool check_edge_gen_iscc_code_v0(const iscc_edge_gen_iscc_code_v0_vector& v) {
    const std::vector<std::string> codes = edge_strings(v.codes);
    std::string iscc;
    try {
        iscc = iscc::gen_iscc_code_v0(codes, v.wide != 0).iscc;
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && iscc == edge_string(v.iscc);
}

/// Check one gen_meta_code_v0 edge row against its cpp expectation.
/// An empty description is passed as std::nullopt (no description).
static bool check_edge_gen_meta_code_v0(const iscc_edge_gen_meta_code_v0_vector& v) {
    const std::string name = edge_string(v.input_name);
    const std::string description = edge_string(v.description);
    const std::optional<std::string> desc =
        description.empty() ? std::nullopt : std::optional<std::string>(description);
    const std::string meta = edge_string(v.meta);
    std::string iscc;
    try {
        iscc = iscc::gen_meta_code_v0(name, desc, meta, v.bits).iscc;
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && iscc == edge_string(v.iscc);
}

/// Check one gen_video_code_v0 edge row against its cpp expectation.
static bool check_edge_gen_video_code_v0(const iscc_edge_gen_video_code_v0_vector& v) {
    const std::vector<std::vector<int32_t>> frames = edge_frames(v.frame_sigs);
    std::string iscc;
    try {
        iscc = iscc::gen_video_code_v0(frames, v.bits).iscc;
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && iscc == edge_string(v.iscc);
}

/// Check one alg_cdc_chunks edge row against its cpp expectation (rejection = IsccError).
static bool check_edge_alg_cdc_chunks(const iscc_edge_alg_cdc_chunks_vector& v) {
    const std::vector<uint8_t> data = edge_bytes(v.data);
    std::vector<std::vector<uint8_t>> chunks;
    try {
        chunks = iscc::alg_cdc_chunks(data, v.utf32 != 0, v.avg_chunk_size);
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && chunks == edge_byte_lists(v.chunks);
}

/// Check one encode_component edge row against its cpp expectation (rejection = IsccError).
static bool check_edge_encode_component(const iscc_edge_encode_component_vector& v) {
    const std::vector<uint8_t> digest = edge_bytes(v.digest);
    std::string unit;
    try {
        unit = iscc::encode_component(v.mtype, v.stype, v.version, v.bit_length, digest);
    } catch (const iscc::IsccError&) {
        return v.cpp == ISCC_EDGE_ERROR;
    } catch (...) {
        return false;
    }
    return v.cpp == ISCC_EDGE_OK && unit == edge_string(v.unit);
}

/// Run every row of one data-edge section: skip rows marked cpp skip, check the rest,
/// print PASS/FAIL per row, and return the number of rows visited (run or skipped).
template <typename Row, typename Check>
static size_t run_edge_section(const char* section, const Row* rows, size_t count,
                               Check check) {
    size_t seen = 0;
    for (size_t i = 0; i < count; ++i) {
        ++seen;
        const std::string name = std::string("data_edge/") + section + "/" + rows[i].name;
        if (rows[i].cpp == ISCC_EDGE_SKIP) {
            std::printf("SKIP: %s\n", name.c_str());
            continue;
        }
        assert_true(check(rows[i]), name.c_str());
    }
    return seen;
}

int main() {
    // 1. conformance_selftest
    assert_true(iscc::conformance_selftest(), "conformance_selftest");

    // 2. gen_meta_code_v0 — name only
    {
        auto r = iscc::gen_meta_code_v0("Die Unendliche Geschichte");
        assert_str_eq(r.iscc, "ISCC:AAAZXZ6OU74YAZIM", "gen_meta_code_v0(name only)");
    }

    // 3. gen_meta_code_v0 — name + description
    {
        auto r = iscc::gen_meta_code_v0("Die Unendliche Geschichte",
                                         std::string("Von Michael Ende"));
        assert_str_eq(r.iscc, "ISCC:AAAZXZ6OU4E45RB5",
                      "gen_meta_code_v0(name + description)");
    }

    // 4. gen_text_code_v0
    {
        auto r = iscc::gen_text_code_v0("Hello World");
        assert_str_eq(r.iscc, "ISCC:EAASKDNZNYGUUF5A", "gen_text_code_v0");
    }

    // 5. gen_image_code_v0 — 1024 zero pixels
    {
        std::vector<uint8_t> pixels(1024, 0);
        auto r = iscc::gen_image_code_v0(pixels);
        assert_str_eq(r.iscc, "ISCC:EEAQAAAAAAAAAAAA", "gen_image_code_v0(zeros)");
    }

    // 6. gen_instance_code_v0 — empty data
    {
        std::vector<uint8_t> empty;
        auto r = iscc::gen_instance_code_v0(empty);
        assert_str_eq(r.iscc, "ISCC:IAA26E2JXH27TING", "gen_instance_code_v0(empty)");
    }

    // 7. gen_data_code_v0 — "Hello World"
    {
        std::vector<uint8_t> data(reinterpret_cast<const uint8_t*>("Hello World"),
                                  reinterpret_cast<const uint8_t*>("Hello World") + 11);
        auto r = iscc::gen_data_code_v0(data);
        assert_starts_with(r.iscc, "ISCC:", "gen_data_code_v0(Hello World)");
    }

    // 8. text_clean
    {
        auto cleaned = iscc::text_clean("  Hello   World  ");
        assert_str_eq(cleaned, "Hello   World", "text_clean");
    }

    // 9. text_collapse
    {
        auto collapsed = iscc::text_collapse("Hello, World!");
        assert_str_eq(collapsed, "helloworld", "text_collapse");
    }

    // 10. text_remove_newlines
    {
        auto result = iscc::text_remove_newlines("Hello\nWorld");
        assert_str_eq(result, "Hello World", "text_remove_newlines");
    }

    // 11. text_trim
    {
        auto trimmed = iscc::text_trim("Hello World", 5);
        assert_str_eq(trimmed, "Hello", "text_trim");
    }

    // 12. encode_base64
    {
        std::vector<uint8_t> data = {0x48, 0x65, 0x6c, 0x6c, 0x6f}; // "Hello"
        auto encoded = iscc::encode_base64(data);
        assert_str_eq(encoded, "SGVsbG8", "encode_base64");
    }

    // 13. DataHasher streaming
    {
        iscc::DataHasher dh;
        dh.update(reinterpret_cast<const uint8_t*>("Hello World"), 11);
        auto r = dh.finalize();
        assert_starts_with(r.iscc, "ISCC:", "DataHasher finalize");
    }

    // 14. DataHasher multi-update matches single update
    {
        iscc::DataHasher dh1;
        dh1.update(reinterpret_cast<const uint8_t*>("Hello World"), 11);
        auto r1 = dh1.finalize();

        iscc::DataHasher dh2;
        dh2.update(reinterpret_cast<const uint8_t*>("Hello"), 5);
        dh2.update(reinterpret_cast<const uint8_t*>(" World"), 6);
        auto r2 = dh2.finalize();

        assert_str_eq(r1.iscc, r2.iscc, "DataHasher multi-update matches single");
    }

    // 15. InstanceHasher — empty finalize
    {
        iscc::InstanceHasher ih;
        auto r = ih.finalize();
        assert_str_eq(r.iscc, "ISCC:IAA26E2JXH27TING", "InstanceHasher(empty)");
    }

    // 16. iscc_decode — known Meta-Code
    {
        auto dr = iscc::iscc_decode("AAAZXZ6OU74YAZIM");
        assert_eq(dr.maintype, 0, "iscc_decode maintype == 0");
        assert_eq(dr.subtype, 0, "iscc_decode subtype == 0");
        assert_eq(dr.version, 0, "iscc_decode version == 0");
        assert_eq(dr.length, 1, "iscc_decode length == 1");
        assert_eq(dr.digest.size(), 8, "iscc_decode digest len == 8");
    }

    // 17. iscc_decode — invalid input throws
    {
        bool threw = false;
        try {
            iscc::iscc_decode("INVALID");
        } catch (const iscc::IsccError&) {
            threw = true;
        }
        assert_true(threw, "iscc_decode(invalid) throws IsccError");
    }

    // 18. encode_component + decode roundtrip
    {
        std::vector<uint8_t> digest = {0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89};
        auto encoded = iscc::encode_component(0, 0, 0, 64, digest);
        auto dr = iscc::iscc_decode(encoded);
        assert_eq(dr.maintype, 0, "roundtrip: maintype == 0");
        assert_eq(dr.digest.size(), 8, "roundtrip: digest len == 8");
        bool match = (dr.digest == digest);
        assert_true(match, "roundtrip: digest matches");
    }

    // 19. sliding_window
    {
        auto ngrams = iscc::sliding_window("Hello", 3);
        assert_eq(ngrams.size(), 3, "sliding_window count == 3");
        if (ngrams.size() >= 3) {
            assert_str_eq(ngrams[0], "Hel", "sliding_window[0]");
            assert_str_eq(ngrams[1], "ell", "sliding_window[1]");
            assert_str_eq(ngrams[2], "llo", "sliding_window[2]");
        }
    }

    // 20. Constants
    assert_eq(iscc::meta_trim_name(), 128, "meta_trim_name == 128");
    assert_eq(iscc::meta_trim_description(), 4096, "meta_trim_description == 4096");
    assert_eq(iscc::meta_trim_meta(), 128000, "meta_trim_meta == 128000");
    assert_eq(iscc::io_read_size(), 4194304, "io_read_size == 4194304");
    assert_eq(iscc::text_ngram_size(), 13, "text_ngram_size == 13");

    // 21. alg_minhash_256
    {
        std::vector<uint32_t> features = {1, 2, 3, 4, 5};
        auto result = iscc::alg_minhash_256(features);
        assert_eq(result.size(), 32, "alg_minhash_256 len == 32");
    }

    // 22. alg_simhash
    {
        std::vector<std::vector<uint8_t>> digests = {{0xFF, 0x00, 0xFF, 0x00}};
        auto result = iscc::alg_simhash(digests);
        assert_eq(result.size(), 4, "alg_simhash len == 4");
    }

    // 23. alg_cdc_chunks
    {
        std::vector<uint8_t> data(reinterpret_cast<const uint8_t*>("Hello World"),
                                  reinterpret_cast<const uint8_t*>("Hello World") + 11);
        auto chunks = iscc::alg_cdc_chunks(data, false, 1024);
        assert_true(chunks.size() >= 1, "alg_cdc_chunks count >= 1");
        size_t total = 0;
        for (const auto& c : chunks) {
            total += c.size();
        }
        assert_eq(total, 11, "alg_cdc_chunks total bytes == 11");
    }

    // 24. soft_hash_video_v0
    {
        std::vector<int32_t> frame1(380);
        std::vector<int32_t> frame2(380);
        for (int i = 0; i < 380; ++i) {
            frame1[static_cast<size_t>(i)] = i;
            frame2[static_cast<size_t>(i)] = i + 1;
        }
        std::vector<std::vector<int32_t>> sigs = {frame1, frame2};
        auto result = iscc::soft_hash_video_v0(sigs, 64);
        assert_eq(result.size(), 8, "soft_hash_video_v0 len == 8");
    }

    // 25. json_to_data_url
    {
        auto result = iscc::json_to_data_url("{\"key\":\"value\"}");
        assert_starts_with(result, "data:application/json;base64,", "json_to_data_url prefix");
    }

    // 26. iscc_decompose
    {
        // First generate a known ISCC-CODE to decompose
        auto data_result = iscc::gen_data_code_v0(
            reinterpret_cast<const uint8_t*>("Hello World"), 11);
        auto inst_result = iscc::gen_instance_code_v0(
            reinterpret_cast<const uint8_t*>("Hello World"), 11);
        std::vector<std::string> codes = {data_result.iscc, inst_result.iscc};
        auto iscc_code = iscc::gen_iscc_code_v0(codes);
        auto units = iscc::iscc_decompose(iscc_code.iscc);
        assert_true(units.size() >= 2, "iscc_decompose returns >= 2 units");
    }

    // 27. gen_iscc_code_v0
    {
        auto data_result = iscc::gen_data_code_v0(
            reinterpret_cast<const uint8_t*>("Hello World"), 11);
        auto inst_result = iscc::gen_instance_code_v0(
            reinterpret_cast<const uint8_t*>("Hello World"), 11);
        std::vector<std::string> codes = {data_result.iscc, inst_result.iscc};
        auto r = iscc::gen_iscc_code_v0(codes);
        assert_starts_with(r.iscc, "ISCC:", "gen_iscc_code_v0");
    }

    // 28. gen_sum_code_v0 — temp file
    {
        auto tmppath = (std::filesystem::temp_directory_path() / "iscc_cpp_test_sum.bin").string();
        {
            std::ofstream f(tmppath, std::ios::binary);
            f.write("Hello World", 11);
        }
        auto r = iscc::gen_sum_code_v0(tmppath);
        assert_starts_with(r.iscc, "ISCC:", "gen_sum_code_v0 iscc");
        assert_true(!r.datahash.empty(), "gen_sum_code_v0 datahash not empty");
        assert_eq(r.filesize, 11, "gen_sum_code_v0 filesize == 11");
        assert_true(r.units.empty(), "gen_sum_code_v0 units empty when disabled");
        std::filesystem::remove(tmppath);
    }

    // 29. gen_sum_code_v0 — add_units=true
    {
        auto tmppath = (std::filesystem::temp_directory_path() / "iscc_cpp_test_sum_units.bin").string();
        {
            std::ofstream f(tmppath, std::ios::binary);
            f.write("Hello World", 11);
        }
        auto r = iscc::gen_sum_code_v0(tmppath, 64, false, true);
        assert_true(r.units.size() >= 2, "gen_sum_code_v0(units) has >= 2 units");
        if (r.units.size() >= 2) {
            assert_starts_with(r.units[0], "ISCC:", "gen_sum_code_v0 units[0]");
            assert_starts_with(r.units[1], "ISCC:", "gen_sum_code_v0 units[1]");
        }
        std::filesystem::remove(tmppath);
    }

    // 30. gen_sum_code_v0 — nonexistent path throws
    {
        bool threw = false;
        try {
            iscc::gen_sum_code_v0("/nonexistent/file.bin");
        } catch (const iscc::IsccError&) {
            threw = true;
        }
        assert_true(threw, "gen_sum_code_v0(nonexistent) throws IsccError");
    }

    // 31. gen_audio_code_v0
    {
        // Minimal Chromaprint feature vector (380 elements, matching iscc-core)
        std::vector<int32_t> cv(380);
        for (int i = 0; i < 380; ++i) {
            cv[static_cast<size_t>(i)] = i * 1000;
        }
        auto r = iscc::gen_audio_code_v0(cv);
        assert_starts_with(r.iscc, "ISCC:", "gen_audio_code_v0");
    }

    // 32. gen_video_code_v0
    {
        std::vector<int32_t> frame1(380);
        std::vector<int32_t> frame2(380);
        for (int i = 0; i < 380; ++i) {
            frame1[static_cast<size_t>(i)] = i;
            frame2[static_cast<size_t>(i)] = i + 1;
        }
        std::vector<std::vector<int32_t>> sigs = {frame1, frame2};
        auto r = iscc::gen_video_code_v0(sigs);
        assert_starts_with(r.iscc, "ISCC:", "gen_video_code_v0");
    }

    // 33. gen_mixed_code_v0
    {
        auto text_code = iscc::gen_text_code_v0("Hello World");
        std::vector<uint8_t> pixels(1024, 0);
        auto image_code = iscc::gen_image_code_v0(pixels);
        std::vector<std::string> codes = {text_code.iscc, image_code.iscc};
        auto r = iscc::gen_mixed_code_v0(codes);
        assert_starts_with(r.iscc, "ISCC:", "gen_mixed_code_v0");
    }

    // 34. DataHasher move semantics
    {
        iscc::DataHasher dh1;
        dh1.update(reinterpret_cast<const uint8_t*>("Hello"), 5);
        iscc::DataHasher dh2 = std::move(dh1);
        dh2.update(reinterpret_cast<const uint8_t*>(" World"), 6);
        auto r = dh2.finalize();
        assert_starts_with(r.iscc, "ISCC:", "DataHasher move semantics");
    }

    // 35. gen_audio_code_v0 with empty vector (regression: cv.data() NULL crash)
    {
        auto r = iscc::gen_audio_code_v0({});
        assert_str_eq(r.iscc, "ISCC:EIAQAAAAAAAAAAAA",
                       "gen_audio_code_v0 empty vector");
    }

    // 36. gen_iscc_id_v1 — golden vector
    {
        auto r = iscc::gen_iscc_id_v1(1751831876325218ULL, 1, 0);
        assert_str_eq(r.iscc, "ISCC:MAIGHFECJMOPMIAB", "gen_iscc_id_v1(golden)");
    }

    // 37. gen_iscc_id_v1 — invalid realm throws
    {
        bool threw = false;
        try {
            iscc::gen_iscc_id_v1(1751831876325218ULL, 1, 2);
        } catch (const iscc::IsccError&) {
            threw = true;
        }
        assert_true(threw, "gen_iscc_id_v1(realm=2) throws IsccError");
    }

    // 38. gen_iscc_id_v1 — decode round-trip reports IDv1
    {
        auto dr = iscc::iscc_decode("ISCC:MAIGHFECJMOPMIAB");
        assert_eq(dr.version, 1, "gen_iscc_id_v1 decode version == 1");
    }

    // 39. encode_seq — data.json encode_seq test_0001_four_units (full bytes)
    const std::string seq_four_hex =
        "0007578c7c8fbfbdb58f29f63efeb3871553faa75ff97458d49eff5ee4ddf36bb6c9"
        "2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305"
        "300705b41f3472493225d27edcbdb63236846a53ee28deca5e3561acc0c9bbdf81b5"
        "4007a522af4a2c9cbf6a808924afd2acfa0793347b936678f5982e22fdd3dd4a0e37";
    const std::vector<std::string> seq_four_units = {
        "ISCC:AADVPDD4R6733NMPFH3D57VTQ4KVH6VHL74XIWGUT37V5ZG56NV3NSI",
        "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI",
        "ISCC:GADQLNA7GRZESMRF2J7NZPNWGI3II2ST5YUN5SS6GVQ2ZQGJXPPYDNI",
        "ISCC:IAD2KIVPJIWJZP3KQCESJL6SVT5APEZUPOJWM6HVTAXCF7OT3VFA4NY",
    };
    {
        auto seq = iscc::encode_seq(seq_four_units);
        assert_true(seq == from_hex(seq_four_hex), "encode_seq(test_0001_four_units) bytes");
    }

    // 40. decode_seq — data.json decode_seq test_0001_four_units (units)
    {
        auto units = iscc::decode_seq(from_hex(seq_four_hex));
        assert_true(units == seq_four_units, "decode_seq(test_0001_four_units) units");
    }

    // 41. decode_seq / encode_seq byte round trip (data.json decode_seq test_0004)
    {
        auto data = from_hex(
            "21018c3585376ee22d5a22039f07f3ff9e0061ff3ef7cbffffef37ff"
            "300705b41f3472493225d27edcbdb63236846a53ee28deca5e3561acc0c9bbdf81b5");
        auto units = iscc::decode_seq(data);
        assert_eq(units.size(), 3, "decode_seq(test_0004) count == 3");
        assert_true(iscc::encode_seq(units) == data,
                    "encode_seq(decode_seq(test_0004)) byte round trip");
    }

    // 42. encode_seq — empty list surfaces the core error (not the NULL-array error)
    assert_throws_with([] { iscc::encode_seq({}); }, "at least one ISCC-UNIT",
                       "encode_seq(empty) throws 'at least one ISCC-UNIT'");

    // 42a. Empty list inputs surface the core error (not the FFI's "must not be NULL")
    assert_throws_with([] { iscc::gen_mixed_code_v0({}); }, "at least 2 Content-Codes",
                       "gen_mixed_code_v0(empty) throws 'at least 2 Content-Codes'");
    assert_throws_with([] { iscc::gen_iscc_code_v0({}); }, "at least 2 ISCC unit codes",
                       "gen_iscc_code_v0(empty) throws 'at least 2 ISCC unit codes'");
    assert_throws_with([] { iscc::gen_video_code_v0({}); }, "frame_sigs must not be empty",
                       "gen_video_code_v0(empty) throws 'frame_sigs must not be empty'");
    assert_throws_with([] { iscc::soft_hash_video_v0({}, 64); },
                       "frame_sigs must not be empty",
                       "soft_hash_video_v0(empty) throws 'frame_sigs must not be empty'");
    assert_throws_with([] { iscc::alg_simhash({}); }, "must not be empty",
                       "alg_simhash(empty) throws 'must not be empty'");

    // 42b. An empty vector or inner row may reach the FFI as (NULL, 0); it is read as empty
    assert_eq(iscc::alg_minhash_256({}).size(), 32, "alg_minhash_256(empty) returns 32 bytes");
    try {
        iscc::encode_component(0, 0, 0, 64, {});
        pass("encode_component(empty digest) does not report NULL");
    } catch (const iscc::IsccError& e) {
        const std::string msg = e.what();
        if (msg.find("NULL") == std::string::npos) {
            pass("encode_component(empty digest) does not report NULL");
        } else {
            fail("encode_component(empty digest) does not report NULL", msg.c_str());
        }
    }
    try {
        iscc::alg_simhash({std::vector<uint8_t>{}});
        pass("alg_simhash(one empty inner row) does not report NULL");
    } catch (const iscc::IsccError& e) {
        const std::string msg = e.what();
        if (msg.find("must not be NULL") == std::string::npos) {
            pass("alg_simhash(one empty inner row) does not report NULL");
        } else {
            fail("alg_simhash(one empty inner row) does not report NULL", msg.c_str());
        }
    }
    {
        iscc::DataHasher dh;
        dh.update(std::vector<uint8_t>{});
        assert_str_eq(dh.finalize().iscc, iscc::gen_data_code_v0(std::vector<uint8_t>{}).iscc,
                      "DataHasher.update(empty vector) == gen_data_code_v0(empty)");
        iscc::InstanceHasher ih;
        ih.update(std::vector<uint8_t>{});
        assert_str_eq(ih.finalize().iscc,
                      iscc::gen_instance_code_v0(std::vector<uint8_t>{}).iscc,
                      "InstanceHasher.update(empty vector) == gen_instance_code_v0(empty)");
    }

    // 43. encode_seq — embedded NUL in a unit is rejected (not truncated)
    {
        std::string bad = seq_four_units[1];
        bad.push_back('\0');
        bad += "junk";
        assert_throws_with([&] { iscc::encode_seq({seq_four_units[0], bad}); },
                           "index 1", "encode_seq(unit with embedded NUL) throws");
    }

    // 44. decode_seq — empty input throws the core error
    assert_throws_with([] { iscc::decode_seq({}); }, "Empty ISCC-SEQ",
                       "decode_seq(empty) throws 'Empty ISCC-SEQ'");

    // 45. decode_seq — truncated input (last byte dropped) throws
    {
        auto data = from_hex(seq_four_hex);
        data.pop_back();
        assert_throws_with([&] { iscc::decode_seq(data); }, "Truncated ISCC-UNIT",
                           "decode_seq(truncated) throws 'Truncated ISCC-UNIT'");
    }

    // 46. embedded NUL in any string input is rejected (not truncated)
    {
        const std::string nul_needle = "embedded NUL";
        const std::string hello_world("Hello\0World", 11);
        assert_eq(hello_world.size(), 11, "NUL test literal keeps all 11 bytes");

        // Positive control: NUL-free input still yields the documented code.
        assert_str_eq(iscc::gen_text_code_v0("Hello World").iscc, "ISCC:EAASKDNZNYGUUF5A",
                      "gen_text_code_v0(NUL-free) positive control");

        assert_throws_with([&] { iscc::gen_text_code_v0(hello_world); }, nul_needle,
                           "gen_text_code_v0(embedded NUL) throws");
        assert_throws_with([&] { iscc::text_clean(hello_world); }, nul_needle,
                           "text_clean(embedded NUL) throws");

        auto data_code = iscc::gen_data_code_v0(
            reinterpret_cast<const uint8_t*>("Hello World"), 11);
        auto inst_code = iscc::gen_instance_code_v0(
            reinterpret_cast<const uint8_t*>("Hello World"), 11);
        auto iscc_code = iscc::gen_iscc_code_v0({data_code.iscc, inst_code.iscc});
        std::string code_junk = iscc_code.iscc;
        code_junk.push_back('\0');
        code_junk += "junk";
        assert_throws_with([&] { iscc::iscc_decompose(code_junk); }, nul_needle,
                           "iscc_decompose(embedded NUL) throws");
        std::string unit_junk = data_code.iscc;
        unit_junk.push_back('\0');
        unit_junk += "junk";
        assert_throws_with([&] { iscc::iscc_decode(unit_junk); }, nul_needle,
                           "iscc_decode(embedded NUL) throws");
        assert_throws_with(
            [&] { iscc::gen_meta_code_v0("Title", std::string("Desc\0junk", 9)); },
            nul_needle, "gen_meta_code_v0(description with embedded NUL) throws");
        assert_throws_with([&] { iscc::gen_iscc_code_v0({data_code.iscc, unit_junk}); },
                           nul_needle, "gen_iscc_code_v0(element with embedded NUL) throws");
        auto text_code = iscc::gen_text_code_v0("Hello World");
        std::vector<uint8_t> pixels(1024, 0);
        auto image_code = iscc::gen_image_code_v0(pixels);
        std::string image_junk = image_code.iscc;
        image_junk.push_back('\0');
        image_junk += "junk";
        assert_throws_with([&] { iscc::gen_mixed_code_v0({text_code.iscc, image_junk}); },
                           nul_needle, "gen_mixed_code_v0(element with embedded NUL) throws");
        assert_throws_with([&] { iscc::encode_seq({unit_junk}); }, "embedded NUL",
                           "encode_seq(embedded NUL) throws");
        assert_throws_with(
            [&] { iscc::gen_sum_code_v0(std::string("/tmp/x\0junk", 11)); }, nul_needle,
            "gen_sum_code_v0(path with embedded NUL) throws");
    }

    // Shared edge-case vectors (data-edge.json) via the generated header
    {
        iscc_edge_init();  // fill rows over 4095 bytes
        size_t seen = 0;
        seen += run_edge_section("iscc_decode", iscc_edge_iscc_decode,
                                 ISCC_EDGE_ISCC_DECODE_COUNT, check_edge_iscc_decode);
        seen += run_edge_section("encode_seq", iscc_edge_encode_seq,
                                 ISCC_EDGE_ENCODE_SEQ_COUNT, check_edge_encode_seq);
        seen += run_edge_section("gen_text_code_v0", iscc_edge_gen_text_code_v0,
                                 ISCC_EDGE_GEN_TEXT_CODE_V0_COUNT, check_edge_gen_text_code_v0);
        seen += run_edge_section("gen_mixed_code_v0", iscc_edge_gen_mixed_code_v0,
                                 ISCC_EDGE_GEN_MIXED_CODE_V0_COUNT,
                                 check_edge_gen_mixed_code_v0);
        seen += run_edge_section("gen_iscc_code_v0", iscc_edge_gen_iscc_code_v0,
                                 ISCC_EDGE_GEN_ISCC_CODE_V0_COUNT, check_edge_gen_iscc_code_v0);
        seen += run_edge_section("alg_simhash", iscc_edge_alg_simhash,
                                 ISCC_EDGE_ALG_SIMHASH_COUNT, check_edge_alg_simhash);
        seen += run_edge_section("gen_meta_code_v0", iscc_edge_gen_meta_code_v0,
                                 ISCC_EDGE_GEN_META_CODE_V0_COUNT, check_edge_gen_meta_code_v0);
        seen += run_edge_section("gen_video_code_v0", iscc_edge_gen_video_code_v0,
                                 ISCC_EDGE_GEN_VIDEO_CODE_V0_COUNT, check_edge_gen_video_code_v0);
        seen += run_edge_section("alg_cdc_chunks", iscc_edge_alg_cdc_chunks,
                                 ISCC_EDGE_ALG_CDC_CHUNKS_COUNT, check_edge_alg_cdc_chunks);
        seen += run_edge_section("encode_component", iscc_edge_encode_component,
                                 ISCC_EDGE_ENCODE_COMPONENT_COUNT, check_edge_encode_component);
        assert_eq(seen, ISCC_EDGE_TOTAL_COUNT, "data_edge total rows == ISCC_EDGE_TOTAL_COUNT");
    }

    // data-edge check functions fail a bad row (one bad case each)
    {
        const iscc_edge_iscc_decode_vector accepted = {
            "negative", ISCC_EDGE_ERROR, ISCC_EDGE_ERROR, {"ISCC:GAAAAAAAAA", 15},
            0, 0, 0, 0, {nullptr, 0}};
        const iscc_edge_gen_text_code_v0_vector mismatch = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {"Hello World", 11},
            64, {"ISCC:EAAWSPUDF7GJWQVD", 21}};
        assert_true(!check_edge_iscc_decode(accepted),
                    "data_edge negative: expected rejection that succeeds fails");
        assert_true(!check_edge_gen_text_code_v0(mismatch),
                    "data_edge negative: output mismatch fails");
    }
    {
        // The AND of the tie digests, so a strict-majority threshold would match.
        static const char* const digests[] = {"\360\017", "<<"};
        static const size_t lens[] = {2, 2};
        const iscc_edge_alg_simhash_vector tie_as_and = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {digests, lens, 2}, {"0\014", 2}};
        assert_true(!check_edge_alg_simhash(tie_as_and),
                    "data_edge negative: alg_simhash output mismatch fails");
    }
    {
        // The Meta-Code of a different object, against the raw-JCS row's expected code.
        const iscc_edge_gen_meta_code_v0_vector meta_mismatch = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {"Hello", 5}, {"", 0},
            {"{\"t\":\"a&b\"}", 11}, 64, {"ISCC:AAAWKLHFXO3IKDKI", 21}};
        // A ragged frame list whose row expects the iscc-core truncation result.
        const iscc_edge_gen_video_code_v0_vector ragged_ok = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK,
            iscc_edge_gen_video_code_v0[1].frame_sigs, 64, {"ISCC:EMAVFD4RIMPXYQWS", 21}};
        assert_true(!check_edge_gen_meta_code_v0(meta_mismatch),
                    "data_edge negative: gen_meta_code_v0 output mismatch fails");
        assert_true(!check_edge_gen_video_code_v0(ragged_ok),
                    "data_edge negative: gen_video_code_v0 expected output that errors fails");
    }

    // Summary
    std::printf("\n%d passed, %d failed\n", tests_passed, tests_failed);
    return tests_failed > 0 ? 1 : 0;
}
