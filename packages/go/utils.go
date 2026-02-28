// Pure Go text utility functions for ISCC code generation.
// Provides text cleaning, trimming, collapsing, and newline removal
// ported from the Rust implementation in crates/iscc-lib/src/utils.rs.
package iscc

import (
	"strings"
	"unicode"
	"unicode/utf8"

	"golang.org/x/text/unicode/norm"
)

// newlineSet contains characters treated as newlines (preserved during control-char removal).
var newlineSet = map[rune]bool{
	'\u000A': true, // LINE FEED
	'\u000B': true, // VERTICAL TAB
	'\u000C': true, // FORM FEED
	'\u000D': true, // CARRIAGE RETURN
	'\u0085': true, // NEXT LINE
	'\u2028': true, // LINE SEPARATOR
	'\u2029': true, // PARAGRAPH SEPARATOR
}

// isCCategory checks if a rune belongs to a Unicode "C" (control/format/etc) category.
func isCCategory(c rune) bool {
	return unicode.Is(unicode.C, c)
}

// isCMPCategory checks if a rune belongs to Unicode "C", "M", or "P" categories.
func isCMPCategory(c rune) bool {
	return unicode.Is(unicode.C, c) || unicode.Is(unicode.M, c) || unicode.Is(unicode.P, c)
}

// TextClean normalizes text for display.
//
// Applies NFKC normalization, removes control characters (except newlines),
// normalizes \r\n to \n, collapses consecutive empty lines to at most one,
// and strips leading/trailing whitespace.
func TextClean(text string) string {
	// 1. NFKC normalize
	text = norm.NFKC.String(text)

	// 2. Remove control chars except newlines, normalizing all newlines to \n
	var cleaned strings.Builder
	cleaned.Grow(len(text))
	runes := []rune(text)
	for i := 0; i < len(runes); i++ {
		c := runes[i]
		if newlineSet[c] {
			// Handle \r\n as a single newline
			if c == '\r' && i+1 < len(runes) && runes[i+1] == '\n' {
				i++
			}
			cleaned.WriteRune('\n')
		} else if isCCategory(c) {
			// Skip control characters
		} else {
			cleaned.WriteRune(c)
		}
	}

	// 3. Split on \n, collapse consecutive empty/whitespace-only lines
	lines := strings.Split(cleaned.String(), "\n")
	var resultLines []string
	prevEmpty := false
	for _, line := range lines {
		isEmpty := strings.TrimSpace(line) == ""
		if isEmpty {
			if prevEmpty {
				continue
			}
			prevEmpty = true
		} else {
			prevEmpty = false
		}
		resultLines = append(resultLines, line)
	}

	// 4. Join with \n and strip leading/trailing whitespace
	return strings.TrimSpace(strings.Join(resultLines, "\n"))
}

// TextRemoveNewlines converts multi-line text into a single normalized line.
//
// Splits on whitespace boundaries and joins with a single space,
// removing newlines and collapsing consecutive whitespace.
func TextRemoveNewlines(text string) string {
	return strings.Join(strings.Fields(text), " ")
}

// TextTrim trims text so its UTF-8 encoded size does not exceed nbytes.
//
// Finds the largest valid UTF-8 prefix within nbytes, then strips
// leading/trailing whitespace. Multi-byte characters that would be
// split are dropped entirely.
func TextTrim(text string, nbytes int) string {
	if len(text) <= nbytes {
		return strings.TrimSpace(text)
	}
	// Take first nbytes bytes, find last valid rune boundary
	truncated := text[:nbytes]
	// Walk backwards to find valid UTF-8 end
	for len(truncated) > 0 && !utf8.ValidString(truncated) {
		truncated = truncated[:len(truncated)-1]
	}
	return strings.TrimSpace(truncated)
}

// TextCollapse normalizes and simplifies text for similarity hashing.
//
// Applies NFD normalization, lowercasing, removes whitespace and characters
// in Unicode categories C (control), M (mark), and P (punctuation), then
// recombines with NFKC normalization.
func TextCollapse(text string) string {
	// 1. NFD normalize and lowercase
	nfdLower := strings.ToLower(norm.NFD.String(text))

	// 2. Filter: keep chars that are NOT whitespace AND NOT in C/M/P categories
	var filtered strings.Builder
	filtered.Grow(len(nfdLower))
	for _, c := range nfdLower {
		if !unicode.IsSpace(c) && !isCMPCategory(c) {
			filtered.WriteRune(c)
		}
	}

	// 3. NFKC normalize the filtered result
	return norm.NFKC.String(filtered.String())
}
