package scenarios

func ChannelCloseScenario() chan int {
	ch := make(chan int)
	close(ch)
	return ch
}
