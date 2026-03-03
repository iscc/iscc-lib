package io.iscc.iscc_lib;

/**
 * Result of generating an ISCC-SUM code via
 * {@link IsccLib#genSumCodeV0(String, int, boolean, boolean)}.
 *
 * <p>Contains the composite ISCC-CODE string, the BLAKE3 datahash of the file,
 * the file size in bytes, and optionally the individual Data-Code and Instance-Code
 * ISCC strings.
 */
public class SumCodeResult {

    /** Composite ISCC-CODE string (e.g., "ISCC:KAC..."). */
    public final String iscc;

    /** Hex-encoded BLAKE3 multihash of the file (e.g., "1e20..."). */
    public final String datahash;

    /** Byte length of the file. */
    public final long filesize;

    /**
     * Individual ISCC unit strings ({@code [Data-Code, Instance-Code]}), or
     * {@code null} when {@code addUnits} was {@code false}.
     */
    public final String[] units;

    /**
     * Construct a sum code result with all fields.
     *
     * @param iscc     composite ISCC-CODE string
     * @param datahash hex-encoded BLAKE3 multihash
     * @param filesize byte length of the file
     * @param units    individual ISCC unit strings, or {@code null}
     */
    public SumCodeResult(String iscc, String datahash, long filesize, String[] units) {
        this.iscc = iscc;
        this.datahash = datahash;
        this.filesize = filesize;
        this.units = units;
    }
}
