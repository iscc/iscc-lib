// Conformance tests for GenMetaCodeV0 against data.json vectors.
package iscc

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenMetaCodeV0(t *testing.T) {
	data, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}

	allVectors, err := parseConformanceData(data)
	if err != nil {
		t.Fatalf("parse data.json: %v", err)
	}

	vectors, ok := allVectors["gen_meta_code_v0"]
	if !ok {
		t.Fatal("gen_meta_code_v0 section not found in data.json")
	}

	for name, vec := range vectors {
		t.Run(name, func(t *testing.T) {
			// Parse inputs: [name, description, meta, bits]
			var inputName string
			if err := json.Unmarshal(vec.Inputs[0], &inputName); err != nil {
				t.Fatalf("parse name: %v", err)
			}

			// Description can be string or null
			var desc *string
			var descStr string
			if string(vec.Inputs[1]) != "null" {
				if err := json.Unmarshal(vec.Inputs[1], &descStr); err != nil {
					t.Fatalf("parse description: %v", err)
				}
				if descStr != "" {
					desc = &descStr
				}
			}

			// Meta can be null, string (data-URL), or JSON object
			var meta *string
			rawMeta := string(vec.Inputs[2])
			if rawMeta != "null" {
				// Check if it's a string (starts with quote) or a JSON object
				if rawMeta[0] == '"' {
					var metaStr string
					if err := json.Unmarshal(vec.Inputs[2], &metaStr); err != nil {
						t.Fatalf("parse meta string: %v", err)
					}
					meta = &metaStr
				} else {
					// JSON object — marshal to string for our API
					metaStr := string(vec.Inputs[2])
					meta = &metaStr
				}
			}

			var bits float64
			if err := json.Unmarshal(vec.Inputs[3], &bits); err != nil {
				t.Fatalf("parse bits: %v", err)
			}

			result, err := GenMetaCodeV0(inputName, desc, meta, uint32(bits))
			if err != nil {
				t.Fatalf("GenMetaCodeV0: %v", err)
			}

			// Verify ISCC code
			if wantISCC, ok := vec.Outputs["iscc"].(string); ok {
				if result.Iscc != wantISCC {
					t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
				}
			}

			// Verify name
			if wantName, ok := vec.Outputs["name"].(string); ok {
				if result.Name != wantName {
					t.Errorf("name: got %q, want %q", result.Name, wantName)
				}
			}

			// Verify description (may be absent in outputs)
			if wantDesc, ok := vec.Outputs["description"].(string); ok {
				if result.Description != wantDesc {
					t.Errorf("description: got %q, want %q", result.Description, wantDesc)
				}
			}

			// Verify meta (may be absent in outputs)
			if wantMeta, ok := vec.Outputs["meta"].(string); ok {
				if result.Meta != wantMeta {
					t.Errorf("meta: got %q, want %q", result.Meta, wantMeta)
				}
			}

			// Verify metahash
			if wantHash, ok := vec.Outputs["metahash"].(string); ok {
				if result.Metahash != wantHash {
					t.Errorf("metahash: got %q, want %q", result.Metahash, wantHash)
				}
			}
		})
	}
}

func TestPureGoGenMetaCodeV0FinalSigma(t *testing.T) {
	// End-to-end Final_Sigma regression: the similarity hash lowercases via
	// TextCollapse, so a word-final capital sigma changes the Meta-Code.
	// Expected value produced by the Rust core, which matches iscc-core.
	result, err := GenMetaCodeV0("ΛΟΓΟΣ", nil, nil, 64)
	if err != nil {
		t.Fatalf("GenMetaCodeV0: %v", err)
	}
	if result.Iscc != "ISCC:AAAXNTFLFFVJ2QUN" {
		t.Errorf("iscc: got %q, want %q", result.Iscc, "ISCC:AAAXNTFLFFVJ2QUN")
	}
}

// jcsStringCase holds a meta JSON input and the canonical outputs computed by the
// Rust core (iscc_lib.json_to_data_url / gen_meta_code_v0("Name", None, meta, 64)).
type jcsStringCase struct {
	name       string
	meta       string
	payloadHex string
	dataURL    string
	iscc       string
	metahash   string
}

// jcsStringCases covers the five characters RFC 8785 emits raw that encoding/json
// escapes, one per case, plus a combined input with the literal-backslash trap.
var jcsStringCases = []jcsStringCase{
	{"amp", `{"t":"a&b"}`, "7b2274223a22612662227d",
		"data:application/json;base64,eyJ0IjoiYSZiIn0=",
		"ISCC:AAAX74W77X2W4P7W", "1e20aa656626ec46ab8baa7c7ac8fb054ca4c29bb4a64957e77ceb0c6fcb6a71ce56"},
	{"lt", `{"t":"a<b"}`, "7b2274223a22613c62227d",
		"data:application/json;base64,eyJ0IjoiYTxiIn0=",
		"ISCC:AAAX74W77W3X5NHF", "1e2089f369255a7c7433a56fce7245abca7ba19426422c2d3f381486a7e350933364"},
	{"gt", `{"t":"a>b"}`, "7b2274223a22613e62227d",
		"data:application/json;base64,eyJ0IjoiYT5iIn0=",
		"ISCC:AAAX74W77WYU7NXW", "1e20c73f3872f9616ba508be08d3b92e781084f72b5b7836cd706f590befc099b1ae"},
	{"line_separator", `{"t":"a\u2028b"}`, "7b2274223a2261e280a862227d",
		"data:application/json;base64,eyJ0IjoiYeKAqGIifQ==",
		"ISCC:AAAX74W77X3G5ZPW", "1e208d8d0821420032c7111326fda815e919e86cde64993b6cb28a90b01230085847"},
	{"paragraph_separator", `{"t":"a\u2029b"}`, "7b2274223a2261e280a962227d",
		"data:application/json;base64,eyJ0IjoiYeKAqWIifQ==",
		"ISCC:AAAX74W77X3C47RX", "1e20f966e2bb66d0a39aa24117263c068a6ac144936b60df47d087b6817470dadee3"},
	{"combined", `{"title":"Tom & Jerry <3 > \u2028 \u2029","note":"a\\u2028b \b\f\u001f"}`,
		"7b226e6f7465223a22615c5c753230323862205c625c665c7530303166222c227469746c65223a22546f6d2026204a65727279203c33203e20e280a820e280a9227d",
		"data:application/json;base64,eyJub3RlIjoiYVxcdTIwMjhiIFxiXGZcdTAwMWYiLCJ0aXRsZSI6IlRvbSAmIEplcnJ5IDwzID4g4oCoIOKAqSJ9",
		"ISCC:AAAX74W77W3IKDKI", "1e207dd6353ce193db311db03f79168a99738635c0c36f66ab187f24e62c831d669c"},
}

// TestParseMetaJSONStringEscaping asserts the canonical bytes match the Rust core.
func TestParseMetaJSONStringEscaping(t *testing.T) {
	for _, tc := range jcsStringCases {
		t.Run(tc.name, func(t *testing.T) {
			got, err := parseMetaJSON(tc.meta)
			if err != nil {
				t.Fatalf("parseMetaJSON: %v", err)
			}
			if hex.EncodeToString(got) != tc.payloadHex {
				t.Errorf("payload = %q (%x), want %s", got, got, tc.payloadHex)
			}
		})
	}
}

// TestJsonToDataUrlStringEscaping asserts the data-URL and its decoded payload
// match the Rust core.
func TestJsonToDataUrlStringEscaping(t *testing.T) {
	for _, tc := range jcsStringCases {
		t.Run(tc.name, func(t *testing.T) {
			url, err := JsonToDataUrl(tc.meta)
			if err != nil {
				t.Fatalf("JsonToDataUrl: %v", err)
			}
			if url != tc.dataURL {
				t.Errorf("data-URL = %q, want %q", url, tc.dataURL)
			}
			payload, err := decodeDataURL(url)
			if err != nil {
				t.Fatalf("decode data-URL: %v", err)
			}
			if hex.EncodeToString(payload) != tc.payloadHex {
				t.Errorf("decoded payload = %x, want %s", payload, tc.payloadHex)
			}
		})
	}
}

// TestGenMetaCodeV0StringEscaping asserts iscc, metahash and meta match the Rust core.
func TestGenMetaCodeV0StringEscaping(t *testing.T) {
	for _, tc := range jcsStringCases {
		t.Run(tc.name, func(t *testing.T) {
			meta := tc.meta
			r, err := GenMetaCodeV0("Name", nil, &meta, 64)
			if err != nil {
				t.Fatalf("GenMetaCodeV0: %v", err)
			}
			if r.Iscc != tc.iscc {
				t.Errorf("iscc = %q, want %q", r.Iscc, tc.iscc)
			}
			if r.Metahash != tc.metahash {
				t.Errorf("metahash = %q, want %q", r.Metahash, tc.metahash)
			}
			if r.Meta != tc.dataURL {
				t.Errorf("meta = %q, want %q", r.Meta, tc.dataURL)
			}
		})
	}
}

// TestJsonToDataUrlTomAndJerry asserts the exact Rust data-URL for an HTML-special title.
func TestJsonToDataUrlTomAndJerry(t *testing.T) {
	const want = "data:application/json;base64,eyJ0aXRsZSI6IlRvbSAmIEplcnJ5IDwzIn0="
	got, err := JsonToDataUrl(`{"title":"Tom & Jerry <3"}`)
	if err != nil {
		t.Fatalf("JsonToDataUrl: %v", err)
	}
	if got != want {
		t.Errorf("JsonToDataUrl = %q, want %q", got, want)
	}
}
