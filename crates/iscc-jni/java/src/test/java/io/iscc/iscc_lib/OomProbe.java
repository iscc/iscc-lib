/**
 * Child-JVM probe for the heap-exhaustion mapping of the JNI bridge.
 *
 * <p>{@code IsccLibTest.allocationFailureSurfacesAsOutOfMemoryError} runs this class in a child
 * JVM with a small heap ({@code -Xmx16m}) so the test JVM is never starved. The probe asks
 * {@code encodeSeq} for an ISCC-SEQ larger than the whole heap: the {@code String[]} input fits,
 * but the native {@code byte[]} result cannot be allocated. It prints the class name of the
 * exception the native call raised (or {@code NO_EXCEPTION}) on stdout.
 */
package io.iscc.iscc_lib;

import java.util.Arrays;

final class OomProbe {

    /** A 256-bit Data-Code unit: 34 bytes per unit in the ISCC-SEQ result. */
    private static final String UNIT =
            "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI";

    /** 600,000 units x 34 bytes = ~20 MB of result, more than a 16 MB heap can hold. */
    private static final int UNIT_COUNT = 600_000;

    private OomProbe() {}

    /** Build the input outside the probed call, then report what {@code encodeSeq} throws. */
    public static void main(String[] args) {
        String[] units = new String[UNIT_COUNT];
        Arrays.fill(units, UNIT);
        try {
            IsccLib.encodeSeq(units);
            System.out.println("NO_EXCEPTION");
        } catch (Throwable t) {
            System.out.println(t.getClass().getName());
        }
    }
}
