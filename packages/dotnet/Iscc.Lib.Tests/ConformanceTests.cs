// Conformance tests for all 9 gen_*_v0 functions against data.json vectors.
// Validates the .NET binding produces correct ISCC codes matching the iscc-core reference.

using System.Text.Json;
using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates all 9 gen functions against official ISCC conformance vectors (50 total).</summary>
public class ConformanceTests
{
    /// <summary>Cached parsed data.json for all test methods.</summary>
    private static readonly Lazy<JsonElement> DataJson = new(LoadDataJson);

    /// <summary>Load and parse testdata/data.json from the test output directory.</summary>
    private static JsonElement LoadDataJson()
    {
        string path = Path.Combine(AppContext.BaseDirectory, "testdata", "data.json");
        string json = File.ReadAllText(path);
        return JsonDocument.Parse(json).RootElement;
    }

    /// <summary>Decode a "stream:&lt;hex&gt;" string to a byte array.</summary>
    private static byte[] DecodeStream(string streamStr)
    {
        string hex = streamStr["stream:".Length..];
        if (hex.Length == 0) return [];
        return Convert.FromHexString(hex);
    }

    /// <summary>Prepare the meta parameter from a JSON element (null, string, or object).</summary>
    private static string? PrepareMeta(JsonElement el)
    {
        if (el.ValueKind == JsonValueKind.Null) return null;
        if (el.ValueKind == JsonValueKind.String) return el.GetString();
        if (el.ValueKind == JsonValueKind.Object) return JsonSerializer.Serialize(el);
        throw new InvalidOperationException($"Unexpected meta type: {el.ValueKind}");
    }

    // ── gen_meta_code_v0 ─────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_meta_code_v0 (20 vectors).</summary>
    public static IEnumerable<object[]> MetaCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_meta_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(MetaCodeVectors))]
    public void GenMetaCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        string name = inputs[0].GetString()!;
        string descStr = inputs[1].GetString()!;
        string? description = descStr.Length == 0 ? null : descStr;
        string? meta = PrepareMeta(inputs[2]);
        uint bits = inputs[3].GetUInt32();

        string result = IsccLib.GenMetaCodeV0(name, description: description, meta: meta, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_text_code_v0 ─────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_text_code_v0 (5 vectors).</summary>
    public static IEnumerable<object[]> TextCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_text_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(TextCodeVectors))]
    public void GenTextCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        string text = inputs[0].GetString()!;
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenTextCodeV0(text, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_image_code_v0 ────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_image_code_v0 (3 vectors).</summary>
    public static IEnumerable<object[]> ImageCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_image_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(ImageCodeVectors))]
    public void GenImageCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        JsonElement pixelArray = inputs[0];
        byte[] pixels = new byte[pixelArray.GetArrayLength()];
        for (int i = 0; i < pixels.Length; i++)
            pixels[i] = (byte)pixelArray[i].GetInt32();
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenImageCodeV0(pixels, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_audio_code_v0 ────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_audio_code_v0 (5 vectors).</summary>
    public static IEnumerable<object[]> AudioCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_audio_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(AudioCodeVectors))]
    public void GenAudioCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        JsonElement cvArray = inputs[0];
        int[] cv = new int[cvArray.GetArrayLength()];
        for (int i = 0; i < cv.Length; i++)
            cv[i] = cvArray[i].GetInt32();
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenAudioCodeV0(cv, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_video_code_v0 ────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_video_code_v0 (3 vectors).</summary>
    public static IEnumerable<object[]> VideoCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_video_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(VideoCodeVectors))]
    public void GenVideoCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        JsonElement framesArray = inputs[0];
        int[][] frameSigs = new int[framesArray.GetArrayLength()][];
        for (int f = 0; f < frameSigs.Length; f++)
        {
            JsonElement frame = framesArray[f];
            frameSigs[f] = new int[frame.GetArrayLength()];
            for (int i = 0; i < frameSigs[f].Length; i++)
                frameSigs[f][i] = frame[i].GetInt32();
        }
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenVideoCodeV0(frameSigs, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_mixed_code_v0 ────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_mixed_code_v0 (2 vectors).</summary>
    public static IEnumerable<object[]> MixedCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_mixed_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(MixedCodeVectors))]
    public void GenMixedCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        JsonElement codesArray = inputs[0];
        string[] codes = new string[codesArray.GetArrayLength()];
        for (int i = 0; i < codes.Length; i++)
            codes[i] = codesArray[i].GetString()!;
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenMixedCodeV0(codes, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_data_code_v0 ─────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_data_code_v0 (4 vectors).</summary>
    public static IEnumerable<object[]> DataCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_data_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(DataCodeVectors))]
    public void GenDataCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        byte[] data = DecodeStream(inputs[0].GetString()!);
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenDataCodeV0(data, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_instance_code_v0 ─────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_instance_code_v0 (3 vectors).</summary>
    public static IEnumerable<object[]> InstanceCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_instance_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(InstanceCodeVectors))]
    public void GenInstanceCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        byte[] data = DecodeStream(inputs[0].GetString()!);
        uint bits = inputs[1].GetUInt32();

        string result = IsccLib.GenInstanceCodeV0(data, bits: bits);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }

    // ── gen_iscc_code_v0 ─────────────────────────────────────────────────────

    /// <summary>Yield test cases for gen_iscc_code_v0 (5 vectors).</summary>
    public static IEnumerable<object[]> IsccCodeVectors()
    {
        foreach (JsonProperty kv in DataJson.Value.GetProperty("gen_iscc_code_v0").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(IsccCodeVectors))]
    public void GenIsccCodeV0(string vectorName, JsonElement tc)
    {
        JsonElement inputs = tc.GetProperty("inputs");
        JsonElement outputs = tc.GetProperty("outputs");

        JsonElement codesArray = inputs[0];
        string[] codes = new string[codesArray.GetArrayLength()];
        for (int i = 0; i < codes.Length; i++)
            codes[i] = codesArray[i].GetString()!;

        string result = IsccLib.GenIsccCodeV0(codes);
        Assert.Equal(outputs.GetProperty("iscc").GetString(), result);
    }
}
