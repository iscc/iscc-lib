// Tests for the pure Go text utility functions.
// These tests do NOT require the WASM binary — they test pure Go functions.
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
