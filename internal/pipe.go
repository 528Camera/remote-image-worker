package internal

import (
	"fmt"
	"log"
	"os"

	"github.com/AsriFox/remote-image-worker/internal/encod"
	"github.com/AsriFox/remote-image-worker/internal/proc"
	"github.com/AsriFox/remote-image-worker/internal/socket"
	"gocv.io/x/gocv"
)

type Pipeline struct {
	puller    *socket.PullSocket
	pusher    *socket.PushSocket
	dump_path string
}

func NewPipeline(pull_ep string, push_ep string) (*Pipeline, error) {
	puller, err := socket.NewPullSocket(pull_ep)
	if err != nil {
		log.Println("Failed to create a PULL socket")
		return nil, err
	}
	pusher, err := socket.NewPushSocket()
	if err != nil {
		log.Println("Failed to create a PUSH socket")
		puller.Close()
		return nil, err
	}
	err = pusher.Connect(push_ep)
	if err != nil {
		log.Println("Failed to connect the PUSH socket")
		puller.Close()
		pusher.Close()
		return nil, err
	}
	return &Pipeline{puller, pusher, ""}, nil
}

func NewPipelineDump(pull_ep string, dump_path string) (*Pipeline, error) {
	puller, err := socket.NewPullSocket(pull_ep)
	if err != nil {
		log.Println("Failed to create a PULL socket")
		return nil, err
	}
	return &Pipeline{puller, nil, dump_path}, nil
}

func (pipe *Pipeline) SetDumpPath(dump_path string) {
	pipe.dump_path = dump_path
}

func (pipe Pipeline) Process() error {
	frame_data, err := pipe.puller.Pull()
	if err != nil {
		log.Println("Pull failed")
		return err
	}
	log.Println("Frame received")
	log.Printf("Index: %d\n", frame_data.FrameIndex)
	log.Printf("Time: %s\n", frame_data.Time)
	img, err := encod.Imdecode(frame_data.Image)
	if err != nil {
		// log.Println("Failed to decode")
		return err
	}

	edg := proc.DetectEdges(&img)
	gocv.CvtColor(edg, &edg, gocv.ColorGrayToBGR)
	err = proc.ReduceColors(&img)
	if err != nil {
		// log.Println("Failed to process the image")
		return err
	}
	imgr := img.Clone()
	gocv.AddWeighted(img, 1.0, edg, -1.0, 0.0, &imgr)

	buf, err := encod.NewJpegDefaults(50).Imencode(img)
	if err != nil {
		// log.Println("Failed to encode the image")
		return err
	}
	if pipe.dump_path != "" {
		err = os.WriteFile(
			fmt.Sprintf("%s/frame_%d.jpg", pipe.dump_path, frame_data.FrameIndex),
			buf,
			os.ModePerm,
		)
		return err
	}
	frame_data.Image = buf
	return pipe.pusher.Push(frame_data)
	// if err != nil {
	// 	log.Println("Push failed")
	// }
}

func (pipe Pipeline) Close() error {
	e1 := pipe.puller.Close()
	e2 := pipe.pusher.Close()
	if e1 != nil || e2 != nil {
		return fmt.Errorf(
			"errors: %s, %s",
			e1.Error(),
			e2.Error(),
		)
	}
	return nil
}
