/**
 * Shared edge-case vectors (data-edge.json, vendored from crates/iscc-lib/tests) run against the
 * Kotlin binding.
 *
 * Every vector runs, `except["kotlin"]` is the only deviation, and a function section this harness
 * does not handle fails. A rejection is asserted on the raw binding call alone (it must throw
 * `IsccUniException.IsccException`); the result is converted to its fixture form only after a
 * successful call. Every expected output key must be present in the converted result and equal.
 */
package uniffi.iscc_uniffi

import com.google.gson.JsonArray
import com.google.gson.JsonElement
import com.google.gson.JsonObject
import com.google.gson.JsonParser
import java.util.HexFormat
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

/** One handled section: `prepare` reads the inputs into a raw call, `convert` maps its result. */
class EdgeCaller(val prepare: (JsonArray) -> () -> Any, val convert: (Any) -> Any)

/** The vector checks behind [DataEdgeTest], kept free of assertions so synthetic docs can be fed in. */
object DataEdge {

    /** Surface id of this harness in the `except` map. */
    const val SURFACE = "kotlin"

    /** Read the input at [index] as a JSON string. */
    fun str(inputs: JsonArray, index: Int): String {
        val value = inputs.get(index)
        require(value.isJsonPrimitive && value.asJsonPrimitive.isString) { "input $index is not a string" }
        return value.asString
    }

    /** Read the input at [index] as a list of JSON strings. */
    fun strList(inputs: JsonArray, index: Int): List<String> {
        val value = inputs.get(index)
        require(value.isJsonArray) { "input $index is not an array" }
        val items = value.asJsonArray
        return List(items.size()) { str(items, it) }
    }

    /** Read the input at [index] as a `UInt` bit length. */
    fun bits(inputs: JsonArray, index: Int): UInt {
        val value = inputs.get(index)
        require(value.isJsonPrimitive && value.asJsonPrimitive.isNumber) { "input $index is not a number" }
        val number = value.asString.toLong()
        require(number in 0L..UInt.MAX_VALUE.toLong()) { "input $index is not a UInt" }
        return number.toUInt()
    }

    /** Read the optional boolean input at [index], defaulting to false when absent. */
    fun optBool(inputs: JsonArray, index: Int): Boolean {
        if (index >= inputs.size()) return false
        val value = inputs.get(index)
        require(value.isJsonPrimitive && value.asJsonPrimitive.isBoolean) { "input $index is not a boolean" }
        return value.asBoolean
    }

    /** Encode bytes as a `bytes:<hex>` fixture value. */
    fun bytesValue(data: ByteArray): String = "bytes:" + HexFormat.of().formatHex(data)

    /** Prepare `isccDecode` on the vector's string input. */
    fun prepareDecode(inputs: JsonArray): () -> Any {
        val iscc = str(inputs, 0)
        return { isccDecode(iscc) }
    }

    /** Convert a [DecodeResult] to `[maintype, subtype, version, length, "bytes:<hex>"]`. */
    fun convertDecode(result: Any): Any {
        val r = result as DecodeResult
        return listOf(r.maintype.toLong(), r.subtype.toLong(), r.version.toLong(), r.length.toLong(), bytesValue(r.digest))
    }

    /** Prepare `encodeSeq` on the vector's unit list. */
    fun prepareSeq(inputs: JsonArray): () -> Any {
        val units = strList(inputs, 0)
        return { encodeSeq(units) }
    }

    /** Convert `encodeSeq` bytes to a `bytes:<hex>` value. */
    fun convertSeq(result: Any): Any = bytesValue(result as ByteArray)

    /** Prepare `genTextCodeV0` on the vector's text and bits. */
    fun prepareText(inputs: JsonArray): () -> Any {
        val text = str(inputs, 0)
        val bits = bits(inputs, 1)
        return { genTextCodeV0(text, bits) }
    }

    /** Convert a [TextCodeResult] to a map of all its fields. */
    fun convertText(result: Any): Any {
        val r = result as TextCodeResult
        return mapOf("iscc" to r.iscc, "characters" to r.characters.toLong())
    }

    /** Prepare `genMixedCodeV0` on the vector's codes and bits. */
    fun prepareMixed(inputs: JsonArray): () -> Any {
        val codes = strList(inputs, 0)
        val bits = bits(inputs, 1)
        return { genMixedCodeV0(codes, bits) }
    }

    /** Convert a [MixedCodeResult] to a map of all its fields. */
    fun convertMixed(result: Any): Any {
        val r = result as MixedCodeResult
        return mapOf("iscc" to r.iscc, "parts" to r.parts)
    }

    /** Prepare `genIsccCodeV0` (the optional second input is `wide`). */
    fun prepareIsccCode(inputs: JsonArray): () -> Any {
        val codes = strList(inputs, 0)
        val wide = optBool(inputs, 1)
        return { genIsccCodeV0(codes, wide) }
    }

    /** Convert an [IsccCodeResult] to a map of all its fields. */
    fun convertIsccCode(result: Any): Any = mapOf("iscc" to (result as IsccCodeResult).iscc)

    /** Section name -> caller for every function section this harness handles. */
    val SECTIONS: Map<String, EdgeCaller> = mapOf(
        "iscc_decode" to EdgeCaller(::prepareDecode, ::convertDecode),
        "encode_seq" to EdgeCaller(::prepareSeq, ::convertSeq),
        "gen_text_code_v0" to EdgeCaller(::prepareText, ::convertText),
        "gen_mixed_code_v0" to EdgeCaller(::prepareMixed, ::convertMixed),
        "gen_iscc_code_v0" to EdgeCaller(::prepareIsccCode, ::convertIsccCode),
    )

    /** Load and parse the vendored data-edge.json from the test resources. */
    fun loadDoc(): JsonObject {
        val stream = DataEdge::class.java.classLoader.getResourceAsStream("data-edge.json")
            ?: error("data-edge.json not found in test resources")
        val text = stream.bufferedReader(Charsets.UTF_8).use { it.readText() }
        return JsonParser.parseString(text).asJsonObject
    }

    /** Compare an expected fixture value with a plain Kotlin value; null means equal. */
    fun compareValue(expected: JsonElement, actual: Any?): String? {
        if (expected.isJsonArray) {
            val want = expected.asJsonArray
            if (actual !is List<*> || actual.size != want.size()) return "expected $want, got $actual"
            return (0 until want.size()).firstNotNullOfOrNull { compareValue(want[it], actual[it]) }
        }
        val primitive = if (expected.isJsonPrimitive) expected.asJsonPrimitive else null
        return when {
            primitive == null -> "unsupported expected value $expected"
            primitive.isString -> if (actual is String && actual == primitive.asString) null else "expected $expected, got $actual"
            primitive.isNumber -> if (actual is Long && actual.toString() == primitive.asString) null else "expected $expected, got $actual"
            else -> "unsupported expected value $expected"
        }
    }

    /** Compare a vector's `outputs` with the converted result; every expected key must be present. */
    fun compare(expected: JsonElement, actual: Any): String? {
        if (!expected.isJsonObject) return compareValue(expected, actual)
        if (actual !is Map<*, *>) return "expected an object, got $actual"
        for (key in expected.asJsonObject.keySet().sorted()) {
            if (!actual.containsKey(key)) return "missing field $key in $actual"
            compareValue(expected.asJsonObject[key], actual[key])?.let { return "field $key: $it" }
        }
        return null
    }

    /** Judge a successful raw call against the vector's expectation. */
    fun judgeOk(expectError: Boolean, got: Any, vector: JsonObject): String? {
        if (expectError) return "expected an error, got $got"
        val outputs = vector["outputs"] ?: return "vector has neither outputs nor error"
        return compare(outputs, got)
    }

    /** Read this surface's `except` entry of a vector, or null. */
    fun exception(vector: JsonObject): String? {
        val except = vector["except"]
        if (except == null || !except.isJsonObject) return null
        return except.asJsonObject[SURFACE]?.asString
    }

    /** Check one vector and return a failure description, or null. */
    fun checkVector(caller: EdgeCaller, vector: JsonObject): String? {
        val exception = exception(vector)
        if (exception == "skip") return null
        val expectError = exception == "error" || vector.has("error")
        val call = try {
            caller.prepare(vector.getAsJsonArray("inputs") ?: JsonArray())
        } catch (e: Exception) {
            return "harness: $e"
        }
        val result = try {
            call()
        } catch (e: IsccUniException.IsccException) {
            return if (expectError) null else "expected an output, got IsccException: ${e.msg}"
        } catch (e: Exception) {
            return "raw call threw $e"
        }
        return judgeOk(expectError, caller.convert(result), vector)
    }

    /** Run every vector of a parsed document; return one `section/vector: reason` per failure. */
    fun checkDoc(doc: JsonObject): List<String> {
        val failures = mutableListOf<String>()
        for (section in doc.keySet().sorted()) {
            val caller = SECTIONS[section]
            if (caller == null) {
                failures.add("$section: unknown section")
                continue
            }
            val vectors = doc.getAsJsonObject(section)
            for (name in vectors.keySet().sorted()) {
                checkVector(caller, vectors.getAsJsonObject(name))?.let { failures.add("$section/$name: $it") }
            }
        }
        return failures
    }
}

class DataEdgeTest {

    @Test
    fun testDataEdgeVectorsPass() {
        val doc = DataEdge.loadDoc()
        for (section in DataEdge.SECTIONS.keys) {
            val vectors = doc.getAsJsonObject(section)
            assertFalse(vectors == null || vectors.size() == 0, "section $section has no vectors")
        }
        val failures = DataEdge.checkDoc(doc)

        assertEquals(emptyList<String>(), failures, "data-edge.json failures:\n${failures.joinToString("\n")}")
    }

    @Test
    fun testDataEdgeFailsUnknownSection() {
        val doc = JsonParser.parseString(
            """{"gen_unknown_v0": {"test_0001_x": {"inputs": ["x"], "outputs": "x", "note": "n"}}}"""
        ).asJsonObject
        val failures = DataEdge.checkDoc(doc)

        assertEquals(1, failures.size, failures.toString())
        assertTrue(failures[0].contains("unknown section"), failures.toString())
    }

    @Test
    fun testDataEdgeFailsExpectedErrorThatSucceeds() {
        // Valid inputs marked `error` prove each rejection-only caller reaches the binding.
        val doc = JsonParser.parseString(
            """
            {
              "gen_mixed_code_v0": {"test_0001_x": {
                "inputs": [["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"], 64], "error": "e", "note": "n"
              }},
              "gen_iscc_code_v0": {"test_0001_x": {
                "inputs": [["GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
                            "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]],
                "error": "e", "note": "n"
              }}
            }
            """
        ).asJsonObject
        val failures = DataEdge.checkDoc(doc)

        assertEquals(2, failures.size, failures.toString())
        assertTrue(failures.all { it.contains("expected an error") }, failures.toString())
    }

    @Test
    fun testDataEdgeFailsOutputMismatch() {
        val doc = JsonParser.parseString(
            """{"encode_seq": {"test_0001_x": {
                "inputs": [["ISCC:GAAAAAAAAA"]], "outputs": "bytes:300000000001", "note": "n"
            }}}"""
        ).asJsonObject
        val failures = DataEdge.checkDoc(doc)

        assertEquals(1, failures.size, failures.toString())
        assertTrue(failures[0].contains("expected \"bytes:300000000001\""), failures.toString())
    }

    @Test
    fun testDataEdgeFailsMissingOutputKey() {
        // `iscc` is correct; the only bad case is a key the result does not have.
        val doc = JsonParser.parseString(
            """{"gen_text_code_v0": {"test_0001_x": {
                "inputs": ["Hello\u0000World", 64],
                "outputs": {"iscc": "ISCC:EAASKDNZNYGUUF5A", "datahash": "x"}, "note": "n"
            }}}"""
        ).asJsonObject
        val failures = DataEdge.checkDoc(doc)

        assertEquals(1, failures.size, failures.toString())
        assertTrue(failures[0].contains("missing field datahash"), failures.toString())
    }

    @Test
    fun testDataEdgeHonoursExcept() {
        val doc = JsonParser.parseString(
            """
            {"iscc_decode": {
              "test_0001_error": {"inputs": ["ISCC:GAAAAAAAAA"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
                                  "note": "n", "except": {"kotlin": "error"}},
              "test_0002_skip": {"inputs": ["ISCC:GAAAAAAAAB"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
                                 "note": "n", "except": {"kotlin": "skip"}}
            }}
            """
        ).asJsonObject
        val failures = DataEdge.checkDoc(doc)

        assertEquals(1, failures.size, failures.toString())
        assertTrue(failures[0].startsWith("iscc_decode/test_0001_error: expected an error"), failures.toString())
    }
}
