package scenarios

func HighCyclomaticScenario(x, y, z int) int {
	if x > 0 {
		if y > 0 {
			if z > 0 {
				return 1
			} else if z < 0 {
				return 2
			} else {
				return 3
			}
		} else if y < 0 {
			if z > 0 {
				return 4
			} else if z < 0 {
				return 5
			} else {
				return 6
			}
		} else {
			return 7
		}
	} else if x < 0 {
		return 8
	} else {
		return 9
	}
}
