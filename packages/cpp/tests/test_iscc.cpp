/// @file test_iscc.cpp
/// @brief Smoke tests for the iscc C++17 header-only wrapper.
///
/// Links against the iscc-ffi shared library and validates all wrapper
/// patterns against known expected values from the C test suite.
/// Exits 0 if all tests pass, non-zero on any failure.

#include <iscc/iscc.hpp>

#include <cassert>
#include <cstdio>
#include <cstring>
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
        const char* tmppath = "/tmp/iscc_cpp_test_sum.bin";
        {
            std::ofstream f(tmppath, std::ios::binary);
            f.write("Hello World", 11);
        }
        auto r = iscc::gen_sum_code_v0(tmppath);
        assert_starts_with(r.iscc, "ISCC:", "gen_sum_code_v0 iscc");
        assert_true(!r.datahash.empty(), "gen_sum_code_v0 datahash not empty");
        assert_eq(r.filesize, 11, "gen_sum_code_v0 filesize == 11");
        assert_true(r.units.empty(), "gen_sum_code_v0 units empty when disabled");
        std::remove(tmppath);
    }

    // 29. gen_sum_code_v0 — add_units=true
    {
        const char* tmppath = "/tmp/iscc_cpp_test_sum_units.bin";
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
        std::remove(tmppath);
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

    // Summary
    std::printf("\n%d passed, %d failed\n", tests_passed, tests_failed);
    return tests_failed > 0 ? 1 : 0;
}
