// Shared edge-case vectors (testdata/data-edge.json) run against the pure-Go port.
//
// The file holds JSON-expressible edge-case inputs with their expected outcome
// (`outputs` or `error`). This harness runs every vector, honours `except.go`,
// and fails on any function section it does not handle. The vendored copy is
// read only from this test file, never embedded in the package.
package iscc

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"reflect"
	"sort"
	"strings"
	"testing"
)

// edgeSurface is this harness's surface id in the `except` map.
const edgeSurface = "go"

// edgeVector is one vector of data-edge.json.
type edgeVector struct {
	Inputs  []json.RawMessage `json:"inputs"`
	Outputs json.RawMessage   `json:"outputs"`
	Error   *string           `json:"error"`
	Except  map[string]string `json:"except"`
}

// edgeCall is a parsed vector ready to run: call is the raw library call, and
// convert turns its successful result into the fixture form.
type edgeCall struct {
	call    func() (any, error)
	convert func(any) any
}

// edgeCaller parses a vector's inputs into an edgeCall (error = harness failure).
type edgeCaller func(inputs []json.RawMessage) (edgeCall, error)

// edgeSections maps each handled function section to its caller.
var edgeSections = map[string]edgeCaller{
	"iscc_decode":       edgeCallDecode,
	"encode_seq":        edgeCallSeq,
	"gen_text_code_v0":  edgeCallText,
	"gen_mixed_code_v0": edgeCallMixed,
	"gen_iscc_code_v0":  edgeCallIsccCode,
	"alg_simhash":       edgeCallSimhash,
}

// loadEdgeDoc reads and parses the vendored data-edge.json.
func loadEdgeDoc(t *testing.T) map[string]map[string]edgeVector {
	t.Helper()
	raw, err := os.ReadFile("testdata/data-edge.json")
	if err != nil {
		t.Fatalf("read data-edge.json: %v", err)
	}
	return parseEdgeDoc(t, raw)
}

// parseEdgeDoc unmarshals a data-edge.json document.
func parseEdgeDoc(t *testing.T, raw []byte) map[string]map[string]edgeVector {
	t.Helper()
	var doc map[string]map[string]edgeVector
	if err := json.Unmarshal(raw, &doc); err != nil {
		t.Fatalf("parse data-edge.json: %v", err)
	}
	return doc
}

// edgeArg unmarshals the input at index into dst.
func edgeArg(inputs []json.RawMessage, index int, dst any) error {
	if index >= len(inputs) {
		return fmt.Errorf("input %d is missing", index)
	}
	if err := json.Unmarshal(inputs[index], dst); err != nil {
		return fmt.Errorf("input %d: %v", index, err)
	}
	return nil
}

// edgeBytes encodes bytes as a `bytes:<hex>` fixture value.
func edgeBytes(data []byte) string {
	return "bytes:" + hex.EncodeToString(data)
}

// edgeBytesArg reads the input at index as a list of `bytes:<hex>` values.
func edgeBytesArg(inputs []json.RawMessage, index int) ([][]byte, error) {
	var items []string
	if err := edgeArg(inputs, index, &items); err != nil {
		return nil, err
	}
	digests := make([][]byte, 0, len(items))
	for _, item := range items {
		digits, ok := strings.CutPrefix(item, "bytes:")
		data, err := hex.DecodeString(digits)
		if !ok || err != nil {
			return nil, fmt.Errorf("input %d holds a non-bytes:<hex> item %q", index, item)
		}
		digests = append(digests, data)
	}
	return digests, nil
}

// edgeCallDecode prepares IsccDecode on the vector's string input.
func edgeCallDecode(inputs []json.RawMessage) (edgeCall, error) {
	var iscc string
	if err := edgeArg(inputs, 0, &iscc); err != nil {
		return edgeCall{}, err
	}
	call := func() (any, error) { return IsccDecode(iscc) }
	convert := func(r any) any {
		d := r.(*DecodeResult)
		return []any{d.Maintype, d.Subtype, d.Version, d.Length, edgeBytes(d.Digest)}
	}
	return edgeCall{call, convert}, nil
}

// edgeCallSeq prepares EncodeSeq on the vector's unit list.
func edgeCallSeq(inputs []json.RawMessage) (edgeCall, error) {
	var units []string
	if err := edgeArg(inputs, 0, &units); err != nil {
		return edgeCall{}, err
	}
	call := func() (any, error) { return EncodeSeq(units) }
	convert := func(r any) any { return edgeBytes(r.([]byte)) }
	return edgeCall{call, convert}, nil
}

// edgeCallText prepares GenTextCodeV0 on the vector's text and bits.
func edgeCallText(inputs []json.RawMessage) (edgeCall, error) {
	var text string
	var bits uint32
	if err := edgeArg(inputs, 0, &text); err != nil {
		return edgeCall{}, err
	}
	if err := edgeArg(inputs, 1, &bits); err != nil {
		return edgeCall{}, err
	}
	call := func() (any, error) { return GenTextCodeV0(text, bits) }
	convert := func(r any) any {
		res := r.(*TextCodeResult)
		return map[string]any{"iscc": res.Iscc, "characters": res.Characters}
	}
	return edgeCall{call, convert}, nil
}

// edgeCallMixed prepares GenMixedCodeV0 on the vector's codes and bits.
func edgeCallMixed(inputs []json.RawMessage) (edgeCall, error) {
	var codes []string
	var bits uint32
	if err := edgeArg(inputs, 0, &codes); err != nil {
		return edgeCall{}, err
	}
	if err := edgeArg(inputs, 1, &bits); err != nil {
		return edgeCall{}, err
	}
	call := func() (any, error) { return GenMixedCodeV0(codes, bits) }
	convert := func(r any) any {
		res := r.(*MixedCodeResult)
		return map[string]any{"iscc": res.Iscc, "parts": res.Parts}
	}
	return edgeCall{call, convert}, nil
}

// edgeCallIsccCode prepares GenIsccCodeV0 (the optional second input is `wide`).
func edgeCallIsccCode(inputs []json.RawMessage) (edgeCall, error) {
	var codes []string
	wide := false
	if err := edgeArg(inputs, 0, &codes); err != nil {
		return edgeCall{}, err
	}
	if len(inputs) > 1 {
		if err := edgeArg(inputs, 1, &wide); err != nil {
			return edgeCall{}, err
		}
	}
	call := func() (any, error) { return GenIsccCodeV0(codes, wide) }
	convert := func(r any) any { return map[string]any{"iscc": r.(*IsccCodeResult).Iscc} }
	return edgeCall{call, convert}, nil
}

// edgeCallSimhash prepares AlgSimhash on the vector's digest list.
func edgeCallSimhash(inputs []json.RawMessage) (edgeCall, error) {
	digests, err := edgeBytesArg(inputs, 0)
	if err != nil {
		return edgeCall{}, err
	}
	call := func() (any, error) { return AlgSimhash(digests) }
	convert := func(r any) any { return edgeBytes(r.([]byte)) }
	return edgeCall{call, convert}, nil
}

// edgeNormalize round-trips a value through JSON so numbers compare as float64.
func edgeNormalize(value any) (any, error) {
	raw, err := json.Marshal(value)
	if err != nil {
		return nil, err
	}
	var out any
	err = json.Unmarshal(raw, &out)
	return out, err
}

// edgeCompare compares an actual output with the expected one and returns a
// failure description. For an object only the expected keys are compared.
func edgeCompare(expected json.RawMessage, actual any) string {
	var want any
	if err := json.Unmarshal(expected, &want); err != nil {
		return fmt.Sprintf("outputs: %v", err)
	}
	got, err := edgeNormalize(actual)
	if err != nil {
		return fmt.Sprintf("normalize output: %v", err)
	}
	fields, isObject := want.(map[string]any)
	if !isObject {
		if !reflect.DeepEqual(want, got) {
			return fmt.Sprintf("expected %v, got %v", want, got)
		}
		return ""
	}
	gotFields, _ := got.(map[string]any)
	for key, value := range fields {
		if !reflect.DeepEqual(value, gotFields[key]) {
			return fmt.Sprintf("field %s: expected %v, got %v", key, value, gotFields[key])
		}
	}
	return ""
}

// edgeJudge judges a raw call's outcome against the vector's expectation.
func edgeJudge(expectError bool, prepared edgeCall, vector edgeVector) string {
	result, err := prepared.call()
	if expectError {
		if err == nil {
			return fmt.Sprintf("expected an error, got %v", prepared.convert(result))
		}
		return ""
	}
	if err != nil {
		return fmt.Sprintf("expected an output, got error: %v", err)
	}
	if vector.Outputs == nil {
		return "vector has neither outputs nor error"
	}
	return edgeCompare(vector.Outputs, prepared.convert(result))
}

// edgeCheckVector checks one vector and returns a failure description, if any.
func edgeCheckVector(caller edgeCaller, vector edgeVector) string {
	exception := vector.Except[edgeSurface]
	if exception == "skip" {
		return ""
	}
	prepared, err := caller(vector.Inputs)
	if err != nil {
		return err.Error()
	}
	return edgeJudge(exception == "error" || vector.Error != nil, prepared, vector)
}

// edgeCheckDoc runs every vector of a parsed document and returns one
// `section/vector: reason` line per failure. An unknown section fails as a whole.
func edgeCheckDoc(doc map[string]map[string]edgeVector) []string {
	var failures []string
	for section, vectors := range doc {
		caller, known := edgeSections[section]
		if !known {
			failures = append(failures, section+": unknown section")
			continue
		}
		for name, vector := range vectors {
			if reason := edgeCheckVector(caller, vector); reason != "" {
				failures = append(failures, fmt.Sprintf("%s/%s: %s", section, name, reason))
			}
		}
	}
	sort.Strings(failures)
	return failures
}

func TestDataEdgeVectorsPass(t *testing.T) {
	doc := loadEdgeDoc(t)
	for section := range edgeSections {
		if len(doc[section]) == 0 {
			t.Errorf("section %s has no vectors", section)
		}
	}
	if failures := edgeCheckDoc(doc); len(failures) > 0 {
		t.Fatalf("data-edge.json failures:\n%s", strings.Join(failures, "\n"))
	}
}

func TestDataEdgeUnknownSectionFails(t *testing.T) {
	doc := parseEdgeDoc(t, []byte(`{"gen_unknown_v0": {"test_0001_x":
		{"inputs": ["x"], "outputs": "x", "note": "n"}}}`))
	failures := edgeCheckDoc(doc)
	if len(failures) != 1 || !strings.Contains(failures[0], "unknown section") {
		t.Fatalf("got %v", failures)
	}
}

func TestDataEdgeExpectedErrorThatSucceedsFails(t *testing.T) {
	// Valid inputs marked `error` prove each rejection-only caller reaches the library.
	doc := parseEdgeDoc(t, []byte(`{
		"gen_mixed_code_v0": {"test_0001_x": {"inputs": [["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"], 64],
			"error": "e", "note": "n"}},
		"gen_iscc_code_v0": {"test_0001_x": {"inputs": [["GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
			"IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]], "error": "e", "note": "n"}}}`))
	failures := edgeCheckDoc(doc)
	if len(failures) != 2 {
		t.Fatalf("got %v", failures)
	}
	for _, failure := range failures {
		if !strings.Contains(failure, "expected an error") {
			t.Fatalf("got %v", failures)
		}
	}
}

func TestDataEdgeOutputMismatchFails(t *testing.T) {
	doc := parseEdgeDoc(t, []byte(`{"encode_seq": {"test_0001_x":
		{"inputs": [["ISCC:GAAAAAAAAA"]], "outputs": "bytes:300000000001", "note": "n"}}}`))
	failures := edgeCheckDoc(doc)
	if len(failures) != 1 || !strings.Contains(failures[0], "expected bytes:300000000001") {
		t.Fatalf("got %v", failures)
	}
}

func TestDataEdgeHonoursExcept(t *testing.T) {
	doc := parseEdgeDoc(t, []byte(`{"iscc_decode": {
		"test_0001_error": {"inputs": ["ISCC:GAAAAAAAAA"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
			"note": "n", "except": {"go": "error"}},
		"test_0002_skip": {"inputs": ["ISCC:GAAAAAAAAB"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
			"note": "n", "except": {"go": "skip"}}}}`))
	failures := edgeCheckDoc(doc)
	if len(failures) != 1 || !strings.HasPrefix(failures[0], "iscc_decode/test_0001_error: expected an error") {
		t.Fatalf("got %v", failures)
	}
}
