package io.iscc.iscc_lib;

/**
 * Result of generating an ISCC-SUM code via {@link IsccLib#genSumCodeV0(String, int, boolean)}.
 *
 * <p>Contains the composite ISCC-CODE string, the BLAKE3 datahash of the file,
 * and the file size in bytes.
 */
public class SumCodeResult {

    /** Composite ISCC-CODE string (e.g., "ISCC:KAC..."). */
    public final String iscc;

    /** Hex-encoded BLAKE3 multihash of the file (e.g., "1e20..."). */
    public final String datahash;

    /** Byte length of the file. */
    public final long filesize;

    /**
     * Construct a sum code result with all fields.
     *
     * @param iscc     composite ISCC-CODE string
     * @param datahash hex-encoded BLAKE3 multihash
     * @param filesize byte length of the file
     */
    public SumCodeResult(String iscc, String datahash, long filesize) {
        this.iscc = iscc;
        this.datahash = datahash;
        this.filesize = filesize;
    }
}
