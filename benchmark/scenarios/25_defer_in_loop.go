package scenarios

import "os"

func DeferInLoopScenario(files []string) {
	for _, f := range files {
		file, _ := os.Open(f)
		defer file.Close()
	}
}
