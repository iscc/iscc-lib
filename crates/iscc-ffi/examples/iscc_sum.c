/* iscc_sum.c — ISCC-SUM streaming example.
 *
 * Reads a file in chunks, feeds both DataHasher and InstanceHasher
 * simultaneously, composes the ISCC-CODE, and prints the result.
 *
 * Build (assuming cargo has built iscc-ffi):
 *   gcc -o iscc_sum iscc_sum.c -I ../include -L /path/to/lib -liscc_ffi
 *
 * Usage:
 *   ./iscc_sum <filepath>
 */

#include "iscc.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
    /* Buffer size for reading file chunks (4 MB, matches iscc_io_read_size). */
    const uint32_t buf_size = iscc_io_read_size();

    /* Hash bit length for Data-Code and Instance-Code. */
    const uint32_t bits = 64;

    FILE *fp;
    unsigned char *buf;
    size_t bytes_read;
    struct iscc_FfiDataHasher *dh;
    struct iscc_FfiInstanceHasher *ih;
    char *data_code;
    char *instance_code;
    const char *codes[2];
    char *iscc_code;

    /* --- 1. Validate arguments ---------------------------------------- */
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <filepath>\n", argv[0]);
        return 1;
    }

    /* --- 2. Open the file --------------------------------------------- */
    fp = fopen(argv[1], "rb");
    if (fp == NULL) {
        fprintf(stderr, "Error: cannot open '%s'\n", argv[1]);
        return 1;
    }

    /* --- 3. Allocate read buffer -------------------------------------- */
    buf = (unsigned char *)malloc(buf_size);
    if (buf == NULL) {
        fprintf(stderr, "Error: cannot allocate %u byte buffer\n", buf_size);
        fclose(fp);
        return 1;
    }

    /* --- 4. Create both hashers --------------------------------------- */
    dh = iscc_data_hasher_new();
    ih = iscc_instance_hasher_new();
    if (dh == NULL || ih == NULL) {
        fprintf(stderr, "Error: failed to create hashers\n");
        if (dh != NULL) iscc_data_hasher_free(dh);
        if (ih != NULL) iscc_instance_hasher_free(ih);
        free(buf);
        fclose(fp);
        return 1;
    }

    /* --- 5. Read file in chunks, feed both hashers -------------------- */
    while ((bytes_read = fread(buf, 1, buf_size, fp)) > 0) {
        if (!iscc_data_hasher_update(dh, buf, bytes_read)) {
            fprintf(stderr, "Error: data hasher update failed: %s\n",
                    iscc_last_error());
            iscc_data_hasher_free(dh);
            iscc_instance_hasher_free(ih);
            free(buf);
            fclose(fp);
            return 1;
        }
        if (!iscc_instance_hasher_update(ih, buf, bytes_read)) {
            fprintf(stderr, "Error: instance hasher update failed: %s\n",
                    iscc_last_error());
            iscc_data_hasher_free(dh);
            iscc_instance_hasher_free(ih);
            free(buf);
            fclose(fp);
            return 1;
        }
    }
    if (ferror(fp)) {
        fprintf(stderr, "Error: read failed for '%s'\n", argv[1]);
        iscc_data_hasher_free(dh);
        iscc_instance_hasher_free(ih);
        free(buf);
        fclose(fp);
        return 1;
    }
    fclose(fp);
    free(buf);

    /* --- 6. Finalize both hashers ------------------------------------- */
    data_code = iscc_data_hasher_finalize(dh, bits);
    if (data_code == NULL) {
        fprintf(stderr, "Error: data hasher finalize failed: %s\n",
                iscc_last_error());
        iscc_data_hasher_free(dh);
        iscc_instance_hasher_free(ih);
        return 1;
    }

    instance_code = iscc_instance_hasher_finalize(ih, bits);
    if (instance_code == NULL) {
        fprintf(stderr, "Error: instance hasher finalize failed: %s\n",
                iscc_last_error());
        iscc_free_string(data_code);
        iscc_data_hasher_free(dh);
        iscc_instance_hasher_free(ih);
        return 1;
    }

    /* --- 7. Compose ISCC-CODE from Data-Code + Instance-Code ---------- */
    codes[0] = data_code;
    codes[1] = instance_code;
    iscc_code = iscc_gen_iscc_code_v0(codes, 2, false);
    if (iscc_code == NULL) {
        fprintf(stderr, "Error: ISCC-CODE composition failed: %s\n",
                iscc_last_error());
        iscc_free_string(data_code);
        iscc_free_string(instance_code);
        iscc_data_hasher_free(dh);
        iscc_instance_hasher_free(ih);
        return 1;
    }

    /* --- 8. Print results --------------------------------------------- */
    printf("ISCC-CODE:     %s\n", iscc_code);
    printf("Data-Code:     %s\n", data_code);
    printf("Instance-Code: %s\n", instance_code);

    /* --- 9. Free everything ------------------------------------------- */
    iscc_free_string(iscc_code);
    iscc_free_string(data_code);
    iscc_free_string(instance_code);
    iscc_data_hasher_free(dh);
    iscc_instance_hasher_free(ih);

    return 0;
}
