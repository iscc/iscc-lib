package io.iscc.iscc_lib;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

/**
 * Automatic native library loader for iscc-jni.
 *
 * <p>Extracts the platform-specific native library from the JAR's
 * {@code META-INF/native/} directory and loads it. Falls back to
 * {@link System#loadLibrary(String)}, which searches {@code java.library.path},
 * on every platform the bundled library does not serve.
 *
 * <p>Loading strategy (tried in order):
 * <ol>
 *   <li>Extract from JAR resource at {@code META-INF/native/{os}-{arch}/{libname}}.
 *       Skipped when the OS or architecture is not recognized.</li>
 *   <li>Fall back to {@code System.loadLibrary("iscc_jni")}. Reached when strategy 1 is
 *       skipped, finds no bundled library, or the bundled library fails to load
 *       (for example a glibc build on musl Linux).</li>
 * </ol>
 *
 * <p>This class is thread-safe. The {@link #load()} method uses synchronized
 * access with a volatile guard to ensure the library is loaded exactly once.
 *
 * @see IsccLib
 */
public final class NativeLoader {

    private static volatile boolean loaded = false;

    private NativeLoader() {
        // Prevent instantiation -- utility class.
    }

    /**
     * Load the iscc_jni native library.
     *
     * <p>First attempts to extract the library from the JAR's
     * {@code META-INF/native/} directory when the OS and architecture are recognized.
     * If there is no bundled binary for this platform, or the bundled binary fails to
     * extract or load, falls back to {@link System#loadLibrary(String)}.
     *
     * @throws UnsatisfiedLinkError if the library cannot be loaded by either method; its cause
     *     is the {@code System.loadLibrary} error and a bundled-library failure is attached as a
     *     suppressed exception
     */
    public static synchronized void load() {
        if (loaded) {
            return;
        }

        String os = detectOs();
        String arch = detectArch();
        String resourcePath = null;
        Throwable bundledError = null;

        // Strategy 1: Extract from JAR resource (only for a recognized platform)
        if (os != null && arch != null) {
            resourcePath = "META-INF/native/" + os + "-" + arch + "/" + libraryFileName(os);
            try {
                if (loadFromResource(resourcePath)) {
                    loaded = true;
                    return;
                }
            } catch (IOException | UnsatisfiedLinkError e) {
                bundledError = e;
            }
        }

        // Strategy 2: System.loadLibrary (java.library.path)
        try {
            System.loadLibrary("iscc_jni");
            loaded = true;
        } catch (UnsatisfiedLinkError e) {
            UnsatisfiedLinkError error = new UnsatisfiedLinkError(failureMessage(resourcePath));
            error.initCause(e);
            if (bundledError != null) {
                error.addSuppressed(bundledError);
            }
            throw error;
        }
    }

    /**
     * Build the message for a failure of both loading strategies.
     *
     * @param resourcePath the JAR resource tried by strategy 1, or {@code null} if the OS or
     *     architecture was not recognized and strategy 1 was skipped
     * @return a message naming what was tried and the raw platform properties
     */
    static String failureMessage(String resourcePath) {
        String platform = "os.name=" + System.getProperty("os.name")
                + ", os.arch=" + System.getProperty("os.arch");
        String tried = resourcePath == null
                ? "No bundled library exists for " + platform + "; tried System.loadLibrary(\"iscc_jni\"). "
                : "Tried JAR resource '/" + resourcePath + "' and System.loadLibrary(\"iscc_jni\") for "
                        + platform + ". ";
        return "Failed to load iscc_jni native library. " + tried
                + "Ensure the native library is bundled in the JAR or available on java.library.path.";
    }

    /**
     * Detect the operating system and normalize to a canonical name.
     *
     * @return one of {@code "linux"}, {@code "macos"}, or {@code "windows"}, or {@code null} if
     *     the OS is not recognized
     */
    static String detectOs() {
        String osName = System.getProperty("os.name", "").toLowerCase();
        if (osName.startsWith("linux")) {
            return "linux";
        } else if (osName.startsWith("mac") || osName.contains("darwin")) {
            return "macos";
        } else if (osName.startsWith("windows")) {
            return "windows";
        }
        return null;
    }

    /**
     * Detect the CPU architecture and normalize to a canonical name.
     *
     * @return one of {@code "x86_64"} or {@code "aarch64"}, or {@code null} if the architecture
     *     is not recognized
     */
    static String detectArch() {
        String osArch = System.getProperty("os.arch", "").toLowerCase();
        if (osArch.equals("amd64") || osArch.equals("x86_64")) {
            return "x86_64";
        } else if (osArch.equals("aarch64") || osArch.equals("arm64")) {
            return "aarch64";
        }
        return null;
    }

    /**
     * Return the platform-specific library filename.
     *
     * @param os normalized OS name from {@link #detectOs()}, never {@code null}
     * @return library filename (e.g., {@code "libiscc_jni.so"})
     * @throws IllegalArgumentException if {@code os} is not a name {@link #detectOs()} returns
     */
    static String libraryFileName(String os) {
        switch (os) {
            case "linux":
                return "libiscc_jni.so";
            case "macos":
                return "libiscc_jni.dylib";
            case "windows":
                return "iscc_jni.dll";
            default:
                throw new IllegalArgumentException("Not a normalized OS name: " + os);
        }
    }

    /**
     * Attempt to load the native library from a JAR resource.
     *
     * <p>Extracts the resource to a temporary file in a unique temporary
     * directory, loads it via {@link System#load(String)}, and marks both
     * the file and directory for deletion on JVM exit.
     *
     * @param resourcePath resource path relative to classpath root
     * @return {@code true} if the library was loaded, {@code false} if the resource was not found
     * @throws IOException if extraction fails after the resource is found
     * @throws UnsatisfiedLinkError if the extracted library fails to load
     */
    private static boolean loadFromResource(String resourcePath) throws IOException {
        InputStream stream = NativeLoader.class.getResourceAsStream("/" + resourcePath);
        if (stream == null) {
            return false;
        }

        try (stream) {
            Path tempDir = Files.createTempDirectory("iscc-jni-");
            String libName = resourcePath.substring(resourcePath.lastIndexOf('/') + 1);
            Path tempFile = tempDir.resolve(libName);

            Files.copy(stream, tempFile, StandardCopyOption.REPLACE_EXISTING);

            // The JVM deletes in reverse order of registration: the directory is
            // registered first so that it is empty by the time its turn comes.
            tempDir.toFile().deleteOnExit();
            tempFile.toFile().deleteOnExit();

            System.load(tempFile.toAbsolutePath().toString());
            return true;
        }
    }
}
