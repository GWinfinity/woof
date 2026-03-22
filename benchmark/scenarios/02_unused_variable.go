package scenarios

func UnusedVariableScenario() {
	used := "hello"
	unused := "world"
	_ = used
}
