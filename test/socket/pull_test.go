package socket

import (
	"fmt"
	"testing"

	"github.com/AsriFox/remote-image-worker/internal/socket"
)

func TestPull(t *testing.T) {
	t.Parallel()
	s, err := socket.NewPullSocket("tcp://127.0.0.1:5557")
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	for i := 0; i < 10; i++ {
		msg, err := s.Pull()
		if err != nil {
			t.Error(err)
		}
		fmt.Printf("Received %s\n", msg)
	}
}
