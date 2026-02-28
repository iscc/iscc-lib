// Pure Go implementation of the ISCC codec: type enums, varnibble header
// encoding/decoding, base32/base64, component encoding, ISCC decompose/decode.
// This module has zero external dependencies — it uses only the Go standard library.
package iscc

import (
	"encoding/base32"
	"encoding/base64"
	"fmt"
	"strings"
)

// ---- Algorithm Configuration Constants ----

// Algorithm configuration constants matching iscc-core core_opts.
const (
	MetaTrimName        = 128
	MetaTrimDescription = 4096
	IoReadSize          = 4_194_304
	TextNgramSize       = 13
)

// ---- Type Enums (Go typed constants) ----

// MainType identifies the primary ISCC unit type.
type MainType uint8

const (
	MTMeta     MainType = iota // 0
	MTSemantic                 // 1
	MTContent                  // 2
	MTData                     // 3
	MTInstance                 // 4
	MTIscc                     // 5
	MTId                       // 6
	MTFlake                    // 7
)

// SubType identifies the ISCC sub-type (interpretation depends on MainType context).
type SubType uint8

const (
	STNone     SubType = iota // 0 (also TEXT in Content-Code context)
	STImage                   // 1
	STAudio                   // 2
	STVideo                   // 3
	STMixed                   // 4
	STSum                     // 5
	STIsccNone                // 6
	STWide                    // 7
)

// STText is an alias for STNone (value 0) in Content-Code / Semantic-Code context.
const STText = STNone

// Version identifies the ISCC algorithm version.
type Version uint8

// VSV0 is ISCC version 0.
const VSV0 Version = 0

// ---- Bit Manipulation Helpers (unexported) ----

// getBit reads the bit at position bitPos from data (MSB-first ordering).
func getBit(data []byte, bitPos int) bool {
	byteIdx := bitPos / 8
	bitIdx := 7 - (bitPos % 8)
	return (data[byteIdx]>>uint(bitIdx))&1 == 1
}

// extractBits extracts count bits starting at bitPos as a uint32 (MSB-first).
func extractBits(data []byte, bitPos, count int) uint32 {
	var value uint32
	for i := 0; i < count; i++ {
		value = (value << 1)
		if getBit(data, bitPos+i) {
			value |= 1
		}
	}
	return value
}

// bitsToBytes converts a bit slice to bytes, padding with zero bits on the right.
func bitsToBytes(bits []bool) []byte {
	nBytes := (len(bits) + 7) / 8
	result := make([]byte, nBytes)
	for i, b := range bits {
		if b {
			byteIdx := i / 8
			bitIdx := 7 - (i % 8)
			result[byteIdx] |= 1 << uint(bitIdx)
		}
	}
	return result
}

// ---- Varnibble Encoding (unexported) ----

// encodeVarnibble encodes an integer as a variable-length nibble bit sequence.
//
// Encoding scheme:
//   - 0xxx (4 bits): values 0–7
//   - 10xxxxxx (8 bits): values 8–71
//   - 110xxxxxxxxx (12 bits): values 72–583
//   - 1110xxxxxxxxxxxx (16 bits): values 584–4679
func encodeVarnibble(value uint32) ([]bool, error) {
	switch {
	case value <= 7:
		bits := make([]bool, 4)
		for i := 0; i < 4; i++ {
			bits[i] = (value>>(3-uint(i)))&1 == 1
		}
		return bits, nil
	case value <= 71:
		v := value - 8
		bits := make([]bool, 8)
		bits[0] = true  // prefix: 10
		bits[1] = false //
		for i := 0; i < 6; i++ {
			bits[2+i] = (v>>(5-uint(i)))&1 == 1
		}
		return bits, nil
	case value <= 583:
		v := value - 72
		bits := make([]bool, 12)
		bits[0] = true  // prefix: 110
		bits[1] = true  //
		bits[2] = false //
		for i := 0; i < 9; i++ {
			bits[3+i] = (v>>(8-uint(i)))&1 == 1
		}
		return bits, nil
	case value <= 4679:
		v := value - 584
		bits := make([]bool, 16)
		bits[0] = true  // prefix: 1110
		bits[1] = true  //
		bits[2] = true  //
		bits[3] = false //
		for i := 0; i < 12; i++ {
			bits[4+i] = (v>>(11-uint(i)))&1 == 1
		}
		return bits, nil
	default:
		return nil, fmt.Errorf("iscc: varnibble value out of range (0-4679): %d", value)
	}
}

// decodeVarnibbleFromBytes decodes the first varnibble from data at the given bit position.
// Returns the decoded value and the number of bits consumed.
func decodeVarnibbleFromBytes(data []byte, bitPos int) (uint32, int, error) {
	available := len(data)*8 - bitPos
	if available < 4 {
		return 0, 0, fmt.Errorf("iscc: insufficient bits for varnibble")
	}

	if !getBit(data, bitPos) {
		// 0xxx — 4 bits, values 0–7
		return extractBits(data, bitPos, 4), 4, nil
	}
	if available >= 8 && !getBit(data, bitPos+1) {
		// 10xxxxxx — 8 bits, values 8–71
		return extractBits(data, bitPos+2, 6) + 8, 8, nil
	}
	if available >= 12 && !getBit(data, bitPos+2) {
		// 110xxxxxxxxx — 12 bits, values 72–583
		return extractBits(data, bitPos+3, 9) + 72, 12, nil
	}
	if available >= 16 && !getBit(data, bitPos+3) {
		// 1110xxxxxxxxxxxx — 16 bits, values 584–4679
		return extractBits(data, bitPos+4, 12) + 584, 16, nil
	}
	return 0, 0, fmt.Errorf("iscc: invalid varnibble prefix or insufficient bits")
}

// ---- Header Encoding (unexported) ----

// encodeHeader encodes ISCC header fields into bytes.
// Concatenates varnibble-encoded MainType, SubType, Version, and length,
// then pads to byte boundary with zero bits on the right.
func encodeHeader(mtype MainType, stype SubType, version Version, length uint32) ([]byte, error) {
	var bits []bool

	mtBits, err := encodeVarnibble(uint32(mtype))
	if err != nil {
		return nil, err
	}
	bits = append(bits, mtBits...)

	stBits, err := encodeVarnibble(uint32(stype))
	if err != nil {
		return nil, err
	}
	bits = append(bits, stBits...)

	vsBits, err := encodeVarnibble(uint32(version))
	if err != nil {
		return nil, err
	}
	bits = append(bits, vsBits...)

	lnBits, err := encodeVarnibble(length)
	if err != nil {
		return nil, err
	}
	bits = append(bits, lnBits...)

	// Pad to byte boundary with zero bits
	remainder := len(bits) % 8
	if remainder != 0 {
		padding := make([]bool, 8-remainder)
		bits = append(bits, padding...)
	}

	return bitsToBytes(bits), nil
}

// decodeHeader decodes ISCC header from bytes.
// Returns (MainType, SubType, Version, length, tail) where tail contains
// remaining data after the header.
func decodeHeader(data []byte) (MainType, SubType, Version, uint32, []byte, error) {
	bitPos := 0

	mtypeVal, consumed, err := decodeVarnibbleFromBytes(data, bitPos)
	if err != nil {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: decode header maintype: %w", err)
	}
	bitPos += consumed

	stypeVal, consumed, err := decodeVarnibbleFromBytes(data, bitPos)
	if err != nil {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: decode header subtype: %w", err)
	}
	bitPos += consumed

	versionVal, consumed, err := decodeVarnibbleFromBytes(data, bitPos)
	if err != nil {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: decode header version: %w", err)
	}
	bitPos += consumed

	length, consumed, err := decodeVarnibbleFromBytes(data, bitPos)
	if err != nil {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: decode header length: %w", err)
	}
	bitPos += consumed

	// Strip 4-bit zero padding if header bits are not byte-aligned
	if bitPos%8 != 0 && bitPos+4 <= len(data)*8 && extractBits(data, bitPos, 4) == 0 {
		bitPos += 4
	}

	// Advance to next byte boundary for tail extraction
	tailByteStart := (bitPos + 7) / 8
	var tail []byte
	if tailByteStart < len(data) {
		tail = make([]byte, len(data)-tailByteStart)
		copy(tail, data[tailByteStart:])
	}

	if mtypeVal > 7 {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: invalid MainType: %d", mtypeVal)
	}
	if stypeVal > 7 {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: invalid SubType: %d", stypeVal)
	}
	if versionVal > 0 {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: invalid Version: %d", versionVal)
	}

	return MainType(mtypeVal), SubType(stypeVal), Version(versionVal), length, tail, nil
}

// ---- Length Encoding (unexported) ----

// encodeLength encodes bit length to header length field value.
//
// Semantics depend on MainType:
//   - META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: (bitLength / 32) - 1
//   - ISCC: pass-through (0–7, unit composition flags)
//   - ID: (bitLength - 64) / 8
func encodeLength(mtype MainType, length uint32) (uint32, error) {
	switch mtype {
	case MTMeta, MTSemantic, MTContent, MTData, MTInstance, MTFlake:
		if length >= 32 && length%32 == 0 {
			return length/32 - 1, nil
		}
		return 0, fmt.Errorf("iscc: invalid length %d for MainType %d (must be multiple of 32, >= 32)", length, mtype)
	case MTIscc:
		if length <= 7 {
			return length, nil
		}
		return 0, fmt.Errorf("iscc: invalid length %d for ISCC (must be 0-7)", length)
	case MTId:
		if length >= 64 && length <= 96 && (length-64)%8 == 0 {
			return (length - 64) / 8, nil
		}
		return 0, fmt.Errorf("iscc: invalid length %d for ID (must be 64-96, step 8)", length)
	default:
		return 0, fmt.Errorf("iscc: unknown MainType: %d", mtype)
	}
}

// decodeLength decodes header length field to actual bit length.
//
// Inverse of encodeLength. Returns the number of bits in the digest.
//   - META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: (length + 1) * 32
//   - ISCC + Wide: 256
//   - ISCC + other: popcount(length) * 64 + 128
//   - ID: length * 8 + 64
func decodeLength(mtype MainType, length uint32, stype SubType) uint32 {
	switch mtype {
	case MTMeta, MTSemantic, MTContent, MTData, MTInstance, MTFlake:
		return (length + 1) * 32
	case MTIscc:
		if stype == STWide {
			return 256
		}
		return popcount(length)*64 + 128
	case MTId:
		return length*8 + 64
	default:
		return 0
	}
}

// popcount returns the number of 1-bits in v.
func popcount(v uint32) uint32 {
	var count uint32
	for v != 0 {
		count += v & 1
		v >>= 1
	}
	return count
}

// ---- Unit Encoding (unexported) ----

// encodeUnits encodes optional ISCC-UNIT MainTypes as a unit combination index (0–7).
// Maps optional units (Meta, Semantic, Content) to a bitfield:
// bit 0 = Content, bit 1 = Semantic, bit 2 = Meta.
func encodeUnits(mainTypes []MainType) (uint32, error) {
	var result uint32
	for _, mt := range mainTypes {
		switch mt {
		case MTContent:
			result |= 1
		case MTSemantic:
			result |= 2
		case MTMeta:
			result |= 4
		default:
			return 0, fmt.Errorf("iscc: %d is not a valid optional unit type", mt)
		}
	}
	return result, nil
}

// decodeUnits decodes a unit combination index (0–7) to a sorted list of optional MainTypes.
// Inverse of encodeUnits. Results in MainType order: Meta, Semantic, Content.
func decodeUnits(unitID uint32) ([]MainType, error) {
	if unitID > 7 {
		return nil, fmt.Errorf("iscc: invalid unit_id: %d (must be 0-7)", unitID)
	}
	var result []MainType
	if unitID&4 != 0 {
		result = append(result, MTMeta)
	}
	if unitID&2 != 0 {
		result = append(result, MTSemantic)
	}
	if unitID&1 != 0 {
		result = append(result, MTContent)
	}
	return result, nil
}

// ---- Base32/Base64 Encoding (unexported helpers + public functions) ----

// b32Encoding is the RFC 4648 base32 encoder with no padding.
var b32Encoding = base32.StdEncoding.WithPadding(base32.NoPadding)

// b64Encoding is the RFC 4648 §5 URL-safe base64 encoder with no padding.
var b64Encoding = base64.RawURLEncoding

// encodeBase32 encodes bytes as base32 (RFC 4648, uppercase, no padding).
func encodeBase32(data []byte) string {
	return b32Encoding.EncodeToString(data)
}

// decodeBase32 decodes a base32 string to bytes (case-insensitive, no padding).
func decodeBase32(code string) ([]byte, error) {
	upper := strings.ToUpper(code)
	decoded, err := b32Encoding.DecodeString(upper)
	if err != nil {
		return nil, fmt.Errorf("iscc: base32 decode error: %w", err)
	}
	return decoded, nil
}

// EncodeBase64 encodes bytes as base64url (RFC 4648 §5, no padding).
func EncodeBase64(data []byte) string {
	return b64Encoding.EncodeToString(data)
}

// JsonToDataUrl converts a JSON string to a data-URL with base64-encoded canonical JSON.
//
// Parses and canonicalizes the JSON (sorted keys, compact format). If the JSON
// contains an "@context" key, uses "application/ld+json" media type; otherwise
// uses "application/json".
func JsonToDataUrl(jsonStr string) (string, error) {
	canonical, err := parseMetaJSON(jsonStr)
	if err != nil {
		return "", err
	}
	hasContext := jsonHasContext(jsonStr)
	return buildMetaDataURL(canonical, hasContext), nil
}

// ---- Component Encoding (public) ----

// EncodeComponent encodes an ISCC-UNIT with header and body as a base32 string.
// Produces the base32-encoded string (without "ISCC:" prefix).
// ISCC MainType (MTIscc) is rejected — use gen_iscc_code_v0 instead.
func EncodeComponent(mtype, stype uint8, version uint8, bitLength uint32, digest []byte) (string, error) {
	mt := MainType(mtype)
	st := SubType(stype)
	vs := Version(version)

	if mt > MTFlake {
		return "", fmt.Errorf("iscc: invalid MainType: %d", mtype)
	}
	if st > STWide {
		return "", fmt.Errorf("iscc: invalid SubType: %d", stype)
	}
	if vs > VSV0 {
		return "", fmt.Errorf("iscc: invalid Version: %d", version)
	}
	if mt == MTIscc {
		return "", fmt.Errorf("iscc: ISCC MainType is not a unit; use gen_iscc_code_v0 instead")
	}

	encodedLength, err := encodeLength(mt, bitLength)
	if err != nil {
		return "", err
	}
	nbytes := int(bitLength / 8)
	header, err := encodeHeader(mt, st, vs, encodedLength)
	if err != nil {
		return "", err
	}

	bodyLen := nbytes
	if bodyLen > len(digest) {
		bodyLen = len(digest)
	}

	component := make([]byte, 0, len(header)+bodyLen)
	component = append(component, header...)
	component = append(component, digest[:bodyLen]...)

	return encodeBase32(component), nil
}

// encodeComponentInternal is the internal version that accepts typed enums directly.
func encodeComponentInternal(mtype MainType, stype SubType, version Version, bitLength uint32, digest []byte) (string, error) {
	return EncodeComponent(uint8(mtype), uint8(stype), uint8(version), bitLength, digest)
}

// ---- ISCC Decompose (public) ----

// IsccDecompose decomposes a composite ISCC-CODE or ISCC sequence into individual ISCC-UNITs.
// The optional "ISCC:" prefix is stripped before decoding. Returns a list of
// base32-encoded ISCC-UNIT strings (without "ISCC:" prefix).
func IsccDecompose(isccCode string) ([]string, error) {
	clean := strings.TrimPrefix(isccCode, "ISCC:")
	rawCode, err := decodeBase32(clean)
	if err != nil {
		return nil, err
	}

	var components []string
	for len(rawCode) > 0 {
		mt, st, vs, ln, body, err := decodeHeader(rawCode)
		if err != nil {
			return nil, err
		}

		// Standard ISCC-UNIT with tail continuation
		if mt != MTIscc {
			lnBits := decodeLength(mt, ln, st)
			nbytes := int(lnBits / 8)
			if len(body) < nbytes {
				return nil, fmt.Errorf("iscc: truncated ISCC body: expected %d bytes, got %d", nbytes, len(body))
			}
			code, err := encodeComponentInternal(mt, st, vs, lnBits, body[:nbytes])
			if err != nil {
				return nil, err
			}
			components = append(components, code)
			rawCode = body[nbytes:]
			continue
		}

		// ISCC-CODE: decode into constituent units
		mainTypes, err := decodeUnits(ln)
		if err != nil {
			return nil, err
		}

		// Wide mode: 128-bit Data-Code + 128-bit Instance-Code
		if st == STWide {
			if len(body) < 32 {
				return nil, fmt.Errorf("iscc: truncated ISCC body: expected 32 bytes, got %d", len(body))
			}
			dataCode, err := encodeComponentInternal(MTData, STNone, vs, 128, body[:16])
			if err != nil {
				return nil, err
			}
			instanceCode, err := encodeComponentInternal(MTInstance, STNone, vs, 128, body[16:32])
			if err != nil {
				return nil, err
			}
			components = append(components, dataCode, instanceCode)
			break
		}

		// Non-wide ISCC-CODE: total body = dynamic units × 8 + Data 8 + Instance 8
		expectedBody := len(mainTypes)*8 + 16
		if len(body) < expectedBody {
			return nil, fmt.Errorf("iscc: truncated ISCC body: expected %d bytes, got %d", expectedBody, len(body))
		}

		// Rebuild dynamic units (Meta, Semantic, Content)
		for idx, mtype := range mainTypes {
			unitStype := st
			if mtype == MTMeta {
				unitStype = STNone
			}
			code, err := encodeComponentInternal(mtype, unitStype, vs, 64, body[idx*8:])
			if err != nil {
				return nil, err
			}
			components = append(components, code)
		}

		// Rebuild static units (Data-Code, Instance-Code)
		dataCode, err := encodeComponentInternal(MTData, STNone, vs, 64, body[len(body)-16:len(body)-8])
		if err != nil {
			return nil, err
		}
		instanceCode, err := encodeComponentInternal(MTInstance, STNone, vs, 64, body[len(body)-8:])
		if err != nil {
			return nil, err
		}
		components = append(components, dataCode, instanceCode)
		break
	}

	return components, nil
}

// ---- ISCC Decode (public) ----

// DecodeResult holds the decoded header components and raw digest of an ISCC unit.
type DecodeResult struct {
	Maintype uint8
	Subtype  uint8
	Version  uint8
	Length   uint8
	Digest   []byte
}

// IsccDecode decodes an ISCC string into its header components and raw digest.
// Strips optional "ISCC:" prefix and dashes.
func IsccDecode(iscc string) (*DecodeResult, error) {
	clean := strings.TrimPrefix(iscc, "ISCC:")
	clean = strings.ReplaceAll(clean, "-", "")
	raw, err := decodeBase32(clean)
	if err != nil {
		return nil, err
	}
	mt, st, vs, lengthIndex, tail, err := decodeHeader(raw)
	if err != nil {
		return nil, err
	}
	bitLength := decodeLength(mt, lengthIndex, st)
	nbytes := int(bitLength / 8)
	if len(tail) < nbytes {
		return nil, fmt.Errorf("iscc: decoded body too short: expected %d digest bytes, got %d", nbytes, len(tail))
	}
	digest := make([]byte, nbytes)
	copy(digest, tail[:nbytes])

	return &DecodeResult{
		Maintype: uint8(mt),
		Subtype:  uint8(st),
		Version:  uint8(vs),
		Length:   uint8(lengthIndex),
		Digest:   digest,
	}, nil
}
