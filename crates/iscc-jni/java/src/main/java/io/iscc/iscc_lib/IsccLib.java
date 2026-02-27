package io.iscc.iscc_lib;

/**
 * Low-level JNI interface to iscc-lib (ISO 24138:2024 ISCC).
 *
 * <p>All methods are static JNI calls into the Rust iscc-jni shared library.
 * The library is loaded automatically via {@link NativeLoader#load()}, which
 * first tries to extract a platform-specific binary from the JAR's
 * {@code META-INF/native/} directory, then falls back to the standard
 * JVM library loading mechanism for development and CI environments.
 *
 * <p>Methods that accept invalid input throw {@link IllegalArgumentException}.
 * Streaming hashers use opaque {@code long} handles for memory management --
 * callers must call the corresponding {@code *Free} method to release resources.
 *
 * @see NativeLoader
 * @see <a href="https://iscc.io">ISCC Foundation</a>
 */
public class IsccLib {

    /** Maximum UTF-8 byte length for the name parameter in gen_meta_code_v0. */
    public static final int META_TRIM_NAME = 128;

    /** Maximum UTF-8 byte length for the description parameter in gen_meta_code_v0. */
    public static final int META_TRIM_DESCRIPTION = 4096;

    /** Default read buffer size for streaming I/O (4 MB). */
    public static final int IO_READ_SIZE = 4_194_304;

    /** N-gram window size for text similarity hashing. */
    public static final int TEXT_NGRAM_SIZE = 13;

    static {
        NativeLoader.load();
    }

    private IsccLib() {
        // Prevent instantiation -- all methods are static.
    }

    // ── Conformance ─────────────────────────────────────────────────────────

    /**
     * Run all ISCC conformance tests against vendored test vectors.
     *
     * @return {@code true} if all tests pass, {@code false} otherwise
     */
    public static native boolean conformanceSelftest();

    // ── Gen functions ───────────────────────────────────────────────────────

    /**
     * Generate a Meta-Code from name and optional metadata.
     *
     * @param name        content title or name (required)
     * @param description content description (nullable)
     * @param meta        JSON metadata string (nullable)
     * @param bits        hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string (e.g., "ISCC:AAA...")
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genMetaCodeV0(String name, String description, String meta, int bits);

    /**
     * Generate a Text-Code from plain text content.
     *
     * @param text plain text content
     * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genTextCodeV0(String text, int bits);

    /**
     * Generate an Image-Code from 1024 grayscale pixel bytes.
     *
     * @param pixels 32x32 grayscale pixel array (1024 bytes)
     * @param bits   hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genImageCodeV0(byte[] pixels, int bits);

    /**
     * Generate an Audio-Code from a Chromaprint feature vector.
     *
     * @param cv   signed 32-bit Chromaprint features
     * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genAudioCodeV0(int[] cv, int bits);

    /**
     * Generate a Video-Code from frame signature data.
     *
     * @param frameSigs array of MPEG-7 frame signature vectors (each int[])
     * @param bits      hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genVideoCodeV0(int[][] frameSigs, int bits);

    /**
     * Generate a Mixed-Code from multiple Content-Code strings.
     *
     * @param codes array of ISCC Content-Code strings
     * @param bits  hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genMixedCodeV0(String[] codes, int bits);

    /**
     * Generate a Data-Code from raw byte data.
     *
     * @param data raw byte content
     * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genDataCodeV0(byte[] data, int bits);

    /**
     * Generate an Instance-Code from raw byte data.
     *
     * @param data raw byte content
     * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genInstanceCodeV0(byte[] data, int bits);

    /**
     * Generate a composite ISCC-CODE from individual unit codes.
     *
     * @param codes array of ISCC unit code strings
     * @param wide  {@code true} for 256-bit output, {@code false} for 128-bit
     * @return ISCC string
     * @throws IllegalArgumentException on invalid input
     */
    public static native String genIsccCodeV0(String[] codes, boolean wide);

    // ── Text utilities ──────────────────────────────────────────────────────

    /**
     * Clean and normalize text for display.
     *
     * <p>Applies NFKC normalization, removes control characters (except newlines),
     * normalizes CRLF to LF, collapses consecutive empty lines, and strips
     * leading/trailing whitespace.
     *
     * @param text input text
     * @return cleaned text
     */
    public static native String textClean(String text);

    /**
     * Remove newlines and collapse whitespace to single spaces.
     *
     * @param text input text
     * @return single-line normalized text
     */
    public static native String textRemoveNewlines(String text);

    /**
     * Trim text so its UTF-8 encoded size does not exceed {@code nbytes}.
     *
     * <p>Multi-byte characters that would be split are dropped entirely.
     * Leading/trailing whitespace is stripped from the result.
     *
     * @param text   input text
     * @param nbytes maximum UTF-8 byte length
     * @return trimmed text
     */
    public static native String textTrim(String text, int nbytes);

    /**
     * Normalize and simplify text for similarity hashing.
     *
     * <p>Applies NFD normalization, lowercasing, removes whitespace and characters
     * in Unicode categories C, M, and P, then recombines with NFKC normalization.
     *
     * @param text input text
     * @return collapsed text
     */
    public static native String textCollapse(String text);

    // ── Encoding ────────────────────────────────────────────────────────────

    /**
     * Encode bytes as base64url (RFC 4648 section 5, no padding).
     *
     * @param data raw bytes to encode
     * @return URL-safe base64 encoded string without padding
     */
    public static native String encodeBase64(byte[] data);

    // ── Encoding ── (additional) ────────────────────────────────────────────

    /**
     * Convert a JSON string to a data URL with base64 encoding.
     *
     * <p>Uses {@code application/ld+json} media type when the JSON contains an
     * {@code @context} key, otherwise {@code application/json}.
     *
     * @param json valid JSON string
     * @return data URL string (e.g., "data:application/json;base64,...")
     * @throws IllegalArgumentException on invalid JSON input
     */
    public static native String jsonToDataUrl(String json);

    // ── Codec ───────────────────────────────────────────────────────────────

    /**
     * Encode header fields and a raw digest into a base32-encoded ISCC unit string.
     *
     * @param mtype     MainType enum value (0-255)
     * @param stype     SubType enum value (0-255)
     * @param version   Version enum value (0-255)
     * @param bitLength digest bit length (must be a multiple of 32)
     * @param digest    raw digest bytes (length must be >= bitLength / 8)
     * @return base32-encoded ISCC unit string (without "ISCC:" prefix)
     * @throws IllegalArgumentException on invalid input
     */
    public static native String encodeComponent(int mtype, int stype, int version, int bitLength, byte[] digest);

    /**
     * Decode an ISCC unit string into its header components and raw digest.
     *
     * <p>Strips an optional "ISCC:" prefix before decoding.
     *
     * @param isccUnit ISCC unit string (with or without "ISCC:" prefix)
     * @return decoded result with maintype, subtype, version, length, and digest
     * @throws IllegalArgumentException on invalid input
     */
    public static native IsccDecodeResult isccDecode(String isccUnit);

    /**
     * Decompose a composite ISCC-CODE into individual ISCC-UNITs.
     *
     * @param isccCode composite ISCC-CODE string
     * @return array of base32-encoded ISCC-UNIT strings (without prefix)
     * @throws IllegalArgumentException on invalid input
     */
    public static native String[] isccDecompose(String isccCode);

    // ── Sliding window ──────────────────────────────────────────────────────

    /**
     * Generate sliding window n-grams from a string.
     *
     * @param seq   input string
     * @param width window width in Unicode characters (must be >= 2)
     * @return array of overlapping substrings
     * @throws IllegalArgumentException if width is less than 2
     */
    public static native String[] slidingWindow(String seq, int width);

    // ── Algorithm primitives ────────────────────────────────────────────────

    /**
     * Compute a SimHash from a sequence of equal-length hash digests.
     *
     * @param hashDigests array of equal-length byte arrays
     * @return similarity-preserving hash digest
     * @throws IllegalArgumentException on invalid input
     */
    public static native byte[] algSimhash(byte[][] hashDigests);

    /**
     * Compute a 256-bit MinHash digest from 32-bit integer features.
     *
     * @param features array of 32-bit feature values
     * @return 32-byte MinHash digest
     */
    public static native byte[] algMinhash256(int[] features);

    /**
     * Split data into content-defined chunks using gear rolling hash.
     *
     * @param data         raw byte data to split
     * @param utf32        if {@code true}, align cut points to 4-byte boundaries
     * @param avgChunkSize target average chunk size (default 1024)
     * @return array of byte array chunks
     */
    public static native byte[][] algCdcChunks(byte[] data, boolean utf32, int avgChunkSize);

    /**
     * Compute a similarity-preserving hash from video frame signatures.
     *
     * @param frameSigs array of MPEG-7 frame signature vectors (each int[])
     * @param bits      output bit length
     * @return byte array of length {@code bits / 8}
     * @throws IllegalArgumentException if input is empty
     */
    public static native byte[] softHashVideoV0(int[][] frameSigs, int bits);

    // ── Streaming hashers ───────────────────────────────────────────────────

    /**
     * Create a new streaming Data-Code hasher.
     *
     * <p>Returns an opaque handle. The caller must eventually call
     * {@link #dataHasherFree(long)} to release the memory.
     *
     * @return opaque handle to the hasher
     */
    public static native long dataHasherNew();

    /**
     * Push data into a streaming DataHasher.
     *
     * @param ptr  opaque handle from {@link #dataHasherNew()}
     * @param data byte data to feed into the hasher
     * @throws IllegalArgumentException if the hasher has been finalized
     */
    public static native void dataHasherUpdate(long ptr, byte[] data);

    /**
     * Finalize a streaming DataHasher and return an ISCC string.
     *
     * <p>Consumes the inner hasher state. After this call, subsequent
     * {@code update} or {@code finalize} calls will throw. The caller must
     * still call {@link #dataHasherFree(long)} to release the wrapper.
     *
     * @param ptr  opaque handle from {@link #dataHasherNew()}
     * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException if already finalized or on error
     */
    public static native String dataHasherFinalize(long ptr, int bits);

    /**
     * Free a DataHasher previously created by {@link #dataHasherNew()}.
     *
     * <p>Zero/null handle is a no-op. Each handle must be freed exactly once.
     *
     * @param ptr opaque handle from {@link #dataHasherNew()}
     */
    public static native void dataHasherFree(long ptr);

    /**
     * Create a new streaming Instance-Code hasher.
     *
     * <p>Returns an opaque handle. The caller must eventually call
     * {@link #instanceHasherFree(long)} to release the memory.
     *
     * @return opaque handle to the hasher
     */
    public static native long instanceHasherNew();

    /**
     * Push data into a streaming InstanceHasher.
     *
     * @param ptr  opaque handle from {@link #instanceHasherNew()}
     * @param data byte data to feed into the hasher
     * @throws IllegalArgumentException if the hasher has been finalized
     */
    public static native void instanceHasherUpdate(long ptr, byte[] data);

    /**
     * Finalize a streaming InstanceHasher and return an ISCC string.
     *
     * <p>Consumes the inner hasher state. After this call, subsequent
     * {@code update} or {@code finalize} calls will throw. The caller must
     * still call {@link #instanceHasherFree(long)} to release the wrapper.
     *
     * @param ptr  opaque handle from {@link #instanceHasherNew()}
     * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
     * @return ISCC string
     * @throws IllegalArgumentException if already finalized or on error
     */
    public static native String instanceHasherFinalize(long ptr, int bits);

    /**
     * Free an InstanceHasher previously created by {@link #instanceHasherNew()}.
     *
     * <p>Zero/null handle is a no-op. Each handle must be freed exactly once.
     *
     * @param ptr opaque handle from {@link #instanceHasherNew()}
     */
    public static native void instanceHasherFree(long ptr);
}
