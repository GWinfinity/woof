package scenarios

func DeepNestingScenario(x int) {
	if x > 0 {
		if x > 1 {
			if x > 2 {
				if x > 3 {
					if x > 4 {
						if x > 5 {
							_ = x
						}
					}
				}
			}
		}
	}
}
