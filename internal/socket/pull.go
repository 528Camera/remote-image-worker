package socket

import (
	"context"
	"time"

	pb "github.com/AsriFox/remote-image-worker/internal/pb"
	zmq "github.com/go-zeromq/zmq4"
	"google.golang.org/protobuf/proto"
)

type PullSocket struct {
	socket zmq.Socket
}

func NewPullSocket(endpoint string) (*PullSocket, error) {
	s := zmq.NewPull(
		context.Background(),
		zmq.WithDialerTimeout(time.Second*3),
	)
	if err := s.Dial(endpoint); err != nil {
		s.Close()
		return nil, err
	}
	return &PullSocket{s}, nil
}

func (puller PullSocket) Pull() (*pb.FrameData, error) {
	msg, err := puller.socket.Recv()
	if err != nil {
		return nil, err
	}
	var frame_data pb.FrameData
	err = proto.Unmarshal(msg.Bytes(), &frame_data)
	return &frame_data, err
}

func (puller PullSocket) Close() error {
	return puller.socket.Close()
}
