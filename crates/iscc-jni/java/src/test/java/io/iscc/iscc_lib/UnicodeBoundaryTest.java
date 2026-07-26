/**
 * Unicode 16.0.0 boundary conformance tests for the JNI binding.
 *
 * <p>Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json)
 * against textClean and textCollapse, gating the declared-Unicode-version freeze
 * rule: code points unassigned in Unicode 16.0.0 are mapped to the noncharacter
 * sentinel U+FFFF before normalization and removed by the unchanged category-C
 * filter, while assigned code points flow through the regular pipeline. The
 * sequence vectors additionally pin the sentinel map against the superseded
 * delete-filter design.
 */
package io.iscc.iscc_lib;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Map;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

class UnicodeBoundaryTest {

    private static JsonObject boundary;

    /** Load the canonical Unicode boundary vectors (read in place, no vendored copy). */
    @BeforeAll
    static void loadBoundaryData() throws Exception {
        String json = Files.readString(Path.of("../../iscc-lib/tests/unicode_boundary.json"));
        boundary = JsonParser.parseString(json).getAsJsonObject();
    }

    /** Guard against a truncated or renamed fixture silently defining zero vector tests. */
    @Test
    void boundaryFixtureMetadata() {
        assertEquals(
                "16.0.0",
                boundary.getAsJsonObject("_metadata").get("unicode_data_version").getAsString(),
                "fixture must declare Unicode data version 16.0.0");
        assertEquals(
                7,
                boundary.getAsJsonObject("text_clean").size(),
                "expected 7 text_clean boundary cases");
        assertEquals(
                5,
                boundary.getAsJsonObject("text_collapse").size(),
                "expected 5 text_collapse boundary cases");
    }

    // ── text_clean ───────────────────────────────────────────────────────────

    /** Test all text_clean boundary vectors. */
    @TestFactory
    Collection<DynamicTest> textCleanBoundary() {
        JsonObject section = boundary.getAsJsonObject("text_clean");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                String input = tc.getAsJsonArray("inputs").get(0).getAsString();
                String expected = tc.getAsJsonObject("outputs").get("result").getAsString();
                assertEquals(expected, IsccLib.textClean(input),
                        "text_clean mismatch for " + name);
            }));
        }
        return tests;
    }

    // ── text_collapse ────────────────────────────────────────────────────────

    /** Test all text_collapse boundary vectors. */
    @TestFactory
    Collection<DynamicTest> textCollapseBoundary() {
        JsonObject section = boundary.getAsJsonObject("text_collapse");
        Collection<DynamicTest> tests = new ArrayList<>();
        for (Map.Entry<String, JsonElement> entry : section.entrySet()) {
            String name = entry.getKey();
            JsonObject tc = entry.getValue().getAsJsonObject();
            tests.add(DynamicTest.dynamicTest(name, () -> {
                String input = tc.getAsJsonArray("inputs").get(0).getAsString();
                String expected = tc.getAsJsonObject("outputs").get("result").getAsString();
                assertEquals(expected, IsccLib.textCollapse(input),
                        "text_collapse mismatch for " + name);
            }));
        }
        return tests;
    }
}
