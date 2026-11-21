// Tests for the pure Go text utility functions.
package iscc

import "testing"

// ---- TextClean tests ----

func TestUtilsTextCleanNFKCNormalization(t *testing.T) {
	// U+FB01 (fi ligature) should normalize to "fi" under NFKC
	result := TextClean("  Hel\uFB01 World  ")
	expected := "Helfi World"
	if result != expected {
		t.Errorf("TextClean NFKC: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCleanRemovesControlChars(t *testing.T) {
	// Tab (U+0009) is a control character and should be removed
	result := TextClean("hello\tworld")
	expected := "helloworld"
	if result != expected {
		t.Errorf("TextClean control chars: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCleanPreservesNewlines(t *testing.T) {
	result := TextClean("hello\nworld")
	expected := "hello\nworld"
	if result != expected {
		t.Errorf("TextClean newlines: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCleanCollapsesEmptyLines(t *testing.T) {
	// Three consecutive newlines should collapse to one empty line (two newlines)
	result := TextClean("a\n\n\nb")
	expected := "a\n\nb"
	if result != expected {
		t.Errorf("TextClean collapse: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCleanHandlesCRLF(t *testing.T) {
	result := TextClean("a\r\nb")
	expected := "a\nb"
	if result != expected {
		t.Errorf("TextClean CRLF: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCleanStripsWhitespace(t *testing.T) {
	result := TextClean("  hello  ")
	expected := "hello"
	if result != expected {
		t.Errorf("TextClean strip: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCleanEmpty(t *testing.T) {
	result := TextClean("")
	if result != "" {
		t.Errorf("TextClean empty: got %q, want %q", result, "")
	}
}

func TestUtilsTextCleanNFKCMathH(t *testing.T) {
	// U+210D (double-struck H) should normalize to H under NFKC
	result := TextClean("\u210D")
	if result != "H" {
		t.Errorf("TextClean NFKC math H: got %q, want %q", result, "H")
	}
}

// ---- TextRemoveNewlines tests ----

func TestUtilsTextRemoveNewlines(t *testing.T) {
	result := TextRemoveNewlines("hello\nworld\r\nfoo")
	expected := "hello world foo"
	if result != expected {
		t.Errorf("TextRemoveNewlines: got %q, want %q", result, expected)
	}
}

func TestUtilsTextRemoveNewlinesCollapsesSpaces(t *testing.T) {
	result := TextRemoveNewlines("a  b   c")
	expected := "a b c"
	if result != expected {
		t.Errorf("TextRemoveNewlines collapse: got %q, want %q", result, expected)
	}
}

// ---- TextTrim tests ----

func TestUtilsTextTrimNoTruncation(t *testing.T) {
	result := TextTrim("hello", 10)
	expected := "hello"
	if result != expected {
		t.Errorf("TextTrim no truncation: got %q, want %q", result, expected)
	}
}

func TestUtilsTextTrimExact(t *testing.T) {
	result := TextTrim("hello", 5)
	expected := "hello"
	if result != expected {
		t.Errorf("TextTrim exact: got %q, want %q", result, expected)
	}
}

func TestUtilsTextTrimTruncates(t *testing.T) {
	result := TextTrim("hello world", 5)
	expected := "hello"
	if result != expected {
		t.Errorf("TextTrim truncate: got %q, want %q", result, expected)
	}
}

func TestUtilsTextTrimUnicodeBoundary(t *testing.T) {
	// "é" is 2 bytes in UTF-8 (C3 A9). Truncating at 1 byte should drop it.
	result := TextTrim("é", 1)
	if result != "" {
		t.Errorf("TextTrim unicode boundary: got %q, want %q", result, "")
	}
}

func TestUtilsTextTrimStrips(t *testing.T) {
	result := TextTrim("hello ", 6)
	expected := "hello"
	if result != expected {
		t.Errorf("TextTrim strip: got %q, want %q", result, expected)
	}
}

func TestUtilsTextTrimLongString(t *testing.T) {
	result := TextTrim("Hello, World! This is a long string.", 10)
	// Result should be at most 10 UTF-8 bytes, with trailing whitespace stripped.
	if len(result) > 10 {
		t.Errorf("TextTrim long: result %q exceeds 10 bytes (got %d bytes)", result, len(result))
	}
	if len(result) == 0 {
		t.Error("TextTrim long: result is empty")
	}
}

// ---- TextCollapse tests ----

func TestUtilsTextCollapseBasic(t *testing.T) {
	result := TextCollapse("Hello, World!")
	expected := "helloworld"
	if result != expected {
		t.Errorf("TextCollapse basic: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseStripsAccents(t *testing.T) {
	// NFD decomposes accented chars, then M-category marks are filtered
	result := TextCollapse("café")
	expected := "cafe"
	if result != expected {
		t.Errorf("TextCollapse accents: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseStripsPunctuation(t *testing.T) {
	result := TextCollapse("hello, world!")
	expected := "helloworld"
	if result != expected {
		t.Errorf("TextCollapse punctuation: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseEmpty(t *testing.T) {
	result := TextCollapse("")
	if result != "" {
		t.Errorf("TextCollapse empty: got %q, want %q", result, "")
	}
}

func TestUtilsTextCollapseHelloWorld(t *testing.T) {
	result := TextCollapse("Hello World")
	expected := "helloworld"
	if result != expected {
		t.Errorf("TextCollapse hello world: got %q, want %q", result, expected)
	}
}

// Final_Sigma regression tests: context-sensitive lowercasing must map a
// word-final capital sigma to "ς" (U+03C2), not "σ" (U+03C3). Expected
// values produced by the Rust core (iscc_lib.text_collapse), which matches
// the iscc-core reference implementation.

func TestUtilsTextCollapseFinalSigmaLogos(t *testing.T) {
	result := TextCollapse("ΛΟΓΟΣ")
	expected := "λογος"
	if result != expected {
		t.Errorf("TextCollapse final sigma: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseFinalSigmaShortWord(t *testing.T) {
	result := TextCollapse("ΑΣ")
	expected := "ας"
	if result != expected {
		t.Errorf("TextCollapse final sigma short: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseFinalSigmaDouble(t *testing.T) {
	// Only the second (word-final) sigma becomes "ς"
	result := TextCollapse("ΣΣ")
	expected := "σς"
	if result != expected {
		t.Errorf("TextCollapse double sigma: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseFinalSigmaOdysseus(t *testing.T) {
	result := TextCollapse("ὈΔΥΣΣΕΎΣ")
	expected := "οδυσσευς"
	if result != expected {
		t.Errorf("TextCollapse Odysseus: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseNonFinalSigma(t *testing.T) {
	// Guard against over-correction: a "Σ" followed by a cased letter must
	// stay "σ"
	result := TextCollapse("ΑΣΒ")
	expected := "ασβ"
	if result != expected {
		t.Errorf("TextCollapse non-final sigma: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseGreekPhrase(t *testing.T) {
	// Word-internal sigmas stay "σ" even with surrounding whitespace removed
	result := TextCollapse("ΓΕΙΑ ΣΟΥ ΚΟΣΜΕ")
	expected := "γειασουκοσμε"
	if result != expected {
		t.Errorf("TextCollapse Greek phrase: got %q, want %q", result, expected)
	}
}

func TestUtilsTextCollapseTurkishCapitalI(t *testing.T) {
	// Und locale: "İ" (U+0130) lowercases to "i" + combining dot, and the
	// combining mark is filtered — no Turkish-specific mapping leaks in
	result := TextCollapse("İstanbul")
	expected := "istanbul"
	if result != expected {
		t.Errorf("TextCollapse Turkish I: got %q, want %q", result, expected)
	}
}

func TestUtilsTextTrimStripsInfoSeparators(t *testing.T) {
	// Python str.strip() removes U+001C..U+001F; pinned literally so a shrunken
	// isPyStripChar cannot hide behind a loop over the predicate itself
	for _, sep := range []string{"\u001c", "\u001d", "\u001e", "\u001f"} {
		// Short path (nbytes >= len): outer separators stripped, inner kept
		short := sep + "ab" + sep + "cd" + sep
		if got, want := TextTrim(short, 64), "ab"+sep+"cd"; got != want {
			t.Errorf("TextTrim short %q: got %q, want %q", short, got, want)
		}
		// Truncation path: the cut lands right after a separator run
		long := sep + "ab" + sep + sep + "cd"
		nbytes := len(sep + "ab" + sep + sep)
		if got := TextTrim(long, nbytes); got != "ab" {
			t.Errorf("TextTrim truncated %q at %d: got %q, want %q", long, nbytes, got, "ab")
		}
	}
}

func TestUtilsTextTrimKeepsZeroWidthSpace(t *testing.T) {
	// U+200B is outside Python's strip set
	in := "\u200bab\u200b"
	if got := TextTrim(in, 64); got != in {
		t.Errorf("TextTrim(%q): got %q, want unchanged", in, got)
	}
}

func TestUtilsTextRemoveNewlinesSplitsOnInfoSeparators(t *testing.T) {
	if got := TextRemoveNewlines("\u001eab\u001fcd\u001e"); got != "ab cd" {
		t.Errorf("TextRemoveNewlines issue example: got %q, want %q", got, "ab cd")
	}
	for _, sep := range []string{"\u001c", "\u001d", "\u001e", "\u001f"} {
		// leading, interior (single and run) and trailing positions
		in := sep + "a" + sep + "b" + sep + sep + "c" + sep
		if got := TextRemoveNewlines(in); got != "a b c" {
			t.Errorf("TextRemoveNewlines(%q): got %q, want %q", in, got, "a b c")
		}
	}
}

func TestUtilsTextRemoveNewlinesKeepsZeroWidthSpace(t *testing.T) {
	in := "a\u200bb"
	if got := TextRemoveNewlines(in); got != in {
		t.Errorf("TextRemoveNewlines(%q): got %q, want unchanged", in, got)
	}
}

func TestUtilsTextTrimNegativeNbytes(t *testing.T) {
	if got := TextTrim("abc", -1); got != "" {
		t.Errorf("TextTrim(%q, -1): got %q, want empty", "abc", got)
	}
}

func TestUtilsTextTrimNegativeNbytesEmptyText(t *testing.T) {
	if got := TextTrim("", -1); got != "" {
		t.Errorf("TextTrim(%q, -1): got %q, want empty", "", got)
	}
}

func TestUtilsTextTrimZeroNbytes(t *testing.T) {
	if got := TextTrim("abc", 0); got != "" {
		t.Errorf("TextTrim(%q, 0): got %q, want empty", "abc", got)
	}
}
