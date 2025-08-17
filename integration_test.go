package main

import (
	"testing"

	"github.com/ebitengine/purego"
)

func Test_GetRoverCoordinates_Integration(t *testing.T) {
	libPath, err := resolveLibPath()
	if err != nil {
		t.Fatalf("Failed to resolve name to load the library. : %v", err)
	}

	rustlib, err := purego.Dlopen(libPath, purego.RTLD_NOW|purego.RTLD_GLOBAL)
	if err != nil {
		t.Fatalf("failed to load library. This could be because you have not built the rust FFI yet with `mask build`: %v", err)
	}
	defer purego.Dlclose(rustlib)

	var extern_return_coordinates func(string, string) string
	purego.RegisterLibFunc(&extern_return_coordinates, rustlib, "extern_return_coordinates")

	topRight := "5 5"
	instructions := `1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM`

	got, err := getRoverCoordinates(extern_return_coordinates, topRight, instructions)
	if err != nil {
		t.Fatalf("error from rust call: %v", err)
	}

	want := "1 3 N\n5 1 E"

	if got != want {
		t.Fatalf("got %q want %q", got, want)
	}

	t.Logf("final coordinates:\n%s", got)
}

func Test_GetRoverCoordinatesOOB_Integration(t *testing.T) {
	libPath, err := resolveLibPath()
	if err != nil {
		t.Fatalf("Failed to resolve name to load the library. : %v", err)
	}

	rustlib, err := purego.Dlopen(libPath, purego.RTLD_NOW|purego.RTLD_GLOBAL)
	if err != nil {
		t.Fatalf("failed to load library. This could be because you have not built the rust FFI yet with `mask build`: %v", err)
	}
	defer purego.Dlclose(rustlib)

	var extern_return_coordinates func(string, string) string
	purego.RegisterLibFunc(&extern_return_coordinates, rustlib, "extern_return_coordinates")

	topRight := "5 5"
	instructions := `1 2 N
MMMMMMMMMMMMMMMM`

	got, err := getRoverCoordinates(extern_return_coordinates, topRight, instructions)

	if err == nil {
		t.Fatalf("Expected an error from the rust call. Instead received success output: %v", got)
	}

	want := "Instruction tried to send Rover too far \"N\""

	if err.Error() != want {
		t.Fatalf("got %q want %q", got, want)
	}

	t.Logf("final coordinates:\n%s", got)
}
