// Tests for GenSumCodeV0: equivalence, result fields, error handling, wide mode.
package iscc

import (
	"os"
	"strings"
	"testing"
)

// TestGenSumCodeV0Equivalence verifies that GenSumCodeV0 produces the same
// ISCC-CODE as calling GenDataCodeV0 + GenInstanceCodeV0 + GenIsccCodeV0
// separately on the same data.
func TestGenSumCodeV0Equivalence(t *testing.T) {
	content := []byte("Hello ISCC GenSumCodeV0 equivalence test data!")

	f, err := os.CreateTemp("", "iscc-sum-equiv-*.bin")
	if err != nil {
		t.Fatalf("create temp file: %v", err)
	}
	defer os.Remove(f.Name())

	if _, err := f.Write(content); err != nil {
		t.Fatalf("write temp file: %v", err)
	}
	if err := f.Close(); err != nil {
		t.Fatalf("close temp file: %v", err)
	}

	// Single-pass via GenSumCodeV0
	sumResult, err := GenSumCodeV0(f.Name(), 64, false)
	if err != nil {
		t.Fatalf("GenSumCodeV0: %v", err)
	}

	// Two-pass: separate Data + Instance + ISCC assembly
	dataResult, err := GenDataCodeV0(content, 64)
	if err != nil {
		t.Fatalf("GenDataCodeV0: %v", err)
	}
	instanceResult, err := GenInstanceCodeV0(content, 64)
	if err != nil {
		t.Fatalf("GenInstanceCodeV0: %v", err)
	}
	isccResult, err := GenIsccCodeV0([]string{dataResult.Iscc, instanceResult.Iscc}, false)
	if err != nil {
		t.Fatalf("GenIsccCodeV0: %v", err)
	}

	// Verify ISCC codes match
	if sumResult.Iscc != isccResult.Iscc {
		t.Errorf("iscc mismatch: sum=%q, composed=%q", sumResult.Iscc, isccResult.Iscc)
	}

	// Verify datahash matches instance result
	if sumResult.Datahash != instanceResult.Datahash {
		t.Errorf("datahash mismatch: sum=%q, instance=%q", sumResult.Datahash, instanceResult.Datahash)
	}

	// Verify filesize matches instance result
	if sumResult.Filesize != instanceResult.Filesize {
		t.Errorf("filesize mismatch: sum=%d, instance=%d", sumResult.Filesize, instanceResult.Filesize)
	}
}

// TestGenSumCodeV0ResultFields verifies that SumCodeResult has the expected
// field formats: ISCC prefix, datahash prefix, and correct filesize.
func TestGenSumCodeV0ResultFields(t *testing.T) {
	content := []byte("Result fields test content for ISCC SUM")

	f, err := os.CreateTemp("", "iscc-sum-fields-*.bin")
	if err != nil {
		t.Fatalf("create temp file: %v", err)
	}
	defer os.Remove(f.Name())

	if _, err := f.Write(content); err != nil {
		t.Fatalf("write temp file: %v", err)
	}
	if err := f.Close(); err != nil {
		t.Fatalf("close temp file: %v", err)
	}

	result, err := GenSumCodeV0(f.Name(), 64, false)
	if err != nil {
		t.Fatalf("GenSumCodeV0: %v", err)
	}

	if !strings.HasPrefix(result.Iscc, "ISCC:") {
		t.Errorf("iscc should start with 'ISCC:', got %q", result.Iscc)
	}

	if !strings.HasPrefix(result.Datahash, "1e20") {
		t.Errorf("datahash should start with '1e20', got %q", result.Datahash)
	}

	if result.Filesize != uint64(len(content)) {
		t.Errorf("filesize: got %d, want %d", result.Filesize, len(content))
	}
}

// TestGenSumCodeV0ErrorNonExistent verifies that GenSumCodeV0 returns an error
// for a non-existent file path.
func TestGenSumCodeV0ErrorNonExistent(t *testing.T) {
	_, err := GenSumCodeV0("/nonexistent/path/to/file.bin", 64, false)
	if err == nil {
		t.Fatal("expected error for non-existent file, got nil")
	}
}

// TestGenSumCodeV0WideMode verifies that wide=true produces a different ISCC
// than wide=false for 128-bit codes, but datahash and filesize remain identical.
// Uses bits=128 because 64-bit codes produce identical output in both modes.
func TestGenSumCodeV0WideMode(t *testing.T) {
	content := []byte("Wide mode test content for ISCC SUM generation!")

	f, err := os.CreateTemp("", "iscc-sum-wide-*.bin")
	if err != nil {
		t.Fatalf("create temp file: %v", err)
	}
	defer os.Remove(f.Name())

	if _, err := f.Write(content); err != nil {
		t.Fatalf("write temp file: %v", err)
	}
	if err := f.Close(); err != nil {
		t.Fatalf("close temp file: %v", err)
	}

	narrowResult, err := GenSumCodeV0(f.Name(), 128, false)
	if err != nil {
		t.Fatalf("GenSumCodeV0 narrow: %v", err)
	}

	wideResult, err := GenSumCodeV0(f.Name(), 128, true)
	if err != nil {
		t.Fatalf("GenSumCodeV0 wide: %v", err)
	}

	// ISCC codes should differ between narrow and wide modes
	if narrowResult.Iscc == wideResult.Iscc {
		t.Errorf("expected different ISCC codes for narrow vs wide, both got %q", narrowResult.Iscc)
	}

	// Datahash should be identical regardless of mode
	if narrowResult.Datahash != wideResult.Datahash {
		t.Errorf("datahash should be identical: narrow=%q, wide=%q", narrowResult.Datahash, wideResult.Datahash)
	}

	// Filesize should be identical regardless of mode
	if narrowResult.Filesize != wideResult.Filesize {
		t.Errorf("filesize should be identical: narrow=%d, wide=%d", narrowResult.Filesize, wideResult.Filesize)
	}
}
