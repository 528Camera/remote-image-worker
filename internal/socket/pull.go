package socket

import (
	"time"

	pb "github.com/AsriFox/remote-image-worker/internal/pb"
	zmq "github.com/pebbe/zmq4"
	"google.golang.org/protobuf/proto"
)

type PullSocket struct {
	socket zmq.Socket
}

func NewPullSocket(endpoint string) (*PullSocket, error) {
	zctx, err := zmq.NewContext()
	if err != nil {
		return nil, err
	}
	s, err := zctx.NewSocket(zmq.PULL)
	if err != nil {
		return nil, err
	}
	err = s.SetLinger(100)
	if err != nil {
		return nil, err
	}
	err = s.Connect(endpoint)
	return &PullSocket{*s}, err
}

func (self PullSocket) Pull() (*pb.FrameData, error) {
	poller := zmq.NewPoller()
	poller.Add(&self.socket, zmq.POLLIN)
	poll, err := poller.Poll(time.Millisecond * 3000)
	if err != nil {
		return nil, err
	}
	for _, socket := range poll {
		var msg []byte
		switch s := socket.Socket; s {
		case &self.socket:
			msg, err = s.RecvBytes(0)
		}
		if err != nil {
			return nil, err
		}
		var frame_data pb.FrameData
		err = proto.Unmarshal(msg, &frame_data)
		return &frame_data, err
	}
	return nil, nil
}

func (self PullSocket) Close() error {
	return self.socket.Close()
}
