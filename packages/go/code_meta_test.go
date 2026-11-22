// Conformance tests for GenMetaCodeV0 against data.json vectors.
package iscc

import (
	"encoding/hex"
	"encoding/json"
	"strings"
	"testing"
)

func TestPureGoGenMetaCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_meta_code_v0") {
		t.Run(name, func(t *testing.T) {
			in, err := metaInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}
			var want struct {
				Name        *string `json:"name"`
				Description *string `json:"description"`
				Meta        *string `json:"meta"`
				Metahash    *string `json:"metahash"`
			}
			if err := json.Unmarshal(vec.Outputs, &want); err != nil {
				t.Fatalf("parse outputs: %v", err)
			}

			result, err := GenMetaCodeV0(in.name, in.desc, in.meta, in.bits)
			if err != nil {
				t.Fatalf("GenMetaCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
			// name, description, meta and metahash are checked when present
			if want.Name != nil && result.Name != *want.Name {
				t.Errorf("name: got %q, want %q", result.Name, *want.Name)
			}
			if want.Description != nil && result.Description != *want.Description {
				t.Errorf("description: got %q, want %q", result.Description, *want.Description)
			}
			if want.Meta != nil && result.Meta != *want.Meta {
				t.Errorf("meta: got %q, want %q", result.Meta, *want.Meta)
			}
			if want.Metahash != nil && result.Metahash != *want.Metahash {
				t.Errorf("metahash: got %q, want %q", result.Metahash, *want.Metahash)
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

// jcsStructureCases covers RFC 8785 key order by UTF-16 code units and negative
// zero, with the canonical payload computed by the Rust core (json_to_data_url).
var jcsStructureCases = []struct {
	name       string
	meta       string
	payloadHex string
}{
	{"utf16_key_order", `{"\ue000":1,"\ud83d\ude00":2}`, "7b22f09f9880223a322c22ee8080223a317d"},
	{"negative_zero_float", `{"a":-0.0}`, "7b2261223a307d"},
	{"negative_zero_int", `{"a":-0}`, "7b2261223a307d"},
	{"nested_in_array", `{"a":[{"\ue000":1,"\ud83d\ude00":-0.0}]}`,
		"7b2261223a5b7b22f09f9880223a302c22ee8080223a317d5d7d"},
	{"nested_objects", `{"b":{"\ue000":[-0,{"\ud83d\ude00":0}],"a":null}}`,
		"7b2262223a7b2261223a6e756c6c2c22ee8080223a5b302c7b22f09f9880223a307d5d7d7d"},
	{"surrogate_pair_value", `{"a":"\ud83d\ude00"}`, "7b2261223a22f09f9880227d"},
	{"escaped_backslash_before_u", `{"a":"\\ud800"}`, "7b2261223a225c5c7564383030227d"},
	{"numbers", `{"a":[1.0,1e21,1e-7,9007199254740993, 0.1, -1.5e300]}`,
		"7b2261223a5b312c31652b32312c31652d372c393030373139393235343734303939322c302e312c2d312e35652b3330305d7d"},
}

// TestParseMetaJSONStructure asserts key order and number output match the Rust core.
func TestParseMetaJSONStructure(t *testing.T) {
	for _, tc := range jcsStructureCases {
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

// TestParseMetaJSONLoneSurrogate asserts every lone surrogate escape is rejected,
// as the Rust core's serde_json parser does.
func TestParseMetaJSONLoneSurrogate(t *testing.T) {
	cases := map[string]string{
		"high_alone":     `{"a":"\ud800"}`,
		"low_alone":      `{"a":"\udc00"}`,
		"high_then_text": `{"a":"\ud800A"}`,
		"high_then_high": `{"\ud800\ud800":1}`,
		"high_in_key":    `{"\ud800":1}`,
		"high_at_end":    `{"a":"x\uDBFF"}`,
		"low_after_pair": `{"a":"\ud83d\ude00\ude00"}`,
	}
	for name, meta := range cases {
		t.Run(name, func(t *testing.T) {
			_, err := parseMetaJSON(meta)
			if err == nil || !strings.HasPrefix(err.Error(), "iscc: invalid JSON in meta:") {
				t.Errorf("parseMetaJSON(%s) error = %v, want invalid JSON in meta", meta, err)
			}
		})
	}
}

// TestGenMetaCodeV0LoneSurrogate asserts both public entry points reject a lone surrogate.
func TestGenMetaCodeV0LoneSurrogate(t *testing.T) {
	meta := `{"a":"\ud800"}`
	if _, err := GenMetaCodeV0("Hello", nil, &meta, 64); err == nil {
		t.Error("GenMetaCodeV0 accepted a lone surrogate")
	}
	if _, err := JsonToDataUrl(meta); err == nil {
		t.Error("JsonToDataUrl accepted a lone surrogate")
	}
}

// TestGenMetaCodeV0Structure asserts iscc, meta and metahash match the Rust core
// for the UTF-16 key order and negative zero inputs.
func TestGenMetaCodeV0Structure(t *testing.T) {
	cases := []struct{ meta, iscc, dataURL, metahash string }{
		{`{"\ue000":1,"\ud83d\ude00":2}`, "ISCC:AAAWKLHFXP2I3X3H",
			"data:application/json;base64,eyLwn5iAIjoyLCLugIAiOjF9",
			"1e200776750a1835f47c953eac436a0991c2bced534a742b6e5b8893205a1469f3b9"},
		{`{"a":-0.0}`, "ISCC:AAAWKLHFXMHM7WHO", "data:application/json;base64,eyJhIjowfQ==",
			"1e20393859791157de56f69287760924ba3a7045e3d7c1b1674e3f5c213ee48a5196"},
	}
	for _, tc := range cases {
		meta := tc.meta
		r, err := GenMetaCodeV0("Hello", nil, &meta, 64)
		if err != nil {
			t.Fatalf("GenMetaCodeV0(%s): %v", tc.meta, err)
		}
		if r.Iscc != tc.iscc || r.Meta != tc.dataURL || r.Metahash != tc.metahash {
			t.Errorf("GenMetaCodeV0(%s) = %+v, want %s %s %s", tc.meta, r, tc.iscc, tc.dataURL, tc.metahash)
		}
	}
}

// metaJSONOfSize returns the JSON meta {"a":"xxx…"} whose canonical form is size bytes.
func metaJSONOfSize(size int) string {
	return `{"a":"` + strings.Repeat("x", size-8) + `"}`
}

// TestGenMetaCodeV0MetaSizeLimit pins the MetaTrimMeta boundary for both meta forms:
// a payload of exactly MetaTrimMeta bytes succeeds, one byte more is rejected.
func TestGenMetaCodeV0MetaSizeLimit(t *testing.T) {
	cases := []struct {
		name    string
		size    int
		dataURL bool
		wantErr bool
	}{
		{"json at limit", MetaTrimMeta, false, false},
		{"json over limit", MetaTrimMeta + 1, false, true},
		{"data-url at limit", MetaTrimMeta, true, false},
		{"data-url over limit", MetaTrimMeta + 1, true, true},
	}
	for _, tc := range cases {
		meta := metaJSONOfSize(tc.size)
		if tc.dataURL {
			meta = buildMetaDataURL([]byte(meta), false)
		}
		_, err := GenMetaCodeV0("Hello", nil, &meta, 64)
		if (err != nil) != tc.wantErr {
			t.Errorf("%s: err = %v, wantErr %v", tc.name, err, tc.wantErr)
		}
		if err != nil && !strings.Contains(err.Error(), "decoded meta payload exceeds size limit") {
			t.Errorf("%s: err = %v, want post-decode rejection", tc.name, err)
		}
	}
}

// TestGenMetaCodeV0MetaPreDecodeLimit asserts a raw meta string longer than the
// pre-decode bound is rejected even when its canonical form is tiny.
func TestGenMetaCodeV0MetaPreDecodeLimit(t *testing.T) {
	small := `{"a":1}`
	atBound := small + strings.Repeat(" ", metaPreDecodeLimit-len(small))
	if _, err := GenMetaCodeV0("Hello", nil, &atBound, 64); err != nil {
		t.Errorf("meta string of %d bytes rejected: %v", len(atBound), err)
	}
	over := atBound + " "
	_, err := GenMetaCodeV0("Hello", nil, &over, 64)
	if err == nil || !strings.Contains(err.Error(), "meta string exceeds size limit") {
		t.Errorf("meta string of %d bytes: err = %v, want pre-decode rejection", len(over), err)
	}
	if metaPreDecodeLimit != 170_922 {
		t.Errorf("metaPreDecodeLimit = %d, want 170922", metaPreDecodeLimit)
	}
}
