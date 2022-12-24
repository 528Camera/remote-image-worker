package socket

import (
	"context"
	"time"

	pb "github.com/AsriFox/remote-image-worker/internal/pb"
	zmq "github.com/go-zeromq/zmq4"
	"google.golang.org/protobuf/proto"
)

type PushSocket struct {
	socket zmq.Socket
}

func NewPushSocket() (*PushSocket, error) {
	s := zmq.NewPush(
		context.Background(),
		zmq.WithDialerTimeout(time.Second*3),
	)
	return &PushSocket{s}, nil
}

func (pusher PushSocket) Bind(endpoint string) error {
	return pusher.socket.Listen(endpoint)
}

func (pusher PushSocket) Connect(endpoint string) error {
	return pusher.socket.Dial(endpoint)
}

func (pusher PushSocket) Push(frame_data *pb.FrameData) error {
	out, err := proto.Marshal(frame_data)
	if err != nil {
		return err
	}
	msg := zmq.NewMsg(out)
	return pusher.socket.Send(msg)
}

func (pusher PushSocket) Close() error {
	return pusher.socket.Close()
}
