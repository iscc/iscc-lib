// Tests for the pure Go ISCC codec module.
// These tests do NOT require the WASM binary — they test pure Go functions.
package iscc

import (
	"encoding/json"
	"os"
	"strings"
	"testing"
)

// ---- Varnibble roundtrip tests ----

func TestCodecVarnibbleRoundtrip(t *testing.T) {
	testValues := []uint32{0, 1, 7, 8, 71, 72, 583, 584, 4679}
	for _, v := range testValues {
		bits, err := encodeVarnibble(v)
		if err != nil {
			t.Fatalf("encodeVarnibble(%d): %v", v, err)
		}
		bs := bitsToBytes(bits)
		decoded, consumed, err := decodeVarnibbleFromBytes(bs, 0)
		if err != nil {
			t.Fatalf("decodeVarnibble(%d): %v", v, err)
		}
		if decoded != v {
			t.Errorf("roundtrip failed for %d: got %d", v, decoded)
		}
		if consumed != len(bits) {
			t.Errorf("consumed mismatch for %d: got %d, want %d", v, consumed, len(bits))
		}
	}
}

func TestCodecVarnibbleBitLengths(t *testing.T) {
	tests := []struct {
		value   uint32
		bitLen  int
	}{
		{0, 4}, {7, 4},       // 1 nibble
		{8, 8}, {71, 8},      // 2 nibbles
		{72, 12}, {583, 12},  // 3 nibbles
		{584, 16}, {4679, 16}, // 4 nibbles
	}
	for _, tc := range tests {
		bits, err := encodeVarnibble(tc.value)
		if err != nil {
			t.Fatalf("encodeVarnibble(%d): %v", tc.value, err)
		}
		if len(bits) != tc.bitLen {
			t.Errorf("value %d: got %d bits, want %d", tc.value, len(bits), tc.bitLen)
		}
	}
}

func TestCodecVarnibbleOutOfRange(t *testing.T) {
	_, err := encodeVarnibble(4680)
	if err == nil {
		t.Error("expected error for varnibble value 4680")
	}
}

func TestCodecVarnibbleBoundaryBitPatterns(t *testing.T) {
	// Value 0: 0000
	bits0, _ := encodeVarnibble(0)
	expected0 := []bool{false, false, false, false}
	if !boolSliceEqual(bits0, expected0) {
		t.Errorf("value 0: got %v, want %v", bits0, expected0)
	}

	// Value 7: 0111
	bits7, _ := encodeVarnibble(7)
	expected7 := []bool{false, true, true, true}
	if !boolSliceEqual(bits7, expected7) {
		t.Errorf("value 7: got %v, want %v", bits7, expected7)
	}

	// Value 8: 10_000000
	bits8, _ := encodeVarnibble(8)
	expected8 := []bool{true, false, false, false, false, false, false, false}
	if !boolSliceEqual(bits8, expected8) {
		t.Errorf("value 8: got %v, want %v", bits8, expected8)
	}
}

// ---- Bit extraction tests ----

func TestCodecExtractBits(t *testing.T) {
	// 0xA5 = 1010_0101
	data := []byte{0xA5}
	if v := extractBits(data, 0, 4); v != 0b1010 {
		t.Errorf("first nibble: got %d, want %d", v, 0b1010)
	}
	if v := extractBits(data, 4, 4); v != 0b0101 {
		t.Errorf("second nibble: got %d, want %d", v, 0b0101)
	}
	if v := extractBits(data, 0, 8); v != 0xA5 {
		t.Errorf("full byte: got %x, want %x", v, 0xA5)
	}

	// Multi-byte crossing boundary
	data2 := []byte{0xFF, 0x00}
	if v := extractBits(data2, 4, 8); v != 0xF0 {
		t.Errorf("cross-byte: got %x, want %x", v, 0xF0)
	}
}

// ---- Header encoding tests ----

func TestCodecEncodeHeaderMetaV0(t *testing.T) {
	header, err := encodeHeader(MTMeta, STNone, VSV0, 1)
	if err != nil {
		t.Fatal(err)
	}
	expected := []byte{0x00, 0x01}
	if !byteSliceEqual(header, expected) {
		t.Errorf("got %x, want %x", header, expected)
	}
}

func TestCodecEncodeHeaderWithPadding(t *testing.T) {
	// META, NONE, V0, length=8 → 20 bits, padded to 24 = 3 bytes
	header, err := encodeHeader(MTMeta, STNone, VSV0, 8)
	if err != nil {
		t.Fatal(err)
	}
	if len(header) != 3 {
		t.Errorf("header length: got %d, want 3", len(header))
	}
	expected := []byte{0x00, 0x08, 0x00}
	if !byteSliceEqual(header, expected) {
		t.Errorf("got %x, want %x", header, expected)
	}
}

func TestCodecEncodeHeaderDataType(t *testing.T) {
	// DATA=3, NONE=0, V0=0, length=1
	header, err := encodeHeader(MTData, STNone, VSV0, 1)
	if err != nil {
		t.Fatal(err)
	}
	expected := []byte{0x30, 0x01}
	if !byteSliceEqual(header, expected) {
		t.Errorf("got %x, want %x", header, expected)
	}
}

func TestCodecEncodeHeaderInstanceType(t *testing.T) {
	// INSTANCE=4, NONE=0, V0=0, length=1
	header, err := encodeHeader(MTInstance, STNone, VSV0, 1)
	if err != nil {
		t.Fatal(err)
	}
	expected := []byte{0x40, 0x01}
	if !byteSliceEqual(header, expected) {
		t.Errorf("got %x, want %x", header, expected)
	}
}

func TestCodecDecodeHeaderRoundtripAllMainTypes(t *testing.T) {
	mainTypes := []MainType{MTMeta, MTSemantic, MTContent, MTData, MTInstance, MTIscc, MTId, MTFlake}
	for _, mt := range mainTypes {
		header, err := encodeHeader(mt, STNone, VSV0, 1)
		if err != nil {
			t.Fatalf("encodeHeader(%d): %v", mt, err)
		}
		decMt, decSt, decVs, decLn, tail, err := decodeHeader(header)
		if err != nil {
			t.Fatalf("decodeHeader for MT=%d: %v", mt, err)
		}
		if decMt != mt {
			t.Errorf("MainType mismatch: got %d, want %d", decMt, mt)
		}
		if decSt != STNone {
			t.Errorf("SubType mismatch: got %d, want %d", decSt, STNone)
		}
		if decVs != VSV0 {
			t.Errorf("Version mismatch: got %d, want %d", decVs, VSV0)
		}
		if decLn != 1 {
			t.Errorf("Length mismatch: got %d, want 1", decLn)
		}
		if len(tail) != 0 {
			t.Errorf("unexpected tail for MT=%d: %x", mt, tail)
		}
	}
}

func TestCodecDecodeHeaderWithTail(t *testing.T) {
	header, _ := encodeHeader(MTMeta, STNone, VSV0, 1)
	body := []byte{0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44}
	data := make([]byte, 0, len(header)+len(body))
	data = append(data, header...)
	data = append(data, body...)

	mt, st, vs, ln, tail, err := decodeHeader(data)
	if err != nil {
		t.Fatal(err)
	}
	if mt != MTMeta || st != STNone || vs != VSV0 || ln != 1 {
		t.Errorf("header mismatch: mt=%d st=%d vs=%d ln=%d", mt, st, vs, ln)
	}
	if !byteSliceEqual(tail, body) {
		t.Errorf("tail: got %x, want %x", tail, body)
	}
}

func TestCodecDecodeHeaderSubtypes(t *testing.T) {
	header, _ := encodeHeader(MTContent, STImage, VSV0, 1)
	mt, st, vs, ln, _, err := decodeHeader(header)
	if err != nil {
		t.Fatal(err)
	}
	if mt != MTContent || st != STImage || vs != VSV0 || ln != 1 {
		t.Errorf("header mismatch: mt=%d st=%d vs=%d ln=%d", mt, st, vs, ln)
	}
}

// ---- Length encoding tests ----

func TestCodecEncodeLengthStandard(t *testing.T) {
	tests := []struct {
		mt     MainType
		length uint32
		want   uint32
	}{
		{MTMeta, 32, 0}, {MTMeta, 64, 1}, {MTMeta, 96, 2},
		{MTMeta, 128, 3}, {MTMeta, 256, 7},
		{MTData, 64, 1}, {MTInstance, 64, 1},
	}
	for _, tc := range tests {
		got, err := encodeLength(tc.mt, tc.length)
		if err != nil {
			t.Fatalf("encodeLength(%d, %d): %v", tc.mt, tc.length, err)
		}
		if got != tc.want {
			t.Errorf("encodeLength(%d, %d): got %d, want %d", tc.mt, tc.length, got, tc.want)
		}
	}
}

func TestCodecEncodeLengthIscc(t *testing.T) {
	for i := uint32(0); i <= 7; i++ {
		got, err := encodeLength(MTIscc, i)
		if err != nil {
			t.Fatalf("encodeLength(ISCC, %d): %v", i, err)
		}
		if got != i {
			t.Errorf("encodeLength(ISCC, %d): got %d", i, got)
		}
	}
	_, err := encodeLength(MTIscc, 8)
	if err == nil {
		t.Error("expected error for ISCC length 8")
	}
}

func TestCodecEncodeLengthId(t *testing.T) {
	tests := []struct {
		length uint32
		want   uint32
	}{
		{64, 0}, {72, 1}, {80, 2}, {96, 4},
	}
	for _, tc := range tests {
		got, err := encodeLength(MTId, tc.length)
		if err != nil {
			t.Fatalf("encodeLength(ID, %d): %v", tc.length, err)
		}
		if got != tc.want {
			t.Errorf("encodeLength(ID, %d): got %d, want %d", tc.length, got, tc.want)
		}
	}
}

func TestCodecEncodeLengthInvalid(t *testing.T) {
	// Not multiple of 32
	if _, err := encodeLength(MTMeta, 48); err == nil {
		t.Error("expected error for Meta length 48")
	}
	// Too small
	if _, err := encodeLength(MTMeta, 0); err == nil {
		t.Error("expected error for Meta length 0")
	}
	// ID out of range
	if _, err := encodeLength(MTId, 63); err == nil {
		t.Error("expected error for ID length 63")
	}
	if _, err := encodeLength(MTId, 97); err == nil {
		t.Error("expected error for ID length 97")
	}
}

func TestCodecDecodeLengthStandard(t *testing.T) {
	tests := []struct {
		mt   MainType
		ln   uint32
		want uint32
	}{
		{MTMeta, 0, 32}, {MTMeta, 1, 64}, {MTMeta, 7, 256}, {MTData, 1, 64},
	}
	for _, tc := range tests {
		got := decodeLength(tc.mt, tc.ln, STNone)
		if got != tc.want {
			t.Errorf("decodeLength(%d, %d, NONE): got %d, want %d", tc.mt, tc.ln, got, tc.want)
		}
	}
}

func TestCodecDecodeLengthIscc(t *testing.T) {
	// Wide → 256
	if v := decodeLength(MTIscc, 0, STWide); v != 256 {
		t.Errorf("ISCC Wide: got %d, want 256", v)
	}
	// Non-wide: popcount(length) * 64 + 128
	if v := decodeLength(MTIscc, 0, STSum); v != 128 {
		t.Errorf("ISCC Sum ln=0: got %d, want 128", v)
	}
	if v := decodeLength(MTIscc, 1, STNone); v != 192 {
		t.Errorf("ISCC ln=1: got %d, want 192", v)
	}
	if v := decodeLength(MTIscc, 3, STNone); v != 256 {
		t.Errorf("ISCC ln=3: got %d, want 256", v)
	}
	if v := decodeLength(MTIscc, 7, STNone); v != 320 {
		t.Errorf("ISCC ln=7: got %d, want 320", v)
	}
}

func TestCodecDecodeLengthId(t *testing.T) {
	if v := decodeLength(MTId, 0, STNone); v != 64 {
		t.Errorf("ID ln=0: got %d, want 64", v)
	}
	if v := decodeLength(MTId, 1, STNone); v != 72 {
		t.Errorf("ID ln=1: got %d, want 72", v)
	}
	if v := decodeLength(MTId, 4, STNone); v != 96 {
		t.Errorf("ID ln=4: got %d, want 96", v)
	}
}

func TestCodecEncodeLengthRoundtrip(t *testing.T) {
	for _, mt := range []MainType{MTMeta, MTData, MTInstance, MTContent} {
		for bl := uint32(32); bl <= 256; bl += 32 {
			encoded, err := encodeLength(mt, bl)
			if err != nil {
				t.Fatalf("encodeLength(%d, %d): %v", mt, bl, err)
			}
			decoded := decodeLength(mt, encoded, STNone)
			if decoded != bl {
				t.Errorf("roundtrip failed for MT=%d bl=%d: got %d", mt, bl, decoded)
			}
		}
	}
}

// ---- Base32 tests ----

func TestCodecBase32Roundtrip(t *testing.T) {
	testData := [][]byte{
		{0x00},
		{0xFF},
		{0x00, 0x01, 0x02, 0x03},
		{0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE},
		make([]byte, 10),
		{0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF},
	}
	for _, data := range testData {
		encoded := encodeBase32(data)
		decoded, err := decodeBase32(encoded)
		if err != nil {
			t.Fatalf("decodeBase32(%q): %v", encoded, err)
		}
		if !byteSliceEqual(decoded, data) {
			t.Errorf("roundtrip failed: got %x, want %x", decoded, data)
		}
	}
}

func TestCodecBase32NoPadding(t *testing.T) {
	encoded := encodeBase32([]byte{0x00, 0x01})
	if strings.Contains(encoded, "=") {
		t.Error("base32 output should not contain padding")
	}
}

func TestCodecBase32CaseInsensitive(t *testing.T) {
	data := []byte{0xDE, 0xAD, 0xBE, 0xEF}
	encoded := encodeBase32(data)
	lower := strings.ToLower(encoded)
	decoded, err := decodeBase32(lower)
	if err != nil {
		t.Fatalf("decodeBase32(lowercase): %v", err)
	}
	if !byteSliceEqual(decoded, data) {
		t.Errorf("case-insensitive decode failed: got %x, want %x", decoded, data)
	}
}

// ---- Base64 tests ----

func TestCodecEncodeBase64Empty(t *testing.T) {
	if v := EncodeBase64([]byte{}); v != "" {
		t.Errorf("got %q, want empty", v)
	}
}

func TestCodecEncodeBase64KnownValue(t *testing.T) {
	// Python: base64.urlsafe_b64encode(bytes([0,1,2,3])).decode().rstrip("=") == "AAECAw"
	if v := EncodeBase64([]byte{0, 1, 2, 3}); v != "AAECAw" {
		t.Errorf("got %q, want %q", v, "AAECAw")
	}
}

func TestCodecEncodeBase64NoPadding(t *testing.T) {
	for l := 1; l <= 10; l++ {
		data := make([]byte, l)
		for i := range data {
			data[i] = 0xAB
		}
		encoded := EncodeBase64(data)
		if strings.Contains(encoded, "=") {
			t.Errorf("base64 output for len=%d should not contain padding", l)
		}
	}
}

// ---- Unit encoding tests ----

func TestCodecEncodeUnits(t *testing.T) {
	tests := []struct {
		types []MainType
		want  uint32
	}{
		{nil, 0},
		{[]MainType{MTContent}, 1},
		{[]MainType{MTSemantic}, 2},
		{[]MainType{MTSemantic, MTContent}, 3},
		{[]MainType{MTMeta}, 4},
		{[]MainType{MTMeta, MTContent}, 5},
		{[]MainType{MTMeta, MTSemantic}, 6},
		{[]MainType{MTMeta, MTSemantic, MTContent}, 7},
	}
	for _, tc := range tests {
		got, err := encodeUnits(tc.types)
		if err != nil {
			t.Fatalf("encodeUnits(%v): %v", tc.types, err)
		}
		if got != tc.want {
			t.Errorf("encodeUnits(%v): got %d, want %d", tc.types, got, tc.want)
		}
	}
}

func TestCodecEncodeUnitsRejectsInvalid(t *testing.T) {
	invalids := []MainType{MTData, MTInstance, MTIscc}
	for _, mt := range invalids {
		if _, err := encodeUnits([]MainType{mt}); err == nil {
			t.Errorf("expected error for encodeUnits([%d])", mt)
		}
	}
}

func TestCodecDecodeUnits(t *testing.T) {
	tests := []struct {
		id   uint32
		want []MainType
	}{
		{0, nil},
		{1, []MainType{MTContent}},
		{2, []MainType{MTSemantic}},
		{3, []MainType{MTSemantic, MTContent}},
		{4, []MainType{MTMeta}},
		{5, []MainType{MTMeta, MTContent}},
		{6, []MainType{MTMeta, MTSemantic}},
		{7, []MainType{MTMeta, MTSemantic, MTContent}},
	}
	for _, tc := range tests {
		got, err := decodeUnits(tc.id)
		if err != nil {
			t.Fatalf("decodeUnits(%d): %v", tc.id, err)
		}
		if !mainTypeSliceEqual(got, tc.want) {
			t.Errorf("decodeUnits(%d): got %v, want %v", tc.id, got, tc.want)
		}
	}
}

func TestCodecDecodeUnitsInvalid(t *testing.T) {
	if _, err := decodeUnits(8); err == nil {
		t.Error("expected error for decodeUnits(8)")
	}
}

func TestCodecDecodeUnitsRoundtrip(t *testing.T) {
	for id := uint32(0); id <= 7; id++ {
		types, err := decodeUnits(id)
		if err != nil {
			t.Fatal(err)
		}
		encoded, err := encodeUnits(types)
		if err != nil {
			t.Fatal(err)
		}
		if encoded != id {
			t.Errorf("roundtrip failed for id=%d: got %d", id, encoded)
		}
	}
}

// ---- EncodeComponent tests ----

func TestCodecEncodeComponentMetaKnownVector(t *testing.T) {
	// gen_meta_code_v0("Hello World") → "ISCC:AAAWKLHFPV6OPKDG"
	knownCode := "AAAWKLHFPV6OPKDG"
	raw, err := decodeBase32(knownCode)
	if err != nil {
		t.Fatal(err)
	}
	if len(raw) != 10 {
		t.Fatalf("raw length: got %d, want 10", len(raw))
	}

	// Verify header decodes correctly
	mt, st, vs, ln, tail, err := decodeHeader(raw)
	if err != nil {
		t.Fatal(err)
	}
	if mt != MTMeta || st != STNone || vs != VSV0 || ln != 1 {
		t.Errorf("header: mt=%d st=%d vs=%d ln=%d", mt, st, vs, ln)
	}
	if len(tail) != 8 {
		t.Fatalf("tail length: got %d, want 8", len(tail))
	}

	// Re-encode from extracted digest
	result, err := EncodeComponent(uint8(MTMeta), uint8(STNone), uint8(VSV0), 64, tail)
	if err != nil {
		t.Fatal(err)
	}
	if result != knownCode {
		t.Errorf("got %q, want %q", result, knownCode)
	}
}

func TestCodecEncodeComponentRejectsIscc(t *testing.T) {
	_, err := EncodeComponent(uint8(MTIscc), uint8(STSum), uint8(VSV0), 128, make([]byte, 16))
	if err == nil {
		t.Error("expected error for MTIscc")
	}
}

func TestCodecEncodeComponentDataType(t *testing.T) {
	digest := make([]byte, 32)
	for i := range digest {
		digest[i] = 0xAA
	}
	code, err := EncodeComponent(uint8(MTData), uint8(STNone), uint8(VSV0), 64, digest)
	if err != nil {
		t.Fatal(err)
	}

	raw, err := decodeBase32(code)
	if err != nil {
		t.Fatal(err)
	}
	mt, st, _, ln, tail, err := decodeHeader(raw)
	if err != nil {
		t.Fatal(err)
	}
	if mt != MTData || st != STNone || ln != 1 {
		t.Errorf("header: mt=%d st=%d ln=%d", mt, st, ln)
	}
	if !byteSliceEqual(tail, digest[:8]) {
		t.Errorf("tail: got %x, want %x", tail, digest[:8])
	}
}

// ---- EncodeComponent → IsccDecode roundtrip ----

func TestCodecEncodeComponentDecodeRoundtrip(t *testing.T) {
	tests := []struct {
		name string
		mt   uint8
		st   uint8
		bits uint32
	}{
		{"meta-64", uint8(MTMeta), uint8(STNone), 64},
		{"meta-128", uint8(MTMeta), uint8(STNone), 128},
		{"content-text-64", uint8(MTContent), uint8(STNone), 64},
		{"content-image-64", uint8(MTContent), uint8(STImage), 64},
		{"content-audio-64", uint8(MTContent), uint8(STAudio), 64},
		{"data-64", uint8(MTData), uint8(STNone), 64},
		{"instance-64", uint8(MTInstance), uint8(STNone), 64},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			nbytes := int(tc.bits / 8)
			digest := make([]byte, nbytes)
			for i := range digest {
				digest[i] = byte(i + 1)
			}
			code, err := EncodeComponent(tc.mt, tc.st, 0, tc.bits, digest)
			if err != nil {
				t.Fatal(err)
			}

			result, err := IsccDecode("ISCC:" + code)
			if err != nil {
				t.Fatal(err)
			}
			if result.Maintype != tc.mt {
				t.Errorf("maintype: got %d, want %d", result.Maintype, tc.mt)
			}
			if result.Subtype != tc.st {
				t.Errorf("subtype: got %d, want %d", result.Subtype, tc.st)
			}
			if result.Version != 0 {
				t.Errorf("version: got %d, want 0", result.Version)
			}
			if !byteSliceEqual(result.Digest, digest) {
				t.Errorf("digest: got %x, want %x", result.Digest, digest)
			}
		})
	}
}

// ---- IsccDecode known vector tests ----

func TestCodecIsccDecodeMetaCodeVector(t *testing.T) {
	// gen_meta_code_v0 test_0001_title_only → "ISCC:AAAZXZ6OU74YAZIM"
	result, err := IsccDecode("ISCC:AAAZXZ6OU74YAZIM")
	if err != nil {
		t.Fatal(err)
	}
	if result.Maintype != uint8(MTMeta) {
		t.Errorf("maintype: got %d, want %d", result.Maintype, MTMeta)
	}
	if result.Subtype != uint8(STNone) {
		t.Errorf("subtype: got %d, want %d", result.Subtype, STNone)
	}
	if result.Version != uint8(VSV0) {
		t.Errorf("version: got %d, want 0", result.Version)
	}
	if result.Length != 1 {
		t.Errorf("length: got %d, want 1", result.Length)
	}
	if len(result.Digest) != 8 {
		t.Errorf("digest length: got %d, want 8", len(result.Digest))
	}
}

func TestCodecIsccDecodeDataCodeVector(t *testing.T) {
	// gen_data_code_v0 test_0000_two_bytes_64 → "ISCC:GAAXL2XYM5BQIAZ3"
	result, err := IsccDecode("ISCC:GAAXL2XYM5BQIAZ3")
	if err != nil {
		t.Fatal(err)
	}
	if result.Maintype != uint8(MTData) {
		t.Errorf("maintype: got %d, want %d", result.Maintype, MTData)
	}
	if result.Subtype != uint8(STNone) {
		t.Errorf("subtype: got %d, want %d", result.Subtype, STNone)
	}
	if len(result.Digest) != 8 {
		t.Errorf("digest length: got %d, want 8", len(result.Digest))
	}
}

func TestCodecIsccDecodeStripsDashes(t *testing.T) {
	// Same code with dashes inserted should decode identically
	result1, err := IsccDecode("ISCC:AAAZXZ6OU74YAZIM")
	if err != nil {
		t.Fatal(err)
	}
	result2, err := IsccDecode("ISCC:AAAZ-XZ6O-U74Y-AZIM")
	if err != nil {
		t.Fatal(err)
	}
	if !byteSliceEqual(result1.Digest, result2.Digest) {
		t.Error("dash-stripped decode mismatch")
	}
}

func TestCodecIsccDecodeWithoutPrefix(t *testing.T) {
	result, err := IsccDecode("AAAZXZ6OU74YAZIM")
	if err != nil {
		t.Fatal(err)
	}
	if result.Maintype != uint8(MTMeta) {
		t.Errorf("maintype: got %d, want %d", result.Maintype, MTMeta)
	}
}

func TestCodecIsccDecodeInvalidBase32(t *testing.T) {
	_, err := IsccDecode("ISCC:!@#$%^&*")
	if err == nil {
		t.Error("expected error for invalid base32")
	}
}

func TestCodecIsccDecodeBodyTooShort(t *testing.T) {
	// Create a header for 64-bit Meta but provide no body
	header, _ := encodeHeader(MTMeta, STNone, VSV0, 1)
	code := encodeBase32(header)
	_, err := IsccDecode(code)
	if err == nil {
		t.Error("expected error for body too short")
	}
	if !strings.Contains(err.Error(), "too short") {
		t.Errorf("error should mention 'too short': %v", err)
	}
}

// ---- IsccDecompose tests ----

func TestCodecDecomposeSingleMetaUnit(t *testing.T) {
	result, err := IsccDecompose("AAAYPXW445FTYNJ3")
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 1 || result[0] != "AAAYPXW445FTYNJ3" {
		t.Errorf("got %v, want [AAAYPXW445FTYNJ3]", result)
	}
}

func TestCodecDecomposeSingleUnitWithPrefix(t *testing.T) {
	result, err := IsccDecompose("ISCC:AAAYPXW445FTYNJ3")
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 1 || result[0] != "AAAYPXW445FTYNJ3" {
		t.Errorf("got %v, want [AAAYPXW445FTYNJ3]", result)
	}
}

func TestCodecDecomposeInvalidBase32(t *testing.T) {
	_, err := IsccDecompose("!!!invalid!!!")
	if err == nil {
		t.Error("expected error for invalid base32")
	}
}

func TestCodecDecomposeTruncatedBody(t *testing.T) {
	// Meta-Code header for 64 bits but only 4 body bytes
	encodedLen, _ := encodeLength(MTMeta, 64)
	header, _ := encodeHeader(MTMeta, STNone, VSV0, encodedLen)
	shortBody := []byte{0xAB, 0xAB, 0xAB, 0xAB}
	raw := append(header, shortBody...)
	code := encodeBase32(raw)
	_, err := IsccDecompose(code)
	if err == nil {
		t.Error("expected error for truncated body")
	}
	if !strings.Contains(err.Error(), "truncated") {
		t.Errorf("error should mention truncation: %v", err)
	}
}

// ---- IsccDecompose conformance tests using data.json ----

func TestCodecDecomposeConformanceVectors(t *testing.T) {
	dataJSON, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Skipf("data.json not found: %v", err)
	}

	var data map[string]json.RawMessage
	if err := json.Unmarshal(dataJSON, &data); err != nil {
		t.Fatal(err)
	}

	// Parse gen_iscc_code_v0 section
	var isccSection map[string]struct {
		Inputs  []json.RawMessage `json:"inputs"`
		Outputs struct {
			ISCC string `json:"iscc"`
		} `json:"outputs"`
	}
	if err := json.Unmarshal(data["gen_iscc_code_v0"], &isccSection); err != nil {
		t.Fatal(err)
	}

	for name, tc := range isccSection {
		t.Run(name, func(t *testing.T) {
			isccCode := strings.TrimPrefix(tc.Outputs.ISCC, "ISCC:")

			// Count input codes
			var inputCodes []string
			if err := json.Unmarshal(tc.Inputs[0], &inputCodes); err != nil {
				t.Fatal(err)
			}

			decomposed, err := IsccDecompose(isccCode)
			if err != nil {
				t.Fatal(err)
			}

			// Decomposed unit count should match input count
			if len(decomposed) != len(inputCodes) {
				t.Errorf("unit count: got %d, want %d", len(decomposed), len(inputCodes))
			}

			// Each decomposed code should decode to a valid non-ISCC MainType
			for _, code := range decomposed {
				result, err := IsccDecode(code)
				if err != nil {
					t.Fatalf("IsccDecode(%q): %v", code, err)
				}
				if MainType(result.Maintype) == MTIscc {
					t.Errorf("decomposed unit should not be MTIscc: %q", code)
				}
			}

			// Last two units must be Data + Instance
			if len(decomposed) >= 2 {
				r1, _ := IsccDecode(decomposed[len(decomposed)-2])
				r2, _ := IsccDecode(decomposed[len(decomposed)-1])
				if MainType(r1.Maintype) != MTData {
					t.Errorf("second-to-last should be Data, got %d", r1.Maintype)
				}
				if MainType(r2.Maintype) != MTInstance {
					t.Errorf("last should be Instance, got %d", r2.Maintype)
				}
			}
		})
	}
}

// ---- IsccDecode conformance: decode all gen_meta_code_v0 and gen_data_code_v0 vectors ----

func TestCodecIsccDecodeConformanceVectors(t *testing.T) {
	dataJSON, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Skipf("data.json not found: %v", err)
	}

	var data map[string]json.RawMessage
	if err := json.Unmarshal(dataJSON, &data); err != nil {
		t.Fatal(err)
	}

	// Test Meta-Code vectors
	t.Run("gen_meta_code_v0", func(t *testing.T) {
		var section map[string]struct {
			Outputs struct {
				ISCC string `json:"iscc"`
			} `json:"outputs"`
		}
		if err := json.Unmarshal(data["gen_meta_code_v0"], &section); err != nil {
			t.Fatal(err)
		}
		for name, tc := range section {
			t.Run(name, func(t *testing.T) {
				result, err := IsccDecode(tc.Outputs.ISCC)
				if err != nil {
					t.Fatal(err)
				}
				if result.Maintype != uint8(MTMeta) {
					t.Errorf("maintype: got %d, want %d", result.Maintype, MTMeta)
				}
				if result.Subtype != uint8(STNone) {
					t.Errorf("subtype: got %d, want %d", result.Subtype, STNone)
				}
			})
		}
	})

	// Test Data-Code vectors
	t.Run("gen_data_code_v0", func(t *testing.T) {
		var section map[string]struct {
			Outputs struct {
				ISCC string `json:"iscc"`
			} `json:"outputs"`
		}
		if err := json.Unmarshal(data["gen_data_code_v0"], &section); err != nil {
			t.Fatal(err)
		}
		for name, tc := range section {
			t.Run(name, func(t *testing.T) {
				result, err := IsccDecode(tc.Outputs.ISCC)
				if err != nil {
					t.Fatal(err)
				}
				if result.Maintype != uint8(MTData) {
					t.Errorf("maintype: got %d, want %d", result.Maintype, MTData)
				}
			})
		}
	})
}

// ---- Helper comparison functions ----

func boolSliceEqual(a, b []bool) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func byteSliceEqual(a, b []byte) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func mainTypeSliceEqual(a, b []MainType) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
