package scenarios

import "fmt"

func ShadowVariableScenario() {
	x := 1
	if true {
		x := 2
		fmt.Println(x)
	}
	fmt.Println(x)
}
