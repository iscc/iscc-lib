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
 * {@link System#loadLibrary(String)} for development and CI environments
 * where the library is on {@code java.library.path}.
 *
 * <p>Loading strategy (tried in order):
 * <ol>
 *   <li>Extract from JAR resource at {@code META-INF/native/{os}-{arch}/{libname}}</li>
 *   <li>Fall back to {@code System.loadLibrary("iscc_jni")}</li>
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
     * {@code META-INF/native/} directory. If that fails (e.g., no bundled
     * binary for this platform), falls back to {@link System#loadLibrary(String)}.
     *
     * @throws UnsatisfiedLinkError if the library cannot be loaded by either method
     */
    public static synchronized void load() {
        if (loaded) {
            return;
        }

        String os = detectOs();
        String arch = detectArch();
        String libName = libraryFileName(os);
        String resourcePath = "META-INF/native/" + os + "-" + arch + "/" + libName;

        // Strategy 1: Extract from JAR resource
        try {
            if (loadFromResource(resourcePath)) {
                loaded = true;
                return;
            }
        } catch (IOException e) {
            // Fall through to System.loadLibrary
        }

        // Strategy 2: System.loadLibrary (dev/CI with java.library.path)
        try {
            System.loadLibrary("iscc_jni");
            loaded = true;
            return;
        } catch (UnsatisfiedLinkError e) {
            throw new UnsatisfiedLinkError(
                    "Failed to load iscc_jni native library. "
                            + "Tried JAR resource '/"
                            + resourcePath
                            + "' and System.loadLibrary(\"iscc_jni\"). "
                            + "Detected OS="
                            + os
                            + ", arch="
                            + arch
                            + ". "
                            + "Ensure the native library is bundled in the JAR or available on java.library.path.");
        }
    }

    /**
     * Detect the operating system and normalize to a canonical name.
     *
     * @return one of {@code "linux"}, {@code "macos"}, or {@code "windows"}
     * @throws UnsatisfiedLinkError if the OS is not recognized
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
        throw new UnsatisfiedLinkError("Unsupported OS: " + System.getProperty("os.name"));
    }

    /**
     * Detect the CPU architecture and normalize to a canonical name.
     *
     * @return one of {@code "x86_64"} or {@code "aarch64"}
     * @throws UnsatisfiedLinkError if the architecture is not recognized
     */
    static String detectArch() {
        String osArch = System.getProperty("os.arch", "").toLowerCase();
        if (osArch.equals("amd64") || osArch.equals("x86_64")) {
            return "x86_64";
        } else if (osArch.equals("aarch64") || osArch.equals("arm64")) {
            return "aarch64";
        }
        throw new UnsatisfiedLinkError("Unsupported architecture: " + System.getProperty("os.arch"));
    }

    /**
     * Return the platform-specific library filename.
     *
     * @param os normalized OS name from {@link #detectOs()}
     * @return library filename (e.g., {@code "libiscc_jni.so"})
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
                throw new UnsatisfiedLinkError("Unsupported OS: " + os);
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

            tempFile.toFile().deleteOnExit();
            tempDir.toFile().deleteOnExit();

            System.load(tempFile.toAbsolutePath().toString());
            return true;
        }
    }
}
