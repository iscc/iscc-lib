// Shared edge-case vectors (testdata/data-edge.json) run against the .NET binding.
// Every vector runs, `except.dotnet` is the only deviation, and a function section this
// harness does not handle fails. A rejection is asserted on the raw call alone (it must
// throw IsccException); the result is converted to its fixture form only after a
// successful call.

using System.Text.Json;
using System.Text.Json.Nodes;
using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Runs every data-edge.json vector against the .NET binding.</summary>
public class DataEdgeTests
{
    /// <summary>Surface id of this harness in the <c>except</c> map.</summary>
    private const string Surface = "dotnet";

    /// <summary>A prepared vector: the raw binding call and the converter for its successful result.</summary>
    private sealed record Prepared(Func<object> Call, Func<object, JsonNode> Convert);

    /// <summary>Section name to caller, which turns a vector's inputs into a prepared call.</summary>
    private static readonly Dictionary<string, Func<JsonElement, Prepared>> Sections = new()
    {
        ["iscc_decode"] = CallDecode,
        ["encode_seq"] = CallSeq,
        ["gen_text_code_v0"] = CallText,
        ["gen_mixed_code_v0"] = CallMixed,
        ["gen_iscc_code_v0"] = CallIsccCode,
    };

    /// <summary>Load and parse testdata/data-edge.json from the test output directory.</summary>
    private static JsonElement LoadDoc()
    {
        string path = Path.Combine(AppContext.BaseDirectory, "testdata", "data-edge.json");
        return JsonDocument.Parse(File.ReadAllText(path, System.Text.Encoding.UTF8)).RootElement;
    }

    /// <summary>Encode bytes as a <c>bytes:&lt;hex&gt;</c> fixture value.</summary>
    private static JsonNode BytesValue(byte[] data) =>
        JsonValue.Create("bytes:" + Convert.ToHexString(data).ToLowerInvariant());

    /// <summary>Convert a JSON array of strings to a string array.</summary>
    private static string[] ToStringArray(JsonElement array)
    {
        string[] result = new string[array.GetArrayLength()];
        for (int i = 0; i < result.Length; i++)
            result[i] = array[i].GetString()!;
        return result;
    }

    /// <summary>Wrap an ISCC string as an <c>{iscc}</c> object.</summary>
    private static JsonNode IsccObject(string iscc) => new JsonObject { ["iscc"] = iscc };

    /// <summary>Convert an IsccDecode result to <c>[mt, st, vs, length, "bytes:&lt;hex&gt;"]</c>.</summary>
    private static JsonNode DecodeArray(object result)
    {
        var r = (DecodeResult)result;
        return new JsonArray(
            (int)r.Maintype, (int)r.Subtype, (int)r.Version, (int)r.Length, BytesValue(r.Digest));
    }

    /// <summary>Prepare IsccDecode on the vector's string input.</summary>
    private static Prepared CallDecode(JsonElement inputs)
    {
        string iscc = inputs[0].GetString()!;
        return new Prepared(() => IsccLib.IsccDecode(iscc), DecodeArray);
    }

    /// <summary>Prepare EncodeSeq on the vector's unit list.</summary>
    private static Prepared CallSeq(JsonElement inputs)
    {
        string[] units = ToStringArray(inputs[0]);
        return new Prepared(() => IsccLib.EncodeSeq(units), r => BytesValue((byte[])r));
    }

    /// <summary>Prepare GenTextCodeV0 on the vector's text and bits.</summary>
    private static Prepared CallText(JsonElement inputs)
    {
        string text = inputs[0].GetString()!;
        uint bits = inputs[1].GetUInt32();
        return new Prepared(
            () => IsccLib.GenTextCodeV0(text, bits), r => IsccObject(((TextCodeResult)r).Iscc));
    }

    /// <summary>Prepare GenMixedCodeV0 on the vector's codes and bits.</summary>
    private static Prepared CallMixed(JsonElement inputs)
    {
        string[] codes = ToStringArray(inputs[0]);
        uint bits = inputs[1].GetUInt32();
        return new Prepared(
            () => IsccLib.GenMixedCodeV0(codes, bits), r => IsccObject(((MixedCodeResult)r).Iscc));
    }

    /// <summary>Prepare GenIsccCodeV0 (the optional second input is <c>wide</c>, default false).</summary>
    private static Prepared CallIsccCode(JsonElement inputs)
    {
        string[] codes = ToStringArray(inputs[0]);
        bool wide = inputs.GetArrayLength() > 1 && inputs[1].GetBoolean();
        return new Prepared(
            () => IsccLib.GenIsccCodeV0(codes, wide), r => IsccObject(((IsccCodeResult)r).Iscc));
    }

    /// <summary>Serialize a JSON node compactly for comparison and messages.</summary>
    private static string Text(JsonNode? node) => node?.ToJsonString() ?? "null";

    /// <summary>
    /// Compare an actual output with the expected one and return a failure, or null.
    /// For an object, only the expected keys the result exposes are compared.
    /// </summary>
    private static string? Compare(JsonNode expected, JsonNode actual)
    {
        if (expected is not JsonObject want)
            return Text(expected) == Text(actual) ? null : $"expected {Text(expected)}, got {Text(actual)}";
        var got = (JsonObject)actual;
        var keys = got.Select(kv => kv.Key).Where(want.ContainsKey).ToList();
        if (keys.Count == 0)
            return $"no expected field in {Text(got)}";
        foreach (string key in keys)
        {
            if (Text(want[key]) != Text(got[key]))
                return $"field {key}: expected {Text(want[key])}, got {Text(got[key])}";
        }
        return null;
    }

    /// <summary>Return this surface's <c>except</c> entry of a vector, or null.</summary>
    private static string? ExceptEntry(JsonElement vector)
    {
        if (!vector.TryGetProperty("except", out JsonElement except)) return null;
        return except.TryGetProperty(Surface, out JsonElement entry) ? entry.GetString() : null;
    }

    /// <summary>Judge a successful raw call against the vector's expectation.</summary>
    private static string? JudgeOk(bool expectError, Prepared prepared, object result, JsonElement vector)
    {
        JsonNode got = prepared.Convert(result);
        if (expectError)
            return $"expected an error, got {Text(got)}";
        if (!vector.TryGetProperty("outputs", out JsonElement outputs))
            return "vector has neither outputs nor error";
        return Compare(JsonNode.Parse(outputs.GetRawText())!, got);
    }

    /// <summary>Check one vector and return a failure description, or null.</summary>
    private static string? CheckVector(Func<JsonElement, Prepared> caller, JsonElement vector)
    {
        string? exception = ExceptEntry(vector);
        if (exception == "skip") return null;
        bool expectError = exception == "error" || vector.TryGetProperty("error", out _);
        Prepared prepared = caller(vector.GetProperty("inputs"));
        object result;
        try
        {
            result = prepared.Call();
        }
        catch (IsccException e)
        {
            return expectError ? null : $"expected an output, got error: {e.Message}";
        }
        catch (Exception e)
        {
            return $"raw call threw {e.GetType().FullName}: {e.Message}";
        }
        return JudgeOk(expectError, prepared, result, vector);
    }

    /// <summary>Run every vector of a parsed document; return one <c>section/vector: reason</c> per failure.</summary>
    private static List<string> CheckDoc(JsonElement doc)
    {
        var failures = new List<string>();
        foreach (JsonProperty section in doc.EnumerateObject())
        {
            if (!Sections.TryGetValue(section.Name, out var caller))
            {
                failures.Add($"{section.Name}: unknown section");
                continue;
            }
            foreach (JsonProperty vector in section.Value.EnumerateObject())
            {
                string? reason = CheckVector(caller, vector.Value);
                if (reason != null) failures.Add($"{section.Name}/{vector.Name}: {reason}");
            }
        }
        return failures;
    }

    /// <summary>Parse a synthetic document for the negative tests.</summary>
    private static JsonElement Parse(string json) => JsonDocument.Parse(json).RootElement;

    [Fact]
    public void DataEdgeVectorsPass()
    {
        JsonElement doc = LoadDoc();
        foreach (string section in Sections.Keys)
        {
            bool present = doc.TryGetProperty(section, out JsonElement vectors)
                && vectors.EnumerateObject().Any();
            Assert.True(present, $"section {section} has no vectors");
        }
        List<string> failures = CheckDoc(doc);
        Assert.True(failures.Count == 0, "data-edge.json failures:\n" + string.Join("\n", failures));
    }

    [Fact]
    public void DataEdgeFailsUnknownSection()
    {
        List<string> failures = CheckDoc(Parse(
            """{"gen_unknown_v0": {"test_0001_x": {"inputs": ["x"], "outputs": "x", "note": "n"}}}"""));
        Assert.Single(failures);
        Assert.Contains("unknown section", failures[0]);
    }

    [Fact]
    public void DataEdgeFailsExpectedErrorThatSucceeds()
    {
        // Valid inputs marked `error` prove each rejection-only caller reaches the binding
        // (gen_text_code_v0 is one on .NET: its NUL vector carries except.dotnet = error).
        List<string> failures = CheckDoc(Parse("""
            {"gen_mixed_code_v0": {"test_0001_x": {"inputs": [["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"], 64],
                "error": "e", "note": "n"}},
             "gen_iscc_code_v0": {"test_0001_x": {"inputs": [["GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
                "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]], "error": "e", "note": "n"}},
             "gen_text_code_v0": {"test_0001_x": {"inputs": ["Hello World", 64], "error": "e", "note": "n"}}}
            """));
        Assert.Equal(3, failures.Count);
        Assert.All(failures, f => Assert.Contains("expected an error", f));
    }

    [Fact]
    public void DataEdgeFailsOutputMismatch()
    {
        List<string> failures = CheckDoc(Parse("""
            {"encode_seq": {"test_0001_x": {"inputs": [["ISCC:GAAAAAAAAA"]],
                "outputs": "bytes:300000000001", "note": "n"}}}
            """));
        Assert.Single(failures);
        Assert.Contains("expected \"bytes:300000000001\"", failures[0]);
    }

    [Fact]
    public void DataEdgeHonoursExcept()
    {
        List<string> failures = CheckDoc(Parse("""
            {"iscc_decode": {
                "test_0001_error": {"inputs": ["ISCC:GAAAAAAAAA"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
                    "note": "n", "except": {"dotnet": "error"}},
                "test_0002_skip": {"inputs": ["ISCC:GAAAAAAAAB"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
                    "note": "n", "except": {"dotnet": "skip"}}}}
            """));
        Assert.Single(failures);
        Assert.StartsWith("iscc_decode/test_0001_error: expected an error", failures[0]);
    }
}
