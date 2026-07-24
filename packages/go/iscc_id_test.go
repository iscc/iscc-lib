// Tests for experimental ISCC-IDv1 encoding and decoding (iscc_id.go).
// The known vector ISCC:MAIGHFECJMOPMIAB -> realm 0, hub_id 1,
// timestamp 1751831876325218 comes from iscc-hub's schema.py example.
package iscc

import (
	"strings"
	"testing"
)

const (
	knownIsccID    = "ISCC:MAIGHFECJMOPMIAB"
	knownRealm     = uint8(0)
	knownHubID     = uint16(1)
	knownTimestamp = uint64(1751831876325218)
)

func TestEncodeIsccIDKnownVector(t *testing.T) {
	got, err := EncodeIsccID(knownRealm, knownHubID, knownTimestamp)
	if err != nil {
		t.Fatalf("EncodeIsccID: %v", err)
	}
	if got != knownIsccID {
		t.Errorf("got %q, want %q", got, knownIsccID)
	}
}

func TestDecodeIsccIDKnownVector(t *testing.T) {
	for _, code := range []string{knownIsccID, strings.TrimPrefix(knownIsccID, "ISCC:")} {
		result, err := DecodeIsccID(code)
		if err != nil {
			t.Fatalf("DecodeIsccID(%q): %v", code, err)
		}
		if result.Realm != knownRealm {
			t.Errorf("Realm: got %d, want %d", result.Realm, knownRealm)
		}
		if result.HubID != knownHubID {
			t.Errorf("HubID: got %d, want %d", result.HubID, knownHubID)
		}
		if result.Timestamp != knownTimestamp {
			t.Errorf("Timestamp: got %d, want %d", result.Timestamp, knownTimestamp)
		}
	}
}

func TestIsccIDRoundTripBoundaries(t *testing.T) {
	maxTimestamp := uint64(1<<52 - 1)
	for _, realm := range []uint8{0, 1} {
		for _, hubID := range []uint16{0, 4095} {
			for _, timestamp := range []uint64{0, knownTimestamp, maxTimestamp} {
				code, err := EncodeIsccID(realm, hubID, timestamp)
				if err != nil {
					t.Fatalf("EncodeIsccID(%d, %d, %d): %v", realm, hubID, timestamp, err)
				}
				result, err := DecodeIsccID(code)
				if err != nil {
					t.Fatalf("DecodeIsccID(%q): %v", code, err)
				}
				if result.Realm != realm || result.HubID != hubID || result.Timestamp != timestamp {
					t.Errorf("round-trip mismatch for (%d, %d, %d): got (%d, %d, %d)",
						realm, hubID, timestamp, result.Realm, result.HubID, result.Timestamp)
				}
			}
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

func TestEncodeIsccIDValidation(t *testing.T) {
	cases := []struct {
		name      string
		realm     uint8
		hubID     uint16
		timestamp uint64
	}{
		{"timestamp overflow", 0, 0, 1 << 52},
		{"hub-id overflow", 0, 4096, 0},
		{"invalid realm", 2, 0, 0},
	}
	for _, tc := range cases {
		_, err := EncodeIsccID(tc.realm, tc.hubID, tc.timestamp)
		if err == nil {
			t.Errorf("%s: expected error, got nil", tc.name)
			continue
		}
		if !strings.HasPrefix(err.Error(), "iscc:") {
			t.Errorf("%s: error not iscc-prefixed: %v", tc.name, err)
		}
	}
}

func TestDecodeIsccIDRejectsNonID(t *testing.T) {
	// A Data-Code unit is not an ISCC-ID.
	dataCode, err := EncodeComponent(uint8(MTData), uint8(STNone), 0, 64, make([]byte, 8))
	if err != nil {
		t.Fatalf("EncodeComponent: %v", err)
	}
	_, err = DecodeIsccID(dataCode)
	if err == nil {
		t.Fatal("expected error for non-ID MainType, got nil")
	}
	if !strings.Contains(err.Error(), "not ISCC-ID") {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestDecodeIsccIDRejectsTrailingBytes(t *testing.T) {
	// Trailing-byte rejection in IsccDecode propagates through the delegation.
	_, err := DecodeIsccID(knownIsccID + "AA")
	if err == nil {
		t.Fatal("expected error for trailing bytes, got nil")
	}
	// Control: the canonical form still decodes to the known fields.
	result, err := DecodeIsccID(knownIsccID)
	if err != nil {
		t.Fatalf("DecodeIsccID(%q): %v", knownIsccID, err)
	}
	if result.Realm != knownRealm || result.HubID != knownHubID || result.Timestamp != knownTimestamp {
		t.Errorf("got (%d, %d, %d), want (%d, %d, %d)",
			result.Realm, result.HubID, result.Timestamp, knownRealm, knownHubID, knownTimestamp)
	}
}
