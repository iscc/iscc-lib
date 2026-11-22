// Tests for ConformanceSelftest and its section runner — validates every
// conformance vector in data.json and proves each runner failure arm alone.
package iscc

import (
	"encoding/json"
	"errors"
	"sync"
	"testing"
)

func TestPureGoConformanceSelftest(t *testing.T) {
	passed, err := ConformanceSelftest()
	if err != nil {
		t.Fatalf("ConformanceSelftest returned error: %v", err)
	}
	if !passed {
		t.Fatal("ConformanceSelftest returned false — one or more vectors failed (see stderr)")
	}
}

// loadConformance parses the embedded data.json once per test binary.
var loadConformance = sync.OnceValues(func() (map[string]map[string]vectorEntry, error) {
	return parseConformanceData([]byte(conformanceData))
})

// loadConformanceData returns every section of the embedded data.json.
func loadConformanceData(t *testing.T) map[string]map[string]vectorEntry {
	t.Helper()
	data, err := loadConformance()
	if err != nil {
		t.Fatalf("parseConformanceData: %v", err)
	}
	return data
}

// conformanceSection returns one non-empty section of the embedded data.json.
func conformanceSection(t *testing.T, name string) map[string]vectorEntry {
	t.Helper()
	cases := loadConformanceData(t)[name]
	if len(cases) == 0 {
		t.Fatalf("section %s has no vectors", name)
	}
	return cases
}

// seqCase builds a single-case ISCC-SEQ section map from raw JSON fragments.
func seqCase(section, inputs, outputs string) map[string]map[string]vectorEntry {
	var in []json.RawMessage
	if err := json.Unmarshal([]byte(inputs), &in); err != nil {
		panic(err)
	}
	return map[string]map[string]vectorEntry{
		section: {"case": {Inputs: in, Outputs: json.RawMessage(outputs)}},
	}
}

func TestSeqSectionsRun(t *testing.T) {
	data := loadConformanceData(t)
	for _, section := range []string{"encode_seq", "decode_seq"} {
		conformanceSection(t, section)
	}
	if !runSection(data, "encode_seq", runEncodeSeqCase) {
		t.Error("encode_seq runner failed on the real vectors")
	}
	if !runSection(data, "decode_seq", runDecodeSeqCase) {
		t.Error("decode_seq runner failed on the real vectors")
	}
}

func TestSeqSectionsMissingFail(t *testing.T) {
	empty := map[string]map[string]vectorEntry{}
	if runSection(empty, "encode_seq", runEncodeSeqCase) {
		t.Error("encode_seq runner passed without an encode_seq section")
	}
	if runSection(empty, "decode_seq", runDecodeSeqCase) {
		t.Error("decode_seq runner passed without a decode_seq section")
	}
}

func TestEncodeSeqRunnerMutatedVectorFails(t *testing.T) {
	data := loadConformanceData(t)
	for name, tc := range data["encode_seq"] {
		var out string
		if err := json.Unmarshal(tc.Outputs, &out); err != nil {
			t.Fatal(err)
		}
		// Flip the last hex digit of the expected bytes.
		last := out[len(out)-1]
		flipped := byte('0')
		if last == '0' {
			flipped = '1'
		}
		mutatedOut, _ := json.Marshal(out[:len(out)-1] + string(flipped))
		mutated := map[string]map[string]vectorEntry{
			"encode_seq": {name: {Inputs: tc.Inputs, Outputs: mutatedOut}},
		}
		if runSection(mutated, "encode_seq", runEncodeSeqCase) {
			t.Errorf("encode_seq runner passed a mutated %s", name)
		}
		return
	}
	t.Fatal("no encode_seq vector to mutate")
}

func TestDecodeSeqRunnerMutatedVectorFails(t *testing.T) {
	data := loadConformanceData(t)
	for name, tc := range data["decode_seq"] {
		var out []string
		if err := json.Unmarshal(tc.Outputs, &out); err != nil {
			t.Fatal(err)
		}
		// Swap the expected order of units, or rename a single unit.
		if len(out) > 1 {
			out[0], out[1] = out[1], out[0]
		} else {
			out[0] += "A"
		}
		mutatedOut, _ := json.Marshal(out)
		mutated := map[string]map[string]vectorEntry{
			"decode_seq": {name: {Inputs: tc.Inputs, Outputs: mutatedOut}},
		}
		if runSection(mutated, "decode_seq", runDecodeSeqCase) {
			t.Errorf("decode_seq runner passed a mutated %s", name)
		}
		return
	}
	t.Fatal("no decode_seq vector to mutate")
}

func TestEncodeSeqRunnerFailureArms(t *testing.T) {
	// One bad case per call, so each failure arm is proven alone.
	cases := map[string][2]string{
		"mismatch":     {`[["ISCC:GAAAAAAAAA"]]`, `"bytes:00"`},
		"error":        {`[[""]]`, `"bytes:00"`},
		"parse_input":  {`[[1]]`, `"bytes:00"`},
		"parse_output": {`[["ISCC:GAAAAAAAAA"]]`, `"300000000000"`},
		"no_inputs":    {`[]`, `"bytes:300000000000"`},
	}
	for name, c := range cases {
		if runSection(seqCase("encode_seq", c[0], c[1]), "encode_seq", runEncodeSeqCase) {
			t.Errorf("encode_seq runner passed the %s case", name)
		}
	}
}

func TestDecodeSeqRunnerFailureArms(t *testing.T) {
	// One bad case per call, so each failure arm is proven alone.
	cases := map[string][2]string{
		"mismatch":     {`["bytes:300000000000"]`, `["ISCC:X"]`},
		"error":        {`["bytes:"]`, `[]`},
		"parse_input":  {`["300000000000"]`, `["ISCC:GAAAAAAAAA"]`},
		"parse_output": {`["bytes:300000000000"]`, `"ISCC:GAAAAAAAAA"`},
		"no_inputs":    {`[]`, `["ISCC:GAAAAAAAAA"]`},
	}
	for name, c := range cases {
		if runSection(seqCase("decode_seq", c[0], c[1]), "decode_seq", runDecodeSeqCase) {
			t.Errorf("decode_seq runner passed the %s case", name)
		}
	}
}

func TestDecodeSeqRunnerPositiveCase(t *testing.T) {
	// Guards the failure-arm tests: the same shape with a correct output passes.
	if !runSection(seqCase("decode_seq", `["bytes:300000000000"]`, `["ISCC:GAAAAAAAAA"]`), "decode_seq", runDecodeSeqCase) {
		t.Error("decode_seq runner failed a correct hand-built case")
	}
	if !runSection(seqCase("encode_seq", `[["ISCC:GAAAAAAAAA"]]`, `"bytes:300000000000"`), "encode_seq", runEncodeSeqCase) {
		t.Error("encode_seq runner failed a correct hand-built case")
	}
}

// genCase builds a single-case gen_data_code_v0 section map from raw JSON fragments.
func genCase(inputs, outputs string) map[string]map[string]vectorEntry {
	var tc vectorEntry
	if err := json.Unmarshal([]byte(`{"inputs": `+inputs+`, "outputs": `+outputs+`}`), &tc); err != nil {
		panic(err)
	}
	return map[string]map[string]vectorEntry{"gen_data_code_v0": {"case": tc}}
}

func TestGenSectionsRun(t *testing.T) {
	data := loadConformanceData(t)
	sections := map[string]func(vectorEntry) (string, string, error){
		"gen_meta_code_v0":     runMetaCase,
		"gen_text_code_v0":     runTextCase,
		"gen_image_code_v0":    runImageCase,
		"gen_audio_code_v0":    runAudioCase,
		"gen_video_code_v0":    runVideoCase,
		"gen_mixed_code_v0":    runMixedCase,
		"gen_data_code_v0":     runDataCase,
		"gen_instance_code_v0": runInstanceCase,
		"gen_iscc_code_v0":     runIsccCase,
	}
	for section, runCase := range sections {
		conformanceSection(t, section)
		if !runSection(data, section, runCase) {
			t.Errorf("%s runner failed on the real vectors", section)
		}
	}
}

func TestGenSectionMissingFails(t *testing.T) {
	if runSection(map[string]map[string]vectorEntry{}, "gen_data_code_v0", runDataCase) {
		t.Error("gen_data_code_v0 runner passed without its section")
	}
}

func TestGenSectionPositiveCase(t *testing.T) {
	// Guards the failure-arm test: the same shape with a correct output passes.
	result, err := GenDataCodeV0([]byte{}, 64)
	if err != nil {
		t.Fatal(err)
	}
	outputs, _ := json.Marshal(map[string]string{"iscc": result.Iscc})
	if !runSection(genCase(`["stream:", 64]`, string(outputs)), "gen_data_code_v0", runDataCase) {
		t.Error("gen_data_code_v0 runner failed a correct hand-built case")
	}
}

func TestGenSectionFailureArms(t *testing.T) {
	// One bad case per call, so each failure arm is proven alone.
	if _, err := GenDataCodeV0([]byte{}, 7); err == nil {
		t.Fatal("bits 7 must reach the error arm")
	}
	cases := map[string][2]string{
		"mismatch":     {`["stream:", 64]`, `{"iscc": "ISCC:X"}`},
		"error":        {`["stream:", 7]`, `{"iscc": "ISCC:X"}`},
		"parse_input":  {`[1, 64]`, `{"iscc": "ISCC:X"}`},
		"parse_stream": {`["stream:zz", 64]`, `{"iscc": "ISCC:X"}`},
		"no_inputs":    {`[]`, `{"iscc": "ISCC:X"}`},
		"bare_hex":     {`["00ff", 64]`, `{"iscc": "ISCC:X"}`},
		"no_iscc":      {`["stream:", 64]`, `{}`},
	}
	for name, c := range cases {
		if runSection(genCase(c[0], c[1]), "gen_data_code_v0", runDataCase) {
			t.Errorf("gen_data_code_v0 runner passed the %s case", name)
		}
	}
}

func TestGenRunnerRejectsMalformedVectorsAsUnparsable(t *testing.T) {
	// The Rust runner's policy: a stream without "stream:" and a case without
	// a string "iscc" output fail the parse arm, not the comparison.
	cases := map[string][2]string{
		"bare_hex":  {`["00ff", 64]`, `{"iscc": "ISCC:X"}`},
		"no_iscc":   {`["stream:", 64]`, `{}`},
		"null_iscc": {`["stream:", 64]`, `{"iscc": null}`},
		"seq_shape": {`["stream:", 64]`, `"bytes:00"`},
	}
	for name, c := range cases {
		tc := genCase(c[0], c[1])["gen_data_code_v0"]["case"]
		if _, _, err := runDataCase(tc); !errors.Is(err, errUnparsable) {
			t.Errorf("%s: got error %v, want errUnparsable", name, err)
		}
	}
}
