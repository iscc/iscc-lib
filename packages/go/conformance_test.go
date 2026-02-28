// Tests for ConformanceSelftest — validates all 46 conformance vectors in a single call.
package iscc

import "testing"

func TestPureGoConformanceSelftest(t *testing.T) {
	passed, err := ConformanceSelftest()
	if err != nil {
		t.Fatalf("ConformanceSelftest returned error: %v", err)
	}
	if !passed {
		t.Fatal("ConformanceSelftest returned false — one or more vectors failed (see stderr)")
	}
}
