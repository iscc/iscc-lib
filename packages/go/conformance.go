// Conformance selftest for ISO 24138:2024 (ISCC).
// Runs every gen function and the ISCC-SEQ codec (EncodeSeq/DecodeSeq) against
// the vendored conformance vectors from data.json and reports pass/fail. An
// application that claims ISCC conformance MUST pass all tests in this suite.
package iscc

import (
	"bytes"
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"slices"
	"strings"
)

//go:embed testdata/data.json
var conformanceData string

// vectorEntry holds a single conformance test case from data.json.
type vectorEntry struct {
	Inputs  []json.RawMessage      `json:"inputs"`
	Outputs map[string]interface{} `json:"outputs"`
}

// parseConformanceData unmarshals the gen_* function sections of data.json.
// Other top-level keys are skipped: "_metadata" holds no test vectors, and the
// codec sections (e.g., "encode_seq") have non-object outputs.
func parseConformanceData(raw []byte) (map[string]map[string]vectorEntry, error) {
	var sections map[string]json.RawMessage
	if err := json.Unmarshal(raw, &sections); err != nil {
		return nil, fmt.Errorf("could not parse conformance data: %w", err)
	}

	data := make(map[string]map[string]vectorEntry, len(sections))
	for key, rawSection := range sections {
		if !strings.HasPrefix(key, "gen_") {
			continue
		}
		var cases map[string]vectorEntry
		if err := json.Unmarshal(rawSection, &cases); err != nil {
			return nil, fmt.Errorf("could not parse section %s: %w", key, err)
		}
		data[key] = cases
	}
	return data, nil
}

// ConformanceSelftest validates every gen function and ISCC-SEQ conformance
// vector in data.json.
// Returns (true, nil) if all vectors pass, (false, nil) if any mismatch,
// and (false, error) if data.json cannot be parsed.
func ConformanceSelftest() (bool, error) {
	data, err := parseConformanceData([]byte(conformanceData))
	if err != nil {
		return false, err
	}
	seqData, err := parseSeqConformanceData([]byte(conformanceData))
	if err != nil {
		return false, err
	}

	passed := true
	passed = runMetaTests(data) && passed
	passed = runTextTests(data) && passed
	passed = runImageTests(data) && passed
	passed = runAudioTests(data) && passed
	passed = runVideoTests(data) && passed
	passed = runMixedTests(data) && passed
	passed = runDataTests(data) && passed
	passed = runInstanceTests(data) && passed
	passed = runIsccTests(data) && passed
	passed = runEncodeSeqTests(seqData) && passed
	passed = runDecodeSeqTests(seqData) && passed

	return passed, nil
}

// runMetaTests validates gen_meta_code_v0 conformance vectors.
func runMetaTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_meta_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		if !runMetaCase(funcName, tcName, tc) {
			passed = false
		}
	}
	return passed
}

// runMetaCase runs a single gen_meta_code_v0 test case.
func runMetaCase(funcName, tcName string, tc vectorEntry) bool {
	var inputName string
	if err := json.Unmarshal(tc.Inputs[0], &inputName); err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
		return false
	}

	// Description: string or null
	var desc *string
	var descStr string
	if string(tc.Inputs[1]) != "null" {
		if err := json.Unmarshal(tc.Inputs[1], &descStr); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			return false
		}
		if descStr != "" {
			desc = &descStr
		}
	}

	// Meta: null, string, or JSON object
	var meta *string
	rawMeta := string(tc.Inputs[2])
	if rawMeta != "null" {
		if rawMeta[0] == '"' {
			var metaStr string
			if err := json.Unmarshal(tc.Inputs[2], &metaStr); err != nil {
				fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
				return false
			}
			meta = &metaStr
		} else {
			metaStr := rawMeta
			meta = &metaStr
		}
	}

	var bits float64
	if err := json.Unmarshal(tc.Inputs[3], &bits); err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
		return false
	}

	expectedISCC, _ := tc.Outputs["iscc"].(string)
	result, err := GenMetaCodeV0(inputName, desc, meta, uint32(bits))
	if err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
		return false
	}
	if result.Iscc != expectedISCC {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
		return false
	}
	return true
}

// runTextTests validates gen_text_code_v0 conformance vectors.
func runTextTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_text_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var text string
		if err := json.Unmarshal(tc.Inputs[0], &text); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenTextCodeV0(text, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// runImageTests validates gen_image_code_v0 conformance vectors.
func runImageTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_image_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var pixelsJSON []float64
		if err := json.Unmarshal(tc.Inputs[0], &pixelsJSON); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}
		pixels := make([]byte, len(pixelsJSON))
		for i, v := range pixelsJSON {
			pixels[i] = byte(v)
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenImageCodeV0(pixels, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// runAudioTests validates gen_audio_code_v0 conformance vectors.
func runAudioTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_audio_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var cvJSON []float64
		if err := json.Unmarshal(tc.Inputs[0], &cvJSON); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}
		cv := make([]int32, len(cvJSON))
		for i, v := range cvJSON {
			cv[i] = int32(v)
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenAudioCodeV0(cv, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// runVideoTests validates gen_video_code_v0 conformance vectors.
func runVideoTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_video_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var framesJSON [][]float64
		if err := json.Unmarshal(tc.Inputs[0], &framesJSON); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}
		frameSigs := make([][]int32, len(framesJSON))
		for i, frame := range framesJSON {
			frameSigs[i] = make([]int32, len(frame))
			for j, v := range frame {
				frameSigs[i][j] = int32(v)
			}
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenVideoCodeV0(frameSigs, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// runMixedTests validates gen_mixed_code_v0 conformance vectors.
func runMixedTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_mixed_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var codes []string
		if err := json.Unmarshal(tc.Inputs[0], &codes); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenMixedCodeV0(codes, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// decodeStream decodes a "stream:<hex>" value into bytes.
func decodeStream(s string) ([]byte, error) {
	hexData := strings.TrimPrefix(s, "stream:")
	if hexData == "" {
		return []byte{}, nil
	}
	return hex.DecodeString(hexData)
}

// runDataTests validates gen_data_code_v0 conformance vectors.
func runDataTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_data_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var streamStr string
		if err := json.Unmarshal(tc.Inputs[0], &streamStr); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}
		inputBytes, err := decodeStream(streamStr)
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not decode stream: %v\n", funcName, tcName, err)
			passed = false
			continue
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenDataCodeV0(inputBytes, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// runInstanceTests validates gen_instance_code_v0 conformance vectors.
func runInstanceTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_instance_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var streamStr string
		if err := json.Unmarshal(tc.Inputs[0], &streamStr); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}
		inputBytes, err := decodeStream(streamStr)
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not decode stream: %v\n", funcName, tcName, err)
			passed = false
			continue
		}

		var bits float64
		if err := json.Unmarshal(tc.Inputs[1], &bits); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		result, err := GenInstanceCodeV0(inputBytes, uint32(bits))
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// runIsccTests validates gen_iscc_code_v0 conformance vectors.
func runIsccTests(data map[string]map[string]vectorEntry) bool {
	funcName := "gen_iscc_code_v0"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		var codes []string
		if err := json.Unmarshal(tc.Inputs[0], &codes); err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
			passed = false
			continue
		}

		expectedISCC, _ := tc.Outputs["iscc"].(string)
		// Conformance vectors use default (non-wide) mode
		result, err := GenIsccCodeV0(codes, false)
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
			passed = false
			continue
		}
		if result.Iscc != expectedISCC {
			fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, expectedISCC, result.Iscc)
			passed = false
		}
	}
	return passed
}

// seqVector holds a single ISCC-SEQ conformance test case from data.json.
// The outputs are a "bytes:<hex>" string (encode_seq) or a list of unit
// strings (decode_seq), so they stay raw until the runner parses them.
type seqVector struct {
	Inputs  []json.RawMessage `json:"inputs"`
	Outputs json.RawMessage   `json:"outputs"`
}

// seqSections names the ISCC-SEQ codec sections of data.json.
var seqSections = [...]string{"encode_seq", "decode_seq"}

// parseSeqConformanceData unmarshals the ISCC-SEQ sections of data.json.
// A missing section is left out, so its runner reports it as missing.
func parseSeqConformanceData(raw []byte) (map[string]map[string]seqVector, error) {
	var sections map[string]json.RawMessage
	if err := json.Unmarshal(raw, &sections); err != nil {
		return nil, fmt.Errorf("could not parse conformance data: %w", err)
	}

	data := make(map[string]map[string]seqVector, len(seqSections))
	for _, key := range seqSections {
		rawSection, ok := sections[key]
		if !ok {
			continue
		}
		var cases map[string]seqVector
		if err := json.Unmarshal(rawSection, &cases); err != nil {
			return nil, fmt.Errorf("could not parse section %s: %w", key, err)
		}
		data[key] = cases
	}
	return data, nil
}

// decodeBytesValue decodes a "bytes:<hex>" fixture value into bytes.
func decodeBytesValue(s string) ([]byte, error) {
	hexData, ok := strings.CutPrefix(s, "bytes:")
	if !ok {
		return nil, fmt.Errorf("missing bytes: prefix in %q", s)
	}
	return hex.DecodeString(hexData)
}

// runEncodeSeqTests validates encode_seq conformance vectors
// (inputs [[str…]], output "bytes:<hex>").
func runEncodeSeqTests(data map[string]map[string]seqVector) bool {
	funcName := "encode_seq"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		if !runEncodeSeqCase(funcName, tcName, tc) {
			passed = false
		}
	}
	return passed
}

// runEncodeSeqCase runs a single encode_seq test case.
func runEncodeSeqCase(funcName, tcName string, tc seqVector) bool {
	var units []string
	var outputStr string
	if len(tc.Inputs) < 1 || json.Unmarshal(tc.Inputs[0], &units) != nil ||
		json.Unmarshal(tc.Outputs, &outputStr) != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
		return false
	}
	expected, err := decodeBytesValue(outputStr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not decode output: %v\n", funcName, tcName, err)
		return false
	}

	seq, err := EncodeSeq(units)
	if err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
		return false
	}
	if !bytes.Equal(seq, expected) {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %x, got %x\n", funcName, tcName, expected, seq)
		return false
	}
	return true
}

// runDecodeSeqTests validates decode_seq conformance vectors
// (inputs ["bytes:<hex>"], output [str…]).
func runDecodeSeqTests(data map[string]map[string]seqVector) bool {
	funcName := "decode_seq"
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		if !runDecodeSeqCase(funcName, tcName, tc) {
			passed = false
		}
	}
	return passed
}

// runDecodeSeqCase runs a single decode_seq test case.
func runDecodeSeqCase(funcName, tcName string, tc seqVector) bool {
	var inputStr string
	var expected []string
	if len(tc.Inputs) < 1 || json.Unmarshal(tc.Inputs[0], &inputStr) != nil ||
		json.Unmarshal(tc.Outputs, &expected) != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
		return false
	}
	seq, err := decodeBytesValue(inputStr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not decode input: %v\n", funcName, tcName, err)
		return false
	}

	units, err := DecodeSeq(seq)
	if err != nil {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
		return false
	}
	if !slices.Equal(units, expected) {
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %v, got %v\n", funcName, tcName, expected, units)
		return false
	}
	return true
}
