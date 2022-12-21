package main

import (
	"fmt"
	"log"
	"os"

	"github.com/AsriFox/remote-image-worker/internal/encod"
	"github.com/AsriFox/remote-image-worker/internal/proc"
	"github.com/AsriFox/remote-image-worker/internal/socket"
)

func main() {
	if len(os.Args) < 2 {
		return
	}
	pull_ep := os.Args[0]
	push_ep := os.Args[1]
	var dump_path string
	if len(os.Args) == 3 {
		dump_path = os.Args[2]
	}
	puller, err := socket.NewPullSocket(pull_ep)
	if err != nil {
		log.Println("Failed to create a PULL socket")
		return
	}
	defer puller.Close()
	pusher, err := socket.NewPushSocket()
	if err != nil {
		log.Println("Failed to create a PUSH socket")
		return
	}
	defer pusher.Close()
	err = pusher.Connect(push_ep)
	if err != nil {
		log.Println("Failed to connect the PUSH socket")
		return
	}
	for {
		frame_data, err := puller.Pull()
		if err != nil {
			log.Println("Pull failed")
			continue
		}
		log.Println("Frame received")
		log.Printf("Index: %d\n", frame_data.FrameIndex)
		log.Printf("Time: %s\n", frame_data.Time)
		img, err := encod.Imdecode(frame_data.Image)
		if err != nil {
			log.Println("Failed to decode")
			continue
		}
		err = proc.ReduceColors(&img)
		if err != nil {
			log.Println("Failed to process the image")
			continue
		}
		buf, err := encod.NewJpegDefaults(50).Imencode(img)
		if err != nil {
			log.Println("Failed to encode the image")
			continue
		}
		if dump_path != "" {
			err = os.WriteFile(
				fmt.Sprintf("%s/frame_%d", dump_path, frame_data.FrameIndex),
				buf,
				os.ModePerm,
			)
			if err != nil {
				log.Println(err)
			}
			continue
		}
		frame_data.Image = buf
		err = pusher.Push(frame_data)
		if err != nil {
			log.Println("Push failed")
		}
	}
}
