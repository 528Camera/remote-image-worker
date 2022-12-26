package main

import (
	"fmt"
	"log"
	"os"

	"github.com/AsriFox/remote-image-worker/internal"
)

// Can process only MAX frames at the same time
const MAX = 10

func printUsage(help bool) {
	fmt.Println("Usage:")
	fmt.Println("remote-image-worker [pull-endpoint] [push-endpoint] {dump-path}")
	if help {
		fmt.Print("\t- Normal operation as a remote worker.\n\t- Optional: dump images in {dump-path} location.\n\n")
	}
	fmt.Println("remote-image-worker [pull-endpoint] -no-push {dump-path}")
	if help {
		fmt.Print("\t- Process incoming images but don't transmit them.\n\t- Optional: dump images in {dump-path} location.\n\n")
	}
	fmt.Println("remote-image-worker -h")
	if help {
		fmt.Print("\t- Print this message.\n\n")
	}
}

func main() {
	// Parse command line arguments:
	if len(os.Args) == 2 && os.Args[1] == "-h" {
		printUsage(true)
		return
	}
	if len(os.Args) < 3 {
		fmt.Println("Error: Insufficient number of arguments.")
		printUsage(false)
		return
	}
	pull_ep := os.Args[1]
	push_ep := os.Args[2]
	var dump_path string = ""
	if len(os.Args) == 4 {
		dump_path = os.Args[3]
	}
	var (
		pipe *internal.Pipeline
		err  error
	)
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
