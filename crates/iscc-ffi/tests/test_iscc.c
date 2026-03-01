/* C test program for iscc-ffi.
 *
 * Links against the iscc-ffi shared library and validates all FFI
 * entrypoints against known expected values from the Rust unit tests.
 * Exits 0 if all tests pass, non-zero on any failure.
 */

#include "iscc.h"
#include <stdio.h>
#include <string.h>

static int tests_passed = 0;
static int tests_failed = 0;

#define ASSERT_STR_EQ(actual, expected, test_name) \
    do { \
        if ((actual) == NULL) { \
            printf("FAIL: %s — got NULL, expected \"%s\"\n", (test_name), (expected)); \
            tests_failed++; \
        } else if (strcmp((actual), (expected)) != 0) { \
            printf("FAIL: %s — got \"%s\", expected \"%s\"\n", (test_name), (actual), (expected)); \
            tests_failed++; \
        } else { \
            printf("PASS: %s\n", (test_name)); \
            tests_passed++; \
        } \
    } while (0)

#define ASSERT_STR_STARTS_WITH(actual, prefix, test_name) \
    do { \
        if ((actual) == NULL) { \
            printf("FAIL: %s — got NULL, expected prefix \"%s\"\n", (test_name), (prefix)); \
            tests_failed++; \
        } else if (strncmp((actual), (prefix), strlen(prefix)) != 0) { \
            printf("FAIL: %s — got \"%s\", expected prefix \"%s\"\n", (test_name), (actual), (prefix)); \
            tests_failed++; \
        } else { \
            printf("PASS: %s\n", (test_name)); \
            tests_passed++; \
        } \
    } while (0)

#define ASSERT_NULL(ptr, test_name) \
    do { \
        if ((ptr) != NULL) { \
            printf("FAIL: %s — expected NULL, got non-NULL\n", (test_name)); \
            tests_failed++; \
        } else { \
            printf("PASS: %s\n", (test_name)); \
            tests_passed++; \
        } \
    } while (0)

#define ASSERT_NOT_NULL(ptr, test_name) \
    do { \
        if ((ptr) == NULL) { \
            printf("FAIL: %s — expected non-NULL, got NULL\n", (test_name)); \
            tests_failed++; \
        } else { \
            printf("PASS: %s\n", (test_name)); \
            tests_passed++; \
        } \
    } while (0)

#define ASSERT_EQ(actual, expected, test_name) \
    do { \
        if ((actual) != (expected)) { \
            printf("FAIL: %s — got %zu, expected %zu\n", (test_name), (size_t)(actual), (size_t)(expected)); \
            tests_failed++; \
        } else { \
            printf("PASS: %s\n", (test_name)); \
            tests_passed++; \
        } \
    } while (0)

int main(void) {
    char *result;

    /* 1. gen_meta_code_v0 — name only */
    result = iscc_gen_meta_code_v0("Die Unendliche Geschichte", NULL, NULL, 64);
    ASSERT_STR_EQ(result, "ISCC:AAAZXZ6OU74YAZIM", "gen_meta_code_v0(name only)");
    iscc_free_string(result);

    /* 2. gen_meta_code_v0 — name + description */
    result = iscc_gen_meta_code_v0("Die Unendliche Geschichte", "Von Michael Ende", NULL, 64);
    ASSERT_STR_EQ(result, "ISCC:AAAZXZ6OU4E45RB5", "gen_meta_code_v0(name + description)");
    iscc_free_string(result);

    /* 3. gen_text_code_v0 */
    result = iscc_gen_text_code_v0("Hello World", 64);
    ASSERT_STR_EQ(result, "ISCC:EAASKDNZNYGUUF5A", "gen_text_code_v0");
    iscc_free_string(result);

    /* 4. gen_image_code_v0 — 1024 zero bytes */
    {
        uint8_t pixels[1024];
        memset(pixels, 0, sizeof(pixels));
        result = iscc_gen_image_code_v0(pixels, sizeof(pixels), 64);
        ASSERT_STR_EQ(result, "ISCC:EEAQAAAAAAAAAAAA", "gen_image_code_v0(zeros)");
        iscc_free_string(result);
    }

    /* 5. gen_instance_code_v0 — empty data */
    {
        uint8_t empty = 0;
        result = iscc_gen_instance_code_v0(&empty, 0, 64);
        ASSERT_STR_EQ(result, "ISCC:IAA26E2JXH27TING", "gen_instance_code_v0(empty)");
        iscc_free_string(result);
    }

    /* 6. gen_data_code_v0 — "Hello World" */
    result = iscc_gen_data_code_v0((const uint8_t *)"Hello World", 11, 64);
    ASSERT_STR_STARTS_WITH(result, "ISCC:", "gen_data_code_v0(Hello World)");
    iscc_free_string(result);

    /* 7. Error handling — NULL text triggers error */
    result = iscc_gen_text_code_v0(NULL, 64);
    ASSERT_NULL(result, "gen_text_code_v0(NULL) returns NULL");
    {
        const char *err = iscc_last_error();
        ASSERT_NOT_NULL(err, "iscc_last_error() non-NULL after error");
    }

    /* 8. Error cleared on success */
    result = iscc_gen_text_code_v0("Hello World", 64);
    ASSERT_NOT_NULL(result, "gen_text_code_v0 succeeds after error");
    {
        const char *err = iscc_last_error();
        ASSERT_NULL(err, "iscc_last_error() NULL after success");
    }
    iscc_free_string(result);

    /* 9. iscc_free_string(NULL) is a no-op */
    iscc_free_string(NULL);
    printf("PASS: iscc_free_string(NULL) no-op\n");
    tests_passed++;

    /* 10. alg_minhash_256 — feed known features, check 32 bytes output */
    {
        uint32_t features[] = {1, 2, 3, 4, 5};
        struct iscc_IsccByteBuffer buf = iscc_alg_minhash_256(features, 5);
        ASSERT_NOT_NULL(buf.data, "alg_minhash_256 data not NULL");
        ASSERT_EQ(buf.len, 32, "alg_minhash_256 len == 32");
        iscc_free_byte_buffer(buf);
    }

    /* 11. alg_simhash — feed single 4-byte digest, check output length matches */
    {
        uint8_t digest[] = {0xFF, 0x00, 0xFF, 0x00};
        const uint8_t *digests[] = {digest};
        size_t lens[] = {4};
        struct iscc_IsccByteBuffer buf = iscc_alg_simhash(digests, lens, 1);
        ASSERT_NOT_NULL(buf.data, "alg_simhash data not NULL");
        ASSERT_EQ(buf.len, 4, "alg_simhash len == 4");
        iscc_free_byte_buffer(buf);
    }

    /* 12. alg_cdc_chunks — feed "Hello World", check at least 1 chunk */
    {
        const uint8_t *data = (const uint8_t *)"Hello World";
        struct iscc_IsccByteBufferArray arr = iscc_alg_cdc_chunks(data, 11, false, 1024);
        ASSERT_NOT_NULL(arr.buffers, "alg_cdc_chunks buffers not NULL");
        if (arr.count >= 1) {
            printf("PASS: alg_cdc_chunks count >= 1 (got %zu)\n", arr.count);
            tests_passed++;
        } else {
            printf("FAIL: alg_cdc_chunks count >= 1 (got %zu)\n", arr.count);
            tests_failed++;
        }
        /* Verify chunk data concatenates to original */
        {
            size_t total = 0;
            size_t i;
            for (i = 0; i < arr.count; i++) {
                total += arr.buffers[i].len;
            }
            ASSERT_EQ(total, 11, "alg_cdc_chunks total bytes == 11");
        }
        iscc_free_byte_buffer_array(arr);
    }

    /* 13. soft_hash_video_v0 — feed frame sigs, check output len == 8 */
    {
        int32_t frame1[380];
        int32_t frame2[380];
        int i;
        for (i = 0; i < 380; i++) {
            frame1[i] = i;
            frame2[i] = i + 1;
        }
        const int32_t *sigs[] = {frame1, frame2};
        size_t lens[] = {380, 380};
        struct iscc_IsccByteBuffer buf = iscc_soft_hash_video_v0(sigs, lens, 2, 64);
        ASSERT_NOT_NULL(buf.data, "soft_hash_video_v0 data not NULL");
        ASSERT_EQ(buf.len, 8, "soft_hash_video_v0 len == 8 (64 bits)");
        iscc_free_byte_buffer(buf);
    }

    /* 14. DataHasher basic lifecycle */
    {
        struct iscc_FfiDataHasher *dh = iscc_data_hasher_new();
        ASSERT_NOT_NULL(dh, "data_hasher_new returns non-NULL");
        bool ok = iscc_data_hasher_update(dh, (const uint8_t *)"Hello World", 11);
        if (ok) {
            printf("PASS: data_hasher_update returns true\n");
            tests_passed++;
        } else {
            printf("FAIL: data_hasher_update returned false\n");
            tests_failed++;
        }
        result = iscc_data_hasher_finalize(dh, 64);
        ASSERT_STR_STARTS_WITH(result, "ISCC:", "data_hasher_finalize starts with ISCC:");
        iscc_free_string(result);
        iscc_data_hasher_free(dh);
    }

    /* 15. InstanceHasher empty data — finalize immediately */
    {
        struct iscc_FfiInstanceHasher *ih = iscc_instance_hasher_new();
        ASSERT_NOT_NULL(ih, "instance_hasher_new returns non-NULL");
        result = iscc_instance_hasher_finalize(ih, 64);
        ASSERT_STR_EQ(result, "ISCC:IAA26E2JXH27TING", "instance_hasher_finalize(empty)");
        iscc_free_string(result);
        iscc_instance_hasher_free(ih);
    }

    /* 16. DataHasher multi-update matches single update */
    {
        /* Single update */
        struct iscc_FfiDataHasher *dh1 = iscc_data_hasher_new();
        iscc_data_hasher_update(dh1, (const uint8_t *)"Hello World", 11);
        char *r1 = iscc_data_hasher_finalize(dh1, 64);
        iscc_data_hasher_free(dh1);

        /* Split update */
        struct iscc_FfiDataHasher *dh2 = iscc_data_hasher_new();
        iscc_data_hasher_update(dh2, (const uint8_t *)"Hello", 5);
        iscc_data_hasher_update(dh2, (const uint8_t *)" World", 6);
        char *r2 = iscc_data_hasher_finalize(dh2, 64);
        iscc_data_hasher_free(dh2);

        ASSERT_NOT_NULL(r1, "data_hasher multi-update r1 not NULL");
        ASSERT_NOT_NULL(r2, "data_hasher multi-update r2 not NULL");
        if (r1 != NULL && r2 != NULL) {
            if (strcmp(r1, r2) == 0) {
                printf("PASS: data_hasher multi-update matches single update\n");
                tests_passed++;
            } else {
                printf("FAIL: data_hasher multi-update mismatch: \"%s\" vs \"%s\"\n", r1, r2);
                tests_failed++;
            }
        }
        iscc_free_string(r1);
        iscc_free_string(r2);
    }

    /* 17. Free NULL safety for both hasher types */
    iscc_data_hasher_free(NULL);
    printf("PASS: iscc_data_hasher_free(NULL) no-op\n");
    tests_passed++;

    iscc_instance_hasher_free(NULL);
    printf("PASS: iscc_instance_hasher_free(NULL) no-op\n");
    tests_passed++;

    /* 18. Algorithm constants */
    ASSERT_EQ(iscc_meta_trim_name(), 128, "iscc_meta_trim_name() == 128");
    ASSERT_EQ(iscc_meta_trim_description(), 4096, "iscc_meta_trim_description() == 4096");
    ASSERT_EQ(iscc_meta_trim_meta(), 128000, "iscc_meta_trim_meta() == 128000");
    ASSERT_EQ(iscc_io_read_size(), 4194304, "iscc_io_read_size() == 4194304");
    ASSERT_EQ(iscc_text_ngram_size(), 13, "iscc_text_ngram_size() == 13");

    /* 19. json_to_data_url */
    result = iscc_json_to_data_url("{\"key\":\"value\"}");
    ASSERT_STR_STARTS_WITH(result, "data:application/json;base64,", "json_to_data_url prefix");
    iscc_free_string(result);

    /* 20. encode_component — Meta-Code (mtype=0, stype=0, version=0, 64-bit) */
    {
        uint8_t digest[8] = {0};
        result = iscc_encode_component(0, 0, 0, 64, digest, 8);
        ASSERT_NOT_NULL(result, "encode_component returns non-NULL");
        iscc_free_string(result);
    }

    /* 21. iscc_decode — known Meta-Code */
    {
        struct iscc_IsccDecodeResult dr = iscc_decode("AAAZXZ6OU74YAZIM");
        if (dr.ok) {
            printf("PASS: iscc_decode ok == true\n");
            tests_passed++;
        } else {
            printf("FAIL: iscc_decode ok == false\n");
            tests_failed++;
        }
        ASSERT_EQ(dr.maintype, 0, "iscc_decode maintype == 0 (Meta)");
        ASSERT_EQ(dr.subtype, 0, "iscc_decode subtype == 0");
        ASSERT_EQ(dr.version, 0, "iscc_decode version == 0");
        ASSERT_EQ(dr.length, 1, "iscc_decode length == 1 (64-bit)");
        ASSERT_NOT_NULL(dr.digest.data, "iscc_decode digest not NULL");
        ASSERT_EQ(dr.digest.len, 8, "iscc_decode digest len == 8");
        iscc_free_decode_result(dr);
    }

    /* 22. iscc_decode — invalid input returns ok=false */
    {
        struct iscc_IsccDecodeResult dr = iscc_decode("INVALID");
        if (!dr.ok) {
            printf("PASS: iscc_decode(invalid) ok == false\n");
            tests_passed++;
        } else {
            printf("FAIL: iscc_decode(invalid) ok should be false\n");
            tests_failed++;
        }
        iscc_free_decode_result(dr);
    }

    /* 23. Roundtrip: encode_component → iscc_decode */
    {
        uint8_t digest[8] = {0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89};
        char *encoded = iscc_encode_component(0, 0, 0, 64, digest, 8);
        ASSERT_NOT_NULL(encoded, "roundtrip: encode_component not NULL");
        if (encoded != NULL) {
            struct iscc_IsccDecodeResult dr = iscc_decode(encoded);
            if (dr.ok) {
                printf("PASS: roundtrip: decode ok\n");
                tests_passed++;
            } else {
                printf("FAIL: roundtrip: decode not ok\n");
                tests_failed++;
            }
            ASSERT_EQ(dr.maintype, 0, "roundtrip: maintype == 0");
            ASSERT_EQ(dr.digest.len, 8, "roundtrip: digest len == 8");
            if (dr.digest.data != NULL && dr.digest.len == 8 &&
                memcmp(dr.digest.data, digest, 8) == 0) {
                printf("PASS: roundtrip: digest matches\n");
                tests_passed++;
            } else {
                printf("FAIL: roundtrip: digest mismatch\n");
                tests_failed++;
            }
            iscc_free_decode_result(dr);
            iscc_free_string(encoded);
        }
    }

    /* Summary */
    printf("\n%d passed, %d failed\n", tests_passed, tests_failed);
    return tests_failed > 0 ? 1 : 0;
}
