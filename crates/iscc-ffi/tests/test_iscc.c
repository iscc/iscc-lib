/* C test program for iscc-ffi.
 *
 * Links against the iscc-ffi shared library and validates all FFI
 * entrypoints against known expected values from the Rust unit tests.
 * Exits 0 if all tests pass, non-zero on any failure.
 */

#include "iscc.h"
#include "data_edge_vectors.h"
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

/* Return 1 if the last error is set and contains `needle`. */
static int last_error_has(const char *needle) {
    const char *err = iscc_last_error();
    return err != NULL && strstr(err, needle) != NULL;
}

/* Return 1 if the last error is unset or lacks "must not be NULL". */
static int last_error_not_null_rejection(void) {
    return !last_error_has("must not be NULL");
}

/* Return 1 if two FFI string results are equal (both NULL counts); frees both. */
static int same_str_result(char *a, char *b) {
    int same = (a == NULL && b == NULL) || (a != NULL && b != NULL && strcmp(a, b) == 0);
    iscc_free_string(a);
    iscc_free_string(b);
    return same;
}

/* Return 1 if two FFI byte buffers are equal (both NULL counts); frees both. */
static int same_buf_result(struct iscc_IsccByteBuffer a, struct iscc_IsccByteBuffer b) {
    int same = (a.data == NULL && b.data == NULL) ||
               (a.data != NULL && b.data != NULL && a.len == b.len &&
                memcmp(a.data, b.data, a.len) == 0);
    iscc_free_byte_buffer(a);
    iscc_free_byte_buffer(b);
    return same;
}

/* Check that a (NULL, 0) call matches the same call with a non-NULL pointer and that it
 * never fails with "must not be NULL". `null_call` runs first, so the error is its own. */
#define CHECK_NULL_EMPTY_STR(null_call, ptr_call, test_name) \
    do { \
        char *null_res = (null_call); \
        int null_ok = last_error_not_null_rejection(); \
        char *ptr_res = (ptr_call); \
        ASSERT_EQ(null_ok, 1, test_name " — no NULL rejection"); \
        ASSERT_EQ(same_str_result(null_res, ptr_res), 1, test_name " — matches non-NULL"); \
    } while (0)

#define CHECK_NULL_EMPTY_BUF(null_call, ptr_call, test_name) \
    do { \
        struct iscc_IsccByteBuffer null_res = (null_call); \
        int null_ok = last_error_not_null_rejection(); \
        struct iscc_IsccByteBuffer ptr_res = (ptr_call); \
        ASSERT_EQ(null_ok, 1, test_name " — no NULL rejection"); \
        ASSERT_EQ(same_buf_result(null_res, ptr_res), 1, test_name " — matches non-NULL"); \
    } while (0)

/* Return 1 if an edge-vector string reaches C intact: non-NULL and no embedded NUL,
 * so a row whose input C cannot express fails instead of hashing the prefix. */
static int edge_str_fits_c(struct iscc_edge_str s) {
    return s.ptr != NULL && strlen(s.ptr) == s.len;
}

/* Return 1 if every item of an edge-vector list reaches C intact. */
static int edge_list_fits_c(struct iscc_edge_list list) {
    size_t i;
    for (i = 0; i < list.count; i++) {
        if (list.items[i] == NULL || strlen(list.items[i]) != list.lens[i]) {
            return 0;
        }
    }
    return 1;
}

/* Return 1 if a returned buffer equals an expected edge-vector byte string. */
static int edge_bytes_eq(const uint8_t *data, size_t len, struct iscc_edge_str expected) {
    return len == expected.len && (len == 0 || memcmp(data, expected.ptr, len) == 0);
}

/* Return 1 if a returned gen string meets the row expectation (rejection = NULL). */
static int edge_gen_result_ok(int expect, const char *actual, struct iscc_edge_str iscc) {
    if (expect == ISCC_EDGE_ERROR) {
        return actual == NULL;
    }
    return expect == ISCC_EDGE_OK && actual != NULL && strlen(actual) == iscc.len &&
           memcmp(actual, iscc.ptr, iscc.len) == 0;
}

/* Check one iscc_decode edge row against its c expectation (rejection = ok false). */
static int check_edge_iscc_decode(const struct iscc_edge_iscc_decode_vector *v) {
    struct iscc_IsccDecodeResult r;
    int passed;
    if (!edge_str_fits_c(v->input)) {
        return 0;
    }
    r = iscc_decode(v->input.ptr);
    if (v->c == ISCC_EDGE_ERROR) {
        passed = !r.ok;
    } else {
        passed = v->c == ISCC_EDGE_OK && r.ok && r.maintype == v->maintype &&
                 r.subtype == v->subtype && r.version == v->version &&
                 r.length == v->length && r.digest.data != NULL &&
                 edge_bytes_eq(r.digest.data, r.digest.len, v->digest);
    }
    iscc_free_decode_result(r);
    return passed;
}

/* Check one encode_seq edge row against its c expectation (rejection = NULL data). */
static int check_edge_encode_seq(const struct iscc_edge_encode_seq_vector *v) {
    struct iscc_IsccByteBuffer buf;
    int passed;
    if (!edge_list_fits_c(v->units)) {
        return 0;
    }
    buf = iscc_encode_seq(v->units.items, v->units.count);
    if (v->c == ISCC_EDGE_ERROR) {
        passed = buf.data == NULL;
    } else {
        passed = v->c == ISCC_EDGE_OK && buf.data != NULL &&
                 edge_bytes_eq(buf.data, buf.len, v->seq);
    }
    iscc_free_byte_buffer(buf);
    return passed;
}

/* Check one alg_simhash edge row against its c expectation (rejection = NULL data).
 * Digests are binary with explicit lengths, so no C-string fit check applies. */
static int check_edge_alg_simhash(const struct iscc_edge_alg_simhash_vector *v) {
    struct iscc_IsccByteBuffer buf;
    int passed;
    buf = iscc_alg_simhash((const uint8_t *const *)v->digests.items, v->digests.lens,
                           v->digests.count);
    if (v->c == ISCC_EDGE_ERROR) {
        passed = buf.data == NULL;
    } else {
        passed = v->c == ISCC_EDGE_OK && buf.data != NULL &&
                 edge_bytes_eq(buf.data, buf.len, v->simhash);
    }
    iscc_free_byte_buffer(buf);
    return passed;
}

/* Check one gen_text_code_v0 edge row against its c expectation. */
static int check_edge_gen_text_code_v0(const struct iscc_edge_gen_text_code_v0_vector *v) {
    char *actual;
    int passed;
    if (!edge_str_fits_c(v->text)) {
        return 0;
    }
    actual = iscc_gen_text_code_v0(v->text.ptr, v->bits);
    passed = edge_gen_result_ok(v->c, actual, v->iscc);
    iscc_free_string(actual);
    return passed;
}

/* Check one gen_mixed_code_v0 edge row against its c expectation. */
static int check_edge_gen_mixed_code_v0(const struct iscc_edge_gen_mixed_code_v0_vector *v) {
    char *actual;
    int passed;
    if (!edge_list_fits_c(v->codes)) {
        return 0;
    }
    actual = iscc_gen_mixed_code_v0(v->codes.items, v->codes.count, v->bits);
    passed = edge_gen_result_ok(v->c, actual, v->iscc);
    iscc_free_string(actual);
    return passed;
}

/* Check one gen_iscc_code_v0 edge row against its c expectation. */
static int check_edge_gen_iscc_code_v0(const struct iscc_edge_gen_iscc_code_v0_vector *v) {
    char *actual;
    int passed;
    if (!edge_list_fits_c(v->codes)) {
        return 0;
    }
    actual = iscc_gen_iscc_code_v0(v->codes.items, v->codes.count, v->wide != 0);
    passed = edge_gen_result_ok(v->c, actual, v->iscc);
    iscc_free_string(actual);
    return passed;
}

/* Check one gen_meta_code_v0 edge row against its c expectation.
 * An empty description is passed as NULL (no description). */
static int check_edge_gen_meta_code_v0(const struct iscc_edge_gen_meta_code_v0_vector *v) {
    char *actual;
    int passed;
    if (!edge_str_fits_c(v->input_name) || !edge_str_fits_c(v->description) ||
        !edge_str_fits_c(v->meta)) {
        return 0;
    }
    actual = iscc_gen_meta_code_v0(v->input_name.ptr,
                                   v->description.len ? v->description.ptr : NULL,
                                   v->meta.ptr, v->bits);
    passed = edge_gen_result_ok(v->c, actual, v->iscc);
    iscc_free_string(actual);
    return passed;
}

/* Check one gen_video_code_v0 edge row against its c expectation. */
static int check_edge_gen_video_code_v0(const struct iscc_edge_gen_video_code_v0_vector *v) {
    char *actual;
    int passed;
    actual = iscc_gen_video_code_v0(v->frame_sigs.frames, v->frame_sigs.lens,
                                    v->frame_sigs.count, v->bits);
    passed = edge_gen_result_ok(v->c, actual, v->iscc);
    iscc_free_string(actual);
    return passed;
}

/* Run every row of one data-edge section: skip rows marked c skip, check the rest,
 * print PASS/FAIL per row, and add each row (run or skipped) to `seen`. */
#define RUN_EDGE_SECTION(section, rows, count, check, seen) \
    do { \
        size_t edge_i_; \
        for (edge_i_ = 0; edge_i_ < (size_t)(count); edge_i_++) { \
            char edge_name_[160]; \
            int edge_passed_; \
            (seen)++; \
            snprintf(edge_name_, sizeof(edge_name_), "data_edge/%s/%s", (section), \
                     (rows)[edge_i_].name); \
            if ((rows)[edge_i_].c == ISCC_EDGE_SKIP) { \
                printf("SKIP: %s\n", edge_name_); \
                continue; \
            } \
            edge_passed_ = check(&(rows)[edge_i_]); \
            ASSERT_EQ(edge_passed_, 1, edge_name_); \
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

    /* 24. gen_sum_code_v0 — temp file */
    {
        const char *tmppath = "/tmp/iscc_c_test_sum.bin";
        FILE *fp = fopen(tmppath, "wb");
        if (fp != NULL) {
            fwrite("Hello World", 1, 11, fp);
            fclose(fp);

            struct iscc_IsccSumCodeResult sr = iscc_gen_sum_code_v0(tmppath, 64, false, false);
            if (sr.ok) {
                printf("PASS: gen_sum_code_v0 ok == true\n");
                tests_passed++;
            } else {
                printf("FAIL: gen_sum_code_v0 ok should be true\n");
                tests_failed++;
            }
            ASSERT_STR_STARTS_WITH(sr.iscc, "ISCC:", "gen_sum_code_v0 iscc starts with ISCC:");
            ASSERT_NOT_NULL(sr.datahash, "gen_sum_code_v0 datahash not NULL");
            ASSERT_EQ(sr.filesize, 11, "gen_sum_code_v0 filesize == 11");
            ASSERT_NULL(sr.units, "gen_sum_code_v0 units NULL when disabled");
            iscc_free_sum_code_result(sr);
            remove(tmppath);
        } else {
            printf("FAIL: gen_sum_code_v0 — could not create temp file\n");
            tests_failed++;
        }
    }

    /* 25. gen_sum_code_v0 — NULL path */
    {
        struct iscc_IsccSumCodeResult sr = iscc_gen_sum_code_v0(NULL, 64, false, false);
        if (!sr.ok) {
            printf("PASS: gen_sum_code_v0(NULL) ok == false\n");
            tests_passed++;
        } else {
            printf("FAIL: gen_sum_code_v0(NULL) ok should be false\n");
            tests_failed++;
        }
        ASSERT_NULL(sr.iscc, "gen_sum_code_v0(NULL) iscc is NULL");
    }

    /* 26. gen_sum_code_v0 — nonexistent path */
    {
        struct iscc_IsccSumCodeResult sr = iscc_gen_sum_code_v0("/nonexistent/file.bin", 64, false, false);
        if (!sr.ok) {
            printf("PASS: gen_sum_code_v0(nonexistent) ok == false\n");
            tests_passed++;
        } else {
            printf("FAIL: gen_sum_code_v0(nonexistent) ok should be false\n");
            tests_failed++;
        }
    }

    /* 27. gen_sum_code_v0 — add_units=true returns units array */
    {
        const char *tmppath = "/tmp/iscc_c_test_sum_units.bin";
        FILE *fp = fopen(tmppath, "wb");
        if (fp != NULL) {
            fwrite("Hello World", 1, 11, fp);
            fclose(fp);

            struct iscc_IsccSumCodeResult sr = iscc_gen_sum_code_v0(tmppath, 64, false, true);
            if (sr.ok) {
                printf("PASS: gen_sum_code_v0(units=true) ok == true\n");
                tests_passed++;
            } else {
                printf("FAIL: gen_sum_code_v0(units=true) ok should be true\n");
                tests_failed++;
            }
            ASSERT_NOT_NULL(sr.units, "gen_sum_code_v0(units=true) units not NULL");
            if (sr.units != NULL) {
                /* Verify 2 ISCC strings + NULL terminator */
                ASSERT_STR_STARTS_WITH(sr.units[0], "ISCC:", "units[0] starts with ISCC:");
                ASSERT_STR_STARTS_WITH(sr.units[1], "ISCC:", "units[1] starts with ISCC:");
                ASSERT_NULL(sr.units[2], "units[2] is NULL terminator");
            }
            iscc_free_sum_code_result(sr);
            remove(tmppath);
        } else {
            printf("FAIL: gen_sum_code_v0(units=true) — could not create temp file\n");
            tests_failed++;
        }
    }

    /* 28. gen_sum_code_v0 — add_units=false returns NULL units */
    {
        const char *tmppath = "/tmp/iscc_c_test_sum_nounits.bin";
        FILE *fp = fopen(tmppath, "wb");
        if (fp != NULL) {
            fwrite("Hello World", 1, 11, fp);
            fclose(fp);

            struct iscc_IsccSumCodeResult sr = iscc_gen_sum_code_v0(tmppath, 64, false, false);
            if (sr.ok) {
                printf("PASS: gen_sum_code_v0(units=false) ok == true\n");
                tests_passed++;
            } else {
                printf("FAIL: gen_sum_code_v0(units=false) ok should be true\n");
                tests_failed++;
            }
            ASSERT_NULL(sr.units, "gen_sum_code_v0(units=false) units is NULL");
            iscc_free_sum_code_result(sr);
            remove(tmppath);
        } else {
            printf("FAIL: gen_sum_code_v0(units=false) — could not create temp file\n");
            tests_failed++;
        }
    }

    /* 29. gen_iscc_id_v1 — golden vector */
    result = iscc_gen_iscc_id_v1(1751831876325218ULL, 1, 0);
    ASSERT_STR_EQ(result, "ISCC:MAIGHFECJMOPMIAB", "gen_iscc_id_v1(golden)");
    iscc_free_string(result);

    /* 30. gen_iscc_id_v1 — invalid realm returns NULL + sets error */
    result = iscc_gen_iscc_id_v1(1751831876325218ULL, 1, 2);
    ASSERT_NULL(result, "gen_iscc_id_v1(realm=2) returns NULL");
    {
        const char *err = iscc_last_error();
        ASSERT_NOT_NULL(err, "iscc_last_error() non-NULL after gen_iscc_id_v1 error");
    }

    /* 31. encode_seq / decode_seq — round trip (data.json test_0001_four_units) */
    {
        const char *seq_units[] = {
            "ISCC:AADVPDD4R6733NMPFH3D57VTQ4KVH6VHL74XIWGUT37V5ZG56NV3NSI",
            "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI",
            "ISCC:GADQLNA7GRZESMRF2J7NZPNWGI3II2ST5YUN5SS6GVQ2ZQGJXPPYDNI",
            "ISCC:IAD2KIVPJIWJZP3KQCESJL6SVT5APEZUPOJWM6HVTAXCF7OT3VFA4NY",
        };
        struct iscc_IsccByteBuffer seq = iscc_encode_seq(seq_units, 4);
        ASSERT_NOT_NULL(seq.data, "encode_seq(four units) data not NULL");
        ASSERT_EQ(seq.len, 4 * 34, "encode_seq(four units) len == 136");

        char **decoded = iscc_decode_seq(seq.data, seq.len);
        ASSERT_NOT_NULL(decoded, "decode_seq(four units) not NULL");
        if (decoded != NULL) {
            size_t count = 0;
            while (decoded[count] != NULL) {
                count++;
            }
            ASSERT_EQ(count, 4, "decode_seq(four units) count == 4");
            for (size_t i = 0; i < count && i < 4; i++) {
                ASSERT_STR_EQ(decoded[i], seq_units[i], "decode_seq(four units) round trip");
            }
        }
        iscc_free_string_array(decoded);
        iscc_free_byte_buffer(seq);
    }

    /* 32. encode_seq — empty list (num_units = 0, non-NULL array) returns NULL data */
    {
        const char *seq_units[] = {
            "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI",
        };
        struct iscc_IsccByteBuffer seq = iscc_encode_seq(seq_units, 0);
        ASSERT_NULL(seq.data, "encode_seq(num_units=0) data is NULL");
        const char *err = iscc_last_error();
        ASSERT_NOT_NULL(err, "iscc_last_error() non-NULL after encode_seq error");
    }

    /* 33. Shared edge-case vectors (data-edge.json) via the generated header */
    {
        size_t edge_seen = 0;
        RUN_EDGE_SECTION("iscc_decode", iscc_edge_iscc_decode, ISCC_EDGE_ISCC_DECODE_COUNT,
                         check_edge_iscc_decode, edge_seen);
        RUN_EDGE_SECTION("encode_seq", iscc_edge_encode_seq, ISCC_EDGE_ENCODE_SEQ_COUNT,
                         check_edge_encode_seq, edge_seen);
        RUN_EDGE_SECTION("gen_text_code_v0", iscc_edge_gen_text_code_v0,
                         ISCC_EDGE_GEN_TEXT_CODE_V0_COUNT, check_edge_gen_text_code_v0,
                         edge_seen);
        RUN_EDGE_SECTION("gen_mixed_code_v0", iscc_edge_gen_mixed_code_v0,
                         ISCC_EDGE_GEN_MIXED_CODE_V0_COUNT, check_edge_gen_mixed_code_v0,
                         edge_seen);
        RUN_EDGE_SECTION("gen_iscc_code_v0", iscc_edge_gen_iscc_code_v0,
                         ISCC_EDGE_GEN_ISCC_CODE_V0_COUNT, check_edge_gen_iscc_code_v0,
                         edge_seen);
        RUN_EDGE_SECTION("alg_simhash", iscc_edge_alg_simhash, ISCC_EDGE_ALG_SIMHASH_COUNT,
                         check_edge_alg_simhash, edge_seen);
        RUN_EDGE_SECTION("gen_meta_code_v0", iscc_edge_gen_meta_code_v0,
                         ISCC_EDGE_GEN_META_CODE_V0_COUNT, check_edge_gen_meta_code_v0,
                         edge_seen);
        RUN_EDGE_SECTION("gen_video_code_v0", iscc_edge_gen_video_code_v0,
                         ISCC_EDGE_GEN_VIDEO_CODE_V0_COUNT, check_edge_gen_video_code_v0,
                         edge_seen);
        ASSERT_EQ(edge_seen, ISCC_EDGE_TOTAL_COUNT,
                  "data_edge total rows == ISCC_EDGE_TOTAL_COUNT");
    }

    /* 34. data-edge check functions fail a bad row (one bad case each) */
    {
        const struct iscc_edge_iscc_decode_vector accepted = {
            "negative", ISCC_EDGE_ERROR, ISCC_EDGE_ERROR, {"ISCC:GAAAAAAAAA", 15},
            0, 0, 0, 0, {0, 0}};
        const struct iscc_edge_gen_text_code_v0_vector mismatch = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {"Hello World", 11},
            64, {"ISCC:EAAWSPUDF7GJWQVD", 21}};
        const struct iscc_edge_gen_text_code_v0_vector truncated = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {"Hello\000World", 11},
            64, {"ISCC:EAAWSPUDF7GJWQVD", 21}};
        ASSERT_EQ(check_edge_iscc_decode(&accepted), 0,
                  "data_edge negative: expected rejection that succeeds fails");
        ASSERT_EQ(check_edge_gen_text_code_v0(&mismatch), 0,
                  "data_edge negative: output mismatch fails");
        ASSERT_EQ(check_edge_gen_text_code_v0(&truncated), 0,
                  "data_edge negative: string with embedded NUL fails");
    }
    {
        /* The AND of the tie digests, so a strict-majority threshold would match. */
        static const char *const digests[] = {"\360\017", "<<"};
        static const size_t lens[] = {2, 2};
        const struct iscc_edge_alg_simhash_vector tie_as_and = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {digests, lens, 2}, {"0\014", 2}};
        ASSERT_EQ(check_edge_alg_simhash(&tie_as_and), 0,
                  "data_edge negative: alg_simhash output mismatch fails");
    }
    {
        /* The Meta-Code of a different object, against the raw-JCS row's expected code. */
        const struct iscc_edge_gen_meta_code_v0_vector meta_mismatch = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK, {"Hello", 5}, {"", 0},
            {"{\"t\":\"a&b\"}", 11}, 64, {"ISCC:AAAWKLHFXO3IKDKI", 21}};
        /* A ragged frame list whose row expects the iscc-core truncation result. */
        const struct iscc_edge_gen_video_code_v0_vector ragged_ok = {
            "negative", ISCC_EDGE_OK, ISCC_EDGE_OK,
            iscc_edge_gen_video_code_v0[1].frame_sigs, 64, {"ISCC:EMAVFD4RIMPXYQWS", 21}};
        ASSERT_EQ(check_edge_gen_meta_code_v0(&meta_mismatch), 0,
                  "data_edge negative: gen_meta_code_v0 output mismatch fails");
        ASSERT_EQ(check_edge_gen_video_code_v0(&ragged_ok), 0,
                  "data_edge negative: gen_video_code_v0 expected output that errors fails");
    }

    /* 35. (NULL, 0) is an empty input on every pointer+length parameter */
    {
        static const uint8_t byte0 = 0;
        static const int32_t int0 = 0;
        static const uint32_t u0 = 0;
        static const char *const no_codes[] = {NULL};
        static const int32_t *const empty_row[] = {&int0};
        static const int32_t *const null_row[] = {NULL};
        static const uint8_t *const empty_digest[] = {&byte0};
        static const uint8_t *const null_digest[] = {NULL};
        static const size_t zero_len[] = {0};
        CHECK_NULL_EMPTY_STR(iscc_gen_image_code_v0(NULL, 0, 64),
                             iscc_gen_image_code_v0(&byte0, 0, 64), "gen_image_code_v0(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_audio_code_v0(NULL, 0, 64),
                             iscc_gen_audio_code_v0(&int0, 0, 64), "gen_audio_code_v0(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_data_code_v0(NULL, 0, 64),
                             iscc_gen_data_code_v0(&byte0, 0, 64), "gen_data_code_v0(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_instance_code_v0(NULL, 0, 64),
                             iscc_gen_instance_code_v0(&byte0, 0, 64),
                             "gen_instance_code_v0(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_encode_base64(NULL, 0), iscc_encode_base64(&byte0, 0),
                             "encode_base64(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_mixed_code_v0(NULL, 0, 64),
                             iscc_gen_mixed_code_v0(no_codes, 0, 64),
                             "gen_mixed_code_v0(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_iscc_code_v0(NULL, 0, false),
                             iscc_gen_iscc_code_v0(no_codes, 0, false),
                             "gen_iscc_code_v0(NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_video_code_v0(NULL, NULL, 0, 64),
                             iscc_gen_video_code_v0(empty_row, zero_len, 0, 64),
                             "gen_video_code_v0(NULL, NULL, 0)");
        CHECK_NULL_EMPTY_STR(iscc_gen_video_code_v0(null_row, zero_len, 1, 64),
                             iscc_gen_video_code_v0(empty_row, zero_len, 1, 64),
                             "gen_video_code_v0 NULL row of length 0");
        CHECK_NULL_EMPTY_BUF(iscc_encode_seq(NULL, 0), iscc_encode_seq(no_codes, 0),
                             "encode_seq(NULL, 0)");
        CHECK_NULL_EMPTY_BUF(iscc_alg_minhash_256(NULL, 0), iscc_alg_minhash_256(&u0, 0),
                             "alg_minhash_256(NULL, 0)");
        CHECK_NULL_EMPTY_BUF(iscc_soft_hash_video_v0(null_row, zero_len, 1, 64),
                             iscc_soft_hash_video_v0(empty_row, zero_len, 1, 64),
                             "soft_hash_video_v0 NULL row of length 0");
        CHECK_NULL_EMPTY_BUF(iscc_alg_simhash(null_digest, zero_len, 1),
                             iscc_alg_simhash(empty_digest, zero_len, 1),
                             "alg_simhash NULL row of length 0");
    }

    /* 36. NULL with a non-zero length is rejected with the parameter name (one per call) */
    {
        static const int32_t frame[380] = {0};
        static const uint8_t digest[4] = {1, 2, 3, 4};
        static const int32_t *const sigs_null_row[] = {frame, NULL};
        static const size_t frame_lens[] = {380, 380};
        static const uint8_t *const digests_null_row[] = {digest, NULL};
        static const size_t digest_lens[] = {4, 4};
        char *s;
        struct iscc_IsccByteBuffer b;

        s = iscc_gen_data_code_v0(NULL, 4, 64);
        ASSERT_NULL(s, "gen_data_code_v0(NULL, 4) returns NULL");
        ASSERT_EQ(last_error_has("data must not be NULL"), 1,
                  "gen_data_code_v0(NULL, 4) names data");
        s = iscc_gen_image_code_v0(NULL, 1024, 64);
        ASSERT_NULL(s, "gen_image_code_v0(NULL, 1024) returns NULL");
        ASSERT_EQ(last_error_has("pixels must not be NULL"), 1,
                  "gen_image_code_v0(NULL, 1024) names pixels");
        s = iscc_gen_mixed_code_v0(NULL, 2, 64);
        ASSERT_NULL(s, "gen_mixed_code_v0(NULL, 2) returns NULL");
        ASSERT_EQ(last_error_has("codes must not be NULL"), 1,
                  "gen_mixed_code_v0(NULL, 2) names codes");
        s = iscc_gen_video_code_v0(sigs_null_row, frame_lens, 2, 64);
        ASSERT_NULL(s, "gen_video_code_v0 NULL row with length returns NULL");
        ASSERT_EQ(last_error_has("frame_sigs[1] must not be NULL"), 1,
                  "gen_video_code_v0 NULL row names frame_sigs[1]");
        b = iscc_soft_hash_video_v0(sigs_null_row, frame_lens, 2, 64);
        ASSERT_NULL(b.data, "soft_hash_video_v0 NULL row with length returns NULL");
        ASSERT_EQ(last_error_has("frame_sigs[1] must not be NULL"), 1,
                  "soft_hash_video_v0 NULL row names frame_sigs[1]");
        b = iscc_alg_simhash(digests_null_row, digest_lens, 2);
        ASSERT_NULL(b.data, "alg_simhash NULL row with length returns NULL");
        ASSERT_EQ(last_error_has("digests[1] must not be NULL"), 1,
                  "alg_simhash NULL row names digests[1]");
    }

    /* Summary */
    printf("\n%d passed, %d failed\n", tests_passed, tests_failed);
    return tests_failed > 0 ? 1 : 0;
}
