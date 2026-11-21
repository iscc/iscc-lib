// Conformance selftest for ISO 24138:2024 (ISCC).
// Runs every gen function and the ISCC-SEQ codec (EncodeSeq/DecodeSeq) against
// the vendored conformance vectors from data.json and reports pass/fail. An
// application that claims ISCC conformance MUST pass all tests in this suite.
package iscc

import (
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"os"
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
	passed = runSection(data, "gen_meta_code_v0", runMetaCase) && passed
	passed = runSection(data, "gen_text_code_v0", runTextCase) && passed
	passed = runSection(data, "gen_image_code_v0", runImageCase) && passed
	passed = runSection(data, "gen_audio_code_v0", runAudioCase) && passed
	passed = runSection(data, "gen_video_code_v0", runVideoCase) && passed
	passed = runSection(data, "gen_mixed_code_v0", runMixedCase) && passed
	passed = runSection(data, "gen_data_code_v0", runDataCase) && passed
	passed = runSection(data, "gen_instance_code_v0", runInstanceCase) && passed
	passed = runSection(data, "gen_iscc_code_v0", runIsccCase) && passed
	passed = runSection(seqData, "encode_seq", runEncodeSeqCase) && passed
	passed = runSection(seqData, "decode_seq", runDecodeSeqCase) && passed

	return passed, nil
}

// errUnparsable marks a conformance case whose inputs or expected outputs
// cannot be parsed.
var errUnparsable = errors.New("unparsable conformance case")

// runSection runs every case of one data.json section through runCase and
// reports each failure on stderr. runCase parses a case, calls the function
// under test and returns its actual and expected output rendered as strings.
// Returns false if the section is missing or any case fails.
func runSection[V any](data map[string]map[string]V, funcName string, runCase func(V) (got, want string, err error)) bool {
	cases, ok := data[funcName]
	if !ok {
		fmt.Fprintf(os.Stderr, "FAILED: %s section missing from conformance data\n", funcName)
		return false
	}

	passed := true
	for tcName, tc := range cases {
		got, want, err := runCase(tc)
		passed = reportCase(funcName, tcName, got, want, err) && passed
	}
	return passed
}

// reportCase prints the failure of one conformance case, if any, and returns
// whether the case passed.
func reportCase(funcName, tcName, got, want string, err error) bool {
	switch {
	case errors.Is(err, errUnparsable):
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — could not parse test inputs\n", funcName, tcName)
	case err != nil:
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — error: %v\n", funcName, tcName, err)
	case got != want:
		fmt.Fprintf(os.Stderr, "FAILED: %s.%s — expected %s, got %s\n", funcName, tcName, want, got)
	default:
		return true
	}
	return false
}

// unmarshalInputs decodes the leading test inputs into targets, in order.
// Returns errUnparsable if an input is missing or does not decode.
func unmarshalInputs(inputs []json.RawMessage, targets ...any) error {
	if len(inputs) < len(targets) {
		return errUnparsable
	}
	for i, target := range targets {
		if json.Unmarshal(inputs[i], target) != nil {
			return errUnparsable
		}
	}
	return nil
}

// expectedISCC returns the expected "iscc" output of a gen_* case.
func expectedISCC(tc vectorEntry) string {
	iscc, _ := tc.Outputs["iscc"].(string)
	return iscc
}

// metaArg converts a raw gen_meta_code_v0 meta input (null, string, or JSON
// object) into the meta argument of GenMetaCodeV0.
func metaArg(raw json.RawMessage) (*string, error) {
	text := string(raw)
	if text == "null" {
		return nil, nil
	}
	if raw[0] != '"' {
		return &text, nil
	}
	var meta string
	if json.Unmarshal(raw, &meta) != nil {
		return nil, errUnparsable
	}
	return &meta, nil
}

// runMetaCase runs a single gen_meta_code_v0 test case.
func runMetaCase(tc vectorEntry) (got, want string, err error) {
	var name, descStr string
	var rawMeta json.RawMessage
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &name, &descStr, &rawMeta, &bits); err != nil {
		return "", "", err
	}
	meta, err := metaArg(rawMeta)
	if err != nil {
		return "", "", err
	}
	// Description: an empty string or null means no description
	var desc *string
	if descStr != "" {
		desc = &descStr
	}

	result, err := GenMetaCodeV0(name, desc, meta, uint32(bits))
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runTextCase runs a single gen_text_code_v0 test case.
func runTextCase(tc vectorEntry) (got, want string, err error) {
	var text string
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &text, &bits); err != nil {
		return "", "", err
	}
	result, err := GenTextCodeV0(text, uint32(bits))
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runImageCase runs a single gen_image_code_v0 test case.
func runImageCase(tc vectorEntry) (got, want string, err error) {
	var pixelsJSON []float64
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &pixelsJSON, &bits); err != nil {
		return "", "", err
	}
	pixels := make([]byte, len(pixelsJSON))
	for i, v := range pixelsJSON {
		pixels[i] = byte(v)
	}

	result, err := GenImageCodeV0(pixels, uint32(bits))
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runAudioCase runs a single gen_audio_code_v0 test case.
func runAudioCase(tc vectorEntry) (got, want string, err error) {
	var cvJSON []float64
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &cvJSON, &bits); err != nil {
		return "", "", err
	}
	cv := make([]int32, len(cvJSON))
	for i, v := range cvJSON {
		cv[i] = int32(v)
	}

	result, err := GenAudioCodeV0(cv, uint32(bits))
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runVideoCase runs a single gen_video_code_v0 test case.
func runVideoCase(tc vectorEntry) (got, want string, err error) {
	var framesJSON [][]float64
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &framesJSON, &bits); err != nil {
		return "", "", err
	}
	frameSigs := make([][]int32, len(framesJSON))
	for i, frame := range framesJSON {
		frameSigs[i] = make([]int32, len(frame))
		for j, v := range frame {
			frameSigs[i][j] = int32(v)
		}
	}

	result, err := GenVideoCodeV0(frameSigs, uint32(bits))
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runMixedCase runs a single gen_mixed_code_v0 test case.
func runMixedCase(tc vectorEntry) (got, want string, err error) {
	var codes []string
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &codes, &bits); err != nil {
		return "", "", err
	}
	result, err := GenMixedCodeV0(codes, uint32(bits))
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// decodeStream decodes a "stream:<hex>" value into bytes.
func decodeStream(s string) ([]byte, error) {
	hexData := strings.TrimPrefix(s, "stream:")
	if hexData == "" {
		return []byte{}, nil
	}
	return hex.DecodeString(hexData)
}

// streamInputs decodes the "stream:<hex>" and bits inputs of a gen_data_code_v0
// or gen_instance_code_v0 case.
func streamInputs(tc vectorEntry) ([]byte, uint32, error) {
	var streamStr string
	var bits float64
	if err := unmarshalInputs(tc.Inputs, &streamStr, &bits); err != nil {
		return nil, 0, err
	}
	inputBytes, err := decodeStream(streamStr)
	if err != nil {
		return nil, 0, errUnparsable
	}
	return inputBytes, uint32(bits), nil
}

// runDataCase runs a single gen_data_code_v0 test case.
func runDataCase(tc vectorEntry) (got, want string, err error) {
	inputBytes, bits, err := streamInputs(tc)
	if err != nil {
		return "", "", err
	}
	result, err := GenDataCodeV0(inputBytes, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runInstanceCase runs a single gen_instance_code_v0 test case.
func runInstanceCase(tc vectorEntry) (got, want string, err error) {
	inputBytes, bits, err := streamInputs(tc)
	if err != nil {
		return "", "", err
	}
	result, err := GenInstanceCodeV0(inputBytes, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
}

// runIsccCase runs a single gen_iscc_code_v0 test case.
func runIsccCase(tc vectorEntry) (got, want string, err error) {
	var codes []string
	if err := unmarshalInputs(tc.Inputs, &codes); err != nil {
		return "", "", err
	}
	// Conformance vectors use default (non-wide) mode
	result, err := GenIsccCodeV0(codes, false)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, expectedISCC(tc), nil
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

// runEncodeSeqCase runs a single encode_seq test case
// (inputs [[str…]], output "bytes:<hex>"). Both sides render as lower-case hex.
func runEncodeSeqCase(tc seqVector) (got, want string, err error) {
	var units []string
	var outputStr string
	if unmarshalInputs(tc.Inputs, &units) != nil || json.Unmarshal(tc.Outputs, &outputStr) != nil {
		return "", "", errUnparsable
	}
	expected, err := decodeBytesValue(outputStr)
	if err != nil {
		return "", "", errUnparsable
	}

	seq, err := EncodeSeq(units)
	if err != nil {
		return "", "", err
	}
	return hex.EncodeToString(seq), hex.EncodeToString(expected), nil
}

// runDecodeSeqCase runs a single decode_seq test case
// (inputs ["bytes:<hex>"], output [str…]). Both unit lists render with %q, so
// the comparison is exact and a nil list equals an empty one.
func runDecodeSeqCase(tc seqVector) (got, want string, err error) {
	var inputStr string
	var expected []string
	if unmarshalInputs(tc.Inputs, &inputStr) != nil || json.Unmarshal(tc.Outputs, &expected) != nil {
		return "", "", errUnparsable
	}
	seq, err := decodeBytesValue(inputStr)
	if err != nil {
		return "", "", errUnparsable
	}

	units, err := DecodeSeq(seq)
	if err != nil {
		return "", "", err
	}
	return fmt.Sprintf("%q", units), fmt.Sprintf("%q", expected), nil
}
