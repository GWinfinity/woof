package scenarios

import "errors"

func ErrorNotLastReturnScenario() (error, int) {
	return errors.New("test"), 42
}
