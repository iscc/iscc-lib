/** Conformance tests for all 9 gen_*_v0 functions against data.json vectors. */
package uniffi.iscc_uniffi

import com.google.gson.JsonElement
import com.google.gson.JsonObject
import com.google.gson.JsonParser
import java.util.HexFormat
import java.util.TreeMap
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ConformanceTest {

    companion object {
        /** Cached parsed data.json for all test methods. */
        val dataJson: JsonObject by lazy {
            val stream = ConformanceTest::class.java.classLoader.getResourceAsStream("data.json")
                ?: error("data.json not found in test resources")
            val text = stream.bufferedReader().use { it.readText() }
            JsonParser.parseString(text).asJsonObject
        }

        /** Decode a "stream:<hex>" string to ByteArray. */
        fun decodeStream(streamStr: String): ByteArray {
            val hex = streamStr.removePrefix("stream:")
            if (hex.isEmpty()) return ByteArray(0)
            return HexFormat.of().parseHex(hex)
        }

        /** Prepare the meta parameter from a JSON element (null, string, or object). */
        fun prepareMeta(element: JsonElement): String? {
            if (element.isJsonNull) return null
            if (element.isJsonPrimitive && element.asJsonPrimitive.isString) return element.asString
            if (element.isJsonObject) {
                val sorted = TreeMap<String, JsonElement>()
                for ((key, value) in element.asJsonObject.entrySet()) {
                    sorted[key] = value
                }
                val obj = JsonObject()
                for ((key, value) in sorted) {
                    obj.add(key, value)
                }
                return obj.toString()
            }
            error("Unexpected meta type: $element")
        }

        /** Load test vectors for a given function name as sorted key-value pairs. */
        fun vectors(functionName: String): List<Pair<String, JsonObject>> {
            val section = dataJson.getAsJsonObject(functionName) ?: return emptyList()
            return section.entrySet()
                .sortedBy { it.key }
                .map { it.key to it.value.asJsonObject }
        }
    }

    // -- gen_meta_code_v0 --

    @Test
    fun testGenMetaCodeV0() {
        val vectors = vectors("gen_meta_code_v0")
        assertEquals(20, vectors.size, "Expected 20 meta code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val nameStr = inputs[0].asString
            val descStr = inputs[1].asString
            val description: String? = descStr.ifEmpty { null }
            val meta = prepareMeta(inputs[2])
            val bits = inputs[3].asNumber.toInt().toUInt()

            val result = genMetaCodeV0(nameStr, description, meta, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_text_code_v0 --

    @Test
    fun testGenTextCodeV0() {
        val vectors = vectors("gen_text_code_v0")
        assertEquals(5, vectors.size, "Expected 5 text code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val text = inputs[0].asString
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genTextCodeV0(text, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_image_code_v0 --

    @Test
    fun testGenImageCodeV0() {
        val vectors = vectors("gen_image_code_v0")
        assertEquals(3, vectors.size, "Expected 3 image code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val pixelArray = inputs[0].asJsonArray
            val pixels = ByteArray(pixelArray.size()) { pixelArray[it].asInt.toByte() }
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genImageCodeV0(pixels, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_audio_code_v0 --

    @Test
    fun testGenAudioCodeV0() {
        val vectors = vectors("gen_audio_code_v0")
        assertEquals(5, vectors.size, "Expected 5 audio code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val cvArray = inputs[0].asJsonArray
            val cv = List(cvArray.size()) { cvArray[it].asInt }
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genAudioCodeV0(cv, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_video_code_v0 --

    @Test
    fun testGenVideoCodeV0() {
        val vectors = vectors("gen_video_code_v0")
        assertEquals(3, vectors.size, "Expected 3 video code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val framesArray = inputs[0].asJsonArray
            val frameSigs = List(framesArray.size()) { i ->
                val frame = framesArray[i].asJsonArray
                List(frame.size()) { j -> frame[j].asInt }
            }
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genVideoCodeV0(frameSigs, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_mixed_code_v0 --

    @Test
    fun testGenMixedCodeV0() {
        val vectors = vectors("gen_mixed_code_v0")
        assertEquals(2, vectors.size, "Expected 2 mixed code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val codesArray = inputs[0].asJsonArray
            val codes = List(codesArray.size()) { codesArray[it].asString }
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genMixedCodeV0(codes, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_data_code_v0 --

    @Test
    fun testGenDataCodeV0() {
        val vectors = vectors("gen_data_code_v0")
        assertEquals(4, vectors.size, "Expected 4 data code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val data = decodeStream(inputs[0].asString)
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genDataCodeV0(data, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_instance_code_v0 --

    @Test
    fun testGenInstanceCodeV0() {
        val vectors = vectors("gen_instance_code_v0")
        assertEquals(3, vectors.size, "Expected 3 instance code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val data = decodeStream(inputs[0].asString)
            val bits = inputs[1].asNumber.toInt().toUInt()

            val result = genInstanceCodeV0(data, bits)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }

    // -- gen_iscc_code_v0 --

    @Test
    fun testGenIsccCodeV0() {
        val vectors = vectors("gen_iscc_code_v0")
        assertEquals(5, vectors.size, "Expected 5 iscc code vectors")

        for ((name, tc) in vectors) {
            val inputs = tc.getAsJsonArray("inputs")
            val outputs = tc.getAsJsonObject("outputs")

            val codesArray = inputs[0].asJsonArray
            val codes = List(codesArray.size()) { codesArray[it].asString }
            // gen_iscc_code_v0 vectors have no wide parameter -- always pass false
            val result = genIsccCodeV0(codes, false)
            assertEquals(outputs["iscc"].asString, result.iscc, "Failed vector: $name")
        }
    }
}
