package io.iscc.iscc_lib;

/**
 * Result of decoding an ISCC unit string via {@link IsccLib#isccDecode(String)}.
 *
 * <p>Contains the header fields (maintype, subtype, version, length index) and
 * the raw digest bytes extracted from the ISCC unit.
 */
public class IsccDecodeResult {

    /** MainType enum value (0-7). */
    public final int maintype;

    /** SubType enum value (0-7). */
    public final int subtype;

    /** Version enum value (0-255). */
    public final int version;

    /** Length index from the ISCC header. */
    public final int length;

    /** Raw digest bytes. */
    public final byte[] digest;

    /**
     * Construct a decode result with all header fields and digest.
     *
     * @param maintype MainType enum value
     * @param subtype  SubType enum value
     * @param version  Version enum value
     * @param length   length index from header
     * @param digest   raw digest bytes
     */
    public IsccDecodeResult(int maintype, int subtype, int version, int length, byte[] digest) {
        this.maintype = maintype;
        this.subtype = subtype;
        this.version = version;
        this.length = length;
        this.digest = digest;
    }
}
