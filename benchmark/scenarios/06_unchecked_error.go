package scenarios

import (
	"os"
	"io/ioutil"
)

func UncheckedErrorScenario() {
	os.Open("test.txt")
	ioutil.ReadFile("test.txt")
}
