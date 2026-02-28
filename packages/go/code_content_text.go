// Pure Go implementation of ISCC Text-Code (Content-Code for text).
// Generates a similarity-preserving fingerprint from text content using
// character n-gram MinHash. Matches the Rust gen_text_code_v0 implementation.
package iscc

// TextCodeResult holds the output of GenTextCodeV0.
type TextCodeResult struct {
	Iscc       string // ISCC code string with "ISCC:" prefix
	Characters int    // Number of characters after collapsing
}

// GenTextCodeV0 generates an ISCC Content-Code for text content.
// Collapses the input text, computes a MinHash-based similarity digest
// from character n-grams, and encodes as an ISCC component.
func GenTextCodeV0(text string, bits uint32) (*TextCodeResult, error) {
	collapsed := TextCollapse(text)
	characters := len([]rune(collapsed))
	hashDigest := softHashTextV0(collapsed)
	component, err := EncodeComponent(
		uint8(MTContent), uint8(STText), uint8(VSV0), bits, hashDigest,
	)
	if err != nil {
		return nil, err
	}
	return &TextCodeResult{
		Iscc:       "ISCC:" + component,
		Characters: characters,
	}, nil
}

// softHashTextV0 computes a 256-bit similarity-preserving hash from collapsed text.
// Generates width-13 character n-grams, hashes each with xxh32 (seed 0),
// then applies MinHash to produce a 32-byte digest.
func softHashTextV0(text string) []byte {
	ngrams, _ := SlidingWindow(text, TextNgramSize)
	features := make([]uint32, len(ngrams))
	for i, ng := range ngrams {
		features[i] = xxh32([]byte(ng), 0)
	}
	return AlgMinhash256(features)
}
