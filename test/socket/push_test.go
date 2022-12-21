package socket

import (
	"fmt"
	"testing"

	pb "github.com/AsriFox/remote-image-worker/internal/pb"
	"github.com/AsriFox/remote-image-worker/internal/socket"
	"google.golang.org/protobuf/types/known/timestamppb"
)

func TestPush(t *testing.T) {
	t.Parallel()
	s, err := socket.NewPushSocket()
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	err = s.Bind("tcp://*:5557")
	if err != nil {
		t.Fatal(err)
	}
	for i := 0; i < 10; i++ {
		fmt.Printf("Sending frame %d\n", i)
		frame_data := pb.FrameData{
			FrameIndex: int32(i),
			Version:    1,
			Time:       timestamppb.Now(),
		}
		err = s.Push(&frame_data)
		if err != nil {
			t.Error(err)
		}
	}
}
