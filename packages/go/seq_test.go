// Tests for the ISCC-SEQ codec (IEP-0020): EncodeSeq, DecodeSeq, readUnit and
// the raw header helpers. Port of crates/iscc-lib/tests/test_seq.rs.
package iscc

import (
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"slices"
	"strings"
	"testing"
)

var seqUnits = []string{
	"ISCC:AADVPDD4R6733NMPFH3D57VTQ4KVH6VHL74XIWGUT37V5ZG56NV3NSI",
	"ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI",
	"ISCC:GADQLNA7GRZESMRF2J7NZPNWGI3II2ST5YUN5SS6GVQ2ZQGJXPPYDNI",
	"ISCC:IAD2KIVPJIWJZP3KQCESJL6SVT5APEZUPOJWM6HVTAXCF7OT3VFA4NY",
}

const seqHex = "0007578c7c8fbfbdb58f29f63efeb3871553faa75ff97458d49eff5ee4ddf36bb6c92007a88248c53a852c8d" +
	"7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305300705b41f3472493225d27edcbdb63236846a53" +
	"ee28deca5e3561acc0c9bbdf81b54007a522af4a2c9cbf6a808924afd2acfa0793347b936678f5982e22fdd3" +
	"dd4a0e37"

const seqB64 = "AAdXjHyPv721jyn2Pv6zhxVT+qdf+XRY1J7/XuTd82u2ySAHqIJIxTqFLI1+nFRwR8mjbEBStqyr7x4qTOFeWG4M" +
	"swUwBwW0HzRySTIl0n7cvbYyNoRqU+4o3speNWGswMm734G1QAelIq9KLJy/aoCJJK/SrPoHkzR7k2Z49ZguIv3T" +
	"3UoONw=="

const seqText = "ISCC:AADVPDD4R6733NMPFH3D57VTQ4KVH6VHL74XIWGUT37V5ZG56NV3NSJAA6UIESGFHKCSZDL6TRKHAR6JUNWE" +
	"AUVWVSV66HRKJTQV4WDOBSZQKMAHAW2B6NDSJEZCLUT63S63MMRWQRVFH3RI33FF4NLBVTAMTO67QG2UAB5FEKXUU" +
	"LE4X5VIBCJEV7JKZ6QHSM2HXE3GPD2ZQLRC7XJ52SQOG4"

const seqSingleHex = "2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305"

var seqUnits128 = []string{
	"ISCC:GABQLNA7GRZESMRF2J7NZPNWGI3II",
	"ISCC:IAB2KIVPJIWJZP3KQCESJL6SVT5AO",
}

const seqHex128 = "300305b41f3472493225d27edcbdb63236844003a522af4a2c9cbf6a808924afd2acfa07"

var seqUnitsMixed = []string{
	"ISCC:EEAYYNMFG5XOELK2",
	"ISCC:EIBZ6B7T76PAAYP7H334X77754376",
	"ISCC:GADQLNA7GRZESMRF2J7NZPNWGI3II2ST5YUN5SS6GVQ2ZQGJXPPYDNI",
}

const seqHexMixed = "21018c3585376ee22d5a22039f07f3ff9e0061ff3ef7cbffffef37ff300705b41f3472493225d27edcbdb632" +
	"36846a53ee28deca5e3561acc0c9bbdf81b5"

const seqIsccCodeHex = "5005578c7c8fbfbdb58fa88248c53a852c8d05b41f3472493225a522af4a2c9cbf6a"
const seqIsccIDHex = "60100000000000000000"

// seqUnknownSTHeaders lists Content-Code header bytes for unknown SubType/Version
// values with Length 7, as produced by iscc-core 1.5.0
// encode_header(MT.CONTENT, st, vs, 7).
var seqUnknownSTHeaders = []struct {
	st, vs uint32
	hex    string
}{
	{7, 0, "2707"}, {7, 1, "2717"},
	{8, 0, "280070"}, {8, 1, "280170"},
	{71, 0, "2bf070"}, {71, 1, "2bf170"},
	{72, 0, "2c0007"}, {72, 1, "2c0017"},
	{583, 0, "2dff07"}, {583, 1, "2dff17"},
	{584, 0, "2e000070"}, {584, 1, "2e000170"},
	{4679, 0, "2efff070"}, {4679, 1, "2efff170"},
}

// seqStrictTwins lists the canonical twins of the non-canonical base32 encode_seq
// vectors in testdata/data-edge.json (decisions.md 2026-11-18), each with its bytes.
// The rejection of the aliases themselves is covered by data_edge_test.go.
var seqStrictTwins = []struct {
	name, twin, hex string
}{
	{"32bit", "ISCC:GAAAAAAAAA", "300000000000"},
	{"256bit", seqUnits[1], seqSingleHex},
	{"case_fold", "ISCC:GAAWDLQNG6SSDPF2", "300161ae0d37a521bcba"},
}

// hx decodes a hex test constant into bytes.
func hx(t *testing.T, s string) []byte {
	t.Helper()
	b, err := hex.DecodeString(s)
	if err != nil {
		t.Fatalf("bad hex %q: %v", s, err)
	}
	return b
}

// typedHeader returns the header for a SEQ-permitted MainType with SubType 0, Version 0.
func typedHeader(t *testing.T, mt MainType, ln uint32) []byte {
	t.Helper()
	h, err := encodeHeader(mt, STNone, VSV0, ln)
	if err != nil {
		t.Fatal(err)
	}
	return h
}

// seqBody returns n bytes counting up from zero.
func seqBody(n int) []byte {
	b := make([]byte, n)
	for i := range b {
		b[i] = byte(i)
	}
	return b
}

// mustEncodeSeq calls EncodeSeq and fails the test on error.
func mustEncodeSeq(t *testing.T, units []string) []byte {
	t.Helper()
	seq, err := EncodeSeq(units)
	if err != nil {
		t.Fatalf("EncodeSeq(%q): %v", units, err)
	}
	return seq
}

// mustDecodeSeq calls DecodeSeq and fails the test on error.
func mustDecodeSeq(t *testing.T, data []byte) []string {
	t.Helper()
	units, err := DecodeSeq(data)
	if err != nil {
		t.Fatalf("DecodeSeq(%x): %v", data, err)
	}
	return units
}

// assertErrContains fails unless err is non-nil and its message contains needle.
func assertErrContains(t *testing.T, err error, needle string) {
	t.Helper()
	if err == nil {
		t.Fatalf("expected error containing %q, got nil", needle)
	}
	if !strings.Contains(err.Error(), needle) {
		t.Fatalf("error %q lacks %q", err.Error(), needle)
	}
}

// ---- Positive cases ----

func TestSeqIEPExample(t *testing.T) {
	value := mustEncodeSeq(t, seqUnits)
	if got := mustDecodeSeq(t, value); !slices.Equal(got, seqUnits) {
		t.Fatalf("roundtrip: got %v", got)
	}
	if text := "ISCC:" + encodeBase32(value); text != seqText {
		t.Fatalf("text form: got %s", text)
	}
}

func TestEncodeSeqVector1(t *testing.T) {
	value := mustEncodeSeq(t, seqUnits)
	if len(value) != 136 {
		t.Fatalf("len: got %d", len(value))
	}
	if hex.EncodeToString(value) != seqHex {
		t.Fatalf("hex: got %x", value)
	}
	if base64.StdEncoding.EncodeToString(value) != seqB64 {
		t.Fatal("base64 mismatch")
	}
	if n := len(encodeBase32(value)); n != 218 {
		t.Fatalf("base32 len: got %d", n)
	}
}

func TestDecodeSeqVector1(t *testing.T) {
	if got := mustDecodeSeq(t, hx(t, seqHex)); !slices.Equal(got, seqUnits) {
		t.Fatalf("hex: got %v", got)
	}
	raw, err := base64.StdEncoding.DecodeString(seqB64)
	if err != nil {
		t.Fatal(err)
	}
	if got := mustDecodeSeq(t, raw); !slices.Equal(got, seqUnits) {
		t.Fatalf("base64: got %v", got)
	}
}

func TestSeqSingleUnit(t *testing.T) {
	unit := seqUnits[1]
	value := mustEncodeSeq(t, []string{unit})
	if hex.EncodeToString(value) != seqSingleHex {
		t.Fatalf("got %x", value)
	}
	if got := mustDecodeSeq(t, value); !slices.Equal(got, []string{unit}) {
		t.Fatalf("got %v", got)
	}
}

func TestSeqTwo128BitUnits(t *testing.T) {
	value := mustEncodeSeq(t, seqUnits128)
	if hex.EncodeToString(value) != seqHex128 {
		t.Fatalf("got %x", value)
	}
	if got := mustDecodeSeq(t, value); !slices.Equal(got, seqUnits128) {
		t.Fatalf("got %v", got)
	}
}

func TestSeqMixedLengthsVector2(t *testing.T) {
	value := mustEncodeSeq(t, seqUnitsMixed)
	if hex.EncodeToString(value) != seqHexMixed {
		t.Fatalf("got %x", value)
	}
	if got := mustDecodeSeq(t, value); !slices.Equal(got, seqUnitsMixed) {
		t.Fatalf("got %v", got)
	}
}

func TestEncodeSeqTolerantInput(t *testing.T) {
	variants := map[string]func(string) string{
		"bare":       func(u string) string { return strings.TrimPrefix(u, "ISCC:") },
		"lower":      strings.ToLower,
		"bare_lower": func(u string) string { return strings.ToLower(strings.TrimPrefix(u, "ISCC:")) },
		"hyphen":     func(u string) string { return u[:9] + "-" + u[9:] },
	}
	for name, form := range variants {
		units := make([]string, len(seqUnits))
		for i, u := range seqUnits {
			units[i] = form(u)
		}
		if got := hex.EncodeToString(mustEncodeSeq(t, units)); got != seqHex {
			t.Errorf("%s: got %s", name, got)
		}
	}
}

func TestEncodeSeqMultibaseLikeContent(t *testing.T) {
	for _, unit := range []string{
		"ISCC:FAAAAAAAAAAA", "iscc:faaa-aaaaaaaa", "FAAA-AAAAAAAA", "faaaaaaaaaaa", "faaa-aaaaaaaa",
	} {
		if got := hex.EncodeToString(mustEncodeSeq(t, []string{unit})); got != "28000000000000" {
			t.Errorf("%s: got %s", unit, got)
		}
	}
}

func TestEncodeSeqLowercaseMultibaseLikeHyphenated(t *testing.T) {
	// Only works because units are upper-cased before isccClean sees the "f".
	if got := hex.EncodeToString(mustEncodeSeq(t, []string{"faaa-aaaaaaaa"})); got != "28000000000000" {
		t.Fatalf("got %s", got)
	}
}

func TestEncodeSeqMultibaseLikeMeta(t *testing.T) {
	if got := hex.EncodeToString(mustEncodeSeq(t, []string{"baaa-aaaaaaaa"})); got != "08000000000000" {
		t.Fatalf("got %s", got)
	}
}

func TestSeqAllLengthsRoundtrip(t *testing.T) {
	for _, mt := range seqMainTypes {
		for ln := uint32(0); ln < 8; ln++ {
			raw := append(typedHeader(t, mt, ln), seqBody(int(4*(ln+1)))...)
			units := mustDecodeSeq(t, raw)
			if len(units) != 1 {
				t.Fatalf("mt=%d ln=%d: %d units", mt, ln, len(units))
			}
			if got := mustEncodeSeq(t, units); !byteSliceEqual(got, raw) {
				t.Errorf("mt=%d ln=%d: got %x want %x", mt, ln, got, raw)
			}
		}
	}
}

func TestEncodeHeaderRawUnknownSubtypes(t *testing.T) {
	for _, c := range seqUnknownSTHeaders {
		got, err := encodeHeaderRaw(uint32(MTContent), c.st, c.vs, 7)
		if err != nil {
			t.Fatalf("st=%d vs=%d: %v", c.st, c.vs, err)
		}
		if hex.EncodeToString(got) != c.hex {
			t.Errorf("st=%d vs=%d: got %x want %s", c.st, c.vs, got, c.hex)
		}
		mt, st, vs, ln, tailStart, err := decodeHeaderRaw(got)
		if err != nil || mt != uint32(MTContent) || st != c.st || vs != c.vs || ln != 7 || tailStart != len(got) {
			t.Errorf("decodeHeaderRaw(%x) = %d %d %d %d %d %v", got, mt, st, vs, ln, tailStart, err)
		}
	}
}

func TestSeqUnknownSubtypeVersionRoundtrip(t *testing.T) {
	for _, c := range seqUnknownSTHeaders {
		raw := append(hx(t, c.hex), seqBody(32)...)
		units := mustDecodeSeq(t, raw)
		if len(units) != 1 {
			t.Fatalf("st=%d vs=%d: %d units", c.st, c.vs, len(units))
		}
		if got := mustEncodeSeq(t, units); !byteSliceEqual(got, raw) {
			t.Errorf("st=%d vs=%d: got %x", c.st, c.vs, got)
		}
		bare := strings.TrimPrefix(units[0], "ISCC:")
		hyphen := bare[:4] + "-" + bare[4:]
		for _, variant := range []string{bare, strings.ToLower(bare), hyphen, strings.ToLower(hyphen)} {
			if got := mustEncodeSeq(t, []string{variant}); !byteSliceEqual(got, raw) {
				t.Errorf("%s: got %x", variant, got)
			}
		}
	}
}

func TestSeqSubtypeAbove255Roundtrip(t *testing.T) {
	// SubType 4679 would wrap to 71 through the uint8 typed header path.
	raw := append(hx(t, "2efff070"), seqBody(32)...)
	units := mustDecodeSeq(t, raw)
	if got := mustEncodeSeq(t, units); !byteSliceEqual(got, raw) {
		t.Fatalf("got %x want %x", got, raw)
	}
}

func TestSeqVersion1Roundtrip(t *testing.T) {
	// Data-Code Version 1 is rejected by the typed decodeHeader but valid in a SEQ.
	raw := hx(t, "301000000000")
	units := mustDecodeSeq(t, raw)
	if !slices.Equal(units, []string{"ISCC:GAIAAAAAAA"}) {
		t.Fatalf("got %v", units)
	}
	if got := mustEncodeSeq(t, units); !byteSliceEqual(got, raw) {
		t.Fatalf("got %x", got)
	}
}

func TestSeqUnknownSubtypeCanonicalStrings(t *testing.T) {
	// Exact unit strings from iscc-core 1.5.0 decode_seq (header + bytes 0..32).
	cases := [][2]string{
		{"2707", "ISCC:E4DQAAICAMCAKBQHBAEQUCYMBUHA6EARCIJRIFIWC4MBSGQ3DQOR4HY"},
		{"280070", "ISCC:FAAHAAABAIBQIBIGA4EASCQLBQGQ4DYQCEJBGFAVCYLRQGI2DMOB2HQ7"},
		{"2c0017", "ISCC:FQABOAABAIBQIBIGA4EASCQLBQGQ4DYQCEJBGFAVCYLRQGI2DMOB2HQ7"},
		{"2efff170", "ISCC:F377C4AAAEBAGBAFAYDQQCIKBMGA2DQPCAIREEYUCULBOGAZDINRYHI6D4"},
	}
	for _, c := range cases {
		raw := append(hx(t, c[0]), seqBody(32)...)
		if got := mustDecodeSeq(t, raw); !slices.Equal(got, []string{c[1]}) {
			t.Errorf("%s: got %v", c[0], got)
		}
	}
}

func TestDecodeSeqUndefinedSubtype(t *testing.T) {
	if got := mustDecodeSeq(t, hx(t, "270000000000")); !slices.Equal(got, []string{"ISCC:E4AAAAAAAA"}) {
		t.Fatalf("got %v", got)
	}
}

func TestSeqInteropWithIsccDecompose(t *testing.T) {
	value := mustEncodeSeq(t, seqUnits)
	decomposed, err := IsccDecompose("ISCC:" + encodeBase32(value))
	if err != nil {
		t.Fatal(err)
	}
	for i, u := range seqUnits {
		if decomposed[i] != strings.TrimPrefix(u, "ISCC:") {
			t.Errorf("unit %d: got %s", i, decomposed[i])
		}
	}
}

func TestSeqRepeatedKeysKeepOrder(t *testing.T) {
	data64, _ := GenDataCodeV0([]byte("a"), 64)
	data256, _ := GenDataCodeV0([]byte("a"), 256)
	inst64, _ := GenInstanceCodeV0([]byte("a"), 64)
	inst256, _ := GenInstanceCodeV0([]byte("a"), 256)
	units := []string{data256.Iscc, inst64.Iscc, data64.Iscc, inst256.Iscc}
	if got := mustDecodeSeq(t, mustEncodeSeq(t, units)); !slices.Equal(got, units) {
		t.Fatalf("got %v", got)
	}
}

func TestSeqConformanceDecodeVectorsRoundtrip(t *testing.T) {
	data, err := parseSeqConformanceData([]byte(conformanceData))
	if err != nil {
		t.Fatal(err)
	}
	cases := data["decode_seq"]
	if len(cases) == 0 {
		t.Fatal("no decode_seq vectors")
	}
	for name, tc := range cases {
		var input string
		if err := json.Unmarshal(tc.Inputs[0], &input); err != nil {
			t.Fatal(err)
		}
		raw := hx(t, strings.TrimPrefix(input, "bytes:"))
		if got := mustEncodeSeq(t, mustDecodeSeq(t, raw)); !byteSliceEqual(got, raw) {
			t.Errorf("%s: got %x", name, got)
		}
	}
}

// ---- DecodeSeq negative cases ----

func TestDecodeSeqEmpty(t *testing.T) {
	_, err := DecodeSeq(nil)
	assertErrContains(t, err, "Empty")
}

func TestDecodeSeqIncompleteHeader(t *testing.T) {
	_, err := DecodeSeq(hx(t, "28"))
	assertErrContains(t, err, "Malformed ISCC-HEADER at offset 0")
}

func TestDecodeSeqZeroHeaderPaddingAccepted(t *testing.T) {
	if got := mustDecodeSeq(t, hx(t, "28000000000000")); !slices.Equal(got, []string{"ISCC:FAAAAAAAAAAA"}) {
		t.Fatalf("got %v", got)
	}
}

func TestDecodeSeqNonzeroHeaderPadding(t *testing.T) {
	_, err := DecodeSeq(hx(t, "28000100000000"))
	assertErrContains(t, err, "Non-canonical ISCC-HEADER at offset 0")
}

func TestDecodeSeqNonzeroHeaderPaddingSecondUnit(t *testing.T) {
	first := hx(t, seqSingleHex)
	_, err := DecodeSeq(append(first, hx(t, "28000100000000")...))
	assertErrContains(t, err, fmt.Sprintf("Non-canonical ISCC-HEADER at offset %d", len(first)))
}

func TestDecodeSeqLengthAbove7(t *testing.T) {
	for _, ln := range []uint32{8, 9} {
		raw := append(typedHeader(t, MTContent, ln), make([]byte, 4*(ln+1))...)
		_, err := DecodeSeq(raw)
		assertErrContains(t, err, fmt.Sprintf("Unsupported Length field %d", ln))
	}
}

func TestDecodeSeqTruncated(t *testing.T) {
	raw := hx(t, seqHex)
	_, err := DecodeSeq(raw[:len(raw)-1])
	assertErrContains(t, err, "Truncated ISCC-UNIT at offset 102")
}

func TestDecodeSeqTrailingByte(t *testing.T) {
	_, err := DecodeSeq(append(hx(t, seqHex), 0))
	assertErrContains(t, err, "offset 136")
}

func TestDecodeSeqRejectsIsccCode(t *testing.T) {
	_, err := DecodeSeq(hx(t, seqIsccCodeHex))
	assertErrContains(t, err, "MainType 5 is not permitted")
}

func TestDecodeSeqRejectsIsccID(t *testing.T) {
	_, err := DecodeSeq(hx(t, seqIsccIDHex))
	assertErrContains(t, err, "MainType 6 is not permitted")
}

func TestDecodeSeqRejectsFlake(t *testing.T) {
	raw := append(typedHeader(t, MTFlake, 1), make([]byte, 8)...)
	_, err := DecodeSeq(raw)
	assertErrContains(t, err, "MainType 7 is not permitted")
}

func TestIEP0020Table2Rejects(t *testing.T) {
	cases := [][2]string{
		{"28000100000000", "Non-canonical ISCC-HEADER at offset 0"},
		{"200800" + strings.Repeat("00", 36), "Unsupported Length field 8 at offset 0"},
		{"3000000000", "Truncated ISCC-UNIT at offset 0"},
		{"30000000000000", "Malformed ISCC-HEADER at offset 6"},
	}
	for _, c := range cases {
		_, err := DecodeSeq(hx(t, c[0]))
		assertErrContains(t, err, c[1])
	}
}

func TestIEP0020AcceptsDataVersion1Then0(t *testing.T) {
	got := mustDecodeSeq(t, hx(t, "301000000000300000000000"))
	if !slices.Equal(got, []string{"ISCC:GAIAAAAAAA", "ISCC:GAAAAAAAAA"}) {
		t.Fatalf("got %v", got)
	}
}

// ---- EncodeSeq negative cases ----

func TestEncodeSeqNil(t *testing.T) {
	_, err := EncodeSeq(nil)
	assertErrContains(t, err, "at least one")
}

func TestEncodeSeqEmptyList(t *testing.T) {
	_, err := EncodeSeq([]string{})
	assertErrContains(t, err, "at least one")
}

func TestEncodeSeqEmptyUnit(t *testing.T) {
	_, err := EncodeSeq([]string{""})
	assertErrContains(t, err, "must not be empty")
}

func TestEncodeSeqBlankUnit(t *testing.T) {
	_, err := EncodeSeq([]string{"  \u001f"})
	assertErrContains(t, err, "must not be empty")
}

func TestEncodeSeqRejectsIsccCode(t *testing.T) {
	_, err := EncodeSeq([]string{"ISCC:" + encodeBase32(hx(t, seqIsccCodeHex))})
	assertErrContains(t, err, "not permitted")
}

func TestEncodeSeqRejectsIsccID(t *testing.T) {
	_, err := EncodeSeq([]string{"ISCC:" + encodeBase32(hx(t, seqIsccIDHex))})
	assertErrContains(t, err, "not permitted")
}

func TestEncodeSeqRejectsFlake(t *testing.T) {
	raw := append(typedHeader(t, MTFlake, 1), make([]byte, 8)...)
	_, err := EncodeSeq([]string{"ISCC:" + encodeBase32(raw)})
	assertErrContains(t, err, "not permitted")
}

func TestEncodeSeqRejectsMultiformatBase16(t *testing.T) {
	raw := hx(t, seqSingleHex)
	if _, err := EncodeSeq([]string{"f" + hex.EncodeToString(append([]byte{0xcc, 0x01}, raw...))}); err == nil {
		t.Fatal("base16 multiformat accepted")
	}
}

func TestEncodeSeqRejectsMultiformatBase32(t *testing.T) {
	raw := hx(t, seqSingleHex)
	mf := "b" + strings.ToLower(encodeBase32(append([]byte{0xcc, 0x01}, raw...)))
	if _, err := EncodeSeq([]string{mf}); err == nil {
		t.Fatal("base32 multiformat accepted")
	}
}

func TestEncodeSeqTrailingData(t *testing.T) {
	_, err := EncodeSeq([]string{encodeBase32(make([]byte, 8))})
	assertErrContains(t, err, "Trailing data")
}

func TestEncodeSeqMultipleUnitsInOneString(t *testing.T) {
	_, err := EncodeSeq([]string{seqText})
	assertErrContains(t, err, "Trailing data")
}

func TestEncodeSeqStrictBase32TwinsAccepted(t *testing.T) {
	for _, c := range seqStrictTwins {
		t.Run(c.name, func(t *testing.T) {
			if got := hex.EncodeToString(mustEncodeSeq(t, []string{c.twin})); got != c.hex {
				t.Fatalf("twin %q: got %s want %s", c.twin, got, c.hex)
			}
		})
	}
}

// ---- readUnit ----

func TestReadUnitSingle(t *testing.T) {
	raw := hx(t, seqSingleHex)
	if end, err := readUnit(raw, 0); err != nil || end != len(raw) {
		t.Fatalf("got %d, %v", end, err)
	}
}

func TestReadUnitSequentialOffsets(t *testing.T) {
	raw := hx(t, seqHex)
	offsets := []int{0}
	for offsets[len(offsets)-1] < len(raw) {
		end, err := readUnit(raw, offsets[len(offsets)-1])
		if err != nil {
			t.Fatal(err)
		}
		offsets = append(offsets, end)
	}
	if !slices.Equal(offsets, []int{0, 34, 68, 102, 136}) {
		t.Fatalf("got %v", offsets)
	}
}

func TestReadUnitOffsets128(t *testing.T) {
	raw := hx(t, seqHex128)
	if end, err := readUnit(raw, 0); err != nil || end != 18 {
		t.Fatalf("offset 0: got %d, %v", end, err)
	}
	if end, err := readUnit(raw, 18); err != nil || end != len(raw) {
		t.Fatalf("offset 18: got %d, %v", end, err)
	}
}

func TestSeqMainTypes(t *testing.T) {
	// Pins the permitted set: no vector or other test carries a SEMANTIC unit.
	want := []MainType{MTMeta, MTSemantic, MTContent, MTData, MTInstance}
	if !slices.Equal(seqMainTypes[:], want) {
		t.Fatalf("got %v", seqMainTypes)
	}
}

func TestReadUnitTruncatedAtOffset(t *testing.T) {
	raw := hx(t, seqHex128)
	_, err := readUnit(raw[:len(raw)-1], 18)
	assertErrContains(t, err, "Truncated ISCC-UNIT at offset 18")
}

func TestReadUnitOffsetOutOfRange(t *testing.T) {
	raw := hx(t, "300000000000")
	for _, offset := range []int{len(raw), len(raw) + 1, 100, -1} {
		_, err := readUnit(raw, offset)
		assertErrContains(t, err, fmt.Sprintf("Malformed ISCC-HEADER at offset %d", offset))
	}
}

// ---- Typed codec keeps rejecting what DecodeSeq accepts ----

func TestTypedCodecRejectsDataVersion1(t *testing.T) {
	if _, err := IsccDecode("ISCC:GAIAAAAAAA"); err == nil {
		t.Error("IsccDecode accepted Data-Code Version 1")
	}
	if _, err := IsccDecompose("ISCC:GAIAAAAAAA"); err == nil {
		t.Error("IsccDecompose accepted Data-Code Version 1")
	}
}

func TestTypedCodecRejectsUnknownSubtype(t *testing.T) {
	units := mustDecodeSeq(t, append(hx(t, "280070"), seqBody(32)...))
	if _, err := IsccDecode(units[0]); err == nil {
		t.Error("IsccDecode accepted SubType 8")
	}
	if _, err := IsccDecompose(units[0]); err == nil {
		t.Error("IsccDecompose accepted SubType 8")
	}
}

// ---- asciiUpper ----

// asciiUpperSink keeps asciiUpper results alive so the compiler cannot elide a copy.
var asciiUpperSink string

func TestASCIIUpperNoAllocWhenUppercase(t *testing.T) {
	allocs := testing.AllocsPerRun(100, func() { asciiUpperSink = asciiUpper("GAAWDLQNG6SSDPF2") })
	if allocs != 0 {
		t.Fatalf("asciiUpper allocated %v times on an uppercase input", allocs)
	}
}

func TestASCIIUpperLowercaseInput(t *testing.T) {
	if got := asciiUpper("Gaaw-dlqng6ſsdpf2"); got != "GAAW-DLQNG6ſSDPF2" {
		t.Fatalf("got %q", got)
	}
}
