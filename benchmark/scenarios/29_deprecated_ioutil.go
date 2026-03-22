package scenarios

import "io/ioutil"

func DeprecatedIOUtilScenario() ([]byte, error) {
	return ioutil.ReadFile("test.txt")
}
