// Conformance tests for GenDataCodeV0 against data.json vectors.
package iscc

import (
	"strings"
	"testing"
)

func TestPureGoGenDataCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_data_code_v0") {
		t.Run(name, func(t *testing.T) {
			inputBytes, bits, err := streamInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}

			result, err := GenDataCodeV0(inputBytes, bits)
			if err != nil {
				t.Fatalf("GenDataCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}

// finalizedDataHasher returns a DataHasher whose Finalize already succeeded.
func finalizedDataHasher(t *testing.T) *DataHasher {
	t.Helper()
	h := NewDataHasher()
	if err := h.Push([]byte("hello world")); err != nil {
		t.Fatalf("Push: %v", err)
	}
	if _, err := h.Finalize(64); err != nil {
		t.Fatalf("Finalize: %v", err)
	}
	return h
}

func TestDataHasherPushAfterFinalize(t *testing.T) {
	h := finalizedDataHasher(t)
	err := h.Push([]byte("more"))
	if err == nil || !strings.Contains(err.Error(), "already finalized") {
		t.Fatalf("Push after Finalize: got %v, want already finalized error", err)
	}
}

func TestDataHasherFinalizeTwice(t *testing.T) {
	h := finalizedDataHasher(t)
	result, err := h.Finalize(64)
	if err == nil || !strings.Contains(err.Error(), "already finalized") {
		t.Fatalf("second Finalize: got %v, want already finalized error", err)
	}
	if result != nil {
		t.Fatalf("second Finalize: got result %+v, want nil", result)
	}
}

func TestDataHasherFailedFinalizeIsFinal(t *testing.T) {
	h := NewDataHasher()
	if err := h.Push([]byte("hello world")); err != nil {
		t.Fatalf("Push: %v", err)
	}
	if _, err := h.Finalize(63); err == nil {
		t.Fatal("Finalize(63): want invalid bits error, got nil")
	}
	result, err := h.Finalize(64)
	if err == nil || !strings.Contains(err.Error(), "already finalized") {
		t.Fatalf("Finalize after failed Finalize: got %v, want already finalized error", err)
	}
	if result != nil {
		t.Fatalf("Finalize after failed Finalize: got result %+v, want nil", result)
	}
}

func TestDataHasherPushSplitMatchesOneShot(t *testing.T) {
	a := []byte(strings.Repeat("ISCC streaming ", 500))
	b := []byte(strings.Repeat("content defined chunks ", 700))
	h := NewDataHasher()
	if err := h.Push(a); err != nil {
		t.Fatalf("Push(a): %v", err)
	}
	if err := h.Push(b); err != nil {
		t.Fatalf("Push(b): %v", err)
	}
	got, err := h.Finalize(64)
	if err != nil {
		t.Fatalf("Finalize: %v", err)
	}
	want, err := GenDataCodeV0(append(append([]byte{}, a...), b...), 64)
	if err != nil {
		t.Fatalf("GenDataCodeV0: %v", err)
	}
	if got.Iscc != want.Iscc {
		t.Fatalf("Push(a); Push(b) = %s, GenDataCodeV0(a+b) = %s", got.Iscc, want.Iscc)
	}
}

func TestGenDataCodeV0Above256(t *testing.T) {
	data := []byte("hello world")
	_, err := GenDataCodeV0(data, 288)
	assertEncodeBoundErr(t, err)
	if _, err := GenDataCodeV0(data, 256); err != nil {
		t.Fatalf("GenDataCodeV0 256 bits: %v", err)
	}
}

func TestDataHasherFinalizeAbove256(t *testing.T) {
	h := NewDataHasher()
	if err := h.Push([]byte("hello world")); err != nil {
		t.Fatalf("Push: %v", err)
	}
	_, err := h.Finalize(288)
	assertEncodeBoundErr(t, err)
	ok := NewDataHasher()
	if _, err := ok.Finalize(256); err != nil {
		t.Fatalf("Finalize 256 bits: %v", err)
	}
}
