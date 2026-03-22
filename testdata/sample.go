package main

import (
	"fmt"
	"os"
	"strings"
)

// This is a very long line that exceeds the maximum line length allowed by the linter and should trigger an error

func main() {
	unused := "this variable is unused"
	_ = unused

	// Empty block
	if true {
	}

	result := add(1, 2)
	fmt.Println(result)
}

func add(a, b int) int {
	return a + b
}

// Unexported function doesn't need documentation
func unexported() {}
