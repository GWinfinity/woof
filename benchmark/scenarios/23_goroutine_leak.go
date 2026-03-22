package scenarios

import "time"

func GoroutineLeakScenario() {
	go func() {
		time.Sleep(1 * time.Second)
	}()
}
