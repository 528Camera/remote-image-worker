package socket

import (
	pb "github.com/AsriFox/remote-image-worker/internal/pb"
	zmq "github.com/pebbe/zmq4"
	"google.golang.org/protobuf/proto"
)

type PushSocket struct {
	socket zmq.Socket
}

func NewPushSocket() (*PushSocket, error) {
	zctx, err := zmq.NewContext()
	if err != nil {
		return nil, err
	}
	s, err := zctx.NewSocket(zmq.PUSH)
	if err != nil {
		return nil, err
	}
	err = s.SetLinger(100)
	return &PushSocket{*s}, err
}

func (self PushSocket) Bind(endpoint string) error {
	return self.socket.Bind(endpoint)
}

func (self PushSocket) Connect(endpoint string) error {
	return self.socket.Connect(endpoint)
}

func (self PushSocket) Push(frame_data *pb.FrameData) error {
	out, err := proto.Marshal(frame_data)
	if err != nil {
		return err
	}
	_, err = self.socket.SendBytes(out, 0)
	return err
}

func (self PushSocket) Close() error {
	return self.socket.Close()
}
