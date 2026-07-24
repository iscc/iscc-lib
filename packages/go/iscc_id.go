// Pure Go implementation of experimental ISCC-IDv1 encoding and decoding.
// An ISCC-IDv1 is an 80-bit identifier (16-bit header + 64-bit body) whose body
// packs a 52-bit microsecond timestamp and a 12-bit HUB-ID, matching the Python
// reference in iscc-core's iscc_id.py (gen_iscc_id_v1).
package iscc

import (
	"encoding/binary"
	"fmt"
)

// IsccIDv1Result holds the structured fields of a decoded ISCC-IDv1.
//
// Experimental: ISCC-IDv1 is not part of ISO 24138 and may change in a minor release.
type IsccIDv1Result struct {
	Realm     uint8  // Realm-ID: 0 = test, 1 = operational
	HubID     uint16 // ID of the issuing ISCC-HUB (0-4095)
	Timestamp uint64 // Microseconds since 1970-01-01T00:00:00Z (52-bit)
}

// EncodeIsccID encodes realm, hub ID, and timestamp as an ISCC-IDv1 string.
//
// The 64-bit body packs the 52-bit microsecond timestamp in the high bits and
// the 12-bit HUB-ID in the low 12 bits. The returned string carries the "ISCC:"
// prefix. Valid inputs: realm 0 (test) or 1 (operational), hubID 0-4095,
// timestamp < 2^52.
//
// Experimental: ISCC-IDv1 is not part of ISO 24138 and may change in a minor release.
func EncodeIsccID(realm uint8, hubID uint16, timestamp uint64) (string, error) {
	if timestamp >= 1<<52 {
		return "", fmt.Errorf("iscc: timestamp overflow: %d (must be < 2^52)", timestamp)
	}
	if hubID >= 1<<12 {
		return "", fmt.Errorf("iscc: HUB-ID overflow: %d (must be 0-4095)", hubID)
	}
	if realm > 1 {
		return "", fmt.Errorf("iscc: Realm-ID must be 0 (test) or 1 (operational), got %d", realm)
	}

	body := (timestamp << 12) | uint64(hubID)
	var digest [8]byte
	binary.BigEndian.PutUint64(digest[:], body)

	encodedLen, err := encodeLength(MTId, 64)
	if err != nil {
		return "", err
	}
	header, err := encodeHeader(MTId, SubType(realm), VSV1, encodedLen)
	if err != nil {
		return "", err
	}

	component := make([]byte, 0, len(header)+len(digest))
	component = append(component, header...)
	component = append(component, digest[:]...)

	return "ISCC:" + encodeBase32(component), nil
}

// DecodeIsccID decodes an ISCC-IDv1 string into realm, hub ID, and timestamp.
// The optional "ISCC:" prefix and dashes are stripped before decoding.
//
// Experimental: ISCC-IDv1 is not part of ISO 24138 and may change in a minor release.
func DecodeIsccID(code string) (*IsccIDv1Result, error) {
	decoded, err := IsccDecode(code)
	if err != nil {
		return nil, err
	}
	if MainType(decoded.Maintype) != MTId {
		return nil, fmt.Errorf("iscc: MainType %d is not ISCC-ID", decoded.Maintype)
	}
	if Version(decoded.Version) != VSV1 {
		return nil, fmt.Errorf("iscc: unsupported ISCC-ID version: %d (expected 1)", decoded.Version)
	}
	if len(decoded.Digest) != 8 {
		return nil, fmt.Errorf("iscc: invalid ISCC-IDv1 digest length: %d (expected 8)", len(decoded.Digest))
	}

	body := binary.BigEndian.Uint64(decoded.Digest)
	return &IsccIDv1Result{
		Realm:     decoded.Subtype,
		HubID:     uint16(body & 0xFFF),
		Timestamp: body >> 12,
	}, nil
}
