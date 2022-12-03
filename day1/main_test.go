package main

import (
	"bytes"
	"testing"
)

func TestFindMostCalories(t *testing.T) {
	var buf bytes.Buffer

	buf.WriteString(`
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
`)

	want := 24000

	got, err := findMostCalories(&buf)
	if err != nil {
		t.Fatalf("findMostCalories() returned an unexpected error: %v", err)
	}
	if got != want {
		t.Errorf("findMostCalories() = %d, want %d", got, want)
	}
}
