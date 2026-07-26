/**
 * Unicode 16.0.0 boundary conformance tests for the Kotlin (UniFFI) binding.
 *
 * Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json,
 * located via the iscc.fixtureDir system property set in build.gradle.kts) against
 * textClean and textCollapse, gating the declared-Unicode-version freeze rule:
 * code points unassigned in Unicode 16.0.0 are mapped to the noncharacter sentinel
 * U+FFFF before normalization and removed by the unchanged category-C filter, while
 * assigned code points flow through the regular pipeline. The sequence vectors
 * additionally pin the sentinel map against the superseded delete-filter design.
 */
package uniffi.iscc_uniffi

import com.google.gson.JsonObject
import com.google.gson.JsonParser
import java.io.File
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestFactory

class UnicodeBoundaryTest {

    companion object {
        /** Cached canonical Unicode boundary fixture (read in place, no vendored copy). */
        val boundary: JsonObject by lazy {
            val dir = System.getProperty("iscc.fixtureDir") ?: error("iscc.fixtureDir not set")
            val text = File(dir, "unicode_boundary.json").readText()
            JsonParser.parseString(text).asJsonObject
        }
    }

    /** Guard against a truncated or renamed fixture silently defining zero vector tests. */
    @Test
    fun boundaryFixtureMetadata() {
        assertEquals(
            "16.0.0",
            boundary.getAsJsonObject("_metadata")["unicode_data_version"].asString,
            "fixture must declare Unicode data version 16.0.0"
        )
        assertEquals(
            7,
            boundary.getAsJsonObject("text_clean").size(),
            "expected 7 text_clean boundary cases"
        )
        assertEquals(
            5,
            boundary.getAsJsonObject("text_collapse").size(),
            "expected 5 text_collapse boundary cases"
        )
    }

    // -- text_clean --

    /** Test all text_clean boundary vectors. */
    @TestFactory
    fun textCleanBoundary(): List<DynamicTest> {
        val section = boundary.getAsJsonObject("text_clean")
        return section.entrySet().map { (name, element) ->
            DynamicTest.dynamicTest(name) {
                val tc = element.asJsonObject
                val input = tc.getAsJsonArray("inputs")[0].asString
                val expected = tc.getAsJsonObject("outputs")["result"].asString
                assertEquals(expected, textClean(input), "text_clean mismatch for $name")
            }
        }
    }

    // -- text_collapse --

    /** Test all text_collapse boundary vectors. */
    @TestFactory
    fun textCollapseBoundary(): List<DynamicTest> {
        val section = boundary.getAsJsonObject("text_collapse")
        return section.entrySet().map { (name, element) ->
            DynamicTest.dynamicTest(name) {
                val tc = element.asJsonObject
                val input = tc.getAsJsonArray("inputs")[0].asString
                val expected = tc.getAsJsonObject("outputs")["result"].asString
                assertEquals(expected, textCollapse(input), "text_collapse mismatch for $name")
            }
        }
    }
}
