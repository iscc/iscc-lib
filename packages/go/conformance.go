// Conformance selftest for ISO 24138:2024 (ISCC).
// Runs all 9 gen functions against vendored conformance vectors from data.json
// and reports pass/fail. An application that claims ISCC conformance MUST pass
// all tests in this suite.
package iscc

import (
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"strings"
)

//go:embed testdata/data.json
var conformanceData string

// vectorEntry holds a single conformance test case from data.json.
type vectorEntry struct {
	Inputs  []json.RawMessage     `json:"inputs"`
	Outputs map[string]interface{} `json:"outputs"`
}

// ConformanceSelftest validates all 46 conformance vectors from data.json.
// Returns (true, nil) if all vectors pass, (false, nil) if any mismatch,
// and (false, error) if data.json cannot be parsed.
func ConformanceSelftest() (bool, error) {
	var data map[string]map[string]vectorEntry
	if err := json.Unmarshal([]byte(conformanceData), &data); err != nil {
		return false, fmt.Errorf("could not parse conformance data: %w", err)
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
