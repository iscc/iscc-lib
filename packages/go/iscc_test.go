// Tests for the iscc Go package WASM bridge.
//
// Requires a prebuilt iscc_ffi.wasm binary in the package directory.
// Build with: cargo build -p iscc-ffi --target wasm32-wasip1
package iscc

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"os"
	"strings"
	"testing"
)

// TestMain checks for the WASM binary before running any tests.
// If missing, all tests are skipped with a clear message.
func TestMain(m *testing.M) {
	if _, err := os.Stat("iscc_ffi.wasm"); os.IsNotExist(err) {
		println("SKIP: iscc_ffi.wasm not found. Build with: cargo build -p iscc-ffi --target wasm32-wasip1")
		println("Then copy: cp target/wasm32-wasip1/debug/iscc_ffi.wasm packages/go/")
		os.Exit(0)
	}
	os.Exit(m.Run())
}

// newTestRuntime creates a Runtime for testing and registers cleanup.
func newTestRuntime(t *testing.T) *Runtime {
	t.Helper()
	ctx := context.Background()
	rt, err := NewRuntime(ctx)
	if err != nil {
		t.Fatalf("NewRuntime: %v", err)
	}
	t.Cleanup(func() {
		if err := rt.Close(ctx); err != nil {
			t.Errorf("Runtime.Close: %v", err)
		}
	})
	return rt
}

// TestRuntimeInit verifies that a Runtime can be created and closed.
func TestRuntimeInit(t *testing.T) {
	ctx := context.Background()
	rt, err := NewRuntime(ctx)
	if err != nil {
		t.Fatalf("NewRuntime failed: %v", err)
	}
	if err := rt.Close(ctx); err != nil {
		t.Fatalf("Close failed: %v", err)
	}
}

// TestConformanceSelftest calls iscc_conformance_selftest() and asserts it returns true.
// This proves the full Rust ISCC core runs correctly inside the WASM module via the Go bridge.
func TestConformanceSelftest(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	passed, err := rt.ConformanceSelftest(ctx)
	if err != nil {
		t.Fatalf("ConformanceSelftest error: %v", err)
	}
	if !passed {
		t.Fatal("ConformanceSelftest returned false — conformance tests failed inside WASM")
	}
}

// TestAllocDealloc allocates memory, writes bytes, reads them back, and deallocates.
func TestAllocDealloc(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	// Allocate 4 bytes.
	ptr, err := rt.alloc(ctx, 4)
	if err != nil {
		t.Fatalf("alloc: %v", err)
	}
	if ptr == 0 {
		t.Fatal("alloc returned null pointer")
	}

	// Write test data.
	data := []byte{0xDE, 0xAD, 0xBE, 0xEF}
	if !rt.mod.Memory().Write(ptr, data) {
		t.Fatal("Memory.Write failed")
	}

	// Read it back.
	got, ok := rt.mod.Memory().Read(ptr, 4)
	if !ok {
		t.Fatal("Memory.Read failed")
	}
	for i, b := range got {
		if b != data[i] {
			t.Fatalf("byte %d: got 0x%02X, want 0x%02X", i, b, data[i])
		}
	}

	// Deallocate.
	if err := rt.dealloc(ctx, ptr, 4); err != nil {
		t.Fatalf("dealloc: %v", err)
	}
}

// TestWriteReadString roundtrips a Unicode string through WASM memory.
func TestWriteReadString(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	// Use a Unicode string with multi-byte characters.
	input := "Hello, 世界! 🌍"

	ptr, size, err := rt.writeString(ctx, input)
	if err != nil {
		t.Fatalf("writeString: %v", err)
	}
	defer func() { _ = rt.dealloc(ctx, ptr, size) }()

	got, err := rt.readString(ctx, ptr)
	if err != nil {
		t.Fatalf("readString: %v", err)
	}
	if got != input {
		t.Fatalf("roundtrip mismatch: got %q, want %q", got, input)
	}
}

// TestTextClean calls iscc_text_clean via the Go bridge and verifies string marshaling.
// Tests NFKC normalization (fi ligature → "fi") and leading/trailing whitespace stripping,
// proving the full writeString → call → readString → freeString cycle works.
func TestTextClean(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	// NFKC normalizes the fi ligature (U+FB01) to "fi", strips leading/trailing whitespace.
	result, err := rt.TextClean(ctx, "  Hel\uFB01 World  ")
	if err != nil {
		t.Fatalf("TextClean error: %v", err)
	}
	expected := "Helfi World"
	if result != expected {
		t.Fatalf("TextClean: got %q, want %q", result, expected)
	}
}

// ── Conformance tests against data.json ─────────────────────────────────────

// testVector represents a single conformance test case from data.json.
type testVector struct {
	Inputs  []json.RawMessage      `json:"inputs"`
	Outputs map[string]interface{} `json:"outputs"`
}

// loadVectors reads and parses data.json from the Rust test directory.
func loadVectors(t *testing.T) map[string]map[string]testVector {
	t.Helper()
	raw, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}
	var data map[string]map[string]testVector
	if err := json.Unmarshal(raw, &data); err != nil {
		t.Fatalf("parse data.json: %v", err)
	}
	return data
}

// expectedISCC extracts the "iscc" string from a test vector's outputs.
func expectedISCC(t *testing.T, name string, v testVector) string {
	t.Helper()
	iscc, ok := v.Outputs["iscc"].(string)
	if !ok {
		t.Fatalf("%s: missing or non-string 'iscc' in outputs", name)
	}
	return iscc
}

// parseBits extracts the bits parameter (last element) from inputs as uint32.
func parseBits(t *testing.T, name string, raw json.RawMessage) uint32 {
	t.Helper()
	var f float64
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatalf("%s: parse bits: %v", name, err)
	}
	return uint32(f)
}

// parseStreamData strips the "stream:" prefix and hex-decodes the remainder.
func parseStreamData(t *testing.T, name string, raw json.RawMessage) []byte {
	t.Helper()
	var s string
	if err := json.Unmarshal(raw, &s); err != nil {
		t.Fatalf("%s: parse stream string: %v", name, err)
	}
	hexStr := strings.TrimPrefix(s, "stream:")
	if hexStr == "" {
		return []byte{}
	}
	data, err := hex.DecodeString(hexStr)
	if err != nil {
		t.Fatalf("%s: hex decode: %v", name, err)
	}
	return data
}

// parseF64Array parses a JSON array of numbers as []float64.
func parseF64Array(t *testing.T, name string, raw json.RawMessage) []float64 {
	t.Helper()
	var arr []float64
	if err := json.Unmarshal(raw, &arr); err != nil {
		t.Fatalf("%s: parse float64 array: %v", name, err)
	}
	return arr
}

// f64ToI32 converts a []float64 to []int32.
func f64ToI32(arr []float64) []int32 {
	result := make([]int32, len(arr))
	for i, v := range arr {
		result[i] = int32(v)
	}
	return result
}

// f64ToByte converts a []float64 to []byte.
func f64ToByte(arr []float64) []byte {
	result := make([]byte, len(arr))
	for i, v := range arr {
		result[i] = byte(v)
	}
	return result
}

// TestGenMetaCodeV0 tests gen_meta_code_v0 against all conformance vectors.
func TestGenMetaCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_meta_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_meta_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [name, description, meta, bits]
			var inputName string
			if err := json.Unmarshal(v.Inputs[0], &inputName); err != nil {
				t.Fatalf("parse name: %v", err)
			}

			// Description: string or null
			var descRaw interface{}
			if err := json.Unmarshal(v.Inputs[1], &descRaw); err != nil {
				t.Fatalf("parse description: %v", err)
			}
			var desc *string
			if s, ok := descRaw.(string); ok {
				desc = &s
			}

			// Meta: null, string, or JSON object
			var metaRaw interface{}
			if err := json.Unmarshal(v.Inputs[2], &metaRaw); err != nil {
				t.Fatalf("parse meta: %v", err)
			}
			var meta *string
			switch mv := metaRaw.(type) {
			case string:
				meta = &mv
			case map[string]interface{}:
				// JSON object → serialize to JSON string
				jsonBytes, err := json.Marshal(mv)
				if err != nil {
					t.Fatalf("marshal meta object: %v", err)
				}
				s := string(jsonBytes)
				meta = &s
			case nil:
				// meta is null → nil
			default:
				t.Fatalf("unexpected meta type: %T", metaRaw)
			}

			bits := parseBits(t, name, v.Inputs[3])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenMetaCodeV0(ctx, inputName, desc, meta, bits)
			if err != nil {
				t.Fatalf("GenMetaCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenTextCodeV0 tests gen_text_code_v0 against all conformance vectors.
func TestGenTextCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_text_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_text_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [text, bits]
			var text string
			if err := json.Unmarshal(v.Inputs[0], &text); err != nil {
				t.Fatalf("parse text: %v", err)
			}
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenTextCodeV0(ctx, text, bits)
			if err != nil {
				t.Fatalf("GenTextCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenImageCodeV0 tests gen_image_code_v0 against all conformance vectors.
func TestGenImageCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_image_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_image_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [pixels_array, bits]
			pixels := f64ToByte(parseF64Array(t, name, v.Inputs[0]))
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenImageCodeV0(ctx, pixels, bits)
			if err != nil {
				t.Fatalf("GenImageCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenAudioCodeV0 tests gen_audio_code_v0 against all conformance vectors.
func TestGenAudioCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_audio_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_audio_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [cv_array, bits]
			cv := f64ToI32(parseF64Array(t, name, v.Inputs[0]))
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenAudioCodeV0(ctx, cv, bits)
			if err != nil {
				t.Fatalf("GenAudioCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenVideoCodeV0 tests gen_video_code_v0 against all conformance vectors.
func TestGenVideoCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_video_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_video_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [frames_array, bits]
			// frames_array is [][]float64 → [][]int32
			var rawFrames [][]float64
			if err := json.Unmarshal(v.Inputs[0], &rawFrames); err != nil {
				t.Fatalf("parse frames: %v", err)
			}
			frames := make([][]int32, len(rawFrames))
			for i, rf := range rawFrames {
				frames[i] = f64ToI32(rf)
			}
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenVideoCodeV0(ctx, frames, bits)
			if err != nil {
				t.Fatalf("GenVideoCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenMixedCodeV0 tests gen_mixed_code_v0 against all conformance vectors.
func TestGenMixedCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_mixed_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_mixed_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [codes_array, bits]
			var rawCodes []interface{}
			if err := json.Unmarshal(v.Inputs[0], &rawCodes); err != nil {
				t.Fatalf("parse codes: %v", err)
			}
			codes := make([]string, len(rawCodes))
			for i, c := range rawCodes {
				codes[i] = c.(string)
			}
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenMixedCodeV0(ctx, codes, bits)
			if err != nil {
				t.Fatalf("GenMixedCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenDataCodeV0 tests gen_data_code_v0 against all conformance vectors.
func TestGenDataCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_data_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_data_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: ["stream:<hex>", bits]
			data := parseStreamData(t, name, v.Inputs[0])
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenDataCodeV0(ctx, data, bits)
			if err != nil {
				t.Fatalf("GenDataCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenInstanceCodeV0 tests gen_instance_code_v0 against all conformance vectors.
func TestGenInstanceCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_instance_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_instance_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: ["stream:<hex>", bits]
			data := parseStreamData(t, name, v.Inputs[0])
			bits := parseBits(t, name, v.Inputs[1])
			expected := expectedISCC(t, name, v)

			got, err := rt.GenInstanceCodeV0(ctx, data, bits)
			if err != nil {
				t.Fatalf("GenInstanceCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}

// TestGenIsccCodeV0 tests gen_iscc_code_v0 against all conformance vectors.
func TestGenIsccCodeV0(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)
	vectors := loadVectors(t)

	cases := vectors["gen_iscc_code_v0"]
	if len(cases) == 0 {
		t.Fatal("no gen_iscc_code_v0 vectors found")
	}

	for name, v := range cases {
		t.Run(name, func(t *testing.T) {
			// inputs: [codes_array] (no bits param)
			var rawCodes []interface{}
			if err := json.Unmarshal(v.Inputs[0], &rawCodes); err != nil {
				t.Fatalf("parse codes: %v", err)
			}
			codes := make([]string, len(rawCodes))
			for i, c := range rawCodes {
				codes[i] = c.(string)
			}
			expected := expectedISCC(t, name, v)

			got, err := rt.GenIsccCodeV0(ctx, codes)
			if err != nil {
				t.Fatalf("GenIsccCodeV0: %v", err)
			}
			if got != expected {
				t.Fatalf("got %q, want %q", got, expected)
			}
		})
	}
}
