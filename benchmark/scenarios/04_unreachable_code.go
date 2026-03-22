package scenarios

func UnreachableCodeScenario() bool {
	return true
	x := 1
	_ = x
}
