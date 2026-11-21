/**
 * Child-JVM probe for the native library fallback of {@link NativeLoader}.
 *
 * <p>The loader tests in {@code IsccLibTest} run this class in a child JVM with an overridden
 * {@code os.name} or {@code os.arch}, or with a broken bundled library on the classpath, so the
 * test JVM keeps its already-loaded library. The probe triggers the native library load through
 * {@link IsccLib#conformanceSelftest()} and prints {@code true}/{@code false}, or the class name
 * and message of whatever the load or the call threw, on stdout. A throwable is followed by one
 * {@code cause: <class>} line if it has a cause and one {@code suppressed: <class>} line per
 * suppressed exception.
 */
package io.iscc.iscc_lib;

final class LoaderProbe {

    private LoaderProbe() {}

    /** Load the native library via {@code IsccLib} and report the selftest result or failure. */
    public static void main(String[] args) {
        try {
            System.out.println(IsccLib.conformanceSelftest());
        } catch (Throwable t) {
            System.out.println(t.getClass().getName() + ": " + t.getMessage());
            if (t.getCause() != null) {
                System.out.println("cause: " + t.getCause().getClass().getName());
            }
            for (Throwable s : t.getSuppressed()) {
                System.out.println("suppressed: " + s.getClass().getName());
            }
        }
    }
}
