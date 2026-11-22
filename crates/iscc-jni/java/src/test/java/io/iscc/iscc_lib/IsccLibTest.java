/**
 * Conformance tests for the gen_*_v0 JNI functions against data.json vectors.
 * genSumCodeV0 has no vectors and is checked against its Data-Code + Instance-Code composition.
 *
 * <p>Mirrors the Node.js conformance tests in crates/iscc-napi/__tests__/conformance.test.mjs.
 * Uses JUnit 6 {@code @TestFactory} with {@code DynamicTest} for data-driven test names
 * matching the JSON keys (e.g., {@code test_0001_title_only}).
 */
package io.iscc.iscc_lib;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.io.File;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Comparator;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Stream;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;
import org.junit.jupiter.api.function.Executable;

class IsccLibTest {

    private static JsonObject data;

    /** Load the shared data.json test vectors. */
    @BeforeAll
    static void loadTestData() throws Exception {
        String json = Files.readString(Path.of("../../iscc-lib/tests/data.json"));
        data = JsonParser.parseString(json).getAsJsonObject();
    }

    // ── Helpers ──────────────────────────────────────────────────────────────

    /**
     * Convert a meta JSON value to the string argument for genMetaCodeV0.
     *
     * <p>JsonObject is serialized with sorted keys. JsonNull/null becomes null.
     * JsonPrimitive string is returned as-is.
     */
    private static String prepareMetaArg(JsonElement meta) {
        if (meta == null || meta.isJsonNull()) {
            return null;
        }
        if (meta.isJsonPrimitive() && meta.getAsJsonPrimitive().isString()) {
            return meta.getAsString();
        }
        if (meta.isJsonObject()) {
            // Sorted keys via TreeMap
            TreeMap<String, JsonElement> sorted = new TreeMap<>();
            for (Map.Entry<String, JsonElement> entry : meta.getAsJsonObject().entrySet()) {
                sorted.put(entry.getKey(), entry.getValue());
            }
            JsonObject obj = new JsonObject();
            for (Map.Entry<String, JsonElement> entry : sorted.entrySet()) {
                obj.add(entry.getKey(), entry.getValue());
            }
            return obj.toString();
        }
        throw new IllegalArgumentException("unexpected meta type: " + meta);
    }

    /**
     * Decode a "stream:&lt;hex&gt;" string to byte[].
     *
     * <p>Empty hex after prefix yields an empty byte array.
     */
    private static byte[] decodeStream(String streamStr) {
        String hex = streamStr.replaceFirst("^stream:", "");
        if (hex.isEmpty()) {
            return new byte[0];
        }
        return HexFormat.of().parseHex(hex);
    }

    /**
     * Convert a JsonArray of integers (0-255) to byte[].
     *
     * <p>Each integer is cast to byte (Java bytes are signed, values &gt; 127
     * wrap correctly).
     */
    private static byte[] jsonArrayToBytes(JsonArray arr) {
        byte[] result = new byte[arr.size()];
        for (int i = 0; i < arr.size(); i++) {
            result[i] = (byte) arr.get(i).getAsInt();
        }
        return result;
    }

    /**
     * Convert a JsonArray of integers to int[].
     */
    private static int[] jsonArrayToInts(JsonArray arr) {
        int[] result = new int[arr.size()];
        for (int i = 0; i < arr.size(); i++) {
            result[i] = arr.get(i).getAsInt();
        }
        return result;
    }

    /**
     * Convert a JsonArray of JsonArrays of integers to int[][].
     */
    private static int[][] jsonArrayToInts2d(JsonArray arr) {
        int[][] result = new int[arr.size()][];
        for (int i = 0; i < arr.size(); i++) {
            result[i] = jsonArrayToInts(arr.get(i).getAsJsonArray());
        }
        return result;
    }

    /**
     * Convert a JsonArray of strings to String[].
     */
    private static String[] jsonArrayToStrings(JsonArray arr) {
        String[] result = new String[arr.size()];
        for (int i = 0; i < arr.size(); i++) {
            result[i] = arr.get(i).getAsString();
        }
        return result;
    }

    // ── gen_meta_code_v0 ─────────────────────────────────────────────────────

    /** Test all gen_meta_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genMetaCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_meta_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                String nameArg = inputs.get(0).getAsString();
                String description = inputs.get(1).getAsString();
                if (description.isEmpty()) {
                    description = null;
                }
                String meta = prepareMetaArg(inputs.get(2));
                int bits = inputs.get(3).getAsInt();

                String result = IsccLib.genMetaCodeV0(nameArg, description, meta, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_text_code_v0 ─────────────────────────────────────────────────────

    /** Test all gen_text_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genTextCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_text_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                String text = inputs.get(0).getAsString();
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genTextCodeV0(text, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_image_code_v0 ────────────────────────────────────────────────────

    /** Test all gen_image_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genImageCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_image_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                byte[] pixels = jsonArrayToBytes(inputs.get(0).getAsJsonArray());
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genImageCodeV0(pixels, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_audio_code_v0 ────────────────────────────────────────────────────

    /** Test all gen_audio_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genAudioCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_audio_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                int[] cv = jsonArrayToInts(inputs.get(0).getAsJsonArray());
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genAudioCodeV0(cv, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_video_code_v0 ────────────────────────────────────────────────────

    /** Test all gen_video_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genVideoCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_video_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                int[][] frameSigs = jsonArrayToInts2d(inputs.get(0).getAsJsonArray());
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genVideoCodeV0(frameSigs, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_mixed_code_v0 ────────────────────────────────────────────────────

    /** Test all gen_mixed_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genMixedCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_mixed_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                String[] codes = jsonArrayToStrings(inputs.get(0).getAsJsonArray());
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genMixedCodeV0(codes, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_data_code_v0 ─────────────────────────────────────────────────────

    /** Test all gen_data_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genDataCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_data_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                byte[] bytes = decodeStream(inputs.get(0).getAsString());
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genDataCodeV0(bytes, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_instance_code_v0 ─────────────────────────────────────────────────

    /** Test all gen_instance_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genInstanceCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_instance_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                byte[] bytes = decodeStream(inputs.get(0).getAsString());
                int bits = inputs.get(1).getAsInt();

                String result = IsccLib.genInstanceCodeV0(bytes, bits);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_iscc_code_v0 ─────────────────────────────────────────────────────

    /** Test all gen_iscc_code_v0 vectors. */
    @TestFactory
    Collection<DynamicTest> genIsccCodeV0() {
        JsonObject section = data.getAsJsonObject("gen_iscc_code_v0");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                JsonArray inputs = tc.getAsJsonArray("inputs");
                String[] codes = jsonArrayToStrings(inputs.get(0).getAsJsonArray());

                String result = IsccLib.genIsccCodeV0(codes, false);
                String expected = tc.getAsJsonObject("outputs").get("iscc").getAsString();
                assertEquals(expected, result);
            }));
        }
        return tests;
    }

    // ── gen_sum_code_v0 ──────────────────────────────────────────────────────

    /**
     * Verify genSumCodeV0 produces the same ISCC as composing Data-Code and
     * Instance-Code manually, and returns correct filesize.
     */
    @Test
    void genSumCodeV0Equivalence() throws Exception {
        byte[] content = "Hello ISCC from Java JNI!".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        java.io.File tmp = java.io.File.createTempFile("iscc-jni-sum-", ".bin");
        tmp.deleteOnExit();
        java.nio.file.Files.write(tmp.toPath(), content);

        SumCodeResult result = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 64, false, false);

        // Compose manually: genDataCodeV0 + genInstanceCodeV0 -> genIsccCodeV0
        String dataCode = IsccLib.genDataCodeV0(content, 64);
        String instanceCode = IsccLib.genInstanceCodeV0(content, 64);
        String composedIscc = IsccLib.genIsccCodeV0(new String[] {dataCode, instanceCode}, false);

        assertEquals(composedIscc, result.iscc, "ISCC should match manual composition");
        assertEquals(content.length, result.filesize, "filesize should match content length");
    }

    /** Verify genSumCodeV0 returns non-null, non-empty fields. */
    @Test
    void genSumCodeV0ResultFields() throws Exception {
        byte[] content = new byte[] {1, 2, 3, 4, 5, 6, 7, 8};
        java.io.File tmp = java.io.File.createTempFile("iscc-jni-sum-fields-", ".bin");
        tmp.deleteOnExit();
        java.nio.file.Files.write(tmp.toPath(), content);

        SumCodeResult result = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 64, false, false);

        assertNotNull(result.iscc, "iscc should not be null");
        assertTrue(result.iscc.length() > 0, "iscc should not be empty");
        assertNotNull(result.datahash, "datahash should not be null");
        assertTrue(result.datahash.length() > 0, "datahash should not be empty");
        assertTrue(result.filesize > 0, "filesize should be positive");
    }

    /** Verify genSumCodeV0 throws IllegalArgumentException for non-existent file. */
    @Test
    void genSumCodeV0NonExistentFile() {
        assertThrows(
                IllegalArgumentException.class,
                () -> IsccLib.genSumCodeV0("/non/existent/path/to/file.bin", 64, false, false));
    }

    /** Verify narrow and wide modes produce different ISCC but same datahash and filesize. */
    @Test
    void genSumCodeV0WideMode() throws Exception {
        byte[] content = "Wide mode test content for ISCC-SUM".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        java.io.File tmp = java.io.File.createTempFile("iscc-jni-sum-wide-", ".bin");
        tmp.deleteOnExit();
        java.nio.file.Files.write(tmp.toPath(), content);

        SumCodeResult narrow = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 128, false, false);
        SumCodeResult wide = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 128, true, false);

        assertTrue(!narrow.iscc.equals(wide.iscc), "narrow and wide ISCC should differ");
        assertEquals(narrow.datahash, wide.datahash, "datahash should be same regardless of wide");
        assertEquals(narrow.filesize, wide.filesize, "filesize should be same regardless of wide");
    }

    /** Verify genSumCodeV0 returns non-null units array with 2 elements when addUnits=true. */
    @Test
    void genSumCodeV0UnitsEnabled() throws Exception {
        byte[] content = "Units enabled test".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        java.io.File tmp = java.io.File.createTempFile("iscc-jni-sum-units-", ".bin");
        tmp.deleteOnExit();
        java.nio.file.Files.write(tmp.toPath(), content);

        SumCodeResult result = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 64, false, true);

        assertNotNull(result.units, "units should not be null when addUnits=true");
        assertEquals(2, result.units.length, "units should contain 2 elements");
        assertTrue(result.units[0].startsWith("ISCC:"), "units[0] should start with ISCC:");
        assertTrue(result.units[1].startsWith("ISCC:"), "units[1] should start with ISCC:");
    }

    /** Verify genSumCodeV0 returns null units when addUnits=false. */
    @Test
    void genSumCodeV0UnitsDisabled() throws Exception {
        byte[] content = "Units disabled test".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        java.io.File tmp = java.io.File.createTempFile("iscc-jni-sum-nounits-", ".bin");
        tmp.deleteOnExit();
        java.nio.file.Files.write(tmp.toPath(), content);

        SumCodeResult result = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 64, false, false);

        assertNull(result.units, "units should be null when addUnits=false");
    }

    /** Verify genSumCodeV0 units contain Data-Code and Instance-Code ISCC strings. */
    @Test
    void genSumCodeV0UnitsContent() throws Exception {
        byte[] content = "Units content verification".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        java.io.File tmp = java.io.File.createTempFile("iscc-jni-sum-unitscontent-", ".bin");
        tmp.deleteOnExit();
        java.nio.file.Files.write(tmp.toPath(), content);

        SumCodeResult result = IsccLib.genSumCodeV0(tmp.getAbsolutePath(), 64, false, true);

        assertNotNull(result.units, "units should not be null");
        assertTrue(result.units[0].startsWith("ISCC:GA"),
                "units[0] should be a Data-Code (starts with ISCC:GA)");
        assertTrue(result.units[1].startsWith("ISCC:IA"),
                "units[1] should be an Instance-Code (starts with ISCC:IA)");
    }

    // ── gen_iscc_id_v1 (experimental) ────────────────────────────────────────

    /** Verify genIsccIdV1 mints the frozen cross-surface golden value. */
    @Test
    void genIsccIdV1Golden() {
        assertEquals("ISCC:MAIGHFECJMOPMIAB", IsccLib.genIsccIdV1(1751831876325218L, 1, 0));
    }

    /** Verify genIsccIdV1 throws IllegalArgumentException for an out-of-range realm. */
    @Test
    void genIsccIdV1InvalidRealm() {
        assertThrows(
                IllegalArgumentException.class, () -> IsccLib.genIsccIdV1(1751831876325218L, 1, 2));
    }

    /** Verify a minted ISCC-IDv1 round-trips through isccDecode at Version 1. */
    @Test
    void genIsccIdV1DecodeRoundtrip() {
        String iscc = IsccLib.genIsccIdV1(1751831876325218L, 1, 0);
        IsccDecodeResult decoded = IsccLib.isccDecode(iscc);
        assertEquals(6, decoded.maintype, "maintype should be 6 (ISCC-ID)");
        assertEquals(0, decoded.subtype, "subtype should be 0 (realm)");
        assertEquals(1, decoded.version, "version should be 1 (IDv1)");
    }

    /**
     * Verify multi-invalid inputs report the normative winning field (ts → hub → realm order, first
     * failing check wins). An out-of-range timestamp wins over an out-of-range hub; an out-of-range
     * hub wins over an out-of-range realm.
     */
    @Test
    void genIsccIdV1ValidationOrder() {
        // timestamp = 2^52 (out of range), hub = 65536 (out of range): timestamp must win.
        IllegalArgumentException tsFirst =
                assertThrows(
                        IllegalArgumentException.class,
                        () -> IsccLib.genIsccIdV1(1L << 52, 65536, 0));
        assertTrue(
                tsFirst.getMessage().contains("timestamp"),
                "timestamp check must win over hub: " + tsFirst.getMessage());

        // timestamp valid, hub = 4096 (out of range), realm = 256 (out of range): hub must win.
        IllegalArgumentException hubFirst =
                assertThrows(
                        IllegalArgumentException.class,
                        () -> IsccLib.genIsccIdV1(0L, 4096, 256));
        assertTrue(
                hubFirst.getMessage().contains("hub"),
                "hub check must win over realm: " + hubFirst.getMessage());
    }

    // ── Negative jint validation ─────────────────────────────────────────────

    /** Assert that {@code call} throws IllegalArgumentException naming a negative {@code param}. */
    private static void assertRejectsNegative(String param, Executable call) {
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class, call);
        assertEquals(param + " must be non-negative", e.getMessage());
    }

    /** Verify textTrim throws IllegalArgumentException for negative nbytes. */
    @Test
    void textTrimNegativeNbytes() {
        assertRejectsNegative("nbytes", () -> IsccLib.textTrim("hello", -1));
    }

    /** Verify slidingWindow throws IllegalArgumentException for negative width. */
    @Test
    void slidingWindowNegativeWidth() {
        assertRejectsNegative("width", () -> IsccLib.slidingWindow("hello", -1));
    }

    /** Verify every gen function rejects negative bits before the unsigned cast. */
    @Test
    void genFunctionsRejectNegativeBits() {
        int[][] frames = {{0, 1, 2}};
        assertRejectsNegative("bits", () -> IsccLib.genMetaCodeV0("name", null, null, -1));
        assertRejectsNegative("bits", () -> IsccLib.genTextCodeV0("text", -1));
        assertRejectsNegative("bits", () -> IsccLib.genImageCodeV0(new byte[1024], -1));
        assertRejectsNegative("bits", () -> IsccLib.genAudioCodeV0(new int[] {1, 2}, -1));
        assertRejectsNegative("bits", () -> IsccLib.genVideoCodeV0(frames, -1));
        assertRejectsNegative("bits", () -> IsccLib.genMixedCodeV0(new String[0], -1));
        assertRejectsNegative("bits", () -> IsccLib.genDataCodeV0(new byte[] {1}, -1));
        assertRejectsNegative("bits", () -> IsccLib.genInstanceCodeV0(new byte[] {1}, -1));
        assertRejectsNegative("bits", () -> IsccLib.genSumCodeV0("unused", -1, false, false));
        assertRejectsNegative("bits", () -> IsccLib.softHashVideoV0(frames, -1));
    }

    /** Assert that a valid finalize after a rejected one reports the finalized state. */
    private static void assertConsumed(String name, Executable finalize64) {
        IllegalStateException e = assertThrows(IllegalStateException.class, finalize64);
        assertEquals(name + " already finalized", e.getMessage());
    }

    /** Verify dataHasherFinalize rejecting negative bits still consumes the hasher. */
    @Test
    void dataHasherFinalizeNegativeBitsConsumes() {
        long ptr = IsccLib.dataHasherNew();
        try {
            assertRejectsNegative("bits", () -> IsccLib.dataHasherFinalize(ptr, -1));
            assertConsumed("DataHasher", () -> IsccLib.dataHasherFinalize(ptr, 64));
        } finally {
            IsccLib.dataHasherFree(ptr);
        }
    }

    /** Verify dataHasherFinalize rejected by the core (bits 13) still consumes the hasher. */
    @Test
    void dataHasherFinalizeCoreRejectedBitsConsumes() {
        long ptr = IsccLib.dataHasherNew();
        try {
            assertThrows(IllegalArgumentException.class, () -> IsccLib.dataHasherFinalize(ptr, 13));
            assertConsumed("DataHasher", () -> IsccLib.dataHasherFinalize(ptr, 64));
        } finally {
            IsccLib.dataHasherFree(ptr);
        }
    }

    /** Verify instanceHasherFinalize rejecting negative bits still consumes the hasher. */
    @Test
    void instanceHasherFinalizeNegativeBitsConsumes() {
        long ptr = IsccLib.instanceHasherNew();
        try {
            assertRejectsNegative("bits", () -> IsccLib.instanceHasherFinalize(ptr, -1));
            assertConsumed("InstanceHasher", () -> IsccLib.instanceHasherFinalize(ptr, 64));
        } finally {
            IsccLib.instanceHasherFree(ptr);
        }
    }

    /** Verify instanceHasherFinalize rejected by the core (bits 13) still consumes the hasher. */
    @Test
    void instanceHasherFinalizeCoreRejectedBitsConsumes() {
        long ptr = IsccLib.instanceHasherNew();
        try {
            assertThrows(
                    IllegalArgumentException.class, () -> IsccLib.instanceHasherFinalize(ptr, 13));
            assertConsumed("InstanceHasher", () -> IsccLib.instanceHasherFinalize(ptr, 64));
        } finally {
            IsccLib.instanceHasherFree(ptr);
        }
    }

    /** Verify algCdcChunks throws IllegalArgumentException for negative avg_chunk_size. */
    @Test
    void algCdcChunksNegativeAvgChunkSize() {
        assertThrows(
                IllegalArgumentException.class,
                () -> IsccLib.algCdcChunks(new byte[] {1, 2, 3}, false, -1));
    }

    // ── Hasher state validation ─────────────────────────────────────────────

    /** Verify DataHasher throws IllegalStateException when update is called after finalize. */
    @Test
    void testDataHasherThrowsIllegalStateAfterFinalize() {
        long ptr = IsccLib.dataHasherNew();
        try {
            IsccLib.dataHasherUpdate(ptr, new byte[] {1, 2, 3});
            IsccLib.dataHasherFinalize(ptr, 64);
            assertThrows(
                    IllegalStateException.class,
                    () -> IsccLib.dataHasherUpdate(ptr, new byte[] {4, 5, 6}));
        } finally {
            IsccLib.dataHasherFree(ptr);
        }
    }

    /** Verify InstanceHasher throws IllegalStateException when update is called after finalize. */
    @Test
    void testInstanceHasherThrowsIllegalStateAfterFinalize() {
        long ptr = IsccLib.instanceHasherNew();
        try {
            IsccLib.instanceHasherUpdate(ptr, new byte[] {1, 2, 3});
            IsccLib.instanceHasherFinalize(ptr, 64);
            assertThrows(
                    IllegalStateException.class,
                    () -> IsccLib.instanceHasherUpdate(ptr, new byte[] {4, 5, 6}));
        } finally {
            IsccLib.instanceHasherFree(ptr);
        }
    }

    // ── Hasher null-handle guard ────────────────────────────────────────────

    /** Verify dataHasherUpdate rejects a zero handle instead of dereferencing it. */
    @Test
    void testDataHasherUpdateRejectsNullHandle() {
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class,
                () -> IsccLib.dataHasherUpdate(0L, new byte[] {1, 2, 3}));
        assertTrue(e.getMessage().contains("handle is null"), e.getMessage());
    }

    /** Verify dataHasherFinalize rejects a zero handle instead of dereferencing it. */
    @Test
    void testDataHasherFinalizeRejectsNullHandle() {
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class,
                () -> IsccLib.dataHasherFinalize(0L, 64));
        assertTrue(e.getMessage().contains("handle is null"), e.getMessage());
    }

    /** Verify instanceHasherUpdate rejects a zero handle instead of dereferencing it. */
    @Test
    void testInstanceHasherUpdateRejectsNullHandle() {
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class,
                () -> IsccLib.instanceHasherUpdate(0L, new byte[] {1, 2, 3}));
        assertTrue(e.getMessage().contains("handle is null"), e.getMessage());
    }

    /** Verify instanceHasherFinalize rejects a zero handle instead of dereferencing it. */
    @Test
    void testInstanceHasherFinalizeRejectsNullHandle() {
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class,
                () -> IsccLib.instanceHasherFinalize(0L, 64));
        assertTrue(e.getMessage().contains("handle is null"), e.getMessage());
    }

    // ── Allocation failure ───────────────────────────────────────────────────

    /**
     * Verify a JVM allocation failure inside a native call reaches Java as OutOfMemoryError.
     *
     * <p>Runs {@link OomProbe} in a child JVM with {@code -Xmx16m -Xcheck:jni}, so the test JVM
     * keeps its heap. The child's stderr must carry no JNI-check warning (a {@code ThrowNew} over
     * a pending exception would print one).
     */
    @Test
    void allocationFailureSurfacesAsOutOfMemoryError() throws Exception {
        Path javaBin = Path.of(System.getProperty("java.home"), "bin", "java");
        ProcessBuilder pb = new ProcessBuilder(
                javaBin.toString(),
                "-Xmx16m",
                "-Xcheck:jni",
                "-Djava.library.path=" + System.getProperty("java.library.path"),
                "-cp", System.getProperty("java.class.path"),
                OomProbe.class.getName());
        Path stderrFile = Files.createTempFile("iscc-oom-probe", ".err");
        pb.redirectError(stderrFile.toFile());
        try {
            Process proc = pb.start();
            String stdout = new String(proc.getInputStream().readAllBytes()).trim();
            int exit = proc.waitFor();
            String stderr = Files.readString(stderrFile);
            assertEquals(0, exit, "probe exit code; stderr:\n" + stderr);
            assertEquals("java.lang.OutOfMemoryError", stdout, "probe stderr:\n" + stderr);
            assertTrue(!stderr.contains("WARNING in native method"), stderr);
        } finally {
            Files.deleteIfExists(stderrFile);
        }
    }

    // ── Native library loading ───────────────────────────────────────────────

    /** Exit code, stdout and stderr of a finished child JVM. */
    private record ProbeRun(int exit, String stdout, String stderr) {}

    /**
     * Run {@link LoaderProbe} in a child JVM and return what it printed.
     *
     * @param classpath the child's classpath
     * @param property a {@code -D} system property override for the child, or {@code null}
     * @param libraryDir the child's {@code java.library.path}
     */
    private static ProbeRun runLoaderProbe(String classpath, String property, Path libraryDir)
            throws Exception {
        Path javaBin = Path.of(System.getProperty("java.home"), "bin", "java");
        List<String> cmd = new ArrayList<>();
        cmd.add(javaBin.toString());
        if (property != null) {
            cmd.add(property);
        }
        cmd.add("-Djava.library.path=" + libraryDir);
        cmd.add("-cp");
        cmd.add(classpath);
        cmd.add(LoaderProbe.class.getName());
        ProcessBuilder pb = new ProcessBuilder(cmd);
        Path stderrFile = Files.createTempFile("iscc-loader-probe", ".err");
        pb.redirectError(stderrFile.toFile());
        try {
            Process proc = pb.start();
            String stdout = new String(proc.getInputStream().readAllBytes()).trim();
            int exit = proc.waitFor();
            return new ProbeRun(exit, stdout, Files.readString(stderrFile));
        } finally {
            Files.deleteIfExists(stderrFile);
        }
    }

    /**
     * Return the first entry of this JVM's {@code java.library.path} that holds the iscc_jni library.
     *
     * @return the directory, or {@code null} if no entry holds the library
     */
    private static Path libraryPathEntryWithLibrary() {
        String libName = System.mapLibraryName("iscc_jni");
        for (String entry : System.getProperty("java.library.path", "").split(File.pathSeparator)) {
            if (!entry.isEmpty() && Files.isRegularFile(Path.of(entry, libName))) {
                return Path.of(entry);
            }
        }
        return null;
    }

    /**
     * Copy the library bundled for this host into a fresh temp directory under its
     * {@code System.loadLibrary} name.
     *
     * <p>Reads the resource through this JVM's classpath, which never holds a broken test library.
     *
     * @return the temp directory, or {@code null} if no library is bundled for this host
     */
    private static Path extractBundledLibrary() throws Exception {
        String os = NativeLoader.detectOs();
        String arch = NativeLoader.detectArch();
        if (os == null || arch == null) {
            return null;
        }
        String resource = "/META-INF/native/" + os + "-" + arch + "/" + NativeLoader.libraryFileName(os);
        try (InputStream stream = NativeLoader.class.getResourceAsStream(resource)) {
            if (stream == null) {
                return null;
            }
            Path dir = Files.createTempDirectory("iscc-fallback-native");
            Files.copy(stream, dir.resolve(System.mapLibraryName("iscc_jni")));
            return dir;
        }
    }

    /**
     * Delete a file or a directory tree, if it exists.
     *
     * @param root the file or directory to delete, or {@code null} for nothing
     */
    private static void deleteTree(Path root) throws Exception {
        if (root == null || !Files.exists(root)) {
            return;
        }
        try (Stream<Path> paths = Files.walk(root)) {
            for (Path p : paths.sorted(Comparator.reverseOrder()).toList()) {
                Files.delete(p);
            }
        }
    }

    /**
     * Run {@link LoaderProbe} in a child JVM whose {@code java.library.path} holds a real library,
     * and assert it loaded the library and the selftest passed.
     *
     * <p>The fallback directory is the {@code java.library.path} entry that holds the library (the
     * CI layout), or else a temp copy of the library bundled as a resource (the release layout).
     * Neither found fails the test: skipping would leave the release job testing nothing.
     *
     * @param classpath the child's classpath
     * @param property a {@code -D} system property override for the child, or {@code null}
     */
    private static void assertLoaderProbePasses(String classpath, String property) throws Exception {
        Path extracted = null;
        Path libraryDir = libraryPathEntryWithLibrary();
        if (libraryDir == null) {
            extracted = extractBundledLibrary();
            libraryDir = extracted;
        }
        assertNotNull(libraryDir, "no " + System.mapLibraryName("iscc_jni")
                + " on java.library.path (" + System.getProperty("java.library.path")
                + ") and no library bundled under /META-INF/native/ for this host");
        try {
            ProbeRun run = runLoaderProbe(classpath, property, libraryDir);
            assertEquals(0, run.exit(),
                    "probe exit code; stdout:\n" + run.stdout() + "\nstderr:\n" + run.stderr());
            assertEquals("true", run.stdout(), "probe stderr:\n" + run.stderr());
        } finally {
            deleteTree(extracted);
        }
    }

    /**
     * Write garbage bytes where the JAR would bundle the library for this host.
     *
     * @return the classpath root holding the broken library
     */
    private static Path writeBrokenBundledLibrary() throws Exception {
        String os = NativeLoader.detectOs();
        String arch = NativeLoader.detectArch();
        assumeTrue(os != null && arch != null,
                "host platform has no bundled-library slot");
        Path root = Files.createTempDirectory("iscc-broken-native");
        Path lib = root.resolve("META-INF/native/" + os + "-" + arch + "/"
                + NativeLoader.libraryFileName(os));
        Files.createDirectories(lib.getParent());
        Files.write(lib, "not a shared library".getBytes());
        return root;
    }

    /** Verify an unrecognized architecture falls back to {@code java.library.path}. */
    @Test
    void loaderFallsBackOnUnknownArchitecture() throws Exception {
        assertLoaderProbePasses(System.getProperty("java.class.path"), "-Dos.arch=riscv64");
    }

    /** Verify an unrecognized operating system falls back to {@code java.library.path}. */
    @Test
    void loaderFallsBackOnUnknownOperatingSystem() throws Exception {
        assertLoaderProbePasses(System.getProperty("java.class.path"), "-Dos.name=FreeBSD");
    }

    /**
     * Verify a bundled library that fails to load falls back to {@code java.library.path}.
     *
     * <p>Puts a garbage library first on the child's classpath, so strategy 1 finds and rejects it.
     */
    @Test
    void loaderFallsBackOnBrokenBundledLibrary() throws Exception {
        Path root = writeBrokenBundledLibrary();
        try {
            String classpath = root + File.pathSeparator
                    + System.getProperty("java.class.path");
            assertLoaderProbePasses(classpath, null);
        } finally {
            deleteTree(root);
        }
    }

    /**
     * Verify the error when both strategies fail.
     *
     * <p>A garbage bundled library and an empty {@code java.library.path}: the child must throw an
     * {@code UnsatisfiedLinkError} that names the resource tried, has the
     * {@code System.loadLibrary} error as its cause, and carries the bundled-library error as its
     * one suppressed exception.
     */
    @Test
    void loaderReportsBothFailures() throws Exception {
        Path root = writeBrokenBundledLibrary();
        Path emptyDir = Files.createTempDirectory("iscc-empty-libpath");
        try {
            String classpath = root + File.pathSeparator
                    + System.getProperty("java.class.path");
            ProbeRun run = runLoaderProbe(classpath, null, emptyDir);
            assertEquals(0, run.exit(), "probe stderr:\n" + run.stderr());
            List<String> lines = run.stdout().lines().toList();
            assertEquals(3, lines.size(), "probe stdout:\n" + run.stdout());
            String resource = "META-INF/native/" + NativeLoader.detectOs() + "-"
                    + NativeLoader.detectArch() + "/"
                    + NativeLoader.libraryFileName(NativeLoader.detectOs());
            assertTrue(lines.get(0).startsWith("java.lang.UnsatisfiedLinkError: "), lines.get(0));
            assertTrue(lines.get(0).contains("'/" + resource + "'"), lines.get(0));
            assertEquals("cause: java.lang.UnsatisfiedLinkError", lines.get(1));
            assertEquals("suppressed: java.lang.UnsatisfiedLinkError", lines.get(2));
        } finally {
            deleteTree(root);
            deleteTree(emptyDir);
        }
    }

    // ── Constants ────────────────────────────────────────────────────────────

    /** Verify the 5 algorithm configuration constants. */
    @Test
    void testConstants() {
        assertEquals(128, IsccLib.META_TRIM_NAME);
        assertEquals(4096, IsccLib.META_TRIM_DESCRIPTION);
        assertEquals(128_000, IsccLib.META_TRIM_META);
        assertEquals(4_194_304, IsccLib.IO_READ_SIZE);
        assertEquals(13, IsccLib.TEXT_NGRAM_SIZE);
    }

    // ── jsonToDataUrl ────────────────────────────────────────────────────────

    /** Verify jsonToDataUrl produces a data URL with application/json media type. */
    @Test
    void testJsonToDataUrl() {
        String result = IsccLib.jsonToDataUrl("{\"key\":\"value\"}");
        assertTrue(result.startsWith("data:application/json;base64,"),
                "should start with data:application/json;base64,");
    }

    /** Verify jsonToDataUrl uses application/ld+json for JSON-LD content. */
    @Test
    void testJsonToDataUrlLdJson() {
        String result = IsccLib.jsonToDataUrl("{\"@context\":\"https://schema.org\"}");
        assertTrue(result.startsWith("data:application/ld+json;base64,"),
                "should start with data:application/ld+json;base64,");
    }

    // ── encodeComponent ──────────────────────────────────────────────────────

    /** Verify encodeComponent produces a valid ISCC unit string for a Meta-Code. */
    @Test
    void testEncodeComponent() {
        byte[] digest = new byte[8];
        String result = IsccLib.encodeComponent(0, 0, 0, 64, digest);
        assertNotNull(result);
        assertTrue(result.length() > 0, "encoded component should not be empty");
    }

    // ── isccDecode ───────────────────────────────────────────────────────────

    /** Verify isccDecode returns correct fields for a known Meta-Code. */
    @Test
    void testIsccDecode() {
        IsccDecodeResult result = IsccLib.isccDecode("AAAZXZ6OU74YAZIM");
        assertEquals(0, result.maintype, "maintype should be 0 (Meta)");
        assertEquals(0, result.subtype, "subtype should be 0");
        assertEquals(0, result.version, "version should be 0");
        assertEquals(1, result.length, "length index should be 1 (64-bit)");
        assertNotNull(result.digest);
        assertEquals(8, result.digest.length, "digest should be 8 bytes for 64-bit");
    }

    /** Verify isccDecode throws IllegalArgumentException on invalid input. */
    @Test
    void testIsccDecodeInvalid() {
        assertThrows(IllegalArgumentException.class, () -> IsccLib.isccDecode("INVALID"));
    }

    /** Verify roundtrip: encodeComponent -> isccDecode -> fields match inputs. */
    @Test
    void testEncodeDecodeRoundtrip() {
        byte[] digest = new byte[] {
            (byte) 0xAB, (byte) 0xCD, (byte) 0xEF, 0x01, 0x23, 0x45, 0x67, (byte) 0x89
        };
        String encoded = IsccLib.encodeComponent(0, 0, 0, 64, digest);
        IsccDecodeResult decoded = IsccLib.isccDecode(encoded);
        assertEquals(0, decoded.maintype);
        assertEquals(0, decoded.subtype);
        assertEquals(0, decoded.version);
        assertEquals(1, decoded.length, "length index should be 1 for 64-bit");
        assertArrayEquals(digest, decoded.digest);
    }

    // ── isccDecompose ────────────────────────────────────────────────────────

    /** Verify isccDecompose splits a composite ISCC-CODE into its two units. */
    @Test
    void isccDecomposeReturnsStringArray() {
        byte[] data = "Hello World".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        String dataCode = IsccLib.genDataCodeV0(data, 64);
        String instanceCode = IsccLib.genInstanceCodeV0(data, 64);
        String isccCode = IsccLib.genIsccCodeV0(new String[] {dataCode, instanceCode}, false);
        String[] units = IsccLib.isccDecompose(isccCode);
        assertEquals("[Ljava.lang.String;", units.getClass().getName(),
                "native must return a genuine String[]");
        assertEquals(2, units.length, "Data+Instance composite should decompose into 2 units");
        for (String unit : units) {
            assertTrue(unit.length() > 0, "decomposed unit should not be empty");
        }
    }

    // ── encode_seq / decode_seq ──────────────────────────────────────────────

    /** Decode a "bytes:&lt;hex&gt;" string to byte[]. */
    private static byte[] decodeBytes(String bytesStr) {
        assertTrue(bytesStr.startsWith("bytes:"), "expected bytes: prefix: " + bytesStr);
        return HexFormat.of().parseHex(bytesStr.substring("bytes:".length()));
    }

    /** Test all encode_seq vectors. */
    @TestFactory
    Collection<DynamicTest> encodeSeq() {
        JsonObject section = data.getAsJsonObject("encode_seq");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                String[] units = jsonArrayToStrings(tc.getAsJsonArray("inputs").get(0).getAsJsonArray());
                byte[] expected = decodeBytes(tc.get("outputs").getAsString());
                assertArrayEquals(expected, IsccLib.encodeSeq(units));
            }));
        }
        return tests;
    }

    /** Test all decode_seq vectors. */
    @TestFactory
    Collection<DynamicTest> decodeSeq() {
        JsonObject section = data.getAsJsonObject("decode_seq");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                byte[] seq = decodeBytes(tc.getAsJsonArray("inputs").get(0).getAsString());
                String[] expected = jsonArrayToStrings(tc.getAsJsonArray("outputs"));
                assertArrayEquals(expected, IsccLib.decodeSeq(seq));
            }));
        }
        return tests;
    }

    /** Verify the byte round trip encodeSeq(decodeSeq(x)) == x over the decode_seq vectors. */
    @TestFactory
    Collection<DynamicTest> seqByteRoundtrip() {
        JsonObject section = data.getAsJsonObject("decode_seq");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                byte[] seq = decodeBytes(tc.getAsJsonArray("inputs").get(0).getAsString());
                assertArrayEquals(seq, IsccLib.encodeSeq(IsccLib.decodeSeq(seq)));
            }));
        }
        return tests;
    }

    /** Verify decodeSeq returns a genuine String[] (the JVM does not type-check native returns). */
    @Test
    void decodeSeqReturnsStringArray() {
        byte[] seq = decodeBytes(data.getAsJsonObject("decode_seq")
                .getAsJsonObject("test_0002_single_unit").getAsJsonArray("inputs").get(0).getAsString());
        String[] units = IsccLib.decodeSeq(seq);
        assertEquals("[Ljava.lang.String;", units.getClass().getName(),
                "native must return a genuine String[]");
    }

    /** Verify encodeSeq rejects an empty unit array with the core message. */
    @Test
    void encodeSeqRejectsEmptyArray() {
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class,
                () -> IsccLib.encodeSeq(new String[0]));
        assertTrue(e.getMessage().contains("at least one ISCC-UNIT"), e.getMessage());
    }

    /** Verify encodeSeq rejects a unit with an embedded NUL followed by junk. */
    @Test
    void encodeSeqRejectsEmbeddedNul() {
        String unit = "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI";
        assertThrows(IllegalArgumentException.class,
                () -> IsccLib.encodeSeq(new String[] {unit + "\0junk"}));
    }

    /** Verify encodeSeq rejects a non-ASCII case-fold alias ("ſ" for "s"). */
    @Test
    void encodeSeqRejectsCaseFoldAlias() {
        assertThrows(IllegalArgumentException.class,
                () -> IsccLib.encodeSeq(new String[] {"gaawdlqng6ſsdpf2"}));
    }

    /** Verify encodeSeq rejects an "="-padded unit. */
    @Test
    void encodeSeqRejectsPadding() {
        assertThrows(IllegalArgumentException.class,
                () -> IsccLib.encodeSeq(new String[] {"ISCC:EEAYYNMFG5XOELK2===="}));
    }

    /** Verify decodeSeq rejects empty input. */
    @Test
    void decodeSeqRejectsEmpty() {
        assertThrows(IllegalArgumentException.class, () -> IsccLib.decodeSeq(new byte[0]));
    }

    /** Verify decodeSeq rejects a truncated sequence (last byte dropped). */
    @Test
    void decodeSeqRejectsTruncated() {
        byte[] seq = decodeBytes(data.getAsJsonObject("decode_seq")
                .getAsJsonObject("test_0002_single_unit").getAsJsonArray("inputs").get(0).getAsString());
        byte[] truncated = java.util.Arrays.copyOf(seq, seq.length - 1);
        assertThrows(IllegalArgumentException.class, () -> IsccLib.decodeSeq(truncated));
    }

    // ── conformanceSelftest ──────────────────────────────────────────────────

    /** Verify conformanceSelftest passes against the vendored test vectors. */
    @Test
    void conformanceSelftestReturnsTrue() {
        assertTrue(IsccLib.conformanceSelftest(), "conformance selftest should pass");
    }

    // ── encodeBase64 ─────────────────────────────────────────────────────────

    /** Verify encodeBase64 produces base64url (RFC 4648 §5) without padding. */
    @Test
    void encodeBase64KnownValue() {
        byte[] data = "Hello".getBytes(java.nio.charset.StandardCharsets.UTF_8);
        assertEquals("SGVsbG8", IsccLib.encodeBase64(data));
    }

    // ── textRemoveNewlines ───────────────────────────────────────────────────

    /** Verify textRemoveNewlines collapses a multi-line string to one line. */
    @Test
    void textRemoveNewlinesCollapsesToSingleLine() {
        assertEquals("Hello World", IsccLib.textRemoveNewlines("Hello\nWorld"));
    }

    // ── algSimhash ───────────────────────────────────────────────────────────

    /** Verify algSimhash of two complementary digests sets all bits (OR at n=2). */
    @Test
    void algSimhashKnownValue() {
        byte[][] digests = {
            {(byte) 0xF0, (byte) 0x0F, (byte) 0xF0, (byte) 0x0F},
            {(byte) 0x0F, (byte) 0xF0, (byte) 0x0F, (byte) 0xF0},
        };
        byte[] expected = {(byte) 0xFF, (byte) 0xFF, (byte) 0xFF, (byte) 0xFF};
        assertArrayEquals(expected, IsccLib.algSimhash(digests));
    }

    /** Verify algSimhash rejects an empty digest list. */
    @Test
    void algSimhashEmptyInput() {
        IllegalArgumentException ex = assertThrows(
                IllegalArgumentException.class, () -> IsccLib.algSimhash(new byte[0][]));
        assertTrue(ex.getMessage().contains("must not be empty"), ex.getMessage());
    }

    /** Verify algSimhash returns a single digest unchanged. */
    @Test
    void algSimhashSingleDigest() {
        byte[] digest = {(byte) 0xAB, (byte) 0xCD, (byte) 0xEF, (byte) 0x01};
        assertArrayEquals(digest, IsccLib.algSimhash(new byte[][] {digest}));
    }

    // ── algMinhash256 ────────────────────────────────────────────────────────

    /** Verify algMinhash256 of empty features returns 32 bytes of 0xFF. */
    @Test
    void algMinhash256EmptyFeatures() {
        byte[] expected = new byte[32];
        java.util.Arrays.fill(expected, (byte) 0xFF);
        assertArrayEquals(expected, IsccLib.algMinhash256(new int[0]));
    }

    /** Verify algMinhash256 is deterministic and returns a 32-byte digest. */
    @Test
    void algMinhash256Deterministic() {
        int[] features = {100, 200, 300, 400, 500};
        byte[] first = IsccLib.algMinhash256(features);
        byte[] second = IsccLib.algMinhash256(features);
        assertEquals(32, first.length, "digest should be 32 bytes");
        assertArrayEquals(first, second, "same features should give same digest");
    }

    // ── algCdcChunks (positive) ──────────────────────────────────────────────

    /** Verify algCdcChunks returns a genuine byte[][] whose chunks reassemble the input. */
    @Test
    void algCdcChunksSplitsAndReassembles() {
        byte[] data = new byte[8192];
        for (int i = 0; i < data.length; i++) {
            data[i] = (byte) i;
        }
        byte[][] chunks = IsccLib.algCdcChunks(data, false, 1024);
        assertEquals("[[B", chunks.getClass().getName(), "native must return a genuine byte[][]");
        assertTrue(chunks.length > 1, "8 KiB should split into multiple chunks");
        java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
        for (byte[] chunk : chunks) {
            out.write(chunk, 0, chunk.length);
        }
        assertArrayEquals(data, out.toByteArray(), "chunks must reassemble the input");
    }

    // ── softHashVideoV0 ──────────────────────────────────────────────────────

    /** Verify softHashVideoV0 digest matches the body of genVideoCodeV0. */
    @Test
    void softHashVideoV0MatchesGenVideoCodeBody() {
        int[][] frameSigs = {new int[380]};
        byte[] digest = IsccLib.softHashVideoV0(frameSigs, 64);
        assertEquals(8, digest.length, "bits=64 should produce an 8-byte digest");
        String code = IsccLib.genVideoCodeV0(frameSigs, 64);
        IsccDecodeResult decoded = IsccLib.isccDecode(code);
        assertArrayEquals(decoded.digest, digest,
                "soft hash digest should match the gen_video_code_v0 body");
    }
}
