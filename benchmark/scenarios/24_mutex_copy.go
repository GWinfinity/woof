package scenarios

import "sync"

type MutexHolder struct {
	mu sync.Mutex
	x  int
}

func MutexCopyScenario() MutexHolder {
	m := MutexHolder{x: 1}
	return m
}
