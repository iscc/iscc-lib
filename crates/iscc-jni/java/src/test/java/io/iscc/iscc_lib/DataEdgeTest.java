/**
 * Shared edge-case vectors (crates/iscc-lib/tests/data-edge.json) run against the JNI bindings.
 *
 * <p>Every vector runs, {@code except.java} is the only deviation, and a function section this
 * harness does not handle fails. A rejection is asserted on the raw call alone (it must throw
 * {@link IllegalArgumentException}); the result is converted to its fixture form only after a
 * successful call.
 */
package io.iscc.iscc_lib;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.google.gson.JsonPrimitive;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.Callable;
import java.util.function.Function;
import org.junit.jupiter.api.Test;

class DataEdgeTest {

    /** Surface id of this harness in the {@code except} map. */
    private static final String SURFACE = "java";

    /** A prepared vector: the raw binding call and the converter for its successful result. */
    private record Prepared(Callable<Object> call, Function<Object, JsonElement> convert) {}

    /** Section name to caller, which turns a vector's inputs into a {@link Prepared} call. */
    private static final Map<String, Function<JsonArray, Prepared>> SECTIONS = Map.of(
            "iscc_decode", DataEdgeTest::callDecode,
            "encode_seq", DataEdgeTest::callSeq,
            "gen_text_code_v0", DataEdgeTest::callText,
            "gen_mixed_code_v0", DataEdgeTest::callMixed,
            "gen_iscc_code_v0", DataEdgeTest::callIsccCode,
            "alg_simhash", DataEdgeTest::callSimhash,
            "gen_meta_code_v0", DataEdgeTest::callMeta,
            "gen_video_code_v0", DataEdgeTest::callVideo);

    /** Load and parse the canonical data-edge.json. */
    private static JsonObject loadDoc() throws Exception {
        Path path = Path.of("../../iscc-lib/tests/data-edge.json");
        return JsonParser.parseString(Files.readString(path, StandardCharsets.UTF_8)).getAsJsonObject();
    }

    /** Encode bytes as a {@code bytes:<hex>} fixture value. */
    private static JsonElement bytesValue(byte[] data) {
        return new JsonPrimitive("bytes:" + HexFormat.of().formatHex(data));
    }

    /** Read a JSON array input as a String array. */
    private static String[] strings(JsonElement input) {
        JsonArray items = input.getAsJsonArray();
        String[] out = new String[items.size()];
        for (int i = 0; i < out.length; i++) {
            out[i] = items.get(i).getAsString();
        }
        return out;
    }

    /** Decode a JSON array input of {@code bytes:<hex>} values into a byte array per item. */
    private static byte[][] byteArrays(JsonElement input) {
        JsonArray items = input.getAsJsonArray();
        byte[][] out = new byte[items.size()][];
        for (int i = 0; i < out.length; i++) {
            String item = items.get(i).getAsString();
            if (!item.startsWith("bytes:")) {
                throw new IllegalStateException("expected a bytes:<hex> input, got " + item);
            }
            out[i] = HexFormat.of().parseHex(item.substring("bytes:".length()));
        }
        return out;
    }

    /** Read a JSON array input of integer lists as an int array per item. */
    private static int[][] intArrays(JsonElement input) {
        JsonArray items = input.getAsJsonArray();
        int[][] out = new int[items.size()][];
        for (int i = 0; i < out.length; i++) {
            JsonArray values = items.get(i).getAsJsonArray();
            out[i] = new int[values.size()];
            for (int j = 0; j < out[i].length; j++) {
                out[i][j] = values.get(j).getAsInt();
            }
        }
        return out;
    }

    /** Read a meta input: a JSON object is serialized, a string is passed as is. */
    private static String metaArg(JsonElement input) {
        return input.isJsonObject() ? input.toString() : input.getAsString();
    }

    /** Wrap a bare ISCC string result (gen functions) as an {@code {iscc}} object. */
    private static JsonElement isccObject(Object iscc) {
        JsonObject obj = new JsonObject();
        obj.addProperty("iscc", (String) iscc);
        return obj;
    }

    /** Convert an {@code isccDecode} result to {@code [mt, st, vs, length, "bytes:<hex>"]}. */
    private static JsonElement decodeArray(Object result) {
        IsccDecodeResult r = (IsccDecodeResult) result;
        JsonArray out = new JsonArray();
        out.add(r.maintype);
        out.add(r.subtype);
        out.add(r.version);
        out.add(r.length);
        out.add(bytesValue(r.digest));
        return out;
    }

    /** Prepare {@code isccDecode} on the vector's string input. */
    private static Prepared callDecode(JsonArray inputs) {
        String iscc = inputs.get(0).getAsString();
        return new Prepared(() -> IsccLib.isccDecode(iscc), DataEdgeTest::decodeArray);
    }

    /** Prepare {@code encodeSeq} on the vector's unit list. */
    private static Prepared callSeq(JsonArray inputs) {
        String[] units = strings(inputs.get(0));
        return new Prepared(() -> IsccLib.encodeSeq(units), r -> bytesValue((byte[]) r));
    }

    /** Prepare {@code genTextCodeV0} on the vector's text and bits. */
    private static Prepared callText(JsonArray inputs) {
        String text = inputs.get(0).getAsString();
        int bits = inputs.get(1).getAsInt();
        return new Prepared(() -> IsccLib.genTextCodeV0(text, bits), DataEdgeTest::isccObject);
    }

    /** Prepare {@code genMixedCodeV0} on the vector's codes and bits. */
    private static Prepared callMixed(JsonArray inputs) {
        String[] codes = strings(inputs.get(0));
        int bits = inputs.get(1).getAsInt();
        return new Prepared(() -> IsccLib.genMixedCodeV0(codes, bits), DataEdgeTest::isccObject);
    }

    /** Prepare {@code genIsccCodeV0} (the optional second input is {@code wide}, default false). */
    private static Prepared callIsccCode(JsonArray inputs) {
        String[] codes = strings(inputs.get(0));
        boolean wide = inputs.size() > 1 && inputs.get(1).getAsBoolean();
        return new Prepared(() -> IsccLib.genIsccCodeV0(codes, wide), DataEdgeTest::isccObject);
    }

    /** Prepare {@code algSimhash} on the vector's digest list. */
    private static Prepared callSimhash(JsonArray inputs) {
        byte[][] digests = byteArrays(inputs.get(0));
        return new Prepared(() -> IsccLib.algSimhash(digests), r -> bytesValue((byte[]) r));
    }

    /** Prepare {@code genMetaCodeV0} (an empty description means none). */
    private static Prepared callMeta(JsonArray inputs) {
        String name = inputs.get(0).getAsString();
        String description = inputs.get(1).getAsString();
        String desc = description.isEmpty() ? null : description;
        String meta = metaArg(inputs.get(2));
        int bits = inputs.get(3).getAsInt();
        return new Prepared(() -> IsccLib.genMetaCodeV0(name, desc, meta, bits), DataEdgeTest::isccObject);
    }

    /** Prepare {@code genVideoCodeV0} on the vector's frame signatures and bits. */
    private static Prepared callVideo(JsonArray inputs) {
        int[][] frames = intArrays(inputs.get(0));
        int bits = inputs.get(1).getAsInt();
        return new Prepared(() -> IsccLib.genVideoCodeV0(frames, bits), DataEdgeTest::isccObject);
    }

    /**
     * Section name to the expected output keys its JNI result does not carry, which are therefore
     * not compared (an absent section compares every key). The gen functions here return a bare
     * ISCC string, so text drops {@code characters}, mixed drops {@code parts} and meta drops
     * {@code name}, {@code meta} and {@code metahash}.
     */
    private static final Map<String, Set<String>> UNCOMPARED = Map.of(
            "gen_text_code_v0", Set.of("characters"),
            "gen_mixed_code_v0", Set.of("parts"),
            "gen_meta_code_v0", Set.of("name", "meta", "metahash"));

    /**
     * Compare an actual output with the expected one and return a failure, or null.
     *
     * <p>For an object, every expected key outside the section's static uncompared set must be
     * present in the result and equal; a missing key fails.
     */
    private static String compare(String section, JsonElement expected, JsonElement actual) {
        if (!expected.isJsonObject()) {
            return expected.equals(actual) ? null : "expected " + expected + ", got " + actual;
        }
        if (!actual.isJsonObject()) {
            return "expected an object, got " + actual;
        }
        JsonObject want = expected.getAsJsonObject();
        JsonObject got = actual.getAsJsonObject();
        Set<String> uncompared = UNCOMPARED.getOrDefault(section, Set.of());
        for (String key : want.keySet()) {
            if (uncompared.contains(key)) {
                continue;
            }
            if (!got.has(key)) {
                return "missing field " + key + " in " + got;
            }
            if (!want.get(key).equals(got.get(key))) {
                return "field " + key + ": expected " + want.get(key) + ", got " + got.get(key);
            }
        }
        return null;
    }

    /** Return this surface's {@code except} entry of a vector, or null. */
    private static String exception(JsonObject vector) {
        JsonObject except = vector.getAsJsonObject("except");
        return except != null && except.has(SURFACE) ? except.get(SURFACE).getAsString() : null;
    }

    /** Judge a successful raw call against the vector's expectation. */
    private static String judgeOk(
            String section, boolean expectError, Prepared prepared, Object result, JsonObject vector) {
        JsonElement got = prepared.convert().apply(result);
        if (expectError) {
            return "expected an error, got " + got;
        }
        if (!vector.has("outputs")) {
            return "vector has neither outputs nor error";
        }
        return compare(section, vector.get("outputs"), got);
    }

    /** Check one vector and return a failure description, or null. */
    private static String checkVector(String section, Function<JsonArray, Prepared> caller, JsonObject vector) {
        String exception = exception(vector);
        if ("skip".equals(exception)) {
            return null;
        }
        boolean expectError = "error".equals(exception) || vector.has("error");
        Prepared prepared = caller.apply(vector.getAsJsonArray("inputs"));
        Object result;
        try {
            result = prepared.call().call();
        } catch (IllegalArgumentException e) {
            return expectError ? null : "expected an output, got error: " + e.getMessage();
        } catch (Exception e) {
            return "raw call threw " + e.getClass().getName() + ": " + e.getMessage();
        }
        return judgeOk(section, expectError, prepared, result, vector);
    }

    /** Run every vector of a parsed document; return one {@code section/vector: reason} per failure. */
    private static List<String> checkDoc(JsonObject doc) {
        List<String> failures = new ArrayList<>();
        for (Map.Entry<String, JsonElement> section : doc.entrySet()) {
            Function<JsonArray, Prepared> caller = SECTIONS.get(section.getKey());
            if (caller == null) {
                failures.add(section.getKey() + ": unknown section");
                continue;
            }
            for (Map.Entry<String, JsonElement> entry : section.getValue().getAsJsonObject().entrySet()) {
                String reason = checkVector(section.getKey(), caller, entry.getValue().getAsJsonObject());
                if (reason != null) {
                    failures.add(section.getKey() + "/" + entry.getKey() + ": " + reason);
                }
            }
        }
        return failures;
    }

    /** Parse a synthetic document for the negative tests. */
    private static JsonObject parse(String json) {
        return JsonParser.parseString(json).getAsJsonObject();
    }

    /** Every vector of data-edge.json passes, and every handled section has vectors. */
    @Test
    void dataEdgeVectorsPass() throws Exception {
        JsonObject doc = loadDoc();
        for (String section : SECTIONS.keySet()) {
            assertTrue(doc.has(section) && !doc.getAsJsonObject(section).isEmpty(),
                    "section " + section + " has no vectors");
        }
        List<String> failures = checkDoc(doc);
        assertTrue(failures.isEmpty(), "data-edge.json failures:\n" + String.join("\n", failures));
    }

    /** A section the harness does not handle fails. */
    @Test
    void dataEdgeFailsUnknownSection() {
        List<String> failures = checkDoc(parse(
                "{\"gen_unknown_v0\": {\"test_0001_x\": {\"inputs\": [\"x\"], \"outputs\": \"x\", \"note\": \"n\"}}}"));
        assertEquals(1, failures.size(), failures.toString());
        assertTrue(failures.get(0).contains("unknown section"), failures.toString());
    }

    /** Valid inputs marked {@code error} prove each rejection-only caller reaches the binding. */
    @Test
    void dataEdgeFailsExpectedErrorThatSucceeds() {
        List<String> failures = checkDoc(parse("{"
                + "\"gen_mixed_code_v0\": {\"test_0001_x\": {\"inputs\": "
                + "[[\"EUA6GIKXN42IQV3S\", \"EIAUKMOUIOYZCKA5\"], 64], \"error\": \"e\", \"note\": \"n\"}},"
                + "\"gen_iscc_code_v0\": {\"test_0001_x\": {\"inputs\": [[\"GABVVC5DMJJGYKZ4ZBYVNYABFFYXG\", "
                + "\"IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ\"]], \"error\": \"e\", \"note\": \"n\"}}}"));
        assertEquals(2, failures.size(), failures.toString());
        assertTrue(failures.stream().allMatch(f -> f.contains("expected an error")), failures.toString());
    }

    /** A wrong output fails. */
    @Test
    void dataEdgeFailsOutputMismatch() {
        List<String> failures = checkDoc(parse("{\"encode_seq\": {\"test_0001_x\": {\"inputs\": "
                + "[[\"ISCC:GAAAAAAAAA\"]], \"outputs\": \"bytes:300000000001\", \"note\": \"n\"}}}"));
        assertEquals(1, failures.size(), failures.toString());
        assertTrue(failures.get(0).contains("expected \"bytes:300000000001\""), failures.toString());
    }

    /** {@code except.java} turns an output vector into a rejection and {@code skip} makes no call. */
    @Test
    void dataEdgeHonoursExcept() {
        List<String> failures = checkDoc(parse("{\"iscc_decode\": {"
                + "\"test_0001_error\": {\"inputs\": [\"ISCC:GAAAAAAAAA\"], "
                + "\"outputs\": [3, 0, 0, 0, \"bytes:00000000\"], \"note\": \"n\", \"except\": {\"java\": \"error\"}},"
                + "\"test_0002_skip\": {\"inputs\": [\"ISCC:GAAAAAAAAB\"], "
                + "\"outputs\": [3, 0, 0, 0, \"bytes:00000000\"], \"note\": \"n\", \"except\": {\"java\": \"skip\"}}}}"));
        assertEquals(1, failures.size(), failures.toString());
        assertTrue(failures.get(0).startsWith("iscc_decode/test_0001_error: expected an error"),
                failures.toString());
    }

    /** The data.json Hello World vector passes: {@code characters} is uncompared, {@code iscc} matches. */
    @Test
    void dataEdgeComparesObjectOutput() {
        List<String> failures = checkDoc(parse("{\"gen_text_code_v0\": {\"test_0001_x\": {\"inputs\": "
                + "[\"Hello World\", 64], \"outputs\": {\"iscc\": \"ISCC:EAASKDNZNYGUUF5A\", "
                + "\"characters\": 10}, \"note\": \"n\"}}}"));
        assertTrue(failures.isEmpty(), failures.toString());
    }

    /** {@code parts} is uncompared for mixed only, so a text result lacking it fails. */
    @Test
    void dataEdgeFailsKeyOutsideUncomparedSet() {
        List<String> failures = checkDoc(parse("{\"gen_text_code_v0\": {\"test_0001_x\": {\"inputs\": "
                + "[\"Hello World\", 64], \"outputs\": {\"iscc\": \"ISCC:EAASKDNZNYGUUF5A\", "
                + "\"parts\": []}, \"note\": \"n\"}}}"));
        assertEquals(1, failures.size(), failures.toString());
        assertTrue(failures.get(0).contains("missing field parts"), failures.toString());
    }

    /** A result without {@code iscc} fails even though its other key matches. */
    @Test
    void dataEdgeFailsResultWithoutIscc() {
        String failure = compare("gen_text_code_v0",
                JsonParser.parseString("{\"iscc\": \"ISCC:EAASKDNZNYGUUF5A\", \"characters\": 10}"),
                JsonParser.parseString("{\"characters\": 10}"));
        assertNotNull(failure);
        assertTrue(failure.contains("missing field iscc"), failure);
    }
}
