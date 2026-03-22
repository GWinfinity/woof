package main

import (
	"fmt"
	"os"
)

// This function has a naked return
func namedReturn() (result int) {
	result = 42
	return  // naked return
}

// This function has an unused parameter
func withUnusedParam(used int, unused int) int {
	return used * 2
}

// This function ignores error return value
func ignoreError() {
	os.Open("test.txt")  // error not checked
	fmt.Println("done")
}

// Redundant slice
func redundantSlice(s []int) []int {
	return s[:]
}

func main() {
	_ = namedReturn()
	_ = withUnusedParam(1, 2)
	ignoreError()
	_ = redundantSlice([]int{1, 2, 3})
}
