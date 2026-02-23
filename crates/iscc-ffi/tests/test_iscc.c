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

    /* Summary */
    printf("\n%d passed, %d failed\n", tests_passed, tests_failed);
    return tests_failed > 0 ? 1 : 0;
}
