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
	"errors"
	"fmt"
	"io"
	"os"
	"strings"
)

//go:embed testdata/data.json
var conformanceData string

// vectorEntry holds a single conformance test case from data.json. The
// outputs stay raw until a runner parses them: they are an object (gen_*), a
// "bytes:<hex>" string (encode_seq) or a list of unit strings (decode_seq).
type vectorEntry struct {
	Inputs  []json.RawMessage `json:"inputs"`
	Outputs json.RawMessage   `json:"outputs"`
}

// parseConformanceData unmarshals every test vector section of data.json.
// "_metadata" holds no test vectors and is skipped; every other key must map
// to an object of case objects. A missing section is left out, so its runner
// reports it as missing.
func parseConformanceData(raw []byte) (map[string]map[string]vectorEntry, error) {
	var sections map[string]json.RawMessage
	if err := json.Unmarshal(raw, &sections); err != nil {
		return nil, fmt.Errorf("could not parse conformance data: %w", err)
	}

	data := make(map[string]map[string]vectorEntry, len(sections))
	for key, rawSection := range sections {
		if key == "_metadata" {
			continue
		}
		cases, err := parseSection(key, rawSection)
		if err != nil {
			return nil, err
		}
		data[key] = cases
	}
	return data, nil
}

// isJSONObject reports whether a raw JSON value is an object.
func isJSONObject(raw json.RawMessage) bool {
	trimmed := bytes.TrimSpace(raw)
	return len(trimmed) > 0 && trimmed[0] == '{'
}

// parseSection unmarshals one data.json section, which must be an object of
// case objects (a null section or case is rejected, not read as empty).
func parseSection(key string, raw json.RawMessage) (map[string]vectorEntry, error) {
	var rawCases map[string]json.RawMessage
	if !isJSONObject(raw) || json.Unmarshal(raw, &rawCases) != nil {
		return nil, fmt.Errorf("could not parse section %s: not an object of cases", key)
	}
	cases := make(map[string]vectorEntry, len(rawCases))
	for name, rawCase := range rawCases {
		var tc vectorEntry
		if !isJSONObject(rawCase) || json.Unmarshal(rawCase, &tc) != nil {
			return nil, fmt.Errorf("could not parse case %s.%s: not an object", key, name)
		}
		cases[name] = tc
	}
	return cases, nil
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
	passed = runSection(data, "encode_seq", runEncodeSeqCase) && passed
	passed = runSection(data, "decode_seq", runDecodeSeqCase) && passed

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

// containsNull reports whether a raw JSON value holds a null at any depth.
// encoding/json leaves a string, number or slice element untouched on null,
// so a strict reader rejects it first. Malformed JSON counts as containing one.
func containsNull(raw json.RawMessage) bool {
	dec := json.NewDecoder(bytes.NewReader(raw))
	for {
		tok, err := dec.Token()
		if err != nil {
			return !errors.Is(err, io.EOF)
		}
		if tok == nil {
			return true
		}
	}
}

// unmarshalInputs decodes the leading test inputs into targets, in order.
// A *json.RawMessage target receives the input as is, for its caller to
// check. Returns errUnparsable if an input is missing, holds a null, or does
// not decode into its target (a wrong JSON type, a fraction for an integer
// target, or a number outside the target's range).
func unmarshalInputs(inputs []json.RawMessage, targets ...any) error {
	if len(inputs) < len(targets) {
		return errUnparsable
	}
	for i, target := range targets {
		if raw, ok := target.(*json.RawMessage); ok {
			*raw = inputs[i]
			continue
		}
		if containsNull(inputs[i]) || json.Unmarshal(inputs[i], target) != nil {
			return errUnparsable
		}
	}
	return nil
}

// expectedISCC returns the expected "iscc" output of a gen_* case.
// Returns errUnparsable if the outputs are not an object with a string "iscc".
func expectedISCC(tc vectorEntry) (string, error) {
	var outputs map[string]json.RawMessage
	var iscc *string
	if json.Unmarshal(tc.Outputs, &outputs) != nil || json.Unmarshal(outputs["iscc"], &iscc) != nil || iscc == nil {
		return "", errUnparsable
	}
	return *iscc, nil
}

// metaArg converts a raw gen_meta_code_v0 meta input (null, string, or JSON
// object) into the meta argument of GenMetaCodeV0. Any other JSON type is
// errUnparsable.
func metaArg(raw json.RawMessage) (*string, error) {
	text := string(raw)
	if text == "null" {
		return nil, nil
	}
	if isJSONObject(raw) {
		return &text, nil
	}
	if len(raw) == 0 || raw[0] != '"' {
		return nil, errUnparsable
	}
	var meta string
	if json.Unmarshal(raw, &meta) != nil {
		return nil, errUnparsable
	}
	return &meta, nil
}

// metaInput holds the parsed arguments of a gen_meta_code_v0 case.
type metaInput struct {
	name string
	desc *string
	meta *string
	bits uint32
}

// metaInputs parses the [name, description, meta, bits] inputs of a
// gen_meta_code_v0 case. An empty description means no description; a null
// one is unparsable.
func metaInputs(tc vectorEntry) (metaInput, error) {
	var in metaInput
	var descStr string
	var rawMeta json.RawMessage
	if err := unmarshalInputs(tc.Inputs, &in.name, &descStr, &rawMeta, &in.bits); err != nil {
		return metaInput{}, err
	}
	meta, err := metaArg(rawMeta)
	if err != nil {
		return metaInput{}, err
	}
	if descStr != "" {
		in.desc = &descStr
	}
	in.meta = meta
	return in, nil
}

// textInputs parses the [text, bits] inputs of a gen_text_code_v0 case.
func textInputs(tc vectorEntry) (string, uint32, error) {
	var text string
	var bits uint32
	if err := unmarshalInputs(tc.Inputs, &text, &bits); err != nil {
		return "", 0, err
	}
	return text, bits, nil
}

// imageInputs parses the [pixels, bits] inputs of a gen_image_code_v0 case.
// Pixels decode into []int first: encoding/json reads a []byte target from a
// base64 string, not from a list of numbers. Each pixel must be 0..=255.
func imageInputs(tc vectorEntry) ([]byte, uint32, error) {
	var pixelsJSON []int
	var bits uint32
	if err := unmarshalInputs(tc.Inputs, &pixelsJSON, &bits); err != nil {
		return nil, 0, err
	}
	pixels := make([]byte, len(pixelsJSON))
	for i, v := range pixelsJSON {
		if v < 0 || v > 255 {
			return nil, 0, errUnparsable
		}
		pixels[i] = byte(v)
	}
	return pixels, bits, nil
}

// audioInputs parses the [cv, bits] inputs of a gen_audio_code_v0 case.
func audioInputs(tc vectorEntry) ([]int32, uint32, error) {
	var cv []int32
	var bits uint32
	if err := unmarshalInputs(tc.Inputs, &cv, &bits); err != nil {
		return nil, 0, err
	}
	return cv, bits, nil
}

// videoInputs parses the [frame_sigs, bits] inputs of a gen_video_code_v0 case.
func videoInputs(tc vectorEntry) ([][]int32, uint32, error) {
	var frameSigs [][]int32
	var bits uint32
	if err := unmarshalInputs(tc.Inputs, &frameSigs, &bits); err != nil {
		return nil, 0, err
	}
	return frameSigs, bits, nil
}

// mixedInputs parses the [codes, bits] inputs of a gen_mixed_code_v0 case.
func mixedInputs(tc vectorEntry) ([]string, uint32, error) {
	var codes []string
	var bits uint32
	if err := unmarshalInputs(tc.Inputs, &codes, &bits); err != nil {
		return nil, 0, err
	}
	return codes, bits, nil
}

// decodeStream decodes a "stream:<hex>" value into bytes. A value without the
// "stream:" prefix is an error; "stream:" alone is the empty stream.
func decodeStream(s string) ([]byte, error) {
	hexData, ok := strings.CutPrefix(s, "stream:")
	if !ok {
		return nil, fmt.Errorf("missing stream: prefix in %q", s)
	}
	if hexData == "" {
		return []byte{}, nil
	}
	return hex.DecodeString(hexData)
}

// streamInputs decodes the "stream:<hex>" and bits inputs of a gen_data_code_v0
// or gen_instance_code_v0 case.
func streamInputs(tc vectorEntry) ([]byte, uint32, error) {
	var streamStr string
	var bits uint32
	if err := unmarshalInputs(tc.Inputs, &streamStr, &bits); err != nil {
		return nil, 0, err
	}
	inputBytes, err := decodeStream(streamStr)
	if err != nil {
		return nil, 0, errUnparsable
	}
	return inputBytes, bits, nil
}

// isccInputs parses the [codes] input of a gen_iscc_code_v0 case.
func isccInputs(tc vectorEntry) ([]string, error) {
	var codes []string
	if err := unmarshalInputs(tc.Inputs, &codes); err != nil {
		return nil, err
	}
	return codes, nil
}

// runMetaCase runs a single gen_meta_code_v0 test case.
func runMetaCase(tc vectorEntry) (got, want string, err error) {
	in, err := metaInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenMetaCodeV0(in.name, in.desc, in.meta, in.bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runTextCase runs a single gen_text_code_v0 test case.
func runTextCase(tc vectorEntry) (got, want string, err error) {
	text, bits, err := textInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenTextCodeV0(text, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runImageCase runs a single gen_image_code_v0 test case.
func runImageCase(tc vectorEntry) (got, want string, err error) {
	pixels, bits, err := imageInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenImageCodeV0(pixels, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runAudioCase runs a single gen_audio_code_v0 test case.
func runAudioCase(tc vectorEntry) (got, want string, err error) {
	cv, bits, err := audioInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenAudioCodeV0(cv, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runVideoCase runs a single gen_video_code_v0 test case.
func runVideoCase(tc vectorEntry) (got, want string, err error) {
	frameSigs, bits, err := videoInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenVideoCodeV0(frameSigs, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runMixedCase runs a single gen_mixed_code_v0 test case.
func runMixedCase(tc vectorEntry) (got, want string, err error) {
	codes, bits, err := mixedInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenMixedCodeV0(codes, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runDataCase runs a single gen_data_code_v0 test case.
func runDataCase(tc vectorEntry) (got, want string, err error) {
	inputBytes, bits, err := streamInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenDataCodeV0(inputBytes, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runInstanceCase runs a single gen_instance_code_v0 test case.
func runInstanceCase(tc vectorEntry) (got, want string, err error) {
	inputBytes, bits, err := streamInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	result, err := GenInstanceCodeV0(inputBytes, bits)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
}

// runIsccCase runs a single gen_iscc_code_v0 test case.
func runIsccCase(tc vectorEntry) (got, want string, err error) {
	codes, err := isccInputs(tc)
	if err != nil {
		return "", "", err
	}
	if want, err = expectedISCC(tc); err != nil {
		return "", "", err
	}
	// Conformance vectors use default (non-wide) mode
	result, err := GenIsccCodeV0(codes, false)
	if err != nil {
		return "", "", err
	}
	return result.Iscc, want, nil
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
func runEncodeSeqCase(tc vectorEntry) (got, want string, err error) {
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

// decodeSeqInputs parses the ["bytes:<hex>"] input of a decode_seq case.
func decodeSeqInputs(tc vectorEntry) ([]byte, error) {
	var inputStr string
	if err := unmarshalInputs(tc.Inputs, &inputStr); err != nil {
		return nil, err
	}
	seq, err := decodeBytesValue(inputStr)
	if err != nil {
		return nil, errUnparsable
	}
	return seq, nil
}

// runDecodeSeqCase runs a single decode_seq test case
// (inputs ["bytes:<hex>"], output [str…]). Both unit lists render with %q, so
// the comparison is exact and a nil list equals an empty one.
func runDecodeSeqCase(tc vectorEntry) (got, want string, err error) {
	seq, err := decodeSeqInputs(tc)
	if err != nil {
		return "", "", err
	}
	var expected []string
	if containsNull(tc.Outputs) || json.Unmarshal(tc.Outputs, &expected) != nil {
		return "", "", errUnparsable
	}

	units, err := DecodeSeq(seq)
	if err != nil {
		return "", "", err
	}
	return fmt.Sprintf("%q", units), fmt.Sprintf("%q", expected), nil
}
