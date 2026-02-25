/**
 * Conformance tests for all 9 gen_*_v0 JNI functions against data.json vectors.
 *
 * <p>Mirrors the Node.js conformance tests in crates/iscc-napi/__tests__/conformance.test.mjs.
 * Uses JUnit 5 {@code @TestFactory} with {@code DynamicTest} for data-driven test names
 * matching the JSON keys (e.g., {@code test_0001_title_only}).
 */
package io.iscc.iscc_lib;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.HexFormat;
import java.util.Map;
import java.util.TreeMap;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

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

    // ── Negative jint validation ─────────────────────────────────────────────

    /** Verify textTrim throws IllegalArgumentException for negative nbytes. */
    @Test
    void textTrimNegativeNbytes() {
        assertThrows(IllegalArgumentException.class, () -> IsccLib.textTrim("hello", -1));
    }

    /** Verify slidingWindow throws IllegalArgumentException for negative width. */
    @Test
    void slidingWindowNegativeWidth() {
        assertThrows(IllegalArgumentException.class, () -> IsccLib.slidingWindow("hello", -1));
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
}
