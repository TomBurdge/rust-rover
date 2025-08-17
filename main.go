package main

import (
	"encoding/json"
	"fmt"
	"log"
	"runtime"

	"github.com/ebitengine/purego"
)

type externFunc func(string, string) string

func getRoverCoordinates(call externFunc, topRight, instructions string) (string, error) {
	type CoordinatesResult struct {
		Result string `json:"result"`
		Error  string `json:"error"`
	}

	res := call(topRight, instructions)

	var out CoordinatesResult
	if err := json.Unmarshal([]byte(res), &out); err != nil {
		return "", fmt.Errorf("decoding in go from rust result: %w", err)
	}
	if out.Error != "" {
		return "", fmt.Errorf(out.Error)
	}
	return out.Result, nil
}

func resolveLibPath() (string, error) {
	var target string
	switch {
	case runtime.GOOS == "linux" && runtime.GOARCH == "amd64":
		target = "x86_64-unknown-linux-gnu"
	case runtime.GOOS == "linux" && runtime.GOARCH == "arm64":
		target = "aarch64-unknown-linux-gnu"
	case runtime.GOOS == "darwin" && runtime.GOARCH == "arm64":
		target = "aarch64-apple-darwin"
	default:
		return "", fmt.Errorf("unsupported platform")
	}
	return fmt.Sprintf("target/%s/release/libc_rover.so", target), nil
}

func main() {
	libPath, err := resolveLibPath()

	rustlib, err := purego.Dlopen(libPath, purego.RTLD_NOW|purego.RTLD_GLOBAL)
	if err != nil {
		log.Fatalf("failed to load library: %v", err)
	}
	defer purego.Dlclose(rustlib)

	var extern_return_coordinates func(string, string) string
	purego.RegisterLibFunc(&extern_return_coordinates, rustlib, "extern_return_coordinates")

	println("Top right go input:")
	top_right := "5 5"
	println(top_right)
	instructions := `1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM`

	println("Starting position and instructions go inputs:")
	println(instructions)

	get_rover_coordinates := func(top_right string, instructions string) (string, error) {
		type CoordinatesResult struct {
			Result string `json:"result"`
			Error  string `json:"error"`
		}
		res := extern_return_coordinates(top_right, instructions)

		var out CoordinatesResult
		if err := json.Unmarshal([]byte(res), &out); err != nil {
			return "", fmt.Errorf("decoding in go from rust result: %w", err)
		}

		if out.Error != "" {
			return "", fmt.Errorf(out.Error)
		}

		return out.Result, nil
	}
	res, err := get_rover_coordinates(top_right, instructions)
	if err != nil {
		println(err)
	}
	println("Success! Received the output from rust. Final coordinates:")
	println(res)
}
