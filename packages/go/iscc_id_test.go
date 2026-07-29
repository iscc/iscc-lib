// Tests for experimental ISCC-IDv1 minting (iscc_id.go).
// The known vector ISCC:MAIGHFECJMOPMIAB -> realm 0, hub_id 1,
// timestamp 1751831876325218 comes from iscc-hub's schema.py example.
package iscc

import (
	"encoding/binary"
	"strings"
	"testing"
)

const (
	knownIsccID    = "ISCC:MAIGHFECJMOPMIAB"
	knownRealm     = uint8(0)
	knownHubID     = uint16(1)
	knownTimestamp = uint64(1751831876325218)
)

// extractIsccIDv1 unpacks an ISCC-IDv1 string into its (timestamp, hubID, realm)
// fields via the generic IsccDecode path — the documented field-extraction recipe.
func extractIsccIDv1(t *testing.T, code string) (timestamp uint64, hubID uint16, realm uint8) {
	t.Helper()
	d, err := IsccDecode(code)
	if err != nil {
		t.Fatalf("IsccDecode(%q): %v", code, err)
	}
	if len(d.Digest) != 8 {
		t.Fatalf("digest length: got %d, want 8", len(d.Digest))
	}
	n := binary.BigEndian.Uint64(d.Digest)
	return n >> 12, uint16(n & 0xFFF), d.Subtype
}

func TestGenIsccIDV1KnownVector(t *testing.T) {
	got, err := GenIsccIDV1(knownTimestamp, knownHubID, knownRealm)
	if err != nil {
		t.Fatalf("GenIsccIDV1: %v", err)
	}
	if got.ISCC != knownIsccID {
		t.Errorf("got %q, want %q", got.ISCC, knownIsccID)
	}
}

func TestGenIsccIDV1RoundTripBoundaries(t *testing.T) {
	maxTimestamp := uint64(1<<52 - 1)
	for _, realm := range []uint8{0, 1} {
		for _, hubID := range []uint16{0, 4095} {
			for _, timestamp := range []uint64{0, knownTimestamp, maxTimestamp} {
				result, err := GenIsccIDV1(timestamp, hubID, realm)
				if err != nil {
					t.Fatalf("GenIsccIDV1(%d, %d, %d): %v", timestamp, hubID, realm, err)
				}
				gotTS, gotHub, gotRealm := extractIsccIDv1(t, result.ISCC)
				if gotTS != timestamp || gotHub != hubID || gotRealm != realm {
					t.Errorf("round-trip mismatch for (%d, %d, %d): got (%d, %d, %d)",
						timestamp, hubID, realm, gotTS, gotHub, gotRealm)
				}
			}
		}
	}
}

func TestGenIsccIDV1KnownVectorExtraction(t *testing.T) {
	for _, code := range []string{knownIsccID, strings.TrimPrefix(knownIsccID, "ISCC:")} {
		gotTS, gotHub, gotRealm := extractIsccIDv1(t, code)
		if gotTS != knownTimestamp {
			t.Errorf("timestamp: got %d, want %d", gotTS, knownTimestamp)
		}
		if gotHub != knownHubID {
			t.Errorf("hubID: got %d, want %d", gotHub, knownHubID)
		}
		if gotRealm != knownRealm {
			t.Errorf("realm: got %d, want %d", gotRealm, knownRealm)
		}
	}
}

func TestIsccDecodeAcceptsIDv1(t *testing.T) {
	result, err := IsccDecode(knownIsccID)
	if err != nil {
		t.Fatalf("IsccDecode(%q): %v", knownIsccID, err)
	}
	if result.Maintype != uint8(MTId) {
		t.Errorf("Maintype: got %d, want %d", result.Maintype, MTId)
	}
	if result.Version != uint8(VSV1) {
		t.Errorf("Version: got %d, want %d", result.Version, VSV1)
	}
	if len(result.Digest) != 8 {
		t.Errorf("Digest length: got %d, want 8", len(result.Digest))
	}
}

func TestIsccDecomposeRoundTripsIDv1(t *testing.T) {
	// Decomposition re-encodes each unit via EncodeComponent, which must accept
	// Version 1 for MainType ID or an ISCC-IDv1 fails to survive the round trip.
	result, err := GenIsccIDV1(knownTimestamp, knownHubID, knownRealm)
	if err != nil {
		t.Fatalf("GenIsccIDV1: %v", err)
	}
	units, err := IsccDecompose(result.ISCC)
	if err != nil {
		t.Fatalf("IsccDecompose(%q): %v", result.ISCC, err)
	}
	if len(units) != 1 {
		t.Fatalf("unit count: got %d, want 1", len(units))
	}
	if got := "ISCC:" + units[0]; got != result.ISCC {
		t.Errorf("round-trip: got %q, want %q", got, result.ISCC)
	}
}

func TestEncodeComponentRejectsVersion1ForNonID(t *testing.T) {
	// The encode side mirrors decodeHeader: Version 1 is exclusive to MainType ID.
	_, err := EncodeComponent(uint8(MTData), uint8(STNone), uint8(VSV1), 64, make([]byte, 8))
	if err == nil {
		t.Fatal("expected error for Version=1 with MainType Data, got nil")
	}
	if !strings.Contains(err.Error(), "invalid Version") {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestDecodeHeaderRejectsVersion1ForNonID(t *testing.T) {
	// Construct a Data-Code-shaped header with Version=1 (64-bit length index 1).
	header, err := encodeHeader(MTData, STNone, VSV1, 1)
	if err != nil {
		t.Fatalf("encodeHeader: %v", err)
	}
	data := append(header, make([]byte, 8)...)
	_, _, _, _, _, err = decodeHeader(data)
	if err == nil {
		t.Fatal("expected error for Version=1 with MainType Data, got nil")
	}
	if !strings.Contains(err.Error(), "invalid Version") {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestDecodeHeaderRejectsVersion2ForID(t *testing.T) {
	// Version acceptance for MainType ID is exact (only Version=1 beyond Version=0).
	header, err := encodeHeader(MTId, STNone, Version(2), 0)
	if err != nil {
		t.Fatalf("encodeHeader: %v", err)
	}
	data := append(header, make([]byte, 8)...)
	_, _, _, _, _, err = decodeHeader(data)
	if err == nil {
		t.Fatal("expected error for Version=2 with MainType ID, got nil")
	}
}

func TestGenIsccIDV1Validation(t *testing.T) {
	cases := []struct {
		name      string
		timestamp uint64
		hubID     uint16
		realm     uint8
	}{
		{"timestamp overflow", 1 << 52, 0, 0},
		{"hub-id overflow", 0, 4096, 0},
		{"invalid realm", 0, 0, 2},
	}
	for _, tc := range cases {
		_, err := GenIsccIDV1(tc.timestamp, tc.hubID, tc.realm)
		if err == nil {
			t.Errorf("%s: expected error, got nil", tc.name)
			continue
		}
		if !strings.HasPrefix(err.Error(), "iscc:") {
			t.Errorf("%s: error not iscc-prefixed: %v", tc.name, err)
		}
	}
}
