package scenarios

import "os"

func IgnoredErrorScenario() {
	f, _ := os.Open("test.txt")
	_ = f
}
