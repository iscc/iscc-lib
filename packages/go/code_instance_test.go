// Conformance tests for GenInstanceCodeV0 against data.json vectors.
package iscc

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"strings"
	"testing"
)

func TestPureGoGenInstanceCodeV0(t *testing.T) {
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

	vectors, ok := allVectors["gen_instance_code_v0"]
	if !ok {
		t.Fatal("gen_instance_code_v0 section not found in data.json")
	}

	for name, vec := range vectors {
		t.Run(name, func(t *testing.T) {
			// Parse inputs: [stream_hex, bits]
			var streamStr string
			if err := json.Unmarshal(vec.Inputs[0], &streamStr); err != nil {
				t.Fatalf("parse stream input: %v", err)
			}

			// Decode "stream:<hex>" format
			hexData := strings.TrimPrefix(streamStr, "stream:")
			var inputBytes []byte
			if hexData != "" {
				inputBytes, err = hex.DecodeString(hexData)
				if err != nil {
					t.Fatalf("decode hex: %v", err)
				}
			} else {
				inputBytes = []byte{}
			}

			var bits float64
			if err := json.Unmarshal(vec.Inputs[1], &bits); err != nil {
				t.Fatalf("parse bits input: %v", err)
			}

			result, err := GenInstanceCodeV0(inputBytes, uint32(bits))
			if err != nil {
				t.Fatalf("GenInstanceCodeV0: %v", err)
			}

			// Verify ISCC code
			wantISCC, _ := vec.Outputs["iscc"].(string)
			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}

			// Verify datahash
			wantDatahash, _ := vec.Outputs["datahash"].(string)
			if result.Datahash != wantDatahash {
				t.Errorf("datahash: got %q, want %q", result.Datahash, wantDatahash)
			}

			// Verify filesize
			wantFilesize, _ := vec.Outputs["filesize"].(float64)
			if result.Filesize != uint64(wantFilesize) {
				t.Errorf("filesize: got %d, want %d", result.Filesize, uint64(wantFilesize))
			}
		})
	}
}
