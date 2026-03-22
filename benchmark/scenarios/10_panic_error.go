package scenarios

func PanicErrorScenario(x int) int {
	if x < 0 {
		panic("negative")
	}
	return x
}
