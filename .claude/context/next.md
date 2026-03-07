# Next Work Package

## Step: C# conformance tests (ConformanceTests.cs + vendored data.json)

## Goal

Add xUnit conformance tests for the C# binding that validate all 9 gen functions (50 vectors)
against the official `data.json` test vectors. This ensures the .NET binding produces correct ISCC
codes matching the iscc-core reference implementation before further API refactoring.

## Scope

- **Create**:
    - `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` — xUnit test class, one `[Theory]` per gen
        function
    - `packages/dotnet/Iscc.Lib.Tests/testdata/data.json` — vendored copy of
        `crates/iscc-lib/tests/data.json`
- **Modify**:
    - `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — add `<Content>` item for
        `testdata/data.json` with `CopyToOutputDirectory` so xUnit can load it at test runtime
- **Reference**:
    - `crates/iscc-rb/test/test_conformance.rb` — pattern for conformance test structure, input
        decoding, output assertions per function group
    - `crates/iscc-lib/tests/data.json` — source vectors (50 total, 9 function groups + `_metadata`)
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — current C# API signatures (all gen functions return
        `string` except `GenSumCodeV0` → `SumCodeResult`)
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — existing test patterns

## Not In Scope

- **Structured record return types** (`Results.cs`, `MetaCodeResult`, etc.) — the conformance tests
    validate the ISCC string output using the current string-returning API; structured records are a
    separate future step
- **Modifying `IsccLib.cs`** — no API changes; tests work against the current public surface
- **Adding `gen_sum_code_v0` conformance tests** — no conformance vectors exist for this function in
    data.json (it's the 10th gen function but only 9 have vectors)
- **Documentation updates** — `docs/howto/dotnet.md`, README C# section, etc. are separate steps
- **NuGet publish pipeline** — separate step

## Implementation Notes

### data.json Structure

Top-level keys: `_metadata` (skip), then 9 function groups (`gen_meta_code_v0`..`gen_iscc_code_v0`).
Each function group maps vector names to `{"inputs": [...], "outputs": {...}}` objects. Total: 50
vectors.

### Input Decoding Per Function

Follow the Ruby conformance test (`test_conformance.rb`) pattern for input decoding:

- **`gen_meta_code_v0`**: `inputs[0]` = name (string), `inputs[1]` = description (string, empty →
    `null`), `inputs[2]` = meta (null, string, or JSON object — if object, serialize to JSON
    string), `inputs[3]` = bits (uint). Call `IsccLib.GenMetaCodeV0(name, description, meta, bits)`.
- **`gen_text_code_v0`**: `inputs[0]` = text, `inputs[1]` = bits. Call
    `IsccLib.GenTextCodeV0(text, bits)`.
- **`gen_image_code_v0`**: `inputs[0]` = JSON array of integers → `byte[]` pixels (cast each int to
    byte), `inputs[1]` = bits. Call `IsccLib.GenImageCodeV0(pixels, bits)`.
- **`gen_audio_code_v0`**: `inputs[0]` = JSON array of integers → `int[]` cv, `inputs[1]` = bits.
    Call `IsccLib.GenAudioCodeV0(cv, bits)`.
- **`gen_video_code_v0`**: `inputs[0]` = 2D JSON array → `int[][]` frameSigs, `inputs[1]` = bits.
    Call `IsccLib.GenVideoCodeV0(frameSigs, bits)`.
- **`gen_mixed_code_v0`**: `inputs[0]` = string array of ISCC codes, `inputs[1]` = bits. Call
    `IsccLib.GenMixedCodeV0(codes, bits)`.
- **`gen_data_code_v0`**: `inputs[0]` = `"stream:<hex>"` string → decode hex to `byte[]`,
    `inputs[1]` = bits. Call `IsccLib.GenDataCodeV0(data, bits)`.
- **`gen_instance_code_v0`**: Same `"stream:<hex>"` decoding as data code. Call
    `IsccLib.GenInstanceCodeV0(data, bits)`.
- **`gen_iscc_code_v0`**: `inputs[0]` = string array of ISCC codes. No `wide` param in vectors —
    call `IsccLib.GenIsccCodeV0(codes)`.

### Output Assertions

All functions: assert `result == outputs["iscc"]` (the returned string IS the ISCC code since
current API returns `string`). When structured records are added later, tests will need to assert
`result.Iscc` instead.

### xUnit Pattern

Use `[Theory]` with `[MemberData]` for each function group. Load `data.json` from a static helper.
Each theory test receives the vector name and test case data. This produces one test result per
vector for clear diagnostics.

```csharp
public static IEnumerable<object[]> MetaCodeVectors()
{
    var data = LoadDataJson();
    foreach (var kv in data["gen_meta_code_v0"].EnumerateObject())
        yield return new object[] { kv.Name, kv.Value };
}

[Theory]
[MemberData(nameof(MetaCodeVectors))]
public void GenMetaCodeV0(string vectorName, JsonElement tc) { ... }
```

Use `System.Text.Json` (built-in, no extra dependency) rather than Newtonsoft.Json.

### .csproj Content Item

```xml
<ItemGroup>
  <Content Include="testdata\data.json">
    <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
  </Content>
</ItemGroup>
```

### Stream Hex Decoding

```csharp
static byte[] DecodeStream(string streamStr)
{
    var hex = streamStr["stream:".Length..];
    if (hex.Length == 0) return Array.Empty<byte>();
    return Convert.FromHexString(hex);
}
```

### Meta Parameter Handling

If `inputs[2]` is a JSON object (not null, not a string), serialize it to a JSON string using
`JsonSerializer.Serialize` with default options. The C FFI layer handles JCS canonicalization
internally.

## Verification

- `dotnet test packages/dotnet -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` passes — all 41
    existing smoke tests + 50 new conformance tests (91 total)
- `grep -c '\[Theory\]' packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` outputs `9` (one per gen
    function)
- `diff crates/iscc-lib/tests/data.json packages/dotnet/Iscc.Lib.Tests/testdata/data.json` shows no
    differences (identical copy)
- `cargo clippy --workspace --all-targets -- -D warnings` remains clean

## Done When

All verification criteria pass — 50 conformance vectors validate correct ISCC output for all 9 gen
functions through the C# P/Invoke bridge.
