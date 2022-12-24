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
	if len(os.Args) < 3 {
		log.Println("Insufficient number of arguments.\nUsage: remote-image-worker [pull-endpoint] [push-endpoint] {dump-path}")
		return
	}
	pull_ep := os.Args[1]
	push_ep := os.Args[2]
	var (
		pipe *internal.Pipeline
		err  error
	)
	var dump_path string
	if len(os.Args) == 4 {
		dump_path = os.Args[3]
	}
	if push_ep == "-no-push" {
		pipe, err = internal.NewPipelineDump(pull_ep, dump_path)
	} else {
		pipe, err = internal.NewPipeline(pull_ep, push_ep)
		pipe.SetDumpPath(dump_path)
	}
	if err != nil {
		log.Println(err)
		return
	}
	defer pipe.Close()

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
