// Conformance tests for GenMetaCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenMetaCodeV0(t *testing.T) {
	data, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}

	var allVectors map[string]map[string]struct {
		Inputs  []json.RawMessage      `json:"inputs"`
		Outputs map[string]interface{} `json:"outputs"`
	}
	if err := json.Unmarshal(data, &allVectors); err != nil {
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
