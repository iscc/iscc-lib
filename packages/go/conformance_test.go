// Tests for ConformanceSelftest and its ISCC-SEQ runners — validates every
// conformance vector in data.json and proves each runner failure arm alone.
package iscc

import (
	"encoding/json"
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

// loadSeqData parses the ISCC-SEQ sections of the embedded data.json.
func loadSeqData(t *testing.T) map[string]map[string]seqVector {
	t.Helper()
	data, err := parseSeqConformanceData([]byte(conformanceData))
	if err != nil {
		t.Fatalf("parseSeqConformanceData: %v", err)
	}
	return data
}

// seqCase builds a single-case ISCC-SEQ section map from raw JSON fragments.
func seqCase(section, inputs, outputs string) map[string]map[string]seqVector {
	var in []json.RawMessage
	if err := json.Unmarshal([]byte(inputs), &in); err != nil {
		panic(err)
	}
	return map[string]map[string]seqVector{
		section: {"case": {Inputs: in, Outputs: json.RawMessage(outputs)}},
	}
}

func TestSeqSectionsRun(t *testing.T) {
	data := loadSeqData(t)
	for _, section := range seqSections {
		if len(data[section]) == 0 {
			t.Fatalf("section %s has no vectors", section)
		}
	}
	if !runEncodeSeqTests(data) {
		t.Error("runEncodeSeqTests failed on the real vectors")
	}
	if !runDecodeSeqTests(data) {
		t.Error("runDecodeSeqTests failed on the real vectors")
	}
}

func TestSeqSectionsMissingFail(t *testing.T) {
	empty := map[string]map[string]seqVector{}
	if runEncodeSeqTests(empty) {
		t.Error("runEncodeSeqTests passed without an encode_seq section")
	}
	if runDecodeSeqTests(empty) {
		t.Error("runDecodeSeqTests passed without a decode_seq section")
	}
}

func TestEncodeSeqRunnerMutatedVectorFails(t *testing.T) {
	data := loadSeqData(t)
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
		mutated := map[string]map[string]seqVector{
			"encode_seq": {name: {Inputs: tc.Inputs, Outputs: mutatedOut}},
		}
		if runEncodeSeqTests(mutated) {
			t.Errorf("runEncodeSeqTests passed a mutated %s", name)
		}
		return
	}
	t.Fatal("no encode_seq vector to mutate")
}

func TestDecodeSeqRunnerMutatedVectorFails(t *testing.T) {
	data := loadSeqData(t)
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
		mutated := map[string]map[string]seqVector{
			"decode_seq": {name: {Inputs: tc.Inputs, Outputs: mutatedOut}},
		}
		if runDecodeSeqTests(mutated) {
			t.Errorf("runDecodeSeqTests passed a mutated %s", name)
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
		if runEncodeSeqTests(seqCase("encode_seq", c[0], c[1])) {
			t.Errorf("runEncodeSeqTests passed the %s case", name)
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
		if runDecodeSeqTests(seqCase("decode_seq", c[0], c[1])) {
			t.Errorf("runDecodeSeqTests passed the %s case", name)
		}
	}
}

func TestDecodeSeqRunnerPositiveCase(t *testing.T) {
	// Guards the failure-arm tests: the same shape with a correct output passes.
	if !runDecodeSeqTests(seqCase("decode_seq", `["bytes:300000000000"]`, `["ISCC:GAAAAAAAAA"]`)) {
		t.Error("runDecodeSeqTests failed a correct hand-built case")
	}
	if !runEncodeSeqTests(seqCase("encode_seq", `[["ISCC:GAAAAAAAAA"]]`, `"bytes:300000000000"`)) {
		t.Error("runEncodeSeqTests failed a correct hand-built case")
	}
}
