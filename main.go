package main

import (
	"log"
	"os"

	"github.com/AsriFox/remote-image-worker/internal"
)

// Can process only MAX frames at the same time
const MAX = 10

func main() {
	// Parse command line arguments:
	if len(os.Args) < 2 {
		log.Println("Insufficient number of arguments.\nUsage: remote-image-worker [pull-endpoint] [push-endpoint] {dump-path}")
		return
	}
	pull_ep := os.Args[0]
	push_ep := os.Args[1]
	pipe, err := internal.NewPipeline(pull_ep, push_ep)
	if err != nil {
		return
	}
	defer pipe.Close()
	if len(os.Args) == 3 {
		pipe.SetDumpPath(os.Args[2])
	}

	// Run the worker process:
	sem := make(chan struct{}, MAX)
	for {
		sem <- struct{}{}
		go func() {
			if err := pipe.Process(); err != nil {
				log.Println(err)
			}
			<-sem
		}()
	}
}
