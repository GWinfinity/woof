package scenarios

func RaceConditionScenario() int {
	var counter int
	go func() {
		counter++
	}()
	go func() {
		counter++
	}()
	return counter
}
